use crate::fs::BlockDevice;
use alloc::boxed::Box;

// region Fat32IO begin
pub struct FatDeviceDriver {
    device: Box<dyn BlockDevice>,
}

unsafe impl Send for FatDeviceDriver {}
unsafe impl Sync for FatDeviceDriver {}

impl FatDeviceDriver {
    pub const fn new(device: Box<dyn BlockDevice>) -> Self {
        Self { device }
    }
}

impl FatDeviceDriver {
    fn read_inner(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        let len = buf.len();

        assert!(
            len <= 512,
            "buf.len() must be less than or equal to 512, found: {}",
            len
        );

        let device = &mut self.device;
        let device_offset = device.get_position() % 512;

        // Virtio_driver can only read 512 bytes at a time
        let size_read = if device_offset != 0 || len < 512 {
            let mut tmp = [0u8; 512];
            device.read_blocks(&mut tmp);

            let start = device_offset;
            let end = (device_offset + len).min(512);

            buf[..end - start].copy_from_slice(&tmp[start..end]);
            end - start
        } else {
            device.read_blocks(buf);
            512
        };

        device.move_cursor(size_read);
        Ok(size_read)
    }
}

impl fatfs::IoBase for FatDeviceDriver {
    type Error = ();
}

impl fatfs::Read for FatDeviceDriver {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        self.read_exact(buf).map(|_| buf.len()).map_err(|_| ())
    }

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), Self::Error> {
        while !buf.is_empty() {
            match buf.len() {
                0..=512 => {
                    let size = self.read_inner(buf)?;
                    buf = &mut buf[size..];
                }
                _ => {
                    let (left, right) = buf.split_at_mut(512);
                    self.read_inner(left)?;
                    buf = right;
                }
            }
        }
        if buf.is_empty() {
            Ok(())
        } else {
            log::debug!("failed to fill whole buffer in read_exact");
            Err(())
        }
    }
}

impl fatfs::Write for FatDeviceDriver {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let device = &mut self.device;
        let device_offset = device.get_position() % 512;

        let size_written = if device_offset != 0 || buf.len() < 512 {
            let mut tmp = [0u8; 512];
            device.read_blocks(&mut tmp);

            let start = device_offset;
            let end = (device_offset + buf.len()).min(512);

            tmp[start..end].copy_from_slice(&buf[..end - start]);
            device.write_blocks(&tmp);
            end - start
        } else {
            device.write_blocks(buf);
            512
        };

        device.move_cursor(size_written);
        Ok(size_written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl fatfs::Seek for FatDeviceDriver {
    fn seek(&mut self, pos: fatfs::SeekFrom) -> Result<u64, Self::Error> {
        let device = &mut self.device;
        match pos {
            fatfs::SeekFrom::Start(i) => {
                device.set_position(i as usize);
                Ok(i)
            }
            fatfs::SeekFrom::Current(i) => {
                let new_pos = (device.get_position() as i64) + i;
                device.set_position(new_pos as usize);
                Ok(new_pos as u64)
            }
            fatfs::SeekFrom::End(_) => unreachable!(),
        }
    }
}
// region Fat32IO end
