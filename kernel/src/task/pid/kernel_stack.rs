use crate::{
    config::{kernel_stack_top, KERNEL_STACK_SIZE},
    mm::{self, MapPermission, VirtAddr},
    task::PidHandle,
};

// region KernelSpace begin
pub struct KernelStack {
    pid: usize,
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let pid = self.pid;
        let kstack_top = kernel_stack_top(pid);
        mm::get_kernel_space()
            .inner_mut()
            .remove_area(VirtAddr(kstack_top).to_vpn());
    }
}

impl KernelStack {
    pub fn new(pid_handle: &PidHandle) -> Self {
        let pid = pid_handle.0;

        // let kstack_top = kernel_stack_top(pid);
        // let kstacp_bottom = kstack_top + KERNEL_STACK_SIZE;

        // // allocate kernel stack
        // mm::get_kernel_space().inner_mut().insert_framed_area(
        //     VirtAddr(kstack_top),
        //     VirtAddr(kstacp_bottom),
        //     MapPermission::R | MapPermission::W,
        // );

        Self { pid }
    }

    pub fn get_sp(&self) -> usize {
        kernel_stack_top(self.pid) + KERNEL_STACK_SIZE
    }
}
// region KernelSpace end
