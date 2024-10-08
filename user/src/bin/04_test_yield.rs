#![no_std]
#![no_main]

use user_lib::{get_time, println, yield_, TimeVal};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_yield");
    println!("[User] test_yield: time = {}", get_time().format());
    let time = get_time();
    while get_time() - time < TimeVal::new(1, 0) {
        yield_();
    }
    println!("[User] test_yield: time = {}", get_time().format());
    println!("[User] test_yield: done");
    0
}