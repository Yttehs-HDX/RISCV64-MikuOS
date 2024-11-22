use crate::task;

pub fn sys_read(fd: usize, buffer: *mut u8, len: usize) -> isize {
    if let Some(fd) = &task::get_processor().current().inner().get_fd_table_ref()[fd] {
        assert!(fd.readable());
        let slice = unsafe { core::slice::from_raw_parts_mut(buffer, len) };
        fd.read(slice) as isize
    } else {
        panic!("sys_read: fd {} not supported", fd);
    }
}

pub fn sys_write(fd: usize, buffer: *const u8, len: usize) -> isize {
    if let Some(fd) = &task::get_processor().current().inner().get_fd_table_ref()[fd] {
        assert!(fd.writable());
        let slice = unsafe { core::slice::from_raw_parts(buffer, len) };
        fd.write(slice) as isize
    } else {
        panic!("sys_write: fd {} not supported", fd);
    }
}
