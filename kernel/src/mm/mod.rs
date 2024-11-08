pub use address::*;
pub use kernel_space::*;
pub use memory_set::*;
pub use page_table::*;
pub use ppn_allocator::*;
pub use user_space::*;

mod address;
mod heap_allocator;
mod kernel_space;
mod memory_set;
mod page_table;
mod ppn_allocator;
mod user_space;

pub fn init() {
    heap_allocator::init_heap();
    get_kernel_space().activate();
}
