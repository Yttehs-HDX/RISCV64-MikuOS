
#![no_std]
#![no_main]

use user_lib::{println, sbrk};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_sbrk");
    let ptr = sbrk(4096);
    println!("[User] test_sbrk: sbrk(4096) = {:#x}", ptr as usize);
    let ptr = sbrk(0);
    println!("[User] test_sbrk: sbrk(0) = {:#x}", ptr as usize);
    println!("[User] test_sbrk: done");
    0
}
