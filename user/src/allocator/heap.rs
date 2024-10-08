use buddy_system_allocator::LockedHeap;

const USER_HEAP_SIZE: usize = 0x1000;

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR.lock().init(KERNEL_HEAP.as_ptr() as usize, USER_HEAP_SIZE);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<USER_HEAP_SIZE> = LockedHeap::empty();

static KERNEL_HEAP: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];