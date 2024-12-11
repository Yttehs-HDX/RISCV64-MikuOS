use crate::{fs::{File, Inode}, sync::UPSafeCell};
use alloc::string::String;
use core::cell::RefMut;
use fatfs::{Read, Write};

// region FatFile begin
pub struct FatFile {
    readable: bool,
    writable: bool,
    path: String,
    inner: UPSafeCell<FatFileInner<'static>>,
}

impl FatFile {
    pub fn new(path: String, inner: FatFileInner<'static>, readable: bool, writable: bool) -> Self {
        Self {
            readable,
            writable,
            path,
            inner: unsafe { UPSafeCell::new(inner) },
        }
    }

    fn inner_mut(&self) -> RefMut<FatFileInner<'static>> {
        self.inner.exclusive_access()
    }
}

unsafe impl Send for FatFile {}
unsafe impl Sync for FatFile {}

impl File for FatFile {
    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writable
    }

    fn read(&self, buf: &mut [u8]) -> usize {
        assert!(self.readable);
        let mut inner = self.inner_mut();
        inner.read_exact(buf).ok();
        buf.len()
    }

    fn write(&self, buf: &[u8]) -> usize {
        assert!(self.writable);
        let mut inner = self.inner_mut();
        inner.write_all(buf).ok();
        inner.flush().ok();
        buf.len()
    }

    fn path(&self) -> String {
        self.path.clone()
    }
}
// region FatFile end

type FatFileInner<'a> = fatfs::File<
    'a,
    crate::fs::fat::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
