use crate::{
    config::{KERNEL_STACK_SP, ROOT_DIR, TRAP_CX_PTR},
    fs::{self, File, Inode, Stderr, Stdin, Stdout},
    mm::{
        self, MapArea, MapPermission, MapType, MemorySpace, PhysPageNum, PpnOffset, UserSpace,
        VirtAddr,
    },
    sync::UPSafeCell,
    task::{alloc_pid_handle, PidHandle, TaskContext, Tms},
    trap::TrapContext,
};
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::{Arc, Weak},
    vec::Vec,
};
use core::cell::{Ref, RefMut};

// region ProcessControlBlock begin
pub struct ProcessControlBlock {
    pid: PidHandle,
    #[allow(unused)]
    inner: UPSafeCell<ProcessControlBlockInner>,
}

impl ProcessControlBlock {
    pub fn new(elf_data: &[u8]) -> Self {
        let pid = alloc_pid_handle();
        let user_space = UserSpace::from_elf(elf_data);
        let trap_cx_ppn = user_space
            .inner_mut()
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn()
            .low_to_high();
        *trap_cx_ppn.as_mut() = TrapContext::new(user_space.get_entry());
        let task_cx = TaskContext::empty();
        let cwd = ROOT_DIR.to_string();
        let mut fd_table: BTreeMap<usize, Arc<dyn File + Send + Sync>> = BTreeMap::new();
        fd_table.insert(0, Arc::new(Stdin));
        fd_table.insert(1, Arc::new(Stdout));
        fd_table.insert(2, Arc::new(Stderr));

        Self {
            pid,
            inner: unsafe {
                UPSafeCell::new(ProcessControlBlockInner::new(
                    trap_cx_ppn,
                    task_cx,
                    user_space,
                    cwd,
                    fd_table,
                ))
            },
        }
    }

    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        let pid = alloc_pid_handle();
        let user_space = UserSpace::from_existed(self.inner().get_user_space());
        let trap_cx_ppn = user_space
            .inner_mut()
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn()
            .low_to_high();
        let task_cx = TaskContext::empty();
        let cwd = self.inner().get_cwd().clone();
        let mut fd_table: BTreeMap<usize, Arc<dyn File + Send + Sync>> = BTreeMap::new();
        self.inner().fd_table.iter().for_each(|(&no, fd)| {
            fd_table.insert(no, fd.clone());
        });

        let pcb = Arc::new(Self {
            pid,
            inner: unsafe {
                UPSafeCell::new(ProcessControlBlockInner::new(
                    trap_cx_ppn,
                    task_cx,
                    user_space,
                    cwd,
                    fd_table,
                ))
            },
        });
        pcb.get_trap_cx_mut().set_kernel_sp(KERNEL_STACK_SP);

        // Set parent
        pcb.set_parent(Arc::downgrade(self));

        // Add to parent's children
        self.inner_mut().children.push(pcb.clone());

        pcb
    }

    pub fn inner(&self) -> Ref<ProcessControlBlockInner> {
        self.inner.shared_access()
    }

    pub fn inner_mut(&self) -> RefMut<ProcessControlBlockInner> {
        self.inner.exclusive_access()
    }
}

impl ProcessControlBlock {
    pub fn get_pid(&self) -> usize {
        self.pid.0
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let user_space = UserSpace::from_elf(elf_data);
        let trap_cx_ppn = user_space
            .inner_mut()
            .translate(VirtAddr(TRAP_CX_PTR).to_vpn())
            .unwrap()
            .ppn()
            .low_to_high();
        *trap_cx_ppn.as_mut() = TrapContext::new(user_space.get_entry());

        // update program brk, user space and trap context
        self.inner_mut().program_brk = user_space.get_base_size();
        self.drop_user_space();
        self.inner_mut().user_space = Some(user_space);
        self.inner_mut().trap_cx_ppn = trap_cx_ppn;
    }
}

impl ProcessControlBlock {
    pub fn get_ppid(&self) -> usize {
        self.inner()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .get_pid()
    }

    pub fn set_parent(&self, parent: Weak<ProcessControlBlock>) {
        self.inner_mut().parent = Some(parent);
    }

    pub fn set_exit_code(&self, exit_code: i32) {
        self.inner_mut().exit_code = exit_code;
    }

    pub fn get_exit_code(&self) -> i32 {
        self.inner().exit_code
    }

    pub fn drop_user_space(&self) {
        mm::switch_to_kernel_space();
        self.inner_mut().user_space = None;
    }

    pub fn is_zombie(&self) -> bool {
        self.inner().user_space.is_none()
    }

    pub fn get_satp(&self) -> usize {
        self.inner().get_user_space().get_satp()
    }

    pub fn get_trap_cx_mut(&self) -> &'static mut TrapContext {
        self.inner().trap_cx_ppn.as_mut()
    }

    pub fn set_break(&self, increase: i32) -> Option<usize> {
        let base_size = self.inner().get_user_space().get_base_size();
        let old_brk = self.inner().program_brk;
        let new_brk = (old_brk as i32 + increase) as usize;
        if new_brk < base_size {
            return None;
        }

        self.inner()
            .get_user_space()
            .inner_mut()
            .change_area_end(VirtAddr(base_size), VirtAddr(new_brk));
        self.inner_mut().program_brk = new_brk;
        Some(old_brk)
    }
}
// region ProcessControlBlock end

// region ProcessControlBlockInner begin
pub struct ProcessControlBlockInner {
    trap_cx_ppn: PhysPageNum,
    task_cx: TaskContext,
    user_space: Option<UserSpace>,
    program_brk: usize,

    parent: Option<Weak<ProcessControlBlock>>,
    children: Vec<Arc<ProcessControlBlock>>,
    exit_code: i32,

    cwd: String,
    fd_table: BTreeMap<usize, Arc<dyn File + Send + Sync>>,

    stime_base: usize,
    utime_base: usize,
    tms: Tms,

    mmap_base: usize,
    mmap_pair: Vec<(usize, usize)>,
}

impl ProcessControlBlockInner {
    fn new(
        trap_cx_ppn: PhysPageNum,
        task_cx: TaskContext,
        user_space: UserSpace,
        cwd: String,
        fd_table: BTreeMap<usize, Arc<dyn File + Send + Sync>>,
    ) -> Self {
        let program_brk = user_space.get_base_size();
        Self {
            trap_cx_ppn,
            task_cx,
            user_space: Some(user_space),
            program_brk,
            parent: None,
            children: Vec::new(),
            exit_code: 0,
            cwd,
            fd_table,
            stime_base: 0,
            utime_base: 0,
            tms: Tms::empty(),
            mmap_base: 0xffff_ffff_c020_0000,
            mmap_pair: Vec::new(),
        }
    }

    fn get_user_space(&self) -> &UserSpace {
        self.user_space.as_ref().unwrap()
    }

    pub fn get_task_cx_ref(&self) -> &TaskContext {
        &self.task_cx
    }

    pub fn get_task_cx_mut(&mut self) -> &mut TaskContext {
        &mut self.task_cx
    }

    pub fn get_children_ref(&self) -> &Vec<Arc<ProcessControlBlock>> {
        &self.children
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<Arc<ProcessControlBlock>> {
        &mut self.children
    }

    pub fn set_cwd(&mut self, cwd: String) {
        self.cwd = cwd;
    }

    pub fn get_cwd(&self) -> String {
        self.cwd.clone()
    }
}

impl ProcessControlBlockInner {
    pub fn get_stime_base(&self) -> usize {
        self.stime_base
    }

    pub fn set_stime_base(&mut self, stime_base: usize) {
        self.stime_base = stime_base;
    }

    pub fn get_utime_base(&self) -> usize {
        self.utime_base
    }

    pub fn set_utime_base(&mut self, utime_base: usize) {
        self.utime_base = utime_base;
    }

    pub fn get_tms_ref(&self) -> &Tms {
        &self.tms
    }

    pub fn get_tms_mut(&mut self) -> &mut Tms {
        &mut self.tms
    }
}

impl ProcessControlBlockInner {
    pub fn alloc_mmap(&mut self, fd: usize) -> usize {
        let file = self.find_fd(fd).unwrap();
        let inode = fs::open_inode(&file.path()).unwrap();
        let size = inode.size();
        let file = inode.to_file();
        let mut buf: Vec<u8> = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        file.read(&mut buf);

        let end_va = VirtAddr(self.mmap_base);
        let start_va = VirtAddr(self.mmap_base - size).to_vpn_floor().to_va();
        self.mmap_base = start_va.0;

        self.user_space
            .as_mut()
            .unwrap()
            .inner_mut()
            .insert_area_with_data(
                MapArea::new(
                    start_va,
                    end_va,
                    MapType::Framed,
                    MapPermission::U | MapPermission::R | MapPermission::W,
                ),
                &buf,
            );
        self.mmap_pair.push((fd, start_va.0));

        start_va.0
    }

    pub fn dealloc_mmap(&mut self, start: usize) {
        self.user_space
            .as_mut()
            .unwrap()
            .inner_mut()
            .remove_area(start);
        self.mmap_pair.retain(|&(_, start_va)| start_va != start);
    }
}

impl ProcessControlBlockInner {
    pub fn alloc_fd(&mut self, file: Arc<dyn File + Send + Sync>) -> usize {
        let mut fd = 0;
        while self.fd_table.contains_key(&fd) {
            fd += 1;
        }
        self.fd_table.insert(fd, file);
        fd
    }

    pub fn insert_fd(&mut self, fd: usize, file: Arc<dyn File + Send + Sync>) {
        self.fd_table.insert(fd, file);
    }

    pub fn find_fd(&self, fd: usize) -> Option<Arc<dyn File + Send + Sync>> {
        self.fd_table.get(&fd).map(|fd| fd.clone())
    }

    pub fn take_fd(&mut self, fd: usize) -> Option<Arc<dyn File + Send + Sync>> {
        self.fd_table.remove(&fd)
    }
}
// region ProcessControlBlockInner end
