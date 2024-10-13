use crate::mm::PhysPageNum;
use super::dealloc_frame;

// region FrameTracker begin
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        dealloc_frame(self.ppn);
    }
}

impl FrameTracker {
    pub fn new(page_number: usize) -> Self { Self { ppn: PhysPageNum(page_number) } }

    pub fn clear_ppn(&mut self) {
        self.ppn.as_bytes_array().iter_mut().for_each( |p| {
            *p = 0;
        });
    }
}
// region FrameTracker end