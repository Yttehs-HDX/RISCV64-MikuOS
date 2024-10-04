use crate::{app::App, trap::TrapContext};

pub use status::*;
pub use context::*;
pub use stack::*;

mod status;
mod context;
mod stack;

// region TaskControlBlock begin
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub trap_cx: TrapContext,
    pub task_cx: TaskContext,
    pub kernel_stack: KernelStack,
    pub user_stack: UserStack,
}

impl TaskControlBlock {
    pub fn new(app: &App) -> Self {
        let mut tcb = TaskControlBlock {
            status: TaskStatus::Suspended,
            trap_cx: TrapContext::empty(),
            task_cx: TaskContext::empty(),
            kernel_stack: KernelStack::new(),
            user_stack: UserStack::new(),
        };
        tcb.trap_cx = TrapContext::new(
            app.base_addr(),
            tcb.user_stack.get_sp(),
            tcb.kernel_stack.get_sp()
        );
        tcb.task_cx = TaskContext::goto_restore(
            tcb.trap_cx.get_ptr_from_sp() as usize
        );
        tcb
    }
}
// region TaskControlBlock end