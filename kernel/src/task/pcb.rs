use crate::{
    config::{TRAP_CX_PTR, USER_STACK_SP},
    mm::{self, MemorySpace, PhysPageNum, UserSpace, VirtAddr},
    sync::UPSafeCell,
    task::{alloc_pid_handle, KernelStack, PidHandle, TaskContext},
    trap::{self, TrapContext},
};
use core::cell::RefMut;

// region ProcessControlBlock begin
pub struct ProcessControlBlock {
    pid: PidHandle,
    #[allow(unused)]
    kernel_stack: KernelStack,
    inner: UPSafeCell<ProcessControlBlockInner>,
}

impl ProcessControlBlock {
    pub fn new(elf_data: &[u8]) -> Self {
        let pid = alloc_pid_handle();
        let kernel_stack = KernelStack::new(&pid);
        let user_space = UserSpace::from_elf(elf_data);
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

        Self {
            pid,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(ProcessControlBlockInner::new(
                    trap_cx_ppn,
                    task_cx,
                    user_space,
                ))
            },
        }
    }

    #[allow(unused)]
    pub fn get_pid(&self) -> usize {
        self.pid.0
    }

    pub fn inner_mut(&self) -> RefMut<ProcessControlBlockInner> {
        self.inner.exclusive_access()
    }
}
// region ProcessControlBlock end

// region ProcessControlBlockInner begin
pub struct ProcessControlBlockInner {
    trap_cx_ppn: PhysPageNum,
    task_cx: TaskContext,
    user_space: UserSpace,
    program_brk: usize,
}

impl ProcessControlBlockInner {
    fn new(trap_cx_ppn: PhysPageNum, task_cx: TaskContext, user_space: UserSpace) -> Self {
        let program_brk = user_space.get_base_size();
        Self {
            trap_cx_ppn,
            task_cx,
            user_space,
            program_brk,
        }
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

    pub fn set_break(&mut self, increase: i32) -> Option<usize> {
        let base_size = self.user_space.get_base_size();
        let old_brk = self.program_brk;
        let new_brk = (old_brk as i32 + increase) as usize;
        if new_brk < base_size {
            return None;
        }

        self.user_space
            .inner_mut()
            .change_area_end(VirtAddr(base_size), VirtAddr(new_brk));
        self.program_brk = new_brk;
        Some(old_brk)
    }
}
// region ProcessControlBlockInner end
