use crate::syscall;

pub fn exit(code: i32) -> isize {
    syscall::sys_exit(code)
}

pub fn yield_() -> isize {
    syscall::sys_yield()
}

use crate::timer::TimeVal;

pub fn get_time() -> TimeVal {
    let mut ts = TimeVal::empty();
    syscall::sys_get_time(&mut ts, 0);
    ts
}

pub fn read(buf: &mut [u8]) -> isize {
    let fd = 0;
    syscall::sys_read(fd, buf)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::sys_write(fd, buf)
}

pub fn sbrk(inc: i32) -> isize {
    syscall::sys_sbrk(inc)
}
