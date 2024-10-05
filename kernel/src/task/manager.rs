use alloc::vec::Vec;
use lazy_static::lazy_static;
use crate::{app::{self, App}, sbi, sync::UPSafeCell};
use super::{switch, TaskControlBlock};

pub fn run_first_task() {
    let mut manager = TASK_MANAGER.exclusive_access();
    manager.add_task(app::get_app("test_print").unwrap());
    let mut _unused = TaskControlBlock::empty();
    let tcb = manager.get_task(0).unwrap();

    unsafe { switch(&mut _unused, tcb) };
}

pub fn exit_handler() -> ! {
    sbi::sbi_shutdown_success();
}

lazy_static! {
    static ref TASK_MANAGER: UPSafeCell<TaskManager> = unsafe {
        UPSafeCell::new(
            TaskManager {
                tasks: Vec::new(),
            }
        )
    };
}

// region TaskManager begin
struct TaskManager {
    tasks: Vec<TaskControlBlock>,
}

impl TaskManager {
    fn get_task(&self, index: usize) -> Option<&TaskControlBlock> {
        self.tasks.get(index)
    }

    fn add_task(&mut self, app: &App) {
        let index = self.tasks.len();
        self.tasks.push(TaskControlBlock::empty());
        self.tasks[index].late_init(app);
    }
}
// region TaskManager end