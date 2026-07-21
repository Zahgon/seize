
mod thread_id;

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{self, AtomicBool, AtomicPtr, Ordering};
use std::{mem, ptr};

pub use thread_id::Thread;

pub struct ThreadLocal<T> {
    
    buckets: [AtomicPtr<Entry<T>>; thread_id::BUCKETS],
}

struct Entry<T> {
    
    present: AtomicBool,

    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send> Send for ThreadLocal<T> {}

unsafe impl<T: Send> Sync for ThreadLocal<T> {}

impl<T> ThreadLocal<T> {
    
    pub fn with_capacity(capacity: usize) -> ThreadLocal<T> { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn load(&self, thread: Thread) -> &T
    where
        T: Default,
    { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn load_or(&self, create: impl Fn() -> T, thread: Thread) -> &T { panic!("STUB: not implemented") }

    #[cfg(test)]
    fn try_load(&self) -> Option<&T> {
        let thread = Thread::current();

        let bucket_ptr =
            unsafe { self.buckets.get_unchecked(thread.bucket) }.load(Ordering::Acquire);

        if bucket_ptr.is_null() {
            return None;
        }

        let entry = unsafe { &*bucket_ptr.add(thread.entry) };

        if !entry.present.load(Ordering::Relaxed) {
            return None;
        }

        unsafe { Some((*entry.value.get()).assume_init_ref()) }
    }

    #[cold]
    #[inline(never)]
    unsafe fn write(&self, entry: &Entry<T>, create: impl Fn() -> T) { panic!("STUB: not implemented") }

    #[cold]
    #[inline(never)]
    fn initialize(&self, bucket: &AtomicPtr<Entry<T>>, thread: Thread) -> *mut Entry<T> { panic!("STUB: not implemented") }

    #[inline]
    pub unsafe fn iter(&self) -> Iter<'_, T> { panic!("STUB: not implemented") }
}

impl<T> Drop for ThreadLocal<T> {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

impl<T> Drop for Entry<T> {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

pub struct Iter<'a, T> {
    bucket: usize,
    index: usize,
    bucket_size: usize,
    thread_local: &'a ThreadLocal<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { panic!("STUB: not implemented") }
}

fn allocate_bucket<T>(capacity: usize) -> *mut Entry<T> { panic!("STUB: not implemented") }

#[cfg(test)]
#[allow(clippy::redundant_closure)]
mod tests {
    use super::*;

    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::Relaxed;
    use std::sync::{Arc, Barrier};
    use std::thread;

    fn make_create() -> Arc<dyn Fn() -> usize + Send + Sync> {
        let count = AtomicUsize::new(0);
        Arc::new(move || count.fetch_add(1, Relaxed))
    }

    #[test]
    fn same_thread() {
        
        unsafe {
            let create = make_create();
            let tls = ThreadLocal::with_capacity(1);
            assert_eq!(None, tls.try_load());
            assert_eq!(0, *tls.load_or(|| create(), Thread::current()));
            assert_eq!(Some(&0), tls.try_load());
            assert_eq!(0, *tls.load_or(|| create(), Thread::current()));
            assert_eq!(Some(&0), tls.try_load());
            assert_eq!(0, *tls.load_or(|| create(), Thread::current()));
            assert_eq!(Some(&0), tls.try_load());
        }
    }

    #[test]
    fn different_thread() {
        
        unsafe {
            let create = make_create();
            let tls = Arc::new(ThreadLocal::with_capacity(1));
            assert_eq!(None, tls.try_load());
            assert_eq!(0, *tls.load_or(|| create(), Thread::current()));
            assert_eq!(Some(&0), tls.try_load());

            let tls2 = tls.clone();
            let create2 = create.clone();
            thread::spawn(move || {
                assert_eq!(None, tls2.try_load());
                assert_eq!(1, *tls2.load_or(|| create2(), Thread::current()));
                assert_eq!(Some(&1), tls2.try_load());
            })
            .join()
            .unwrap();

            assert_eq!(Some(&0), tls.try_load());
            assert_eq!(0, *tls.load_or(|| create(), Thread::current()));
        }
    }

    #[test]
    fn iter() {
        
        unsafe {
            let tls = Arc::new(ThreadLocal::with_capacity(1));
            tls.load_or(|| Box::new(1), Thread::current());

            let tls2 = tls.clone();
            thread::spawn(move || {
                tls2.load_or(|| Box::new(2), Thread::current());
                let tls3 = tls2.clone();
                thread::spawn(move || {
                    tls3.load_or(|| Box::new(3), Thread::current());
                })
                .join()
                .unwrap();
                drop(tls2);
            })
            .join()
            .unwrap();

            let tls = Arc::try_unwrap(tls).unwrap_or_else(|_| panic!("."));

            let mut v = tls.iter().map(|x| **x).collect::<Vec<i32>>();
            v.sort_unstable();
            assert_eq!(vec![1, 2, 3], v);
        }
    }

    #[test]
    fn iter_snapshot() {
        
        unsafe {
            let tls = Arc::new(ThreadLocal::with_capacity(1));
            tls.load_or(|| Box::new(1), Thread::current());

            let iterator = tls.iter();
            tls.load_or(|| Box::new(2), Thread::current());

            let v = iterator.map(|x| **x).collect::<Vec<i32>>();
            assert_eq!(vec![1], v);
        }
    }

    #[test]
    fn test_drop() {
        let local = ThreadLocal::with_capacity(1);
        struct Dropped(Arc<AtomicUsize>);
        impl Drop for Dropped {
            fn drop(&mut self) {
                self.0.fetch_add(1, Relaxed);
            }
        }

        let dropped = Arc::new(AtomicUsize::new(0));
        
        unsafe {
            local.load_or(|| Dropped(dropped.clone()), Thread::current());
        }
        assert_eq!(dropped.load(Relaxed), 0);
        drop(local);
        assert_eq!(dropped.load(Relaxed), 1);
    }

    #[test]
    fn iter_many() {
        let tls = Arc::new(ThreadLocal::with_capacity(0));
        let barrier = Arc::new(Barrier::new(65));

        for i in 0..64 {
            let tls = tls.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                dbg!(i);
                
                unsafe {
                    tls.load_or(|| 1, Thread::current());
                }
                barrier.wait();
            });
        }

        barrier.wait();
        unsafe { assert_eq!(tls.iter().count(), 64) }
    }
}
