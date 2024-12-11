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
