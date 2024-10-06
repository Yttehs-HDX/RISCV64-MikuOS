use crate::syscall;

pub fn exit(code: i32) -> isize {
    syscall::sys_exit(code)
}

pub fn yield_() -> isize {
    syscall::sys_yield()
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::sys_write(fd, buf)
}