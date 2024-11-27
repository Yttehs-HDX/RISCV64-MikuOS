use super::syscall;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_SBRK: usize = 214;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;

pub fn sys_exit(code: i32) -> ! {
    syscall(SYSCALL_EXIT, [code as usize, 0, 0]);
    unreachable!();
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

use crate::timer::TimeVal;
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    syscall(SYSCALL_GET_TIME, [ts as usize, _tz, 0])
}

pub fn sys_sbrk(inc: i32) -> isize {
    syscall(SYSCALL_SBRK, [inc as usize, 0, 0])
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

pub fn sys_exec(path: &str, argv: &[&str]) -> isize {
    syscall(
        SYSCALL_EXEC,
        [path.as_ptr() as usize, argv.as_ptr() as usize, 0],
    )
}

pub fn sys_waitpid(pid: isize, wstatus: *mut i32, options: i32) -> isize {
    syscall(
        SYSCALL_WAITPID,
        [pid as usize, wstatus as usize, options as usize],
    )
}
