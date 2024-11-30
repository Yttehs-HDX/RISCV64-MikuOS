use crate::{drivers::VirtIOHal, fs::BlockDevice};
use virtio_drivers::{device::blk::VirtIOBlk, transport::mmio::MmioTransport};

const SECTOR_SIZE: usize = 512;

// region VirtIODisk begin
pub struct VirtIODisk {
    sector: usize,
    offset: usize,
    inner: VirtIODiskInner,
}

impl VirtIODisk {
    pub fn new(virt_io_blk: VirtIODiskInner) -> Self {
        VirtIODisk {
            sector: 0,
            offset: 0,
            inner: virt_io_blk,
        }
    }
}

impl BlockDevice for VirtIODisk {
    fn read_blocks(&mut self, buf: &mut [u8]) {
        self.inner
            .read_blocks(self.sector, buf)
            .expect("Error occurred when reading VirtIOBlk");
    }

    fn write_blocks(&mut self, buf: &[u8]) {
        self.inner
            .write_blocks(self.sector, buf)
            .expect("Error occurred when writing VirtIOBlk");
    }

    fn get_position(&self) -> usize {
        self.sector * SECTOR_SIZE + self.offset
    }

    fn set_position(&mut self, position: usize) {
        self.sector = position / SECTOR_SIZE;
        self.offset = position % SECTOR_SIZE;
    }

    fn move_cursor(&mut self, amount: usize) {
        self.set_position(self.get_position() + amount)
    }
}
// region VirtIODisk end

type VirtIODiskInner = VirtIOBlk<VirtIOHal, MmioTransport>;
