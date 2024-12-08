pub use inode::*;
pub use virtio::*;

mod inode;
mod virtio;

pub trait FileSystem: Send + Sync {
    fn open(&'static self, path: &str, flags: OpenFlags) -> Option<super::fat::FatInode>;
    fn create_dir(&'static self, path: &str, mode: usize) -> bool;
    fn delete(&'static self, path: &str) -> Result<(), ()>;
}
