use lazy_static::lazy_static;
use log::{debug, info};
use crate::{app::App, config::MAX_TASK_NUM, sync::UPSafeCell, task::switch};
use super::{TaskControlBlock, TaskStatus};

pub fn add_task(app: &App) {
    TASK_MANAGER.add_task(app);
}

pub fn run_task() -> ! {
    let task_id = TASK_MANAGER.find_task(TaskStatus::Suspended).unwrap();
    debug!("find next task: {}", task_id);
    TASK_MANAGER.run_task(task_id);
}

pub fn print_task_info() {
    info!("Task number: {}", TASK_MANAGER.get_task_num());
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: unsafe { UPSafeCell::new(
            TaskManagerInner {
                task_num: 0,
                tasks: [TaskControlBlock::empty(); MAX_TASK_NUM],
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
        let task_id = inner.task_num;
        inner.tasks[task_id].late_init(app);
        inner.tasks[task_id].status = TaskStatus::Suspended;
        inner.task_num += 1;
    }

    fn get_task_num(&self) -> usize {
        self.inner.shared_access().task_num
    }

    fn find_task(&self, status: TaskStatus) -> Option<usize> {
        let inner = self.inner.shared_access();
        inner.tasks.iter().position( |tcb|
            tcb.status == status
        )
    }

    fn run_task(&self, i: usize) -> ! {
        let old_task_i_wrap = self.find_task(TaskStatus::Running);
        let mut inner = self.inner.exclusive_access();
        let target_status = inner.tasks[i].status;
        assert!(target_status == TaskStatus::Suspended, "TaskStatus must be Suspended");

        inner.tasks[i].status = TaskStatus::Running;
        let mut old_tcb = if let Some(old_task_i) = old_task_i_wrap {
            inner.tasks[old_task_i].status = TaskStatus::Suspended;
            inner.tasks[old_task_i]
        } else {
            TaskControlBlock::empty()
        };
        drop(inner);
        let new_tcb= &self.inner.shared_access().tasks[i];
        unsafe {
            switch(&mut old_tcb, new_tcb);
        }
        unreachable!();
    }
}
// region TaskManager end

// region TaskManagerInner begin
struct TaskManagerInner {
    task_num: usize,
    tasks: [TaskControlBlock; MAX_TASK_NUM],
}
// region TaskManagerInner end