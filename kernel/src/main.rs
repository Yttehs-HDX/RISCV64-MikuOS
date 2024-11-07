#![no_std]
#![no_main]
#![feature(alloc_error_handler, naked_functions)]

use core::arch::global_asm;

extern crate alloc;

global_asm!(include_str!("entry.S"));

mod app;
mod lang_items;
mod sbi;
#[macro_use]
mod console;
#[path = "boards/qemu/mod.rs"]
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
    mm::init();
    trap::init_trap();
    // trap::enable_timer_interrupt();
    println!("[Kernel] initialized");
    os_start();
    sbi::sbi_shutdown_success();
}

fn os_start() {
    println!("[Kernel] current time: {}", timer::get_current_time());
    task::add_task(app::get_app("test_print").unwrap());
    task::add_task(app::get_app("test_sret").unwrap());
    task::add_task(app::get_app("test_page_fault").unwrap());
    task::add_task(app::get_app("test_yield").unwrap());
    task::add_task(app::get_app("test_sbrk").unwrap());
    task::run_tasks();
}

fn os_end() -> ! {
    println!("[Kernel] current time: {}", timer::get_current_time());
    println!("[Kernel] os end");
    sbi::sbi_shutdown_success();
}

fn clear_bss() {
    (*config::SBSS_NO_STACK..*config::EBSS).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}
