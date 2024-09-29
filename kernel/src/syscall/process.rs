use log::info;
use crate::batch;

pub fn sys_exit(exit_code: usize) -> ! {
    info!("Process exited with code {}", exit_code);
    batch::run_next_app();
}