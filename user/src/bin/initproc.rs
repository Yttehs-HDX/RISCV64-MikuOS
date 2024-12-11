#![no_std]
#![no_main]

use user_lib::{exec, fork, println, wait};

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        exec("user_shell\0");
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                break;
            }
            println!(
                "[user_shell] Process {} exited with code {}",
                pid, exit_code
            );
        }
    }
    0
}
