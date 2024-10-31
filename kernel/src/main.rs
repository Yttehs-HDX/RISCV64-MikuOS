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
mod allocator;
#[path = "boards/qemu.rs"]
mod board;
mod config;
mod mm;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod util;

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    util::logger_init();
    trap::init_trap();
    trap::enable_timer_interrupt();
    allocator::init_heap();
    print_sections();
    println!("[Kernel] initialized");
    os_start();
    sbi::sbi_shutdown_success();
}

fn os_start() {
    println!(
        "[Kernel] current time: {}",
        timer::get_current_time().format()
    );
    task::add_task(app::get_app("test_print").unwrap());
    task::add_task(app::get_app("test_sret").unwrap());
    task::add_task(app::get_app("test_page_fault").unwrap());
    task::add_task(app::get_app("test_yield").unwrap());
    task::print_task_info();
    task::run_task();
}

fn os_end() -> ! {
    println!(
        "[Kernel] current time: {}",
        timer::get_current_time().format()
    );
    sbi::sbi_shutdown_success();
}

fn clear_bss() {
    (config::sbss_no_stack as usize..config::ebss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

fn print_sections() {
    trace!(
        " KERNEL [{:#x}, {:#x})",
        config::skernel as usize,
        config::ekernel as usize
    );
    trace!(
        ".text   [{:#x}, {:#x})",
        config::stext as usize,
        config::etext as usize
    );
    trace!(
        ".rodata [{:#x}, {:#x})",
        config::srodata as usize,
        config::erodata as usize
    );
    trace!(
        ".data   [{:#x}, {:#x})",
        config::sdata as usize,
        config::edata as usize
    );
    trace!(
        ".bss    [{:#x}, {:#x})",
        config::sbss as usize,
        config::ebss as usize
    );
}
