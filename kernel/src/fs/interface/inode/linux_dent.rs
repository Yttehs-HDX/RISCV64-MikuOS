const ENTRY_NAME_MAX_LEN: usize = 256;

// region LinuxDirent64 begin
#[repr(C)]
pub struct LinuxDirent64 {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [u8; ENTRY_NAME_MAX_LEN],
}

impl LinuxDirent64 {
    pub fn new(name: &str) -> Self {
        let mut dirent = Self {
            d_ino: 0,
            d_off: 0,
            d_reclen: 0,
            d_type: 0,
            d_name: [0; ENTRY_NAME_MAX_LEN],
        };
        {
            let bytes = name.as_bytes();
            let len = bytes.len().min(ENTRY_NAME_MAX_LEN);
            dirent.d_name[..len].copy_from_slice(&bytes[..len]);
        }
        dirent
    }
}
// region LinuxDirent64 end
