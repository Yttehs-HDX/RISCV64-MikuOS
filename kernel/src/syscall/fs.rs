use crate::sbi;

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

pub fn sys_read(fd: usize, buffer: *mut u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            assert_eq!(len, 1, "sys_read: only support read one byte from stdin");
            let c = sbi::console_getchar();
            unsafe { *buffer = c as u8 };
            1
        }
        _ => {
            panic!("sys_read: fd not supported");
        }
    }
}

pub fn sys_write(fd: usize, buffer: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buffer, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("sys_write: fd not supported");
        }
    }
}
