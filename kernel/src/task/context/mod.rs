pub use switch::*;

use crate::trap;

mod switch;

// region TaskContext begin
#[repr(C)]
pub struct TaskContext {
    ra: usize,      // +0
    sp: usize,      // +1
    s: [usize; 12], // +2 ~ +13
}

impl TaskContext {
    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        Self {
            ra: trap::trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
// region TaskContext end
