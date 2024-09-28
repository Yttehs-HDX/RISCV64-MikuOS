use core::arch::asm;
use lazy_static::lazy_static;
use log::{info, trace};
use crate::sync::UPSafeCell;

// Before implementing file system, we use include_bytes! to load the binary of the app
const APP_BASE_ADDR: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;
const APP_NUM: usize = 1;
const TEST_PRINT: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_print.bin");

pub fn print_app_info() {
    info!("Total app count: {}", APP_NUM);
    APP_MANAGER.exclusive_access().apps.iter().enumerate().for_each(|(i, app)| {
        trace!("App[{}]: {}, start: {:p}, len: 0x{:x}", i, app.name, app.as_ptr(), app.len());
    });
}

lazy_static! {
    pub static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        let app_arr = [
            App::new("test_print", TEST_PRINT),
        ];
        UPSafeCell::new(AppManager { apps: app_arr })
    };
}

// region AppManager begin
pub struct AppManager {
    apps: [App; APP_NUM],
}

impl AppManager {
    pub unsafe fn load_app(&self, app_id: usize) {
        if app_id >= APP_NUM {
            info!("All apps completed");
            return;
        }

        info!("Loading app {}", app_id);
        let app = &self.apps[app_id];
        // clean instruction cache
        asm!("fence.i");
        // clear app space
        core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = app.bin;
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, app.len());
        app_dst.copy_from_slice(app_src);
    }
}
// region AppManager end

// region App begin
struct App {
    name: &'static str,
    bin: &'static [u8],
}

impl App {
    pub fn new(name: &'static str, bin: &'static [u8]) -> Self {
        Self { name, bin }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.bin.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.bin.len()
    }
}
// region App end