pub use dir::*;
pub use file::*;

use crate::fs::{Inode, InodeType};
use alloc::string::String;

mod dir;
mod file;

// region FatInode begin
pub struct FatInode {
    readable: bool,
    writable: bool,
    path: String,
    inner: FatInodeInner<'static>,
}

impl FatInode {
    pub fn new(
        path: String,
        inner: FatInodeInner<'static>,
        readable: bool,
        writable: bool,
    ) -> Self {
        Self {
            readable,
            writable,
            path,
            inner,
        }
    }
}

unsafe impl Sync for FatInode {}
unsafe impl Send for FatInode {}

impl Inode for FatInode {
    fn name(&self) -> alloc::string::String {
        self.inner.file_name()
    }

    fn size(&self) -> usize {
        self.inner.len() as usize
    }

    fn get_type(&self) -> InodeType {
        match self {
            _ if self.inner.is_file() => InodeType::File,
            _ if self.inner.is_dir() => InodeType::Dir,
            _ => InodeType::Unknown,
        }
    }

    fn to_file(&self) -> FatFile {
        assert!(self.inner.is_file());
        FatFile::new(
            self.path.clone(),
            self.inner.to_file(),
            self.readable,
            self.writable,
        )
    }

    fn to_dir(&self) -> FatDir {
        assert!(self.inner.is_dir());
        FatDir::new(
            self.path.clone(),
            self.inner.to_dir(),
            self.readable,
            self.writable,
        )
    }

    fn atime(&self) -> (usize, usize) {
        let year = self.inner.accessed().year as usize;
        let month = self.inner.accessed().month as usize;
        let day = self.inner.accessed().day as usize;

        let secs = calculate_sec(year, month, day, 0, 0, 0);
        (secs, 0)
    }

    fn mtime(&self) -> (usize, usize) {
        let year = self.inner.modified().date.year as usize;
        let month = self.inner.modified().date.month as usize;
        let day = self.inner.modified().date.day as usize;
        let hour = self.inner.modified().time.hour as usize;
        let min = self.inner.modified().time.min as usize;
        let sec = self.inner.modified().time.sec as usize;
        let nsec = self.inner.modified().time.millis as usize * 1_000_000;

        let secs = calculate_sec(year, month, day, hour, min, sec);
        (secs, nsec)
    }

    fn ctime(&self) -> (usize, usize) {
        let year = self.inner.created().date.year as usize;
        let month = self.inner.created().date.month as usize;
        let day = self.inner.created().date.day as usize;
        let hour = self.inner.created().time.hour as usize;
        let min = self.inner.created().time.min as usize;
        let sec = self.inner.created().time.sec as usize;
        let nsec = self.inner.created().time.millis as usize * 1_000_000;

        let secs = calculate_sec(year, month, day, hour, min, sec);
        (secs, nsec)
    }
}
// region FatInode end

type FatInodeInner<'a> = fatfs::DirEntry<
    'a,
    super::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;

fn calculate_sec(
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
    sec: usize,
) -> usize {
    const UNIX_YEAR: usize = 1970;
    let mut days = 0;
    for i in UNIX_YEAR..year {
        days += 365;
        if i % 4 == 0 {
            days += 1;
        }
    }
    for i in 1..month {
        days += match i {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if year % 4 == 0 {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }
    days += day;

    days * 24 * 60 * 60 + hour * 60 * 60 + min * 60 + sec
}
