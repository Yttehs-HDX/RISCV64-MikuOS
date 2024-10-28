pub use map_permission::*;
pub use map_type::*;

use crate::{
    config::SV39_PAGE_SIZE,
    mm::{alloc_ppn_tracker, PPNTracker, PTEFlags, PageTable, PhysPageNum, VirtAddr, VirtPageNum},
    util::{SimpleRange, StepByOne},
};
use alloc::collections::btree_map::BTreeMap;

mod map_permission;
mod map_type;

// region MapArea begin
pub struct MapArea {
    vpn_range: SimpleRange<VirtPageNum>,
    ppn_map: BTreeMap<VirtPageNum, PPNTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
    ) -> Self {
        let vpn_range = SimpleRange::new(start_va.to_vpn_floor(), end_va.to_vpn_ceil());
        Self {
            vpn_range,
            ppn_map: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }

    pub fn map_one(&mut self, vpn: VirtPageNum, page_table: &mut PageTable) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identity => ppn = PhysPageNum(vpn.0),
            MapType::Framed => {
                let ppn_tracker = alloc_ppn_tracker().unwrap();
                ppn = ppn_tracker.ppn;
                self.ppn_map.insert(vpn, ppn_tracker);
            }
        }
        page_table.map(vpn, ppn, PTEFlags::from_bits_truncate(self.map_perm.bits()));
    }

    pub fn unmap_one(&mut self, vpn: VirtPageNum, page_table: &mut PageTable) {
        match self.map_type {
            MapType::Identity => {}
            MapType::Framed => {
                self.ppn_map.remove(&vpn);
            }
        }
        page_table.unmap(vpn);
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(vpn, page_table);
        }
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(vpn, page_table);
        }
    }
}

impl MapArea {
    pub fn insert_raw_data(&self, data: &[u8], page_table: &mut PageTable) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut current_vpn = self.vpn_range.start();
        let mut data_start = 0;
        let data_end = data.len();

        loop {
            let src = &data[data_start..data_end];
            let dst = &mut page_table
                .tranlate(current_vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[..src.len()];
            dst.copy_from_slice(src);

            data_start += SV39_PAGE_SIZE;
            if data_start >= data_end {
                break;
            }
            current_vpn.step();
        }
    }
}
// region MapArea end
