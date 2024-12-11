use crate::{
    fs::{self, File, Inode, OpenFlags, PathUtil},
    syscall::translate_str,
    task::{self, Tms},
    timer::{self, TimeVal},
};
use alloc::vec;
use alloc::{string::ToString, vec::Vec};

pub fn sys_exit(exit_code: i32) -> ! {
    task::get_processor().exit_current(exit_code);
}

pub fn sys_yield() -> isize {
    task::get_processor().schedule();
}

pub fn sys_nanosleep(req_ptr: *const u8, _rem_ptr: usize) -> isize {
    let now = timer::get_current_time();
    let wait_until = unsafe { *(req_ptr as *const TimeVal) } + now;
    loop {
        let current_time = timer::get_current_time();
        if current_time >= wait_until {
            break;
        }
    }
    0
}

pub fn sys_getpid() -> isize {
    task::get_processor().current().get_pid() as isize
}

pub fn sys_getppid() -> isize {
    let current_task = task::get_processor().current();
    let parent_pid = current_task.get_ppid();
    parent_pid as isize
}

pub fn sys_clone(flags: usize, sp: usize) -> isize {
    const SIGCHLD: usize = 17;

    let current_task = task::get_processor().current();
    let new_task = current_task.fork();
    let new_pid = new_task.get_pid();
    if flags != SIGCHLD || sp != 0 {
        new_task.get_trap_cx_mut().set_sp(sp);
    }
    new_task.get_trap_cx_mut().set_a0(0);
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
        let mut buffer: Vec<u8> = vec![0; len];
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
        let pid = child.get_pid();
        let exit_code = child.get_exit_code();
        if !exit_code_ptr.is_null() {
            unsafe {
                match exit_code {
                    0 => {
                        *exit_code_ptr = exit_code;
                    }
                    _ => {
                        *exit_code_ptr = exit_code << 8;
                    }
                }
            }
        }

        // read tms
        let (cutime_inc, cstime_inc): (usize, usize);
        {
            let child_inner = child.inner();
            cutime_inc =
                child_inner.get_tms_ref().get_utime() + child_inner.get_tms_ref().get_cutime();
            cstime_inc =
                child_inner.get_tms_ref().get_stime() + child_inner.get_tms_ref().get_cstime();
            drop(child_inner);
        }

        children.retain(|c| c.get_pid() != pid);

        // update tms
        {
            task_inner.get_tms_mut().add_cstime(cstime_inc);
            task_inner.get_tms_mut().add_cutime(cutime_inc);
        }

        pid as isize
    } else {
        // child is not zombie
        let pid = if pid == -1 {
            children
                .iter()
                .find(|child| !child.is_zombie())
                .unwrap()
                .get_pid()
        } else {
            pid as usize
        };
        drop(task_inner);
        current_task.get_trap_cx_mut().move_to_prev_ins();
        task::get_processor().wait_for_child(pid);
    }
}

pub fn sys_times(buf: *const u8) -> isize {
    let current_time = timer::get_current_tick();

    let current_task = task::get_processor().current();
    let inner = current_task.inner();
    let tms = inner.get_tms_ref();
    unsafe {
        *(buf as *mut Tms) = *tms;
    }

    current_time as isize
}
