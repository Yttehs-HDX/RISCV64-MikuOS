use crate::{
    allocator::{self, KernelStack, Stack, UserStack},
    app::App,
    trap::TrapContext,
};

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
        let kernel_stack = allocator::alloc_kernel_stack();
        let user_stack = allocator::alloc_user_stack();
        let trap_cx = TrapContext::new(app.base_addr(), user_stack.get_sp(), kernel_stack.get_sp());
        let task_cx = TaskContext::goto_restore(trap_cx.get_ptr_from_sp() as usize);
        Self {
            task_cx,
            kernel_stack,
            user_stack,
        }
    }

    pub fn drop(&mut self) {
        allocator::dealloc_kernel_stack(self.kernel_stack);
        allocator::dealloc_user_stack(self.user_stack);
    }
}
// region TaskControlBlock end
