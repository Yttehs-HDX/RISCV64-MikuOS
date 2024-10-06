use crate::{app::App, config::{KERNEL_STACK_SIZE, MAX_TASK_NUM, USER_STACK_SIZE}};

pub fn get_stack(stack_type: StackType, app: &App) -> usize {
    let index = app.no();
    match stack_type {
        StackType::User => USER_STACKS[index].get_sp(),
        StackType::Kernel => KERNEL_STACKS[index].get_sp(),
    }
}

static USER_STACKS: [UserStack; MAX_TASK_NUM] = [
    UserStack { data: [0; USER_STACK_SIZE] };
    MAX_TASK_NUM
];

static KERNEL_STACKS: [KernelStack; MAX_TASK_NUM] = [
    KernelStack { data: [0; KERNEL_STACK_SIZE] };
    MAX_TASK_NUM
];

// region UserStack begin
#[repr(align(4096))]
#[derive(Clone, Copy)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
// region UserStack end

// region KernelStack begin
#[repr(align(4096))]
#[derive(Clone, Copy)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}
// region KernelStack end

// region StackType begin
pub enum StackType {
    User,
    Kernel,
}
// region StackType end