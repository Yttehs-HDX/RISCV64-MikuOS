use crate::{
    app, task,
    timer::{self, TimeVal},
};

pub fn sys_exit(exit_code: i32) -> ! {
    task::get_processor().exit_current(exit_code);
}

pub fn sys_yield() -> ! {
    task::get_processor().schedule();
}

pub fn sys_get_time(ts_ptr: *mut u8, _tz: usize) -> isize {
    let ts_ptr = ts_ptr as *mut TimeVal;
    let now = timer::get_current_time();
    unsafe {
        *ts_ptr = now;
    }
    0
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

pub fn sys_fork() -> isize {
    let current_task = task::get_processor().current();
    let new_task = current_task.fork();
    let new_pid = new_task.get_pid();
    let trap_cx = new_task.get_trap_cx_mut();
    trap_cx.set_a0(0);
    task::add_task(new_task);

    new_pid as isize
}

pub fn sys_exec(path: *const u8, _argv: *const u8) -> isize {
    let str: &str;
    unsafe {
        let mut len = 0;
        while *path.add(len) != 0 {
            len += 1;
        }
        str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(path, len));
    }

    if let Some(app) = app::get_app(str) {
        let current_task = task::get_processor().current();
        current_task.exec(app.elf());
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
