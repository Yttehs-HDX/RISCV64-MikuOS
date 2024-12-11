use crate::sbi;
use core::panic::PanicInfo;
use log::error;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panicked at {} {}:{} {}",
            location.file(),
            location.line(),
            location.column(),
            info.message().unwrap(),
        );
    } else {
        error!("Panicked: {}", info.message().unwrap())
    }
    sbi::sbi_shutdown_failure();
}
