pub use exit_handle::*;

use crate::entry::KERNEL_ADDR_OFFSET;

pub const CLOCK_FREQ: usize = 12500000;
pub const MEMORY_END: usize = 0x88000000;
pub const MMIO: &[(usize, usize)] = &[
    // (addr, len)
    (VIRT_TEST as usize, 0x2000),
    (VIRT_IO, 0x1000),
];

mod exit_handle;

pub const VIRT_IO: usize = 0x10001000 + KERNEL_ADDR_OFFSET;
