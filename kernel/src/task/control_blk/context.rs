use crate::trap;

// region TaskContext begin
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskContext {
    ra: usize, // +0
    sp: usize, // +1
    s: [usize; 12], // +2 ~ +13
}

impl TaskContext {
    pub fn empty() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_restore(trap_cx: usize) -> Self {
        Self {
            ra: trap::__restore_trap as usize,
            sp: trap_cx,
            s: [0; 12],
        }
    }
}
// region TaskContext end
