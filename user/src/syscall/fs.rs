use super::syscall;

const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd as usize, buf.as_ptr() as usize, buf.len()])
}