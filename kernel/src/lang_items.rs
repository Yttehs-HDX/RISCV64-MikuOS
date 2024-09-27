use core::panic::PanicInfo;
use crate::{println, sbi};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {} {}:{} {}",
            location.file(),
            location.line(),
            location.column(),
            info.message(),
        );
    } else {
        println!(
            "Panicked: {}",
            info.message(),
        )
    }
    sbi::sbi_shutdown_failure();
}