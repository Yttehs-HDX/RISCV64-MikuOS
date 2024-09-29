use core::arch::asm;
use app::*;
use lazy_static::lazy_static;
use log::{info, trace};
use stack::{KERNEL_STACK, USER_STACK};
use crate::{config::{APP_BASE_ADDR, APP_SIZE_LIMIT}, sbi, sync::UPSafeCell, trap::{self, TrapContext}};

mod app;
mod stack;

pub fn init_batch() -> ! {
    print_app_info();
    run_app(0);
}

pub fn exit_handler() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    app_manager.current_app += 1;
    let current_app = app_manager.current_app;
    if current_app >= APP_NUM {
        info!("All apps completed");
        sbi::sbi_shutdown_success();
    }

    drop(app_manager);
    run_app(current_app);
}

fn run_app(id: usize) -> ! {
    let app_manager = APP_MANAGER.exclusive_access();
    unsafe { app_manager.load_app(id) };
    drop(app_manager);
    let trap_cx = TrapContext::init_app_cx(APP_BASE_ADDR, USER_STACK.get_sp());
    let cx_ptr = KERNEL_STACK.push_cx(trap_cx);
    unsafe { trap::__restore_trap(cx_ptr as *const _ as usize) };
    unreachable!();
}

fn print_app_info() {
    info!("Total app count: {}", APP_NUM);
    APP_MANAGER.exclusive_access().apps.iter().enumerate().for_each(|(i, app)| {
        trace!("App[{}]: {}, size: 0x{:x}", i, app.name(), app.len());
    });
}

lazy_static! {
    pub static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        let app_arr = [
            App::new("test_print", TEST_PRINT),
        ];
        UPSafeCell::new(AppManager { current_app: 0, apps: app_arr })
    };
}

// region AppManager begin
pub struct AppManager {
    current_app: usize,
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
        let app_src = app.bin();
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDR as *mut u8, app.len());
        app_dst.copy_from_slice(app_src);
    }
}
// region AppManager end