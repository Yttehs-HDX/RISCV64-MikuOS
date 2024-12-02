use crate::{fs::File, sbi};

// region Stdin begin
pub struct Stdin;

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        false
    }

    fn read(&self, buf: &mut [u8]) -> usize {
        assert_eq!(buf.len(), 1, "Stdin: only support length 1 read");
        let c = sbi::console_getchar();
        buf[0] = c as u8;
        buf.len()
    }

    fn write(&self, _buf: &[u8]) -> usize {
        panic!("Stdin: write is not supported");
    }
}
// region Stdin end

// region Stdout begin
pub struct Stdout;

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }

    fn read(&self, _buf: &mut [u8]) -> usize {
        panic!("Stdout: read is not supported");
    }

    fn write(&self, buf: &[u8]) -> usize {
        let str = core::str::from_utf8(buf).unwrap();
        print!("{}", str);
        buf.len()
    }
}
// region Stdout end

// region Stderr begin
pub struct Stderr;

impl File for Stderr {
    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        false
    }

    fn read(&self, _buf: &mut [u8]) -> usize {
        panic!("Stdout: read is not supported");
    }

    fn write(&self, buf: &[u8]) -> usize {
        let str = core::str::from_utf8(buf).unwrap();
        print!("{}", str);
        buf.len()
    }
}
// region Stderr end
