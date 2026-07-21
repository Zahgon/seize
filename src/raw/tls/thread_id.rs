
use std::cell::Cell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Mutex, OnceLock};

#[derive(Default)]
struct ThreadIdManager {
    free_from: usize,
    free_list: BinaryHeap<Reverse<usize>>,
}

impl ThreadIdManager {
    
    fn alloc(&mut self) -> usize { panic!("STUB: not implemented") }

    fn free(&mut self, id: usize) { panic!("STUB: not implemented") }
}

fn thread_id_manager() -> &'static Mutex<ThreadIdManager> { panic!("STUB: not implemented") }

#[derive(Clone, Copy)]
pub struct Thread {
    
    pub id: usize,

    pub entry: usize,

    pub bucket: usize,
}

const ZERO_ENTRY: usize = 31;

const ZERO_BUCKET: usize = (usize::BITS - ZERO_ENTRY.leading_zeros()) as usize;

pub const BUCKETS: usize = (usize::BITS as usize) - ZERO_BUCKET;

const MAX_INDEX: usize = usize::MAX - ZERO_ENTRY - 1;

impl Thread {
    
    #[inline]
    pub fn new(id: usize) -> Thread { panic!("STUB: not implemented") }

    #[inline]
    pub fn bucket_capacity(bucket: usize) -> usize { panic!("STUB: not implemented") }

    #[inline]
    pub fn current() -> Thread { panic!("STUB: not implemented") }

    #[cold]
    #[inline(never)]
    fn init_slow(thread: &Cell<Option<Thread>>) -> Thread { panic!("STUB: not implemented") }

    pub fn create() -> Thread { panic!("STUB: not implemented") }

    pub unsafe fn free(id: usize) { panic!("STUB: not implemented") }
}

thread_local! { static THREAD: Cell<Option<Thread>> = const { Cell::new(None) }; }
thread_local! { static THREAD_GUARD: ThreadGuard = const { ThreadGuard { id: Cell::new(0) } }; }

struct ThreadGuard {
    
    id: Cell<usize>,
}

impl Drop for ThreadGuard {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread() {
        assert_eq!(Thread::bucket_capacity(0), 32);
        for i in 0..32 {
            let thread = Thread::new(i);
            assert_eq!(thread.id, i);
            assert_eq!(thread.bucket, 0);
            assert_eq!(thread.entry, i);
        }

        assert_eq!(Thread::bucket_capacity(1), 64);
        for i in 33..96 {
            let thread = Thread::new(i);
            assert_eq!(thread.id, i);
            assert_eq!(thread.bucket, 1);
            assert_eq!(thread.entry, i - 32);
        }

        assert_eq!(Thread::bucket_capacity(2), 128);
        for i in 96..224 {
            let thread = Thread::new(i);
            assert_eq!(thread.id, i);
            assert_eq!(thread.bucket, 2);
            assert_eq!(thread.entry, i - 96);
        }
    }

    #[test]
    fn max_entries() {
        let mut entries = 0;
        for i in 0..BUCKETS {
            entries += Thread::bucket_capacity(i);
        }
        assert_eq!(entries, MAX_INDEX + 1);

        let max = Thread::new(MAX_INDEX);
        assert_eq!(max.id, MAX_INDEX);
        assert_eq!(max.bucket, BUCKETS - 1);
        assert_eq!(Thread::bucket_capacity(BUCKETS - 1), 1 << (usize::BITS - 1));
        assert_eq!(max.entry, (1 << (usize::BITS - 1)) - 1);
    }
}
