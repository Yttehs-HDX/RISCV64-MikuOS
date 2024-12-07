pub use dir::*;
pub use file::*;

use crate::fs::{Inode, InodeType};

mod dir;
mod file;

// region FatInode begin
pub struct FatInode {
    readable: bool,
    writable: bool,
    inner: FatInodeInner<'static>,
}

impl FatInode {
    pub fn new(inner: FatInodeInner<'static>, readable: bool, writable: bool) -> Self {
        Self {
            readable,
            writable,
            inner,
        }
    }
}

unsafe impl Sync for FatInode {}
unsafe impl Send for FatInode {}

impl Inode for FatInode {
    fn name(&self) -> alloc::string::String {
        self.inner.file_name()
    }

    fn size(&self) -> usize {
        self.inner.len() as usize
    }

    fn get_type(&self) -> InodeType {
        match self {
            _ if self.inner.is_file() => InodeType::File,
            _ if self.inner.is_dir() => InodeType::Dir,
            _ => InodeType::Unknown,
        }
    }

    fn to_file(&self) -> FatFile {
        assert!(self.inner.is_file());
        FatFile::new(self.inner.to_file(), self.readable, self.writable)
    }

    fn to_dir(&self) -> FatDir {
        assert!(self.inner.is_dir());
        FatDir::new(self.inner.to_dir(), self.readable, self.writable)
    }
}
// region FatInode end

type FatInodeInner<'a> = fatfs::DirEntry<
    'a,
    super::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
