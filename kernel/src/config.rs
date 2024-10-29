pub fn kernel_start() -> usize {
    skernel as usize
}
pub fn text_start() -> usize {
    stext as usize
}
pub fn text_end() -> usize {
    etext as usize
}
pub fn rodata_start() -> usize {
    srodata as usize
}
pub fn rodata_end() -> usize {
    erodata as usize
}
pub fn data_start() -> usize {
    sdata as usize
}
pub fn data_end() -> usize {
    edata as usize
}
pub fn bss_start() -> usize {
    sbss as usize
}
pub fn bss_start_stackless() -> usize {
    sbss_no_stack as usize
}
pub fn bss_end() -> usize {
    ebss as usize
}
pub fn kernel_end() -> usize {
    ekernel as usize
}

#[deprecated]
pub const APP_BASE_ADDR: usize = 0x80400000;
#[deprecated]
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const MAX_TASK_NUM: usize = 16;

#[deprecated]
pub const KERNEL_STACK_SIZE: usize = 4096;
#[deprecated]
pub const USER_STACK_SIZE: usize = 4096;

// heap
pub const KERNEL_HEAP_SIZE: usize = 0x20000;

// timer
pub use crate::board::CLOCK_FREQ;

// memory mapping
pub const SV39_PAGE_SIZE: usize = 1 << 12; // 4096
pub const SV39_OFFSET_BITS: usize = 12;
pub const MEMORY_END: usize = 0x80800000;

// trap
pub const USER_TRAMPOLINE: usize = usize::MAX - SV39_PAGE_SIZE + 1;
pub const USER_TRAP_CX: usize = USER_TRAMPOLINE - SV39_PAGE_SIZE;

extern "C" {
    fn skernel();
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn sbss_no_stack();
    fn ebss();
    fn ekernel();
}
