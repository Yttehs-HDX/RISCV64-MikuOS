use crate::task::{get_task_manager, ProcessControlBlock};
use alloc::sync::Arc;
use lazy_static::lazy_static;

pub(in crate::task) fn add_initproc() {
    get_task_manager().add_to_back(INITPROC.clone());
}

pub(in crate::task) fn get_initproc() -> &'static Arc<ProcessControlBlock> {
    &INITPROC
}

const INITPROC_ELF: &[u8] =
    include_bytes!("../../../../user/target/riscv64gc-unknown-none-elf/release/initproc");

lazy_static! {
    static ref INITPROC: Arc<ProcessControlBlock> =
        Arc::new(ProcessControlBlock::new(INITPROC_ELF));
}
