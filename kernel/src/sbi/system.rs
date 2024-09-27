use crate::board;
use super::sbi_call;

const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
pub fn sbi_shutdown(code: usize) -> ! {
    board::qemu_exit(code as u32);
}

#[allow(unused)]
#[inline(always)]
pub fn sbi_legacy_shutdown(code: usize) -> ! {
    sbi_call(SBI_SHUTDOWN, [code, 0, 0]);
    unreachable!();
}