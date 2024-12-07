use crate::{task::TaskContext, trap};
use core::arch::asm;

#[naked]
pub unsafe extern "C" fn __save_task(task_cx: *mut TaskContext) {
    // a0 -> *mut TaskContext
    asm!(
        // save s0 - s11
        "sd s0, 0 * 8(a0)",
        "sd s1, 1 * 8(a0)",
        "sd s2, 2 * 8(a0)",
        "sd s3, 3 * 8(a0)",
        "sd s4, 4 * 8(a0)",
        "sd s5, 5 * 8(a0)",
        "sd s6, 6 * 8(a0)",
        "sd s7, 7 * 8(a0)",
        "sd s8, 8 * 8(a0)",
        "sd s9, 9 * 8(a0)",
        "sd s10, 10 * 8(a0)",
        "sd s11, 11 * 8(a0)",
        // done
        "ret",
        options(noreturn)
    )
}

#[naked]
pub unsafe extern "C" fn __restore_task(task_cx: *const TaskContext) -> ! {
    // a0 -> *const TaskContext
    asm!(
        // restore s0 - s11
        "ld s0, 0 * 8(a0)",
        "ld s1, 1 * 8(a0)",
        "ld s2, 2 * 8(a0)",
        "ld s3, 3 * 8(a0)",
        "ld s4, 4 * 8(a0)",
        "ld s5, 5 * 8(a0)",
        "ld s6, 6 * 8(a0)",
        "ld s7, 7 * 8(a0)",
        "ld s8, 8 * 8(a0)",
        "ld s9, 9 * 8(a0)",
        "ld s10, 10 * 8(a0)",
        "ld s11, 11 * 8(a0)",

        // goto trap::trap_return
        "la t0, {trap_return}",
        "jr t0",
        trap_return = sym trap::trap_return,
        options(noreturn)
    )
}
