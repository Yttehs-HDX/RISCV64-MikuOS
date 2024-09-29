use crate::{config::{KERNEL_STACK_SIZE, USER_STACK_SIZE}, trap::TrapContext};

pub static KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);
pub static USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

// region KernelStack begin
#[repr(align(4096))]
pub struct KernelStack([u8; KERNEL_STACK_SIZE]);

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.0.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_cx(&self, cx: TrapContext) -> *mut TrapContext {
        let cx_size = core::mem::size_of::<TrapContext>();
        let cx_ptr = (KERNEL_STACK.get_sp() - cx_size) as *mut TrapContext;
        unsafe {
            cx_ptr.write(cx);
            cx_ptr.as_mut().unwrap()
        }
    }
}
// region KernelStack end

// region UserStack begin
#[repr(align(4096))]
pub struct UserStack([u8; USER_STACK_SIZE]);

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.0.as_ptr() as usize + USER_STACK_SIZE
    }
}
// region UserStack end