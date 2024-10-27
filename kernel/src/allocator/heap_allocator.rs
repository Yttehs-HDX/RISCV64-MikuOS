use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(KERNEL_HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<KERNEL_HEAP_SIZE> = LockedHeap::empty();

static KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
