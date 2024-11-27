#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, getpid, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    println!("[User] test_fork: (parent) parent pid = {}", getpid());
    let pid = fork();
    if pid == 0 {
        // child process
        println!("[User] test_fork: (child) child pid = {}", getpid());
        100
    } else {
        // parent process
        let mut exit_code = 0;
        waitpid(pid as usize, &mut exit_code);
        println!(
            "[User] test_fork: (parent) child pid = {}, exit code = {}",
            pid, exit_code
        );
        0
    }
}
