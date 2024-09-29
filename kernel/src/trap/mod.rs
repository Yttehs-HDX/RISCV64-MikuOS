use core::arch::global_asm;
use log::{debug, error};
use riscv::register::{scause::{self, Exception, Trap}, stval, stvec, utvec::TrapMode};
use crate::{batch, syscall};

pub use trap_context::*;

mod trap_context;

global_asm!(include_str!("trap.S"));

pub fn init_trap() {
    unsafe { stvec::write(__save_trap as usize, TrapMode::Direct) };
}

#[no_mangle]
fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let stval = stval::read();
    let scause = scause::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            debug!("ecall from U-mode @ {:#x}", cx.sepc);
            cx.sepc += 4;
            syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            return cx;
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("illegal instruction @ {:#x}, badaddr {:#x}", cx.sepc, stval);
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            error!("store fault @ {:#x}, badaddr {:#x}", cx.sepc, stval);
        }
        _ => {
            error!("unhandled trap {:?} @ {:#x}", scause.cause(), cx.sepc);
        }
    }
    batch::run_next_app();
}

extern "C" {
    pub fn __save_trap();
    pub fn __restore_trap(cx_addr: usize);
}