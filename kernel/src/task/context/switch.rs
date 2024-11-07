use super::TaskContext;
use core::arch::asm;

#[naked]
pub unsafe extern "C" fn __save_task(task_cx: *mut TaskContext) {
    // a0 -> *mut TaskContext
    asm!(
        // save sp
        "sd sp, 1 * 8(a0)",
        // save s0 - s11
        "sd s0, 2 * 8(a0)",
        "sd s1, 3 * 8(a0)",
        "sd s2, 4 * 8(a0)",
        "sd s3, 5 * 8(a0)",
        "sd s4, 6 * 8(a0)",
        "sd s5, 7 * 8(a0)",
        "sd s6, 8 * 8(a0)",
        "sd s7, 9 * 8(a0)",
        "sd s8, 10 * 8(a0)",
        "sd s9, 11 * 8(a0)",
        "sd s10, 12 * 8(a0)",
        "sd s11, 13 * 8(a0)",
        // done

        // return
        "ret",
        options(noreturn)
    )
}

#[naked]
pub unsafe extern "C" fn __restore_task(task_cx: *const TaskContext) {
    // a0 -> *const TaskContext
    asm!(
        // restore s0 - s11
        "ld s0, 2 * 8(a0)",
        "ld s1, 3 * 8(a0)",
        "ld s2, 4 * 8(a0)",
        "ld s3, 5 * 8(a0)",
        "ld s4, 6 * 8(a0)",
        "ld s5, 7 * 8(a0)",
        "ld s6, 8 * 8(a0)",
        "ld s7, 9 * 8(a0)",
        "ld s8, 10 * 8(a0)",
        "ld s9, 11 * 8(a0)",
        "ld s10, 12 * 8(a0)",
        "ld s11, 13 * 8(a0)",
        // set ra
        "ld ra, 0 * 8(a0)",
        // restore sp
        "ld sp, 1 * 8(a0)",
        // done

        // return
        "ret",
        options(noreturn)
    )
}
