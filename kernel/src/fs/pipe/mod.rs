pub use ring_buffer::*;

use crate::{fs::File, task};
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

impl File for Pipe {
    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writable
    }

    fn read(&self, buf: &mut [u8]) -> usize {
        assert!(self.readable);
        let mut buf_iter = buf.iter_mut();
        let mut buf_len = 0usize;

        loop {
            let mut ring_buffer = self.buffer.lock();
            let loop_read = ring_buffer.read_bytes();
            if loop_read == 0 {
                if ring_buffer.all_write_ends_are_closed() {
                    return buf_len;
                }

                drop(ring_buffer);
                task::get_processor().current().get_trap_cx_mut().move_to_prev_ins();
                let current_task = task::get_processor().current();
                let task_inner = current_task.inner();
                if let Some(child) = task_inner.get_children_ref().iter().find(|pcb| !pcb.is_zombie()) {
                    let pid = child.get_pid();
                    drop(task_inner);
                    task::get_processor().wait_for_child(pid);
                }
                task::get_processor().schedule();
            }

            for _ in 0..loop_read {
                if let Some(byte) = buf_iter.next() {
                    *byte = ring_buffer.read_byte();
                    buf_len += 1;
                } else {
                    return buf_len;
                }
            }
        }
    }

    fn write(&self, buf: &[u8]) -> usize {
        assert!(self.writable);
        let want_to_write = buf.len();
        let mut buf_iter = buf.iter();
        let mut buf_len = 0usize;

        loop {
            let mut ring_buffer = self.buffer.lock();
            let loop_write = ring_buffer.write_bytes();
            if loop_write == 0 {
                drop(ring_buffer);
                task::get_processor().current().get_trap_cx_mut().move_to_prev_ins();
                let current_task = task::get_processor().current();
                let task_inner = current_task.inner();
                if let Some(child) = task_inner.get_children_ref().iter().find(|pcb| !pcb.is_zombie()) {
                    let pid = child.get_pid();
                    drop(task_inner);
                    task::get_processor().wait_for_child(pid);
                }
                task::get_processor().schedule();
            }

            for _ in 0..loop_write {
                if let Some(byte) = buf_iter.next() {
                    ring_buffer.write_byte(*byte);
                    buf_len += 1;

                    if buf_len == want_to_write {
                        return buf_len;
                    }
                } else {
                    return buf_len;
                }
            }
        }
    }
}
// region Pipe end
