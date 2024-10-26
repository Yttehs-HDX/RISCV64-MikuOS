/* PageTableEntry: 64 bits
 * 
 * |64      54|53         28|27    19|18    10|9   8|7              0|
 * | Reserved |   PPN[2]    | PPN[1] | PPN[0] | RSW |     Flags      |
 * |----------|-------------|--------|--------|-----|----------------|
 * |    10    |     26      |   9    |   9    |  2  |       8        |
 * 
 *            | <------- PhysPageNum -------> | 44 bits
 *                                                  | <- PTEFlags -> | 8 bits
 */

pub use flags::*;

use crate::mm::{PhysPageNum, SV39_PPN_BITS};

mod flags;

pub const SV39_PTE_BITS: usize = 64;

// region PageTableEntry begin
#[repr(C)]
pub struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    pub fn empty() -> Self { Self { bits: 0 } }

    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self { bits: ppn.0 << 10 | flags.bits() as usize }
    }

    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum(self.bits >> 10 & ((1 << SV39_PPN_BITS) - 1))
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits as u8)
    }

    pub fn is_valid(&self) -> bool {
        self.flags().contains(PTEFlags::V)
    }
}
// region PageTableEntry end