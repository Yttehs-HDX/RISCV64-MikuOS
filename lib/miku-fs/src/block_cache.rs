use crate::BlockDevice;
use alloc::sync::Arc;

pub const BLOCK_SIZE: usize = 512;

// region BlockCache begin
pub struct BlockCache {
    cache: [u8; BLOCK_SIZE],
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        if self.is_modified() {
            self.block_device.write_block(self.block_id, &self.cache);
            self.modified = false;
        }
    }
}

impl BlockCache {
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0; BLOCK_SIZE];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache: [0; BLOCK_SIZE],
            block_id,
            block_device,
            modified: false,
        }
    }

    pub const fn is_modified(&self) -> bool {
        self.modified
    }
}

impl BlockCache {
    fn get_addr_by_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }

    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let size = core::mem::size_of::<T>();
        assert!(offset + size <= BLOCK_SIZE);
        let addr = self.get_addr_by_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let size = core::mem::size_of::<T>();
        assert!(offset + size <= BLOCK_SIZE);
        let addr = self.get_addr_by_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        let value = self.get_ref(offset);
        f(value)
    }
}
// region BlockCache end
