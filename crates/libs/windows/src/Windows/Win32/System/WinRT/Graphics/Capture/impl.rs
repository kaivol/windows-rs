#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IGraphicsCaptureItemInterop_Impl: ::windows_core::BaseImpl {
    fn CreateForWindow(this: &Self::This, window: super::super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateForMonitor(this: &Self::This, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IGraphicsCaptureItemInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsCaptureItemInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGraphicsCaptureItemInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsCaptureItemInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateForWindow(this, ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into())
        }
        unsafe extern "system" fn CreateForMonitor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsCaptureItemInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateForMonitor(this, ::core::mem::transmute_copy(&monitor), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&result)).into())
        }
        IGraphicsCaptureItemInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
            CreateForMonitor: CreateForMonitor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
