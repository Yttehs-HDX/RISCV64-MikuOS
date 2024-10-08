use log::{info, warn};
use crate::{task, timer};

pub fn sys_exit(exit_code: usize) -> ! {
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
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    unsafe {
        *ts = timer::get_current_time();
    }
    0
}