use lazy_static::lazy_static;

// Before implementing file system, we use include_bytes! to load the binary of the app
const APP_NUM: usize = 4;
const TEST_PRINT: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/01_test_print.bin");
const TEST_SRET: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/02_test_sret.bin");
const TEST_PAGE_FAULT: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/03_test_page_fault.bin");
const TEST_YIELD: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/04_test_yield.bin");

pub fn get_app(name: &str) -> Option<&App> {
    APPS.iter().find(|app| app.name() == name)
}

lazy_static! {
    static ref APPS: [App; APP_NUM] = [
        App::new("test_print", TEST_PRINT),
        App::new("test_sret", TEST_SRET),
        App::new("test_page_fault", TEST_PAGE_FAULT),
        App::new("test_yield", TEST_YIELD),
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

    pub fn len(&self) -> usize {
        self.bin.len()
    }
}
// region App end
