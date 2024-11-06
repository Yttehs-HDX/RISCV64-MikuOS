pub use entry::*;

use super::{alloc_ppn_tracker, PPNTracker, PhysPageNum, VirtAddr, VirtPageNum, SV39_PPN_BITS};
use alloc::vec;
use alloc::vec::Vec;
use simple_range::StepByOne;

mod entry;

pub fn translate_bype_buffer(satp: usize, ptr: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from_satp(satp);
    let mut buffer = Vec::new();
    let mut current_va = VirtAddr(ptr as usize);
    let end_va = VirtAddr(ptr as usize + len);

    while current_va < end_va {
        let left_va = current_va;
        let mut vpn = left_va.to_vpn_floor();
        let current_ppn = page_table.translate(vpn).unwrap().ppn();
        vpn.step();
        let right_va = vpn.to_va().min(end_va);
        if right_va.aligned() {
            // more than one page
            buffer.push(&mut current_ppn.get_bytes_array()[left_va.page_offset()..]);
        } else {
            // less than one page
            buffer.push(
                &mut current_ppn.get_bytes_array()[left_va.page_offset()..right_va.page_offset()],
            );
        }

        current_va = right_va;
    }
    buffer
}

// region PageTable begin
pub struct PageTable {
    root_ppn: PhysPageNum,
    ppn_tracker_list: Vec<PPNTracker>,
}

impl PageTable {
    pub fn new() -> Self {
        let root_ppn_tracker = alloc_ppn_tracker().unwrap();
        Self {
            root_ppn: root_ppn_tracker.ppn,
            ppn_tracker_list: vec![root_ppn_tracker],
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

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.get_pte(vpn).map(|pte| *pte)
    }
}
// region PageTable end
