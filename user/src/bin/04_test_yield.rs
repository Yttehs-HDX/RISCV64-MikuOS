#![no_std]
#![no_main]

use user_lib::{get_time, println, yield_};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_yield");
    println!("[User] test_yield: time = {}", get_time());
    let time = get_time();
    while get_time() - time < 10_000_000 {
        yield_();
    }
    println!("[User] test_yield: time = {}", get_time());
    println!("[User] test_yield: done");
    0
}