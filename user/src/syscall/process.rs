use super::syscall;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

pub fn sys_exit(code: i32) -> isize {
    syscall(SYSCALL_EXIT, [code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

use crate::timer::TimeVal;

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    syscall(SYSCALL_GET_TIME, [ts as usize, _tz, 0])
}