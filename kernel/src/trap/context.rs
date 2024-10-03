use riscv::register::sstatus::{self, Sstatus};

// region TrapContext begin
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

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

    pub fn push_to_kstack(&self) -> *mut Self {
        let cx_size = core::mem::size_of::<Self>();
        let cx_ptr = (self.kernel_sp - cx_size) as *mut Self;
        unsafe {
            cx_ptr.write_volatile(*self);
            cx_ptr.as_mut().unwrap()
        }
    }
}
// region TrapContext end