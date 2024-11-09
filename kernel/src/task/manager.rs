use core::cell::RefMut;
use crate::{app::App, sync::UPSafeCell, trap::TrapContext};
use alloc::{collections::vec_deque::VecDeque, sync::Arc};
use lazy_static::lazy_static;
use log::info;
use crate::task::ProcessControlBlock;

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
    TASK_MANAGER.set_current_brk(increase)
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
    TASK_MANAGER.resume_current_task();
}

pub fn get_task_manager() -> &'static TaskManager {
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

// // region TaskManager begin
// struct TaskManager {
//     inner: UPSafeCell<TaskManagerInner>,
// }

// impl TaskManager {
//     fn add_task(&self, app: &App) {
//         let mut inner = self.inner.exclusive_access();
//         let tcb = ProcessControlBlock::new(app);
//         inner.suspend_tasks.push_back(tcb);
//     }

//     fn get_current_user_satp(&self) -> usize {
//         let inner = self.inner.shared_access();
//         let running_task = inner.running_task.as_ref().unwrap();
//         let res = running_task.inner_mut().get_satp();
//         res
//     }

//     fn get_current_trap_cx(&self) -> &'static mut TrapContext {
//         let inner = self.inner.shared_access();
//         let running_task = inner.running_task.as_ref().unwrap();
//         let res = running_task.inner_mut().get_trap_cx_mut();
//         res
//     }

//     fn set_current_brk(&self, increase: i32) -> Option<usize> {
//         let mut inner = self.inner.exclusive_access();
//         let running_task = inner.running_task.as_mut().unwrap();
//         let res = running_task.inner_mut().set_break(increase);
//         res
//     }
// }

// impl TaskManager {
//     fn get_available_task(&self) -> Option<ProcessControlBlock> {
//         let mut inner = self.inner.exclusive_access();
//         if inner.suspend_tasks.is_empty() {
//             return None;
//         }

//         let tcb = inner.suspend_tasks.pop_front().unwrap();
//         Some(tcb)
//     }

//     fn set_current_task(&self, tcb: ProcessControlBlock) {
//         let mut inner = self.inner.exclusive_access();
//         assert!(
//             inner.running_task.is_none(),
//             "TaskManager: already has running task"
//         );
//         inner.running_task = Some(tcb);
//     }

//     fn suspend_current_task(&self) {
//         let mut inner = self.inner.exclusive_access();
//         assert!(inner.running_task.is_some(), "TaskManager: no running task");
//         let tcb = inner.running_task.take().unwrap();
//         unsafe {
//             __save_task(tcb.inner_mut().get_task_cx_mut());
//         }
//         inner.suspend_tasks.push_back(tcb);
//     }

//     fn exit_current_task(&self) {
//         let mut inner = self.inner.exclusive_access();
//         assert!(inner.running_task.is_some(), "TaskManager: no running task");
//         inner.running_task = None;
//     }

//     fn resume_current_task(&self) -> ! {
//         let inner = self.inner.shared_access();
//         assert!(inner.running_task.is_some(), "TaskManager: no running task");

//         // get the task context
//         let running_tcb = inner.running_task.as_ref().unwrap();
//         let running_task_cx = running_tcb.inner_mut().get_task_cx_ref() as *const TaskContext;

//         drop(inner);
//         unsafe {
//             __restore_task(running_task_cx);
//         }
//         unreachable!();
//     }
// }
// // region TaskManager end

// // region TaskManagerInner begin
// struct TaskManagerInner {
//     running_task: Option<ProcessControlBlock>,
//     suspend_tasks: VecDeque<ProcessControlBlock>,
// }
// // region TaskManagerInner end
