use core::cell::RefMut;
use crate::{drivers::VirtIOHal, sync::UPSafeCell};
use alloc::vec;
use ext4_rs::BlockDevice;
use virtio_drivers::{device::blk::VirtIOBlk, transport::mmio::MmioTransport};

const BLOCK_SIZE: usize = 512;

// region VirtIODisk begin
pub struct VirtIODisk {
    inner: UPSafeCell<VirtIODiskInner>,
}

impl VirtIODisk {
    pub fn new(virt_io_blk: VirtIODiskInner) -> Self {
        VirtIODisk {
            inner: unsafe { UPSafeCell::new(virt_io_blk) },
        }
    }

    fn inner_mut(&self) -> RefMut<VirtIODiskInner> {
        self.inner.exclusive_access()
    }
}

impl BlockDevice for VirtIODisk {
    fn read_offset(&self, offset: usize) -> alloc::vec::Vec<u8> {
        let mut inner = self.inner_mut();
        let mut buf = vec![0; BLOCK_SIZE];
        inner
            .read_blocks(offset, &mut buf)
            .expect("Error occurred when reading VirtIOBlk");
        buf
    }

    fn write_offset(&self, offset: usize, data: &[u8]) {
        let mut inner = self.inner_mut();
        inner
            .write_blocks(offset, &data)
            .expect("Error occurred when writing VirtIOBlk");
    }
}
// region VirtIODisk end

type VirtIODiskInner = VirtIOBlk<VirtIOHal, MmioTransport>;
