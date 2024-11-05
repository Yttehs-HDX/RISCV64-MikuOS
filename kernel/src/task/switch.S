    .altmacro
    .macro SAVE_SN n
        sd s\n, (\n + 2) * 8(a0)
    .endm
    .macro LOAD_SN n
        ld s\n, (\n + 2) * 8(a1)
    .endm

    .section .text
    .globl __switch

// save the context of current_task
// restore the context of next_task
__switch:
    // pub fn __swicth(
    //    a0: *TaskContext,
    //    a1: *TaskContext,
    // )

    // a0 -> current_task_cx_ptr, a1 -> next_task_cx_ptr

    // save current TaskContext
    // save sp
    sd sp, 1 * 8(a0)

    // do not save ra, because it is the return address of __switch

    // save s registers
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr

    // restore next TaskContext
    // restore s registers
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr

    // restore ra
    ld ra, 0 * 8(a1)

    // restore sp
    ld sp, 1 * 8(a1)

    ret