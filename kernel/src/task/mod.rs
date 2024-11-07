pub use context::*;
pub use manager::*;
pub use tcb::*;

use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

mod context;
mod manager;
mod tcb;

extern "C" {
    fn __switch(current_task_cx: *mut TaskContext, next_task_cx: *const TaskContext);
}
