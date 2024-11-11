use crate::{sync::UPSafeCell, task::ProcessControlBlock};
use alloc::{collections::vec_deque::VecDeque, sync::Arc};
use core::cell::RefMut;
use lazy_static::lazy_static;

pub fn add_task(pcb: Arc<ProcessControlBlock>) {
    get_task_manager().add(pcb);
}

pub(in crate::task) fn get_task_manager() -> &'static TaskManager {
    &TASK_MANAGER
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager::new();
}

// region TaskManager begin
pub struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    ready_queue: VecDeque::new(),
                })
            },
        }
    }

    fn inner_mut(&self) -> RefMut<TaskManagerInner> {
        self.inner.exclusive_access()
    }
}

impl TaskManager {
    pub fn add(&self, pcb: Arc<ProcessControlBlock>) {
        self.inner_mut().ready_queue.push_back(pcb);
    }

    pub fn fetch(&self) -> Option<Arc<ProcessControlBlock>> {
        self.inner_mut().ready_queue.pop_front()
    }
}
// region TaskManager end

// region TaskManagerInner begin
struct TaskManagerInner {
    ready_queue: VecDeque<Arc<ProcessControlBlock>>,
}
// region TaskManagerInner end
