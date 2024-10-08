use alloc::collections::vec_deque::VecDeque;
use lazy_static::lazy_static;
use log::{debug, info};
use crate::{app::App, sync::UPSafeCell, task::switch};
use super::TaskControlBlock;

pub fn add_task(app: &App) {
    TASK_MANAGER.add_task(app);
}

pub fn exit_handler() -> ! {
    TASK_MANAGER.info();
    TASK_MANAGER.exit_current_task();
    if TASK_MANAGER.get_task_num() == 0 {
        info!("TaskManager: all tasks are finished");
        crate::kernel_end();
    }
    run_task();
}

pub fn yield_handler() -> ! {
    TASK_MANAGER.info();
    run_task();
}

pub fn run_task() -> ! {
    TASK_MANAGER.run_suspend_task();
}

pub fn print_task_info() {
    let num = TASK_MANAGER.get_task_num();
    info!("TaskManager: task number: {}", num);
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: unsafe { UPSafeCell::new(
            TaskManagerInner {
                running_tasks: VecDeque::new(),
                waiting_tasks: VecDeque::new(),
            }
        )}
    };
}

// region TaskManager begin
struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn add_task(&self, app: &App) {
        let mut inner = self.inner.exclusive_access();
        let mut tcb = TaskControlBlock::empty();
        tcb.late_init(app);
        inner.waiting_tasks.push_back(tcb);
    }

    fn exit_current_task(&self) {
        let mut inner = self.inner.exclusive_access();
        let tcb = inner.running_tasks.pop_front();
        if let Some(mut tcb) = tcb {
            tcb.drop();
        }
    }

    fn get_task_num(&self) -> usize {
        let running = self.inner.shared_access().running_tasks.len();
        let waiting = self.inner.shared_access().waiting_tasks.len();
        running + waiting
    }

    fn info(&self) {
        let running = self.inner.shared_access().running_tasks.len();
        let waiting = self.inner.shared_access().waiting_tasks.len();
        debug!("TaskManager: running: {}, suspended: {}", running, waiting);
    }

    fn run_suspend_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        if inner.waiting_tasks.is_empty() {
            let tcb = inner.running_tasks.pop_front().unwrap();
            inner.waiting_tasks.push_back(tcb);
        }
        let mut old_tcb = if inner.running_tasks.is_empty() {
            TaskControlBlock::empty()
        } else {
            inner.running_tasks.pop_front().unwrap()
        };
        let new_tcb = inner.waiting_tasks.pop_front().unwrap();
        inner.running_tasks.push_back(new_tcb);
        drop(inner);
        unsafe {
            switch(&mut old_tcb, &new_tcb);
        }
        unreachable!()
    }
}
// region TaskManager end

// region TaskManagerInner begin
struct TaskManagerInner {
    running_tasks: VecDeque<TaskControlBlock>,
    waiting_tasks: VecDeque<TaskControlBlock>,
}
// region TaskManagerInner end