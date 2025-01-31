pub use map_permission::*;
pub use map_type::*;

use crate::{
    config::SV39_PAGE_SIZE,
    mm::{alloc_ppn_tracker, PageTable, PhysPageNum, PpnOffset, PpnTracker, VirtAddr, VirtPageNum},
};
use alloc::collections::btree_map::BTreeMap;
use core::cmp::Ordering;
use simple_range::{SimpleRange, StepByOne};

mod map_permission;
mod map_type;

// region MapArea begin
pub struct MapArea {
    pub vpn_range: SimpleRange<VirtPageNum>,
    ppn_map: BTreeMap<VirtPageNum, PpnTracker>,
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

    pub fn from_another(another: &Self) -> Self {
        Self {
            vpn_range: another.vpn_range,
            ppn_map: BTreeMap::new(),
            map_type: another.map_type,
            map_perm: another.map_perm,
        }
    }
}

impl MapArea {
    pub fn map_one(&mut self, vpn: VirtPageNum, page_table: &mut PageTable) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Direct => ppn = PhysPageNum(vpn.0).high_to_low(),
            MapType::Framed => {
                let ppn_tracker = alloc_ppn_tracker().unwrap();
                ppn = ppn_tracker.ppn().high_to_low();
                self.ppn_map.insert(vpn, ppn_tracker);
            }
        }
        page_table.map(vpn, ppn, self.map_perm.as_pteflags());
    }

    pub fn unmap_one(&mut self, vpn: VirtPageNum, page_table: &mut PageTable) {
        match self.map_type {
            MapType::Direct => {}
            MapType::Framed => {
                self.ppn_map.remove(&vpn);
            }
        }
        page_table.unmap(vpn);
    }

    pub fn change_vpn_end(&mut self, new_end_vpn: VirtPageNum, page_table: &mut PageTable) {
        match new_end_vpn.cmp(&self.vpn_range.end()) {
            Ordering::Less => {
                for vpn in SimpleRange::new(new_end_vpn, self.vpn_range.end()) {
                    self.unmap_one(vpn, page_table);
                }
            }
            Ordering::Equal => {
                return;
            }
            Ordering::Greater => {
                for vpn in SimpleRange::new(self.vpn_range.end(), new_end_vpn) {
                    self.map_one(vpn, page_table);
                }
            }
        }
        self.vpn_range = SimpleRange::new(self.vpn_range.start(), new_end_vpn);
    }

    pub fn map_all(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(vpn, page_table);
        }
    }

    pub fn unmap_all(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(vpn, page_table);
        }
    }
}

impl MapArea {
    pub fn get_type(&self) -> MapType {
        self.map_type
    }

    pub fn insert_raw_data(&self, data: &[u8], page_table: &mut PageTable) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut current_vpn = self.vpn_range.start();
        let mut data_start = 0;
        let data_end = data.len();

        loop {
            let src = &data[data_start..data_end.min(data_start + SV39_PAGE_SIZE)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .low_to_high()
                .as_bytes_array()[..src.len()];
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
