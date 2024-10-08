use crate::{app::App, app_stack::{self, KernelStack, UserStack}, trap::TrapContext};

pub use status::*;
pub use context::*;

mod status;
mod context;

// region TaskControlBlock begin
#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub trap_cx: TrapContext,
    pub task_cx: TaskContext,
    pub kernel_stack: &'static KernelStack,
    pub user_stack: &'static UserStack,
}

impl TaskControlBlock {
    pub fn empty() -> Self {
        TaskControlBlock {
            status: TaskStatus::Zombie,
            trap_cx: TrapContext::empty(),
            task_cx: TaskContext::empty(),
            kernel_stack: unsafe { &*(core::ptr::null::<KernelStack>()) },
            user_stack: unsafe { &*(core::ptr::null::<UserStack>()) },
        }
    }

    pub fn late_init(&mut self, app: &App) {
        self.user_stack = app_stack::get_user_stack();
        self.kernel_stack = app_stack::get_kernel_stack();
        self.trap_cx = TrapContext::new(
            app.base_addr(),
            self.user_stack.get_sp(),
            self.kernel_stack.get_sp(),
        );
        self.task_cx = TaskContext::goto_restore(
            self.trap_cx.get_ptr_from_sp() as usize
        );
    }

    pub fn drop(&mut self) {
        self.kernel_stack.recycle();
        self.user_stack.recycle();
    }
}
// region TaskControlBlock end