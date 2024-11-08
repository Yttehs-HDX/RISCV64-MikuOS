#![no_std]
#![no_main]
#![feature(linkage, alloc_error_handler)]

pub use console::*;
pub use heap_allocator::*;
pub use timer::*;
pub use wrapper::*;

pub extern crate alloc;

mod lang_items;
mod syscall;
mod wrapper;
#[macro_use]
mod console;
mod heap_allocator;
mod timer;

#[no_mangle]
#[link_section = ".text.entry"]
extern "C" fn _start() -> isize {
    clear_bss();
    init_heap();
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
