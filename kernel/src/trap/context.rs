use riscv::register::sstatus::{self, Sstatus};

// region TrapContext begin
#[repr(C)]
pub struct TrapContext {
    // registers
    x: [usize; 32],   // +0 ~ + 31
    sstatus: Sstatus, // + 32
    sepc: usize,      // +33

    // variables
    kernel_sp: usize,    // +34
    trap_handler: usize, // +35
}

impl TrapContext {
    pub fn new(entry: usize, user_sp: usize, kernel_sp: usize, trap_handler: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(sstatus::SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_sp,
            trap_handler,
        };
        cx.set_sp(user_sp);
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
