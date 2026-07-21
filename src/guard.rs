use std::fmt;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::raw::{self, Reservation, Thread};
use crate::Collector;

pub trait Guard {
    
    fn refresh(&mut self);

    fn flush(&self);

    fn collector(&self) -> &Collector;

    fn thread_id(&self) -> usize;

    fn protect<T>(&self, ptr: &AtomicPtr<T>, order: Ordering) -> *mut T { panic!("STUB: not implemented") }

    fn swap<T>(&self, ptr: &AtomicPtr<T>, value: *mut T, order: Ordering) -> *mut T { panic!("STUB: not implemented") }

    fn compare_exchange<T>(
        &self,
        ptr: &AtomicPtr<T>,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> { panic!("STUB: not implemented") }

    fn compare_exchange_weak<T>(
        &self,
        ptr: &AtomicPtr<T>,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> { panic!("STUB: not implemented") }

    unsafe fn defer_retire<T>(&self, ptr: *mut T, reclaim: unsafe fn(*mut T, &Collector));
}

pub struct LocalGuard<'a> {
    
    collector: &'a Collector,

    thread: Thread,

    reservation: *const Reservation,

    _unsend: PhantomData<*mut ()>,
}

impl LocalGuard<'_> {
    #[inline]
    pub(crate) fn enter(collector: &Collector) -> LocalGuard<'_> { panic!("STUB: not implemented") }
}

impl Guard for LocalGuard<'_> {
    
    #[inline]
    fn refresh(&mut self) { panic!("STUB: not implemented") }

    #[inline]
    fn flush(&self) { panic!("STUB: not implemented") }

    #[inline]
    fn collector(&self) -> &Collector { panic!("STUB: not implemented") }

    #[inline]
    fn thread_id(&self) -> usize { panic!("STUB: not implemented") }

    #[inline]
    unsafe fn defer_retire<T>(&self, ptr: *mut T, reclaim: unsafe fn(*mut T, &Collector)) { panic!("STUB: not implemented") }
}

impl Drop for LocalGuard<'_> {
    #[inline]
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

impl fmt::Debug for LocalGuard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct OwnedGuard<'a> {
    
    collector: &'a Collector,

    thread: Thread,

    reservation: *const Reservation,
}

unsafe impl Sync for OwnedGuard<'_> {}

unsafe impl Send for OwnedGuard<'_> {}

impl OwnedGuard<'_> {
    #[inline]
    pub(crate) fn enter(collector: &Collector) -> OwnedGuard<'_> { panic!("STUB: not implemented") }
}

impl Guard for OwnedGuard<'_> {
    
    #[inline]
    fn refresh(&mut self) { panic!("STUB: not implemented") }

    #[inline]
    fn flush(&self) { panic!("STUB: not implemented") }

    #[inline]
    fn collector(&self) -> &Collector { panic!("STUB: not implemented") }

    #[inline]
    fn thread_id(&self) -> usize { panic!("STUB: not implemented") }

    #[inline]
    unsafe fn defer_retire<T>(&self, ptr: *mut T, reclaim: unsafe fn(*mut T, &Collector)) { panic!("STUB: not implemented") }
}

impl Drop for OwnedGuard<'_> {
    #[inline]
    fn drop(&mut self) { panic!("STUB: not implemented") }
}
