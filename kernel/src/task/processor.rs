use crate::{
    sync::UPSafeCell,
    task::{__restore_task, __save_task, get_task_manager, ProcessControlBlock},
};
use alloc::sync::Arc;
use core::cell::{Ref, RefMut};
use lazy_static::lazy_static;

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

    fn inner(&self) -> Ref<ProcessorInner> {
        self.inner.shared_access()
    }

    fn inner_mut(&self) -> RefMut<ProcessorInner> {
        self.inner.exclusive_access()
    }
}

impl Processor {
    fn take_current(&self) -> Option<Arc<ProcessControlBlock>> {
        self.inner_mut().current.take()
    }

    pub fn current(&self) -> Option<Arc<ProcessControlBlock>> {
        self.inner().current.as_ref().map(Arc::clone)
    }

    pub fn run_tasks(&self) -> ! {
        loop {
            if let Some(pcb) = self.take_current() {
                let task_cx = pcb.inner_mut().get_task_cx_mut() as *mut _;
                unsafe {
                    __save_task(task_cx);
                }
            }
            if let Some(pcb) = get_task_manager().fetch() {
                let task_cx = pcb.inner_mut().get_task_cx_ref() as *const _;
                self.inner_mut().current = Some(pcb);
                unsafe {
                    __restore_task(task_cx);
                }
            }
        }
    }

    pub fn schedule(&self) -> ! {
        loop {
            if let Some(pcb) = self.take_current() {
                let task_cx = pcb.inner_mut().get_task_cx_mut() as *mut _;
                get_task_manager().add(pcb);
                unsafe {
                    __save_task(task_cx);
                }
            }
            if let Some(pcb) = get_task_manager().fetch() {
                let task_cx = pcb.inner().get_task_cx_ref() as *const _;
                self.inner_mut().current = Some(pcb);
                unsafe {
                    __restore_task(task_cx);
                }
            }
        }
    }
}
// region Processor end

// region ProcessorInner begin
struct ProcessorInner {
    current: Option<Arc<ProcessControlBlock>>,
}
// region ProcessorInner end
