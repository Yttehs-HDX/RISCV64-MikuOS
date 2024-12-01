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
        // remove "./"
        let path = if path.starts_with("./") {
            &path[2..]
        } else {
            path
        };
        // construct a path with leading '/'
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };

        if let Some(pos) = path.rfind('/') {
            let parent_dir = &path[..pos];
            let file_name = &path[pos + 1..];

            // open parent directory
            let dir = self.inner.root_dir();
            let dir = if parent_dir.is_empty() {
                dir
            } else {
                dir.open_dir(parent_dir).unwrap()
            };

            // find the file in the directory
            let entry = dir
                .iter()
                .find(|entry| entry.as_ref().unwrap().file_name() == file_name);
            if let Some(file) = entry {
                let file = file.unwrap();
                let (readable, writable) = flags.read_write();
                let inode = FatInode::new(file, readable, writable);
                return Some(inode);
            }
        }
        None
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
