// region TaskStatus begin
#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    Suspended,
    Running,
    Zombie,
}
// region TaskStatus end