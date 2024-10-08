use crate::board;
use super::sbi_call;

const SBI_SHUTDOWN: usize = 8;

#[allow(unused)]
#[inline(always)]
pub fn sbi_shutdown(code: usize) -> ! {
    board::qemu_exit(code as u32);
}

#[inline(always)]
pub fn sbi_shutdown_success() -> ! {
    board::qemu_exit_success();
}

#[inline(always)]
pub fn sbi_shutdown_failure() -> ! {
    board::qemu_exit_failure();
}

#[allow(unused)]
#[inline(always)]
pub fn sbi_legacy_shutdown(code: usize) -> ! {
    sbi_call(SBI_SHUTDOWN, [code, 0, 0]);
    unreachable!();
}