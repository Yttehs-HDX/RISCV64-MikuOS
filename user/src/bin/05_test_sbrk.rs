
#![no_std]
#![no_main]

use user_lib::{println, sbrk};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_sbrk");
    let old_ptr = sbrk(4096) as *mut u8;
    println!("[User] test_sbrk: sbrk(4096) = {:#x}", old_ptr as usize);
    let new_ptr = sbrk(0) as *mut u8;
    println!("[User] test_sbrk: sbrk(0) = {:#x}", new_ptr as usize);

    println!("[User] test_sbrk: heap size = {}", new_ptr as usize - old_ptr as usize);

    println!("[User] test_sbrk: write to old_ptr");
    unsafe {
        *old_ptr = 114;
    }

    println!("[User] test_sbrk: read from old_ptr");
    let value = unsafe { *old_ptr };
    println!("[User] test_sbrk: *old_ptr = {}", value);

    sbrk(-4096);
    let new_ptr = sbrk(0) as *mut u8;
    println!("[User] test_sbrk: sbrk(-4096) = {:#x}", new_ptr as usize);
    println!("[User] test_sbrk: heap size = {}", new_ptr as usize - old_ptr as usize);

    println!("[User] test_sbrk: done");
    0
}
