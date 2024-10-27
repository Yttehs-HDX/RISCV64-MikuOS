pub use entry::*;

use super::{alloc_ppn_tracker, PPNTracker, PhysPageNum, VirtPageNum, SV39_PPN_BITS};
use alloc::vec::Vec;

mod entry;

// region PageTable begin
pub struct PageTable {
    root_ppn: PhysPageNum,
    ppn_tracker_list: Vec<PPNTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let root_ppn = alloc_ppn_tracker().unwrap().ppn;
        Self {
            root_ppn,
            ppn_tracker_list: Vec::new(),
        }
    }

    pub fn from_satp(satp: usize) -> Self {
        let ppn_bits = satp & ((1 << SV39_PPN_BITS) - 1);
        Self {
            root_ppn: PhysPageNum(ppn_bits),
            ppn_tracker_list: Vec::new(),
        }
    }

    pub fn as_satp(&self) -> usize {
        let mode = 8usize;
        let ppn_bits = self.root_ppn.0;
        mode << 60 | ppn_bits
    }
}

impl PageTable {
    fn get_create_pte(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;

        for (i, &idx) in indexes.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[idx];
            if i == indexes.len() - 1 {
                // last level
                return Some(pte);
            }
            if !pte.is_valid() {
                let ppn_tracker = alloc_ppn_tracker().unwrap();
                *pte = PageTableEntry::new(ppn_tracker.ppn, PTEFlags::V);
                self.ppn_tracker_list.push(ppn_tracker);
            }
            ppn = pte.ppn();
        }

        None
    }

    fn get_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexes = vpn.indexes();
        let mut ppn = self.root_ppn;

        for (i, &idx) in indexes.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[idx];
            if i == indexes.len() - 1 {
                // last level
                return Some(pte);
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
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
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.get_pte(vpn).unwrap();
        assert!(pte.is_valid(), "PageTable: VPN {:#x} not mapped", vpn.0);
        *pte = PageTableEntry::empty();
    }

    pub fn tranlate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.get_pte(vpn).map(|pte| *pte)
    }
}
// region PageTable end
