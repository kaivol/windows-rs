use std::mem;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use windows::core::WeakRefCount;
use windows::core::{IUnknown, Interface};

#[test]
fn test() {
    // Ref count starts at 1
    let count = WeakRefCount::new();
    assert_eq!(count.add_ref(), 2);
    assert_eq!(count.add_ref(), 3);
    assert_eq!(count.release(), 2);
    assert_eq!(count.release(), 1);

    // Query implies add_ref
    let weak_reference_source = unsafe {
        count.query(ManuallyDrop::new(IUnknown::from_raw(
            NonNull::dangling().as_ptr(),
        )))
    };
    mem::forget(weak_reference_source);

    // Ref count is now owned by tearoff
    assert_eq!(count.add_ref(), 3);
    assert_eq!(count.release(), 2);
    assert_eq!(count.release(), 1);
    assert_eq!(count.release(), 0);
}
