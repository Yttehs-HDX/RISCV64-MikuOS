/* 56 bits Physical Address
 *
 *  55 30  29  21 20  12 11          0
 * +------+------+------+-------------+
 * | PPN2 | PPN1 | PPN0 | Page Offset |
 * +------+------+------+-------------+
 *    26     9      9         12
 *
 */

use crate::config::{PAGE_OFFSET, PAGE_SIZE};

const SV39_PPN_WIDTH: usize = 44;
const SV39_PA_WIDTH: usize = SV39_PPN_WIDTH + PAGE_SIZE; // 56

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

impl From<PhysPageNum> for PhysAddr {
    fn from(value: PhysPageNum) -> Self {
        value.pa()
    }
}
// region PhysAddr end

// region PhysPageNum begin
pub struct PhysPageNum(pub usize);

impl PhysPageNum {
    pub fn pa(&self) -> PhysAddr { PhysAddr(self.0 << PAGE_OFFSET) }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(value: PhysAddr) -> Self {
        assert!(value.aligned());
        value.ppn()
    }
}
// region PhysPageNum end