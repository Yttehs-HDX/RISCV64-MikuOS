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
    pub fn is_readable(&self) -> bool {
        self.contains(Self::R)
    }
    pub fn is_writable(&self) -> bool {
        self.contains(Self::W)
    }
    pub fn is_executable(&self) -> bool {
        self.contains(Self::X)
    }
    pub fn is_user(&self) -> bool {
        self.contains(Self::U)
    }
}
// region MapPermission end
