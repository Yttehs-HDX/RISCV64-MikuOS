    .altmacro
    .macro SAVE_GP n
        sd x\n, \n * 8(sp)
    .endm
    .macro LOAD_GP n
        ld x\n, \n * 8(sp)
    .endm

    .section .text
    .globl __save_trap
    .align 2 // 4 bytes
__save_trap:                    // fn __save_trap()
    // before: sp -> UserStack, sscratch -> KernelStack
    csrrw sp, sscratch, sp      //     (sp, sscratch) = (sscratch, sp);
    // after: sp -> KernelStack, sscratch -> UserStack

    // TrapContext will be create at sp

    // allocate space for TrapContext
    addi sp, sp, -36 * 8        //     let cx = TrapContext;

    // save general registers
    sd x1, 1 * 8(sp)            //     cx.x[1] = x1;
    // skip sp, which is actually sscratch
    // save x3 - x31
    .set n, 3                   //     for n in 3..31
    .rept 32 - 3
        SAVE_GP %n              //         cx.x[n] = xn;
        .set n, n + 1
    .endr

    // save sstatus
    csrr t0, sstatus            //     t0 = sstatus;
    sd t0, 32 * 8(sp)           //     cx.sstatus = t0;

    // save sepc
    csrr t1, sepc               //     t1 = sepc;
    sd t1, 33 * 8(sp)           //     cx.sepc = t1;

    // save sscratch (actually sp)
    csrr t2, sscratch           //     t2 = sscratch;
    sd t2, 2 * 8(sp)            //     cx.sscratch = t2;

    // save trap_handler
    ld t4, 35 * 8(sp)           //     t4 = cx.trap_handler

    // set formal parameter
    mv a0, sp                   //     a0 = &cx;
    // goto trap_handler
    jr t4                       //     trap_handler(a0);

    .section .text
    .globl __restore_trap
    .align 2 // 4 bytes
__restore_trap:                 // fn __retore_trap(cx: usize)
                                //     let cx = cx as *const TrapContext;
    // sp is at cx
    // now: sp -> KernelStack, sscratch -> UserStack

    // restore sstatus
    ld t0, 32 * 8(sp)           //     t0 = cx.sstatus;
    csrw sstatus, t0            //     cx.sstatus = t0;

    // restore sepc
    ld t1, 33 * 8(sp)           //     t1 = cx.sepc;
    csrw sepc, t1               //     cx.sepc = t1;

    // restore sscratch
    ld t2, 2 * 8(sp)            //     t2 = cx.sscratch;
    csrw sscratch, t2           //     cx.x[2] = t2;

    // restore general registers
    ld x1, 1 * 8(sp)            //     x1 = cx.x[1];
    // skip sp, which is actually sscratch
    .set n, 3                   //     for n in 3..31
    .rept 32 - 3
        LOAD_GP %n              //         xn = cx.x[n];
        .set n, n + 1
    .endr

    // release space for TrapContext
    addi sp, sp, 36 * 8         //     drop(cx);

    // before: sp -> KernelStack, sscratch -> UserStack
    csrrw sp, sscratch, sp      //     (sp, sscratch) = (sscratch, sp);
    // after: sp -> UserStack, sscratch -> KernelStack

    // goto User mode
    sret