use crate::{app, mm, task, timer};
use log::{info, warn};

pub fn sys_exit(exit_code: i32) -> ! {
    match exit_code {
        0 => info!("Process exited with code {}", exit_code),
        _ => warn!("Process exited with code {}", exit_code),
    }
    task::get_processor().exit_current(exit_code);
}

pub fn sys_yield() -> ! {
    task::get_processor().schedule();
}

pub fn sys_get_time(ts_ptr: *const u8, _tz: usize) -> isize {
    let now = timer::get_current_time();
    mm::copy_data_to_space(
        task::get_processor().current().inner_mut().get_satp(),
        ts_ptr,
        &now,
    );
    0
}

pub fn sys_sbrk(increase: i32) -> isize {
    let old_brk = task::get_processor()
        .current()
        .inner_mut()
        .set_break(increase);
    match old_brk {
        Some(brk) => brk as isize,
        None => -1,
    }
}

pub fn sys_fork() -> isize {
    let current_task = task::get_processor().current();
    let new_task = current_task.fork();
    let new_pid = new_task.get_pid();
    let trap_cx = new_task.inner().get_trap_cx_mut();
    trap_cx.set_a0(0);
    task::add_task(new_task);

    new_pid as isize
}

pub fn sys_exec(path: *const u8, _argv: *const u8) -> isize {
    let path = mm::translate_str(task::get_processor().current().inner().get_satp(), path);
    if let Some(app) = app::get_app(&path) {
        let current_task = task::get_processor().current();
        current_task.exec(app.elf());
        0
    } else {
        -1
    }
}
