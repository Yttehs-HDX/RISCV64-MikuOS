use riscv::register::time;
use super::sbi_call;

const SBI_SET_TIMER: usize = 0;

#[inline(always)]
pub fn sbi_get_time() -> usize {
    time::read()
}

#[inline(always)]
pub fn sbi_set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, [timer, 0, 0]);
}