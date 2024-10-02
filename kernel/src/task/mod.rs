use core::arch::global_asm;

pub use context::*;

global_asm!(include_str!("switch.S"));

mod context;

extern "C" {
    pub fn __switch(current_task_cx: *mut TaskContext, next_task_cx: *const TaskContext);
}