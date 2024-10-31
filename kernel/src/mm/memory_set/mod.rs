pub use map_area::*;

use super::PageTable;
use alloc::vec::Vec;

mod map_area;

// region MemorySet begin
pub struct MemorySet {
    pub page_table: PageTable,
    pub areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn get_satp(&self) -> usize {
        self.page_table.as_satp()
    }
}
// region MemorySet end
