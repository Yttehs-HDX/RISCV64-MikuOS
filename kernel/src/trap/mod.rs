use crate::{sbi, timer};
use log::error;
use riscv::register::{sie, stvec, utvec::TrapMode};

pub use context::*;
pub use control_flow::*;

mod context;
mod control_flow;

pub fn init_trap() {
    set_kernel_trap_entry();
}

#[allow(unused)]
pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer() };
    timer::set_next_trigger();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(kernel_trap_handler as usize, TrapMode::Direct);
    }
}

fn kernel_trap_handler() {
    error!("A trap occurred in kernel!");
    sbi::sbi_shutdown_failure();
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(__snap_trap as usize, TrapMode::Direct);
    }
}
