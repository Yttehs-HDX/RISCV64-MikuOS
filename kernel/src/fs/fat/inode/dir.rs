use alloc::vec::Vec;

use crate::fs::{fat, Directory, OpenFlags};

// region FatDir begin
pub struct FatDir<'a> {
    inner: FatDirInner<'a>,
}

impl<'a> FatDir<'a> {
    pub const fn new(inner: FatDirInner<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> Directory for FatDir<'a> {
    fn ls(&self) -> Vec<fat::FatInode> {
        self.inner
            .iter()
            .map(|entry| {
                let entry = entry.unwrap();
                let flags = OpenFlags::RDONLY;
                let (readable, writable) = flags.read_write();
                let inode = fat::FatInode::new(entry, readable, writable);
                inode
            })
            .collect()
    }
}
// region FatDir end

type FatDirInner<'a> = fatfs::Dir<
    'a,
    crate::fs::fat::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
