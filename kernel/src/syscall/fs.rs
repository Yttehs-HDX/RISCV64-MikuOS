use crate::{
    config::ROOT_DIR,
    fs::{self, Inode, InodeType, OpenFlags, PathUtil},
    syscall::translate_str,
    task,
};
use alloc::{string::ToString, sync::Arc};

pub fn sys_read(fd: usize, buffer: *mut u8, len: usize) -> isize {
    let current_task = task::get_processor().current();
    let task_inner = current_task.inner();
    if let Some(fd_impl) = task_inner.find_fd(fd) {
        assert!(fd_impl.readable(), "fd {} not readable", fd);
        let slice = unsafe { core::slice::from_raw_parts_mut(buffer, len) };
        drop(task_inner);
        fd_impl.read(slice) as isize
    } else {
        panic!("sys_read: fd {} not supported", fd);
    }
}

pub fn sys_write(fd: usize, buffer: *const u8, len: usize) -> isize {
    let current_task = task::get_processor().current();
    let task_inner = current_task.inner();
    if let Some(fd_impl) = task_inner.find_fd(fd) {
        assert!(fd_impl.writable(), "fd {} is not writable", fd);
        let slice = unsafe { core::slice::from_raw_parts(buffer, len) };
        drop(task_inner);
        fd_impl.write(slice) as isize
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

        match inode.get_type() {
            InodeType::File => {
                if flags.directory() {
                    return -1;
                }
                let file = Arc::new(inode.to_file());
                let fd = task_inner.alloc_fd(file);
                return fd as isize;
            }
            InodeType::Dir => {
                if !flags.directory() {
                    return -1;
                }
                let dir = Arc::new(inode.to_dir());
                let fd = task_inner.alloc_fd(dir);
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

    match task_inner.take_fd(fd) {
        Some(_) => 0,
        None => -1,
    }
}

pub fn sys_mkdir(_dir_fd: usize, path_ptr: *const u8, mode: usize) -> isize {
    let path = translate_str(path_ptr);
    let path = PathUtil::from_user(path).to_string();

    if fs::create_dir(&path, mode) {
        0
    } else {
        -1
    }
}

pub fn sys_pipe(pipe_ptr: *mut i32) -> isize {
    let current_task = task::get_processor().current();
    let mut task_inner = current_task.inner_mut();

    let (pipe_read, pipe_write) = fs::make_pipe();
    let read_fd = task_inner.alloc_fd(pipe_read);
    let write_fd = task_inner.alloc_fd(pipe_write);

    unsafe {
        *pipe_ptr = read_fd as i32;
        *pipe_ptr.add(1) = write_fd as i32;
    }
    0
}

pub fn sys_mount(_source_ptr: *const u8, _target_ptr: *const u8, _fs_type_ptr: *const u8) -> isize {
    // unsupported for rust-fatfs
    0
}

pub fn sys_umount(_target_ptr: *const u8) -> isize {
    // unsupported for rust-fatfs
    0
}

pub fn sys_unlink(_dirfd: usize, path_ptr: *const u8, _flags: usize) -> isize {
    let path = translate_str(path_ptr);
    let path = PathUtil::from_user(path).to_string();
    let ret = match fs::delete(&path) {
        Ok(_) => 0,
        Err(_) => -1,
    };
    ret
}

pub fn sys_dup(old_fd: usize) -> isize {
    let current_task = task::get_processor().current();
    let mut task_inner = current_task.inner_mut();

    let old = match task_inner.find_fd(old_fd) {
        Some(fd) => fd,
        None => return -1,
    };

    task_inner.alloc_fd(old) as isize
}
