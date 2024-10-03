use lazy_static::lazy_static;
use crate::config::{APP_BASE_ADDR, APP_SIZE_LIMIT};

// Before implementing file system, we use include_bytes! to load the binary of the app
const APP_NUM: usize = 3;
pub const TEST_PRINT: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_print.bin");
pub const TEST_SRET: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_sret.bin");
pub const TEST_PAGE_FAULT: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_page_fault.bin");

pub fn get_app(name: &str) -> Option<&'static App> {
    APPS.iter().find(|app| app.name() == name)
}

lazy_static! {
    static ref APPS: [App; APP_NUM] = [
        App::new(0, "test_print", TEST_PRINT),
        App::new(1, "test_sret", TEST_SRET),
        App::new(2, "test_page_fault", TEST_PAGE_FAULT),
    ];
}

// region App begin
pub struct App {
    no: usize,
    name: &'static str,
    bin: &'static [u8],
}

impl App {
    pub fn new(no: usize, name: &'static str, bin: &'static [u8]) -> Self {
        let app = App { no, name, bin };
        app.load_to_mem();
        app
    }

    pub fn no(&self) -> usize {
        self.no
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn bin(&self) -> &'static [u8] {
        self.bin
    }

    pub fn len(&self) -> usize {
        self.bin.len()
    }

    #[inline(always)]
    pub fn base_addr(&self) -> usize {
        APP_BASE_ADDR + self.no() * APP_SIZE_LIMIT
    }

    fn load_to_mem(&self) {
        unsafe {
            let dst = core::slice::from_raw_parts_mut(
                self.base_addr() as *mut u8,
                self.len()
            );
            dst.copy_from_slice(self.bin());
        }
    }
}
// region App end