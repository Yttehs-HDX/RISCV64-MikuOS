pub use context::*;

use crate::{
    app::App,
    config::{kernel_stack_top, KERNEL_STACK_SIZE, TRAP_CX_PTR, USER_STACK_TOP},
    mm::{self, MapPermission, MapType, MemorySet, PhysPageNum, VirtAddr},
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
        // init MemorySet
        let (memory_set, entry, base_size) = MemorySet::from_elf(&app.elf());

        // alloc TrapContext
        let trap_cx_ppn = memory_set
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn();

        // map Kernel Stack
        let kstack_top = kernel_stack_top(app.id());
        mm::kernel_insert_area(
            VirtAddr(kstack_top),
            VirtAddr(kstack_top + KERNEL_STACK_SIZE),
            MapType::Identity,
            MapPermission::R | MapPermission::W,
        );

        // init TrapContext
        *trap_cx_ppn.get_mut() = TrapContext::new(
            entry,
            USER_STACK_TOP,
            kstack_top,
            mm::kernel_satp(),
            trap::trap_handler as usize,
        );

        Self {
            id: app.id(),
            task_cx: TaskContext::goto_trap_return(kstack_top + KERNEL_STACK_SIZE),
            memory_set,
            trap_cx_ppn,
            base_size,
            heap_bottom: base_size,
            program_brk: base_size,
        }
    }
}
// region TaskControlBlock end
