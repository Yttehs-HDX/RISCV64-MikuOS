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

pub fn alloc_ppn_tracker() -> Option<PpnTracker> {
    PPN_ALLOCATOR.alloc().map(PpnTracker::new)
}

pub fn dealloc_ppn(ppn: PhysPageNum) {
    PPN_ALLOCATOR.dealloc(ppn);
}

lazy_static! {
    static ref PPN_ALLOCATOR: PpnAllocator = PpnAllocator::new(*PA_START, PA_END);
}

// region PpnAllocator begin
struct PpnAllocator {
    inner: UPSafeCell<PpnAllocatorInner>,
}

impl PpnAllocator {
    fn new(pa_begin: usize, pa_end: usize) -> Self {
        let start_ppn = PhysAddr(pa_begin).to_ppn_ceil();
        let end_ppn = PhysAddr(pa_end).to_ppn_floor();
        trace!("PpnAllocator: PA  [{:#x}, {:#x})", pa_begin, pa_end);
        trace!("PpnAllocator: PPN [{:#x}, {:#x})", start_ppn.0, end_ppn.0);
        Self {
            inner: unsafe {
                UPSafeCell::new(PpnAllocatorInner {
                    ppn_start: start_ppn.0,
                    ppn_end: end_ppn.0,
                    recycled_ppn: Vec::new(),
                })
            },
        }
    }

    fn alloc(&self) -> Option<PhysPageNum> {
        let mut inner = self.inner.exclusive_access();
        if let Some(ppn) = inner.recycled_ppn.pop() {
            return Some(PhysPageNum(ppn));
        }
        if inner.ppn_start == inner.ppn_end {
            return None;
        }

        let ppn = inner.ppn_start;
        inner.ppn_start += 1;
        Some(PhysPageNum(ppn))
    }

    fn dealloc(&self, ppn: PhysPageNum) {
        let mut inner = self.inner.exclusive_access();
        let ppn = ppn.0;
        assert!(
            ppn < inner.ppn_start || !inner.recycled_ppn.contains(&ppn),
            "PpnAllocator: dealloc an unallocated ppn"
        );
        inner.recycled_ppn.push(ppn);
    }
}
// region PpnAllocator end

// region PpnAllocatorInner begin
struct PpnAllocatorInner {
    ppn_start: usize,
    ppn_end: usize,
    recycled_ppn: Vec<usize>,
}
// region PpnAllocatorInner end
