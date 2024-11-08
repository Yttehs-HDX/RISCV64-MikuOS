pub use address::*;
pub use memory_set::*;
pub use page_table::*;
pub use ppn_allocator::*;
pub use space::*;

mod address;
mod heap_allocator;
mod memory_set;
mod page_table;
mod ppn_allocator;
mod space;

pub fn init() {
    heap_allocator::init_heap();
    get_kernel_space().activate();
}
