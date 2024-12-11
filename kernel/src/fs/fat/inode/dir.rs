use crate::{
    fs::{File, LinuxDirent64},
    sync::UPSafeCell,
};
use alloc::{string::String, vec::Vec};
use core::cell::Ref;

// region FatDir begin
pub struct FatDir {
    readable: bool,
    writable: bool,
    path: String,
    inner: UPSafeCell<FatDirInner<'static>>,
}

impl FatDir {
    pub fn new(path: String, inner: FatDirInner<'static>, readable: bool, writable: bool) -> Self {
        Self {
            readable,
            writable,
            path,
            inner: unsafe { UPSafeCell::new(inner) },
        }
    }

    fn inner(&self) -> Ref<FatDirInner<'static>> {
        self.inner.shared_access()
    }
}

impl FatDir {
    pub fn get_entries(&self) -> Vec<LinuxDirent64> {
        let mut entries = Vec::new();
        self.inner().iter().for_each(|entry| {
            let entry = entry.unwrap();
            let name = entry.file_name();
            let dent = LinuxDirent64::new(&name);
            entries.push(dent);
        });
        entries
    }
}

unsafe impl Send for FatDir {}
unsafe impl Sync for FatDir {}

impl File for FatDir {
    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writable
    }

    fn read(&self, _buf: &mut [u8]) -> usize {
        0
    }

    fn write(&self, _buf: &[u8]) -> usize {
        0
    }

    fn path(&self) -> String {
        self.path.clone()
    }
}
// region FatDir end

type FatDirInner<'a> = fatfs::Dir<
    'a,
    crate::fs::fat::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
