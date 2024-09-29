use log::info;

pub fn sys_exit(exit_code: usize) -> ! {
    info!("Process exited with code {}", exit_code);
    loop {}
}