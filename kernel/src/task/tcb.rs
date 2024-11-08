use crate::{
    app::App,
    config::{kernel_stack_top, KERNEL_STACK_SIZE, TRAP_CX_PTR, USER_STACK_SIZE, USER_STACK_TOP},
    mm::{self, MapArea, MapPermission, MapType, MemorySet, PhysPageNum, VirtAddr},
    task::{PidHandle, TaskContext},
    trap::{self, TrapContext},
};

use super::alloc_pid_handle;

// region TaskControlBlock begin
pub struct TaskControlBlock {
    pid: PidHandle,
    trap_cx_ppn: PhysPageNum,
    task_cx: TaskContext,
    memory_set: MemorySet,
    // User Heap lower bound
    base_size: usize,
    // User Heap higher bound
    program_brk: usize,
}

impl TaskControlBlock {
    #[allow(unused)]
    pub fn pid(&self) -> usize {
        self.pid.0
    }

    pub fn base_size(&self) -> usize {
        self.base_size
    }

    pub fn get_trap_cx_mut(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.as_mut()
    }

    pub fn get_task_cx_ref(&self) -> &TaskContext {
        &self.task_cx
    }

    pub fn get_task_cx_mut(&mut self) -> &mut TaskContext {
        &mut self.task_cx
    }

    pub fn get_satp(&self) -> usize {
        self.memory_set.get_satp()
    }
}

impl TaskControlBlock {
    pub fn new(app: &App) -> Self {
        let pid = alloc_pid_handle();
        // init MemorySet
        let (memory_set, entry, base_size) = MemorySet::from_elf(app.elf());

        // get TrapContext ppn
        let trap_cx_ppn = memory_set
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn();

        // map Kernel Stack
        let kstack_top = kernel_stack_top(pid.0);
        mm::kernel_insert_area(
            VirtAddr(kstack_top),
            VirtAddr(kstack_top + KERNEL_STACK_SIZE),
            MapType::Framed,
            MapPermission::R | MapPermission::W,
        );

        // init TrapContext
        *trap_cx_ppn.as_mut() = TrapContext::new(
            entry,
            USER_STACK_TOP + USER_STACK_SIZE,
            kstack_top + KERNEL_STACK_SIZE,
            mm::kernel_satp(),
            trap::trap_handler as usize,
        );

        Self {
            pid,
            task_cx: TaskContext::goto_trap_return(kstack_top + KERNEL_STACK_SIZE),
            memory_set,
            trap_cx_ppn,
            base_size,
            program_brk: base_size,
        }
    }

    pub fn set_break(&mut self, increase: i32) -> Option<usize> {
        let old_brk = self.program_brk;
        let new_brk = (old_brk as i32 + increase) as usize;
        if new_brk < self.base_size() {
            return None;
        }

        self.memory_set
            .change_area_end(VirtAddr(self.base_size()), VirtAddr(new_brk));
        self.program_brk = new_brk;
        Some(old_brk)
    }

    pub fn insert_new_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_perm: MapPermission,
    ) {
        self.memory_set
            .insert_area(MapArea::new(start_va, end_va, MapType::Framed, map_perm));
    }

    pub fn remove_area(&mut self, start_va: VirtAddr) {
        self.memory_set.remove_area(start_va.to_vpn_floor());
    }

    pub fn change_area_end(&mut self, start_va: VirtAddr, new_end_va: VirtAddr) {
        self.memory_set.change_area_end(start_va, new_end_va);
    }
}
// region TaskControlBlock end
