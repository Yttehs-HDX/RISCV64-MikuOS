pub(in crate::task) use initproc::*;

use crate::{
    sync::UPSafeCell,
    task::{__restore_task, __save_task, get_task_manager, ProcessControlBlock},
};
use alloc::sync::Arc;
use core::cell::{Ref, RefMut};
use lazy_static::lazy_static;

mod initproc;

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

    pub fn current(&self) -> Arc<ProcessControlBlock> {
        self.inner().current.as_ref().map(Arc::clone).unwrap()
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

    pub fn schedule(&self) {
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

    pub fn exit_current(&self, exit_code: i32) -> ! {
        let pcb = self.take_current().unwrap();
        pcb.set_exit_code(exit_code);
        pcb.drop_user_space();

        // if initproc exits
        #[cfg(not(feature = "test"))]
        if pcb.get_pid() == 1 {
            crate::os_end();
        }

        #[cfg(feature = "test")]
        if get_task_manager().is_empty() {
            crate::os_end();
        }

        // move children to initproc
        {
            for child in pcb.inner_mut().get_children_ref().iter() {
                child.set_parent(Arc::downgrade(get_initproc()));
                get_initproc()
                    .inner_mut()
                    .get_children_mut()
                    .push(child.clone());
            }
        }
        pcb.inner_mut().get_children_mut().clear();

        // drop pcb manually to release resources
        drop(pcb);

        self.run_tasks();
    }
}
// region Processor end

// region ProcessorInner begin
struct ProcessorInner {
    current: Option<Arc<ProcessControlBlock>>,
}
// region ProcessorInner end
