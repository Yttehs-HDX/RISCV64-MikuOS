use super::syscall;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;

pub fn sys_exit(code: i32) -> isize {
    syscall(SYSCALL_EXIT, [code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}