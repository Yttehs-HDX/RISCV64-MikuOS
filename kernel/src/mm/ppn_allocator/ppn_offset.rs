use crate::{config::SV39_PAGE_OFFSET, entry::KERNEL_ADDR_OFFSET, mm::PhysPageNum};

pub trait PpnOffset {
    fn high_to_low(&self) -> PhysPageNum;
    fn low_to_high(&self) -> PhysPageNum;
}

impl PpnOffset for PhysPageNum {
    fn high_to_low(&self) -> PhysPageNum {
        PhysPageNum(self.0 - (KERNEL_ADDR_OFFSET >> SV39_PAGE_OFFSET))
    }

    fn low_to_high(&self) -> PhysPageNum {
        PhysPageNum(self.0 + (KERNEL_ADDR_OFFSET >> SV39_PAGE_OFFSET))
    }
}
