use crate::sbi::sbi_call;

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;

#[inline(always)]
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, [c, 0, 0]);
}

#[inline(always)]
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, [0, 0, 0])
}
