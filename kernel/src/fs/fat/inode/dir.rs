use alloc::vec::Vec;

use crate::fs::Directory;

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
    fn ls(&self) -> Vec<crate::fs::fat::FatInode> {
        self.inner
            .iter()
            .map(|entry| {
                let entry = entry.unwrap();
                let inode = crate::fs::fat::FatInode::new(entry);
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
