#![no_std]
#![no_main]

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let ptr = 0xdeadbeef as *mut i32;
    unsafe {
        *ptr = 0;
    }
    0
}