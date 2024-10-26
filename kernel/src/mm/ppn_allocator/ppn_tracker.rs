use crate::mm::PhysPageNum;

use super::dealloc_ppn;

// region PPNFrame begin
pub struct PPNTracker {
    pub ppn: PhysPageNum,
}

impl Drop for PPNTracker {
    fn drop(&mut self) {
        dealloc_ppn(self.ppn);
    }
}

impl PPNTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        ppn.get_bytes_array().fill(0);
        Self { ppn }
    }
}
// region PPNFrame end