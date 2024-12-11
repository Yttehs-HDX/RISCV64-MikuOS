use crate::timer::{self, TimeVal};

pub fn sys_get_time(ts_ptr: *mut u8, _tz: usize) -> isize {
    let ts_ptr = ts_ptr as *mut TimeVal;
    let now = timer::get_current_time();
    unsafe {
        *ts_ptr = now;
    }
    0
}

// region UtsName begin
struct UtsName {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

impl UtsName {
    fn new() -> Self {
        let mut uts_name = Self {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };
        Self::insert_field("MikuOS", &mut uts_name.sysname);
        Self::insert_field("QEMU-VIRT", &mut uts_name.nodename);
        Self::insert_field("0.0.1-alpha", &mut uts_name.release);
        Self::insert_field("2024-06-02 23:44:19", &mut uts_name.version);
        Self::insert_field("riscv64", &mut uts_name.machine);
        Self::insert_field("shettyyttehs.org", &mut uts_name.domainname);
        uts_name
    }

    fn insert_field(field: &str, target: &mut [u8; 65]) {
        let bytes = field.as_bytes();
        let len = bytes.len().min(65);
        target[..len].copy_from_slice(&bytes[..len]);
    }
}
// region UtsName end

pub fn sys_uname(uts_name_ptr: *const u8) -> isize {
    let uts_name_ptr = uts_name_ptr as *mut UtsName;
    unsafe {
        *uts_name_ptr = UtsName::new();
    }
    0
}
