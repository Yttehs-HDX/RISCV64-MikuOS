pub use stdio::*;

use alloc::sync::Arc;
use lazy_static::lazy_static;

pub mod fat;
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

pub trait BlockDevice {
    fn read_blocks(&mut self, buf: &mut [u8]);
    fn write_blocks(&mut self, buf: &[u8]);
    fn get_position(&self) -> usize;
    fn set_position(&mut self, position: usize);
    fn move_cursor(&mut self, amount: usize);
}

pub trait FileSystem: Send + Sync {
    fn root_dir(&self) -> Dir;
    fn open(&self, path: &str) -> Option<Entry>;
}

type Entry<'a> =
    fatfs::DirEntry<'a, fat::FatDeviceDriver, fatfs::NullTimeProvider, fatfs::LossyOemCpConverter>;
type Dir<'a> =
    fatfs::Dir<'a, fat::FatDeviceDriver, fatfs::NullTimeProvider, fatfs::LossyOemCpConverter>;
