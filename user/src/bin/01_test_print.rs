#![no_std]
#![no_main]

use user_lib::println;

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_print");
    println!("[User] test_print: Hello, world!");
    println!("[User] test_print: done");
    0
}
