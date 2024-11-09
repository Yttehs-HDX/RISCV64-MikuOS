use riscv::register::sstatus::{self, Sstatus};

// region TrapContext begin
#[repr(C)]
pub struct TrapContext {
    // registers
    pub x: [usize; 32],   // +0 ~ + 31
    pub sstatus: Sstatus, // + 32
    pub sepc: usize,      // +33

    // variables
    pub kernel_satp: usize,  // +34
    pub kernel_sp: usize,    // +35
    pub trap_handler: usize, // +36
}

impl TrapContext {
    pub fn new(
        entry: usize,
        user_sp: usize,
        kernel_sp: usize,
        kernel_satp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(sstatus::SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_sp,
            kernel_satp,
            trap_handler,
        };
        cx.set_sp(user_sp);
        cx
    }
}

impl TrapContext {
    fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn set_a0(&mut self, a0: usize) {
        self.x[10] = a0;
    }
}
// region TrapContext end
