use crate::timer;
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

#[link_section = ".text.s_trap"]
fn kernel_trap_handler() {
    panic!("A trap occurred in kernel!");
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(__snap_trap as usize, TrapMode::Direct);
    }
}
