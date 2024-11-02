pub use context::*;

use crate::{
    app::App,
    config::{kernel_stack_bottom, TRAP_CX_PTR, USER_STACK_BOTTOM},
    mm::{self, MemorySet, PhysPageNum, VirtAddr},
    trap::{self, TrapContext},
};

mod context;

// region TaskControlBlock begin
pub struct TaskControlBlock {
    pub id: usize,
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,

    // User Heap
    pub base_size: usize,
    pub heap_bottom: usize,
    pub program_brk: usize,
}

impl TaskControlBlock {
    pub fn new(app: &App) -> Self {
        let (memory_set, entry, base_size) = MemorySet::from_elf(&app.bin());
        let trap_cx_ppn = memory_set
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn();

        let kstack_bottom = kernel_stack_bottom(app.id());

        // init TrapContext
        *trap_cx_ppn.get_mut() = TrapContext::new(
            entry,
            USER_STACK_BOTTOM,
            kstack_bottom,
            mm::kernel_satp(),
            trap::trap_handler as usize,
        );

        Self {
            id: app.id(),
            task_cx: TaskContext::goto_restore(TRAP_CX_PTR),
            memory_set,
            trap_cx_ppn,
            base_size,
            heap_bottom: base_size,
            program_brk: base_size,
        }
    }
}
// region TaskControlBlock end
