pub use interface::*;
pub use path::*;
pub use pipe::*;
pub use stdio::*;

use alloc::sync::Arc;
use lazy_static::lazy_static;

pub mod fat;
mod interface;
mod path;
mod pipe;
mod stdio;

pub fn open_file(path: &str, flags: OpenFlags) -> Option<fat::FatInode> {
    ROOT_FILESYSTEM.open(path, flags)
}

pub fn create_dir(path: &str, mode: usize) -> bool {
    ROOT_FILESYSTEM.create_dir(path, mode)
}

pub fn delete(path: &str) -> Result<(), ()> {
    ROOT_FILESYSTEM.delete(path)
}

pub fn open_inode(path: &str) -> Option<fat::FatInode> {
    open_file(path, OpenFlags::RDONLY)
}

lazy_static! {
    static ref ROOT_FILESYSTEM: Arc<dyn FileSystem> = Arc::new(fat::FatFileSystem::new(0));
}
