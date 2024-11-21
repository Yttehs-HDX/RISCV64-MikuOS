use crate::mm::{dealloc_ppn, PhysPageNum};

// region PpnTracker begin
pub struct PpnTracker {
    ppn: PhysPageNum,
}

impl Drop for PpnTracker {
    fn drop(&mut self) {
        dealloc_ppn(self.ppn);
    }
}

impl PpnTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // clear the page
        ppn.as_bytes_array().fill(0);
        Self { ppn }
    }

    pub fn ppn(&self) -> PhysPageNum {
        self.ppn
    }
}
// region PpnTracker end
