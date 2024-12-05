pub use ring_buffer::*;

use alloc::sync::Arc;
use spin::Mutex;

mod ring_buffer;

pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
    let read_end = Arc::new(Pipe::new_read_end(buffer.clone()));
    let write_end = Arc::new(Pipe::new_write_end(buffer.clone()));
    buffer.lock().set_write_end(&write_end);
    (read_end, write_end)
}

// region Pipe begin
pub struct Pipe {
    readable: bool,
    writable: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
}

impl Pipe {
    pub fn new_read_end(buffer: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: true,
            writable: false,
            buffer,
        }
    }

    pub fn new_write_end(buffer: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: false,
            writable: true,
            buffer,
        }
    }
}
// region Pipe end
