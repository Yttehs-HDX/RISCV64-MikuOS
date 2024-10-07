pub const APP_BASE_ADDR: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const MAX_TASK_NUM: usize = 16;

pub const KERNEL_STACK_SIZE: usize = 4096;
pub const USER_STACK_SIZE: usize = 4096;

pub const KERNEL_HEAP_SIZE: usize = 0x20000;

pub use crate::board::CLOCK_FREQ;