use fatfs::{Read, Write};
use crate::fs::File;

// region FatFile begin
pub struct FatFile<'a> {
    inner: FatFileInner<'a>,
}

impl<'a> FatFile<'a> {
    pub fn new(inner: FatFileInner<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> File for FatFile<'a> {
    fn read(&mut self, buf: &mut [u8]) -> usize {
        self.inner.read_exact(buf).ok().unwrap();
        buf.len()
    }

    fn write(&mut self, buf: &[u8]) -> usize {
        self.inner.write_all(buf).ok().unwrap();
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
