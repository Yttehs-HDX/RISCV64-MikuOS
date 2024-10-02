.altmacro
.macro SAVE_SN n
    sd s\n, (\n + 2) * 8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n + 2) * 8(a1)
.endm

    .section .text
    .globl __switch
__switch:                   // fn __swicth(current_task_cx: *mut TaskContext, next_task_cx: *const TaskContext)
                            //     a0 = current_task_cx;
                            //     a1 = next_task_cx;
    // save current_task
    // save sp (kernel_sp)
    sd sp, 1 * 8(a0)        //     current_task_cx.sp = sp;

    // save ra
    sd ra, 0 * 8(a0)        //     current_task_cx.ra = ra;

    // save s registers
    .set n, 0               //     for n in 0..12
    .rept 12
        SAVE_SN %n          //         current_task_cx.s[n] = s(n);
        .set n, n + 1
    .endr

    // restore next_task
    // restore s registers
    .set n, 0               //     for n in 0..12
    .rept 12
        LOAD_SN %n          //         s(n) = next_task_cx.s[n];
        .set n, n + 1
    .endr

    // restore ra
    ld ra, 0 * 8(a1)        //     ra = next_task_cx.ra;

    // restore sp
    ld sp, 1 * 8(a1)        //     sp = next_task_cx.sp;

    ret                     //     return