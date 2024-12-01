use crate::fs::fat;
use alloc::{string::String, vec::Vec};
use bitflags::bitflags;

pub trait Inode {
    fn name(&self) -> String;
    fn size(&self) -> usize;
    fn get_type(&self) -> InodeType;
    fn to_file(&self) -> fat::FatFile;
    fn to_dir(&self) -> fat::FatDir;
}

pub trait File {
    fn read(&mut self, buf: &mut [u8]) -> usize;
    fn write(&mut self, buf: &[u8]) -> usize;
}

pub trait Directory {
    fn ls(&self) -> Vec<fat::FatInode>;
}

// region InodeType begin
pub enum InodeType {
    Unknown,
    File,
    Dir,
    CharDevice,
}
// region InodeType end

// region OpenFlags begin
bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

impl OpenFlags {
    pub const fn read_write(&self) -> (bool, bool) {
        match self {
            _ if self.is_empty() => (false, false),
            _ if self.contains(Self::RDONLY) => (true, false),
            _ if self.contains(Self::WRONLY) => (false, true),
            _ if self.contains(Self::RDWR) => (true, true),
            _ => (false, false),
        }
    }

    pub const fn create(&self) -> bool {
        self.contains(OpenFlags::CREATE)
    }
}
// region OpenFlags end
