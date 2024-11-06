pub use exit_handle::*;

pub const CLOCK_FREQ: usize = 12500000;
pub const MEMORY_END: usize = 0x88000000;
pub const MMIO: &[(usize, usize)] = &[
    // (addr, len)
    (VIRT_TEST as usize, 0x2000),
];

mod exit_handle;
