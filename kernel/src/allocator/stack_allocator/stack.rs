use crate::config::{KERNEL_STACK_SIZE, USER_STACK_SIZE};

pub trait Stack {
    fn get_sp(&self) -> usize;
}

// region UserStack begin
#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct UserStack {
    pub id: usize,
    pub data: [u8; USER_STACK_SIZE],
}

impl Stack for UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
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

impl Stack for KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}
// region KernelStack end
