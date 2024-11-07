use crate::trap;

// region TaskContext begin
#[repr(C)]
pub struct TaskContext {
    ra: usize,      // +0
    sp: usize,      // +1
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

    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        Self {
            ra: trap::trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
// region TaskContext end
