use crate::fs::fat;
use alloc::{string::String, vec::Vec};

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
