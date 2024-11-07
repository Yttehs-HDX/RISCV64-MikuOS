#![no_std]
#![no_main]

use core::arch::asm;
use user_lib::println;

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_sret");
    unsafe {
        asm!("sret");
    }
    println!("[User] test_sret: done");
    0
}
