use log::{info, warn};

pub fn sys_exit(exit_code: usize) -> ! {
    match exit_code {
        0 => info!("Process exited with code {}", exit_code),
        _ => warn!("Process exited with code {}", exit_code),
    }
    loop {}
}