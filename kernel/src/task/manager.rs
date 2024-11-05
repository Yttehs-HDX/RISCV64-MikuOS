use super::TaskControlBlock;
use crate::{
    app::App,
    sync::UPSafeCell,
    task::{TaskContext, __switch},
    trap::TrapContext,
};
use alloc::collections::vec_deque::VecDeque;
use lazy_static::lazy_static;
use log::info;

pub fn add_task(app: &App) {
    TASK_MANAGER.add_task(app);
}

pub fn current_user_satp() -> usize {
    TASK_MANAGER.get_current_user_satp()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

pub fn change_current_brk(increase: i32) -> Option<usize> {
    TASK_MANAGER.change_current_brk(increase)
}

pub fn run_tasks() -> ! {
    match TASK_MANAGER.get_available_task() {
        Some(tcb) => {
            TASK_MANAGER.set_current_task(tcb);
            TASK_MANAGER.resume_current_task();
        }
        _ => {
            info!("TaskManager: no task to run");
            crate::os_end()
        }
    }
}

pub fn exit_handler() -> ! {
    TASK_MANAGER.exit_current_task();

    // run other task
    run_tasks();
}

pub fn yield_handler() -> ! {
    TASK_MANAGER.suspend_current_task();

    // run other task
    let tcb = TASK_MANAGER.get_available_task().unwrap();
    TASK_MANAGER.set_current_task(tcb);
    TASK_MANAGER.save_last_and_resume_current();
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: unsafe {
            UPSafeCell::new(TaskManagerInner {
                running_task: None,
                suspend_tasks: VecDeque::new(),
            })
        }
    };
}

// region TaskManager begin
struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn add_task(&self, app: &App) {
        let mut inner = self.inner.exclusive_access();
        let tcb = TaskControlBlock::new(app);
        inner.suspend_tasks.push_back(tcb);
    }

    fn get_current_user_satp(&self) -> usize {
        let inner = self.inner.shared_access();
        let running_task = inner.running_task.as_ref().unwrap();
        running_task.get_satp()
    }

    fn get_current_trap_cx(&self) -> &'static mut TrapContext {
        let inner = self.inner.shared_access();
        let running_task = inner.running_task.as_ref().unwrap();
        running_task.get_trap_cx()
    }

    fn change_current_brk(&self, increase: i32) -> Option<usize> {
        let mut inner = self.inner.exclusive_access();
        let running_task = inner.running_task.as_mut().unwrap();
        running_task.add_break(increase)
    }
}

impl TaskManager {
    fn get_available_task(&self) -> Option<TaskControlBlock> {
        let mut inner = self.inner.exclusive_access();
        if inner.suspend_tasks.is_empty() {
            return None;
        }

        let tcb = inner.suspend_tasks.pop_front().unwrap();
        Some(tcb)
    }

    fn set_current_task(&self, tcb: TaskControlBlock) {
        let mut inner = self.inner.exclusive_access();
        assert!(
            inner.running_task.is_none(),
            "TaskManager: already has running task"
        );
        inner.running_task = Some(tcb);
    }

    fn suspend_current_task(&self) {
        let mut inner = self.inner.exclusive_access();
        assert!(inner.running_task.is_some(), "TaskManager: no running task");
        let tcb = inner.running_task.take().unwrap();
        inner.suspend_tasks.push_back(tcb);
    }

    fn exit_current_task(&self) {
        let mut inner = self.inner.exclusive_access();
        assert!(inner.running_task.is_some(), "TaskManager: no running task");
        inner.running_task = None;
    }

    fn resume_current_task(&self) -> ! {
        let inner = self.inner.shared_access();
        assert!(inner.running_task.is_some(), "TaskManager: no running task");

        // get the task context
        let running_tcb = inner.running_task.as_ref().unwrap();
        let running_task_cx = &running_tcb.task_cx as *const TaskContext;

        drop(inner);
        unsafe {
            __switch(&mut TaskContext::empty(), running_task_cx);
        }
        unreachable!();
    }

    fn save_last_and_resume_current(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        assert!(inner.running_task.is_some(), "TaskManager: no running task");

        // get the running task context
        let running_tcb = inner.running_task.as_ref().unwrap();
        let running_task_cx = &running_tcb.task_cx as *const TaskContext;

        // get the suspend task context
        let suspend_tcb = match inner.suspend_tasks.back_mut() {
            Some(tcb) => tcb,
            None => inner.running_task.as_mut().unwrap(),
        };
        let suspend_task_cx = &mut suspend_tcb.task_cx as *mut TaskContext;

        drop(inner);
        unsafe {
            __switch(suspend_task_cx, running_task_cx);
        }
        unreachable!();
    }
}
// region TaskManager end

// region TaskManagerInner begin
struct TaskManagerInner {
    running_task: Option<TaskControlBlock>,
    suspend_tasks: VecDeque<TaskControlBlock>,
}
// region TaskManagerInner end
