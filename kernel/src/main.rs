#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

use core::arch::global_asm;
use log::trace;

extern crate alloc;

global_asm!(include_str!("entry.S"));

mod app;
mod lang_items;
mod sbi;
#[macro_use]
mod console;
#[path ="boards/qemu.rs"]
mod board;
mod util;
mod sync;
mod config;
mod syscall;
mod trap;
mod task;
mod allocator;

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    util::logger_init();
    trap::init_trap();
    allocator::init_heap();
    print_sections();
    println!("[Kernel] initialized");
    kernel_main();
    sbi::sbi_shutdown_success();
}

fn kernel_main() {
    task::run_first_task();
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
