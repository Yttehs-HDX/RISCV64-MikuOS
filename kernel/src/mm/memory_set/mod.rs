pub use map_area::*;

use super::PageTable;
use alloc::vec::Vec;

mod map_area;

// region MemorySet begin
pub struct MemorySet {
    pub page_table: PageTable,
    pub areas: Vec<MapArea>,
}
// region MemorySet end
