use crate::{
    sync::UPSafeCell,
    task::{get_initproc, ProcessControlBlock},
};
use alloc::{collections::vec_deque::VecDeque, sync::Arc};
use core::cell::{Ref, RefMut};
use lazy_static::lazy_static;

pub fn add_task(pcb: Arc<ProcessControlBlock>) {
    get_task_manager().add(pcb);
}

pub(in crate::task) fn get_task_manager() -> &'static TaskManager {
    &TASK_MANAGER
}

#[cfg(feature = "test")]
pub fn create_process(path: &str) {
    use crate::fs::{self, File, Inode, OpenFlags};
    use alloc::vec::Vec;

    let inode = fs::open_file(path, OpenFlags::RDONLY).unwrap();
    let len = inode.size();
    let mut buf = Vec::with_capacity(len);
    unsafe {
        buf.set_len(len);
    }
    let buf = buf.as_mut_slice();
    let file = inode.to_file();
    file.read(buf);
    let pcb = Arc::new(ProcessControlBlock::new(buf));
    pcb.set_parent(Arc::downgrade(get_initproc()));
    add_task(pcb);
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

    fn inner(&self) -> Ref<TaskManagerInner> {
        self.inner.shared_access()
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

    pub fn is_empty(&self) -> bool {
        self.inner().ready_queue.is_empty()
    }
}
// region TaskManager end

// region TaskManagerInner begin
struct TaskManagerInner {
    ready_queue: VecDeque<Arc<ProcessControlBlock>>,
}
// region TaskManagerInner end
