use riscv::register::sstatus::{self, Sstatus};

// region TrapContext begin
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn init_app_cx(entry: usize, sp: usize, trap_handler: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(sstatus::SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}
// region TrapContext end