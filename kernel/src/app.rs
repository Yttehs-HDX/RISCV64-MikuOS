use lazy_static::lazy_static;

// Before implementing file system, we use include_bytes! to load the binary of the app
pub const APP_NUM: usize = 3;
pub const TEST_PRINT: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_print.bin");
pub const TEST_SRET: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_sret.bin");
pub const TEST_PAGE_FAULT: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_page_fault.bin");

pub fn get_app(name: &str) -> Option<&'static App> {
    APPS.iter().find(|app| app.name() == name)
}

lazy_static! {
    static ref APPS: [App; APP_NUM] = [
        App::new("test_print", TEST_PRINT),
        App::new("test_sret", TEST_SRET),
        App::new("test_page_fault", TEST_PAGE_FAULT),
    ];
}

// region App begin
pub struct App {
    name: &'static str,
    bin: &'static [u8],
}

impl App {
    pub fn new(name: &'static str, bin: &'static [u8]) -> Self {
        Self { name, bin }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn bin(&self) -> &'static [u8] {
        self.bin
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.bin.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.bin.len()
    }
}
// region App end