use bitflags::bitflags;

// region OpenFlags begin
bitflags! {
    #[derive(Clone, Copy)]
    pub struct OpenFlags: u32 {
        const RDONLY = 0; // 0x0
        const WRONLY = 1; // 0x1
        const RDWR = 1 << 1; // 0x2
        const CREATE = 1 << 6; // 0x40
        const TRUNC = 1 << 10; // 0x400
        const DIRECTORY = 1 << 21; // 0x200000
    }
}

impl OpenFlags {
    pub const fn read_write(&self) -> (bool, bool) {
        match self {
            _ if self.contains(Self::RDWR) => (true, true),
            _ if self.contains(Self::WRONLY) => (false, true),
            _ => (true, false),
        }
    }

    pub const fn create(&self) -> bool {
        self.contains(OpenFlags::CREATE)
    }

    pub const fn directory(&self) -> bool {
        self.contains(OpenFlags::DIRECTORY)
    }
}
// region OpenFlags end
