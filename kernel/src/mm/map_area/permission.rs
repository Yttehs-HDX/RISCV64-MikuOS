use bitflags::bitflags;
use crate::mm::PTEFlags;

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
    pub fn has_perm(&self, other: MapPermission) -> bool {
        self.contains(other)
    }

    pub fn to_pte_flags(&self) -> PTEFlags {
        PTEFlags::from_bits_retain(self.bits())
    }
}
// region MapPermission end