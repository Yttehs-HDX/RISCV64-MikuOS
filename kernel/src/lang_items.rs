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
            info.message(),
        );
    } else {
        error!("Panicked: {}", info.message(),)
    }
    sbi::sbi_shutdown_failure();
}
