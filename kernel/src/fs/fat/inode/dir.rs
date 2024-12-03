use crate::{
    fs::{fat, Dir, OpenFlags},
    sync::UPSafeCell,
};
use alloc::vec::Vec;
use core::cell::Ref;

// region FatDir begin
pub struct FatDir {
    inner: UPSafeCell<FatDirInner<'static>>,
}

impl FatDir {
    pub fn new(inner: FatDirInner<'static>) -> Self {
        Self {
            inner: unsafe { UPSafeCell::new(inner) },
        }
    }

    fn inner(&self) -> Ref<FatDirInner<'static>> {
        self.inner.shared_access()
    }
}

unsafe impl Send for FatDir {}
unsafe impl Sync for FatDir {}

impl Dir for FatDir {
    fn ls(&self) -> Vec<fat::FatInode> {
        let inner = self.inner();
        inner
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
