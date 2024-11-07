use crate::{
    app::App,
    config::{kernel_stack_top, KERNEL_STACK_SIZE, TRAP_CX_PTR, USER_STACK_SIZE, USER_STACK_TOP},
    mm::{self, MapPermission, MapType, MemorySet, PhysPageNum, VirtAddr},
    task::TaskContext,
    trap::{self, TrapContext},
};

// region TaskControlBlock begin
pub struct TaskControlBlock {
    #[allow(unused)]
    id: usize,
    pub task_cx: TaskContext,
    memory_set: MemorySet,
    trap_cx_ppn: PhysPageNum,

    // User Heap
    base_size: usize,
    pub heap_bottom: usize,
    pub program_brk: usize,
}

impl TaskControlBlock {
    pub fn new(app: &App) -> Self {
        // init MemorySet
        let (memory_set, entry, base_size) = MemorySet::from_elf(app.elf());

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
            MapType::Framed,
            MapPermission::R | MapPermission::W,
        );

        // init TrapContext
        *trap_cx_ppn.get_mut() = TrapContext::new(
            entry,
            USER_STACK_TOP + USER_STACK_SIZE,
            kstack_top + KERNEL_STACK_SIZE,
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

    #[allow(unused)]
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn base_size(&self) -> usize {
        self.base_size
    }

    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn get_satp(&self) -> usize {
        self.memory_set.get_satp()
    }

    #[allow(unused)]
    pub fn set_break(&mut self, new_brk: usize) -> Option<usize> {
        let old_brk = self.program_brk;
        if new_brk < self.base_size() {
            return None;
        }

        self.memory_set
            .set_break(VirtAddr(self.heap_bottom), VirtAddr(new_brk));
        self.program_brk = new_brk;
        Some(old_brk)
    }

    pub fn add_break(&mut self, size: i32) -> Option<usize> {
        let old_brk = self.program_brk;
        let new_brk = (old_brk as i32 + size) as usize;
        if new_brk < self.base_size() {
            return None;
        }

        self.memory_set
            .set_break(VirtAddr(self.heap_bottom), VirtAddr(new_brk));
        self.program_brk = new_brk;
        Some(old_brk)
    }
}
// region TaskControlBlock end
