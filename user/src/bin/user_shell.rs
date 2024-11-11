#![no_std]
#![no_main]

use user_lib::{
    alloc::string::String, exec, exit, fork, get_char, init_heap, print, println, waitpid,
};

extern crate user_lib;

const SHELL_NAME: &str = "user_shell";
const COMMAND_NOT_FOUND: i32 = 127;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

const AU: u8 = 0x1bu8;
const AD: u8 = 0x1cu8;
const AL: u8 = 0x1du8;
const AR: u8 = 0x1eu8;

#[no_mangle]
fn main() -> i32 {
    println!("[User] {}", SHELL_NAME);
    init_heap();
    let mut input = String::new();
    print_prompt();
    loop {
        let c = get_char();
        match c {
            LF | CR => {
                println!();
                if !input.is_empty() {
                    match input.as_str() {
                        "clear" => {
                            print!("\x1b[H\x1b[2J");
                        }
                        "exit" => break,
                        _ => {
                            // mark input as path
                            input.push('\0');
                            let path = input.as_str();
                            let pid = fork();
                            if pid == 0 {
                                if exec(path) == -1 {
                                    println!("{}: {}: command not found", SHELL_NAME, path);
                                    exit(COMMAND_NOT_FOUND);
                                }
                            } else {
                                let mut exit_code = 0;
                                let zombie_pid = waitpid(pid as usize, &mut exit_code);
                                if exit_code != COMMAND_NOT_FOUND {
                                    println!(
                                        "{}: program '{}' (PID={}) exited with code {}",
                                        SHELL_NAME, path, zombie_pid, exit_code
                                    );
                                }
                            }
                        }
                    }
                }
                print_prompt();
                input.clear();
            }
            BS | DL => {
                if !input.is_empty() {
                    input.pop();
                    print!("{}", BS as char);
                    print!("{}", b' ' as char);
                    print!("{}", BS as char);
                }
            }
            AU | AD | AL | AR => {
                // ignore
            }
            _ => {
                input.push(c as char);
                print!("{}", c as char);
            }
        }
    }
    0
}

fn print_prompt() {
    print!("{} $ ", SHELL_NAME);
}
