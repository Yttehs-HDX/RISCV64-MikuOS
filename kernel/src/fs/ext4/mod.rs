pub use virtio::*;

use alloc::sync::Arc;
use core::ptr::NonNull;
use ext4_rs::Ext4;
use virtio_drivers::{device::blk::VirtIOBlk, transport::mmio::{MmioTransport, VirtIOHeader}};
use crate::{board::VIRT_IO, drivers::VirtIOHal};

mod virtio;

// region Ext4Fs begin
pub struct Ext4Fs {
    pub inner: Ext4,
}

impl Ext4Fs {
    pub fn new(device_id: usize) -> Self {
        let addr = VIRT_IO + device_id * 0x1000;
        let header = NonNull::new(addr as *mut VirtIOHeader).unwrap();
        let transport =
            unsafe { MmioTransport::new(header).expect("Failed to create mmio transport") };
        let blk = VirtIOBlk::<VirtIOHal, MmioTransport>::new(transport)
            .expect("Failed to create VirtIOBlk");

        let device = VirtIODisk::new(blk);
        let ext4 = Ext4::open(Arc::new(device));
        Self { inner: ext4 }
    }
}
// region Ext4Fs end
