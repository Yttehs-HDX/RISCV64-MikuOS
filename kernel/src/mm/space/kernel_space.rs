use crate::{
    mm::{MemorySet, MemorySpace, PageTableEntry, VirtPageNum},
    sync::UPSafeCell,
};
use core::cell::RefMut;
use lazy_static::lazy_static;

pub fn get_kernel_space() -> &'static KernelSpace {
    &KERNEL_SPACE
}

lazy_static! {
    static ref KERNEL_SPACE: KernelSpace = KernelSpace::new();
}

// region KernelSpace begin
pub struct KernelSpace {
    inner: UPSafeCell<KernelSpaceInner>,
}

impl MemorySpace for KernelSpace {
    fn activate(&self) {
        self.inner_mut().activate();
    }

    fn get_satp(&self) -> usize {
        self.inner_mut().get_satp()
    }

    fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.inner_mut().translate(vpn)
    }
}

impl KernelSpace {
    fn new() -> Self {
        KernelSpace {
            inner: unsafe { UPSafeCell::new(KernelSpaceInner::new_kernel()) },
        }
    }

    pub fn inner_mut(&self) -> RefMut<KernelSpaceInner> {
        self.inner.exclusive_access()
    }
}
// region KernelSpace end

type KernelSpaceInner = MemorySet;
