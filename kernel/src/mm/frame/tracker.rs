use crate::mm::PhysPageNum;

// region FrameTracker begin
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(page_number: usize) -> Self {
        let ppn = PhysPageNum(page_number);
        ppn.as_bytes_array().iter_mut().for_each( |p| {
            *p = 0;
        });
        Self { ppn }
    }
}
// region FrameTracker end