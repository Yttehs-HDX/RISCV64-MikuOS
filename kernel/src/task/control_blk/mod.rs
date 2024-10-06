use crate::{app::App, app_stack::{self, StackType}, trap::TrapContext};

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
    pub kernel_sp: usize,
    pub user_sp: usize,
}

impl TaskControlBlock {
    pub fn empty() -> Self {
        TaskControlBlock {
            status: TaskStatus::Zombie,
            trap_cx: TrapContext::empty(),
            task_cx: TaskContext::empty(),
            kernel_sp: 0,
            user_sp: 0,
        }
    }

    pub fn late_init(&mut self, app: &App) {
        self.user_sp = app_stack::get_stack(StackType::User, app);
        self.kernel_sp = app_stack::get_stack(StackType::Kernel, app);
        self.trap_cx = TrapContext::new(
            app.base_addr(),
            self.user_sp,
            self.kernel_sp,
        );
        self.task_cx = TaskContext::goto_restore(
            self.trap_cx.get_ptr_from_sp() as usize
        );
    }
}
// region TaskControlBlock end