use lazy_static::lazy_static;

lazy_static! {
    pub static ref SKERNEL: usize = skernel as usize;
    pub static ref STEXT: usize = stext as usize;
    pub static ref ETEXT: usize = etext as usize;
    pub static ref SRODATA: usize = srodata as usize;
    pub static ref ERODATA: usize = erodata as usize;
    pub static ref SDATA: usize = sdata as usize;
    pub static ref EDATA: usize = edata as usize;
    pub static ref SBSS: usize = sbss as usize;
    pub static ref SBSS_NO_STACK: usize = sbss_no_stack as usize;
    pub static ref EBSS: usize = ebss as usize;
    pub static ref EKERNEL: usize = ekernel as usize;
}

extern "C" {
    fn skernel();
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn sbss_no_stack();
    fn ebss();
    fn ekernel();
}
