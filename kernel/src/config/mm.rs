use super::*;

use lazy_static::lazy_static;

// heap
pub const KERNEL_HEAP_SIZE: usize = 0x200000; // 2 MB

// memory mapping
pub use crate::board::MMIO;
pub use crate::entry::KERNEL_ADDR_OFFSET;
pub const SV39_PAGE_OFFSET: usize = 12;
pub const SV39_PAGE_SIZE: usize = 1 << SV39_PAGE_OFFSET; // 4 KB

const MEMORY_END: usize = crate::board::MEMORY_END + KERNEL_ADDR_OFFSET; // 0xffff_ffff_c800_0000
pub const TRAP_CX_PTR: usize = MEMORY_END - SV39_PAGE_SIZE;
pub const USER_STACK_TOP: usize = TRAP_CX_PTR - (USER_STACK_SIZE + SV39_PAGE_SIZE);
pub const USER_STACK_BOTTOM: usize = USER_STACK_TOP + (USER_STACK_SIZE + SV39_PAGE_SIZE);
pub const USER_STACK_SP: usize = USER_STACK_TOP + USER_STACK_SIZE;

// kernel space
// left a guard page for kernel stack
pub const KERNEL_STACK_TOP: usize = USER_STACK_TOP - (KERNEL_STACK_SIZE + SV39_PAGE_SIZE);
pub const KERNEL_STACK_SP: usize = KERNEL_STACK_TOP + KERNEL_STACK_SIZE;
pub const PA_END: usize = KERNEL_STACK_TOP;
lazy_static! {
    pub static ref PA_START: usize = *EKERNEL;
}
