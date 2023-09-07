pub trait ISceSvcAttachmentData_Impl: ::windows_core::BaseImpl {
    fn GetData(this: &Self::This, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: ::core::option::Option<&ISceSvcAttachmentPersistInfo>, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FreeBuffer(this: &Self::This, pvdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CloseHandle(this: &Self::This, scesvchandle: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISceSvcAttachmentData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISceSvcAttachmentData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&scesvchandle), ::core::mem::transmute_copy(&scetype), ::core::mem::transmute_copy(&ppvdata), ::core::mem::transmute_copy(&psceenumhandle)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: *mut ::core::ffi::c_void, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&lpservicename), ::core::mem::transmute_copy(&lptemplatename), ::windows_core::from_raw_borrowed(&lpscesvcpersistinfo), ::core::mem::transmute_copy(&pscesvchandle)).into())
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeBuffer(this, ::core::mem::transmute_copy(&pvdata)).into())
        }
        unsafe extern "system" fn CloseHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseHandle(this, ::core::mem::transmute_copy(&scesvchandle)).into())
        }
        ISceSvcAttachmentData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetData: GetData::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            CloseHandle: CloseHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISceSvcAttachmentPersistInfo_Impl: ::windows_core::BaseImpl {
    fn Save(this: &Self::This, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsDirty(this: &Self::This, lptemplatename: *mut i8) -> ::windows_core::HRESULT;
    fn FreeBuffer(this: &Self::This, pvdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISceSvcAttachmentPersistInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISceSvcAttachmentPersistInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::core::mem::transmute_copy(&lptemplatename), ::core::mem::transmute_copy(&scesvchandle), ::core::mem::transmute_copy(&ppvdata), ::core::mem::transmute_copy(&pboverwriteall)).into())
        }
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this, ::core::mem::transmute_copy(&lptemplatename)))
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeBuffer(this, ::core::mem::transmute_copy(&pvdata)).into())
        }
        ISceSvcAttachmentPersistInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Save: Save::<Identity, Impl, OFFSET>,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
