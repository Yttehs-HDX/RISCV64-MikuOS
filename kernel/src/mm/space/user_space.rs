use crate::{
    mm::{MemorySet, MemorySpace, PageTableEntry, VirtPageNum},
    sync::UPSafeCell,
};
use core::cell::{Ref, RefMut};

// region UserSpace begin
pub struct UserSpace {
    entry: usize,
    base_size: usize,
    inner: UPSafeCell<UserSpaceInner>,
}

impl MemorySpace for UserSpace {
    fn activate(&self) {
        panic!("UserSpace: not support in KenrelSpace");
    }

    fn get_satp(&self) -> usize {
        self.inner_mut().get_satp()
    }

    fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.inner_mut().translate(vpn)
    }
}

impl UserSpace {
    pub fn from_elf(elf_data: &[u8]) -> Self {
        let (space, entry, base_size) = UserSpaceInner::from_elf(elf_data);
        Self {
            entry,
            base_size,
            inner: unsafe { UPSafeCell::new(space) },
        }
    }

    pub fn from_existed(user_space: &Self) -> Self {
        Self {
            entry: 0,
            base_size: 0,
            inner: unsafe { UPSafeCell::new(UserSpaceInner::from_another(&user_space.inner())) },
        }
    }

    pub fn inner(&self) -> Ref<UserSpaceInner> {
        self.inner.shared_access()
    }

    pub fn inner_mut(&self) -> RefMut<UserSpaceInner> {
        self.inner.exclusive_access()
    }
}

impl UserSpace {
    pub fn get_entry(&self) -> usize {
        self.entry
    }

    pub fn get_base_size(&self) -> usize {
        self.base_size
    }
}
// region UserSpace end

type UserSpaceInner = MemorySet;
