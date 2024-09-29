#![no_std]
#![no_main]

use core::arch::asm;

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    unsafe {
        asm!("sret");
    }
    0
}