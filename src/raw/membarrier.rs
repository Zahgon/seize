
#![allow(dead_code)]

#[cfg(all(target_os = "linux", feature = "fast-barrier", not(miri)))]
pub use linux::*;

#[cfg(all(target_os = "windows", feature = "fast-barrier", not(miri)))]
pub use windows::*;

#[cfg(any(
    not(feature = "fast-barrier"),
    not(any(target_os = "windows", target_os = "linux")),
    miri
))]
pub use default::*;

#[cfg(any(
    not(feature = "fast-barrier"),
    not(any(target_os = "windows", target_os = "linux")),
    miri
))]
mod default {
    use core::sync::atomic::{fence, Ordering};

    pub fn detect() { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_store() -> Ordering { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_barrier() { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_load() -> Ordering { panic!("STUB: not implemented") }

    #[inline]
    pub fn heavy() { panic!("STUB: not implemented") }
}

#[cfg(all(target_os = "linux", feature = "fast-barrier", not(miri)))]
mod linux {
    use std::sync::atomic::{self, AtomicU8, Ordering};

    #[inline]
    pub fn light_store() -> Ordering { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_barrier() { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_load() -> Ordering { panic!("STUB: not implemented") }

    #[inline]
    pub fn heavy() { panic!("STUB: not implemented") }

    const MEMBARRIER: u8 = 0;

    const MPROTECT: u8 = 1;

    const FALLBACK: u8 = 2;

    static STRATEGY: AtomicU8 = AtomicU8::new(FALLBACK);

    pub fn detect() { panic!("STUB: not implemented") }

    macro_rules! fatal_assert {
        ($cond:expr) => {
            if !$cond {
                #[allow(unused_unsafe)]
                unsafe {
                    libc::abort();
                }
            }
        };
    }

    mod membarrier {
        
        #[repr(i32)]
        #[allow(dead_code, non_camel_case_types)]
        enum membarrier_cmd {
            MEMBARRIER_CMD_QUERY = 0,
            MEMBARRIER_CMD_GLOBAL = (1 << 0),
            MEMBARRIER_CMD_GLOBAL_EXPEDITED = (1 << 1),
            MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED = (1 << 2),
            MEMBARRIER_CMD_PRIVATE_EXPEDITED = (1 << 3),
            MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED = (1 << 4),
            MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE = (1 << 5),
            MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE = (1 << 6),
        }

        #[inline]
        fn sys_membarrier(cmd: membarrier_cmd) -> libc::c_long { panic!("STUB: not implemented") }

        pub fn is_supported() -> bool { panic!("STUB: not implemented") }

        #[inline]
        pub fn barrier() { panic!("STUB: not implemented") }
    }

    mod mprotect {
        use std::cell::UnsafeCell;
        use std::mem::MaybeUninit;
        use std::ptr;
        use std::sync::{atomic, OnceLock};

        struct Barrier {
            lock: UnsafeCell<libc::pthread_mutex_t>,
            page: u64,
            page_size: libc::size_t,
        }

        unsafe impl Sync for Barrier {}

        impl Barrier {
            
            #[inline]
            fn barrier(&self) { panic!("STUB: not implemented") }
        }

        static BARRIER: OnceLock<Barrier> = OnceLock::new();

        pub fn is_supported() -> bool { panic!("STUB: not implemented") }

        #[inline]
        pub fn barrier() { panic!("STUB: not implemented") }
    }
}

#[cfg(all(target_os = "windows", feature = "fast-barrier", not(miri)))]
mod windows {
    use core::sync::atomic::{self, Ordering};
    use windows_sys;

    pub fn detect() { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_store() -> Ordering { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_barrier() { panic!("STUB: not implemented") }

    #[inline]
    pub fn light_load() -> Ordering { panic!("STUB: not implemented") }

    #[inline]
    pub fn heavy() { panic!("STUB: not implemented") }
}
