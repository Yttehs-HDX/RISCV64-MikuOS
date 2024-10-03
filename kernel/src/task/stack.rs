use crate::config::USER_STACK_SIZE;

// region UserStack begin
pub struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    pub fn new() -> Self {
        UserStack {
            data: [0; USER_STACK_SIZE],
        }
    }

    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
// region UserStack end

// region KernelStack begin
pub struct KernelStack {
    data: [u8; USER_STACK_SIZE],
}

impl KernelStack {
    pub fn new() -> Self {
        KernelStack {
            data: [0; USER_STACK_SIZE],
        }
    }
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
// region KernelStack end