use crate::{mm, task, timer};
use log::{info, warn};

pub fn sys_exit(exit_code: i32) -> ! {
    match exit_code {
        0 => info!("Process exited with code {}", exit_code),
        _ => warn!("Process exited with code {}", exit_code),
    }
    task::exit_handler();
}

pub fn sys_yield() -> ! {
    task::yield_handler();
}

pub use crate::timer::TimeVal;
pub fn sys_get_time(ts: usize, _tz: usize) -> isize {
    let time_val = mm::translate_bype_buffer(
        task::current_user_satp(),
        ts as *mut u8,
        core::mem::size_of::<TimeVal>(),
    )
    .pop()
    .unwrap();
    let time_val_ptr = time_val.as_mut_ptr() as *mut TimeVal;
    unsafe { *time_val_ptr = timer::get_current_time() };
    0
}

pub fn sys_sbrk(increase: i32) -> isize {
    let old_brk = task::change_current_brk(increase);
    match old_brk {
        Some(brk) => brk as isize,
        None => -1,
    }
}
