#![no_std]
#![no_main]

use user_lib::{get_char, print, println};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("[User] test_read");
    print!("[User] test_read: read one byte from stdin: ");
    let c = get_char();
    println!("{}", c as char);
    println!("[User] test_read: done");
    0
}
