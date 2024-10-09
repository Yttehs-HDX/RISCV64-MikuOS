use log::debug;
use crate::{config::{KERNEL_STACK_SIZE, USER_STACK_SIZE}, trap::USER_STACK_ALLOCATOR};

// region UserStack begin
#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct UserStack {
    pub id: usize,
    pub data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }

    pub fn recycle(&self) {
        debug!("UserStack: recycle stack {}", self.id);
        USER_STACK_ALLOCATOR.exclusive_access().dealloc(self.id);
    }
}
// region UserStack end

// region KernelStack begin
#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct KernelStack {
    pub id: usize,
    pub data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn recycle(&self) {
        debug!("KernelStack: recycle stack {}", self.id);
        USER_STACK_ALLOCATOR.exclusive_access().dealloc(self.id);
    }
}
// region KernelStack end