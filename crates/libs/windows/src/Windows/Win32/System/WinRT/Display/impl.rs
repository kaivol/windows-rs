#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDisplayDeviceInterop_Impl: ::windows_core::BaseImpl {
    fn CreateSharedHandle(this: &Self::This, pobject: ::core::option::Option<&::windows_core::IInspectable>, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(this: &Self::This, nthandle: super::super::super::Foundation::HANDLE, riid: &::windows_core::GUID) -> ::windows_core::Result<*mut ::core::ffi::c_void>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::Iids for IDisplayDeviceInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayDeviceInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDisplayDeviceInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayDeviceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSharedHandle(this, ::windows_core::from_raw_borrowed(&pobject), ::core::mem::transmute_copy(&psecurityattributes), ::core::mem::transmute_copy(&access), ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenSharedHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayDeviceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenSharedHandle(this, ::core::mem::transmute_copy(&nthandle), ::core::mem::transmute(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDisplayDeviceInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDisplayPathInterop_Impl: ::windows_core::BaseImpl {
    fn CreateSourcePresentationHandle(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn GetSourceId(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDisplayPathInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayPathInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDisplayPathInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSourcePresentationHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayPathInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSourcePresentationHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayPathInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psourceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDisplayPathInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSourcePresentationHandle: CreateSourcePresentationHandle::<Identity, Impl, OFFSET>,
            GetSourceId: GetSourceId::<Identity, Impl, OFFSET>,
        }
    };
}
