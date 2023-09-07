use crate::*;

impl Iids for IUnknown {
    const IIDS: &'static [GUID] = &[Self::IID];
}

impl<Identity: ImplProvider, const OFFSET: usize> Vtable<Identity, OFFSET> for IUnknown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryInterface<Identity: ImplProvider, const OFFSET: usize>(
            this: *mut std::ffi::c_void,
            iid: *const GUID,
            interface: *mut *mut std::ffi::c_void,
        ) -> HRESULT {
            if let (Some(iid), Some(interface)) = (iid.as_ref(), interface.as_mut()) {
                if let Some(i) = Identity::query_interface::<OFFSET>(this, iid) {
                    *interface = i.as_ptr();
                    imp::S_OK
                } else {
                    *interface = std::ptr::null_mut();
                    imp::E_NOINTERFACE
                }
            } else {
                imp::E_POINTER
            }
        }
        unsafe extern "system" fn AddRef<Identity: ImplProvider, const OFFSET: usize>(
            this: *mut std::ffi::c_void,
        ) -> u32 {
            Identity::add_ref::<OFFSET>(this)
        }
        unsafe extern "system" fn Release<Identity: ImplProvider, const OFFSET: usize>(
            this: *mut std::ffi::c_void,
        ) -> u32 {
            Identity::release::<OFFSET>(this)
        }
        IUnknown_Vtbl {
            QueryInterface: QueryInterface::<Identity, OFFSET>,
            AddRef: AddRef::<Identity, OFFSET>,
            Release: Release::<Identity, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as Vtable<Identity, OFFSET>>::VTABLE;
}
