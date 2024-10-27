use crate::{
    config::{KERNEL_STACK_SIZE, MAX_TASK_NUM, USER_STACK_SIZE},
    sync::UPSafeCell,
};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::debug;

pub use stack::*;

mod stack;

pub fn alloc_kernel_stack() -> &'static KernelStack {
    let index = KERNEL_STACK_ALLOCATOR.exclusive_access().alloc().unwrap();
    debug!("KernelStack: get stack {}", index);
    &KERNEL_STACKS[index]
}

pub fn alloc_user_stack() -> &'static UserStack {
    let index = USER_STACK_ALLOCATOR.exclusive_access().alloc().unwrap();
    debug!("UserStack: get stack {}", index);
    &USER_STACKS[index]
}

pub fn dealloc_kernel_stack(stack: &'static KernelStack) {
    debug!("KernelStack: recycle stack {}", stack.id);
    KERNEL_STACK_ALLOCATOR.exclusive_access().dealloc(stack.id);
}

pub fn dealloc_user_stack(stack: &'static UserStack) {
    debug!("UserStack: recycle stack {}", stack.id);
    USER_STACK_ALLOCATOR.exclusive_access().dealloc(stack.id);
}

lazy_static! {
    pub static ref USER_STACK_ALLOCATOR: UPSafeCell<StackAllocator> =
        unsafe { UPSafeCell::new(StackAllocator::new()) };
    pub static ref KERNEL_STACK_ALLOCATOR: UPSafeCell<StackAllocator> =
        unsafe { UPSafeCell::new(StackAllocator::new()) };
    static ref USER_STACKS: [UserStack; MAX_TASK_NUM] = {
        static mut STACKS: [UserStack; MAX_TASK_NUM] = [UserStack {
            id: 0,
            data: [0; USER_STACK_SIZE],
        }; MAX_TASK_NUM];
        unsafe {
            #[allow(static_mut_refs)]
            STACKS.iter_mut().enumerate().for_each(|(i, stack)| {
                stack.id = i;
            });
            STACKS
        }
    };
    static ref KERNEL_STACKS: [KernelStack; MAX_TASK_NUM] = {
        static mut STACKS: [KernelStack; 16] = [KernelStack {
            id: 0,
            data: [0; KERNEL_STACK_SIZE],
        }; MAX_TASK_NUM];
        unsafe {
            #[allow(static_mut_refs)]
            STACKS.iter_mut().enumerate().for_each(|(i, stack)| {
                stack.id = i;
            });
            STACKS
        }
    };
}

// region StackAllocator begin
pub struct StackAllocator {
    next: usize,
    recycle: Vec<usize>,
}

impl StackAllocator {
    pub fn new() -> Self {
        StackAllocator {
            next: 0,
            recycle: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Option<usize> {
        if let Some(index) = self.recycle.pop() {
            Some(index)
        } else if self.next < MAX_TASK_NUM {
            let index = self.next;
            self.next += 1;
            Some(index)
        } else {
            None
        }
    }

    pub fn dealloc(&mut self, index: usize) {
        self.recycle.push(index);
    }
}
// region StackAllocator end
