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
    println!("Hello, world!");

    extern "C" {
        fn ekernel();
    }
    println!("ekernel: {:#x}", ekernel as usize);

    util::logger_init();
    mm::init();
    // trap::init_trap();
    // // trap::enable_timer_interrupt();
    // task::init();
    println!("[Kernel] initialized");
    os_start();
    sbi::sbi_shutdown_success();
}

fn os_start() {
    println!("[Kernel] current time: {}", timer::get_current_time());
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("test_print").unwrap().elf(),
    // )));
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("test_sret").unwrap().elf(),
    // )));
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("test_page_fault").unwrap().elf(),
    // )));
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("test_yield").unwrap().elf(),
    // )));
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("test_sbrk").unwrap().elf(),
    // )));
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("test_read").unwrap().elf(),
    // )));
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("user_shell").unwrap().elf(),
    // )));
    // task::add_task(Arc::new(ProcessControlBlock::new(
    //     app::get_app("initproc").unwrap().elf(),
    // )));
    task::get_processor().run_tasks();
}

fn os_end() -> ! {
    println!("[Kernel] current time: {}", timer::get_current_time());
    println!("[Kernel] os end");
    sbi::sbi_shutdown_success();
}
