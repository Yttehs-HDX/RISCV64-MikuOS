use driver::*;
use virtio::*;

use crate::{config::VIRT_IO, drivers::VirtIOHal, sync::UPSafeCell};
use alloc::boxed::Box;
use core::{cell::Ref, ptr::NonNull};
use fatfs::{FileSystem, FsOptions, LossyOemCpConverter, NullTimeProvider};
use virtio_drivers::{
    device::blk::VirtIOBlk,
    transport::mmio::{MmioTransport, VirtIOHeader},
};

mod driver;
mod virtio;

// region FatFileSystem begin
pub struct FatFileSystem {
    inner: UPSafeCell<FatFileSystemInner>,
}

impl FatFileSystem {
    pub fn new(device_id: usize) -> Self {
        let addr = VIRT_IO + device_id * 0x1000;
        let header = NonNull::new(addr as *mut VirtIOHeader).unwrap();
        let transport =
            unsafe { MmioTransport::new(header).expect("Failed to create mmio transport") };
        let blk = VirtIOBlk::<VirtIOHal, MmioTransport>::new(transport)
            .expect("Failed to create VirtIOBlk");
        let device = Box::new(VirtIODisk::new(blk));
        let io = FatDeviceDriver::new(device);

        let inner = FileSystem::new(io, FsOptions::new()).unwrap();

        Self {
            inner: unsafe { UPSafeCell::new(inner) },
        }
    }

    pub fn inner(&self) -> Ref<FatFileSystemInner> {
        self.inner.shared_access()
    }
}
// region FatFileSystem end

type FatFileSystemInner = FileSystem<FatDeviceDriver, NullTimeProvider, LossyOemCpConverter>;
