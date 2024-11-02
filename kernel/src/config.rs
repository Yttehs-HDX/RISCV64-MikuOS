use lazy_static::lazy_static;

// timer
pub use crate::board::CLOCK_FREQ;

// task
pub const MAX_TASK_NUM: usize = 16;
pub const KERNEL_STACK_SIZE: usize = SV39_PAGE_SIZE;

// heap
pub const KERNEL_HEAP_SIZE: usize = 0x20000;

// memory mapping
pub const SV39_PAGE_SIZE: usize = 1 << 12; // 4096
pub const SV39_OFFSET_BITS: usize = 12;

// physical address
pub use crate::board::MEMORY_END;
pub const fn KERNEL_STACK_POS(app_id: usize) -> (usize, usize) {
    // left guard pages between kernel stacks
    let top = MEMORY_END - app_id * (KERNEL_STACK_SIZE + SV39_PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
lazy_static! {
    pub static ref PA_START: usize = *EKERNEL;
}
pub const PA_END: usize = MEMORY_END - MAX_TASK_NUM * (KERNEL_STACK_SIZE + SV39_PAGE_SIZE);

// virtual address
pub const TRAMPOLINE: usize = usize::MAX - SV39_PAGE_SIZE + 1;
pub const TRAP_CX_PTR: usize = TRAMPOLINE - SV39_PAGE_SIZE;
// left a guard page for user stack
pub const USER_STACK_BOTTOM: usize = TRAP_CX_PTR - 2 * SV39_PAGE_SIZE;

// sections boundary
lazy_static! {
    pub static ref SKERNEL: usize = skernel as usize;
    pub static ref STEXT: usize = stext as usize;
    pub static ref STRAMPOLINE: usize = strampoline as usize;
    pub static ref ETEXT: usize = etext as usize;
    pub static ref SRODATA: usize = srodata as usize;
    pub static ref ERODATA: usize = erodata as usize;
    pub static ref SDATA: usize = sdata as usize;
    pub static ref EDATA: usize = edata as usize;
    pub static ref SBSS: usize = sbss as usize;
    pub static ref SBSS_NO_STACK: usize = sbss_no_stack as usize;
    pub static ref EBSS: usize = ebss as usize;
    pub static ref EKERNEL: usize = ekernel as usize;
}

extern "C" {
    fn skernel();
    fn stext();
    fn strampoline();
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
