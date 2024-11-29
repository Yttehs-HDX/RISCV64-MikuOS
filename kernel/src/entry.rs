use crate::{
    config::{EBSS, KERNEL_STACK_SIZE, SBSS_NO_STACK},
    mm::{PTEFlags, PageTableEntry, PhysAddr},
};
use core::arch::asm;

const LOW_ADDR: usize = 0x80000000;
const HIGH_ADDR: usize = 0xffffffffc0000000;
pub const KERNEL_ADDR_OFFSET: usize = HIGH_ADDR - LOW_ADDR;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    asm!(
        // set sp
        "la sp, {boot_stack}", // sp = &BOOT_STACK
        "li t0, {stack_size}", // t0 = KERNEL_STACK_SIZE
        "add sp, sp, t0", // sp += t0, low address
        "li t0, {offset}", // t0 = KERNEL_ADDR_OFFSET
        "add sp, sp, t0", // sp += t0, just find the space before mapping

        // get rust_main before enable paging
        "la t3, {rust_main}", // low address

        // construct satp
        "la t1, {root_page}",
        "srli t1, t1, 12", // t1 <<= 12, get ppn
        "li t2, 8 << 60", // t2 = 8 << 60
        "or t1, t1, t2", // t1 |= t2
        "csrw satp, t1",
        "sfence.vma",

        // call rust_main
        "add t3, t3, t0", // high address
        "jr t3",
        offset = const KERNEL_ADDR_OFFSET,
        boot_stack = sym BOOT_STACK,
        stack_size = const KERNEL_STACK_SIZE,
        root_page = sym ROOT_PAGE,
        rust_main = sym rust_main,
        options(noreturn)
    )
}

fn rust_main() {
    clear_bss();
    crate::main();
}

fn clear_bss() {
    (*SBSS_NO_STACK..*EBSS).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

#[link_section = ".bss.boot_stack"]
static BOOT_STACK: [u8; KERNEL_STACK_SIZE] = [0; KERNEL_STACK_SIZE];

#[link_section = ".data.root_page"]
static ROOT_PAGE: [PageTableEntry; 512] = {
    let flags = PTEFlags::from_bits_truncate(0xcf);
    let mut page = [PageTableEntry::empty(); 512];
    page[2] = PageTableEntry::new(PhysAddr(LOW_ADDR).to_ppn(), flags);
    page[511] = PageTableEntry::new(PhysAddr(LOW_ADDR).to_ppn(), flags);
    page
};
