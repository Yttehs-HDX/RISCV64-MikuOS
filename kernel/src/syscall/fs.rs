const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buffer: *const u8, len: usize) -> isize {
    // match fd {
    //     FD_STDOUT => {
    //         let slice = unsafe { core::slice::from_raw_parts(buffer, len) };
    //         let str = core::str::from_utf8(slice).unwrap();
    //         print!("{}", str);
    //         len as isize
    //     }
    //     _ => {
    //         panic!("sys_write: fd not supported");
    //     }
    // }
    println!("sys_write: fd = {}, buffer = {:?}, len = {}", fd, buffer, len);
    len as isize
}
