#![no_std]
#![no_main]
#![feature(linkage)]

pub use timer::*;
pub use wrapper::*;

mod lang_items;
mod syscall;
pub mod wrapper;
#[macro_use]
pub mod console;
pub mod timer;

#[no_mangle]
#[link_section = ".text.entry"]
extern "C" fn _start() -> isize {
    clear_bss();
    exit(main())
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Default main() should not be called")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe {
        (a as *mut u8).write_volatile(0);
    });
}
