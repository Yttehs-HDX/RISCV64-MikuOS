pub use context::*;
pub use manager::*;
pub use pcb::*;
pub use pid::*;
pub use processor::*;

mod context;
mod manager;
mod pcb;
mod pid;
mod processor;

pub fn init() {
    add_initproc();
}
