use crate::task::get_task_manager;
use crate::{app, task::ProcessControlBlock};
use alloc::sync::Arc;
use lazy_static::lazy_static;

pub(in crate::task) fn add_initproc() {
    get_task_manager().add(INITPROC.clone());
}

lazy_static! {
    static ref INITPROC: Arc<ProcessControlBlock> = Arc::new(ProcessControlBlock::new(
        app::get_app("initproc").unwrap().elf()
    ));
}
