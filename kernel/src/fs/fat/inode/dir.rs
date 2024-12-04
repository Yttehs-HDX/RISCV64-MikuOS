use crate::{fs::File, sync::UPSafeCell};
use core::cell::RefMut;

// region FatDir begin
pub struct FatDir {
    readable: bool,
    writable: bool,
    inner: UPSafeCell<FatDirInner<'static>>,
}

impl FatDir {
    pub fn new(inner: FatDirInner<'static>, readable: bool, writable: bool) -> Self {
        Self {
            readable,
            writable,
            inner: unsafe { UPSafeCell::new(inner) },
        }
    }

    #[allow(unused)]
    fn inner_mut(&self) -> RefMut<FatDirInner<'static>> {
        self.inner.exclusive_access()
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
}
// region FatDir end

type FatDirInner<'a> = fatfs::Dir<
    'a,
    crate::fs::fat::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
