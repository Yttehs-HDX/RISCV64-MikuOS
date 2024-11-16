use core::arch::asm;
use crate::mm::{PTEFlags, PageTableEntry, PhysAddr};

const SBI_ADDR: usize = 0x80000000;
const BASE_ADDR: usize = 0xffffffffc0000000;
const OFFSET: usize = BASE_ADDR - SBI_ADDR;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    asm!(
        // set sp
        "la sp, {boot_stack}",
        "li t0, {offset}",
        "add sp, sp, t0",
        // construct satp
        "la t1, {root_page}",
        "srli t1, t1, 12",
        "li t2, 8 << 60",
        "or t1, t1, t2",
        "csrw satp, t1",
        "sfence.vma",
        // call rust_main
        "la t1, rust_main",
        "add t1, t1, t0",
        "jr t1",
        offset = const OFFSET,
        boot_stack = sym BOOT_STACK,
        root_page = sym ROOT_PAGE,
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
    page[2] = PageTableEntry::new(PhysAddr(SBI_ADDR).to_ppn(), flags);
    page[511] = PageTableEntry::new(PhysAddr(SBI_ADDR).to_ppn(), flags);
    page
};
