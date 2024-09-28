use core::cell::{RefCell, RefMut};

// region UPSafeCell begin
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Send for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn exclusive_access<R>(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}
// region UPSafeCell end