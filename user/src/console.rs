use crate::{read, write};
use core::fmt::{Arguments, Write};

const STDOUT: usize = 1;

pub fn get_char() -> u8 {
    let mut buf = [0u8; 1];
    read(&mut buf);
    buf[0]
}

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
    ($($arg:tt)*) => ($crate::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
