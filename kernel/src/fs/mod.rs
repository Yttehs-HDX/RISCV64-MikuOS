pub use fat::*;
pub use stdio::*;

use lazy_static::lazy_static;

mod fat;
mod stdio;

pub fn get_root_fs() -> &'static FatFileSystem {
    &ROOT_FILESYSTEM
}

lazy_static! {
    static ref ROOT_FILESYSTEM: FatFileSystem = FatFileSystem::new(0);
}

pub trait File: Send + Sync {
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
