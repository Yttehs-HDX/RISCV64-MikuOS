use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::debug;
use crate::{config::{KERNEL_STACK_SIZE, MAX_TASK_NUM, USER_STACK_SIZE}, sync::UPSafeCell};

pub fn get_kernel_stack() -> &'static KernelStack {
    let index = KERNEL_STACK_ALLOCATOR.exclusive_access().alloc().unwrap();
    debug!("KernelStack: get stack {}", index);
    &KERNEL_STACKS[index]
}

pub fn get_user_stack() -> &'static UserStack {
    let index = USER_STACK_ALLOCATOR.exclusive_access().alloc().unwrap();
    debug!("UserStack: get stack {}", index);
    &USER_STACKS[index]
}

lazy_static! {
    pub static ref USER_STACK_ALLOCATOR: UPSafeCell<StackAllocator> = unsafe {
        UPSafeCell::new(StackAllocator::new())
    };

    pub static ref KERNEL_STACK_ALLOCATOR: UPSafeCell<StackAllocator> = unsafe {
        UPSafeCell::new(StackAllocator::new())
    };

    static ref USER_STACKS: [UserStack; MAX_TASK_NUM] = {
        static mut STACKS: [UserStack; 16] = [UserStack { id: 0, data:[0;USER_STACK_SIZE] }; MAX_TASK_NUM];
        unsafe {
            #[allow(static_mut_refs)]
            STACKS.iter_mut().enumerate().for_each( |(i, stack)| {
                stack.id = i;
            });
            STACKS
        }
    };

    static ref KERNEL_STACKS: [KernelStack; MAX_TASK_NUM] = {
        static mut STACKS: [KernelStack; 16] = [KernelStack { id: 0, data:[0;KERNEL_STACK_SIZE] }; MAX_TASK_NUM];
        unsafe {
            #[allow(static_mut_refs)]
            STACKS.iter_mut().enumerate().for_each( |(i, stack)| {
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

// region UserStack begin
#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct UserStack {
    id: usize,
    data: [u8; USER_STACK_SIZE],
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
    id: usize,
    data: [u8; KERNEL_STACK_SIZE],
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