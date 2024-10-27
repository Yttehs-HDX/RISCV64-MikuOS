use super::TaskControlBlock;
use crate::{
    app::App,
    sync::UPSafeCell,
    task::{switch, TaskContext, __switch},
};
use alloc::collections::vec_deque::VecDeque;
use lazy_static::lazy_static;
use log::{debug, info};

pub fn add_task(app: &App) {
    TASK_MANAGER.add_task(app);
}

pub fn exit_handler() -> ! {
    TASK_MANAGER.info();
    TASK_MANAGER.exit_current_task();
    TASK_MANAGER.info();
    if TASK_MANAGER.get_task_num(TaskStatus::All) == 0 {
        info!("TaskManager: all tasks are finished");
        crate::os_end();
    }
    run_task();
}

pub fn yield_handler() -> ! {
    TASK_MANAGER.info();
    run_task();
}

pub fn run_task() -> ! {
    TASK_MANAGER.run_task();
}

pub fn print_task_info() {
    let num = TASK_MANAGER.get_task_num(TaskStatus::All);
    info!("TaskManager: task number: {}", num);
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: unsafe {
            UPSafeCell::new(TaskManagerInner {
                running_tasks: VecDeque::new(),
                waiting_tasks: VecDeque::new(),
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
        inner.waiting_tasks.push_back(tcb);
    }

    fn exit_current_task(&self) {
        let mut inner = self.inner.exclusive_access();
        let tcb = inner.running_tasks.pop_front();
        if let Some(mut tcb) = tcb {
            tcb.drop();
        }
    }

    fn get_task_num(&self, status: TaskStatus) -> usize {
        let running = self.inner.shared_access().running_tasks.len();
        let waiting = self.inner.shared_access().waiting_tasks.len();
        match status {
            TaskStatus::All => running + waiting,
            TaskStatus::Running => running,
            TaskStatus::Suspended => waiting,
        }
    }

    fn info(&self) {
        let running = self.get_task_num(TaskStatus::Running);
        let waiting = self.get_task_num(TaskStatus::Suspended);
        debug!("TaskManager: running: {}, suspended: {}", running, waiting);
    }

    fn _run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let tcb = inner.waiting_tasks.pop_front().unwrap();
        inner.running_tasks.push_back(tcb);
        drop(inner);
        unsafe {
            __switch(&mut TaskContext::empty(), &tcb.task_cx);
        }
        unreachable!()
    }

    fn run_task(&self) -> ! {
        if self.get_task_num(TaskStatus::Running) == 0 {
            self._run_first_task();
        }
        if self.get_task_num(TaskStatus::Running) == 1 {
            let mut inner = self.inner.exclusive_access();
            let tcb = inner.running_tasks.pop_front().unwrap();
            inner.waiting_tasks.push_back(tcb);
            drop(inner);
            self._run_first_task();
        }

        let mut inner = self.inner.exclusive_access();
        let new_tcb = inner.waiting_tasks.pop_front().unwrap();
        let mut old_tcb = inner.running_tasks.pop_front().unwrap();
        inner.running_tasks.push_back(new_tcb);
        inner.waiting_tasks.push_back(old_tcb);
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

// region TaskStatus begin
pub enum TaskStatus {
    All,
    Running,
    Suspended,
}
// region TaskStatus end
