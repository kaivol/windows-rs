use std::process::abort;
use std::sync::atomic::{fence, AtomicU32, Ordering};

#[doc(hidden)]
#[repr(transparent)]
#[derive(Default)]
pub struct RefCount(pub(crate) AtomicU32);

impl RefCount {
    /// Creates a new `RefCount` with an initial value of `1`.
    pub fn new(count: u32) -> Self {
        Self(AtomicU32::new(count))
    }

    /// Increments the reference count, returning the new value.
    pub fn add_ref(&self) -> u32 {
        let old = self.0.fetch_add(1, Ordering::Relaxed);
        if old == u32::MAX {
            abort();
        }
        old + 1
    }

    /// Decrements the reference count, returning the new value.
    ///
    /// This operation inserts an `Acquire` fence when the reference count reaches zero.
    /// This prevents reordering before the object is destroyed.
    pub fn release(&self) -> u32 {
        let old = self.0.fetch_sub(1, Ordering::Release);

        // Object has been over-released
        if old == 0 {
            abort()
        }

        if old == 1 {
            fence(Ordering::Acquire);
        }

        old - 1
    }
}
