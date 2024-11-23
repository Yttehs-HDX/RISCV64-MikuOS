#![no_std]
#![no_main]

extern crate alloc;

pub use block_cache::*;

mod block_cache;

pub trait BlockDevice {
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
