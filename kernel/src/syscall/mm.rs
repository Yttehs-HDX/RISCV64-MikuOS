use crate::task;

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

pub fn sys_mmap(_: usize, _: usize, _: usize, _: usize, fd: usize, _: usize) -> isize {
    let current_task = task::get_processor().current();
    let mut task_inner = current_task.inner_mut();
    let ptr = task_inner.alloc_mmap(fd);

    ptr as isize
}

pub fn sys_munmap(start: usize) -> isize {
    let current_task = task::get_processor().current();
    let mut task_inner = current_task.inner_mut();
    task_inner.dealloc_mmap(start);

    0
}
