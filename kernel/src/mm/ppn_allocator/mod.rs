pub use ppn_offset::*;
pub use ppn_tracker::*;

use crate::{
    config::{PA_END, PA_START},
    mm::{PhysAddr, PhysPageNum},
    sync::UPSafeCell,
};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::trace;
use simple_range::{SimpleRange, StepByOne};

mod ppn_offset;
mod ppn_tracker;

pub fn alloc_ppn_tracker() -> Option<PpnTracker> {
    PPN_ALLOCATOR.alloc().map(PpnTracker::new)
}

pub fn alloc_contiguous_ppn_tracker(count: usize) -> Option<Vec<PpnTracker>> {
    PPN_ALLOCATOR
        .alloc_contiguous(count)
        .map(|ppns| ppns.into_iter().map(PpnTracker::new).collect())
}

pub fn dealloc_ppn(ppn: PhysPageNum) {
    PPN_ALLOCATOR.dealloc(ppn);
}

pub fn dealloc_contiguous_ppn(start: PhysPageNum, count: usize) {
    for i in 0..count {
        dealloc_ppn(PhysPageNum(start.0 + i));
    }
}

lazy_static! {
    static ref PPN_ALLOCATOR: PpnAllocator =
        PpnAllocator::new(PhysAddr(*PA_START), PhysAddr(PA_END));
}

// region PpnAllocator begin
struct PpnAllocator {
    inner: UPSafeCell<PpnAllocatorInner>,
}

impl PpnAllocator {
    fn new(pa_begin: PhysAddr, pa_end: PhysAddr) -> Self {
        let start_ppn = pa_begin.to_ppn_ceil();
        let end_ppn = pa_end.to_ppn_floor();
        trace!("PpnAllocator: PA  [{:#x}, {:#x})", pa_begin.0, pa_end.0);
        trace!("PpnAllocator: PPN [{:#x}, {:#x})", start_ppn.0, end_ppn.0);
        Self {
            inner: unsafe {
                UPSafeCell::new(PpnAllocatorInner {
                    ppn_range: SimpleRange::new(start_ppn, end_ppn),
                    recycled_ppn: Vec::new(),
                })
            },
        }
    }

    fn contains(&self, ppn: PhysPageNum) -> bool {
        self.inner.shared_access().contains(ppn)
    }

    fn alloc(&self) -> Option<PhysPageNum> {
        let mut inner = self.inner.exclusive_access();
        if let Some(ppn) = inner.recycled_ppn.pop() {
            return Some(ppn);
        }
        if inner.ppn_range.start() == inner.ppn_range.end() {
            return None;
        }

        let ppn = inner.ppn_range.start();
        inner.ppn_range.get_start_mut().step();
        Some(ppn)
    }

    fn alloc_contiguous(&self, count: usize) -> Option<Vec<PhysPageNum>> {
        let mut inner = self.inner.exclusive_access();
        if inner.ppn_range.start().0 + count >= inner.ppn_range.end().0 {
            None
        } else {
            {
                let start = inner.ppn_range.start();
                *inner.ppn_range.get_start_mut() = PhysPageNum(start.0 + count);
            }
            let arr: Vec<usize> = (1..count + 1).collect();
            let v = arr
                .iter()
                .map(|x| PhysPageNum(inner.ppn_range.start().0 - x))
                .collect();
            Some(v)
        }
    }

    fn dealloc(&self, ppn: PhysPageNum) {
        assert!(
            self.contains(ppn),
            "PpnAllocator: dealloc an unallocated ppn"
        );
        let mut inner = self.inner.exclusive_access();
        inner.recycled_ppn.push(ppn);
    }
}
// region PpnAllocator end

// region PpnAllocatorInner begin
struct PpnAllocatorInner {
    ppn_range: SimpleRange<PhysPageNum>,
    recycled_ppn: Vec<PhysPageNum>,
}

impl PpnAllocatorInner {
    fn contains(&self, ppn: PhysPageNum) -> bool {
        ppn < self.ppn_range.start() && !self.recycled_ppn.contains(&ppn)
    }
}
// region PpnAllocatorInner end
