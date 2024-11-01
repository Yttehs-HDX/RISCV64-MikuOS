pub use map_area::*;

use super::{PTEFlags, PageTable, PhysAddr, VirtAddr};
use crate::{EBSS, EDATA, ERODATA, ETEXT, SBSS, SDATA, SRODATA, STEXT, STRAMPOLINE, TRAMPOLINE};
use alloc::vec::Vec;

mod map_area;

// region MemorySet begin
pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    fn empty() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    fn insert_area(&mut self, area: MapArea) {
        self.areas.push(area);
    }

    fn inser_area_with_data(&mut self, area: MapArea, data: &[u8]) {
        area.insert_raw_data(data, &mut self.page_table);
        self.insert_area(area);
    }

    fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr(TRAMPOLINE).to_vpn(),
            PhysAddr(*STRAMPOLINE).to_ppn(),
            PTEFlags::R | PTEFlags::X,
        );
    }
}

impl MemorySet {
    pub fn new_kernel() -> Self {
        let mut memory_set = Self::empty();

        // map trampoline
        memory_set.map_trampoline();

        // map sections
        memory_set.insert_area(MapArea::new(
            VirtAddr(*STEXT),
            VirtAddr(*ETEXT),
            MapType::Identity,
            MapPermission::R | MapPermission::X,
        ));
        memory_set.insert_area(MapArea::new(
            VirtAddr(*SRODATA),
            VirtAddr(*ERODATA),
            MapType::Identity,
            MapPermission::R,
        ));
        memory_set.insert_area(MapArea::new(
            VirtAddr(*SDATA),
            VirtAddr(*EDATA),
            MapType::Identity,
            MapPermission::R | MapPermission::W,
        ));
        memory_set.insert_area(MapArea::new(
            VirtAddr(*SBSS),
            VirtAddr(*EBSS),
            MapType::Identity,
            MapPermission::R | MapPermission::W,
        ));

        memory_set
    }

    pub fn get_satp(&self) -> usize {
        self.page_table.as_satp()
    }
}
// region MemorySet end
