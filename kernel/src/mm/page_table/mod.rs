pub use entry::*;

use crate::mm::{alloc_ppn_tracker, PhysPageNum, PpnOffset, PpnTracker, VirtPageNum};
use alloc::vec;
use alloc::vec::Vec;

mod entry;

// region PageTable begin
pub struct PageTable {
    root_ppn: PhysPageNum,
    ppn_tracker_list: Vec<PpnTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let root_ppn_tracker = alloc_ppn_tracker().unwrap();
        Self {
            root_ppn: root_ppn_tracker.ppn,
            ppn_tracker_list: vec![root_ppn_tracker],
        }
    }

    pub fn as_satp(&self) -> usize {
        let mode = 8usize;
        let ppn_bits = self.root_ppn.high_to_low().0;
        mode << 60 | ppn_bits
    }
}

impl PageTable {
    fn get_create_pte(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut current_high_ppn = self.root_ppn;

        for (i, &idx) in indexes.iter().enumerate() {
            let pte = &mut current_high_ppn.as_pte_array()[idx];
            if i == indexes.len() - 1 {
                // last level
                return Some(pte);
            }
            if !pte.is_valid() {
                let ppn_tracker = alloc_ppn_tracker().unwrap();
                *pte = PageTableEntry::new(ppn_tracker.ppn.high_to_low(), PTEFlags::V);
                self.ppn_tracker_list.push(ppn_tracker);
            }
            current_high_ppn = pte.ppn().low_to_high();
        }

        None
    }

    fn get_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut current_high_ppn = self.root_ppn;

        for (i, &idx) in indexes.iter().enumerate() {
            let pte = &mut current_high_ppn.as_pte_array()[idx];
            if i == indexes.len() - 1 {
                // last level
                return Some(pte);
            }
            if !pte.is_valid() {
                return None;
            }
            current_high_ppn = pte.ppn().low_to_high();
        }

        None
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.get_create_pte(vpn).unwrap();
        assert!(
            !pte.is_valid(),
            "PageTable: VPN {:#x} already mapped",
            vpn.0
        );
        assert!(
            ppn.0 < crate::board::MEMORY_END,
            "PageTable: ppn must be low"
        );
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.get_pte(vpn).unwrap();
        assert!(pte.is_valid(), "PageTable: VPN {:#x} not mapped", vpn.0);
        *pte = PageTableEntry::empty();
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.get_pte(vpn).map(|pte| *pte)
    }
}
// region PageTable end
