#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

mod lang_items;

#[no_mangle]
fn rust_main() -> ! {
    loop {}
}