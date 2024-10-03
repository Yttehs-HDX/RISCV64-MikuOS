pub use status::*;
pub use context::*;
pub use stack::*;
pub use control_blk::*;

use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

mod status;
mod context;
mod stack;
mod control_blk;

unsafe fn switch(current_tcb: &mut TaskControlBlock, next_tcb: &TaskControlBlock) {
    __switch(&mut current_tcb.task_cx, &next_tcb.task_cx);
}

extern "C" {
    fn __switch(current_task_cx: *mut TaskContext, next_task_cx: *const TaskContext);
}