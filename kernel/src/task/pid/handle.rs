use super::dealloc_pid;

// region PidHandle begin
pub struct PidHandle(pub usize);

impl Drop for PidHandle {
    fn drop(&mut self) {
        let pid = self.0;
        dealloc_pid(pid);
    }
}
// region PidHandle end
