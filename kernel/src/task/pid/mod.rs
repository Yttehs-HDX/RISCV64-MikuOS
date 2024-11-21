pub use handle::*;

use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use lazy_static::lazy_static;

mod handle;

pub fn alloc_pid_handle() -> PidHandle {
    PID_ALLOCATOR.alloc()
}

pub fn dealloc_pid(pid: usize) {
    PID_ALLOCATOR.dealloc(pid);
}

lazy_static! {
    static ref PID_ALLOCATOR: PidAllocator = PidAllocator::new();
}

// region PidAllocator begin
struct PidAllocator {
    inner: UPSafeCell<PidAllocatorInner>,
}

impl PidAllocator {
    fn new() -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(PidAllocatorInner {
                    current: 0,
                    recycled: Vec::new(),
                })
            },
        }
    }

    fn contains(&self, pid: usize) -> bool {
        let inner = self.inner.shared_access();
        pid < inner.current || !inner.recycled.contains(&pid)
    }

    fn alloc(&self) -> PidHandle {
        let mut inner = self.inner.exclusive_access();
        if let Some(pid) = inner.recycled.pop() {
            return PidHandle(pid);
        }

        let pid = inner.current;
        inner.current += 1;
        PidHandle(pid)
    }

    fn dealloc(&self, pid: usize) {
        assert!(
            self.contains(pid),
            "PidAllocator: pid {} is not allocated",
            pid
        );
        let mut inner = self.inner.exclusive_access();
        inner.recycled.push(pid);
    }
}
// region PidAllocator end

// region PidAllocatorInner begin
struct PidAllocatorInner {
    current: usize,
    recycled: Vec<usize>,
}
// region PidAllocatorInner end
