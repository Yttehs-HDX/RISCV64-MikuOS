// Before implementing file system, we use include_bytes! to load the binary of the app

use lazy_static::lazy_static;

const APP_NUM: usize = 6;

pub fn get_app(name: &str) -> Option<&App> {
    APPS.iter().find(|app| app.name() == name)
}

lazy_static! {
    static ref APPS: [App; APP_NUM] = [
        App::new(
            "initproc",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/initproc"),
        ),
        App::new(
            "user_shell",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/user_shell"),
        ),
        App::new(
            "test_print",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/01_test_print"),
        ),
        // App::new(
        //     "test_sret",
        //     include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/02_test_sret"),
        // ),
        // App::new(
        //     "test_page_fault",
        //     include_bytes!(
        //         "../../user/target/riscv64gc-unknown-none-elf/release/03_test_page_fault"
        //     ),
        // ),
        App::new(
            "test_yield",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/04_test_yield"),
        ),
        App::new(
            "test_sbrk",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/05_test_sbrk"),
        ),
        // App::new(
        //     "test_read",
        //     include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/06_test_read"),
        // ),
        App::new(
            "test_fork",
            include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/07_test_fork"),
        )
    ];
}

// region App begin
#[repr(align(4096))]
pub struct App {
    name: &'static str,
    elf: &'static [u8],
}

impl App {
    pub const fn new(name: &'static str, elf: &'static [u8]) -> Self {
        App { name, elf }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn elf(&self) -> &'static [u8] {
        self.elf
    }
}
// region App end
