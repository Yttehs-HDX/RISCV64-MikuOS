#![no_std]
#![no_main]

use user_lib::println;
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_page_fault");
    let ptr = 0xdeadbeef as *mut i32;
    unsafe {
        *ptr = 0;
    }
    println!("[User] test_page_fault: done");
    0
}