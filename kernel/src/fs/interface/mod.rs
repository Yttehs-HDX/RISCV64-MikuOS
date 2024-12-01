pub use inode::*;
pub use virtio::*;

mod inode;
mod virtio;

pub trait FileSystem: Send + Sync {
    fn open(&self, path: &str) -> Option<super::fat::FatInode>;
}
