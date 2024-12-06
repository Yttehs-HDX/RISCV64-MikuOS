use crate::{
    fs::{self, File, Inode, OpenFlags, PathUtil},
    syscall::translate_str,
    task,
    timer::{self, TimeVal},
};
use alloc::{string::ToString, vec::Vec};

pub fn sys_exit(exit_code: i32) -> ! {
    task::get_processor().exit_current(exit_code);
}

pub fn sys_yield() -> isize {
    task::get_processor().schedule();
    0
}

pub fn sys_get_time(ts_ptr: *mut u8, _tz: usize) -> isize {
    let ts_ptr = ts_ptr as *mut TimeVal;
    let now = timer::get_current_time();
    unsafe {
        *ts_ptr = now;
    }
    0
}

pub fn sys_brk(new_end: i32) -> isize {
    if new_end == 0 {
        return sys_sbrk(0);
    }
    let current_brk = sys_sbrk(0);
    let inc = new_end - current_brk as i32;
    sys_sbrk(inc)
}

pub fn sys_sbrk(increase: i32) -> isize {
    let old_brk = task::get_processor().current().set_break(increase);
    match old_brk {
        Some(brk) => brk as isize,
        None => -1,
    }
}

pub fn sys_getpid() -> isize {
    task::get_processor().current().get_pid() as isize
}

pub fn sys_getppid() -> isize {
    let current_task = task::get_processor().current();
    let parent_pid = current_task.get_ppid();
    parent_pid as isize
}

pub fn sys_fork() -> isize {
    let current_task = task::get_processor().current();
    let new_task = current_task.fork();
    let new_pid = new_task.get_pid();
    let trap_cx = new_task.get_trap_cx_mut();
    trap_cx.set_a0(0);
    task::add_task(new_task);

    new_pid as isize
}

pub fn sys_exec(path_ptr: *const u8, _argv: *const u8) -> isize {
    let path = translate_str(path_ptr);
    let path = PathUtil::from_user(path).to_string();
    let entry = fs::open_file(&path, OpenFlags::RDONLY);

    if let Some(entry) = entry {
        // get target file
        let len = entry.size();
        let file = entry.to_file();

        // prepare mut buffer
        #[allow(clippy::uninit_vec)]
        let mut buffer: Vec<u8> = Vec::with_capacity(len);
        #[allow(clippy::uninit_vec)]
        unsafe {
            buffer.set_len(len);
        }
        let buffer = buffer.as_mut_slice();
        file.read(buffer);

        // execute task
        let current_task = task::get_processor().current();
        current_task.exec(buffer);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32, _option: usize) -> isize {
    let current_task = task::get_processor().current();
    let mut task_inner = current_task.inner_mut();
    let children = task_inner.get_children_mut();

    // pid not found
    if !children
        .iter()
        .any(|child| child.get_pid() == pid as usize || pid == -1)
    {
        return -1;
    }

    if let Some(child) = children
        .iter()
        .find(|child| (child.get_pid() == pid as usize || pid == -1) && child.is_zombie())
    {
        // child is zombie
        let exit_code = child.get_exit_code();
        unsafe { *exit_code_ptr = exit_code };
        children.retain(|c| c.get_pid() != pid as usize);
        pid
    } else {
        // child is not zombie
        -2
    }
}
