/* SV39 Physical Address: 56 bits
 *
 * |56         31|30    22|21    12|11     0|
 * |    PPN[2]   | PPN[1] | PPN[0] | Offset |
 * |-------------|--------|--------|--------|
 * |     26      |   9    |   9    |   12   |
 *
 * | <--------- PhysPageNum -----> | 44 bits
 * | <--------- PhysAddr -----------------> | 56 bits
 */

use crate::{
    config::{SV39_OFFSET_BITS, SV39_PAGE_SIZE},
    mm::{PageTableEntry, SV39_PTE_BITS},
};

pub const SV39_PPN_BITS: usize = 44;

// region PhysAddr begin
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

impl PhysAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & (SV39_PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }

    pub fn to_ppn(self) -> PhysPageNum {
        assert!(self.aligned());
        PhysPageNum(self.0 >> SV39_OFFSET_BITS)
    }
    pub fn to_ppn_floor(self) -> PhysPageNum {
        PhysPageNum(self.0 >> SV39_OFFSET_BITS)
    }
    pub fn to_ppn_ceil(self) -> PhysPageNum {
        PhysPageNum((self.0 + SV39_PAGE_SIZE - 1) >> SV39_OFFSET_BITS)
    }
}
// region PhysAddr end

// region PhysPageNum begin
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysPageNum(pub usize);

impl PhysPageNum {
    pub fn to_pa(self) -> PhysAddr {
        PhysAddr(self.0 << SV39_OFFSET_BITS)
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa = self.to_pa();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, SV39_PAGE_SIZE) }
    }

    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa = self.to_pa();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                SV39_PAGE_SIZE / (SV39_PTE_BITS / 8),
            )
        }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa = self.to_pa();
        unsafe { (pa.0 as *mut T).as_mut().unwrap() }
    }
}
// region PhysPageNum end
