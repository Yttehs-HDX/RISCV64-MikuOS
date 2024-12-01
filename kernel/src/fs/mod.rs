pub use interface::*;
pub use stdio::*;

use alloc::sync::Arc;
use lazy_static::lazy_static;

pub mod fat;
mod interface;
mod stdio;

pub fn get_root_fs() -> Arc<dyn FileSystem> {
    ROOT_FILESYSTEM.clone()
}

lazy_static! {
    static ref ROOT_FILESYSTEM: Arc<dyn FileSystem> = Arc::new(fat::FatFileSystem::new(0));
}

pub trait FileDescriptor: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: &mut [u8]) -> usize;
    fn write(&self, buf: &[u8]) -> usize;
}
