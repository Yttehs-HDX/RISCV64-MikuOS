pub use ppn_tracker::*;

use super::{PhysAddr, PhysPageNum};
use crate::{
    config::{PA_END, PA_START},
    sync::UPSafeCell,
};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::trace;

mod ppn_tracker;

pub fn alloc_ppn_tracker() -> Option<PPNTracker> {
    PPN_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(PPNTracker::new)
}

pub fn dealloc_ppn(ppn: PhysPageNum) {
    PPN_ALLOCATOR.exclusive_access().dealloc(ppn);
}

lazy_static! {
    static ref PPN_ALLOCATOR: UPSafeCell<PPNAllocator> =
        unsafe { UPSafeCell::new(PPNAllocator::new(*PA_START, PA_END)) };
}

// region PPNAllocator begin
struct PPNAllocator {
    current_ppn: usize,
    end_ppn: usize,
    recycled_frame: Vec<usize>,
}

impl PPNAllocator {
    fn new(mem_begin: usize, mem_end: usize) -> Self {
        let start_ppn = PhysAddr(mem_begin).to_ppn_ceil();
        let end_ppn = PhysAddr(mem_end).to_ppn_floor();
        trace!("PPNAllocator: Memory [{:#x}, {:#x})", mem_begin, mem_end);
        trace!("PPNAllocator: PPN [{:#x}, {:#x})", start_ppn.0, end_ppn.0);
        Self {
            current_ppn: start_ppn.0,
            end_ppn: end_ppn.0,
            recycled_frame: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled_frame.pop() {
            return Some(PhysPageNum(ppn));
        }
        if self.current_ppn == self.end_ppn {
            return None;
        }

        let ppn = self.current_ppn;
        self.current_ppn += 1;
        return Some(PhysPageNum(ppn));
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        assert!(
            ppn < self.current_ppn || !self.recycled_frame.contains(&ppn),
            "PPNAllocator: dealloc an unallocated frame"
        );
        self.recycled_frame.push(ppn);
    }
}
// region PPNAllocator end
