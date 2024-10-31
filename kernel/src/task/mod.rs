pub use control_blk::*;
pub use manager::*;

use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

mod control_blk;
mod manager;

extern "C" {
    fn __switch(current_task_cx: *mut TaskContext, next_task_cx: *const TaskContext);
}
