use crate::*;

impl Iids for IInspectable {
    const IIDS: &'static [GUID] = &[IUnknown::IID, Self::IID];
}

impl<Identity: ImplProvider, const OFFSET: usize> Vtable<Identity, OFFSET> for IInspectable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIids<Identity: ImplProvider, const OFFSET: usize>(
            _this: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut ::windows_core::GUID,
        ) -> ::windows_core::HRESULT {
            let (ok_data__, ok_data_len__) = Identity::get_iids().into_abi();
            ::core::ptr::write(result__, ok_data__);
            ::core::ptr::write(result_size__, ok_data_len__);
            ::windows_core::HRESULT(0)
        }
        unsafe extern "system" fn GetRuntimeClassName<
            Identity: ::windows_core::ImplProvider,
            const OFFSET: usize,
        >(
            _this: *mut ::core::ffi::c_void,
            result__: *mut std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let value__ = Identity::get_runtime_name();
            ::core::ptr::write(result__, ::core::mem::transmute_copy(&value__));
            ::core::mem::forget(value__);
            ::windows_core::HRESULT(0)
        }
        unsafe extern "system" fn GetTrustLevel<
            Identity: ::windows_core::ImplProvider,
            const OFFSET: usize,
        >(
            _this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let value__ = Identity::get_trust_level();
            ::core::ptr::write(result__, value__);
            ::windows_core::HRESULT(0)
        }
        IInspectable_Vtbl {
            base: <IUnknown as Vtable<Identity, OFFSET>>::VTABLE,
            GetIids: GetIids::<Identity, OFFSET>,
            GetRuntimeClassName: GetRuntimeClassName::<Identity, OFFSET>,
            GetTrustLevel: GetTrustLevel::<Identity, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as Vtable<Identity, OFFSET>>::VTABLE;
}
