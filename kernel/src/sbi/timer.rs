use crate::sbi::sbi_call;
use riscv::register::time;

const SBI_SET_TIMER: usize = 0;

#[inline(always)]
pub fn sbi_get_time() -> usize {
    time::read()
}

#[inline(always)]
pub fn sbi_set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, [timer, 0, 0]);
}
