// region Tms begin
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tms {
    tms_utime: usize,
    tms_stime: usize,
    tms_cutime: usize,
    tms_cstime: usize,
}

impl Tms {
    pub fn empty() -> Self {
        Tms {
            tms_utime: 0,
            tms_stime: 0,
            tms_cutime: 0,
            tms_cstime: 0,
        }
    }

    pub fn get_utime(&self) -> usize {
        self.tms_utime
    }

    pub fn add_utime(&mut self, inc: usize) {
        self.tms_utime += inc;
    }

    pub fn get_stime(&self) -> usize {
        self.tms_stime
    }

    pub fn add_stime(&mut self, inc: usize) {
        self.tms_stime += inc;
    }

    pub fn get_cutime(&self) -> usize {
        self.tms_cutime
    }

    pub fn add_cutime(&mut self, inc: usize) {
        self.tms_cutime += inc;
    }

    pub fn get_cstime(&self) -> usize {
        self.tms_cstime
    }

    pub fn add_cstime(&mut self, inc: usize) {
        self.tms_cstime += inc;
    }
}
// region Tms end
