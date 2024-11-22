#![no_std]
#![no_main]
#![feature(alloc_error_handler, naked_functions)]

extern crate alloc;

mod app;
mod lang_items;
mod sbi;
#[macro_use]
mod console;
#[path = "boards/qemu/mod.rs"]
mod board;
mod config;
mod entry;
mod mm;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod util;

pub fn main() -> ! {
    println!("[Kernel] Hello, world!");
    assert_eq!(*config::SKERNEL, 0xffff_ffff_c020_0000);

    util::init_log();
    mm::init();
    trap::init_trap();
    // trap::enable_timer_interrupt();
    task::init();
    println!("[Kernel] initialized");
    os_start();
}

fn os_start() -> ! {
    println!("[Kernel] current time: {}", timer::get_current_time());
    task::get_processor().run_tasks();
}

fn os_end() -> ! {
    println!("[Kernel] current time: {}", timer::get_current_time());
    println!("[Kernel] shutdown");
    sbi::sbi_shutdown_success();
}
