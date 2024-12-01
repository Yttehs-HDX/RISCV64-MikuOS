pub use dir::*;
pub use file::*;

use crate::fs::{Inode, InodeType};

mod dir;
mod file;

// region FatInode begin
pub struct FatInode<'a> {
    inner: FatInodeInner<'a>,
}

impl<'a> FatInode<'a> {
    pub const fn new(inner: FatInodeInner<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> Inode for FatInode<'a> {
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

    fn to_file(&self) -> FatFile<'a> {
        assert!(self.inner.is_file());
        FatFile::new(self.inner.to_file())
    }

    fn to_dir(&self) -> super::FatDir {
        assert!(self.inner.is_dir());
        super::FatDir::new(self.inner.to_dir())
    }
}
// region FatInode end

type FatInodeInner<'a> = fatfs::DirEntry<
    'a,
    super::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
