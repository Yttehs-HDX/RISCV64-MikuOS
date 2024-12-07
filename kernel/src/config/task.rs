use super::*;

pub const USER_STACK_SIZE: usize = 0x200000; // 2 MB
pub const KERNEL_STACK_SIZE: usize = SV39_PAGE_SIZE;
