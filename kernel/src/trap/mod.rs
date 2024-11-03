use crate::{
    config::{TRAMPOLINE, TRAP_CX_PTR},
    syscall, task, timer,
};
use core::arch::{asm, global_asm};
use log::{debug, error};
use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
    utvec::TrapMode,
};

pub use context::*;

mod context;

global_asm!(include_str!("trap.S"));

pub fn init_trap() {
    set_kernel_trap_entry();
}

#[allow(unused)]
pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer() };
    timer::set_next_trigger();
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> ! {
    let stval = stval::read();
    let scause = scause::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            debug!("Ecall from U-mode @ {:#x}", cx.sepc);
            // move to next instruction
            cx.sepc += 4;
            cx.x[10] = syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            trap_return();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            timer::set_next_trigger();
            syscall::sys_yield();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("Illegal instruction @ {:#x}, badaddr {:#x}", cx.sepc, stval);
            syscall::sys_exit(1);
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("Store fault @ {:#x}, badaddr {:#x}", cx.sepc, stval);
            syscall::sys_exit(1);
        }
        _ => {
            error!("Unhandled trap {:?} @ {:#x}", scause.cause(), cx.sepc);
            syscall::sys_exit(1);
        }
    }
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CX_PTR;
    let user_satp = task::current_user_satp();
    let restore_trap_va = __restore_trap as usize - __save_trap as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_trap}", restore_trap = in(reg) restore_trap_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        )
    }
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(kernel_trap_handler as usize, TrapMode::Direct);
    }
}

fn kernel_trap_handler() {
    todo!();
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}

extern "C" {
    fn __save_trap();
    fn __restore_trap(trap_cx_ptr: *const TrapContext, user_satp: usize);
}
