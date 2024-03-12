#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter_Impl: ::windows_core::BaseImpl + IDXGIObject_Impl {
    fn EnumOutputs(this: &Self::This, output: u32) -> ::windows_core::Result<IDXGIOutput>;
    fn GetDesc(this: &Self::This, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::Result<()>;
    fn CheckInterfaceSupport(this: &Self::This, interfacename: *const ::windows_core::GUID) -> ::windows_core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIAdapter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIObject);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIAdapter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumOutputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumOutputs(this, ::core::mem::transmute_copy(&output)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfacename: *const ::windows_core::GUID, pumdversion: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckInterfaceSupport(this, ::core::mem::transmute_copy(&interfacename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pumdversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIAdapter_Vtbl {
            base__: <IDXGIObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumOutputs: EnumOutputs::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter1_Impl: ::windows_core::BaseImpl + IDXGIAdapter_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIAdapter1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIAdapter);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIAdapter1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        IDXGIAdapter1_Vtbl { base__: <IDXGIAdapter as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter2_Impl: ::windows_core::BaseImpl + IDXGIAdapter1_Impl {
    fn GetDesc2(this: &Self::This, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIAdapter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIAdapter1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIAdapter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc2(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        IDXGIAdapter2_Vtbl { base__: <IDXGIAdapter1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc2: GetDesc2::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter3_Impl: ::windows_core::BaseImpl + IDXGIAdapter2_Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(this: &Self::This, dwcookie: u32);
    fn QueryVideoMemoryInfo(this: &Self::This, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows_core::Result<()>;
    fn SetVideoMemoryReservation(this: &Self::This, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows_core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(this: &Self::This, dwcookie: u32);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIAdapter3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIAdapter2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIAdapter3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterHardwareContentProtectionTeardownStatusEvent(this, ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterHardwareContentProtectionTeardownStatus(this, ::core::mem::transmute_copy(&dwcookie)))
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryVideoMemoryInfo(this, ::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup), ::core::mem::transmute_copy(&pvideomemoryinfo)).into())
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVideoMemoryReservation(this, ::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup), ::core::mem::transmute_copy(&reservation)).into())
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterVideoMemoryBudgetChangeNotificationEvent(this, ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterVideoMemoryBudgetChangeNotification(this, ::core::mem::transmute_copy(&dwcookie)))
        }
        IDXGIAdapter3_Vtbl {
            base__: <IDXGIAdapter2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, Impl, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, Impl, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, Impl, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, Impl, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter4_Impl: ::windows_core::BaseImpl + IDXGIAdapter3_Impl {
    fn GetDesc3(this: &Self::This, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIAdapter4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIAdapter3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIAdapter4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIAdapter4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc3(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        IDXGIAdapter4_Vtbl { base__: <IDXGIAdapter3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc3: GetDesc3::<Identity, Impl, OFFSET> }
    };
}
pub trait IDXGIDebug_Impl: ::windows_core::BaseImpl {
    fn ReportLiveObjects(this: &Self::This, apiid: &::windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDXGIDebug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDebug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDebug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportLiveObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, apiid: ::windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportLiveObjects(this, ::core::mem::transmute(&apiid), ::core::mem::transmute_copy(&flags)).into())
        }
        IDXGIDebug_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportLiveObjects: ReportLiveObjects::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDebug1_Impl: ::windows_core::BaseImpl + IDXGIDebug_Impl {
    fn EnableLeakTrackingForThread(this: &Self::This);
    fn DisableLeakTrackingForThread(this: &Self::This);
    fn IsLeakTrackingEnabledForThread(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIDebug1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDebug);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDebug1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableLeakTrackingForThread(this))
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableLeakTrackingForThread(this))
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLeakTrackingEnabledForThread(this))
        }
        IDXGIDebug1_Vtbl {
            base__: <IDXGIDebug as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDecodeSwapChain_Impl: ::windows_core::BaseImpl {
    fn PresentBuffer(this: &Self::This, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT;
    fn SetSourceRect(this: &Self::This, prect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetTargetRect(this: &Self::This, prect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetDestSize(this: &Self::This, width: u32, height: u32) -> ::windows_core::Result<()>;
    fn GetSourceRect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetTargetRect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetDestSize(this: &Self::This, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::Result<()>;
    fn SetColorSpace(this: &Self::This, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows_core::Result<()>;
    fn GetColorSpace(this: &Self::This) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIDecodeSwapChain {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDecodeSwapChain {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PresentBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PresentBuffer(this, ::core::mem::transmute_copy(&buffertopresent), ::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn SetSourceRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSourceRect(this, ::core::mem::transmute_copy(&prect)).into())
        }
        unsafe extern "system" fn SetTargetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetRect(this, ::core::mem::transmute_copy(&prect)).into())
        }
        unsafe extern "system" fn SetDestSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestSize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn GetSourceRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDestSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDestSize(this, ::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into())
        }
        unsafe extern "system" fn SetColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorSpace(this, ::core::mem::transmute_copy(&colorspace)).into())
        }
        unsafe extern "system" fn GetColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorSpace(this))
        }
        IDXGIDecodeSwapChain_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PresentBuffer: PresentBuffer::<Identity, Impl, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, Impl, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, Impl, OFFSET>,
            SetDestSize: SetDestSize::<Identity, Impl, OFFSET>,
            GetSourceRect: GetSourceRect::<Identity, Impl, OFFSET>,
            GetTargetRect: GetTargetRect::<Identity, Impl, OFFSET>,
            GetDestSize: GetDestSize::<Identity, Impl, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, Impl, OFFSET>,
            GetColorSpace: GetColorSpace::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice_Impl: ::windows_core::BaseImpl + IDXGIObject_Impl {
    fn GetAdapter(this: &Self::This) -> ::windows_core::Result<IDXGIAdapter>;
    fn CreateSurface(this: &Self::This, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::core::option::Option<IDXGISurface>) -> ::windows_core::Result<()>;
    fn QueryResourceResidency(this: &Self::This, ppresources: *const ::core::option::Option<::windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::Result<()>;
    fn SetGPUThreadPriority(this: &Self::This, priority: i32) -> ::windows_core::Result<()>;
    fn GetGPUThreadPriority(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(padapter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSurface(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numsurfaces), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&psharedresource), ::core::mem::transmute_copy(&ppsurface)).into())
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryResourceResidency(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&presidencystatus), ::core::mem::transmute_copy(&numresources)).into())
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGPUThreadPriority(this, ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGPUThreadPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIDevice_Vtbl {
            base__: <IDXGIObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAdapter: GetAdapter::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, Impl, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, Impl, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice1_Impl: ::windows_core::BaseImpl + IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(this: &Self::This, maxlatency: u32) -> ::windows_core::Result<()>;
    fn GetMaximumFrameLatency(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIDevice1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDevice);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDevice1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumFrameLatency(this, ::core::mem::transmute_copy(&maxlatency)).into())
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumFrameLatency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxlatency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIDevice1_Vtbl {
            base__: <IDXGIDevice as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice2_Impl: ::windows_core::BaseImpl + IDXGIDevice1_Impl {
    fn OfferResources(this: &Self::This, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows_core::Result<()>;
    fn ReclaimResources(this: &Self::This, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnqueueSetEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIDevice2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDevice1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDevice2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OfferResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OfferResources(this, ::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn ReclaimResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReclaimResources(this, ::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&pdiscarded)).into())
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnqueueSetEvent(this, ::core::mem::transmute_copy(&hevent)).into())
        }
        IDXGIDevice2_Vtbl {
            base__: <IDXGIDevice1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OfferResources: OfferResources::<Identity, Impl, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, Impl, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice3_Impl: ::windows_core::BaseImpl + IDXGIDevice2_Impl {
    fn Trim(this: &Self::This);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIDevice3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDevice2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDevice3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Trim<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Trim(this))
        }
        IDXGIDevice3_Vtbl { base__: <IDXGIDevice2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Trim: Trim::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice4_Impl: ::windows_core::BaseImpl + IDXGIDevice3_Impl {
    fn OfferResources1(this: &Self::This, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows_core::Result<()>;
    fn ReclaimResources1(this: &Self::This, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIDevice4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDevice3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDevice4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OfferResources1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OfferResources1(this, ::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn ReclaimResources1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReclaimResources1(this, ::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&presults)).into())
        }
        IDXGIDevice4_Vtbl {
            base__: <IDXGIDevice3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OfferResources1: OfferResources1::<Identity, Impl, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDXGIDeviceSubObject_Impl: ::windows_core::BaseImpl + IDXGIObject_Impl {
    fn GetDevice(this: &Self::This, riid: *const ::windows_core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDXGIDeviceSubObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIObject);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDeviceSubObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevice(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdevice)).into())
        }
        IDXGIDeviceSubObject_Vtbl { base__: <IDXGIObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDevice: GetDevice::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDisplayControl_Impl: ::windows_core::BaseImpl {
    fn IsStereoEnabled(this: &Self::This) -> super::super::Foundation::BOOL;
    fn SetStereoEnabled(this: &Self::This, enabled: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIDisplayControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIDisplayControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsStereoEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsStereoEnabled(this))
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStereoEnabled(this, ::core::mem::transmute_copy(&enabled)))
        }
        IDXGIDisplayControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsStereoEnabled: IsStereoEnabled::<Identity, Impl, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory_Impl: ::windows_core::BaseImpl + IDXGIObject_Impl {
    fn EnumAdapters(this: &Self::This, adapter: u32) -> ::windows_core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(this: &Self::This, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows_core::Result<()>;
    fn GetWindowAssociation(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn CreateSwapChain(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows_core::HRESULT;
    fn CreateSoftwareAdapter(this: &Self::This, module: super::super::Foundation::HMODULE) -> ::windows_core::Result<IDXGIAdapter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumAdapters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumAdapters(this, ::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadapter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeWindowAssociation(this, ::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindowAssociation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwindowhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSwapChain(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppswapchain)))
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HMODULE, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSoftwareAdapter(this, ::core::mem::transmute_copy(&module)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadapter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIFactory_Vtbl {
            base__: <IDXGIObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumAdapters: EnumAdapters::<Identity, Impl, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, Impl, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, Impl, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, Impl, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory1_Impl: ::windows_core::BaseImpl + IDXGIFactory_Impl {
    fn EnumAdapters1(this: &Self::This, adapter: u32) -> ::windows_core::Result<IDXGIAdapter1>;
    fn IsCurrent(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumAdapters1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumAdapters1(this, ::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadapter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsCurrent(this))
        }
        IDXGIFactory1_Vtbl {
            base__: <IDXGIFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumAdapters1: EnumAdapters1::<Identity, Impl, OFFSET>,
            IsCurrent: IsCurrent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory2_Impl: ::windows_core::BaseImpl + IDXGIFactory1_Impl {
    fn IsWindowedStereoEnabled(this: &Self::This) -> super::super::Foundation::BOOL;
    fn CreateSwapChainForHwnd(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows_core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, pwindow: ::core::option::Option<&::windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows_core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(this: &Self::This, hresource: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::LUID>;
    fn RegisterStereoStatusWindow(this: &Self::This, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows_core::Result<u32>;
    fn RegisterStereoStatusEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
    fn UnregisterStereoStatus(this: &Self::This, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(this: &Self::This, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows_core::Result<u32>;
    fn RegisterOcclusionStatusEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
    fn UnregisterOcclusionStatus(this: &Self::This, dwcookie: u32);
    fn CreateSwapChainForComposition(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows_core::Result<IDXGISwapChain1>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIFactory1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsWindowedStereoEnabled(this))
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSwapChainForHwnd(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pfullscreendesc), ::windows_core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSwapChainForCoreWindow(this, ::windows_core::from_raw_borrowed(&pdevice), ::windows_core::from_raw_borrowed(&pwindow), ::core::mem::transmute_copy(&pdesc), ::windows_core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSharedResourceAdapterLuid(this, ::core::mem::transmute_copy(&hresource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pluid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterStereoStatusWindow(this, ::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterStereoStatusEvent(this, ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterStereoStatus(this, ::core::mem::transmute_copy(&dwcookie)))
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterOcclusionStatusWindow(this, ::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterOcclusionStatusEvent(this, ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterOcclusionStatus(this, ::core::mem::transmute_copy(&dwcookie)))
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSwapChainForComposition(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pdesc), ::windows_core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIFactory2_Vtbl {
            base__: <IDXGIFactory1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Identity, Impl, OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Identity, Impl, OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Identity, Impl, OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Identity, Impl, OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Identity, Impl, OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Identity, Impl, OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Identity, Impl, OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Identity, Impl, OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory3_Impl: ::windows_core::BaseImpl + IDXGIFactory2_Impl {
    fn GetCreationFlags(this: &Self::This) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIFactory2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationFlags(this))
        }
        IDXGIFactory3_Vtbl { base__: <IDXGIFactory2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory4_Impl: ::windows_core::BaseImpl + IDXGIFactory3_Impl {
    fn EnumAdapterByLuid(this: &Self::This, adapterluid: &super::super::Foundation::LUID, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnumWarpAdapter(this: &Self::This, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIFactory3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAdapterByLuid(this, ::core::mem::transmute(&adapterluid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into())
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumWarpAdapter(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into())
        }
        IDXGIFactory4_Vtbl {
            base__: <IDXGIFactory3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, Impl, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory5_Impl: ::windows_core::BaseImpl + IDXGIFactory4_Impl {
    fn CheckFeatureSupport(this: &Self::This, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIFactory4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckFeatureSupport(this, ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into())
        }
        IDXGIFactory5_Vtbl { base__: <IDXGIFactory4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory6_Impl: ::windows_core::BaseImpl + IDXGIFactory5_Impl {
    fn EnumAdapterByGpuPreference(this: &Self::This, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIFactory5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAdapterByGpuPreference(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&gpupreference), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into())
        }
        IDXGIFactory6_Vtbl {
            base__: <IDXGIFactory5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory7_Impl: ::windows_core::BaseImpl + IDXGIFactory6_Impl {
    fn RegisterAdaptersChangedEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactory7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIFactory6);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactory7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterAdaptersChangedEvent(this, ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterAdaptersChangedEvent(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        IDXGIFactory7_Vtbl {
            base__: <IDXGIFactory6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactoryMedia_Impl: ::windows_core::BaseImpl {
    fn CreateSwapChainForCompositionSurfaceHandle(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows_core::Result<IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::core::option::Option<&IDXGIResource>, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows_core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIFactoryMedia {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIFactoryMedia {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSwapChainForCompositionSurfaceHandle(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::windows_core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut ::core::ffi::c_void, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDecodeSwapChainForCompositionSurfaceHandle(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::windows_core::from_raw_borrowed(&pyuvdecodebuffers), ::windows_core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIFactoryMedia_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIInfoQueue_Impl: ::windows_core::BaseImpl {
    fn SetMessageCountLimit(this: &Self::This, producer: &::windows_core::GUID, messagecountlimit: u64) -> ::windows_core::Result<()>;
    fn ClearStoredMessages(this: &Self::This, producer: &::windows_core::GUID);
    fn GetMessage(this: &Self::This, producer: &::windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(this: &Self::This, producer: &::windows_core::GUID) -> u64;
    fn GetNumStoredMessages(this: &Self::This, producer: &::windows_core::GUID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(this: &Self::This, producer: &::windows_core::GUID) -> u64;
    fn GetMessageCountLimit(this: &Self::This, producer: &::windows_core::GUID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(this: &Self::This, producer: &::windows_core::GUID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(this: &Self::This, producer: &::windows_core::GUID) -> u64;
    fn AddStorageFilterEntries(this: &Self::This, producer: &::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetStorageFilter(this: &Self::This, producer: &::windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearStorageFilter(this: &Self::This, producer: &::windows_core::GUID);
    fn PushEmptyStorageFilter(this: &Self::This, producer: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PushDenyAllStorageFilter(this: &Self::This, producer: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PushCopyOfStorageFilter(this: &Self::This, producer: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PushStorageFilter(this: &Self::This, producer: &::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopStorageFilter(this: &Self::This, producer: &::windows_core::GUID);
    fn GetStorageFilterStackSize(this: &Self::This, producer: &::windows_core::GUID) -> u32;
    fn AddRetrievalFilterEntries(this: &Self::This, producer: &::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetRetrievalFilter(this: &Self::This, producer: &::windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearRetrievalFilter(this: &Self::This, producer: &::windows_core::GUID);
    fn PushEmptyRetrievalFilter(this: &Self::This, producer: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PushDenyAllRetrievalFilter(this: &Self::This, producer: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(this: &Self::This, producer: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PushRetrievalFilter(this: &Self::This, producer: &::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopRetrievalFilter(this: &Self::This, producer: &::windows_core::GUID);
    fn GetRetrievalFilterStackSize(this: &Self::This, producer: &::windows_core::GUID) -> u32;
    fn AddMessage(this: &Self::This, producer: &::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn AddApplicationMessage(this: &Self::This, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetBreakOnCategory(this: &Self::This, producer: &::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnSeverity(this: &Self::This, producer: &::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnID(this: &Self::This, producer: &::windows_core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBreakOnCategory(this: &Self::This, producer: &::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(this: &Self::This, producer: &::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(this: &Self::This, producer: &::windows_core::GUID, id: i32) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(this: &Self::This, producer: &::windows_core::GUID, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(this: &Self::This, producer: &::windows_core::GUID) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIInfoQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIInfoQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, messagecountlimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageCountLimit(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&messagecountlimit)).into())
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStoredMessages(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessage(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into())
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessagesAllowedByRetrievalFilters(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessages(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDiscardedByMessageCountLimit(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessageCountLimit(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesAllowedByStorageFilter(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDeniedByStorageFilter(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStorageFilterEntries(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilter(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStorageFilter(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyStorageFilter(this, ::core::mem::transmute(&producer)).into())
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushDenyAllStorageFilter(this, ::core::mem::transmute(&producer)).into())
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfStorageFilter(this, ::core::mem::transmute(&producer)).into())
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushStorageFilter(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopStorageFilter(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilterStackSize(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRetrievalFilterEntries(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilter(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRetrievalFilter(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyRetrievalFilter(this, ::core::mem::transmute(&producer)).into())
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushDenyAllRetrievalFilter(this, ::core::mem::transmute(&producer)).into())
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfRetrievalFilter(this, ::core::mem::transmute(&producer)).into())
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushRetrievalFilter(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopRetrievalFilter(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilterStackSize(this, ::core::mem::transmute(&producer)))
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMessage(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddApplicationMessage(this, ::core::mem::transmute_copy(&severity), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnCategory(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnSeverity(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnID(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnCategory(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&category)))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnSeverity(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&severity)))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, id: i32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnID(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&id)))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID, bmute: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMuteDebugOutput(this, ::core::mem::transmute(&producer), ::core::mem::transmute_copy(&bmute)))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, producer: ::windows_core::GUID) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMuteDebugOutput(this, ::core::mem::transmute(&producer)))
        }
        IDXGIInfoQueue_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDXGIKeyedMutex_Impl: ::windows_core::BaseImpl + IDXGIDeviceSubObject_Impl {
    fn AcquireSync(this: &Self::This, key: u64, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn ReleaseSync(this: &Self::This, key: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDXGIKeyedMutex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDeviceSubObject);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIKeyedMutex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcquireSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireSync(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&dwmilliseconds)).into())
        }
        unsafe extern "system" fn ReleaseSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseSync(this, ::core::mem::transmute_copy(&key)).into())
        }
        IDXGIKeyedMutex_Vtbl {
            base__: <IDXGIDeviceSubObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcquireSync: AcquireSync::<Identity, Impl, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDXGIObject_Impl: ::windows_core::BaseImpl {
    fn SetPrivateData(this: &Self::This, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, name: *const ::windows_core::GUID, punknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetPrivateData(this: &Self::This, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetParent(this: &Self::This, riid: *const ::windows_core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDXGIObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *const ::windows_core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&name), ::windows_core::from_raw_borrowed(&punknown)).into())
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParent(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparent)).into())
        }
        IDXGIObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput_Impl: ::windows_core::BaseImpl + IDXGIObject_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::Result<()>;
    fn GetDisplayModeList(this: &Self::This, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows_core::Result<()>;
    fn FindClosestMatchingMode(this: &Self::This, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn WaitForVBlank(this: &Self::This) -> ::windows_core::Result<()>;
    fn TakeOwnership(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, exclusive: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ReleaseOwnership(this: &Self::This);
    fn GetGammaControlCapabilities(this: &Self::This, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::Result<()>;
    fn SetGammaControl(this: &Self::This, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()>;
    fn GetGammaControl(this: &Self::This, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::Result<()>;
    fn SetDisplaySurface(this: &Self::This, pscanoutsurface: ::core::option::Option<&IDXGISurface>) -> ::windows_core::Result<()>;
    fn GetDisplaySurfaceData(this: &Self::This, pdestination: ::core::option::Option<&IDXGISurface>) -> ::windows_core::Result<()>;
    fn GetFrameStatistics(this: &Self::This, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGIOutput {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutput {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayModeList(this, ::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindClosestMatchingMode(this, ::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::windows_core::from_raw_borrowed(&pconcerneddevice)).into())
        }
        unsafe extern "system" fn WaitForVBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForVBlank(this).into())
        }
        unsafe extern "system" fn TakeOwnership<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TakeOwnership(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&exclusive)).into())
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseOwnership(this))
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGammaControlCapabilities(this, ::core::mem::transmute_copy(&pgammacaps)).into())
        }
        unsafe extern "system" fn SetGammaControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGammaControl(this, ::core::mem::transmute_copy(&parray)).into())
        }
        unsafe extern "system" fn GetGammaControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGammaControl(this, ::core::mem::transmute_copy(&parray)).into())
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscanoutsurface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplaySurface(this, ::windows_core::from_raw_borrowed(&pscanoutsurface)).into())
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplaySurfaceData(this, ::windows_core::from_raw_borrowed(&pdestination)).into())
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameStatistics(this, ::core::mem::transmute_copy(&pstats)).into())
        }
        IDXGIOutput_Vtbl {
            base__: <IDXGIObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, Impl, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, Impl, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, Impl, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, Impl, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, Impl, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, Impl, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, Impl, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1_Impl: ::windows_core::BaseImpl + IDXGIOutput_Impl {
    fn GetDisplayModeList1(this: &Self::This, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows_core::Result<()>;
    fn FindClosestMatchingMode1(this: &Self::This, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetDisplaySurfaceData1(this: &Self::This, pdestination: ::core::option::Option<&IDXGIResource>) -> ::windows_core::Result<()>;
    fn DuplicateOutput(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGIOutput1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIOutput);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutput1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDisplayModeList1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayModeList1(this, ::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindClosestMatchingMode1(this, ::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::windows_core::from_raw_borrowed(&pconcerneddevice)).into())
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplaySurfaceData1(this, ::windows_core::from_raw_borrowed(&pdestination)).into())
        }
        unsafe extern "system" fn DuplicateOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DuplicateOutput(this, ::windows_core::from_raw_borrowed(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutputduplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIOutput1_Vtbl {
            base__: <IDXGIOutput as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDisplayModeList1: GetDisplayModeList1::<Identity, Impl, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, Impl, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, Impl, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput2_Impl: ::windows_core::BaseImpl + IDXGIOutput1_Impl {
    fn SupportsOverlays(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGIOutput2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIOutput1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutput2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SupportsOverlays<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SupportsOverlays(this))
        }
        IDXGIOutput2_Vtbl { base__: <IDXGIOutput1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SupportsOverlays: SupportsOverlays::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput3_Impl: ::windows_core::BaseImpl + IDXGIOutput2_Impl {
    fn CheckOverlaySupport(this: &Self::This, enumformat: Common::DXGI_FORMAT, pconcerneddevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGIOutput3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIOutput2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutput3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckOverlaySupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckOverlaySupport(this, ::core::mem::transmute_copy(&enumformat), ::windows_core::from_raw_borrowed(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIOutput3_Vtbl { base__: <IDXGIOutput2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CheckOverlaySupport: CheckOverlaySupport::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput4_Impl: ::windows_core::BaseImpl + IDXGIOutput3_Impl {
    fn CheckOverlayColorSpaceSupport(this: &Self::This, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGIOutput4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIOutput3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutput4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckOverlayColorSpaceSupport(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&colorspace), ::windows_core::from_raw_borrowed(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIOutput4_Vtbl {
            base__: <IDXGIOutput3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput5_Impl: ::windows_core::BaseImpl + IDXGIOutput4_Impl {
    fn DuplicateOutput1(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT) -> ::windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGIOutput5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIOutput4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutput5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DuplicateOutput1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DuplicateOutput1(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&supportedformatscount), ::core::mem::transmute_copy(&psupportedformats)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutputduplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIOutput5_Vtbl { base__: <IDXGIOutput4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DuplicateOutput1: DuplicateOutput1::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6_Impl: ::windows_core::BaseImpl + IDXGIOutput5_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows_core::Result<()>;
    fn CheckHardwareCompositionSupport(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGIOutput6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIOutput5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutput6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckHardwareCompositionSupport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIOutput6_Vtbl {
            base__: <IDXGIOutput5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc1: GetDesc1::<Identity, Impl, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIOutputDuplication_Impl: ::windows_core::BaseImpl + IDXGIObject_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut DXGI_OUTDUPL_DESC);
    fn AcquireNextFrame(this: &Self::This, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::core::option::Option<IDXGIResource>) -> ::windows_core::Result<()>;
    fn GetFrameDirtyRects(this: &Self::This, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows_core::Result<()>;
    fn GetFrameMoveRects(this: &Self::This, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows_core::Result<()>;
    fn GetFramePointerShape(this: &Self::This, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows_core::Result<()>;
    fn MapDesktopSurface(this: &Self::This) -> ::windows_core::Result<DXGI_MAPPED_RECT>;
    fn UnMapDesktopSurface(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReleaseFrame(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGIOutputDuplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIOutputDuplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireNextFrame(this, ::core::mem::transmute_copy(&timeoutinmilliseconds), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&ppdesktopresource)).into())
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameDirtyRects(this, ::core::mem::transmute_copy(&dirtyrectsbuffersize), ::core::mem::transmute_copy(&pdirtyrectsbuffer), ::core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into())
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameMoveRects(this, ::core::mem::transmute_copy(&moverectsbuffersize), ::core::mem::transmute_copy(&pmoverectbuffer), ::core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into())
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFramePointerShape(this, ::core::mem::transmute_copy(&pointershapebuffersize), ::core::mem::transmute_copy(&ppointershapebuffer), ::core::mem::transmute_copy(&ppointershapebuffersizerequired), ::core::mem::transmute_copy(&ppointershapeinfo)).into())
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapDesktopSurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plockedrect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnMapDesktopSurface(this).into())
        }
        unsafe extern "system" fn ReleaseFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseFrame(this).into())
        }
        IDXGIOutputDuplication_Vtbl {
            base__: <IDXGIObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Identity, Impl, OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Identity, Impl, OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Identity, Impl, OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Identity, Impl, OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Identity, Impl, OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Identity, Impl, OFFSET>,
            ReleaseFrame: ReleaseFrame::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIResource_Impl: ::windows_core::BaseImpl + IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetUsage(this: &Self::This) -> ::windows_core::Result<DXGI_USAGE>;
    fn SetEvictionPriority(this: &Self::This, evictionpriority: u32) -> ::windows_core::Result<()>;
    fn GetEvictionPriority(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXGIResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDeviceSubObject);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSharedHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSharedHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psharedhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUsage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusage: *mut DXGI_USAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUsage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pusage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEvictionPriority(this, ::core::mem::transmute_copy(&evictionpriority)).into())
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEvictionPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevictionpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIResource_Vtbl {
            base__: <IDXGIDeviceSubObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSharedHandle: GetSharedHandle::<Identity, Impl, OFFSET>,
            GetUsage: GetUsage::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDXGIResource1_Impl: ::windows_core::BaseImpl + IDXGIResource_Impl {
    fn CreateSubresourceSurface(this: &Self::This, index: u32) -> ::windows_core::Result<IDXGISurface2>;
    fn CreateSharedHandle(this: &Self::This, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::Iids for IDXGIResource1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIResource);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGIResource1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSubresourceSurface(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSharedHandle(this, ::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&dwaccess), ::core::mem::transmute(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGIResource1_Vtbl {
            base__: <IDXGIResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, Impl, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurface_Impl: ::windows_core::BaseImpl + IDXGIDeviceSubObject_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows_core::Result<()>;
    fn Map(this: &Self::This, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows_core::Result<()>;
    fn Unmap(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IDXGISurface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDeviceSubObject);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISurface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Map(this, ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&mapflags)).into())
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this).into())
        }
        IDXGISurface_Vtbl {
            base__: <IDXGIDeviceSubObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface1_Impl: ::windows_core::BaseImpl + IDXGISurface_Impl {
    fn GetDC(this: &Self::This, discard: super::super::Foundation::BOOL) -> ::windows_core::Result<super::Gdi::HDC>;
    fn ReleaseDC(this: &Self::This, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGISurface1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGISurface);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISurface1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDC(this, ::core::mem::transmute_copy(&discard)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&pdirtyrect)).into())
        }
        IDXGISurface1_Vtbl {
            base__: <IDXGISurface as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface2_Impl: ::windows_core::BaseImpl + IDXGISurface1_Impl {
    fn GetResource(this: &Self::This, riid: *const ::windows_core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDXGISurface2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGISurface1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISurface2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResource(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparentresource), ::core::mem::transmute_copy(&psubresourceindex)).into())
        }
        IDXGISurface2_Vtbl { base__: <IDXGISurface1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetResource: GetResource::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain_Impl: ::windows_core::BaseImpl + IDXGIDeviceSubObject_Impl {
    fn Present(this: &Self::This, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT;
    fn GetBuffer(this: &Self::This, buffer: u32, riid: *const ::windows_core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetFullscreenState(this: &Self::This, fullscreen: super::super::Foundation::BOOL, ptarget: ::core::option::Option<&IDXGIOutput>) -> ::windows_core::Result<()>;
    fn GetFullscreenState(this: &Self::This, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows_core::Result<()>;
    fn GetDesc(this: &Self::This, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::Result<()>;
    fn ResizeBuffers(this: &Self::This, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::Result<()>;
    fn ResizeTarget(this: &Self::This, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::Result<()>;
    fn GetContainingOutput(this: &Self::This) -> ::windows_core::Result<IDXGIOutput>;
    fn GetFrameStatistics(this: &Self::This, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::Result<()>;
    fn GetLastPresentCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGISwapChain {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGIDeviceSubObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISwapChain {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Present<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Present(this, ::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows_core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsurface)).into())
        }
        unsafe extern "system" fn SetFullscreenState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFullscreenState(this, ::core::mem::transmute_copy(&fullscreen), ::windows_core::from_raw_borrowed(&ptarget)).into())
        }
        unsafe extern "system" fn GetFullscreenState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFullscreenState(this, ::core::mem::transmute_copy(&pfullscreen), ::core::mem::transmute_copy(&pptarget)).into())
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn ResizeBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResizeBuffers(this, ::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&newformat), ::core::mem::transmute_copy(&swapchainflags)).into())
        }
        unsafe extern "system" fn ResizeTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResizeTarget(this, ::core::mem::transmute_copy(&pnewtargetparameters)).into())
        }
        unsafe extern "system" fn GetContainingOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainingOutput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameStatistics(this, ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastPresentCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plastpresentcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGISwapChain_Vtbl {
            base__: <IDXGIDeviceSubObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Present: Present::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            SetFullscreenState: SetFullscreenState::<Identity, Impl, OFFSET>,
            GetFullscreenState: GetFullscreenState::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            ResizeBuffers: ResizeBuffers::<Identity, Impl, OFFSET>,
            ResizeTarget: ResizeTarget::<Identity, Impl, OFFSET>,
            GetContainingOutput: GetContainingOutput::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain1_Impl: ::windows_core::BaseImpl + IDXGISwapChain_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_core::Result<()>;
    fn GetFullscreenDesc(this: &Self::This, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_core::Result<()>;
    fn GetHwnd(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn GetCoreWindow(this: &Self::This, refiid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Present1(this: &Self::This, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_core::HRESULT;
    fn IsTemporaryMonoSupported(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetRestrictToOutput(this: &Self::This) -> ::windows_core::Result<IDXGIOutput>;
    fn SetBackgroundColor(this: &Self::This, pcolor: *const DXGI_RGBA) -> ::windows_core::Result<()>;
    fn GetBackgroundColor(this: &Self::This) -> ::windows_core::Result<DXGI_RGBA>;
    fn SetRotation(this: &Self::This, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_core::Result<()>;
    fn GetRotation(this: &Self::This) -> ::windows_core::Result<Common::DXGI_MODE_ROTATION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGISwapChain1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGISwapChain);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISwapChain1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFullscreenDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHwnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCoreWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCoreWindow(this, ::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn Present1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Present1(this, ::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&presentflags), ::core::mem::transmute_copy(&ppresentparameters)))
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsTemporaryMonoSupported(this))
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictToOutput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprestricttooutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackgroundColor(this, ::core::mem::transmute_copy(&pcolor)).into())
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackgroundColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcolor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRotation(this, ::core::mem::transmute_copy(&rotation)).into())
        }
        unsafe extern "system" fn GetRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRotation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(protation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDXGISwapChain1_Vtbl {
            base__: <IDXGISwapChain as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc1: GetDesc1::<Identity, Impl, OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Identity, Impl, OFFSET>,
            GetHwnd: GetHwnd::<Identity, Impl, OFFSET>,
            GetCoreWindow: GetCoreWindow::<Identity, Impl, OFFSET>,
            Present1: Present1::<Identity, Impl, OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Identity, Impl, OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, Impl, OFFSET>,
            SetRotation: SetRotation::<Identity, Impl, OFFSET>,
            GetRotation: GetRotation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain2_Impl: ::windows_core::BaseImpl + IDXGISwapChain1_Impl {
    fn SetSourceSize(this: &Self::This, width: u32, height: u32) -> ::windows_core::Result<()>;
    fn GetSourceSize(this: &Self::This, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::Result<()>;
    fn SetMaximumFrameLatency(this: &Self::This, maxlatency: u32) -> ::windows_core::Result<()>;
    fn GetMaximumFrameLatency(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFrameLatencyWaitableObject(this: &Self::This) -> super::super::Foundation::HANDLE;
    fn SetMatrixTransform(this: &Self::This, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()>;
    fn GetMatrixTransform(this: &Self::This, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGISwapChain2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGISwapChain1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISwapChain2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSourceSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSourceSize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn GetSourceSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourceSize(this, ::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into())
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumFrameLatency(this, ::core::mem::transmute_copy(&maxlatency)).into())
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumFrameLatency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxlatency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameLatencyWaitableObject(this))
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrixTransform(this, ::core::mem::transmute_copy(&pmatrix)).into())
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMatrixTransform(this, ::core::mem::transmute_copy(&pmatrix)).into())
        }
        IDXGISwapChain2_Vtbl {
            base__: <IDXGISwapChain1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSourceSize: SetSourceSize::<Identity, Impl, OFFSET>,
            GetSourceSize: GetSourceSize::<Identity, Impl, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Identity, Impl, OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Identity, Impl, OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain3_Impl: ::windows_core::BaseImpl + IDXGISwapChain2_Impl {
    fn GetCurrentBackBufferIndex(this: &Self::This) -> u32;
    fn CheckColorSpaceSupport(this: &Self::This, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<u32>;
    fn SetColorSpace1(this: &Self::This, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<()>;
    fn ResizeBuffers1(this: &Self::This, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGISwapChain3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGISwapChain2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISwapChain3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentBackBufferIndex(this))
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckColorSpaceSupport(this, ::core::mem::transmute_copy(&colorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcolorspacesupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColorSpace1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorSpace1(this, ::core::mem::transmute_copy(&colorspace)).into())
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResizeBuffers1(this, ::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&swapchainflags), ::core::mem::transmute_copy(&pcreationnodemask), ::core::mem::transmute_copy(&pppresentqueue)).into())
        }
        IDXGISwapChain3_Vtbl {
            base__: <IDXGISwapChain2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, Impl, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, Impl, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, Impl, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain4_Impl: ::windows_core::BaseImpl + IDXGISwapChain3_Impl {
    fn SetHDRMetaData(this: &Self::This, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDXGISwapChain4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDXGISwapChain3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISwapChain4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHDRMetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChain4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHDRMetaData(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata)).into())
        }
        IDXGISwapChain4_Vtbl { base__: <IDXGISwapChain3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetHDRMetaData: SetHDRMetaData::<Identity, Impl, OFFSET> }
    };
}
pub trait IDXGISwapChainMedia_Impl: ::windows_core::BaseImpl {
    fn GetFrameStatisticsMedia(this: &Self::This, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows_core::Result<()>;
    fn SetPresentDuration(this: &Self::This, duration: u32) -> ::windows_core::Result<()>;
    fn CheckPresentDurationSupport(this: &Self::This, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDXGISwapChainMedia {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGISwapChainMedia {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameStatisticsMedia(this, ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn SetPresentDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPresentDuration(this, ::core::mem::transmute_copy(&duration)).into())
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckPresentDurationSupport(this, ::core::mem::transmute_copy(&desiredpresentduration), ::core::mem::transmute_copy(&pclosestsmallerpresentduration), ::core::mem::transmute_copy(&pclosestlargerpresentduration)).into())
        }
        IDXGISwapChainMedia_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, Impl, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, Impl, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDXGraphicsAnalysis_Impl: ::windows_core::BaseImpl {
    fn BeginCapture(this: &Self::This);
    fn EndCapture(this: &Self::This);
}
impl ::windows_core::Iids for IDXGraphicsAnalysis {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXGraphicsAnalysis {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginCapture(this))
        }
        unsafe extern "system" fn EndCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndCapture(this))
        }
        IDXGraphicsAnalysis_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginCapture: BeginCapture::<Identity, Impl, OFFSET>,
            EndCapture: EndCapture::<Identity, Impl, OFFSET>,
        }
    };
}
