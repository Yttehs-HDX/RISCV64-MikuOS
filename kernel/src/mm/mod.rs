pub use address::*;
pub use kernel_space::*;
pub use memory_set::*;
pub use page_table::*;
pub use ppn_allocator::*;

mod address;
mod heap_allocator;
mod kernel_space;
mod memory_set;
mod page_table;
mod ppn_allocator;

pub fn init() {
    heap_allocator::init_heap();
    get_kernel_space().activate();
}
