pub use context::*;
pub use manager::*;
pub use pcb::*;
pub use pid::*;
pub use processor::*;
pub use tms::*;

mod context;
mod manager;
mod pcb;
mod pid;
mod processor;
mod tms;

pub fn init() {
    #[cfg(not(feature = "test"))]
    add_initproc();

    #[cfg(feature = "test")]
    get_initproc();
}
