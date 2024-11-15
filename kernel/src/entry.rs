use core::arch::asm;
use crate::mm::{PTEFlags, PageTableEntry, PhysAddr};

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    asm!(
        // set sp
        "lui sp, %hi(BOOT_STACK)",
        // construct satp
        "lui t0, %hi(ROOT_PAGE)",
        "li t1, 0xffffffffc0000000 - 0x80000000",
        "sub t0, t0, t1",
        "srli t0, t0, 12",
        "li t1, 8 << 60",
        "or t0, t0, t1",
        "csrw satp, t0",
        "sfence.vma",
        // call rust_main
        "lui t0, %hi(rust_main)",
        "addi t0, t0, %lo(rust_main)",
        "jr t0",
        options(noreturn)
    )
}

#[no_mangle]
#[link_section = ".bss.boot_stack"]
static BOOT_STACK: [u8; 4096] = [0; 4096];

#[no_mangle]
#[link_section = ".data.root_page"]
static ROOT_PAGE: [PageTableEntry; 512] = {
    let flags = PTEFlags::from_bits_truncate(0xcf);
    let mut page = [PageTableEntry::empty(); 512];
    const SBI_ADDR: usize = 0x80000000;
    page[2] = PageTableEntry::new(PhysAddr(SBI_ADDR).to_ppn(), flags);
    page[511] = PageTableEntry::new(PhysAddr(SBI_ADDR).to_ppn(), flags);
    page
};
