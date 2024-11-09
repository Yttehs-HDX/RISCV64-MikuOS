/* Trap control flow
 *
 * A trap happens from user mode
 * goto entry point from stvec (__snap_trap)
 *
 * __snap_trap() - Snap Trap Context
 * save registers
 * goto trap_handler
 *
 * trap_handler() - Handle Trap
 * syscall / exception / interrupt
 * goto trap_return
 *
 * trap_return() - Return to User
 * goto __restore_snap
 *
 * __restore_snap() - Restore Trap Context
 */

use crate::{
    config::{TRAMPOLINE, TRAP_CX_PTR},
    syscall, task, timer,
    trap::{set_kernel_trap_entry, set_user_trap_entry},
};
use core::arch::asm;
use log::{error, trace};
use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    stval,
};

#[naked]
#[no_mangle]
#[link_section = ".text.trampoline"]
unsafe extern "C" fn __snap_trap() -> ! {
    asm!(
        // S mode
        // satp -> UserSpace
        // sp -> UserStack
        // sscratch -> *TrapContext (in UserSpace)
        "csrrw sp, sscratch, sp",
        // sp -> *TrapContext (in UserSpace)
        // sscratch -> UserStack

        // save general purpose registers
        "sd x1, 1*8(sp)",
        // save sp later
        "sd x3, 3*8(sp)",
        // skip tp
        "sd x5, 5*8(sp)",
        "sd x6, 6*8(sp)",
        "sd x7, 7*8(sp)",
        "sd x8, 8*8(sp)",
        "sd x9, 9*8(sp)",
        "sd x10, 10*8(sp)",
        "sd x11, 11*8(sp)",
        "sd x12, 12*8(sp)",
        "sd x13, 13*8(sp)",
        "sd x14, 14*8(sp)",
        "sd x15, 15*8(sp)",
        "sd x16, 16*8(sp)",
        "sd x17, 17*8(sp)",
        "sd x18, 18*8(sp)",
        "sd x19, 19*8(sp)",
        "sd x20, 20*8(sp)",
        "sd x21, 21*8(sp)",
        "sd x22, 22*8(sp)",
        "sd x23, 23*8(sp)",
        "sd x24, 24*8(sp)",
        "sd x25, 25*8(sp)",
        "sd x26, 26*8(sp)",
        "sd x27, 27*8(sp)",
        "sd x28, 28*8(sp)",
        "sd x29, 29*8(sp)",
        "sd x30, 30*8(sp)",
        "sd x31, 31*8(sp)",
        // save sstatus
        "csrr t0, sstatus",
        "sd t0, 32*8(sp)",
        // save sepc
        "csrr t0, sepc",
        "sd t0, 33*8(sp)",
        // save sp (in sscratch)
        "csrr t0, sscratch",
        "sd t0, 2*8(sp)",
        // done

        // read kernel_satp
        "ld t1, 34*8(sp)",
        // read kernel_sp
        "ld t2, 35*8(sp)",
        // read trap_handler address
        "ld t3, 36*8(sp)",
        // switch to KernelSpace
        "csrw satp, t1",
        "sfence.vma",
        // satp -> KernelSpace

        // switch to KernelStack
        "mv sp, t2",
        // sp -> KernelStack

        // goto trap_handler
        "jr t3",
        options(noreturn)
    )
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cx = task::get_processor()
        .current()
        .unwrap()
        .inner_mut()
        .get_trap_cx_mut();
    let stval = stval::read();
    let scause = scause::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            trace!("Ecall from U-mode @ {:#x}", cx.sepc);
            // move to next instruction
            cx.sepc += 4;
            cx.x[10] = syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            trap_return();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            timer::set_next_trigger();
            syscall::sys_yield();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("Illegal instruction @ {:#x}, badaddr {:#x}", cx.sepc, stval);
            syscall::sys_exit(1);
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("Store fault @ {:#x}, badaddr {:#x}", cx.sepc, stval);
            syscall::sys_exit(1);
        }
        _ => {
            error!("Unhandled trap {:?} @ {:#x}", scause.cause(), cx.sepc);
            syscall::sys_exit(1);
        }
    }
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CX_PTR;
    let user_satp = task::get_processor()
        .current()
        .unwrap()
        .inner_mut()
        .get_satp();
    let restore_snap_va = __restore_snap as usize - __snap_trap as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_snap}", restore_snap = in(reg) restore_snap_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        )
    }
}

#[naked]
#[no_mangle]
#[link_section = ".text.trampoline"]
unsafe extern "C" fn __restore_snap() -> ! {
    // a0: trap_cx: *const TrapContext
    // a1: user_satp: usize
    asm!(
        // a0 -> *TrapContext (in UserSpace)
        // a1 -> satp (UserSpace)
        // satp -> KernelSpace
        // sp -> KernelStack
        "csrw satp, a1",
        "sfence.vma",
        // satp -> UserSpace
        "csrw sscratch, a0",
        // ssratch -> *TrapContext (in UserSpace)
        "mv sp, a0",
        // sp -> *TrapContext (in UserSpace)

        // restore sstatus
        "ld t0, 32*8(a0)",
        "csrw sstatus, t0",
        // restore sepc
        "ld t0, 33*8(a0)",
        "csrw sepc, t0",
        // restore general purpose registers
        "ld x1, 1*8(sp)",
        // restore sp later
        "ld x3, 3*8(sp)",
        // skip tp
        "ld x5, 5*8(sp)",
        "ld x6, 6*8(sp)",
        "ld x7, 7*8(sp)",
        "ld x8, 8*8(sp)",
        "ld x9, 9*8(sp)",
        "ld x10, 10*8(sp)",
        "ld x11, 11*8(sp)",
        "ld x12, 12*8(sp)",
        "ld x13, 13*8(sp)",
        "ld x14, 14*8(sp)",
        "ld x15, 15*8(sp)",
        "ld x16, 16*8(sp)",
        "ld x17, 17*8(sp)",
        "ld x18, 18*8(sp)",
        "ld x19, 19*8(sp)",
        "ld x20, 20*8(sp)",
        "ld x21, 21*8(sp)",
        "ld x22, 22*8(sp)",
        "ld x23, 23*8(sp)",
        "ld x24, 24*8(sp)",
        "ld x25, 25*8(sp)",
        "ld x26, 26*8(sp)",
        "ld x27, 27*8(sp)",
        "ld x28, 28*8(sp)",
        "ld x29, 29*8(sp)",
        "ld x30, 30*8(sp)",
        "ld x31, 31*8(sp)",
        // restore sp
        "ld sp, 2*8(sp)",
        // sp -> UserStack
        // done

        // return to U mode
        "sret",
        options(noreturn)
    )
}
