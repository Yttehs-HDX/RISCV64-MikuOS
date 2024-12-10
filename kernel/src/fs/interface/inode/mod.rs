pub use linux_dent::*;
pub use open_flags::*;

use crate::fs::fat;
use alloc::string::String;

mod linux_dent;
mod open_flags;

pub trait Inode: Send + Sync {
    #[allow(unused)]
    fn name(&self) -> String;
    fn size(&self) -> usize;
    fn get_type(&self) -> InodeType;
    fn to_file(&self) -> fat::FatFile;
    fn to_dir(&self) -> fat::FatDir;
    fn atime(&self) -> (usize, usize);
    fn mtime(&self) -> (usize, usize);
    fn ctime(&self) -> (usize, usize);
}

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: &mut [u8]) -> usize;
    fn write(&self, buf: &[u8]) -> usize;
    fn path(&self) -> String;
}

// region InodeType begin
#[derive(PartialEq, Eq)]
pub enum InodeType {
    Unknown,
    File,
    Dir,
    #[allow(unused)]
    CharDevice,
}
// region InodeType end
