pub use switch::*;

mod switch;

// region TaskContext begin
#[repr(C)]
pub struct TaskContext {
    s: [usize; 12], // +0 ~ +11
}

impl TaskContext {
    pub fn empty() -> Self {
        Self { s: [0; 12] }
    }
}
// region TaskContext end
