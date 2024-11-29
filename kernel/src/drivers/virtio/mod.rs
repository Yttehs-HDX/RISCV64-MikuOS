use crate::{
    entry::KERNEL_ADDR_OFFSET,
    mm::{self, PhysAddr, PpnOffset},
};
use core::{mem::forget, ptr::NonNull};
use virtio_drivers::Hal;

// region VirtIOHal begin
pub struct VirtIOHal;

unsafe impl Hal for VirtIOHal {
    fn dma_alloc(
        pages: usize,
        _direction: virtio_drivers::BufferDirection,
    ) -> (virtio_drivers::PhysAddr, NonNull<u8>) {
        let ppn_list = mm::alloc_contiguous_ppn_tracker(pages).unwrap();
        let ppn_begin = ppn_list.last().unwrap().ppn();
        forget(ppn_list);

        let pa = ppn_begin.high_to_low().to_pa().0;
        let va = NonNull::new((ppn_begin.to_pa().0) as *mut u8).unwrap();

        (pa, va)
    }

    unsafe fn dma_dealloc(
        paddr: virtio_drivers::PhysAddr,
        _vaddr: core::ptr::NonNull<u8>,
        pages: usize,
    ) -> i32 {
        let ppn = PhysAddr(paddr).to_ppn().low_to_high();
        mm::dealloc_contiguous_ppn(ppn, pages);

        0
    }

    unsafe fn mmio_phys_to_virt(
        paddr: virtio_drivers::PhysAddr,
        _size: usize,
    ) -> core::ptr::NonNull<u8> {
        let va = paddr + KERNEL_ADDR_OFFSET;
        NonNull::new(va as *mut u8).unwrap()
    }

    unsafe fn share(
        buffer: core::ptr::NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) -> virtio_drivers::PhysAddr {
        let va = buffer.as_ptr() as *const u8 as usize;
        va - KERNEL_ADDR_OFFSET
    }

    unsafe fn unshare(
        _paddr: virtio_drivers::PhysAddr,
        _buffer: core::ptr::NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) {
        // Do nothing
    }
}
// region VirtIOHal end
