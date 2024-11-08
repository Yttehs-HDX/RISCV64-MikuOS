use lazy_static::lazy_static;

// timer
pub use crate::board::CLOCK_FREQ;

// task
pub const USER_STACK_SIZE: usize = 0x200000; // 2 MB
pub const KERNEL_STACK_SIZE: usize = SV39_PAGE_SIZE;

// heap
pub const KERNEL_HEAP_SIZE: usize = 0x200000; // 2 MB

// memory mapping
pub const SV39_PAGE_SIZE: usize = 1 << 12; // 4 KB
pub const SV39_OFFSET_BITS: usize = 12;

// physical address
pub use crate::board::MEMORY_END;
pub use crate::board::MMIO;
lazy_static! {
    pub static ref PA_START: usize = *EKERNEL;
}
pub const PA_END: usize = MEMORY_END;

// virtual address
pub const TRAMPOLINE: usize = usize::MAX - SV39_PAGE_SIZE + 1;
pub const TRAP_CX_PTR: usize = TRAMPOLINE - SV39_PAGE_SIZE;
// left a guard page for user stack
pub const USER_STACK_TOP: usize = TRAP_CX_PTR - (USER_STACK_SIZE + SV39_PAGE_SIZE);
pub const fn kernel_stack_top(pid: usize) -> usize {
    // left guard pages between kernel stacks
    let bottom = USER_STACK_TOP - pid * (KERNEL_STACK_SIZE + SV39_PAGE_SIZE);
    bottom - KERNEL_STACK_SIZE
}

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
