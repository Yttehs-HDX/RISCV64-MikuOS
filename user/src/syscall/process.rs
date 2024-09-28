use super::syscall;

const SYSCALL_EXIT: usize = 93;

pub fn sys_exit(code: i32) -> isize {
    syscall(SYSCALL_EXIT, [code as usize, 0, 0])
}