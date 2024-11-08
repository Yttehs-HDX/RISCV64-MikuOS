use lazy_static::lazy_static;

// Before implementing file system, we use include_bytes! to load the binary of the app
const APP_NUM: usize = 7;

pub fn get_app(name: &str) -> Option<&App> {
    APPS.iter().find(|app| app.name() == name)
}

lazy_static! {
    static ref APPS: [App; APP_NUM] = [
        App::new(
            0,
            "test_print",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/01_test_print")
        ),
        App::new(
            1,
            "test_sret",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/02_test_sret")
        ),
        App::new(
            2,
            "test_page_fault",
            include_bytes!(
                "../../user/target/riscv64gc-unknown-none-elf/release/03_test_page_fault"
            )
        ),
        App::new(
            3,
            "test_yield",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/04_test_yield")
        ),
        App::new(
            4,
            "test_sbrk",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/05_test_sbrk")
        ),
        App::new(
            5,
            "test_read",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/06_test_read")
        ),
        App::new(
            6,
            "user_shell",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/user_shell")
        ),
    ];
}

// region App begin
#[repr(align(4096))]
pub struct App {
    id: usize,
    name: &'static str,
    elf: &'static [u8],
}

impl App {
    pub fn new(id: usize, name: &'static str, elf: &'static [u8]) -> Self {
        App { id, name, elf }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn elf(&self) -> &'static [u8] {
        self.elf
    }
}
// region App end
