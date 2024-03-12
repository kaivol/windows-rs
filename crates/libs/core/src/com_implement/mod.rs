mod default;
mod inspectable;
mod unknown;
mod weak_ref_count;

pub use default::*;
pub use weak_ref_count::WeakRefCount;

use crate::{Array, ComInterface, GUID, HSTRING};
use std::ffi;
use std::ptr::NonNull;

// common traits

#[doc(hidden)]
pub trait Vtable<Identity, const OFFSET: usize>: ComInterface {
    const VTABLE: Self::Vtable;
}

#[doc(hidden)]
pub trait Iids: ComInterface {
    const IIDS: &'static [GUID];
    const REQUIRED_IIDS: &'static [GUID] = &[];
}

#[macro_export]
macro_rules! concat_iids {
    ($parent:ty) => {{
        &{
            const parent: &'static [::windows::core::GUID] = <$parent>::IIDS;
            let mut res = [<Self as ::windows::core::ComInterface>::IID; parent.len() + 1];
            let mut i = 0;
            while i < parent.len() {
                res[i] = parent[i];
                i += 1;
            }
            res
        }
    }};
}

pub trait ImplProvider {
    /// Type of the backing implementation.
    type Impl: BaseImpl;

    /// Call the rust implementation for the given interface method `f`
    ///
    /// # Safety
    unsafe fn call_impl<Result, const OFFSET: usize>(
        interface: *mut ffi::c_void,
        f: impl FnOnce(&<Self::Impl as BaseImpl>::This) -> Result,
    ) -> Result;

    /// The classic `QueryInterface` method from COM.
    ///
    /// # Safety
    unsafe fn query_interface<const OFFSET: usize>(
        interface: *mut ffi::c_void,
        iid: &GUID,
    ) -> Option<NonNull<ffi::c_void>>;

    /// Increments the reference count of the interface
    ///
    /// # Safety
    unsafe fn add_ref<const OFFSET: usize>(interface: *mut ffi::c_void) -> u32;

    /// Decrements the reference count causing the interface's memory to be freed when the count is zero
    ///
    /// # Safety
    unsafe fn release<const OFFSET: usize>(interface: *mut ffi::c_void) -> u32;

    /// Gets the IIDs of all interfaces that are implemented by this class
    fn get_iids() -> Array<GUID>;

    /// Gets the fully qualified name of this class
    fn get_runtime_name() -> HSTRING;

    /// Gets the trust level of this class
    fn get_trust_level() -> i32;
}

pub trait BaseImpl: Sized {
    /// Type of the `this` parameter for the interface implementations
    type This;
}
