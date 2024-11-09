#![no_std]
#![no_main]

use user_lib::{alloc::string::String, get_char, init_heap, print, println};

extern crate user_lib;

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
    println!("[User] user_shell");
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
                        _ => println!("Unknown command: {}", input),
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
    print!("user_shell $ ");
}
