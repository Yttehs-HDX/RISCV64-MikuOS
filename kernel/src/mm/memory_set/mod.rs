pub use map_area::*;

use crate::{
    config::{
        EBSS, EDATA, ERODATA, ETEXT, MMIO, PA_END, PA_START, SBSS, SDATA, SRODATA, STEXT,
        STRAMPOLINE, SV39_PAGE_SIZE, TRAMPOLINE, TRAP_CX_PTR, USER_STACK_SIZE, USER_STACK_TOP,
    },
    mm::{PTEFlags, PageTable, PageTableEntry, PhysAddr, VirtAddr, VirtPageNum},
};
use alloc::vec::Vec;
use core::arch::asm;
use log::{trace, warn};
use riscv::register::satp;

mod map_area;

// region MemorySet begin
pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl Drop for MemorySet {
    fn drop(&mut self) {
        for area in self.areas.iter_mut() {
            area.unmap_all(&mut self.page_table);
        }
    }
}

impl MemorySet {
    fn empty() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
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
    fn insert_area(&mut self, mut area: MapArea) {
        area.map_all(&mut self.page_table);
        self.areas.push(area);
    }

    fn insert_area_with_data(&mut self, mut area: MapArea, data: &[u8]) {
        area.map_all(&mut self.page_table);
        area.insert_raw_data(data, &mut self.page_table);
        self.areas.push(area);
    }

    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_perm: MapPermission,
    ) {
        let area = MapArea::new(start_va, end_va, MapType::Framed, map_perm);
        self.insert_area(area);
    }

    pub fn remove_area(&mut self, start_vpn: VirtPageNum) -> Option<MapArea> {
        if let Some(index) = self
            .areas
            .iter()
            .position(|area| area.vpn_range.start() == start_vpn)
        {
            let mut area = self.areas.remove(index);
            area.unmap_all(&mut self.page_table);
            Some(area)
        } else {
            None
        }
    }

    pub fn change_area_end(&mut self, start_va: VirtAddr, new_end_va: VirtAddr) -> bool {
        if let Some(area) = self
            .areas
            .iter_mut()
            .find(|area| area.vpn_range.start() == start_va.to_vpn_floor())
        {
            area.change_vpn_end(new_end_va.to_vpn_ceil(), &mut self.page_table);
            true
        } else {
            warn!("MemorySet: area not found");
            false
        }
    }

    pub fn activate(&self) {
        let satp = self.get_satp();
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
        }
    }

    pub fn get_satp(&self) -> usize {
        self.page_table.as_satp()
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }
}

// Kernel Space
impl MemorySet {
    pub fn new_kernel() -> Self {
        let mut memory_set = Self::empty();

        trace!(
            "MemorySet: map trampoline [{:#x}, {:#x}] -> [{:#x}, {:#x})",
            TRAMPOLINE,
            TRAMPOLINE - 1 + SV39_PAGE_SIZE,
            *STRAMPOLINE,
            *STRAMPOLINE + SV39_PAGE_SIZE
        );
        memory_set.map_trampoline();

        // map sections
        trace!("MemorySet: map .text      [{:#x}, {:#x})", *STEXT, *ETEXT);
        memory_set.insert_area(MapArea::new(
            VirtAddr(*STEXT),
            VirtAddr(*ETEXT),
            MapType::Identity,
            MapPermission::R | MapPermission::X,
        ));
        trace!(
            "MemorySet: map .rodata    [{:#x}, {:#x})",
            *SRODATA,
            *ERODATA
        );
        memory_set.insert_area(MapArea::new(
            VirtAddr(*SRODATA),
            VirtAddr(*ERODATA),
            MapType::Identity,
            MapPermission::R,
        ));
        trace!("MemorySet: map .data      [{:#x}, {:#x})", *SDATA, *EDATA);
        memory_set.insert_area(MapArea::new(
            VirtAddr(*SDATA),
            VirtAddr(*EDATA),
            MapType::Identity,
            MapPermission::R | MapPermission::W,
        ));
        trace!("MemorySet: map .bss       [{:#x}, {:#x})", *SBSS, *EBSS);
        memory_set.insert_area(MapArea::new(
            VirtAddr(*SBSS),
            VirtAddr(*EBSS),
            MapType::Identity,
            MapPermission::R | MapPermission::W,
        ));

        // map ppn range
        trace!(
            "MemorySet: map phys space [{:#x}, {:#x})",
            *PA_START,
            PA_END
        );
        memory_set.insert_area(MapArea::new(
            VirtAddr(*PA_START),
            VirtAddr(PA_END),
            MapType::Identity,
            MapPermission::R | MapPermission::W,
        ));

        // map MMIO
        for &pair in MMIO {
            memory_set.insert_area(MapArea::new(
                VirtAddr(pair.0),
                VirtAddr(pair.0 + pair.1),
                MapType::Identity,
                MapPermission::R | MapPermission::W,
            ));
        }

        memory_set
    }
}

// User Space
impl MemorySet {
    // return MemorySet for user space, elf entry, program brk
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        use xmas_elf::{program::Type, ElfFile};
        let mut memory_set = Self::empty();

        // map trampoline
        trace!(
            "MemorySet: map trampoline [{:#x}, {:#x}] -> [{:#x}, {:#x})",
            TRAMPOLINE,
            TRAMPOLINE - 1 + SV39_PAGE_SIZE,
            *STRAMPOLINE,
            *STRAMPOLINE + SV39_PAGE_SIZE
        );
        memory_set.map_trampoline();

        // handle elf
        let elf = ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;

        // map elf program headers
        let mut max_vpn = VirtPageNum(0);
        let ph_count = elf_header.pt2.ph_count();
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == Type::Load {
                // read permission
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }

                // map program header
                let start_va = VirtAddr(ph.virtual_addr() as usize);
                let end_va = VirtAddr(ph.virtual_addr() as usize + ph.mem_size() as usize);
                trace!("MemorySet: map elf ph [{:#x}, {:#x})", start_va.0, end_va.0);
                let area = MapArea::new(start_va, end_va, MapType::Framed, map_perm);
                max_vpn = area.vpn_range.end();
                let elf_data =
                    &elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize];
                memory_set.insert_area_with_data(area, elf_data);
            }
        }

        // map User Heap
        let program_brk_va = max_vpn.to_va();
        trace!("MemorySet: user program break at {:#x}", program_brk_va.0);
        memory_set.insert_area(MapArea::new(
            program_brk_va,
            program_brk_va,
            MapType::Framed,
            MapPermission::U | MapPermission::R | MapPermission::W,
        ));

        // map User Stack
        trace!(
            "MemorySet: map User Stack [{:#x}, {:#x})",
            USER_STACK_TOP,
            USER_STACK_TOP + USER_STACK_SIZE
        );
        memory_set.insert_area(MapArea::new(
            VirtAddr(USER_STACK_TOP),
            VirtAddr(USER_STACK_TOP + USER_STACK_SIZE),
            MapType::Framed,
            MapPermission::U | MapPermission::R | MapPermission::W,
        ));

        // map Trap Context
        trace!(
            "MemorySet: map TrapContext [{:#x}, {:#x})",
            TRAP_CX_PTR,
            TRAP_CX_PTR + SV39_PAGE_SIZE
        );
        memory_set.insert_area(MapArea::new(
            VirtAddr(TRAP_CX_PTR),
            VirtAddr(TRAP_CX_PTR + SV39_PAGE_SIZE),
            MapType::Framed,
            MapPermission::R | MapPermission::W,
        ));

        (
            memory_set,
            elf_header.pt2.entry_point() as usize,
            program_brk_va.0,
        )
    }
}
// region MemorySet end
