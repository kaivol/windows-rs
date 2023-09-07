#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink_Impl: ::windows_core::BaseImpl {
    fn Begin_OnDataChange(this: &Self::This, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn Finish_OnDataChange(this: &Self::This);
    fn Begin_OnViewChange(this: &Self::This, dwaspect: u32, lindex: i32);
    fn Finish_OnViewChange(this: &Self::This);
    fn Begin_OnRename(this: &Self::This, pmk: ::core::option::Option<&IMoniker>);
    fn Finish_OnRename(this: &Self::This);
    fn Begin_OnSave(this: &Self::This);
    fn Finish_OnSave(this: &Self::This);
    fn Begin_OnClose(this: &Self::This);
    fn Finish_OnClose(this: &Self::This);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for AsyncIAdviseSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIAdviseSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_OnDataChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_OnDataChange(this, ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pstgmed)))
        }
        unsafe extern "system" fn Finish_OnDataChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_OnDataChange(this))
        }
        unsafe extern "system" fn Begin_OnViewChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_OnViewChange(this, ::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex)))
        }
        unsafe extern "system" fn Finish_OnViewChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_OnViewChange(this))
        }
        unsafe extern "system" fn Begin_OnRename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_OnRename(this, ::windows_core::from_raw_borrowed(&pmk)))
        }
        unsafe extern "system" fn Finish_OnRename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_OnRename(this))
        }
        unsafe extern "system" fn Begin_OnSave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_OnSave(this))
        }
        unsafe extern "system" fn Finish_OnSave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_OnSave(this))
        }
        unsafe extern "system" fn Begin_OnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_OnClose(this))
        }
        unsafe extern "system" fn Finish_OnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_OnClose(this))
        }
        AsyncIAdviseSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_OnDataChange: Begin_OnDataChange::<Identity, Impl, OFFSET>,
            Finish_OnDataChange: Finish_OnDataChange::<Identity, Impl, OFFSET>,
            Begin_OnViewChange: Begin_OnViewChange::<Identity, Impl, OFFSET>,
            Finish_OnViewChange: Finish_OnViewChange::<Identity, Impl, OFFSET>,
            Begin_OnRename: Begin_OnRename::<Identity, Impl, OFFSET>,
            Finish_OnRename: Finish_OnRename::<Identity, Impl, OFFSET>,
            Begin_OnSave: Begin_OnSave::<Identity, Impl, OFFSET>,
            Finish_OnSave: Finish_OnSave::<Identity, Impl, OFFSET>,
            Begin_OnClose: Begin_OnClose::<Identity, Impl, OFFSET>,
            Finish_OnClose: Finish_OnClose::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink2_Impl: ::windows_core::BaseImpl + AsyncIAdviseSink_Impl {
    fn Begin_OnLinkSrcChange(this: &Self::This, pmk: ::core::option::Option<&IMoniker>);
    fn Finish_OnLinkSrcChange(this: &Self::This);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for AsyncIAdviseSink2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(AsyncIAdviseSink);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIAdviseSink2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_OnLinkSrcChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_OnLinkSrcChange(this, ::windows_core::from_raw_borrowed(&pmk)))
        }
        unsafe extern "system" fn Finish_OnLinkSrcChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIAdviseSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_OnLinkSrcChange(this))
        }
        AsyncIAdviseSink2_Vtbl {
            base__: <AsyncIAdviseSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_OnLinkSrcChange: Begin_OnLinkSrcChange::<Identity, Impl, OFFSET>,
            Finish_OnLinkSrcChange: Finish_OnLinkSrcChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIMultiQI_Impl: ::windows_core::BaseImpl {
    fn Begin_QueryMultipleInterfaces(this: &Self::This, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows_core::Result<()>;
    fn Finish_QueryMultipleInterfaces(this: &Self::This, pmqis: *mut MULTI_QI) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIMultiQI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMultiQI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIMultiQI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMultiQI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_QueryMultipleInterfaces(this, ::core::mem::transmute_copy(&cmqis), ::core::mem::transmute_copy(&pmqis)).into())
        }
        unsafe extern "system" fn Finish_QueryMultipleInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMultiQI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_QueryMultipleInterfaces(this, ::core::mem::transmute_copy(&pmqis)).into())
        }
        AsyncIMultiQI_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_QueryMultipleInterfaces: Begin_QueryMultipleInterfaces::<Identity, Impl, OFFSET>,
            Finish_QueryMultipleInterfaces: Finish_QueryMultipleInterfaces::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIPipeByte_Impl: ::windows_core::BaseImpl {
    fn Begin_Pull(this: &Self::This, crequest: u32) -> ::windows_core::Result<()>;
    fn Finish_Pull(this: &Self::This, buf: *mut u8, pcreturned: *mut u32) -> ::windows_core::Result<()>;
    fn Begin_Push(this: &Self::This, buf: *const u8, csent: u32) -> ::windows_core::Result<()>;
    fn Finish_Push(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIPipeByte {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIPipeByte {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Pull(this, ::core::mem::transmute_copy(&crequest)).into())
        }
        unsafe extern "system" fn Finish_Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Pull(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)).into())
        }
        unsafe extern "system" fn Begin_Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Push(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into())
        }
        unsafe extern "system" fn Finish_Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Push(this).into())
        }
        AsyncIPipeByte_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_Pull: Begin_Pull::<Identity, Impl, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, Impl, OFFSET>,
            Begin_Push: Begin_Push::<Identity, Impl, OFFSET>,
            Finish_Push: Finish_Push::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIPipeDouble_Impl: ::windows_core::BaseImpl {
    fn Begin_Pull(this: &Self::This, crequest: u32) -> ::windows_core::Result<()>;
    fn Finish_Pull(this: &Self::This, buf: *mut f64, pcreturned: *mut u32) -> ::windows_core::Result<()>;
    fn Begin_Push(this: &Self::This, buf: *const f64, csent: u32) -> ::windows_core::Result<()>;
    fn Finish_Push(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIPipeDouble {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIPipeDouble {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Pull(this, ::core::mem::transmute_copy(&crequest)).into())
        }
        unsafe extern "system" fn Finish_Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Pull(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)).into())
        }
        unsafe extern "system" fn Begin_Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Push(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into())
        }
        unsafe extern "system" fn Finish_Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Push(this).into())
        }
        AsyncIPipeDouble_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_Pull: Begin_Pull::<Identity, Impl, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, Impl, OFFSET>,
            Begin_Push: Begin_Push::<Identity, Impl, OFFSET>,
            Finish_Push: Finish_Push::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIPipeLong_Impl: ::windows_core::BaseImpl {
    fn Begin_Pull(this: &Self::This, crequest: u32) -> ::windows_core::Result<()>;
    fn Finish_Pull(this: &Self::This, buf: *mut i32, pcreturned: *mut u32) -> ::windows_core::Result<()>;
    fn Begin_Push(this: &Self::This, buf: *const i32, csent: u32) -> ::windows_core::Result<()>;
    fn Finish_Push(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIPipeLong {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIPipeLong {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Pull(this, ::core::mem::transmute_copy(&crequest)).into())
        }
        unsafe extern "system" fn Finish_Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Pull(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)).into())
        }
        unsafe extern "system" fn Begin_Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Push(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into())
        }
        unsafe extern "system" fn Finish_Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Push(this).into())
        }
        AsyncIPipeLong_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_Pull: Begin_Pull::<Identity, Impl, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, Impl, OFFSET>,
            Begin_Push: Begin_Push::<Identity, Impl, OFFSET>,
            Finish_Push: Finish_Push::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIUnknown_Impl: ::windows_core::BaseImpl {
    fn Begin_QueryInterface(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Finish_QueryInterface(this: &Self::This, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Begin_AddRef(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_AddRef(this: &Self::This) -> u32;
    fn Begin_Release(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_Release(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for AsyncIUnknown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIUnknown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_QueryInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_QueryInterface(this, ::core::mem::transmute_copy(&riid)).into())
        }
        unsafe extern "system" fn Finish_QueryInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_QueryInterface(this, ::core::mem::transmute_copy(&ppvobject)).into())
        }
        unsafe extern "system" fn Begin_AddRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_AddRef(this).into())
        }
        unsafe extern "system" fn Finish_AddRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_AddRef(this))
        }
        unsafe extern "system" fn Begin_Release<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Release(this).into())
        }
        unsafe extern "system" fn Finish_Release<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Release(this))
        }
        AsyncIUnknown_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_QueryInterface: Begin_QueryInterface::<Identity, Impl, OFFSET>,
            Finish_QueryInterface: Finish_QueryInterface::<Identity, Impl, OFFSET>,
            Begin_AddRef: Begin_AddRef::<Identity, Impl, OFFSET>,
            Finish_AddRef: Finish_AddRef::<Identity, Impl, OFFSET>,
            Begin_Release: Begin_Release::<Identity, Impl, OFFSET>,
            Finish_Release: Finish_Release::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IActivationFilter_Impl: ::windows_core::BaseImpl {
    fn HandleActivation(this: &Self::This, dwactivationtype: u32, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IActivationFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivationFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActivationFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivationFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows_core::GUID, preplacementclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HandleActivation(this, ::core::mem::transmute_copy(&dwactivationtype), ::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preplacementclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActivationFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleActivation: HandleActivation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAddrExclusionControl_Impl: ::windows_core::BaseImpl {
    fn GetCurrentAddrExclusionList(this: &Self::This, riid: *const ::windows_core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UpdateAddrExclusionList(this: &Self::This, penumerator: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAddrExclusionControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrExclusionControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAddrExclusionControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrExclusionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentAddrExclusionList(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppenumerator)).into())
        }
        unsafe extern "system" fn UpdateAddrExclusionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrExclusionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateAddrExclusionList(this, ::windows_core::from_raw_borrowed(&penumerator)).into())
        }
        IAddrExclusionControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentAddrExclusionList: GetCurrentAddrExclusionList::<Identity, Impl, OFFSET>,
            UpdateAddrExclusionList: UpdateAddrExclusionList::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAddrTrackingControl_Impl: ::windows_core::BaseImpl {
    fn EnableCOMDynamicAddrTracking(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableCOMDynamicAddrTracking(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAddrTrackingControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrTrackingControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAddrTrackingControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableCOMDynamicAddrTracking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrTrackingControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableCOMDynamicAddrTracking(this).into())
        }
        unsafe extern "system" fn DisableCOMDynamicAddrTracking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrTrackingControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableCOMDynamicAddrTracking(this).into())
        }
        IAddrTrackingControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableCOMDynamicAddrTracking: EnableCOMDynamicAddrTracking::<Identity, Impl, OFFSET>,
            DisableCOMDynamicAddrTracking: DisableCOMDynamicAddrTracking::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink_Impl: ::windows_core::BaseImpl {
    fn OnDataChange(this: &Self::This, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn OnViewChange(this: &Self::This, dwaspect: u32, lindex: i32);
    fn OnRename(this: &Self::This, pmk: ::core::option::Option<&IMoniker>);
    fn OnSave(this: &Self::This);
    fn OnClose(this: &Self::This);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IAdviseSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdviseSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDataChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataChange(this, ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pstgmed)))
        }
        unsafe extern "system" fn OnViewChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnViewChange(this, ::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex)))
        }
        unsafe extern "system" fn OnRename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRename(this, ::windows_core::from_raw_borrowed(&pmk)))
        }
        unsafe extern "system" fn OnSave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSave(this))
        }
        unsafe extern "system" fn OnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClose(this))
        }
        IAdviseSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDataChange: OnDataChange::<Identity, Impl, OFFSET>,
            OnViewChange: OnViewChange::<Identity, Impl, OFFSET>,
            OnRename: OnRename::<Identity, Impl, OFFSET>,
            OnSave: OnSave::<Identity, Impl, OFFSET>,
            OnClose: OnClose::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink2_Impl: ::windows_core::BaseImpl + IAdviseSink_Impl {
    fn OnLinkSrcChange(this: &Self::This, pmk: ::core::option::Option<&IMoniker>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IAdviseSink2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAdviseSink);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdviseSink2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLinkSrcChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLinkSrcChange(this, ::windows_core::from_raw_borrowed(&pmk)))
        }
        IAdviseSink2_Vtbl { base__: <IAdviseSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnLinkSrcChange: OnLinkSrcChange::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAgileObject_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IAgileObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAgileObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAgileObject {
    const VTABLE: Self::Vtable = { IAgileObject_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAsyncManager_Impl: ::windows_core::BaseImpl {
    fn CompleteCall(this: &Self::This, result: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetCallContext(this: &Self::This, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetState(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAsyncManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsyncManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompleteCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompleteCall(this, ::core::mem::transmute_copy(&result)).into())
        }
        unsafe extern "system" fn GetCallContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCallContext(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface)).into())
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstateflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAsyncManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CompleteCall: CompleteCall::<Identity, Impl, OFFSET>,
            GetCallContext: GetCallContext::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAsyncRpcChannelBuffer_Impl: ::windows_core::BaseImpl + IRpcChannelBuffer2_Impl {
    fn Send(this: &Self::This, pmsg: *mut RPCOLEMESSAGE, psync: ::core::option::Option<&ISynchronize>, pulstatus: *mut u32) -> ::windows_core::Result<()>;
    fn Receive(this: &Self::This, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::Result<()>;
    fn GetDestCtxEx(this: &Self::This, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAsyncRpcChannelBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRpcChannelBuffer2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsyncRpcChannelBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::core::mem::transmute_copy(&pmsg), ::windows_core::from_raw_borrowed(&psync), ::core::mem::transmute_copy(&pulstatus)).into())
        }
        unsafe extern "system" fn Receive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Receive(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pulstatus)).into())
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDestCtxEx(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)).into())
        }
        IAsyncRpcChannelBuffer_Vtbl {
            base__: <IRpcChannelBuffer2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Send: Send::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticate_Impl: ::windows_core::BaseImpl {
    fn Authenticate(this: &Self::This, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAuthenticate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAuthenticate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Authenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Authenticate(this, ::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into())
        }
        IAuthenticate_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Authenticate: Authenticate::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticateEx_Impl: ::windows_core::BaseImpl + IAuthenticate_Impl {
    fn AuthenticateEx(this: &Self::This, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAuthenticateEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAuthenticate);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticateEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAuthenticateEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AuthenticateEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticateEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows_core::PWSTR, pszpassword: *mut ::windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AuthenticateEx(this, ::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword), ::core::mem::transmute_copy(&pauthinfo)).into())
        }
        IAuthenticateEx_Vtbl { base__: <IAuthenticate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AuthenticateEx: AuthenticateEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBindCtx_Impl: ::windows_core::BaseImpl {
    fn RegisterObjectBound(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RevokeObjectBound(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ReleaseBoundObjects(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetBindOptions(this: &Self::This, pbindopts: *const BIND_OPTS) -> ::windows_core::Result<()>;
    fn GetBindOptions(this: &Self::This, pbindopts: *mut BIND_OPTS) -> ::windows_core::Result<()>;
    fn GetRunningObjectTable(this: &Self::This) -> ::windows_core::Result<IRunningObjectTable>;
    fn RegisterObjectParam(this: &Self::This, pszkey: &::windows_core::PCWSTR, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetObjectParam(this: &Self::This, pszkey: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn EnumObjectParam(this: &Self::This) -> ::windows_core::Result<IEnumString>;
    fn RevokeObjectParam(this: &Self::This, pszkey: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBindCtx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindCtx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterObjectBound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterObjectBound(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn RevokeObjectBound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeObjectBound(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn ReleaseBoundObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseBoundObjects(this).into())
        }
        unsafe extern "system" fn SetBindOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBindOptions(this, ::core::mem::transmute_copy(&pbindopts)).into())
        }
        unsafe extern "system" fn GetBindOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindOptions(this, ::core::mem::transmute_copy(&pbindopts)).into())
        }
        unsafe extern "system" fn GetRunningObjectTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRunningObjectTable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterObjectParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszkey: ::windows_core::PCWSTR, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterObjectParam(this, ::core::mem::transmute(&pszkey), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn GetObjectParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszkey: ::windows_core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectParam(this, ::core::mem::transmute(&pszkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumObjectParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumObjectParam(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RevokeObjectParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszkey: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeObjectParam(this, ::core::mem::transmute(&pszkey)).into())
        }
        IBindCtx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterObjectBound: RegisterObjectBound::<Identity, Impl, OFFSET>,
            RevokeObjectBound: RevokeObjectBound::<Identity, Impl, OFFSET>,
            ReleaseBoundObjects: ReleaseBoundObjects::<Identity, Impl, OFFSET>,
            SetBindOptions: SetBindOptions::<Identity, Impl, OFFSET>,
            GetBindOptions: GetBindOptions::<Identity, Impl, OFFSET>,
            GetRunningObjectTable: GetRunningObjectTable::<Identity, Impl, OFFSET>,
            RegisterObjectParam: RegisterObjectParam::<Identity, Impl, OFFSET>,
            GetObjectParam: GetObjectParam::<Identity, Impl, OFFSET>,
            EnumObjectParam: EnumObjectParam::<Identity, Impl, OFFSET>,
            RevokeObjectParam: RevokeObjectParam::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBindHost_Impl: ::windows_core::BaseImpl {
    fn CreateMoniker(this: &Self::This, szname: &::windows_core::PCWSTR, pbc: ::core::option::Option<&IBindCtx>, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows_core::Result<()>;
    fn MonikerBindToStorage(this: &Self::This, pmk: ::core::option::Option<&IMoniker>, pbc: ::core::option::Option<&IBindCtx>, pbsc: ::core::option::Option<&IBindStatusCallback>, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn MonikerBindToObject(this: &Self::This, pmk: ::core::option::Option<&IMoniker>, pbc: ::core::option::Option<&IBindCtx>, pbsc: ::core::option::Option<&IBindStatusCallback>, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBindHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, pbc: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateMoniker(this, ::core::mem::transmute(&szname), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&ppmk), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn MonikerBindToStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pbsc: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MonikerBindToStorage(this, ::windows_core::from_raw_borrowed(&pmk), ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pbsc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        unsafe extern "system" fn MonikerBindToObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pbsc: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MonikerBindToObject(this, ::windows_core::from_raw_borrowed(&pmk), ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pbsc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        IBindHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateMoniker: CreateMoniker::<Identity, Impl, OFFSET>,
            MonikerBindToStorage: MonikerBindToStorage::<Identity, Impl, OFFSET>,
            MonikerBindToObject: MonikerBindToObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallback_Impl: ::windows_core::BaseImpl {
    fn OnStartBinding(this: &Self::This, dwreserved: u32, pib: ::core::option::Option<&IBinding>) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn OnLowResource(this: &Self::This, reserved: u32) -> ::windows_core::Result<()>;
    fn OnProgress(this: &Self::This, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnStopBinding(this: &Self::This, hresult: ::windows_core::HRESULT, szerror: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetBindInfo(this: &Self::This, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows_core::Result<()>;
    fn OnDataAvailable(this: &Self::This, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows_core::Result<()>;
    fn OnObjectAvailable(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IBindStatusCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindStatusCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStartBinding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStartBinding(this, ::core::mem::transmute_copy(&dwreserved), ::windows_core::from_raw_borrowed(&pib)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnLowResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLowResource(this, ::core::mem::transmute_copy(&reserved)).into())
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax), ::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute(&szstatustext)).into())
        }
        unsafe extern "system" fn OnStopBinding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, szerror: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStopBinding(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&szerror)).into())
        }
        unsafe extern "system" fn GetBindInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindInfo(this, ::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo)).into())
        }
        unsafe extern "system" fn OnDataAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataAvailable(this, ::core::mem::transmute_copy(&grfbscf), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pstgmed)).into())
        }
        unsafe extern "system" fn OnObjectAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectAvailable(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        IBindStatusCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStartBinding: OnStartBinding::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            OnLowResource: OnLowResource::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnStopBinding: OnStopBinding::<Identity, Impl, OFFSET>,
            GetBindInfo: GetBindInfo::<Identity, Impl, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, Impl, OFFSET>,
            OnObjectAvailable: OnObjectAvailable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallbackEx_Impl: ::windows_core::BaseImpl + IBindStatusCallback_Impl {
    fn GetBindInfoEx(this: &Self::This, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IBindStatusCallbackEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBindStatusCallback);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallbackEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindStatusCallbackEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBindInfoEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindStatusCallbackEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindInfoEx(this, ::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        IBindStatusCallbackEx_Vtbl { base__: <IBindStatusCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBindInfoEx: GetBindInfoEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBinding_Impl: ::windows_core::BaseImpl {
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn Suspend(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPriority(this: &Self::This, npriority: i32) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetBindResult(this: &Self::This, pclsidprotocol: *mut ::windows_core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows_core::PWSTR, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBinding {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBinding {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn Suspend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Suspend(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&npriority)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBindResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows_core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows_core::PWSTR, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindResult(this, ::core::mem::transmute_copy(&pclsidprotocol), ::core::mem::transmute_copy(&pdwresult), ::core::mem::transmute_copy(&pszresult), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        IBinding_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Abort: Abort::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            GetBindResult: GetBindResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBlockingLock_Impl: ::windows_core::BaseImpl {
    fn Lock(this: &Self::This, dwtimeout: u32) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBlockingLock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockingLock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBlockingLock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockingLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&dwtimeout)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBlockingLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this).into())
        }
        IBlockingLock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICallFactory_Impl: ::windows_core::BaseImpl {
    fn CreateCall(this: &Self::This, riid: *const ::windows_core::GUID, pctrlunk: ::core::option::Option<&::windows_core::IUnknown>, riid2: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for ICallFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICallFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCall(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&pctrlunk), ::core::mem::transmute_copy(&riid2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICallFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateCall: CreateCall::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICancelMethodCalls_Impl: ::windows_core::BaseImpl {
    fn Cancel(this: &Self::This, ulseconds: u32) -> ::windows_core::Result<()>;
    fn TestCancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICancelMethodCalls {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICancelMethodCalls_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICancelMethodCalls {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICancelMethodCalls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this, ::core::mem::transmute_copy(&ulseconds)).into())
        }
        unsafe extern "system" fn TestCancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICancelMethodCalls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TestCancel(this).into())
        }
        ICancelMethodCalls_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            TestCancel: TestCancel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICatInformation_Impl: ::windows_core::BaseImpl {
    fn EnumCategories(this: &Self::This, lcid: u32) -> ::windows_core::Result<IEnumCATEGORYINFO>;
    fn GetCategoryDesc(this: &Self::This, rcatid: *const ::windows_core::GUID, lcid: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn EnumClassesOfCategories(this: &Self::This, cimplemented: u32, rgcatidimpl: *const ::windows_core::GUID, crequired: u32, rgcatidreq: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumGUID>;
    fn IsClassOfCategories(this: &Self::This, rclsid: *const ::windows_core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows_core::GUID, crequired: u32, rgcatidreq: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumImplCategoriesOfClass(this: &Self::This, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumGUID>;
    fn EnumReqCategoriesOfClass(this: &Self::This, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumGUID>;
}
impl ::windows_core::Iids for ICatInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICatInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCategories(this, ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcategoryinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCategoryDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows_core::GUID, lcid: u32, pszdesc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategoryDesc(this, ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumClassesOfCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows_core::GUID, crequired: u32, rgcatidreq: *const ::windows_core::GUID, ppenumclsid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumClassesOfCategories(this, ::core::mem::transmute_copy(&cimplemented), ::core::mem::transmute_copy(&rgcatidimpl), ::core::mem::transmute_copy(&crequired), ::core::mem::transmute_copy(&rgcatidreq)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsClassOfCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows_core::GUID, crequired: u32, rgcatidreq: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsClassOfCategories(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&cimplemented), ::core::mem::transmute_copy(&rgcatidimpl), ::core::mem::transmute_copy(&crequired), ::core::mem::transmute_copy(&rgcatidreq)).into())
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ppenumcatid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumImplCategoriesOfClass(this, ::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcatid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumReqCategoriesOfClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ppenumcatid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumReqCategoriesOfClass(this, ::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcatid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICatInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumCategories: EnumCategories::<Identity, Impl, OFFSET>,
            GetCategoryDesc: GetCategoryDesc::<Identity, Impl, OFFSET>,
            EnumClassesOfCategories: EnumClassesOfCategories::<Identity, Impl, OFFSET>,
            IsClassOfCategories: IsClassOfCategories::<Identity, Impl, OFFSET>,
            EnumImplCategoriesOfClass: EnumImplCategoriesOfClass::<Identity, Impl, OFFSET>,
            EnumReqCategoriesOfClass: EnumReqCategoriesOfClass::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICatRegister_Impl: ::windows_core::BaseImpl {
    fn RegisterCategories(this: &Self::This, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows_core::Result<()>;
    fn UnRegisterCategories(this: &Self::This, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RegisterClassImplCategories(this: &Self::This, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnRegisterClassImplCategories(this: &Self::This, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RegisterClassReqCategories(this: &Self::This, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnRegisterClassReqCategories(this: &Self::This, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICatRegister {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICatRegister {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterCategories(this, ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcategoryinfo)).into())
        }
        unsafe extern "system" fn UnRegisterCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnRegisterCategories(this, ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into())
        }
        unsafe extern "system" fn RegisterClassImplCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterClassImplCategories(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into())
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnRegisterClassImplCategories(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into())
        }
        unsafe extern "system" fn RegisterClassReqCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterClassReqCategories(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into())
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ccategories: u32, rgcatid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnRegisterClassReqCategories(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into())
        }
        ICatRegister_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterCategories: RegisterCategories::<Identity, Impl, OFFSET>,
            UnRegisterCategories: UnRegisterCategories::<Identity, Impl, OFFSET>,
            RegisterClassImplCategories: RegisterClassImplCategories::<Identity, Impl, OFFSET>,
            UnRegisterClassImplCategories: UnRegisterClassImplCategories::<Identity, Impl, OFFSET>,
            RegisterClassReqCategories: RegisterClassReqCategories::<Identity, Impl, OFFSET>,
            UnRegisterClassReqCategories: UnRegisterClassReqCategories::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IChannelHook_Impl: ::windows_core::BaseImpl {
    fn ClientGetSize(this: &Self::This, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32);
    fn ClientFillBuffer(this: &Self::This, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void);
    fn ClientNotify(this: &Self::This, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows_core::HRESULT);
    fn ServerNotify(this: &Self::This, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32);
    fn ServerGetSize(this: &Self::This, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, hrfault: ::windows_core::HRESULT, pdatasize: *mut u32);
    fn ServerFillBuffer(this: &Self::This, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows_core::HRESULT);
}
impl ::windows_core::Iids for IChannelHook {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChannelHook {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClientGetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClientGetSize(this, ::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdatasize)))
        }
        unsafe extern "system" fn ClientFillBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClientFillBuffer(this, ::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdatabuffer)))
        }
        unsafe extern "system" fn ClientNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClientNotify(this, ::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pdatabuffer), ::core::mem::transmute_copy(&ldatarep), ::core::mem::transmute_copy(&hrfault)))
        }
        unsafe extern "system" fn ServerNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ServerNotify(this, ::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pdatabuffer), ::core::mem::transmute_copy(&ldatarep)))
        }
        unsafe extern "system" fn ServerGetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, hrfault: ::windows_core::HRESULT, pdatasize: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ServerGetSize(this, ::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&hrfault), ::core::mem::transmute_copy(&pdatasize)))
        }
        unsafe extern "system" fn ServerFillBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ServerFillBuffer(this, ::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdatabuffer), ::core::mem::transmute_copy(&hrfault)))
        }
        IChannelHook_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClientGetSize: ClientGetSize::<Identity, Impl, OFFSET>,
            ClientFillBuffer: ClientFillBuffer::<Identity, Impl, OFFSET>,
            ClientNotify: ClientNotify::<Identity, Impl, OFFSET>,
            ServerNotify: ServerNotify::<Identity, Impl, OFFSET>,
            ServerGetSize: ServerGetSize::<Identity, Impl, OFFSET>,
            ServerFillBuffer: ServerFillBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IClassActivator_Impl: ::windows_core::BaseImpl {
    fn GetClassObject(this: &Self::This, rclsid: *const ::windows_core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IClassActivator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassActivator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClassActivator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassActivator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassObject(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwclasscontext), ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IClassActivator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetClassObject: GetClassObject::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IClassFactory_Impl: ::windows_core::BaseImpl {
    fn CreateInstance(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn LockServer(this: &Self::This, flock: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IClassFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClassFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstance(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        unsafe extern "system" fn LockServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockServer(this, ::core::mem::transmute_copy(&flock)).into())
        }
        IClassFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            LockServer: LockServer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IClientSecurity_Impl: ::windows_core::BaseImpl {
    fn QueryBlanket(this: &Self::This, pproxy: ::core::option::Option<&::windows_core::IUnknown>, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::Result<()>;
    fn SetBlanket(this: &Self::This, pproxy: ::core::option::Option<&::windows_core::IUnknown>, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: &::windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: &EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows_core::Result<()>;
    fn CopyProxy(this: &Self::This, pproxy: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IClientSecurity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClientSecurity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryBlanket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryBlanket(this, ::windows_core::from_raw_borrowed(&pproxy), ::core::mem::transmute_copy(&pauthnsvc), ::core::mem::transmute_copy(&pauthzsvc), ::core::mem::transmute_copy(&pserverprincname), ::core::mem::transmute_copy(&pauthnlevel), ::core::mem::transmute_copy(&pimplevel), ::core::mem::transmute_copy(&pauthinfo), ::core::mem::transmute_copy(&pcapabilites)).into())
        }
        unsafe extern "system" fn SetBlanket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows_core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlanket(this, ::windows_core::from_raw_borrowed(&pproxy), ::core::mem::transmute_copy(&dwauthnsvc), ::core::mem::transmute_copy(&dwauthzsvc), ::core::mem::transmute(&pserverprincname), ::core::mem::transmute_copy(&dwauthnlevel), ::core::mem::transmute_copy(&dwimplevel), ::core::mem::transmute_copy(&pauthinfo), ::core::mem::transmute(&dwcapabilities)).into())
        }
        unsafe extern "system" fn CopyProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyProxy(this, ::windows_core::from_raw_borrowed(&pproxy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcopy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IClientSecurity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryBlanket: QueryBlanket::<Identity, Impl, OFFSET>,
            SetBlanket: SetBlanket::<Identity, Impl, OFFSET>,
            CopyProxy: CopyProxy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IComThreadingInfo_Impl: ::windows_core::BaseImpl {
    fn GetCurrentApartmentType(this: &Self::This) -> ::windows_core::Result<APTTYPE>;
    fn GetCurrentThreadType(this: &Self::This) -> ::windows_core::Result<THDTYPE>;
    fn GetCurrentLogicalThreadId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetCurrentLogicalThreadId(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComThreadingInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComThreadingInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentApartmentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentApartmentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentThreadType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentThreadType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pthreadtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentLogicalThreadId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentLogicalThreadId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidlogicalthreadid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCurrentLogicalThreadId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentLogicalThreadId(this, ::core::mem::transmute_copy(&rguid)).into())
        }
        IComThreadingInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentApartmentType: GetCurrentApartmentType::<Identity, Impl, OFFSET>,
            GetCurrentThreadType: GetCurrentThreadType::<Identity, Impl, OFFSET>,
            GetCurrentLogicalThreadId: GetCurrentLogicalThreadId::<Identity, Impl, OFFSET>,
            SetCurrentLogicalThreadId: SetCurrentLogicalThreadId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IConnectionPoint_Impl: ::windows_core::BaseImpl {
    fn GetConnectionInterface(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetConnectionPointContainer(this: &Self::This) -> ::windows_core::Result<IConnectionPointContainer>;
    fn Advise(this: &Self::This, punksink: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn EnumConnections(this: &Self::This) -> ::windows_core::Result<IEnumConnections>;
}
impl ::windows_core::Iids for IConnectionPoint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConnectionPoint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnectionInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectionInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectionPointContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcpc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectionPointContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcpc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&punksink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn EnumConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IConnectionPoint_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnectionInterface: GetConnectionInterface::<Identity, Impl, OFFSET>,
            GetConnectionPointContainer: GetConnectionPointContainer::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumConnections: EnumConnections::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IConnectionPointContainer_Impl: ::windows_core::BaseImpl {
    fn EnumConnectionPoints(this: &Self::This) -> ::windows_core::Result<IEnumConnectionPoints>;
    fn FindConnectionPoint(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<IConnectionPoint>;
}
impl ::windows_core::Iids for IConnectionPointContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPointContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConnectionPointContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumConnectionPoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPointContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumConnectionPoints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindConnectionPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionPointContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppcp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindConnectionPoint(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IConnectionPointContainer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumConnectionPoints: EnumConnectionPoints::<Identity, Impl, OFFSET>,
            FindConnectionPoint: FindConnectionPoint::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContext_Impl: ::windows_core::BaseImpl {
    fn SetProperty(this: &Self::This, rpolicyid: *const ::windows_core::GUID, flags: u32, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RemoveProperty(this: &Self::This, rpolicyid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, rguid: *const ::windows_core::GUID, pflags: *mut u32, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumContextProps(this: &Self::This) -> ::windows_core::Result<IEnumContextProps>;
}
impl ::windows_core::Iids for IContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rpolicyid: *const ::windows_core::GUID, flags: u32, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&rpolicyid), ::core::mem::transmute_copy(&flags), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn RemoveProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rpolicyid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveProperty(this, ::core::mem::transmute_copy(&rpolicyid)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pflags: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pflags), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn EnumContextProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcontextprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumContextProps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcontextprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumContextProps: EnumContextProps::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContextCallback_Impl: ::windows_core::BaseImpl {
    fn ContextCallback(this: &Self::This, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows_core::GUID, imethod: i32, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IContextCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContextCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContextCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows_core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContextCallback(this, ::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&pparam), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&imethod), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        IContextCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContextCallback: ContextCallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDataAdviseHolder_Impl: ::windows_core::BaseImpl {
    fn Advise(this: &Self::This, pdataobject: ::core::option::Option<&IDataObject>, pfetc: *const FORMATETC, advf: u32, padvise: ::core::option::Option<&IAdviseSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumAdvise(this: &Self::This) -> ::windows_core::Result<IEnumSTATDATA>;
    fn SendOnDataChange(this: &Self::This, pdataobject: ::core::option::Option<&IDataObject>, dwreserved: u32, advf: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDataAdviseHolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataAdviseHolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, pfetc: *const FORMATETC, advf: u32, padvise: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&pfetc), ::core::mem::transmute_copy(&advf), ::windows_core::from_raw_borrowed(&padvise)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwconnection)).into())
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumAdvise(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendOnDataChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, dwreserved: u32, advf: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOnDataChange(this, ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&advf)).into())
        }
        IDataAdviseHolder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, Impl, OFFSET>,
            SendOnDataChange: SendOnDataChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDataObject_Impl: ::windows_core::BaseImpl {
    fn GetData(this: &Self::This, pformatetcin: *const FORMATETC) -> ::windows_core::Result<STGMEDIUM>;
    fn GetDataHere(this: &Self::This, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows_core::Result<()>;
    fn QueryGetData(this: &Self::This, pformatetc: *const FORMATETC) -> ::windows_core::HRESULT;
    fn GetCanonicalFormatEtc(this: &Self::This, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows_core::HRESULT;
    fn SetData(this: &Self::This, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnumFormatEtc(this: &Self::This, dwdirection: u32) -> ::windows_core::Result<IEnumFORMATETC>;
    fn DAdvise(this: &Self::This, pformatetc: *const FORMATETC, advf: u32, padvsink: ::core::option::Option<&IAdviseSink>) -> ::windows_core::Result<u32>;
    fn DUnadvise(this: &Self::This, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumDAdvise(this: &Self::This) -> ::windows_core::Result<IEnumSTATDATA>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IDataObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetData(this, ::core::mem::transmute_copy(&pformatetcin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmedium, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataHere<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataHere(this, ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium)).into())
        }
        unsafe extern "system" fn QueryGetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryGetData(this, ::core::mem::transmute_copy(&pformatetc)))
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCanonicalFormatEtc(this, ::core::mem::transmute_copy(&pformatectin), ::core::mem::transmute_copy(&pformatetcout)))
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium), ::core::mem::transmute_copy(&frelease)).into())
        }
        unsafe extern "system" fn EnumFormatEtc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFormatEtc(this, ::core::mem::transmute_copy(&dwdirection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumformatetc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DAdvise(this, ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&advf), ::windows_core::from_raw_borrowed(&padvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DUnadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DUnadvise(this, ::core::mem::transmute_copy(&dwconnection)).into())
        }
        unsafe extern "system" fn EnumDAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDAdvise(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDataObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataHere: GetDataHere::<Identity, Impl, OFFSET>,
            QueryGetData: QueryGetData::<Identity, Impl, OFFSET>,
            GetCanonicalFormatEtc: GetCanonicalFormatEtc::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            EnumFormatEtc: EnumFormatEtc::<Identity, Impl, OFFSET>,
            DAdvise: DAdvise::<Identity, Impl, OFFSET>,
            DUnadvise: DUnadvise::<Identity, Impl, OFFSET>,
            EnumDAdvise: EnumDAdvise::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDispatch_Impl: ::windows_core::BaseImpl {
    fn GetTypeInfoCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetTypeInfo(this: &Self::This, itinfo: u32, lcid: u32) -> ::windows_core::Result<ITypeInfo>;
    fn GetIDsOfNames(this: &Self::This, riid: *const ::windows_core::GUID, rgsznames: *const ::windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::Result<()>;
    fn Invoke(this: &Self::This, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDispatch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDispatch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTypeInfoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeInfoCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeInfo(this, ::core::mem::transmute_copy(&itinfo), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const ::windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIDsOfNames(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&rgdispid)).into())
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into())
        }
        IDispatch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTypeInfoCount: GetTypeInfoCount::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumCATEGORYINFO_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumCATEGORYINFO>;
}
impl ::windows_core::Iids for IEnumCATEGORYINFO {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumCATEGORYINFO {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumCATEGORYINFO_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumConnectionPoints_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cconnections: u32, ppcp: *mut ::core::option::Option<IConnectionPoint>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cconnections: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumConnectionPoints>;
}
impl ::windows_core::Iids for IEnumConnectionPoints {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumConnectionPoints {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cconnections), ::core::mem::transmute_copy(&ppcp), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cconnections)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumConnectionPoints_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumConnections_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cconnections: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumConnections>;
}
impl ::windows_core::Iids for IEnumConnections {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumConnections {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cconnections), ::core::mem::transmute_copy(&rgcd), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cconnections)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumConnections_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumContextProps_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pcontextproperties: *mut ContextProperty, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumContextProps>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumContextProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumContextProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumContextProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumContextProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pcontextproperties: *mut ContextProperty, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pcontextproperties), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumContextProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumContextProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumContextProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcontextprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcontextprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumContextProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumContextProps_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumFORMATETC_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumFORMATETC>;
}
impl ::windows_core::Iids for IEnumFORMATETC {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumFORMATETC {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumFORMATETC_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumGUID_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumGUID>;
}
impl ::windows_core::Iids for IEnumGUID {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumGUID {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumGUID_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumMoniker_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IMoniker>, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumMoniker>;
}
impl ::windows_core::Iids for IEnumMoniker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumMoniker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumMoniker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumSTATDATA_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSTATDATA>;
}
impl ::windows_core::Iids for IEnumSTATDATA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSTATDATA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSTATDATA_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumString_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::windows_core::PWSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumString>;
}
impl ::windows_core::Iids for IEnumString {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumString {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::PWSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumString_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumUnknown_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<::windows_core::IUnknown>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumUnknown>;
}
impl ::windows_core::Iids for IEnumUnknown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumUnknown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumUnknown_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IErrorInfo_Impl: ::windows_core::BaseImpl {
    fn GetGUID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetSource(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHelpFile(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHelpContext(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHelpFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHelpFile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhelpfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHelpContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHelpContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhelpcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IErrorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetHelpFile: GetHelpFile::<Identity, Impl, OFFSET>,
            GetHelpContext: GetHelpContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IErrorLog_Impl: ::windows_core::BaseImpl {
    fn AddError(this: &Self::This, pszpropname: &::windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IErrorLog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorLog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IErrorLog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows_core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddError(this, ::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&pexcepinfo)).into())
        }
        IErrorLog_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddError: AddError::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IExternalConnection_Impl: ::windows_core::BaseImpl {
    fn AddConnection(this: &Self::This, extconn: u32, reserved: u32) -> u32;
    fn ReleaseConnection(this: &Self::This, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IExternalConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExternalConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExternalConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExternalConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddConnection(this, ::core::mem::transmute_copy(&extconn), ::core::mem::transmute_copy(&reserved)))
        }
        unsafe extern "system" fn ReleaseConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExternalConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseConnection(this, ::core::mem::transmute_copy(&extconn), ::core::mem::transmute_copy(&reserved), ::core::mem::transmute_copy(&flastreleasecloses)))
        }
        IExternalConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddConnection: AddConnection::<Identity, Impl, OFFSET>,
            ReleaseConnection: ReleaseConnection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFastRundown_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IFastRundown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFastRundown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFastRundown {
    const VTABLE: Self::Vtable = { IFastRundown_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IForegroundTransfer_Impl: ::windows_core::BaseImpl {
    fn AllowForegroundTransfer(this: &Self::This, lpvreserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IForegroundTransfer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IForegroundTransfer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IForegroundTransfer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllowForegroundTransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IForegroundTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvreserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllowForegroundTransfer(this, ::core::mem::transmute_copy(&lpvreserved)).into())
        }
        IForegroundTransfer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllowForegroundTransfer: AllowForegroundTransfer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGlobalInterfaceTable_Impl: ::windows_core::BaseImpl {
    fn RegisterInterfaceInGlobal(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn RevokeInterfaceFromGlobal(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn GetInterfaceFromGlobal(this: &Self::This, dwcookie: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGlobalInterfaceTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGlobalInterfaceTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterInterfaceInGlobal(this, ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeInterfaceFromGlobal(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterfaceFromGlobal(this, ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IGlobalInterfaceTable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterInterfaceInGlobal: RegisterInterfaceInGlobal::<Identity, Impl, OFFSET>,
            RevokeInterfaceFromGlobal: RevokeInterfaceFromGlobal::<Identity, Impl, OFFSET>,
            GetInterfaceFromGlobal: GetInterfaceFromGlobal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGlobalOptions_Impl: ::windows_core::BaseImpl {
    fn Set(this: &Self::This, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::Result<()>;
    fn Query(this: &Self::This, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows_core::Result<usize>;
}
impl ::windows_core::Iids for IGlobalOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGlobalOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGlobalOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGlobalOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&dwvalue)).into())
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGlobalOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Query(this, ::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGlobalOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Set: Set::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInitializeSpy_Impl: ::windows_core::BaseImpl {
    fn PreInitialize(this: &Self::This, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows_core::Result<()>;
    fn PostInitialize(this: &Self::This, hrcoinit: ::windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows_core::Result<()>;
    fn PreUninitialize(this: &Self::This, dwcurthreadaptrefs: u32) -> ::windows_core::Result<()>;
    fn PostUninitialize(this: &Self::This, dwnewthreadaptrefs: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInitializeSpy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInitializeSpy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PreInitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreInitialize(this, ::core::mem::transmute_copy(&dwcoinit), ::core::mem::transmute_copy(&dwcurthreadaptrefs)).into())
        }
        unsafe extern "system" fn PostInitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrcoinit: ::windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostInitialize(this, ::core::mem::transmute_copy(&hrcoinit), ::core::mem::transmute_copy(&dwcoinit), ::core::mem::transmute_copy(&dwnewthreadaptrefs)).into())
        }
        unsafe extern "system" fn PreUninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreUninitialize(this, ::core::mem::transmute_copy(&dwcurthreadaptrefs)).into())
        }
        unsafe extern "system" fn PostUninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostUninitialize(this, ::core::mem::transmute_copy(&dwnewthreadaptrefs)).into())
        }
        IInitializeSpy_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PreInitialize: PreInitialize::<Identity, Impl, OFFSET>,
            PostInitialize: PostInitialize::<Identity, Impl, OFFSET>,
            PreUninitialize: PreUninitialize::<Identity, Impl, OFFSET>,
            PostUninitialize: PostUninitialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternalUnknown_Impl: ::windows_core::BaseImpl {
    fn QueryInternalInterface(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternalUnknown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternalUnknown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternalUnknown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryInternalInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternalUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryInternalInterface(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IInternalUnknown_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryInternalInterface: QueryInternalInterface::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMachineGlobalObjectTable_Impl: ::windows_core::BaseImpl {
    fn RegisterObject(this: &Self::This, clsid: *const ::windows_core::GUID, identifier: &::windows_core::PCWSTR, object: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<MachineGlobalObjectTableRegistrationToken>;
    fn GetObject(this: &Self::This, clsid: *const ::windows_core::GUID, identifier: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RevokeObject(this: &Self::This, token: MachineGlobalObjectTableRegistrationToken) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMachineGlobalObjectTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMachineGlobalObjectTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, identifier: ::windows_core::PCWSTR, object: *mut ::core::ffi::c_void, token: *mut MachineGlobalObjectTableRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterObject(this, ::core::mem::transmute_copy(&clsid), ::core::mem::transmute(&identifier), ::windows_core::from_raw_borrowed(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, identifier: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObject(this, ::core::mem::transmute_copy(&clsid), ::core::mem::transmute(&identifier), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn RevokeObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: MachineGlobalObjectTableRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeObject(this, ::core::mem::transmute_copy(&token)).into())
        }
        IMachineGlobalObjectTable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterObject: RegisterObject::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            RevokeObject: RevokeObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMalloc_Impl: ::windows_core::BaseImpl {
    fn Alloc(this: &Self::This, cb: usize) -> *mut ::core::ffi::c_void;
    fn Realloc(this: &Self::This, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
    fn Free(this: &Self::This, pv: *const ::core::ffi::c_void);
    fn GetSize(this: &Self::This, pv: *const ::core::ffi::c_void) -> usize;
    fn DidAlloc(this: &Self::This, pv: *const ::core::ffi::c_void) -> i32;
    fn HeapMinimize(this: &Self::This);
}
impl ::windows_core::Iids for IMalloc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMalloc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Alloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Alloc(this, ::core::mem::transmute_copy(&cb)))
        }
        unsafe extern "system" fn Realloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Realloc(this, ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)))
        }
        unsafe extern "system" fn Free<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Free(this, ::core::mem::transmute_copy(&pv)))
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSize(this, ::core::mem::transmute_copy(&pv)))
        }
        unsafe extern "system" fn DidAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DidAlloc(this, ::core::mem::transmute_copy(&pv)))
        }
        unsafe extern "system" fn HeapMinimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HeapMinimize(this))
        }
        IMalloc_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Alloc: Alloc::<Identity, Impl, OFFSET>,
            Realloc: Realloc::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            DidAlloc: DidAlloc::<Identity, Impl, OFFSET>,
            HeapMinimize: HeapMinimize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMallocSpy_Impl: ::windows_core::BaseImpl {
    fn PreAlloc(this: &Self::This, cbrequest: usize) -> usize;
    fn PostAlloc(this: &Self::This, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn PreFree(this: &Self::This, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PostFree(this: &Self::This, fspyed: super::super::Foundation::BOOL);
    fn PreRealloc(this: &Self::This, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize;
    fn PostRealloc(this: &Self::This, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PreGetSize(this: &Self::This, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PostGetSize(this: &Self::This, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize;
    fn PreDidAlloc(this: &Self::This, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PostDidAlloc(this: &Self::This, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32;
    fn PreHeapMinimize(this: &Self::This);
    fn PostHeapMinimize(this: &Self::This);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMallocSpy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMallocSpy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PreAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreAlloc(this, ::core::mem::transmute_copy(&cbrequest)))
        }
        unsafe extern "system" fn PostAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostAlloc(this, ::core::mem::transmute_copy(&pactual)))
        }
        unsafe extern "system" fn PreFree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreFree(this, ::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed)))
        }
        unsafe extern "system" fn PostFree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostFree(this, ::core::mem::transmute_copy(&fspyed)))
        }
        unsafe extern "system" fn PreRealloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreRealloc(this, ::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&cbrequest), ::core::mem::transmute_copy(&ppnewrequest), ::core::mem::transmute_copy(&fspyed)))
        }
        unsafe extern "system" fn PostRealloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostRealloc(this, ::core::mem::transmute_copy(&pactual), ::core::mem::transmute_copy(&fspyed)))
        }
        unsafe extern "system" fn PreGetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreGetSize(this, ::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed)))
        }
        unsafe extern "system" fn PostGetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostGetSize(this, ::core::mem::transmute_copy(&cbactual), ::core::mem::transmute_copy(&fspyed)))
        }
        unsafe extern "system" fn PreDidAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreDidAlloc(this, ::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed)))
        }
        unsafe extern "system" fn PostDidAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostDidAlloc(this, ::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed), ::core::mem::transmute_copy(&factual)))
        }
        unsafe extern "system" fn PreHeapMinimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreHeapMinimize(this))
        }
        unsafe extern "system" fn PostHeapMinimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostHeapMinimize(this))
        }
        IMallocSpy_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PreAlloc: PreAlloc::<Identity, Impl, OFFSET>,
            PostAlloc: PostAlloc::<Identity, Impl, OFFSET>,
            PreFree: PreFree::<Identity, Impl, OFFSET>,
            PostFree: PostFree::<Identity, Impl, OFFSET>,
            PreRealloc: PreRealloc::<Identity, Impl, OFFSET>,
            PostRealloc: PostRealloc::<Identity, Impl, OFFSET>,
            PreGetSize: PreGetSize::<Identity, Impl, OFFSET>,
            PostGetSize: PostGetSize::<Identity, Impl, OFFSET>,
            PreDidAlloc: PreDidAlloc::<Identity, Impl, OFFSET>,
            PostDidAlloc: PostDidAlloc::<Identity, Impl, OFFSET>,
            PreHeapMinimize: PreHeapMinimize::<Identity, Impl, OFFSET>,
            PostHeapMinimize: PostHeapMinimize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMoniker_Impl: ::windows_core::BaseImpl + IPersistStream_Impl {
    fn BindToObject(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>, pmktoleft: ::core::option::Option<&IMoniker>, riidresult: *const ::windows_core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn BindToStorage(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>, pmktoleft: ::core::option::Option<&IMoniker>, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Reduce(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows_core::Result<()>;
    fn ComposeWith(this: &Self::This, pmkright: ::core::option::Option<&IMoniker>, fonlyifnotgeneric: super::super::Foundation::BOOL) -> ::windows_core::Result<IMoniker>;
    fn Enum(this: &Self::This, fforward: super::super::Foundation::BOOL) -> ::windows_core::Result<IEnumMoniker>;
    fn IsEqual(this: &Self::This, pmkothermoniker: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<()>;
    fn Hash(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsRunning(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>, pmktoleft: ::core::option::Option<&IMoniker>, pmknewlyrunning: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<()>;
    fn GetTimeOfLastChange(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>, pmktoleft: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn Inverse(this: &Self::This) -> ::windows_core::Result<IMoniker>;
    fn CommonPrefixWith(this: &Self::This, pmkother: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<IMoniker>;
    fn RelativePathTo(this: &Self::This, pmkother: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<IMoniker>;
    fn GetDisplayName(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>, pmktoleft: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn ParseDisplayName(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>, pmktoleft: ::core::option::Option<&IMoniker>, pszdisplayname: &::windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows_core::Result<()>;
    fn IsSystemMoniker(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMoniker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPersistStream);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMoniker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindToObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, riidresult: *const ::windows_core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToObject(this, ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pmktoleft), ::core::mem::transmute_copy(&riidresult), ::core::mem::transmute_copy(&ppvresult)).into())
        }
        unsafe extern "system" fn BindToStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToStorage(this, ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pmktoleft), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        unsafe extern "system" fn Reduce<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, dwreducehowfar: u32, ppmktoleft: *mut *mut ::core::ffi::c_void, ppmkreduced: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reduce(this, ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&dwreducehowfar), ::core::mem::transmute_copy(&ppmktoleft), ::core::mem::transmute_copy(&ppmkreduced)).into())
        }
        unsafe extern "system" fn ComposeWith<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmkright: *mut ::core::ffi::c_void, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComposeWith(this, ::windows_core::from_raw_borrowed(&pmkright), ::core::mem::transmute_copy(&fonlyifnotgeneric)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmkcomposite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enum(this, ::core::mem::transmute_copy(&fforward)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenummoniker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmkothermoniker: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&pmkothermoniker)).into())
        }
        unsafe extern "system" fn Hash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hash(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhash, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pmknewlyrunning: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsRunning(this, ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pmktoleft), ::windows_core::from_raw_borrowed(&pmknewlyrunning)).into())
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimeOfLastChange(this, ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pmktoleft)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiletime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Inverse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Inverse(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommonPrefixWith<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmkother: *mut ::core::ffi::c_void, ppmkprefix: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommonPrefixWith(this, ::windows_core::from_raw_borrowed(&pmkother)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmkprefix, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RelativePathTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmkother: *mut ::core::ffi::c_void, ppmkrelpath: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RelativePathTo(this, ::windows_core::from_raw_borrowed(&pmkother)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmkrelpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, ppszdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayName(this, ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pmktoleft)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ParseDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pszdisplayname: ::windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseDisplayName(this, ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pmktoleft), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&ppmkout)).into())
        }
        unsafe extern "system" fn IsSystemMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSystemMoniker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmksys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMoniker_Vtbl {
            base__: <IPersistStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BindToObject: BindToObject::<Identity, Impl, OFFSET>,
            BindToStorage: BindToStorage::<Identity, Impl, OFFSET>,
            Reduce: Reduce::<Identity, Impl, OFFSET>,
            ComposeWith: ComposeWith::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Hash: Hash::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, Impl, OFFSET>,
            Inverse: Inverse::<Identity, Impl, OFFSET>,
            CommonPrefixWith: CommonPrefixWith::<Identity, Impl, OFFSET>,
            RelativePathTo: RelativePathTo::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            ParseDisplayName: ParseDisplayName::<Identity, Impl, OFFSET>,
            IsSystemMoniker: IsSystemMoniker::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMultiQI_Impl: ::windows_core::BaseImpl {
    fn QueryMultipleInterfaces(this: &Self::This, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMultiQI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiQI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultiQI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryMultipleInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiQI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryMultipleInterfaces(this, ::core::mem::transmute_copy(&cmqis), ::core::mem::transmute_copy(&pmqis)).into())
        }
        IMultiQI_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryMultipleInterfaces: QueryMultipleInterfaces::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INoMarshal_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for INoMarshal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INoMarshal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INoMarshal {
    const VTABLE: Self::Vtable = { INoMarshal_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IOplockStorage_Impl: ::windows_core::BaseImpl {
    fn CreateStorageEx(this: &Self::This, pwcsname: &::windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows_core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OpenStorageEx(this: &Self::This, pwcsname: &::windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows_core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IOplockStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOplockStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOplockStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateStorageEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOplockStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows_core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateStorageEx(this, ::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&stgfmt), ::core::mem::transmute_copy(&grfattrs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstgopen)).into())
        }
        unsafe extern "system" fn OpenStorageEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOplockStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows_core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenStorageEx(this, ::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&stgfmt), ::core::mem::transmute_copy(&grfattrs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstgopen)).into())
        }
        IOplockStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateStorageEx: CreateStorageEx::<Identity, Impl, OFFSET>,
            OpenStorageEx: OpenStorageEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPSFactoryBuffer_Impl: ::windows_core::BaseImpl {
    fn CreateProxy(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateStub(this: &Self::This, riid: *const ::windows_core::GUID, punkserver: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IRpcStubBuffer>;
}
impl ::windows_core::Iids for IPSFactoryBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPSFactoryBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPSFactoryBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPSFactoryBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppproxy: *mut *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateProxy(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppproxy), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateStub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPSFactoryBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStub(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punkserver)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPSFactoryBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateProxy: CreateProxy::<Identity, Impl, OFFSET>,
            CreateStub: CreateStub::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPersist_Impl: ::windows_core::BaseImpl {
    fn GetClassID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IPersist {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersist_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersist {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPersist_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetClassID: GetClassID::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistFile_Impl: ::windows_core::BaseImpl + IPersist_Impl {
    fn IsDirty(this: &Self::This) -> ::windows_core::HRESULT;
    fn Load(this: &Self::This, pszfilename: &::windows_core::PCWSTR, dwmode: STGM) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, pszfilename: &::windows_core::PCWSTR, fremember: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SaveCompleted(this: &Self::This, pszfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCurFile(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPersistFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPersist);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this))
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, dwmode: STGM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&dwmode)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, fremember: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&fremember)).into())
        }
        unsafe extern "system" fn SaveCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveCompleted(this, ::core::mem::transmute(&pszfilename)).into())
        }
        unsafe extern "system" fn GetCurFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszfilename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurFile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszfilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPersistFile_Vtbl {
            base__: <IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            GetCurFile: GetCurFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistMemory_Impl: ::windows_core::BaseImpl + IPersist_Impl {
    fn IsDirty(this: &Self::This) -> ::windows_core::HRESULT;
    fn Load(this: &Self::This, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows_core::Result<()>;
    fn GetSizeMax(this: &Self::This) -> ::windows_core::Result<u32>;
    fn InitNew(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPersistMemory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPersist);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistMemory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this))
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::core::mem::transmute_copy(&pmem), ::core::mem::transmute_copy(&cbsize)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::core::mem::transmute_copy(&pmem), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&cbsize)).into())
        }
        unsafe extern "system" fn GetSizeMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSizeMax(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this).into())
        }
        IPersistMemory_Vtbl {
            base__: <IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, Impl, OFFSET>,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStream_Impl: ::windows_core::BaseImpl + IPersist_Impl {
    fn IsDirty(this: &Self::This) -> ::windows_core::HRESULT;
    fn Load(this: &Self::This, pstm: ::core::option::Option<&IStream>) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, pstm: ::core::option::Option<&IStream>, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSizeMax(this: &Self::This) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPersistStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPersist);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this))
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&pstm)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&fcleardirty)).into())
        }
        unsafe extern "system" fn GetSizeMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSizeMax(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPersistStream_Vtbl {
            base__: <IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStreamInit_Impl: ::windows_core::BaseImpl + IPersist_Impl {
    fn IsDirty(this: &Self::This) -> ::windows_core::HRESULT;
    fn Load(this: &Self::This, pstm: ::core::option::Option<&IStream>) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, pstm: ::core::option::Option<&IStream>, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSizeMax(this: &Self::This) -> ::windows_core::Result<u64>;
    fn InitNew(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPersistStreamInit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPersist);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistStreamInit {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this))
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&pstm)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&fcleardirty)).into())
        }
        unsafe extern "system" fn GetSizeMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSizeMax(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this).into())
        }
        IPersistStreamInit_Vtbl {
            base__: <IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, Impl, OFFSET>,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPipeByte_Impl: ::windows_core::BaseImpl {
    fn Pull(this: &Self::This, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows_core::Result<()>;
    fn Push(this: &Self::This, buf: *const u8, csent: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPipeByte {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeByte_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPipeByte {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeByte_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pull(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&crequest), ::core::mem::transmute_copy(&pcreturned)).into())
        }
        unsafe extern "system" fn Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeByte_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Push(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into())
        }
        IPipeByte_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Pull: Pull::<Identity, Impl, OFFSET>,
            Push: Push::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPipeDouble_Impl: ::windows_core::BaseImpl {
    fn Pull(this: &Self::This, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows_core::Result<()>;
    fn Push(this: &Self::This, buf: *const f64, csent: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPipeDouble {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeDouble_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPipeDouble {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeDouble_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pull(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&crequest), ::core::mem::transmute_copy(&pcreturned)).into())
        }
        unsafe extern "system" fn Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeDouble_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Push(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into())
        }
        IPipeDouble_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Pull: Pull::<Identity, Impl, OFFSET>,
            Push: Push::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPipeLong_Impl: ::windows_core::BaseImpl {
    fn Pull(this: &Self::This, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows_core::Result<()>;
    fn Push(this: &Self::This, buf: *const i32, csent: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPipeLong {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeLong_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPipeLong {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Pull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeLong_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pull(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&crequest), ::core::mem::transmute_copy(&pcreturned)).into())
        }
        unsafe extern "system" fn Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPipeLong_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Push(this, ::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into())
        }
        IPipeLong_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Pull: Pull::<Identity, Impl, OFFSET>,
            Push: Push::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IProcessInitControl_Impl: ::windows_core::BaseImpl {
    fn ResetInitializerTimeout(this: &Self::This, dwsecondsremaining: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IProcessInitControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessInitControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProcessInitControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResetInitializerTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessInitControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetInitializerTimeout(this, ::core::mem::transmute_copy(&dwsecondsremaining)).into())
        }
        IProcessInitControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResetInitializerTimeout: ResetInitializerTimeout::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IProcessLock_Impl: ::windows_core::BaseImpl {
    fn AddRefOnProcess(this: &Self::This) -> u32;
    fn ReleaseRefOnProcess(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for IProcessLock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessLock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProcessLock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRefOnProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRefOnProcess(this))
        }
        unsafe extern "system" fn ReleaseRefOnProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseRefOnProcess(this))
        }
        IProcessLock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRefOnProcess: AddRefOnProcess::<Identity, Impl, OFFSET>,
            ReleaseRefOnProcess: ReleaseRefOnProcess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProgressNotify_Impl: ::windows_core::BaseImpl {
    fn OnProgress(this: &Self::This, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IProgressNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProgressNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProgressNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&dwprogresscurrent), ::core::mem::transmute_copy(&dwprogressmaximum), ::core::mem::transmute_copy(&faccurate), ::core::mem::transmute_copy(&fowner)).into())
        }
        IProgressNotify_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnProgress: OnProgress::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IROTData_Impl: ::windows_core::BaseImpl {
    fn GetComparisonData(this: &Self::This, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IROTData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IROTData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IROTData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetComparisonData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IROTData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetComparisonData(this, ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbmax), ::core::mem::transmute_copy(&pcbdata)).into())
        }
        IROTData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetComparisonData: GetComparisonData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IReleaseMarshalBuffers_Impl: ::windows_core::BaseImpl {
    fn ReleaseMarshalBuffer(this: &Self::This, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IReleaseMarshalBuffers {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReleaseMarshalBuffers_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReleaseMarshalBuffers {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReleaseMarshalBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReleaseMarshalBuffers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMarshalBuffer(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pchnl)).into())
        }
        IReleaseMarshalBuffers_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReleaseMarshalBuffer: ReleaseMarshalBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcChannelBuffer_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SendReceive(this: &Self::This, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows_core::Result<()>;
    fn FreeBuffer(this: &Self::This, pmessage: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()>;
    fn GetDestCtx(this: &Self::This, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRpcChannelBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcChannelBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&riid)).into())
        }
        unsafe extern "system" fn SendReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendReceive(this, ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeBuffer(this, ::core::mem::transmute_copy(&pmessage)).into())
        }
        unsafe extern "system" fn GetDestCtx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDestCtx(this, ::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)).into())
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsConnected(this).into())
        }
        IRpcChannelBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            SendReceive: SendReceive::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            GetDestCtx: GetDestCtx::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcChannelBuffer2_Impl: ::windows_core::BaseImpl + IRpcChannelBuffer_Impl {
    fn GetProtocolVersion(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IRpcChannelBuffer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRpcChannelBuffer);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcChannelBuffer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProtocolVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProtocolVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRpcChannelBuffer2_Vtbl {
            base__: <IRpcChannelBuffer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProtocolVersion: GetProtocolVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcChannelBuffer3_Impl: ::windows_core::BaseImpl + IRpcChannelBuffer2_Impl {
    fn Send(this: &Self::This, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::Result<()>;
    fn Receive(this: &Self::This, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()>;
    fn GetCallContext(this: &Self::This, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDestCtxEx(this: &Self::This, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetState(this: &Self::This, pmsg: *const RPCOLEMESSAGE) -> ::windows_core::Result<u32>;
    fn RegisterAsync(this: &Self::This, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::core::option::Option<&IAsyncManager>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRpcChannelBuffer3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRpcChannelBuffer2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcChannelBuffer3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pulstatus)).into())
        }
        unsafe extern "system" fn Receive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Receive(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pulstatus)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this, ::core::mem::transmute_copy(&pmsg)).into())
        }
        unsafe extern "system" fn GetCallContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows_core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCallContext(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface)).into())
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDestCtxEx(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)).into())
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this, ::core::mem::transmute_copy(&pmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterAsync(this, ::core::mem::transmute_copy(&pmsg), ::windows_core::from_raw_borrowed(&pasyncmgr)).into())
        }
        IRpcChannelBuffer3_Vtbl {
            base__: <IRpcChannelBuffer2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Send: Send::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetCallContext: GetCallContext::<Identity, Impl, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            RegisterAsync: RegisterAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcHelper_Impl: ::windows_core::BaseImpl {
    fn GetDCOMProtocolVersion(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetIIDFromOBJREF(this: &Self::This, pobjref: *const ::core::ffi::c_void) -> ::windows_core::Result<*mut ::windows_core::GUID>;
}
impl ::windows_core::Iids for IRpcHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDCOMProtocolVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDCOMProtocolVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIIDFromOBJREF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIIDFromOBJREF(this, ::core::mem::transmute_copy(&pobjref)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRpcHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDCOMProtocolVersion: GetDCOMProtocolVersion::<Identity, Impl, OFFSET>,
            GetIIDFromOBJREF: GetIIDFromOBJREF::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcOptions_Impl: ::windows_core::BaseImpl {
    fn Set(this: &Self::This, pprx: ::core::option::Option<&::windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::Result<()>;
    fn Query(this: &Self::This, pprx: ::core::option::Option<&::windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES) -> ::windows_core::Result<usize>;
}
impl ::windows_core::Iids for IRpcOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::windows_core::from_raw_borrowed(&pprx), ::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&dwvalue)).into())
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Query(this, ::windows_core::from_raw_borrowed(&pprx), ::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRpcOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Set: Set::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcProxyBuffer_Impl: ::windows_core::BaseImpl {
    fn Connect(this: &Self::This, prpcchannelbuffer: ::core::option::Option<&IRpcChannelBuffer>) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This);
}
impl ::windows_core::Iids for IRpcProxyBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcProxyBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcProxyBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcProxyBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prpcchannelbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::windows_core::from_raw_borrowed(&prpcchannelbuffer)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcProxyBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this))
        }
        IRpcProxyBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcStubBuffer_Impl: ::windows_core::BaseImpl {
    fn Connect(this: &Self::This, punkserver: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This);
    fn Invoke(this: &Self::This, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::core::option::Option<&IRpcChannelBuffer>) -> ::windows_core::Result<()>;
    fn IsIIDSupported(this: &Self::This, riid: *const ::windows_core::GUID) -> ::core::option::Option<IRpcStubBuffer>;
    fn CountRefs(this: &Self::This) -> u32;
    fn DebugServerQueryInterface(this: &Self::This, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DebugServerRelease(this: &Self::This, pv: *const ::core::ffi::c_void);
}
impl ::windows_core::Iids for IRpcStubBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcStubBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::windows_core::from_raw_borrowed(&punkserver)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this))
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::core::mem::transmute_copy(&_prpcmsg), ::windows_core::from_raw_borrowed(&_prpcchannelbuffer)).into())
        }
        unsafe extern "system" fn IsIIDSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsIIDSupported(this, ::core::mem::transmute_copy(&riid)))
        }
        unsafe extern "system" fn CountRefs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CountRefs(this))
        }
        unsafe extern "system" fn DebugServerQueryInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DebugServerQueryInterface(this, ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn DebugServerRelease<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DebugServerRelease(this, ::core::mem::transmute_copy(&pv)))
        }
        IRpcStubBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
            IsIIDSupported: IsIIDSupported::<Identity, Impl, OFFSET>,
            CountRefs: CountRefs::<Identity, Impl, OFFSET>,
            DebugServerQueryInterface: DebugServerQueryInterface::<Identity, Impl, OFFSET>,
            DebugServerRelease: DebugServerRelease::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRpcSyntaxNegotiate_Impl: ::windows_core::BaseImpl {
    fn NegotiateSyntax(this: &Self::This, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRpcSyntaxNegotiate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcSyntaxNegotiate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRpcSyntaxNegotiate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NegotiateSyntax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRpcSyntaxNegotiate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NegotiateSyntax(this, ::core::mem::transmute_copy(&pmsg)).into())
        }
        IRpcSyntaxNegotiate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NegotiateSyntax: NegotiateSyntax::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRunnableObject_Impl: ::windows_core::BaseImpl {
    fn GetRunningClass(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Run(this: &Self::This, pbc: ::core::option::Option<&IBindCtx>) -> ::windows_core::Result<()>;
    fn IsRunning(this: &Self::This) -> super::super::Foundation::BOOL;
    fn LockRunning(this: &Self::This, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetContainedObject(this: &Self::This, fcontained: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRunnableObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRunnableObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRunningClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRunningClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Run<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Run(this, ::windows_core::from_raw_borrowed(&pbc)).into())
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsRunning(this))
        }
        unsafe extern "system" fn LockRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockRunning(this, ::core::mem::transmute_copy(&flock), ::core::mem::transmute_copy(&flastunlockcloses)).into())
        }
        unsafe extern "system" fn SetContainedObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContainedObject(this, ::core::mem::transmute_copy(&fcontained)).into())
        }
        IRunnableObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRunningClass: GetRunningClass::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
            LockRunning: LockRunning::<Identity, Impl, OFFSET>,
            SetContainedObject: SetContainedObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRunningObjectTable_Impl: ::windows_core::BaseImpl {
    fn Register(this: &Self::This, grfflags: ROT_FLAGS, punkobject: ::core::option::Option<&::windows_core::IUnknown>, pmkobjectname: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<u32>;
    fn Revoke(this: &Self::This, dwregister: u32) -> ::windows_core::Result<()>;
    fn IsRunning(this: &Self::This, pmkobjectname: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<()>;
    fn GetObject(this: &Self::This, pmkobjectname: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn NoteChangeTime(this: &Self::This, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn GetTimeOfLastChange(this: &Self::This, pmkobjectname: ::core::option::Option<&IMoniker>) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn EnumRunning(this: &Self::This) -> ::windows_core::Result<IEnumMoniker>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRunningObjectTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRunningObjectTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Register<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfflags: ROT_FLAGS, punkobject: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, pdwregister: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Register(this, ::core::mem::transmute_copy(&grfflags), ::windows_core::from_raw_borrowed(&punkobject), ::windows_core::from_raw_borrowed(&pmkobjectname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwregister, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Revoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Revoke(this, ::core::mem::transmute_copy(&dwregister)).into())
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsRunning(this, ::windows_core::from_raw_borrowed(&pmkobjectname)).into())
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this, ::windows_core::from_raw_borrowed(&pmkobjectname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NoteChangeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NoteChangeTime(this, ::core::mem::transmute_copy(&dwregister), ::core::mem::transmute_copy(&pfiletime)).into())
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimeOfLastChange(this, ::windows_core::from_raw_borrowed(&pmkobjectname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiletime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenummoniker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRunning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenummoniker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRunningObjectTable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Register: Register::<Identity, Impl, OFFSET>,
            Revoke: Revoke::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            NoteChangeTime: NoteChangeTime::<Identity, Impl, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, Impl, OFFSET>,
            EnumRunning: EnumRunning::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISequentialStream_Impl: ::windows_core::BaseImpl {
    fn Read(this: &Self::This, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT;
    fn Write(this: &Self::This, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT;
}
impl ::windows_core::Iids for ISequentialStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISequentialStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISequentialStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISequentialStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)))
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISequentialStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbwritten)))
        }
        ISequentialStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IServerSecurity_Impl: ::windows_core::BaseImpl {
    fn QueryBlanket(this: &Self::This, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_core::Result<()>;
    fn ImpersonateClient(this: &Self::This) -> ::windows_core::Result<()>;
    fn RevertToSelf(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsImpersonating(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IServerSecurity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServerSecurity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryBlanket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryBlanket(this, ::core::mem::transmute_copy(&pauthnsvc), ::core::mem::transmute_copy(&pauthzsvc), ::core::mem::transmute_copy(&pserverprincname), ::core::mem::transmute_copy(&pauthnlevel), ::core::mem::transmute_copy(&pimplevel), ::core::mem::transmute_copy(&pprivs), ::core::mem::transmute_copy(&pcapabilities)).into())
        }
        unsafe extern "system" fn ImpersonateClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImpersonateClient(this).into())
        }
        unsafe extern "system" fn RevertToSelf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevertToSelf(this).into())
        }
        unsafe extern "system" fn IsImpersonating<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsImpersonating(this))
        }
        IServerSecurity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryBlanket: QueryBlanket::<Identity, Impl, OFFSET>,
            ImpersonateClient: ImpersonateClient::<Identity, Impl, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, Impl, OFFSET>,
            IsImpersonating: IsImpersonating::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IServiceProvider_Impl: ::windows_core::BaseImpl {
    fn QueryService(this: &Self::This, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryService(this, ::core::mem::transmute_copy(&guidservice), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        IServiceProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, QueryService: QueryService::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStdMarshalInfo_Impl: ::windows_core::BaseImpl {
    fn GetClassForHandler(this: &Self::This, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IStdMarshalInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStdMarshalInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStdMarshalInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassForHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStdMarshalInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassForHandler(this, ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStdMarshalInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClassForHandler: GetClassForHandler::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStream_Impl: ::windows_core::BaseImpl + ISequentialStream_Impl {
    fn Seek(this: &Self::This, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows_core::Result<()>;
    fn SetSize(this: &Self::This, libnewsize: u64) -> ::windows_core::Result<()>;
    fn CopyTo(this: &Self::This, pstm: ::core::option::Option<&IStream>, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, grfcommitflags: &STGC) -> ::windows_core::Result<()>;
    fn Revert(this: &Self::This) -> ::windows_core::Result<()>;
    fn LockRegion(this: &Self::This, liboffset: u64, cb: u64, dwlocktype: &LOCKTYPE) -> ::windows_core::Result<()>;
    fn UnlockRegion(this: &Self::This, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()>;
    fn Stat(this: &Self::This, pstatstg: *mut STATSTG, grfstatflag: &STATFLAG) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IStream>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISequentialStream);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin), ::core::mem::transmute_copy(&plibnewposition)).into())
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, ::core::mem::transmute_copy(&libnewsize)).into())
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTo(this, ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread), ::core::mem::transmute_copy(&pcbwritten)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute(&grfcommitflags)).into())
        }
        unsafe extern "system" fn Revert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Revert(this).into())
        }
        unsafe extern "system" fn LockRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockRegion(this, ::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute(&dwlocktype)).into())
        }
        unsafe extern "system" fn UnlockRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockRegion(this, ::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into())
        }
        unsafe extern "system" fn Stat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stat(this, ::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute(&grfstatflag)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStream_Vtbl {
            base__: <ISequentialStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            LockRegion: LockRegion::<Identity, Impl, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISupportAllowLowerTrustActivation_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ISupportAllowLowerTrustActivation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportAllowLowerTrustActivation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISupportAllowLowerTrustActivation {
    const VTABLE: Self::Vtable = { ISupportAllowLowerTrustActivation_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISupportErrorInfo_Impl: ::windows_core::BaseImpl {
    fn InterfaceSupportsErrorInfo(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISupportErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISupportErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InterfaceSupportsErrorInfo(this, ::core::mem::transmute_copy(&riid)).into())
        }
        ISupportErrorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InterfaceSupportsErrorInfo: InterfaceSupportsErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISurrogate_Impl: ::windows_core::BaseImpl {
    fn LoadDllServer(this: &Self::This, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn FreeSurrogate(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISurrogate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISurrogate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadDllServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadDllServer(this, ::core::mem::transmute_copy(&clsid)).into())
        }
        unsafe extern "system" fn FreeSurrogate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeSurrogate(this).into())
        }
        ISurrogate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadDllServer: LoadDllServer::<Identity, Impl, OFFSET>,
            FreeSurrogate: FreeSurrogate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISurrogateService_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, rguidprocessid: *const ::windows_core::GUID, pprocesslock: ::core::option::Option<&IProcessLock>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ApplicationLaunch(this: &Self::This, rguidapplid: *const ::windows_core::GUID, apptype: ApplicationType) -> ::windows_core::Result<()>;
    fn ApplicationFree(this: &Self::This, rguidapplid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn CatalogRefresh(this: &Self::This, ulreserved: u32) -> ::windows_core::Result<()>;
    fn ProcessShutdown(this: &Self::This, shutdowntype: ShutdownType) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISurrogateService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISurrogateService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows_core::GUID, pprocesslock: *mut ::core::ffi::c_void, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Init(this, ::core::mem::transmute_copy(&rguidprocessid), ::windows_core::from_raw_borrowed(&pprocesslock)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfapplicationaware, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplicationLaunch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows_core::GUID, apptype: ApplicationType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplicationLaunch(this, ::core::mem::transmute_copy(&rguidapplid), ::core::mem::transmute_copy(&apptype)).into())
        }
        unsafe extern "system" fn ApplicationFree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplicationFree(this, ::core::mem::transmute_copy(&rguidapplid)).into())
        }
        unsafe extern "system" fn CatalogRefresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CatalogRefresh(this, ::core::mem::transmute_copy(&ulreserved)).into())
        }
        unsafe extern "system" fn ProcessShutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessShutdown(this, ::core::mem::transmute_copy(&shutdowntype)).into())
        }
        ISurrogateService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            ApplicationLaunch: ApplicationLaunch::<Identity, Impl, OFFSET>,
            ApplicationFree: ApplicationFree::<Identity, Impl, OFFSET>,
            CatalogRefresh: CatalogRefresh::<Identity, Impl, OFFSET>,
            ProcessShutdown: ProcessShutdown::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISynchronize_Impl: ::windows_core::BaseImpl {
    fn Wait(this: &Self::This, dwflags: u32, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn Signal(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISynchronize {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISynchronize {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwmilliseconds)).into())
        }
        unsafe extern "system" fn Signal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Signal(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        ISynchronize_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Wait: Wait::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISynchronizeContainer_Impl: ::windows_core::BaseImpl {
    fn AddSynchronize(this: &Self::This, psync: ::core::option::Option<&ISynchronize>) -> ::windows_core::Result<()>;
    fn WaitMultiple(this: &Self::This, dwflags: u32, dwtimeout: u32) -> ::windows_core::Result<ISynchronize>;
}
impl ::windows_core::Iids for ISynchronizeContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISynchronizeContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddSynchronize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psync: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSynchronize(this, ::windows_core::from_raw_borrowed(&psync)).into())
        }
        unsafe extern "system" fn WaitMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitMultiple(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISynchronizeContainer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddSynchronize: AddSynchronize::<Identity, Impl, OFFSET>,
            WaitMultiple: WaitMultiple::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronizeEvent_Impl: ::windows_core::BaseImpl + ISynchronizeHandle_Impl {
    fn SetEventHandle(this: &Self::This, ph: *const super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISynchronizeEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISynchronizeHandle);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISynchronizeEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventHandle(this, ::core::mem::transmute_copy(&ph)).into())
        }
        ISynchronizeEvent_Vtbl { base__: <ISynchronizeHandle as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetEventHandle: SetEventHandle::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronizeHandle_Impl: ::windows_core::BaseImpl {
    fn GetHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISynchronizeHandle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeHandle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISynchronizeHandle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeHandle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ph, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISynchronizeHandle_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetHandle: GetHandle::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISynchronizeMutex_Impl: ::windows_core::BaseImpl + ISynchronize_Impl {
    fn ReleaseMutex(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISynchronizeMutex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISynchronize);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeMutex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISynchronizeMutex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReleaseMutex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizeMutex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMutex(this).into())
        }
        ISynchronizeMutex_Vtbl { base__: <ISynchronize as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ReleaseMutex: ReleaseMutex::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITimeAndNoticeControl_Impl: ::windows_core::BaseImpl {
    fn SuppressChanges(this: &Self::This, res1: u32, res2: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITimeAndNoticeControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimeAndNoticeControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITimeAndNoticeControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SuppressChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimeAndNoticeControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuppressChanges(this, ::core::mem::transmute_copy(&res1), ::core::mem::transmute_copy(&res2)).into())
        }
        ITimeAndNoticeControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SuppressChanges: SuppressChanges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeComp_Impl: ::windows_core::BaseImpl {
    fn Bind(this: &Self::This, szname: &::windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::core::option::Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows_core::Result<()>;
    fn BindType(this: &Self::This, szname: &::windows_core::PCWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, pptcomp: *mut ::core::option::Option<ITypeComp>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITypeComp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeComp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeComp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Bind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeComp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut *mut ::core::ffi::c_void, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bind(this, ::core::mem::transmute(&szname), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&pdesckind), ::core::mem::transmute_copy(&pbindptr)).into())
        }
        unsafe extern "system" fn BindType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeComp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, lhashval: u32, pptinfo: *mut *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindType(this, ::core::mem::transmute(&szname), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&pptcomp)).into())
        }
        ITypeComp_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Bind: Bind::<Identity, Impl, OFFSET>,
            BindType: BindType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeInfo_Impl: ::windows_core::BaseImpl {
    fn GetTypeAttr(this: &Self::This) -> ::windows_core::Result<*mut TYPEATTR>;
    fn GetTypeComp(this: &Self::This) -> ::windows_core::Result<ITypeComp>;
    fn GetFuncDesc(this: &Self::This, index: u32) -> ::windows_core::Result<*mut FUNCDESC>;
    fn GetVarDesc(this: &Self::This, index: u32) -> ::windows_core::Result<*mut VARDESC>;
    fn GetNames(this: &Self::This, memid: i32, rgbstrnames: *mut ::windows_core::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows_core::Result<()>;
    fn GetRefTypeOfImplType(this: &Self::This, index: u32) -> ::windows_core::Result<u32>;
    fn GetImplTypeFlags(this: &Self::This, index: u32) -> ::windows_core::Result<IMPLTYPEFLAGS>;
    fn GetIDsOfNames(this: &Self::This, rgsznames: *const ::windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> ::windows_core::Result<()>;
    fn Invoke(this: &Self::This, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()>;
    fn GetDocumentation(this: &Self::This, memid: i32, pbstrname: *mut ::windows_core::BSTR, pbstrdocstring: *mut ::windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDllEntry(this: &Self::This, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut ::windows_core::BSTR, pbstrname: *mut ::windows_core::BSTR, pwordinal: *mut u16) -> ::windows_core::Result<()>;
    fn GetRefTypeInfo(this: &Self::This, hreftype: u32) -> ::windows_core::Result<ITypeInfo>;
    fn AddressOfMember(this: &Self::This, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateInstance(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetMops(this: &Self::This, memid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetContainingTypeLib(this: &Self::This, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows_core::Result<()>;
    fn ReleaseTypeAttr(this: &Self::This, ptypeattr: *const TYPEATTR);
    fn ReleaseFuncDesc(this: &Self::This, pfuncdesc: *const FUNCDESC);
    fn ReleaseVarDesc(this: &Self::This, pvardesc: *const VARDESC);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITypeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTypeAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeAttr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypeattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeComp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeComp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptcomp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFuncDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFuncDesc(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfuncdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVarDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVarDesc(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvardesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, cmaxnames: u32, pcnames: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNames(this, ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&rgbstrnames), ::core::mem::transmute_copy(&cmaxnames), ::core::mem::transmute_copy(&pcnames)).into())
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRefTypeOfImplType(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preftype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImplTypeFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut IMPLTYPEFLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImplTypeFlags(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pimpltypeflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgsznames: *const ::windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIDsOfNames(this, ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&pmemid)).into())
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut super::Variant::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::core::mem::transmute_copy(&pvinstance), ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into())
        }
        unsafe extern "system" fn GetDocumentation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdocstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocumentation(this, ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrdocstring), ::core::mem::transmute_copy(&pdwhelpcontext), ::core::mem::transmute_copy(&pbstrhelpfile)).into())
        }
        unsafe extern "system" fn GetDllEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pwordinal: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDllEntry(this, ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind), ::core::mem::transmute_copy(&pbstrdllname), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pwordinal)).into())
        }
        unsafe extern "system" fn GetRefTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRefTypeInfo(this, ::core::mem::transmute_copy(&hreftype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddressOfMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddressOfMember(this, ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstance(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        unsafe extern "system" fn GetMops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMops(this, ::core::mem::transmute_copy(&memid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContainingTypeLib<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptlib: *mut *mut ::core::ffi::c_void, pindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContainingTypeLib(this, ::core::mem::transmute_copy(&pptlib), ::core::mem::transmute_copy(&pindex)).into())
        }
        unsafe extern "system" fn ReleaseTypeAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseTypeAttr(this, ::core::mem::transmute_copy(&ptypeattr)))
        }
        unsafe extern "system" fn ReleaseFuncDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseFuncDesc(this, ::core::mem::transmute_copy(&pfuncdesc)))
        }
        unsafe extern "system" fn ReleaseVarDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseVarDesc(this, ::core::mem::transmute_copy(&pvardesc)))
        }
        ITypeInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTypeAttr: GetTypeAttr::<Identity, Impl, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, Impl, OFFSET>,
            GetFuncDesc: GetFuncDesc::<Identity, Impl, OFFSET>,
            GetVarDesc: GetVarDesc::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            GetRefTypeOfImplType: GetRefTypeOfImplType::<Identity, Impl, OFFSET>,
            GetImplTypeFlags: GetImplTypeFlags::<Identity, Impl, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, Impl, OFFSET>,
            GetDllEntry: GetDllEntry::<Identity, Impl, OFFSET>,
            GetRefTypeInfo: GetRefTypeInfo::<Identity, Impl, OFFSET>,
            AddressOfMember: AddressOfMember::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            GetMops: GetMops::<Identity, Impl, OFFSET>,
            GetContainingTypeLib: GetContainingTypeLib::<Identity, Impl, OFFSET>,
            ReleaseTypeAttr: ReleaseTypeAttr::<Identity, Impl, OFFSET>,
            ReleaseFuncDesc: ReleaseFuncDesc::<Identity, Impl, OFFSET>,
            ReleaseVarDesc: ReleaseVarDesc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeInfo2_Impl: ::windows_core::BaseImpl + ITypeInfo_Impl {
    fn GetTypeKind(this: &Self::This) -> ::windows_core::Result<TYPEKIND>;
    fn GetTypeFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFuncIndexOfMemId(this: &Self::This, memid: i32, invkind: INVOKEKIND) -> ::windows_core::Result<u32>;
    fn GetVarIndexOfMemId(this: &Self::This, memid: i32) -> ::windows_core::Result<u32>;
    fn GetCustData(this: &Self::This, guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetFuncCustData(this: &Self::This, index: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetParamCustData(this: &Self::This, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetVarCustData(this: &Self::This, index: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetImplTypeCustData(this: &Self::This, index: u32, guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetDocumentation2(this: &Self::This, memid: i32, lcid: u32, pbstrhelpstring: *mut ::windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetAllCustData(this: &Self::This) -> ::windows_core::Result<CUSTDATA>;
    fn GetAllFuncCustData(this: &Self::This, index: u32) -> ::windows_core::Result<CUSTDATA>;
    fn GetAllParamCustData(this: &Self::This, indexfunc: u32, indexparam: u32) -> ::windows_core::Result<CUSTDATA>;
    fn GetAllVarCustData(this: &Self::This, index: u32) -> ::windows_core::Result<CUSTDATA>;
    fn GetAllImplTypeCustData(this: &Self::This, index: u32) -> ::windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITypeInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITypeInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTypeKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeKind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptypekind, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptypeflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFuncIndexOfMemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFuncIndexOfMemId(this, ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfuncindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVarIndexOfMemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVarIndexOfMemId(this, ::core::mem::transmute_copy(&memid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustData(this, ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFuncCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFuncCustData(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParamCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParamCustData(this, ::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVarCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVarCustData(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImplTypeCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImplTypeCustData(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocumentation2(this, ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrhelpstring), ::core::mem::transmute_copy(&pdwhelpstringcontext), ::core::mem::transmute_copy(&pbstrhelpstringdll)).into())
        }
        unsafe extern "system" fn GetAllCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllCustData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAllFuncCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllFuncCustData(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAllParamCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllParamCustData(this, ::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAllVarCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllVarCustData(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAllImplTypeCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllImplTypeCustData(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITypeInfo2_Vtbl {
            base__: <ITypeInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTypeKind: GetTypeKind::<Identity, Impl, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, Impl, OFFSET>,
            GetFuncIndexOfMemId: GetFuncIndexOfMemId::<Identity, Impl, OFFSET>,
            GetVarIndexOfMemId: GetVarIndexOfMemId::<Identity, Impl, OFFSET>,
            GetCustData: GetCustData::<Identity, Impl, OFFSET>,
            GetFuncCustData: GetFuncCustData::<Identity, Impl, OFFSET>,
            GetParamCustData: GetParamCustData::<Identity, Impl, OFFSET>,
            GetVarCustData: GetVarCustData::<Identity, Impl, OFFSET>,
            GetImplTypeCustData: GetImplTypeCustData::<Identity, Impl, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, Impl, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, Impl, OFFSET>,
            GetAllFuncCustData: GetAllFuncCustData::<Identity, Impl, OFFSET>,
            GetAllParamCustData: GetAllParamCustData::<Identity, Impl, OFFSET>,
            GetAllVarCustData: GetAllVarCustData::<Identity, Impl, OFFSET>,
            GetAllImplTypeCustData: GetAllImplTypeCustData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITypeLib_Impl: ::windows_core::BaseImpl {
    fn GetTypeInfoCount(this: &Self::This) -> u32;
    fn GetTypeInfo(this: &Self::This, index: u32) -> ::windows_core::Result<ITypeInfo>;
    fn GetTypeInfoType(this: &Self::This, index: u32) -> ::windows_core::Result<TYPEKIND>;
    fn GetTypeInfoOfGuid(this: &Self::This, guid: *const ::windows_core::GUID) -> ::windows_core::Result<ITypeInfo>;
    fn GetLibAttr(this: &Self::This) -> ::windows_core::Result<*mut TLIBATTR>;
    fn GetTypeComp(this: &Self::This) -> ::windows_core::Result<ITypeComp>;
    fn GetDocumentation(this: &Self::This, index: i32, pbstrname: *mut ::windows_core::BSTR, pbstrdocstring: *mut ::windows_core::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsName(this: &Self::This, sznamebuf: &::windows_core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FindName(this: &Self::This, sznamebuf: &::windows_core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows_core::Result<()>;
    fn ReleaseTLibAttr(this: &Self::This, ptlibattr: *const TLIBATTR);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITypeLib {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeLib {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTypeInfoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTypeInfoCount(this))
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeInfo(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeInfoType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeInfoType(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptkind, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeInfoOfGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeInfoOfGuid(this, ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLibAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLibAttr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptlibattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeComp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeComp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptcomp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdocstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocumentation(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrdocstring), ::core::mem::transmute_copy(&pdwhelpcontext), ::core::mem::transmute_copy(&pbstrhelpfile)).into())
        }
        unsafe extern "system" fn IsName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsName(this, ::core::mem::transmute(&sznamebuf), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&pfname)).into())
        }
        unsafe extern "system" fn FindName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sznamebuf: ::windows_core::PWSTR, lhashval: u32, pptinfo: *mut *mut ::core::ffi::c_void, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindName(this, ::core::mem::transmute(&sznamebuf), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&rgmemid), ::core::mem::transmute_copy(&pcfound)).into())
        }
        unsafe extern "system" fn ReleaseTLibAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseTLibAttr(this, ::core::mem::transmute_copy(&ptlibattr)))
        }
        ITypeLib_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTypeInfoCount: GetTypeInfoCount::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            GetTypeInfoType: GetTypeInfoType::<Identity, Impl, OFFSET>,
            GetTypeInfoOfGuid: GetTypeInfoOfGuid::<Identity, Impl, OFFSET>,
            GetLibAttr: GetLibAttr::<Identity, Impl, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, Impl, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, Impl, OFFSET>,
            IsName: IsName::<Identity, Impl, OFFSET>,
            FindName: FindName::<Identity, Impl, OFFSET>,
            ReleaseTLibAttr: ReleaseTLibAttr::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITypeLib2_Impl: ::windows_core::BaseImpl + ITypeLib_Impl {
    fn GetCustData(this: &Self::This, guid: *const ::windows_core::GUID) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetLibStatistics(this: &Self::This, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows_core::Result<()>;
    fn GetDocumentation2(this: &Self::This, index: i32, lcid: u32, pbstrhelpstring: *mut ::windows_core::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetAllCustData(this: &Self::This) -> ::windows_core::Result<CUSTDATA>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITypeLib2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITypeLib);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeLib2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustData(this, ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLibStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLibStatistics(this, ::core::mem::transmute_copy(&pcuniquenames), ::core::mem::transmute_copy(&pcchuniquenames)).into())
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocumentation2(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrhelpstring), ::core::mem::transmute_copy(&pdwhelpstringcontext), ::core::mem::transmute_copy(&pbstrhelpstringdll)).into())
        }
        unsafe extern "system" fn GetAllCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllCustData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITypeLib2_Vtbl {
            base__: <ITypeLib as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCustData: GetCustData::<Identity, Impl, OFFSET>,
            GetLibStatistics: GetLibStatistics::<Identity, Impl, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, Impl, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITypeLibRegistration_Impl: ::windows_core::BaseImpl {
    fn GetGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetVersion(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLcid(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetWin32Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetWin64Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetHelpDir(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITypeLibRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeLibRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLcid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLcid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWin32Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwin32path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWin32Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwin32path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWin64Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwin64path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWin64Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwin64path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHelpDir<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phelpdir: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHelpDir(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phelpdir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITypeLibRegistration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGuid: GetGuid::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetLcid: GetLcid::<Identity, Impl, OFFSET>,
            GetWin32Path: GetWin32Path::<Identity, Impl, OFFSET>,
            GetWin64Path: GetWin64Path::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetHelpDir: GetHelpDir::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITypeLibRegistrationReader_Impl: ::windows_core::BaseImpl {
    fn EnumTypeLibRegistrations(this: &Self::This) -> ::windows_core::Result<IEnumUnknown>;
}
impl ::windows_core::Iids for ITypeLibRegistrationReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistrationReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeLibRegistrationReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumTypeLibRegistrations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeLibRegistrationReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumTypeLibRegistrations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITypeLibRegistrationReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumTypeLibRegistrations: EnumTypeLibRegistrations::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUri_Impl: ::windows_core::BaseImpl {
    fn GetPropertyBSTR(this: &Self::This, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::windows_core::BSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetPropertyLength(this: &Self::This, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetPropertyDWORD(this: &Self::This, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn HasProperty(this: &Self::This, uriprop: Uri_PROPERTY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetAbsoluteUri(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAuthority(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDisplayUri(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDomain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetExtension(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFragment(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHost(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPassword(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPathAndQuery(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetQuery(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRawUri(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSchemeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUserInfo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHostType(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPort(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetScheme(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetZone(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetProperties(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsEqual(this: &Self::This, puri: ::core::option::Option<&IUri>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUri {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUri {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyBSTR<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyBSTR(this, ::core::mem::transmute_copy(&uriprop), ::core::mem::transmute_copy(&pbstrproperty), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetPropertyLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyLength(this, ::core::mem::transmute_copy(&uriprop), ::core::mem::transmute_copy(&pcchproperty), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetPropertyDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyDWORD(this, ::core::mem::transmute_copy(&uriprop), ::core::mem::transmute_copy(&pdwproperty), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn HasProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasProperty(this, ::core::mem::transmute_copy(&uriprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAbsoluteUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAbsoluteUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrabsoluteuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAuthority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrauthority: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAuthority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplaystring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrextension: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFragment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfragment: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFragment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfragment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrhost: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHost(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhost, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPassword(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpassword, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPathAndQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPathAndQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathandquery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrquery: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrquery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRawUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRawUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrawuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSchemeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrschemename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSchemeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrschemename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruserinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHostType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHostType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhosttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScheme<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScheme(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwscheme, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetZone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetZone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwzone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUri_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&puri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUri_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyBSTR: GetPropertyBSTR::<Identity, Impl, OFFSET>,
            GetPropertyLength: GetPropertyLength::<Identity, Impl, OFFSET>,
            GetPropertyDWORD: GetPropertyDWORD::<Identity, Impl, OFFSET>,
            HasProperty: HasProperty::<Identity, Impl, OFFSET>,
            GetAbsoluteUri: GetAbsoluteUri::<Identity, Impl, OFFSET>,
            GetAuthority: GetAuthority::<Identity, Impl, OFFSET>,
            GetDisplayUri: GetDisplayUri::<Identity, Impl, OFFSET>,
            GetDomain: GetDomain::<Identity, Impl, OFFSET>,
            GetExtension: GetExtension::<Identity, Impl, OFFSET>,
            GetFragment: GetFragment::<Identity, Impl, OFFSET>,
            GetHost: GetHost::<Identity, Impl, OFFSET>,
            GetPassword: GetPassword::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetPathAndQuery: GetPathAndQuery::<Identity, Impl, OFFSET>,
            GetQuery: GetQuery::<Identity, Impl, OFFSET>,
            GetRawUri: GetRawUri::<Identity, Impl, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, Impl, OFFSET>,
            GetUserInfo: GetUserInfo::<Identity, Impl, OFFSET>,
            GetUserName: GetUserName::<Identity, Impl, OFFSET>,
            GetHostType: GetHostType::<Identity, Impl, OFFSET>,
            GetPort: GetPort::<Identity, Impl, OFFSET>,
            GetScheme: GetScheme::<Identity, Impl, OFFSET>,
            GetZone: GetZone::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUriBuilder_Impl: ::windows_core::BaseImpl {
    fn CreateUriSimple(this: &Self::This, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows_core::Result<IUri>;
    fn CreateUri(this: &Self::This, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows_core::Result<IUri>;
    fn CreateUriWithFlags(this: &Self::This, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows_core::Result<IUri>;
    fn GetIUri(this: &Self::This) -> ::windows_core::Result<IUri>;
    fn SetIUri(this: &Self::This, piuri: ::core::option::Option<&IUri>) -> ::windows_core::Result<()>;
    fn GetFragment(this: &Self::This, pcchfragment: *mut u32, ppwzfragment: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetHost(this: &Self::This, pcchhost: *mut u32, ppwzhost: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPassword(this: &Self::This, pcchpassword: *mut u32, ppwzpassword: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPath(this: &Self::This, pcchpath: *mut u32, ppwzpath: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPort(this: &Self::This, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows_core::Result<()>;
    fn GetQuery(this: &Self::This, pcchquery: *mut u32, ppwzquery: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSchemeName(this: &Self::This, pcchschemename: *mut u32, ppwzschemename: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetUserName(this: &Self::This, pcchusername: *mut u32, ppwzusername: *mut ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFragment(this: &Self::This, pwznewvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHost(this: &Self::This, pwznewvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetPassword(this: &Self::This, pwznewvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetPath(this: &Self::This, pwznewvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetPort(this: &Self::This, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows_core::Result<()>;
    fn SetQuery(this: &Self::This, pwznewvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetSchemeName(this: &Self::This, pwznewvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetUserName(this: &Self::This, pwznewvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoveProperties(this: &Self::This, dwpropertymask: u32) -> ::windows_core::Result<()>;
    fn HasBeenModified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUriBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUriBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateUriSimple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUriSimple(this, ::core::mem::transmute_copy(&dwallowencodingpropertymask), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUri(this, ::core::mem::transmute_copy(&dwcreateflags), ::core::mem::transmute_copy(&dwallowencodingpropertymask), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateUriWithFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUriWithFlags(this, ::core::mem::transmute_copy(&dwcreateflags), ::core::mem::transmute_copy(&dwuribuilderflags), ::core::mem::transmute_copy(&dwallowencodingpropertymask), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piuri: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIUri(this, ::windows_core::from_raw_borrowed(&piuri)).into())
        }
        unsafe extern "system" fn GetFragment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFragment(this, ::core::mem::transmute_copy(&pcchfragment), ::core::mem::transmute_copy(&ppwzfragment)).into())
        }
        unsafe extern "system" fn GetHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHost(this, ::core::mem::transmute_copy(&pcchhost), ::core::mem::transmute_copy(&ppwzhost)).into())
        }
        unsafe extern "system" fn GetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPassword(this, ::core::mem::transmute_copy(&pcchpassword), ::core::mem::transmute_copy(&ppwzpassword)).into())
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPath(this, ::core::mem::transmute_copy(&pcchpath), ::core::mem::transmute_copy(&ppwzpath)).into())
        }
        unsafe extern "system" fn GetPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPort(this, ::core::mem::transmute_copy(&pfhasport), ::core::mem::transmute_copy(&pdwport)).into())
        }
        unsafe extern "system" fn GetQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuery(this, ::core::mem::transmute_copy(&pcchquery), ::core::mem::transmute_copy(&ppwzquery)).into())
        }
        unsafe extern "system" fn GetSchemeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSchemeName(this, ::core::mem::transmute_copy(&pcchschemename), ::core::mem::transmute_copy(&ppwzschemename)).into())
        }
        unsafe extern "system" fn GetUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserName(this, ::core::mem::transmute_copy(&pcchusername), ::core::mem::transmute_copy(&ppwzusername)).into())
        }
        unsafe extern "system" fn SetFragment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFragment(this, ::core::mem::transmute(&pwznewvalue)).into())
        }
        unsafe extern "system" fn SetHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHost(this, ::core::mem::transmute(&pwznewvalue)).into())
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassword(this, ::core::mem::transmute(&pwznewvalue)).into())
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&pwznewvalue)).into())
        }
        unsafe extern "system" fn SetPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPort(this, ::core::mem::transmute_copy(&fhasport), ::core::mem::transmute_copy(&dwnewvalue)).into())
        }
        unsafe extern "system" fn SetQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuery(this, ::core::mem::transmute(&pwznewvalue)).into())
        }
        unsafe extern "system" fn SetSchemeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSchemeName(this, ::core::mem::transmute(&pwznewvalue)).into())
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserName(this, ::core::mem::transmute(&pwznewvalue)).into())
        }
        unsafe extern "system" fn RemoveProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveProperties(this, ::core::mem::transmute_copy(&dwpropertymask)).into())
        }
        unsafe extern "system" fn HasBeenModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasBeenModified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUriBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateUriSimple: CreateUriSimple::<Identity, Impl, OFFSET>,
            CreateUri: CreateUri::<Identity, Impl, OFFSET>,
            CreateUriWithFlags: CreateUriWithFlags::<Identity, Impl, OFFSET>,
            GetIUri: GetIUri::<Identity, Impl, OFFSET>,
            SetIUri: SetIUri::<Identity, Impl, OFFSET>,
            GetFragment: GetFragment::<Identity, Impl, OFFSET>,
            GetHost: GetHost::<Identity, Impl, OFFSET>,
            GetPassword: GetPassword::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetPort: GetPort::<Identity, Impl, OFFSET>,
            GetQuery: GetQuery::<Identity, Impl, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, Impl, OFFSET>,
            GetUserName: GetUserName::<Identity, Impl, OFFSET>,
            SetFragment: SetFragment::<Identity, Impl, OFFSET>,
            SetHost: SetHost::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            SetQuery: SetQuery::<Identity, Impl, OFFSET>,
            SetSchemeName: SetSchemeName::<Identity, Impl, OFFSET>,
            SetUserName: SetUserName::<Identity, Impl, OFFSET>,
            RemoveProperties: RemoveProperties::<Identity, Impl, OFFSET>,
            HasBeenModified: HasBeenModified::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUrlMon_Impl: ::windows_core::BaseImpl {
    fn AsyncGetClassBits(this: &Self::This, rclsid: *const ::windows_core::GUID, psztype: &::windows_core::PCWSTR, pszext: &::windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: &::windows_core::PCWSTR, pbc: ::core::option::Option<&IBindCtx>, dwclasscontext: u32, riid: *const ::windows_core::GUID, flags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUrlMon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlMon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlMon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncGetClassBits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlMon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, psztype: ::windows_core::PCWSTR, pszext: ::windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: ::windows_core::PCWSTR, pbc: *mut ::core::ffi::c_void, dwclasscontext: u32, riid: *const ::windows_core::GUID, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncGetClassBits(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&psztype), ::core::mem::transmute(&pszext), ::core::mem::transmute_copy(&dwfileversionms), ::core::mem::transmute_copy(&dwfileversionls), ::core::mem::transmute(&pszcodebase), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&dwclasscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&flags)).into())
        }
        IUrlMon_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncGetClassBits: AsyncGetClassBits::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWaitMultiple_Impl: ::windows_core::BaseImpl {
    fn WaitMultiple(this: &Self::This, timeout: u32) -> ::windows_core::Result<ISynchronize>;
    fn AddSynchronize(this: &Self::This, psync: ::core::option::Option<&ISynchronize>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWaitMultiple {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWaitMultiple_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWaitMultiple {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WaitMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWaitMultiple_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitMultiple(this, ::core::mem::transmute_copy(&timeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddSynchronize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWaitMultiple_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psync: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSynchronize(this, ::windows_core::from_raw_borrowed(&psync)).into())
        }
        IWaitMultiple_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WaitMultiple: WaitMultiple::<Identity, Impl, OFFSET>,
            AddSynchronize: AddSynchronize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
