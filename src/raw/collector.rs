use super::membarrier;
use super::tls::{Thread, ThreadLocal};
use super::utils::CachePadded;

use std::cell::{Cell, UnsafeCell};
use std::ptr;
use std::sync::atomic::{self, AtomicPtr, AtomicUsize, Ordering};
use std::sync::Mutex;

pub struct Collector {
    
    batches: ThreadLocal<CachePadded<UnsafeCell<LocalBatch>>>,

    reservations: ThreadLocal<CachePadded<Reservation>>,

    pub(crate) id: usize,

    pub(crate) batch_size: usize,
}

impl Collector {
    
    pub fn new(threads: usize, batch_size: usize) -> Self { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn reservation(&self, thread: Thread) -> &Reservation { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn enter(&self, reservation: &Reservation) { panic!("STUB: not implemented") }

    #[inline]
    pub fn protect(_order: Ordering) -> Ordering { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn leave(&self, reservation: &Reservation) { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn refresh(&self, reservation: &Reservation) { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn add<T>(
        &self,
        ptr: *mut T,
        reclaim: unsafe fn(*mut T, &crate::Collector),
        thread: Thread,
    ) { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn try_retire_batch(&self, thread: Thread) { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn try_retire(&self, local_batch: *mut LocalBatch) { panic!("STUB: not implemented") }

    #[cold]
    #[inline(never)]
    unsafe fn traverse(&self, mut list: *mut Entry) { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn reclaim_all(&self) { panic!("STUB: not implemented") }

    #[inline]
    unsafe fn free_batch(&self, batch: *mut Batch) { panic!("STUB: not implemented") }
}

impl Drop for Collector {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

#[repr(C)]
pub struct Reservation {
    
    head: AtomicPtr<Entry>,

    pub guards: Cell<u64>,

    pub lock: Mutex<()>,
}

unsafe impl Sync for Reservation {}

impl Default for Reservation {
    fn default() -> Self { panic!("STUB: not implemented") }
}

struct Batch {
    
    entries: Vec<Entry>,

    active: AtomicUsize,
}

impl Batch {
    
    #[inline]
    fn new(capacity: usize) -> Batch { panic!("STUB: not implemented") }
}

struct Entry {
    
    ptr: *mut (),

    reclaim: unsafe fn(*mut (), &crate::Collector),

    state: EntryState,

    batch: *mut Batch,
}

#[repr(C)]
pub union EntryState {
    
    head: *const AtomicPtr<Entry>,

    next: *mut Entry,
}

impl Entry {
    
    pub const INACTIVE: *mut Entry = usize::MAX as _;
}

pub struct LocalBatch {
    batch: *mut Batch,
}

impl Default for LocalBatch {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl LocalBatch {
    
    const DROP: *mut Batch = usize::MAX as _;

    #[inline]
    fn get_or_init(&mut self, capacity: usize) -> *mut Batch { panic!("STUB: not implemented") }

    #[inline]
    unsafe fn free(batch: *mut Batch) { panic!("STUB: not implemented") }
}

unsafe impl Send for LocalBatch {}
