use core::cell::RefMut;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use crate::{sync::UPSafeCell, task::ProcessControlBlock};

pub fn get_processor() -> &'static Processor {
    &PROCESSOR
}

lazy_static! {
    static ref PROCESSOR: Processor = Processor::new();
}

// region Processor begin
pub struct Processor {
    inner: UPSafeCell<ProcessorInner>,
}

impl Processor {
    fn new() -> Self {
        Self {
            inner: unsafe { UPSafeCell::new(ProcessorInner { current: None }) },
        }
    }

    fn inner_mut(&self) -> RefMut<ProcessorInner> {
        self.inner.exclusive_access()
    }
}

impl Processor {
    pub fn take_current(&self) -> Option<Arc<ProcessControlBlock>> {
        self.inner_mut().current.take()
    }

    pub fn current(&self) -> Option<Arc<ProcessControlBlock>> {
        self.inner_mut().current.as_ref().map(Arc::clone)
    }
}
// region Processor end

// region ProcessorInner begin
struct ProcessorInner {
    current: Option<Arc<ProcessControlBlock>>,
}
// region ProcessorInner end
