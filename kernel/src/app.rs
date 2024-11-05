use lazy_static::lazy_static;

// Before implementing file system, we use include_bytes! to load the binary of the app
const APP_NUM: usize = 4;
const APP_MAX_SIZE: usize = 0x20000;
const TEST_PRINT: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/01_test_print");
const TEST_SRET: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/02_test_sret");
const TEST_PAGE_FAULT: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/03_test_page_fault");
const TEST_YIELD: &[u8] =
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/04_test_yield");

pub fn get_app(name: &str) -> Option<&App> {
    APPS.iter().find(|app| app.name() == name)
}

lazy_static! {
    static ref APPS: [App; APP_NUM] = [
        App::new(0, "test_print", TEST_PRINT),
        App::new(1, "test_sret", TEST_SRET),
        App::new(2, "test_page_fault", TEST_PAGE_FAULT),
        App::new(3, "test_yield", TEST_YIELD),
    ];
}

static mut ALIGNED_SPACES: [AlignedSpace; APP_NUM] = [AlignedSpace {
    data: [0; APP_MAX_SIZE],
}; APP_NUM];

// region App begin
pub struct App {
    id: usize,
    name: &'static str,
    space: &'static AlignedSpace,
}

impl App {
    pub fn new(id: usize, name: &'static str, elf: &'static [u8]) -> Self {
        unsafe {
            ALIGNED_SPACES[id].copy_data(elf);
            Self {
                id,
                name,
                space: &ALIGNED_SPACES[id],
            }
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn elf(&self) -> &'static [u8] {
        &self.space.get_data()
    }
}
// region App end

// region AlignSpace begin
#[repr(align(4096))]
#[derive(Clone, Copy)]
struct AlignedSpace {
    data: [u8; APP_MAX_SIZE],
}

impl AlignedSpace {
    pub fn copy_data(&mut self, data: &[u8]) {
        self.data[..data.len()].copy_from_slice(data);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}
// region AlignSpace end
