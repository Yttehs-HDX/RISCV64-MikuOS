use crate::fs::fat;
use alloc::{string::String, vec::Vec};
use bitflags::bitflags;

pub trait Inode: Send + Sync {
    fn name(&self) -> String;
    fn size(&self) -> usize;
    fn get_type(&self) -> InodeType;
    fn to_file(&self) -> fat::FatFile;
    fn to_dir(&self) -> fat::FatDir;
}

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: &mut [u8]) -> usize;
    fn write(&self, buf: &[u8]) -> usize;
}

pub trait Dir: Send + Sync {
    fn ls(&self) -> Vec<fat::FatInode>;
}

// region InodeType begin
#[derive(PartialEq, Eq)]
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
        const RDONLY = 0; // 0x0
        const WRONLY = 1; // 0x1
        const RDWR = 1 << 1; // 0x2
        const CREATE = 1 << 6; // 0x40
        const TRUNC = 1 << 10; // 0x400
        const DIRECTORY = 1 << 21; // 0x200000
    }
}

impl OpenFlags {
    pub const fn read_write(&self) -> (bool, bool) {
        match self {
            _ if self.contains(Self::RDWR) => (true, true),
            _ if self.contains(Self::WRONLY) => (false, true),
            _ => (true, false),
        }
    }

    pub const fn create(&self) -> bool {
        self.contains(OpenFlags::CREATE)
    }
}
// region OpenFlags end
