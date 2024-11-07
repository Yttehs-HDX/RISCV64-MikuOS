use crate::sbrk;
use buddy_system_allocator::LockedHeap;

pub fn init_heap() {
    let heap_size = 4096;
    let heap_start = sbrk(heap_size);
    unsafe {
        ALLOCATOR
            .lock()
            .init(heap_start as usize, heap_size as usize);
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeap<4096> = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
