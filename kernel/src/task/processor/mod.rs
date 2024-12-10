pub(in crate::task) use initproc::*;

use crate::{
    sync::UPSafeCell,
    task::{__restore_task, __save_task, get_task_manager, ProcessControlBlock},
    timer,
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
                let mut inner = pcb.inner_mut();

                // update tms
                {
                    // stime end
                    let now = timer::get_current_tick();
                    let inc = now - inner.get_stime_base();
                    inner.get_tms_mut().add_stime(inc);
                }

                let task_cx = inner.get_task_cx_mut() as *mut _;
                drop(inner);
                unsafe {
                    __save_task(task_cx);
                }
            }
            if let Some(pcb) = get_task_manager().fetch() {
                let inner = pcb.inner();
                let task_cx = inner.get_task_cx_ref() as *const _;
                drop(inner);
                self.inner_mut().current = Some(pcb);
                unsafe {
                    __restore_task(task_cx);
                }
            }
        }
    }

    pub fn wait_for_child(&self, pid: usize) -> ! {
        loop {
            if let Some(pcb) = self.take_current() {
                let mut inner = pcb.inner_mut();

                // update tms
                {
                    // stime end
                    let now = timer::get_current_tick();
                    let inc = now - inner.get_stime_base();
                    inner.get_tms_mut().add_stime(inc);
                }

                let task_cx = inner.get_task_cx_mut() as *mut _;
                drop(inner);
                get_task_manager().add_to_front(pcb);
                unsafe {
                    __save_task(task_cx);
                }
            }

            if let Some(pcb) = get_task_manager().fetch_by_pid(pid) {
                let inner = pcb.inner();
                let task_cx = inner.get_task_cx_ref() as *const _;
                drop(inner);
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
                let mut inner = pcb.inner_mut();

                // update tms
                {
                    // stime end
                    let now = timer::get_current_tick();
                    let inc = now - inner.get_stime_base();
                    inner.get_tms_mut().add_stime(inc);
                }

                let task_cx = inner.get_task_cx_mut() as *mut _;
                drop(inner);
                get_task_manager().add_to_back(pcb);
                unsafe {
                    __save_task(task_cx);
                }
            }

            if let Some(pcb) = get_task_manager().fetch() {
                let inner = pcb.inner();
                let task_cx = inner.get_task_cx_ref() as *const _;
                drop(inner);
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
