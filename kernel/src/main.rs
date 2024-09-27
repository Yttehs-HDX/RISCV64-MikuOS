#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

mod lang_items;
mod sbi;
#[macro_use]
mod console;
#[path ="boards/qemu.rs"]
mod board;

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    println!("[Kernel] Hello, world!");
    sbi::sbi_shutdown_success();
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