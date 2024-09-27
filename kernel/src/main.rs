#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

mod lang_items;
mod sbi;
mod console;

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();

    sbi::console_putchar('a' as usize);
    sbi::console_putchar('b' as usize);
    sbi::console_putchar('c' as usize);

    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|addr| {
        unsafe {
            (addr as *mut u8).write_volatile(0);
        }
    });
}