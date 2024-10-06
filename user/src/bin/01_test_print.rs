#![no_std]
#![no_main]

use user_lib::println;

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] Hello, world!");
    0
}