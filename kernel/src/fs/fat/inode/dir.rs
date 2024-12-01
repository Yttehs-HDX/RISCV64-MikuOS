use crate::fs::Directory;

// region FatDir begin
pub struct FatDir<'a> {
    inner: FatDirInner<'a>,
}

impl<'a> FatDir<'a> {
    pub const fn new(inner: FatDirInner<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> Directory for FatDir<'a> {}
// region FatDir end

type FatDirInner<'a> = fatfs::Dir<
    'a,
    crate::fs::fat::FatDeviceDriver,
    fatfs::NullTimeProvider,
    fatfs::LossyOemCpConverter,
>;
