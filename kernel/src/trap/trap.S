    .altmacro
    .macro SAVE_GP n
        sd x\n, \n * 8(sp)
    .endm
    .macro LOAD_GP n
        ld x\n, \n * 8(sp)
    .endm

    .section .text.trampoline
    .globl __save_trap
    .globl __restore_trap
    .align 2 // 4 bytes

// when a trap happens
// save the context and jump to the trap handler
__save_trap:
    // pub fn __save_trap()

    // now: satp -> UserSpace
    // sp -> UserStack, sscratch -> *TrapContext in UserSpace

    // switch sp and sscratch
    csrrw sp, sscratch, sp
    // sp -> *TrapContext in UserSpace, sscratch -> UserStack

    // save general registers
    sd x1, 1 * 8(sp)
    // skip x2/sp, which is actually sscratch
    sd x3, 3 * 8(sp)
    // skip x4, which is actually tp
    // save x5 - x31
    .set n, 5
    .rept 32 - 5
        SAVE_GP %n
        .set n, n + 1
    .endr

    // save sstatus
    csrr t0, sstatus
    sd t0, 32 * 8(sp)

    // save sepc
    csrr t0, sepc
    sd t0, 33 * 8(sp)

    // save sp in sscratch
    csrr t0, sscratch
    sd t0, 2 * 8(sp)

    // read kernel_satp
    ld t1, 34 * 8(sp)

    // read kernel_sp
    ld t2, 35 * 8(sp)

    // read trap_handler
    ld t3, 36 * 8(sp)

    // set satp
    csrw satp, t1
    sfence.vma

    // now: satp -> KernelSpace

    // set sp
    mv sp, t2

    // sp -> KernelStack

    // goto trap_handler
    jr t3

// after handling the trap
// restore the context and return to user mode
__restore_trap:
    // pub fn __restore_trap(
    //    a0: *TrapContext,
    //    a1: user_satp,
    // )

    // a0 -> *TrapContext in UserSpace, a1 -> satp

    // now: satp -> KernelSpace
    // sp -> KernelStack

    // set satp
    csrw satp, a1
    sfence.vma
    // now: satp -> UserSpace

    mv sp, a0
    // sp -> *TrapContext in UserSpace

    // restore sstatus
    ld t0, 32 * 8(sp)
    csrw sstatus, t0

    // restore sepc
    ld t0, 33 * 8(sp)
    csrw sepc, t0

    // restore general registers
    ld x1, 1 * 8(sp)
    // skip x2/sp, restore it later
    ld x3, 3 * 8(sp)
    // skip x4, which is actually tp
    .set n, 5
    .rept 32 - 5
        LOAD_GP %n
        .set n, n + 1
    .endr

    // restore sp
    ld sp, 2 * 8(sp)
    // sp -> UserStack

    // goto User mode
    sret
