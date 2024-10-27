use riscv::register::sstatus::{self, Sstatus};

// region TrapContext begin
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub kernel_sp: usize,
}

impl TrapContext {
    pub fn new(entry: usize, user_sp: usize, kernel_sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(sstatus::SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_sp,
        };
        cx.set_sp(user_sp);
        cx.push_to_kstack();
        cx
    }

    pub fn get_ptr_from_sp(&self) -> *mut Self {
        let cx_size = core::mem::size_of::<Self>();
        (self.kernel_sp - cx_size) as *mut Self
    }

    fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    fn push_to_kstack(&self) {
        let cx_ptr = self.get_ptr_from_sp();
        unsafe { *cx_ptr = *self };
    }
}
// region TrapContext end
