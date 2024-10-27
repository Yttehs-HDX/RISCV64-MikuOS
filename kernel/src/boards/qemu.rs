use core::arch::asm;

pub const CLOCK_FREQ: usize = 12500000;

const EXIT_SUCCESS: u32 = 0x5555;
const EXIT_FAILURE_FLAG: u32 = 0x3333;
const EXIT_FAILURE: u32 = encode_exit_code(1);
const EXIT_RESET: u32 = 0x7777;

const fn encode_exit_code(code: u32) -> u32 {
    (code << 16) | EXIT_FAILURE_FLAG
}

const VIRT_TEST: u64 = 0x100000;
const QEMU_EXIT_HANDLE: RISCV64 = RISCV64 { addr: VIRT_TEST };

pub fn qemu_exit(code: u32) -> ! {
    QEMU_EXIT_HANDLE.exit(code)
}

pub fn qemu_exit_success() -> ! {
    QEMU_EXIT_HANDLE.exit_success()
}

pub fn qemu_exit_failure() -> ! {
    QEMU_EXIT_HANDLE.exit_failure()
}

trait QEMUExit {
    fn exit(&self, code: u32) -> !;
    fn exit_success(&self) -> ! {
        self.exit(0);
    }
    fn exit_failure(&self) -> ! {
        self.exit(1);
    }
}

// region RISCV64 begin
struct RISCV64 {
    addr: u64,
}

impl QEMUExit for RISCV64 {
    fn exit(&self, code: u32) -> ! {
        let qemu_exit_code = match code {
            EXIT_SUCCESS | EXIT_RESET => code,
            _ => encode_exit_code(code),
        };

        unsafe {
            asm!(
                "sw {0}, 0({1})",
                in(reg) qemu_exit_code, in(reg) self.addr,
            );

            // loop if exit failed
            loop {
                asm!("wfi", options(nomem, nostack))
            }
        }
    }

    fn exit_success(&self) -> ! {
        self.exit(EXIT_SUCCESS);
    }

    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE);
    }
}
// region RISCV64 end
