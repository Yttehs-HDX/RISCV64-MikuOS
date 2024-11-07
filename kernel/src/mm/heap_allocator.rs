use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
use log::trace;

pub fn init_heap() {
    #[allow(static_mut_refs)]
    unsafe {
        trace!(
            "HeapAllocator: heap [{:#x?}, {:#x?})",
            KERNEL_HEAP.as_mut_ptr(),
            KERNEL_HEAP.as_mut_ptr().add(KERNEL_HEAP_SIZE)
        );
        HEAP_ALLOCATOR
            .lock()
            .init(KERNEL_HEAP.as_mut_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<KERNEL_HEAP_SIZE> = LockedHeap::empty();

// region KernelHeap begin
#[repr(align(4096))]
struct KernelHeap([u8; KERNEL_HEAP_SIZE]);

impl KernelHeap {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}
// region KernelHeap end

static mut KERNEL_HEAP: KernelHeap = KernelHeap([0; KERNEL_HEAP_SIZE]);
