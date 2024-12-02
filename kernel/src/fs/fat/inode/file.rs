use crate::{fs::File, sync::UPSafeCell};
use core::cell::RefMut;
use fatfs::{Read, Write};

// region FatFile begin
pub struct FatFile<'a> {
    readable: bool,
    writable: bool,
    inner: UPSafeCell<FatFileInner<'a>>,
}

impl<'a> FatFile<'a> {
    pub fn new(inner: FatFileInner<'a>, readable: bool, writable: bool) -> Self {
        Self {
            readable,
            writable,
            inner: unsafe { UPSafeCell::new(inner) },
        }
    }

    fn inner_mut(&self) -> RefMut<FatFileInner<'a>> {
        self.inner.exclusive_access()
    }
}

impl<'a> File for FatFile<'a> {
    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writable
    }

    fn read(&self, buf: &mut [u8]) -> usize {
        assert!(self.readable);
        let mut inner = self.inner_mut();
        inner.read_exact(buf).ok().unwrap();
        buf.len()
    }

    fn write(&self, buf: &[u8]) -> usize {
        assert!(self.writable);
        let mut inner = self.inner_mut();
        inner.write_all(buf).ok().unwrap();
        buf.len()
    }
}
// region FatFile end

type FatFileInner<'a> = fatfs::File<
    'a,
    crate::fs::fat::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
