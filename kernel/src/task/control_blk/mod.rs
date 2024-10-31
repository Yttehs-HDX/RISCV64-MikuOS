pub use context::*;

use crate::{app::App, mm::MemorySet};

mod context;

// region TaskControlBlock begin
pub struct TaskControlBlock {
    pub id: usize,
    pub task_cx: TaskContext,
    pub trap_cx_ppn: usize,
    pub memory_set: MemorySet,

    // User Heap
    pub base_size: usize,
    pub heap_bottom: usize,
    pub program_brk: usize,
}

impl TaskControlBlock {
    pub fn new(app: &App) -> Self {
        todo!()
    }
}
// region TaskControlBlock end
