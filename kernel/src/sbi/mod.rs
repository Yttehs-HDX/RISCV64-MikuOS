use core::arch::asm;

pub use console::*;
pub use system::*;
pub use timer::*;

mod console;
mod system;
mod timer;

#[inline(always)]
fn sbi_call(which: usize, args: [usize; 3]) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "li a6, 0",
            "ecall",
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") which,
        );
    }
    ret
}
