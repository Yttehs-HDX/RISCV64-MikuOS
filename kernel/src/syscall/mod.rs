use fs::*;
use mm::*;
use process::*;
use system::*;

use log::error;

mod fs;
mod mm;
mod process;
mod system;

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_BRK: usize = 214;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_GETCWD: usize = 17;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_MKDIR: usize = 34;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_NANOSLEEP: usize = 101;
const SYSCALL_MOUNT: usize = 40;
const SYSCALL_UMOUNT: usize = 39;
const SYSCALL_UNLINK: usize = 35;
const SYSCALL_DUP: usize = 23;
const SYSCALL_DUP3: usize = 24;
const SYSCALL_FSTAT: usize = 80;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GETDENTS: usize = 61;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_MUNMAP: usize = 215;

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
        SYSCALL_CLONE => sys_clone(args[0], args[1]),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8, args[1] as *const u8),
        SYSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32, args[2]),
        SYSCALL_GETCWD => sys_getcwd(args[0] as *mut u8, args[1]),
        SYSCALL_CHDIR => sys_chdir(args[0] as *const u8),
        SYSCALL_OPEN => sys_open(args[0] as i32, args[1] as *const u8, args[2]),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_MKDIR => sys_mkdir(args[0], args[1] as *const u8, args[2]),
        SYSCALL_PIPE => sys_pipe(args[0] as *mut i32),
        SYSCALL_NANOSLEEP => sys_nanosleep(args[0] as *const u8, args[1]),
        SYSCALL_MOUNT => sys_mount(
            args[0] as *const u8,
            args[1] as *const u8,
            args[2] as *const u8,
        ),
        SYSCALL_UMOUNT => sys_umount(args[0] as *const u8),
        SYSCALL_UNLINK => sys_unlink(args[0], args[1] as *const u8, args[2]),
        SYSCALL_DUP => sys_dup(args[0]),
        SYSCALL_DUP3 => sys_dup2(args[0], args[1]),
        SYSCALL_FSTAT => sys_fstat(args[0], args[1] as *mut u8),
        SYSCALL_UNAME => sys_uname(args[0] as *mut u8),
        SYSCALL_GETDENTS => sys_getdents(args[0], args[1] as *const u8, args[2]),
        SYSCALL_TIMES => sys_times(args[0] as *const u8),
        SYSCALL_MMAP => sys_mmap(args[0], args[1], args[2], args[3], args[4], args[5]),
        SYSCALL_MUNMAP => sys_munmap(args[0]),
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
