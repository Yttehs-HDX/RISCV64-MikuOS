use core::arch::asm;
pub use fs::*;
pub use process::*;

mod fs;
mod process;

#[inline(always)]
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "ecall",
            in("a7") id,
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
        );
    }
    ret
}
