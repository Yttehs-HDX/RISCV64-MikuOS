use crate::{
    config::ROOT_DIR,
    fs::{self, Inode, InodeType, OpenFlags, PathUtil},
    syscall::translate_str,
    task,
};
use alloc::{string::ToString, sync::Arc};

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

pub fn sys_getcwd(buffer: *mut u8, len: usize) -> isize {
    let cwd = task::get_processor().current().inner().get_cwd();
    let cwd = if cwd.is_empty() { "/" } else { &cwd };
    let len = len.min(cwd.len());
    let slice = unsafe { core::slice::from_raw_parts_mut(buffer, len) };
    slice.copy_from_slice(cwd.as_bytes());
    len as isize
}

pub fn sys_chdir(path_ptr: *const u8) -> isize {
    let path = translate_str(path_ptr);
    let path = PathUtil::from_user(path).to_string();

    if path == ROOT_DIR {
        // '/' could not be opened
        task::get_processor()
            .current()
            .inner_mut()
            .set_cwd(ROOT_DIR.to_string());
        return 0;
    }

    let inode = fs::open_file(&path, OpenFlags::RDONLY);
    if let Some(inode) = inode {
        if inode.get_type() == InodeType::Dir {
            task::get_processor().current().inner_mut().set_cwd(path);
            return 0;
        }
    }
    -1
}

pub fn sys_open(_dir_fd: i32, path_ptr: *const u8, flags: usize) -> isize {
    let path = translate_str(path_ptr);
    let path = PathUtil::from_user(path).to_string();
    let flags = OpenFlags::from_bits(flags as u32).unwrap();

    let inode = fs::open_file(&path, flags);
    if let Some(inode) = inode {
        let current_task = task::get_processor().current();
        let mut task_inner = current_task.inner_mut();
        let fd_table = task_inner.get_fd_table_mut();

        match inode.get_type() {
            InodeType::File => {
                if flags.directory() {
                    return -1;
                }
                let file = Arc::new(inode.to_file());
                let fd = fd_table.len();
                fd_table.push(Some(file));
                return fd as isize;
            }
            InodeType::Dir => {
                if !flags.directory() {
                    return -1;
                }
                let dir = Arc::new(inode.to_dir());
                let fd = fd_table.len();
                fd_table.push(Some(dir));
                return fd as isize;
            }
            _ => {}
        }
    }
    -1
}

pub fn sys_close(fd: usize) -> isize {
    let current_task = task::get_processor().current();
    let mut task_inner = current_task.inner_mut();
    let fd_table = task_inner.get_fd_table_mut();

    if fd < fd_table.len() {
        fd_table.remove(fd);
        0
    } else {
        -1
    }
}
