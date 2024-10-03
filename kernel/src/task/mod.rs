pub use context::*;
pub use control_blk::*;

use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

mod context;
mod control_blk;

extern "C" {
    pub fn __switch(current_task_cx: *mut TaskContext, next_task_cx: *const TaskContext);
}