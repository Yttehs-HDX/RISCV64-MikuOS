use log::error;

use fs::*;
use process::*;

mod fs;
mod process;

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_BRK: usize = 214;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_GETCWD: usize = 17;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_MKDIR: usize = 34;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_NANO_SLEEP: usize = 101;

pub fn syscall(id: usize, args: [usize; 6]) -> isize {
    match id {
        SYSCALL_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut u8, args[1]),
        SYSCALL_BRK => sys_brk(args[0] as i32),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_GETPPID => sys_getppid(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8, args[1] as *const u8),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32, args[2]),
        SYSCALL_GETCWD => sys_getcwd(args[0] as *mut u8, args[1]),
        SYSCALL_CHDIR => sys_chdir(args[0] as *const u8),
        SYSCALL_OPEN => sys_open(args[0] as i32, args[1] as *const u8, args[2]),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_MKDIR => sys_mkdir(args[0], args[1] as *const u8, args[2]),
        SYSCALL_PIPE => sys_pipe(args[0] as *mut i32),
        SYSCALL_NANO_SLEEP => sys_nanosleep(args[0] as *const u8, args[1]),
        _ => {
            error!("Unsupported syscall id: {}", id);
            sys_exit(-1);
        }
    }
}

pub fn translate_str<'a>(ptr: *const u8) -> &'a str {
    let path: &str;
    unsafe {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        path = core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len));
    }
    path
}
