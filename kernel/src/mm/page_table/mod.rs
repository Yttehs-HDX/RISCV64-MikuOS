use alloc::vec::Vec;
use super::{alloc_frame, FrameTracker, PhysPageNum, VirtPageNum, SV39_PPN_WIDTH};

pub use entry::*;

mod entry;

// region PageTable begin
pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    fn create_pte_by_vpn(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let vpn_idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, &vpn_i) in vpn_idxs.iter().enumerate() {
            let pte = &mut ppn.as_pte_array()[vpn_i];
            if i == vpn_idxs.len() - 1 {
                return Some(pte);
            }
            if !pte.has_flag(PTEFlags::V) {
                // allocate a new frame for pte
                let frame = alloc_frame().unwrap();
                *pte = PageTableEntry::new(
                    frame.ppn,
                    PTEFlags::V | PTEFlags::A | PTEFlags::D
                );
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        None
    }

    fn get_pte_by_vpn(&self, vpn: VirtPageNum) -> Option<&PageTableEntry> {
        let vpn_idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, &vpn_i) in vpn_idxs.iter().enumerate() {
            let pte = &mut ppn.as_pte_array()[vpn_i];
            if i == vpn_idxs.len() - 1 {
                return Some(pte);
            }
            if !pte.has_flag(PTEFlags::V) {
                return None;
            }
            ppn = pte.ppn();
        }
        None
    }
}

impl PageTable {
    pub fn new() -> Self {
        let frame = alloc_frame().unwrap();
        Self {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.create_pte_by_vpn(vpn).unwrap();
        assert!(!pte.has_flag(PTEFlags::V), "PageTable: mapping already exists");
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.create_pte_by_vpn(vpn).unwrap();
        assert!(pte.has_flag(PTEFlags::V), "PageTable: no mapping");
        *pte = PageTableEntry::empty();
    }

    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum(satp & (1 << SV39_PPN_WIDTH - 1)),
            frames: Vec::new(),
        }
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.get_pte_by_vpn(vpn).map( |pte| {
            pte.clone()
        })
    }
}
// region PageTable end