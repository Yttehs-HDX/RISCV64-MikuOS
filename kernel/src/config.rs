use lazy_static::lazy_static;

// timer
pub use crate::board::CLOCK_FREQ;

// task
pub const MAX_TASK_NUM: usize = 16;
pub const KERNEL_STACK_SIZE: usize = SV39_PAGE_SIZE;
pub const fn KERNEL_STACK_POS(app_id: usize) -> (usize, usize) {
    let top = MEMORY_END - app_id * (KERNEL_STACK_SIZE * SV39_PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

// trap
pub const USER_TRAMPOLINE: usize = usize::MAX - SV39_PAGE_SIZE + 1;
pub const USER_TRAP_CX: usize = USER_TRAMPOLINE - SV39_PAGE_SIZE;

// heap
pub const KERNEL_HEAP_SIZE: usize = 0x20000;

// memory mapping
pub const SV39_PAGE_SIZE: usize = 1 << 12; // 4096
pub const SV39_OFFSET_BITS: usize = 12;
pub const MEMORY_END: usize = 0x80800000;
lazy_static! {
    pub static ref PA_BEGIN: usize = ekernel as usize;
}
pub const PA_END: usize = MEMORY_END - MAX_TASK_NUM * (KERNEL_STACK_SIZE + SV39_PAGE_SIZE);

extern "C" {
    pub fn skernel();
    pub fn stext();
    pub fn etext();
    pub fn srodata();
    pub fn erodata();
    pub fn sdata();
    pub fn edata();
    pub fn sbss();
    pub fn sbss_no_stack();
    pub fn ebss();
    pub fn ekernel();
}
