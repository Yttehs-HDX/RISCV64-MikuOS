use crate::config::{KERNEL_STACK_SP, USER_STACK_SP};
use riscv::register::sstatus::{self, Sstatus, SPP};

// region TrapContext begin
#[repr(C)]
pub struct TrapContext {
    // registers
    x: [usize; 32],   // +0 ~ + 31
    sstatus: Sstatus, // + 32
    sepc: usize,      // +33

    // variables
    kernel_sp: usize, // +34
}

impl TrapContext {
    pub fn new(entry: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        sstatus.clear_sum();

        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_sp: KERNEL_STACK_SP,
        };
        cx.set_sp(USER_STACK_SP);
        cx
    }
}

impl TrapContext {
    pub fn get_x(&self, no: usize) -> usize {
        self.x[no]
    }

    pub fn get_sepc(&self) -> usize {
        self.sepc
    }

    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn set_a0(&mut self, a0: usize) {
        self.x[10] = a0;
    }

    pub fn move_to_next_ins(&mut self) {
        self.sepc += 4;
    }

    pub fn set_kernel_sp(&mut self, sp: usize) {
        self.kernel_sp = sp;
    }
}
// region TrapContext end
