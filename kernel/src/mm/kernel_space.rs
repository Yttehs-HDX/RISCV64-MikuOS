use super::MemorySet;
use crate::sync::UPSafeCell;
use core::cell::RefMut;
use lazy_static::lazy_static;

pub fn get_kernel_space<'a>() -> RefMut<'a, KernelSpaceInner> {
    KERNEL_SPACE.inner.exclusive_access()
}

lazy_static! {
    static ref KERNEL_SPACE: KernelSpace = KernelSpace::new();
}

// region KernelSpace begin
pub struct KernelSpace {
    inner: UPSafeCell<KernelSpaceInner>,
}

impl KernelSpace {
    fn new() -> Self {
        KernelSpace {
            inner: unsafe { UPSafeCell::new(KernelSpaceInner::new_kernel()) },
        }
    }
}
// region KernelSpace end

type KernelSpaceInner = MemorySet;
