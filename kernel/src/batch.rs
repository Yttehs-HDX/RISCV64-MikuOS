use core::arch::asm;
use lazy_static::lazy_static;
use log::{info, trace};
use crate::{config::{APP_BASE_ADDR, APP_SIZE_LIMIT, KERNEL_STACK_SIZE, USER_STACK_SIZE}, sbi, sync::UPSafeCell, trap::{self, TrapContext}};

// Before implementing file system, we use include_bytes! to load the binary of the app
const APP_NUM: usize = 1;
const TEST_PRINT: &[u8] = include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/test_print.bin");

pub fn init_batch() {
    print_app_info();
}

pub fn run_next_app() -> ! {
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

pub fn run_app(id: usize) -> ! {
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
        trace!("App[{}]: {}, start: {:p}, len: 0x{:x}", i, app.name, app.as_ptr(), app.len());
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

static KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);
static USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

// region KernelStack begin
#[repr(align(4096))]
struct KernelStack([u8; KERNEL_STACK_SIZE]);

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.0.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    fn push_cx(&self, cx: TrapContext) -> *mut TrapContext {
        let cx_size = core::mem::size_of::<TrapContext>();
        let cx_ptr = (KERNEL_STACK.get_sp() - cx_size) as *mut TrapContext;
        unsafe {
            cx_ptr.write(cx);
            cx_ptr.as_mut().unwrap()
        }
    }
}
// region KernelStack end

// region UserStack begin
#[repr(align(4096))]
struct UserStack([u8; USER_STACK_SIZE]);

impl UserStack {
    fn get_sp(&self) -> usize {
        self.0.as_ptr() as usize + USER_STACK_SIZE
    }
}
// region UserStack end

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