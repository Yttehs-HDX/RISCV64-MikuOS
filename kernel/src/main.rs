#![no_std]
#![no_main]

use core::arch::global_asm;

use log::trace;

global_asm!(include_str!("entry.S"));

mod lang_items;
mod sbi;
#[macro_use]
mod console;
#[path ="boards/qemu.rs"]
mod board;
mod util;
mod sync;
mod config;
mod batch;
mod syscall;
mod trap;

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    util::logger_init();
    print_sections();
    println!("[Kernel] Hello, world!");
    trap::init_trap();
    batch::init_batch();
    batch::run_app(0);
    // sbi::sbi_shutdown_success();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|addr| {
        unsafe {
            (addr as *mut u8).write_volatile(0);
        }
    });
}

fn print_sections() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss_with_stack();
        fn ebss();
    }
    trace!(".text   [{:#x}, {:#x})", stext as usize, etext as usize);
    trace!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    trace!(".data   [{:#x}, {:#x})", sdata as usize, edata as usize);
    trace!(".bss    [{:#x}, {:#x})", sbss_with_stack as usize, ebss as usize);
}
