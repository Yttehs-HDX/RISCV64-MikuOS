pub use driver::*;
pub use inode::*;
pub use virtio::*;

use crate::{
    config::VIRT_IO,
    drivers::VirtIOHal,
    fs::{FileSystem, OpenFlags},
};
use alloc::{boxed::Box, format, string::ToString};
use core::ptr::NonNull;
use fatfs::{FsOptions, LossyOemCpConverter, NullTimeProvider};
use virtio_drivers::{
    device::blk::VirtIOBlk,
    transport::mmio::{MmioTransport, VirtIOHeader},
};

use super::Path;

mod driver;
mod inode;
mod virtio;

// region FatFileSystem begin
pub struct FatFileSystem {
    inner: FatFileSystemInner,
}

unsafe impl Send for FatFileSystem {}
unsafe impl Sync for FatFileSystem {}

impl FileSystem for FatFileSystem {
    fn open(&self, path: &str, flags: OpenFlags) -> Option<FatInode> {
        let path = Path::from_str(path);
        let parent = path.parent();
        let name = path.name();

        // open parent directory
        let dir = self.inner.root_dir();
        let dir = if parent == "/" {
            dir
        } else {
            let dir = dir.open_dir(parent);
            match dir {
                Ok(dir) => dir,
                Err(_) => return None,
            }
        };

        // find the file in the directory
        let entry = dir
            .iter()
            .find(|entry| entry.as_ref().unwrap().file_name() == name);
        if let Some(file) = entry {
            let file = file.unwrap();
            let (readable, writable) = flags.read_write();
            let inode = FatInode::new(file, readable, writable);
            Some(inode)
        } else {
            None
        }
    }
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
        let inner = fatfs::FileSystem::new(io, FsOptions::new()).unwrap();

        Self { inner }
    }
}
// region FatFileSystem end

type FatFileSystemInner = fatfs::FileSystem<FatDeviceDriver, NullTimeProvider, LossyOemCpConverter>;