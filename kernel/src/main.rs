#![no_std]
#![no_main]
#![feature(alloc_error_handler, naked_functions)]

extern crate alloc;

mod lang_items;
mod sbi;
#[macro_use]
mod console;
#[path = "boards/qemu/mod.rs"]
mod board;
mod config;
mod drivers;
mod entry;
mod fs;
mod mm;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod util;

pub fn main() -> ! {
    print_logo();
    println!("[Kernel] Hello, world!");
    assert_eq!(*config::SKERNEL, 0xffff_ffff_c020_0000);

    util::init_log();
    mm::init();
    trap::init_trap();
    // trap::enable_timer_interrupt();
    task::init();
    println!("[Kernel] initialized");

    #[cfg(not(feature = "test"))]
    os_start();
    #[cfg(feature = "test")]
    run_test();
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

#[cfg(feature = "test")]
fn run_test() -> ! {
    for test in [
        "execve",
        "getcwd",
        "munmap",
        "pipe",
        "umount",
        "close",
        "getppid",
        "chdir",
        "open",
        "clone",
        "mount",
        "exit",
        "sleep",
        "mmap",
        "uname",
        "gettimeofday",
        "mkdir_",
        "fstat",
        "getpid",
        "wait",
        "write",
        "getdents",
        "waitpid",
        "dup2",
        "yield",
        "times",
        "brk",
        "read",
        "fork",
        "openat",
        "dup",
        "unlink",
    ] {
        task::create_process(test);
    }
    task::get_processor().run_tasks();
}

fn print_logo() {
    println!(
        r#" __  __   _   _               ___    ____  
|  \/  | (_) | | __  _   _   / _ \  / ___| 
| |\/| | | | | |/ / | | | | | | | | \___ \ 
| |  | | | | |   <  | |_| | | |_| |  ___) |
|_|  |_| |_| |_|\_\  \__,_|  \___/  |____/ "#
    );
}
