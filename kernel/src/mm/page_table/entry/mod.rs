/* 64 bits PageTableEntry
 *
 *  63      54 53  28 27  19 18  10 9   8  7   6   5   4   3   2   1   0
 * +----------+------+------+------+-----+---+---+---+---+---+---+---+---+
 * | Reserved | PPN2 | PPN1 | PPN0 | RSW | D | A | G | U | X | W | R | V |
 * +----------+------+------+------+-----+---+---+---+---+---+---+---+---+
 *      10       26     9      9      2    1   1   1   1   1   1   1   1
 *
 */

pub use flags::*;

use crate::mm::{PhysPageNum, SV39_PPN_WIDTH};

mod flags;

pub const PTE_SIZE: usize = 64;
pub const PPN_START_BIT: usize = 10;

// region PageTableEntry begin
#[repr(C)]
pub struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self {
            bits: ppn.0 << PPN_START_BIT | flags.bits() as usize,
        }
    }

    pub fn ppn(&self) -> PhysPageNum {
        PhysPageNum(self.bits >> PPN_START_BIT & (1 << SV39_PPN_WIDTH - 1))
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits as u8)
    }

    pub fn has_flag(&self, flag: PTEFlags) -> bool {
        self.flags().contains(flag)
    }
}
// region PageTableEntry end