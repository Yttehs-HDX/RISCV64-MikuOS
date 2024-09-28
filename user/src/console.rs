use core::fmt::{Arguments, Write};
use crate::write;

const STDOUT: usize = 1;

pub fn print(args: Arguments) {
    Console.write_fmt(args).unwrap();
}

// region Console begin
struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}
// region Console end

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}