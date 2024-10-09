use core::arch::global_asm;
use log::{debug, error};
use riscv::register::{scause::{self, Exception, Interrupt, Trap}, sie, stval, stvec, utvec::TrapMode};
use crate::{syscall, task, timer};

pub use context::*;
pub use stack_allocator::*;

mod context;
mod stack_allocator;

global_asm!(include_str!("trap.S"));

pub fn init_trap() {
    unsafe { stvec::write(__save_trap as usize, TrapMode::Direct) };
}

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer() };
    timer::set_next_trigger();
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &TrapContext {
    let stval = stval::read();
    let scause = scause::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            debug!("Ecall from U-mode @ {:#x}", cx.sepc);
            cx.sepc += 4;
            cx.x[10] = syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            cx
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            timer::set_next_trigger();
            task::yield_handler();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("Illegal instruction @ {:#x}, badaddr {:#x}", cx.sepc, stval);
            syscall::sys_exit(1);
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            error!("Store fault @ {:#x}, badaddr {:#x}", cx.sepc, stval);
            syscall::sys_exit(1);
        }
        _ => {
            error!("Unhandled trap {:?} @ {:#x}", scause.cause(), cx.sepc);
            syscall::sys_exit(1);
        }
    }
}

extern "C" {
    pub fn __save_trap();
    pub fn __restore_trap();
}