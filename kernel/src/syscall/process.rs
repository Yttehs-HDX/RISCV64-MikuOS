pub fn sys_exit(exit_code: usize) -> ! {
    println!("Process exited with code {}", exit_code);
    loop {}
}