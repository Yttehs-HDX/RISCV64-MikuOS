use log::{info, warn};
use crate::{task, timer::{self, TimeType}};

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

pub fn sys_get_time() -> isize {
    timer::get_current_time().as_sec(TimeType::Raw) as isize
}