use super::TaskContext;

// region TaskControlBlock begin
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub context: TaskContext,
}
// region TaskControlBlock end

// region TaskStatus begin
pub enum TaskStatus {
    Suspended,
    Running,
    Zombie,
}
// region TaskStatus end