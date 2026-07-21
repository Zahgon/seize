
use std::ptr;

use crate::Collector;

pub unsafe fn boxed<T>(ptr: *mut T, _collector: &Collector) { panic!("STUB: not implemented") }

pub unsafe fn in_place<T>(ptr: *mut T, _collector: &Collector) { panic!("STUB: not implemented") }
