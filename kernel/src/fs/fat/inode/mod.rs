pub use dir::*;
pub use file::*;

use crate::{
    config::ROOT_DIR,
    fs::{Inode, InodeType},
};
use alloc::string::{String, ToString};

mod dir;
mod file;

// region FatInode begin
pub struct FatInode {
    readable: bool,
    writable: bool,
    path: String,
    inner: FatInodeType<'static>,
}

impl FatInode {
    pub fn new_normal(
        path: String,
        inner: FatInodeInnerNormal<'static>,
        readable: bool,
        writable: bool,
    ) -> Self {
        Self {
            readable,
            writable,
            path,
            inner: FatInodeType::Normal(inner),
        }
    }

    pub fn from_root(inner: FatInodeInnerRoot<'static>) -> Self {
        Self {
            readable: true,
            writable: false,
            path: ROOT_DIR.to_string(),
            inner: FatInodeType::Root(inner),
        }
    }
}

unsafe impl Sync for FatInode {}
unsafe impl Send for FatInode {}

impl Inode for FatInode {
    fn name(&self) -> alloc::string::String {
        match self.inner {
            FatInodeType::Root(_) => "/".to_string(),
            FatInodeType::Normal(ref inner) => inner.file_name().to_string(),
        }
    }

    fn size(&self) -> usize {
        match self.inner {
            FatInodeType::Root(_) => 0,
            FatInodeType::Normal(ref inner) => inner.len() as usize,
        }
    }

    fn get_type(&self) -> InodeType {
        match self.inner {
            FatInodeType::Root(_) => InodeType::Dir,
            FatInodeType::Normal(ref inner) => match inner {
                _ if inner.is_dir() => InodeType::Dir,
                _ if inner.is_file() => InodeType::File,
                _ => InodeType::Unknown,
            },
        }
    }

    fn to_file(&self) -> FatFile {
        match &self.inner {
            FatInodeType::Root(_) => panic!("Root is not a file"),
            FatInodeType::Normal(ref inner) => {
                assert!(inner.is_file());
                FatFile::new(
                    self.path.clone(),
                    inner.to_file(),
                    self.readable,
                    self.writable,
                )
            }
        }
    }

    fn to_dir(&self) -> FatDir {
        match &self.inner {
            FatInodeType::Root(ref inner) => FatDir::new(
                self.path.clone(),
                (*inner).clone(),
                self.readable,
                self.writable,
            ),
            FatInodeType::Normal(ref inner) => {
                assert!(inner.is_dir());
                FatDir::new(
                    self.path.clone(),
                    inner.to_dir(),
                    self.readable,
                    self.writable,
                )
            }
        }
    }

    fn atime(&self) -> (usize, usize) {
        match self.inner {
            FatInodeType::Root(_) => (0, 0),
            FatInodeType::Normal(ref inner) => {
                let year = inner.accessed().year as usize;
                let month = inner.accessed().month as usize;
                let day = inner.accessed().day as usize;
                let secs = calculate_sec(year, month, day, 0, 0, 0);
                (secs, 0)
            }
        }
    }

    fn mtime(&self) -> (usize, usize) {
        match self.inner {
            FatInodeType::Root(_) => (0, 0),
            FatInodeType::Normal(ref inner) => {
                let year = inner.modified().date.year as usize;
                let month = inner.modified().date.month as usize;
                let day = inner.modified().date.day as usize;
                let hour = inner.modified().time.hour as usize;
                let min = inner.modified().time.min as usize;
                let sec = inner.modified().time.sec as usize;
                let nsec = inner.modified().time.millis as usize * 1_000_000;
                let secs = calculate_sec(year, month, day, hour, min, sec);
                (secs, nsec)
            }
        }
    }

    fn ctime(&self) -> (usize, usize) {
        match self.inner {
            FatInodeType::Root(_) => (0, 0),
            FatInodeType::Normal(ref inner) => {
                let year = inner.created().date.year as usize;
                let month = inner.created().date.month as usize;
                let day = inner.created().date.day as usize;
                let hour = inner.created().time.hour as usize;
                let min = inner.created().time.min as usize;
                let sec = inner.created().time.sec as usize;
                let nsec = inner.created().time.millis as usize * 1_000_000;
                let secs = calculate_sec(year, month, day, hour, min, sec);
                (secs, nsec)
            }
        }
    }
}
// region FatInode end

type FatInodeInnerNormal<'a> = fatfs::DirEntry<
    'a,
    super::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;

type FatInodeInnerRoot<'a> = fatfs::Dir<
    'a,
    crate::fs::fat::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;

pub enum FatInodeType<'a> {
    Root(FatInodeInnerRoot<'a>),
    Normal(FatInodeInnerNormal<'a>),
}

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
