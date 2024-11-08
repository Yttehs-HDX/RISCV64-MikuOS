use crate::{
    app::App,
    config::{TRAP_CX_PTR, USER_STACK_SP},
    mm::{self, MapPermission, MemorySpace, PhysPageNum, UserSpace, VirtAddr},
    task::{alloc_pid_handle, KernelStack, PidHandle, TaskContext},
    trap::{self, TrapContext},
};

// region TaskControlBlock begin
pub struct TaskControlBlock {
    pid: PidHandle,
    trap_cx_ppn: PhysPageNum,
    task_cx: TaskContext,
    #[allow(unused)]
    kernel_stack: KernelStack,
    user_space: UserSpace,
    base_size: usize,
}

impl TaskControlBlock {
    #[allow(unused)]
    pub fn get_pid(&self) -> usize {
        self.pid.0
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
        self.user_space.get_satp()
    }
}

impl TaskControlBlock {
    pub fn new(app: &App) -> Self {
        let pid = alloc_pid_handle();
        let kernel_stack = KernelStack::new(&pid);
        let user_space = UserSpace::from_elf(app.elf());
        let trap_cx_ppn = user_space
            .inner_mut()
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn();
        *trap_cx_ppn.as_mut() = TrapContext::new(
            user_space.get_entry(),
            USER_STACK_SP,
            kernel_stack.get_sp(),
            mm::get_kernel_space().get_satp(),
            trap::trap_handler as usize,
        );
        let task_cx = TaskContext::goto_trap_return(kernel_stack.get_sp());
        let base_size = user_space.get_base_size();

        Self {
            pid,
            trap_cx_ppn,
            task_cx,
            kernel_stack,
            user_space,
            base_size,
        }
    }

    pub fn set_break(&mut self, increase: i32) -> Option<usize> {
        let old_brk = self.base_size;
        let new_brk = (old_brk as i32 + increase) as usize;
        if new_brk < self.base_size {
            return None;
        }

        self.user_space
            .inner_mut()
            .change_area_end(VirtAddr(self.base_size), VirtAddr(new_brk));
        self.base_size = new_brk;
        Some(old_brk)
    }

    pub fn insert_new_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_perm: MapPermission,
    ) {
        self.user_space
            .inner_mut()
            .insert_framed_area(start_va, end_va, map_perm);
    }

    pub fn remove_area(&mut self, start_va: VirtAddr) {
        self.user_space
            .inner_mut()
            .remove_area(start_va.to_vpn_floor());
    }

    pub fn change_area_end(&mut self, start_va: VirtAddr, new_end_va: VirtAddr) {
        self.user_space
            .inner_mut()
            .change_area_end(start_va, new_end_va);
    }
}
// region TaskControlBlock end
