#![no_std]
#![no_main]

use user_lib::{println, yield_};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_yield");
    yield_();
    println!("[User] test_yield: done");
    0
}