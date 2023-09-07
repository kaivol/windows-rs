use super::*;

/// All COM interfaces (and thus WinRT classes and interfaces) implement
/// [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// under the hood to provide reference-counted lifetime management as well as the ability
/// to query for additional interfaces that the object may implement.
#[repr(transparent)]
pub struct IUnknown(std::ptr::NonNull<std::ffi::c_void>);

#[doc(hidden)]
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut std::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut std::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> u32,
}

unsafe impl Interface for IUnknown {
    type Vtable = IUnknown_Vtbl;
}

unsafe impl ComInterface for IUnknown {
    const IID: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().AddRef)(std::mem::transmute_copy(self));
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().Release)(std::mem::transmute_copy(self));
        }
    }
}

impl PartialEq for IUnknown {
    fn eq(&self, other: &Self) -> bool {
        // Since COM objects may implement multiple interfaces, COM identity can only
        // be determined by querying for `IUnknown` explicitly and then comparing the
        // pointer values. This works since `QueryInterface` is required to return
        // the same pointer value for queries for `IUnknown`.
        self.cast::<IUnknown>().unwrap().0 == other.cast::<IUnknown>().unwrap().0
    }
}

impl Eq for IUnknown {}

impl std::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IUnknown").field(&self.0).finish()
    }
}
