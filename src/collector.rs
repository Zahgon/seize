use crate::raw::{self, membarrier, Thread};
use crate::{LocalGuard, OwnedGuard};

use std::fmt;
use std::sync::OnceLock;

#[repr(transparent)]
pub struct Collector {
    
    pub(crate) raw: raw::Collector,
}

impl Default for Collector {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl Collector {
    
    const DEFAULT_BATCH_SIZE: usize = 32;

    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn batch_size(mut self, batch_size: usize) -> Self { panic!("STUB: not implemented") }

    #[inline]
    pub fn enter(&self) -> LocalGuard<'_> { panic!("STUB: not implemented") }

    #[inline]
    pub fn enter_owned(&self) -> OwnedGuard<'_> { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn retire<T>(&self, ptr: *mut T, reclaim: unsafe fn(*mut T, &Collector)) { panic!("STUB: not implemented") }

    pub unsafe fn reclaim_all(&self) { panic!("STUB: not implemented") }

    pub(crate) fn from_raw(raw: &raw::Collector) -> &Collector { panic!("STUB: not implemented") }
}

impl Eq for Collector {}

impl PartialEq for Collector {
    
    #[inline]
    fn eq(&self, other: &Self) -> bool { panic!("STUB: not implemented") }
}

impl fmt::Debug for Collector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}
