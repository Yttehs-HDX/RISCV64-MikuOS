use super::TaskControlBlock;
use crate::{
    app::App,
    sync::UPSafeCell,
    task::{TaskContext, __switch},
};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::info;

pub fn add_task(app: &App) {
    TASK_MANAGER.add_task(app);
}

pub fn run_task() -> ! {
    TASK_MANAGER.run_task();
}

pub fn current_user_satp() -> usize {
    TASK_MANAGER.get_current_user_satp()
}

pub fn exit_handler() -> ! {
    let current_id = TASK_MANAGER.get_running_task_id().unwrap();
    TASK_MANAGER.remove_task_by_id(current_id);

    if TASK_MANAGER.suspend_task_num() == 0 {
        // no task to run
        info!("TaskManager: no task to run");
        crate::os_end();
    }

    // run other task
    run_task();
}

pub fn yield_handler() -> ! {
    run_task();
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: unsafe {
            UPSafeCell::new(TaskManagerInner {
                running_task: None,
                suspend_tasks: Vec::new(),
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
        inner.suspend_tasks.push(tcb);
    }

    fn get_running_task_id(&self) -> Option<usize> {
        let inner = self.inner.shared_access();
        inner.running_task.as_ref().map(|tcb| tcb.id)
    }

    fn find_suspend_task_by_id(&self, id: usize) -> Option<usize> {
        let inner = self.inner.shared_access();
        inner.suspend_tasks.iter().position(|tcb| tcb.id == id)
    }

    fn remove_task_by_id(&self, id: usize) {
        if let Some(id) = self.find_suspend_task_by_id(id) {
            let mut inner = self.inner.exclusive_access();
            inner.suspend_tasks.remove(id);
        }

        if self.has_running_task() {
            let mut inner = self.inner.exclusive_access();
            let running_task = inner.running_task.as_ref().unwrap();
            if running_task.id == id {
                inner.running_task = None;
            }
        }
    }

    fn has_running_task(&self) -> bool {
        let inner = self.inner.shared_access();
        inner.running_task.is_some()
    }

    fn suspend_task_num(&self) -> usize {
        let inner = self.inner.shared_access();
        inner.suspend_tasks.len()
    }

    fn get_current_user_satp(&self) -> usize {
        let inner = self.inner.shared_access();
        let running_task = inner.running_task.as_ref().unwrap();
        running_task.memory_set.get_satp()
    }
}

impl TaskManager {
    fn run_task_by_id(&self, id: usize) -> ! {
        if self.has_running_task() && self.suspend_task_num() == 0 {
            let mut inner = self.inner.exclusive_access();
            // only one task, mark running task as suspend
            let current_tcb = inner.running_task.take().unwrap();
            inner.suspend_tasks.push(current_tcb);
        }

        let mut inner = self.inner.exclusive_access();
        let current_task_cx_ptr: usize;
        let next_task_cx_ptr: usize;
        {
            // remove next task from suspend list
            let next_tcb = match inner.suspend_tasks.iter().position(|tcb| tcb.id == id) {
                Some(index) => inner.suspend_tasks.remove(index),
                _ => panic!("TaskManager: no task with id {}", id),
            };
            match inner.running_task.take() {
                Some(current_tcb) => {
                    // push current task to suspend list
                    inner.suspend_tasks.push(current_tcb);
                    let pos = inner.suspend_tasks.len() - 1;
                    // get current task context pointer
                    current_task_cx_ptr =
                        &mut inner.suspend_tasks[pos].task_cx as *const _ as usize;
                }
                _ => {
                    // no running task
                    current_task_cx_ptr = &TaskContext::empty() as *const _ as usize;
                }
            }
            // set next task as running task
            inner.running_task = Some(next_tcb);
            // get next task context pointer
            next_task_cx_ptr = &inner.running_task.as_ref().unwrap().task_cx as *const _ as usize;
        }

        drop(inner); // drop lock manually
        unsafe {
            __switch(
                current_task_cx_ptr as *mut TaskContext,
                next_task_cx_ptr as *const TaskContext,
            );
        }
        unreachable!()
    }

    fn run_task(&self) -> ! {
        let inner = self.inner.shared_access();
        let waiting_task_id = inner.suspend_tasks[0].id;
        drop(inner); // drop lock manually
        self.run_task_by_id(waiting_task_id);
    }
}
// region TaskManager end

// region TaskManagerInner begin
struct TaskManagerInner {
    running_task: Option<TaskControlBlock>,
    suspend_tasks: Vec<TaskControlBlock>,
}
// region TaskManagerInner end
