pub use address::*;
pub use memory_set::*;
pub use page_table::*;
pub use ppn_allocator::*;

mod address;
mod heap_allocator;
mod memory_set;
mod page_table;
mod ppn_allocator;

pub fn init() {
    heap_allocator::init_heap();
    memory_set::activate_kernel_space();
}
