use crate::{app::App, trap::{self, KernelStack, TrapContext, UserStack}};

pub use context::*;

mod context;

// region TaskControlBlock begin
#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub task_cx: TaskContext,
    pub kernel_stack: &'static KernelStack,
    pub user_stack: &'static UserStack,
}

impl TaskControlBlock {
    pub fn new(app: &App) -> Self {
        let kernel_stack = trap::get_kernel_stack();
        let user_stack = trap::get_user_stack();
        let trap_cx = TrapContext::new(
            app.base_addr(),
            user_stack.get_sp(),
            kernel_stack.get_sp(),
        );
        let task_cx = TaskContext::goto_restore(trap_cx.get_ptr_from_sp() as usize);
        Self {
            task_cx,
            kernel_stack,
            user_stack,
        }
    }

    pub fn drop(&mut self) {
        self.kernel_stack.recycle();
        self.user_stack.recycle();
    }
}
// region TaskControlBlock end