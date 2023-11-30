#![allow(unstable_name_collisions)]
use crate::imp::*;
use crate::{ComInterface, IUnknown};
use sptr::Strict;
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::process::abort;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::{mem, ptr};
use windows_core::{imp, Interface};

#[doc(hidden)]
#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicPtr<c_void>);

enum Repr {
    Pointer(NonNull<TearOff>),
    Count(u32),
}

impl WeakRefCount {
    pub fn new() -> Self {
        Self(AtomicPtr::new(sptr::invalid_mut(1usize)))
    }

    fn as_inner_repr(value: *mut c_void) -> Repr {
        if Self::is_weak_ref(value) {
            Repr::Pointer(unsafe { Self::decode_tear_off(value) })
        } else {
            Repr::Count(value.addr() as u32)
        }
    }

    fn update(
        &self,
        mut on_count: impl FnMut(u32) -> u32,
        on_tear_off: impl FnOnce(NonNull<TearOff>) -> u32,
    ) -> u32 {
        let mut prev = self.0.load(Ordering::Relaxed);
        loop {
            let temp = match Self::as_inner_repr(prev) {
                Repr::Pointer(tear_off) => return on_tear_off(tear_off),
                Repr::Count(count) => on_count(count),
            };
            let next = sptr::invalid_mut(temp as usize);

            match self
                .0
                .compare_exchange_weak(prev, next, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => return temp,
                Err(next_prev) => prev = next_prev,
            }
        }
    }

    pub fn add_ref(&self) -> u32 {
        self.update(
            |count| count.checked_add(1).unwrap_or_else(|| abort()),
            |tear_off| unsafe { tear_off.as_ref().strong_add_ref() },
        )
    }

    pub fn release(&self) -> u32 {
        self.update(
            |count| count.checked_sub(1).unwrap_or_else(|| abort()),
            |tear_off| unsafe { TearOff::strong_release_inner(tear_off) },
        )
    }

    /// # Safety
    pub unsafe fn query(&self, object: ManuallyDrop<IUnknown>) -> IWeakReferenceSource {
        let mut prev = self.0.load(Ordering::Relaxed);

        let count = match Self::as_inner_repr(prev) {
            Repr::Pointer(tear_off) => return tear_off.as_ref().new_weak_reference_source(),
            Repr::Count(count) => count,
        };

        let tear_off_ptr = TearOff::new(object, count);
        let encoding = Self::encode_tear_off(tear_off_ptr);

        loop {
            match self
                .0
                .compare_exchange_weak(prev, encoding, Ordering::AcqRel, Ordering::Relaxed)
            {
                // successfully stored the newly created `TearOff`
                // use it to create a new `IWeakReferenceSource`
                Ok(_) => {
                    return tear_off_ptr.as_ref().new_weak_reference_source();
                }
                Err(new_prev) => match Self::as_inner_repr(new_prev) {
                    // the `TearOff` has been set somewhere else in the meantime
                    // free our `TearOff` and create a new `IWeakReferenceSource` from the other one
                    Repr::Pointer(tear_off) => {
                        let _ = Box::from_raw(tear_off_ptr.as_ptr());
                        return tear_off.as_ref().new_weak_reference_source();
                    }
                    // the simple count has been modified from somewhere else
                    // update our `TearOff` with this new count and try again
                    Repr::Count(count) => {
                        prev = new_prev;
                        tear_off_ptr
                            .as_ref()
                            .strong_count
                            .0
                            .store(count, Ordering::SeqCst);
                    }
                },
            }
        }
    }

    fn encode_tear_off(value: NonNull<TearOff>) -> *mut c_void {
        value
            .as_ptr()
            .map_addr(|ptr| ptr >> 1 | (1 << (mem::size_of::<usize>() * 8 - 1)))
            .cast()
    }

    unsafe fn decode_tear_off(value: *mut c_void) -> NonNull<TearOff> {
        NonNull::new_unchecked(value.map_addr(|ptr| ptr << 1).cast())
    }

    fn is_weak_ref(value: *mut c_void) -> bool {
        (value as isize) < 0
    }
}

#[repr(C)]
struct TearOff {
    strong_vtable: *const IWeakReferenceSource_Vtbl,
    weak_vtable: *const IWeakReference_Vtbl,
    object: ManuallyDrop<IUnknown>,
    strong_count: RefCount,
    weak_count: RefCount,
}

impl TearOff {
    #[allow(clippy::new_ret_no_self)]
    unsafe fn new(object: ManuallyDrop<IUnknown>, strong_count: u32) -> NonNull<TearOff> {
        NonNull::from(Box::leak(Box::new(TearOff {
            strong_vtable: &Self::STRONG_VTABLE,
            weak_vtable: &Self::WEAK_VTABLE,
            object,
            strong_count: RefCount::new(strong_count),
            weak_count: RefCount::new(1),
        })))
    }

    unsafe fn new_weak_reference_source(&self) -> IWeakReferenceSource {
        self.strong_count.add_ref();
        IWeakReferenceSource::from_raw(self as *const Self as *mut Self as *mut _)
    }

    unsafe fn new_weak_reference(&self) -> IWeakReference {
        self.weak_count.add_ref();
        IWeakReference::from_raw(
            (self as *const Self as *mut Self)
                .cast::<*mut c_void>()
                .offset(1)
                .cast(),
        )
    }

    unsafe fn from_strong_ptr<'a>(this: *mut std::ffi::c_void) -> &'a Self {
        &*(this as *mut *mut std::ffi::c_void as *mut Self)
    }

    unsafe fn from_weak_ptr<'a>(this: *mut std::ffi::c_void) -> &'a Self {
        &*((this as *mut *mut std::ffi::c_void).sub(1) as *mut Self)
    }

    unsafe fn strong_query_interface(
        &self,
        iid: *const crate::GUID,
        interface: *mut *mut std::ffi::c_void,
    ) -> crate::HRESULT {
        if let (Some(iid), Some(interface)) = (iid.as_ref(), interface.as_mut()) {
            if *iid == IWeakReferenceSource::IID {
                // Only directly respond to queries for the the tear-off's strong interface. This is
                // effectively a self-query.
                *interface = self.new_weak_reference_source().into_raw();
                crate::HRESULT(0)
            } else {
                // As the tear-off is sharing the identity of the object, simply delegate any remaining
                // queries to the object.
                self.object.query(iid, interface)
            }
        } else {
            imp::E_POINTER
        }
    }

    fn strong_add_ref(&self) -> u32 {
        // Implement `AddRef` directly as we own the weak reference.
        self.strong_count.add_ref()
    }

    unsafe fn strong_release(&self) -> u32 {
        // Forward strong `Release` to the object so that it can destroy itself. It will then
        // decrement its weak reference and allow the tear-off to be released as needed.
        ((self.object.assume_vtable::<IUnknown>()).Release)(self.object.as_raw())
    }

    unsafe fn strong_release_inner(this: NonNull<TearOff>) -> u32 {
        let remaining = this.as_ref().strong_count.release();

        // If this is the last strong reference, we can release the weak reference implied by the strong reference.
        // There may still be weak references, so the WeakRelease is called to handle such possibilities.
        if remaining == 0 {
            TearOff::weak_release(this);
        }

        remaining
    }

    unsafe fn strong_downgrade(&self, interface: *mut *mut std::ffi::c_void) -> crate::HRESULT {
        // The strong vtable hands out a reference to the weak vtable. This is always safe and
        // straightforward since a strong reference guarantees there is at least one weak
        // reference.
        *interface = self.new_weak_reference().into_raw();
        crate::HRESULT(0)
    }

    unsafe fn weak_release(this: NonNull<TearOff>) -> u32 {
        // Implement `Release` directly as we own the weak reference.
        let remaining = this.as_ref().weak_count.release();

        // If there are no remaining references, it means that the object has already been
        // destroyed. Go ahead and destroy the tear-off.
        if remaining == 0 {
            let _ = std::boxed::Box::from_raw(this.as_ptr());
        }

        remaining
    }

    unsafe fn weak_add_ref(&self) -> u32 {
        // Implement `AddRef` directly as we own the weak reference.
        self.weak_count.add_ref()
    }

    unsafe fn weak_query_interface(
        &self,
        iid: *const crate::GUID,
        interface: *mut *mut std::ffi::c_void,
    ) -> crate::HRESULT {
        if let (Some(iid), Some(interface)) = (iid.as_ref(), interface.as_mut()) {
            // While the weak vtable is packed into the same allocation as the strong vtable and
            // tear-off, it represents a distinct COM identity and thus does not share or delegate to
            // the object.

            if *iid == IWeakReference::IID || *iid == IUnknown::IID || *iid == IAgileObject::IID {
                *interface = self.new_weak_reference().into_raw();
                S_OK
            } else {
                *interface = ptr::null_mut();
                E_NOINTERFACE
            }
        } else {
            imp::E_POINTER
        }
    }
    unsafe fn weak_upgrade(
        &self,
        iid: *const crate::GUID,
        interface: *mut *mut std::ffi::c_void,
    ) -> crate::HRESULT {
        if let (Some(iid), Some(interface)) = (iid.as_ref(), interface.as_mut()) {
            match self
                .strong_count
                .0
                .fetch_update(Ordering::Acquire, Ordering::Relaxed, |count| {
                    // Attempt to acquire a strong reference count to stabilize the object for the duration
                    // of the `QueryInterface` call.
                    then_some(count != 0, count + 1)
                }) {
                Ok(_) => {
                    // Let the object respond to the upgrade query.
                    let result = self.object.query(iid, interface);
                    // Decrement the temporary reference account used to stabilize the object.
                    self.strong_count.0.fetch_sub(1, Ordering::Relaxed);
                    // Return the result of the query.
                    result
                }
                Err(_) => {
                    *interface = ptr::null_mut();
                    S_OK
                }
            }
        } else {
            imp::E_POINTER
        }
    }

    const STRONG_VTABLE: IWeakReferenceSource_Vtbl = {
        unsafe extern "system" fn QueryInterface(
            ptr: *mut std::ffi::c_void,
            iid: *const crate::GUID,
            interface: *mut *mut std::ffi::c_void,
        ) -> crate::HRESULT {
            let this = TearOff::from_strong_ptr(ptr);
            TearOff::strong_query_interface(this, iid, interface)
        }
        unsafe extern "system" fn AddRef(ptr: *mut std::ffi::c_void) -> u32 {
            let this = TearOff::from_strong_ptr(ptr);
            TearOff::strong_add_ref(this)
        }
        unsafe extern "system" fn Release(ptr: *mut std::ffi::c_void) -> u32 {
            let this = TearOff::from_strong_ptr(ptr);
            TearOff::strong_release(this)
        }
        unsafe extern "system" fn GetWeakReference(
            ptr: *mut std::ffi::c_void,
            interface: *mut *mut std::ffi::c_void,
        ) -> crate::HRESULT {
            let this = TearOff::from_strong_ptr(ptr);
            TearOff::strong_downgrade(this, interface)
        }
        IWeakReferenceSource_Vtbl {
            base__: crate::IUnknown_Vtbl {
                QueryInterface,
                AddRef,
                Release,
            },
            GetWeakReference,
        }
    };

    const WEAK_VTABLE: IWeakReference_Vtbl = {
        unsafe extern "system" fn QueryInterface(
            ptr: *mut std::ffi::c_void,
            iid: *const crate::GUID,
            interface: *mut *mut std::ffi::c_void,
        ) -> crate::HRESULT {
            let this = TearOff::from_weak_ptr(ptr);
            TearOff::weak_query_interface(this, iid, interface)
        }
        unsafe extern "system" fn AddRef(ptr: *mut std::ffi::c_void) -> u32 {
            let this = TearOff::from_weak_ptr(ptr);
            TearOff::weak_add_ref(this)
        }
        unsafe extern "system" fn Release(ptr: *mut std::ffi::c_void) -> u32 {
            let this = TearOff::from_weak_ptr(ptr);
            TearOff::weak_release(this.into())
        }
        unsafe extern "system" fn Resolve(
            ptr: *mut std::ffi::c_void,
            iid: *const crate::GUID,
            interface: *mut *mut std::ffi::c_void,
        ) -> crate::HRESULT {
            let this = TearOff::from_weak_ptr(ptr);
            TearOff::weak_upgrade(this, iid, interface)
        }
        IWeakReference_Vtbl {
            base__: crate::IUnknown_Vtbl {
                QueryInterface,
                AddRef,
                Release,
            },
            Resolve,
        }
    };
}
