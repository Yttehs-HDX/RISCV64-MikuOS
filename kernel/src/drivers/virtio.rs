use crate::mm::{self, PhysAddr, PpnOffset};
use core::ptr::NonNull;
use virtio_drivers::Hal;

// region VirtIOHal begin
pub struct VirtIOHal;

unsafe impl Hal for VirtIOHal {
    fn dma_alloc(
        pages: usize,
        _direction: virtio_drivers::BufferDirection,
    ) -> (virtio_drivers::PhysAddr, core::ptr::NonNull<u8>) {
        let ppn_vec = mm::alloc_contiguous_ppn_tracker(pages).unwrap();
        let pa = ppn_vec[0].ppn().high_to_low().to_pa().0;
        let va = NonNull::new(pa as *mut u8).unwrap();
        (pa, va)
    }

    unsafe fn dma_dealloc(
        paddr: virtio_drivers::PhysAddr,
        _vaddr: core::ptr::NonNull<u8>,
        pages: usize,
    ) -> i32 {
        let start_ppn = PhysAddr(paddr).to_ppn().low_to_high();
        mm::dealloc_contiguous_ppn(start_ppn, pages);
        0
    }

    unsafe fn mmio_phys_to_virt(
        paddr: virtio_drivers::PhysAddr,
        _size: usize,
    ) -> core::ptr::NonNull<u8> {
        NonNull::new(paddr as *mut u8).unwrap()
    }

    unsafe fn share(
        buffer: core::ptr::NonNull<[u8]>,
        direction: virtio_drivers::BufferDirection,
    ) -> virtio_drivers::PhysAddr {
        buffer.as_ptr() as *mut u8 as virtio_drivers::PhysAddr
    }

    unsafe fn unshare(
        paddr: virtio_drivers::PhysAddr,
        buffer: core::ptr::NonNull<[u8]>,
        direction: virtio_drivers::BufferDirection,
    ) {
        // Do nothing
    }
}
// region VirtIOHal end
