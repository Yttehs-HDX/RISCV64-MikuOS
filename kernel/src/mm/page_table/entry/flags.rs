use bitflags::bitflags;

// region PTEFlags begin
bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

impl PTEFlags {
    pub fn is_valid(&self) -> bool { self.contains(PTEFlags::V) }
    pub fn is_readable(&self) -> bool { self.contains(PTEFlags::R) }
    pub fn is_writable(&self) -> bool { self.contains(PTEFlags::W) }
    pub fn is_executable(&self) -> bool { self.contains(PTEFlags::X) }
    pub fn is_user(&self) -> bool { self.contains(PTEFlags::U) }
    pub fn is_global(&self) -> bool { self.contains(PTEFlags::G) }
    pub fn is_accessed(&self) -> bool { self.contains(PTEFlags::A) }
    pub fn is_dirty(&self) -> bool { self.contains(PTEFlags::D) }
}
// region PTEFlags end