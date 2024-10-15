use alloc::collections::btree_map::BTreeMap;
use riscv::addr::page;
use crate::{config::PAGE_SIZE, util::{SimpleRange, StepByOne}};
use super::{alloc_frame, FrameTracker, PageTable, PhysPageNum, VirtAddr, VirtPageNum};

pub use permission::*;
pub use option::*;

mod permission;
mod option;

// region MapArea begin
pub struct MapArea {
    vpn_range: SimpleRange<VirtPageNum>,
    frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_option: MapOption,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(start_va: VirtAddr, end_va: VirtAddr, option: MapOption, perm: MapPermission) -> Self {
        let start_vpn = start_va.floor().vpn();
        let end_vpn = end_va.ceil().vpn();
        Self {
            vpn_range: SimpleRange::new(start_vpn, end_vpn),
            frames: BTreeMap::new(),
            map_option: option,
            map_perm: perm,
        }
    }

    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_option {
            MapOption::Identity => {
                ppn = PhysPageNum(vpn.0);
            }
            MapOption::Framed => {
                let frame = alloc_frame().unwrap();
                ppn = frame.ppn;
                self.frames.insert(vpn, frame);
            }
        }
        let pte_flags = self.map_perm.to_pte_flags();
        page_table.map(vpn, ppn, pte_flags);
    }

    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_option == MapOption::Framed {
            self.frames.remove(&vpn);
        }
        page_table.unmap(vpn);
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn);
        }
    }

    pub fn map_raw_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert!(
            self.map_option == MapOption::Framed,
            "MapArea: map_raw_data only works for MapOption::Framed"
        );
        let mut current_vpn = self.vpn_range.get_start();
        let (mut start, end) = (0usize, data.len());
        loop {
            let src = &data[start..end.min(start + PAGE_SIZE)];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .as_bytes_array()[..src.len()];
            dst.copy_from_slice(src);

            start += src.len(); if start >= end { break; }
            current_vpn.step();
        }
    }
}
// region MapArea end