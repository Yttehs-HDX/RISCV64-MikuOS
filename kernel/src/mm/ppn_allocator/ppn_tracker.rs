use crate::mm::{dealloc_ppn, PhysPageNum};

// region PpnTracker begin
pub struct PpnTracker {
    pub ppn: PhysPageNum,
}

impl Drop for PpnTracker {
    fn drop(&mut self) {
        dealloc_ppn(self.ppn);
    }
}

impl PpnTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // clear the page
        // ppn.as_bytes_array().fill(0);
        Self { ppn }
    }
}
// region PpnTracker end
