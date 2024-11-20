use crate::{
    config::{TRAP_CX_PTR, USER_STACK_SP},
    mm::{self, MemorySpace, PhysPageNum, PpnOffset, UserSpace, VirtAddr},
    sync::UPSafeCell,
    task::{alloc_pid_handle, KernelStack, PidHandle, TaskContext},
    trap::{self, TrapContext},
};
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use core::cell::{Ref, RefMut};

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
            .ppn()
            .low_to_high();
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

    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        let pid = alloc_pid_handle();
        let kernel_stack = KernelStack::new(&pid);
        let kernel_sp = kernel_stack.get_sp();
        let user_space = UserSpace::from_existed(self.inner().get_user_space());
        let trap_cx_ppn = user_space
            .inner_mut()
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn()
            .low_to_high();
        let task_cx = TaskContext::goto_trap_return(kernel_sp);

        let pcb = Arc::new(Self {
            pid,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(ProcessControlBlockInner::new(
                    trap_cx_ppn,
                    task_cx,
                    user_space,
                ))
            },
        });
        pcb.get_trap_cx_mut().set_kernel_sp(kernel_sp);

        // Set parent
        pcb.set_parent(Arc::downgrade(self));

        // Add to parent's children
        self.inner_mut().children.push(pcb.clone());

        pcb
    }

    pub fn inner(&self) -> Ref<ProcessControlBlockInner> {
        self.inner.shared_access()
    }

    pub fn inner_mut(&self) -> RefMut<ProcessControlBlockInner> {
        self.inner.exclusive_access()
    }
}

impl ProcessControlBlock {
    pub fn get_pid(&self) -> usize {
        self.pid.0
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let user_space = UserSpace::from_elf(elf_data);
        let trap_cx_ppn = user_space
            .inner_mut()
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn();
        *trap_cx_ppn.as_mut() = TrapContext::new(
            user_space.get_entry(),
            USER_STACK_SP,
            self.kernel_stack.get_sp(),
            mm::get_kernel_space().get_satp(),
            trap::trap_handler as usize,
        );

        // update program brk, user space and trap context
        self.inner_mut().program_brk = user_space.get_base_size();
        self.inner_mut().user_space = Some(user_space);
        self.inner_mut().trap_cx_ppn = trap_cx_ppn;
    }
}

impl ProcessControlBlock {
    pub fn set_parent(&self, parent: Weak<ProcessControlBlock>) {
        self.inner_mut().parent = Some(parent);
    }

    pub fn set_exit_code(&self, exit_code: i32) {
        self.inner_mut().exit_code = exit_code;
    }

    pub fn get_exit_code(&self) -> i32 {
        self.inner().exit_code
    }

    pub fn drop_user_space(&self) {
        self.inner_mut().user_space = None;
    }

    pub fn is_zombie(&self) -> bool {
        self.inner().user_space.is_none()
    }

    pub fn get_satp(&self) -> usize {
        self.inner().get_user_space().get_satp()
    }

    pub fn get_trap_cx_mut(&self) -> &'static mut TrapContext {
        self.inner().trap_cx_ppn.as_mut()
    }

    pub fn set_break(&self, increase: i32) -> Option<usize> {
        let base_size = self.inner().get_user_space().get_base_size();
        let old_brk = self.inner().program_brk;
        let new_brk = (old_brk as i32 + increase) as usize;
        if new_brk < base_size {
            return None;
        }

        self.inner()
            .get_user_space()
            .inner_mut()
            .change_area_end(VirtAddr(base_size), VirtAddr(new_brk));
        self.inner_mut().program_brk = new_brk;
        Some(old_brk)
    }
}
// region ProcessControlBlock end

// region ProcessControlBlockInner begin
pub struct ProcessControlBlockInner {
    trap_cx_ppn: PhysPageNum,
    task_cx: TaskContext,
    user_space: Option<UserSpace>,
    program_brk: usize,

    parent: Option<Weak<ProcessControlBlock>>,
    children: Vec<Arc<ProcessControlBlock>>,
    exit_code: i32,
}

impl ProcessControlBlockInner {
    fn new(trap_cx_ppn: PhysPageNum, task_cx: TaskContext, user_space: UserSpace) -> Self {
        let program_brk = user_space.get_base_size();
        Self {
            trap_cx_ppn,
            task_cx,
            user_space: Some(user_space),
            program_brk,
            parent: None,
            children: Vec::new(),
            exit_code: 0,
        }
    }

    fn get_user_space(&self) -> &UserSpace {
        self.user_space.as_ref().unwrap()
    }

    pub fn get_task_cx_ref(&self) -> &TaskContext {
        &self.task_cx
    }

    pub fn get_task_cx_mut(&mut self) -> &mut TaskContext {
        &mut self.task_cx
    }

    pub fn get_children_ref(&self) -> &Vec<Arc<ProcessControlBlock>> {
        &self.children
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<Arc<ProcessControlBlock>> {
        &mut self.children
    }
}
// region ProcessControlBlockInner end
