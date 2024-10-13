/* 56 bits Physical Address
 *
 *  55 30  29  21 20  12 11          0
 * +------+------+------+-------------+
 * | PPN2 | PPN1 | PPN0 | Page Offset |
 * +------+------+------+-------------+
 *    26     9      9         12
 *
 */

use crate::{config::{PAGE_OFFSET, PAGE_SIZE}, mm::{PageTableEntry, PTE_SIZE}};

pub const SV39_PPN_WIDTH: usize = 44;
pub const SV39_PA_WIDTH: usize = SV39_PPN_WIDTH + PAGE_SIZE; // 56

// region PhysAddr begin
pub struct PhysAddr(pub usize);

impl PhysAddr {
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }

    pub fn aligned(&self) -> bool { self.page_offset() == 0 }

    pub fn floor(&self) -> PhysAddr { PhysAddr(self.0 & !(PAGE_SIZE - 1)) }

    pub fn floor_page(&self) -> PhysPageNum { self.floor().ppn() }

    pub fn ceil(&self) -> PhysAddr { PhysAddr((self.0 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) }

    pub fn ceil_page(&self) -> PhysPageNum { self.ceil().ppn() }

    pub fn ppn(&self) -> PhysPageNum { PhysPageNum(self.0 / PAGE_SIZE) }
}
// region PhysAddr end

// region PhysPageNum begin
#[derive(Clone, Copy)]
pub struct PhysPageNum(pub usize);

impl PhysPageNum {
    pub fn pa(&self) -> PhysAddr { PhysAddr(self.0 << PAGE_OFFSET) }

    pub fn as_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa = self.pa();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                PAGE_SIZE / PTE_SIZE
            )
        }
    }

    pub fn as_bytes_array(&self) -> &'static mut [u8] {
        let pa = self.pa();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut u8,
                PAGE_SIZE
            )
        }
    }

    pub fn as_mut<T>(&self) -> &'static mut T {
        let pa = self.pa();
        unsafe {
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}
// region PhysPageNum end