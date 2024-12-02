pub use interface::*;
pub use path::*;
pub use stdio::*;

use alloc::sync::Arc;
use lazy_static::lazy_static;

pub mod fat;
mod interface;
mod path;
mod stdio;

pub fn open_file(path: &str, flags: OpenFlags) -> Option<fat::FatInode> {
    ROOT_FILESYSTEM.open(path, flags)
}

lazy_static! {
    static ref ROOT_FILESYSTEM: Arc<dyn FileSystem> = Arc::new(fat::FatFileSystem::new(0));
}
