// Before implementing file system, we use include_bytes! to load the binary of the app
pub const APP_NUM: usize = 3;
pub const TEST_PRINT: &[u8] = include_bytes!("../../../user/target/riscv64gc-unknown-none-elf/release/test_print.bin");
pub const TEST_SRET: &[u8] = include_bytes!("../../../user/target/riscv64gc-unknown-none-elf/release/test_sret.bin");
pub const TEST_PAGE_FAULT: &[u8] = include_bytes!("../../../user/target/riscv64gc-unknown-none-elf/release/test_page_fault.bin");

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