use lazy_static::lazy_static;
use alloc::vec::Vec;
use crate::{config::MEMORY_END, sync::UPSafeCell};
use super::PhysAddr;

pub use tracker::*;

pub fn alloc_frame() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.alloc()
}

pub fn dealloc_frame(frame: FrameTracker) {
    FRAME_ALLOCATOR.dealloc(frame);
}

mod tracker;

lazy_static! {
    static ref FRAME_ALLOCATOR: FrameAllocator = FrameAllocator::new(
        PhysAddr(crate::kernel_end as usize),
        PhysAddr(MEMORY_END),
    );
}

// region FrameAllocator begin
struct FrameAllocator {
    inner: UPSafeCell<FrameAllocatorInner>,
}

impl FrameAllocator {
    fn new(pa_start: PhysAddr, pa_end: PhysAddr) -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(
                    FrameAllocatorInner {
                        current: pa_start.ppn().0,
                        end: pa_end.ppn().0,
                        free_frames: Vec::new(),
                    }
                )
            }
        }
    }

    fn alloc(&self) -> Option<FrameTracker> {
        let mut inner = self.inner.exclusive_access();
        if let Some(frame) = inner.free_frames.pop() {
            return Some(frame);
        }
        if inner.current < inner.end {
            let frame = FrameTracker::new(inner.current);
            inner.current += 1;
            return Some(frame);
        }
        None
    }

    fn dealloc(&self, frame: FrameTracker) {
        let mut inner = self.inner.exclusive_access();
        inner.free_frames.push(frame);
    }
}
// region FrameAllocator end

// region FrameAllocatorInner begin
struct FrameAllocatorInner {
    current: usize,
    end: usize,
    free_frames: Vec<FrameTracker>,
}
// region FrameAllocatorInner end