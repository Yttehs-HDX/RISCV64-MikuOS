use crate::mm::PTEFlags;
use bitflags::bitflags;

// region MapPermission begin
bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

impl MapPermission {
    pub fn as_pteflags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits())
    }
}
// region MapPermission end
