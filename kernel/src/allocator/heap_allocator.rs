use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
use log::trace;

pub fn init_heap() {
    unsafe {
        trace!(
            "HeapAllocator: heap [{:#x?}, {:#x?})",
            KERNEL_HEAP.as_ptr(),
            KERNEL_HEAP.as_ptr().offset(KERNEL_HEAP_SIZE as isize)
        );
        #[allow(static_mut_refs)]
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

static mut KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
