use alloc::string::ToString;

use crate::{
    fs::{self, Inode, InodeType, OpenFlags, PathUtil},
    task,
};

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

pub fn sys_chdir(path_ptr: *const u8) -> isize {
    let path: &str;
    unsafe {
        let mut len = 0;
        while *path_ptr.add(len) != 0 {
            len += 1;
        }
        path = core::str::from_utf8_unchecked(core::slice::from_raw_parts(path_ptr, len));
    }
    let path = PathUtil::from_user(path).to_string();
    let inode = fs::open_file(&path, OpenFlags::RDONLY);
    if let Some(inode) = inode {
        if inode.get_type() == InodeType::Dir {
            task::get_processor().current().inner_mut().set_cwd(path);
            return 0;
        }
    }
    -1
}
