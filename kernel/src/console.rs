use core::fmt::{Arguments, Write};
use crate::sbi;

pub fn print(args: Arguments) {
    Console.write_fmt(args).unwrap();
}

// region Console begin
struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            sbi::console_putchar(c as usize);
        }
        Ok(())
    }
}
// region Console end

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}