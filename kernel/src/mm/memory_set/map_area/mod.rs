pub use map_permission::*;
pub use map_type::*;

use crate::{
    mm::{alloc_ppn_tracker, PPNTracker, PTEFlags, PageTable, PhysPageNum, VirtAddr, VirtPageNum},
    util::SimpleRange,
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
}
// region MapArea end
