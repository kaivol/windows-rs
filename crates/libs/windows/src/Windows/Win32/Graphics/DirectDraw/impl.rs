#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDDVideoPortContainer_Impl: ::windows_core::BaseImpl {
    fn CreateVideoPort(this: &Self::This, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::core::option::Option<IDirectDrawVideoPort>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumVideoPorts(this: &Self::This, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> ::windows_core::Result<()>;
    fn GetVideoPortConnectInfo(this: &Self::This, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows_core::Result<()>;
    fn QueryVideoPortStatus(this: &Self::This, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDDVideoPortContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDDVideoPortContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVideoPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVideoPort(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn EnumVideoPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumVideoPorts(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetVideoPortConnectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoPortConnectInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&pcinfo), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn QueryVideoPortStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDDVideoPortContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryVideoPortStatus(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDDVideoPortContainer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVideoPort: CreateVideoPort::<Identity, Impl, OFFSET>,
            EnumVideoPorts: EnumVideoPorts::<Identity, Impl, OFFSET>,
            GetVideoPortConnectInfo: GetVideoPortConnectInfo::<Identity, Impl, OFFSET>,
            QueryVideoPortStatus: QueryVideoPortStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDraw_Impl: ::windows_core::BaseImpl {
    fn Compact(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateClipper(this: &Self::This, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(this: &Self::This, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(this: &Self::This, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumSurfaces(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(this: &Self::This, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(this: &Self::This, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(this: &Self::This) -> ::windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(this: &Self::This, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(this: &Self::This, param0: u32, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(this: &Self::This, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDraw {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDraw {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compact(this).into())
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateClipper(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePalette(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DuplicateSurface(this, ::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDisplayModes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlipToGDISurface(this).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayMode(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFourCCCodes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGDISurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMonitorFrequency(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScanLine(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalBlankStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreDisplayMode(this).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayMode(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForVerticalBlank(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectDraw_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDraw2_Impl: ::windows_core::BaseImpl {
    fn Compact(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateClipper(this: &Self::This, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(this: &Self::This, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(this: &Self::This, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumSurfaces(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(this: &Self::This, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(this: &Self::This, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(this: &Self::This) -> ::windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(this: &Self::This, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(this: &Self::This, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(this: &Self::This, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetAvailableVidMem(this: &Self::This, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDraw2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDraw2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compact(this).into())
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateClipper(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePalette(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DuplicateSurface(this, ::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDisplayModes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlipToGDISurface(this).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayMode(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFourCCCodes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGDISurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMonitorFrequency(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScanLine(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalBlankStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreDisplayMode(this).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayMode(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForVerticalBlank(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAvailableVidMem(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectDraw2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDraw4_Impl: ::windows_core::BaseImpl {
    fn Compact(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateClipper(this: &Self::This, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(this: &Self::This, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(this: &Self::This, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface4>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<IDirectDrawSurface4>;
    fn EnumDisplayModes(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::Result<()>;
    fn EnumSurfaces(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(this: &Self::This, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(this: &Self::This, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(this: &Self::This) -> ::windows_core::Result<IDirectDrawSurface4>;
    fn GetMonitorFrequency(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(this: &Self::This, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(this: &Self::This, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(this: &Self::This, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetAvailableVidMem(this: &Self::This, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::Result<()>;
    fn GetSurfaceFromDC(this: &Self::This, param0: super::Gdi::HDC) -> ::windows_core::Result<IDirectDrawSurface4>;
    fn RestoreAllSurfaces(this: &Self::This) -> ::windows_core::Result<()>;
    fn TestCooperativeLevel(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDeviceIdentifier(this: &Self::This, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDraw4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDraw4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compact(this).into())
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateClipper(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePalette(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DuplicateSurface(this, ::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDisplayModes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlipToGDISurface(this).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayMode(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFourCCCodes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGDISurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMonitorFrequency(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScanLine(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalBlankStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreDisplayMode(this).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayMode(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForVerticalBlank(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAvailableVidMem(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSurfaceFromDC(this, ::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreAllSurfaces(this).into())
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TestCooperativeLevel(this).into())
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceIdentifier(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectDraw4_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, Impl, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, Impl, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDraw7_Impl: ::windows_core::BaseImpl {
    fn Compact(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateClipper(this: &Self::This, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePalette(this: &Self::This, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateSurface(this: &Self::This, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface7>, param2: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DuplicateSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<IDirectDrawSurface7>;
    fn EnumDisplayModes(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::Result<()>;
    fn EnumSurfaces(this: &Self::This, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::Result<()>;
    fn FlipToGDISurface(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::Result<()>;
    fn GetDisplayMode(this: &Self::This, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn GetFourCCCodes(this: &Self::This, param0: *mut u32, param1: *mut u32) -> ::windows_core::Result<()>;
    fn GetGDISurface(this: &Self::This) -> ::windows_core::Result<IDirectDrawSurface7>;
    fn GetMonitorFrequency(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetScanLine(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVerticalBlankStatus(this: &Self::This, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreDisplayMode(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCooperativeLevel(this: &Self::This, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::Result<()>;
    fn SetDisplayMode(this: &Self::This, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::Result<()>;
    fn WaitForVerticalBlank(this: &Self::This, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetAvailableVidMem(this: &Self::This, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::Result<()>;
    fn GetSurfaceFromDC(this: &Self::This, param0: super::Gdi::HDC) -> ::windows_core::Result<IDirectDrawSurface7>;
    fn RestoreAllSurfaces(this: &Self::This) -> ::windows_core::Result<()>;
    fn TestCooperativeLevel(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDeviceIdentifier(this: &Self::This, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows_core::Result<()>;
    fn StartModeTest(this: &Self::This, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn EvaluateMode(this: &Self::This, param0: u32, param1: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDraw7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDraw7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compact(this).into())
        }
        unsafe extern "system" fn CreateClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateClipper(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePalette(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)).into())
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2)).into())
        }
        unsafe extern "system" fn DuplicateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DuplicateSurface(this, ::windows_core::from_raw_borrowed(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDisplayModes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn EnumSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlipToGDISurface(this).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayMode(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFourCCCodes(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetGDISurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGDISurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMonitorFrequency(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetScanLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScanLine(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalBlankStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreDisplayMode(this).into())
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCooperativeLevel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayMode(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForVerticalBlank(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAvailableVidMem(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSurfaceFromDC(this, ::core::mem::transmute_copy(&param0)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreAllSurfaces(this).into())
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TestCooperativeLevel(this).into())
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceIdentifier(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn StartModeTest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartModeTest(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn EvaluateMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDraw7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EvaluateMode(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectDraw7_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Compact: Compact::<Identity, Impl, OFFSET>,
            CreateClipper: CreateClipper::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, Impl, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, Impl, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, Impl, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, Impl, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, Impl, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, Impl, OFFSET>,
            GetScanLine: GetScanLine::<Identity, Impl, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, Impl, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, Impl, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, Impl, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, Impl, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, Impl, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, Impl, OFFSET>,
            StartModeTest: StartModeTest::<Identity, Impl, OFFSET>,
            EvaluateMode: EvaluateMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawClipper_Impl: ::windows_core::BaseImpl {
    fn GetClipList(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows_core::Result<()>;
    fn GetHWnd(this: &Self::This, param0: *mut super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: ::core::option::Option<&IDirectDraw>, param1: u32) -> ::windows_core::Result<()>;
    fn IsClipListChanged(this: &Self::This, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetClipList(this: &Self::This, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows_core::Result<()>;
    fn SetHWnd(this: &Self::This, param0: u32, param1: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDrawClipper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawClipper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClipList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClipList(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetHWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHWnd(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn IsClipListChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsClipListChanged(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetClipList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipList(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetHWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawClipper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHWnd(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectDrawClipper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClipList: GetClipList::<Identity, Impl, OFFSET>,
            GetHWnd: GetHWnd::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsClipListChanged: IsClipListChanged::<Identity, Impl, OFFSET>,
            SetClipList: SetClipList::<Identity, Impl, OFFSET>,
            SetHWnd: SetHWnd::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectDrawColorControl_Impl: ::windows_core::BaseImpl {
    fn GetColorControls(this: &Self::This, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
    fn SetColorControls(this: &Self::This, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectDrawColorControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawColorControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawColorControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColorControls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawColorControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorControls(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetColorControls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawColorControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorControls(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectDrawColorControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColorControls: GetColorControls::<Identity, Impl, OFFSET>,
            SetColorControls: SetColorControls::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectDrawGammaControl_Impl: ::windows_core::BaseImpl {
    fn GetGammaRamp(this: &Self::This, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::Result<()>;
    fn SetGammaRamp(this: &Self::This, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectDrawGammaControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawGammaControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawGammaControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGammaRamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawGammaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGammaRamp(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetGammaRamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawGammaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGammaRamp(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectDrawGammaControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGammaRamp: GetGammaRamp::<Identity, Impl, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectDrawKernel_Impl: ::windows_core::BaseImpl {
    fn GetCaps(this: &Self::This, param0: *mut DDKERNELCAPS) -> ::windows_core::Result<()>;
    fn GetKernelHandle(this: &Self::This, param0: *mut usize) -> ::windows_core::Result<()>;
    fn ReleaseKernelHandle(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectDrawKernel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawKernel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDKERNELCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetKernelHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKernelHandle(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawKernel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseKernelHandle(this).into())
        }
        IDirectDrawKernel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetKernelHandle: GetKernelHandle::<Identity, Impl, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirectDrawPalette_Impl: ::windows_core::BaseImpl {
    fn GetCaps(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetEntries(this: &Self::This, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: ::core::option::Option<&IDirectDraw>, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
    fn SetEntries(this: &Self::This, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IDirectDrawPalette {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawPalette {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEntries(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn SetEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEntries(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        IDirectDrawPalette_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetEntries: GetEntries::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetEntries: SetEntries::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface_Impl: ::windows_core::BaseImpl {
    fn AddAttachedSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(this: &Self::This, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(this: &Self::This, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(this: &Self::This, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(this: &Self::This, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn Flip(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(this: &Self::This, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface>) -> ::windows_core::Result<()>;
    fn GetBltStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDSCAPS) -> ::windows_core::Result<()>;
    fn GetClipper(this: &Self::This) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(this: &Self::This, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(this: &Self::This, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(this: &Self::This) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(this: &Self::This, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn IsLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(this: &Self::This, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetClipper(this: &Self::This, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(this: &Self::This, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(this: &Self::This, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, param0: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UpdateOverlay(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDrawSurface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawSurface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAttachedSurface(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOverlayDirtyRect(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Blt(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltBatch(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltFast(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttachedSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumOverlayZOrders(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flip(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBltStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipper(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlipStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPalette(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSurfaceDesc(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLost(this).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipper(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlay(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayDisplay(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayZOrder(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        IDirectDrawSurface_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface2_Impl: ::windows_core::BaseImpl {
    fn AddAttachedSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(this: &Self::This, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(this: &Self::This, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(this: &Self::This, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface2>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(this: &Self::This, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn Flip(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface2>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(this: &Self::This, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn GetBltStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDSCAPS) -> ::windows_core::Result<()>;
    fn GetClipper(this: &Self::This) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(this: &Self::This, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(this: &Self::This, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(this: &Self::This) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(this: &Self::This, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn IsLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(this: &Self::This, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetClipper(this: &Self::This, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(this: &Self::This, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(this: &Self::This, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, param0: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UpdateOverlay(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface2>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface2>) -> ::windows_core::Result<()>;
    fn GetDDInterface(this: &Self::This, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDrawSurface2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawSurface2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAttachedSurface(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOverlayDirtyRect(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Blt(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltBatch(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltFast(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttachedSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumOverlayZOrders(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flip(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBltStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipper(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlipStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPalette(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSurfaceDesc(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLost(this).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipper(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlay(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayDisplay(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayZOrder(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDDInterface(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageLock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageUnlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectDrawSurface2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface3_Impl: ::windows_core::BaseImpl {
    fn AddAttachedSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(this: &Self::This, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(this: &Self::This, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(this: &Self::This, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface3>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(this: &Self::This, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::Result<()>;
    fn Flip(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface3>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(this: &Self::This, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn GetBltStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDSCAPS) -> ::windows_core::Result<()>;
    fn GetClipper(this: &Self::This) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(this: &Self::This, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(this: &Self::This, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(this: &Self::This) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(this: &Self::This, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC) -> ::windows_core::Result<()>;
    fn IsLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(this: &Self::This, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetClipper(this: &Self::This, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(this: &Self::This, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(this: &Self::This, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, param0: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UpdateOverlay(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface3>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface3>) -> ::windows_core::Result<()>;
    fn GetDDInterface(this: &Self::This, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn SetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDrawSurface3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawSurface3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAttachedSurface(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOverlayDirtyRect(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Blt(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltBatch(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltFast(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttachedSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumOverlayZOrders(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flip(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBltStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipper(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlipStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPalette(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSurfaceDesc(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLost(this).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipper(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlay(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayDisplay(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayZOrder(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDDInterface(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageLock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageUnlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSurfaceDesc(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IDirectDrawSurface3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface4_Impl: ::windows_core::BaseImpl {
    fn AddAttachedSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(this: &Self::This, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(this: &Self::This, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(this: &Self::This, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface4>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(this: &Self::This, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::Result<()>;
    fn Flip(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface4>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(this: &Self::This, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn GetBltStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDSCAPS2) -> ::windows_core::Result<()>;
    fn GetClipper(this: &Self::This) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(this: &Self::This, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(this: &Self::This, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(this: &Self::This) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(this: &Self::This, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn IsLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(this: &Self::This, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetClipper(this: &Self::This, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(this: &Self::This, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(this: &Self::This, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn UpdateOverlay(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface4>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface4>) -> ::windows_core::Result<()>;
    fn GetDDInterface(this: &Self::This, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn SetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::Result<()>;
    fn GetPrivateData(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::Result<()>;
    fn FreePrivateData(this: &Self::This, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetUniquenessValue(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn ChangeUniquenessValue(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDrawSurface4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawSurface4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAttachedSurface(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOverlayDirtyRect(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Blt(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltBatch(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltFast(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttachedSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumOverlayZOrders(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flip(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBltStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipper(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlipStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPalette(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSurfaceDesc(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLost(this).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipper(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlay(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayDisplay(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayZOrder(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDDInterface(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageLock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageUnlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSurfaceDesc(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreePrivateData(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUniquenessValue(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeUniquenessValue(this).into())
        }
        IDirectDrawSurface4_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, Impl, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirectDrawSurface7_Impl: ::windows_core::BaseImpl {
    fn AddAttachedSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn AddOverlayDirtyRect(this: &Self::This, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Blt(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::Result<()>;
    fn BltBatch(this: &Self::This, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::Result<()>;
    fn BltFast(this: &Self::This, param0: u32, param1: u32, param2: ::core::option::Option<&IDirectDrawSurface7>, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::Result<()>;
    fn DeleteAttachedSurface(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn EnumAttachedSurfaces(this: &Self::This, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::Result<()>;
    fn EnumOverlayZOrders(this: &Self::This, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::Result<()>;
    fn Flip(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface7>, param1: u32) -> ::windows_core::Result<()>;
    fn GetAttachedSurface(this: &Self::This, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn GetBltStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, param0: *mut DDSCAPS2) -> ::windows_core::Result<()>;
    fn GetClipper(this: &Self::This) -> ::windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn GetDC(this: &Self::This, param0: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn GetFlipStatus(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetOverlayPosition(this: &Self::This, param0: *mut i32, param1: *mut i32) -> ::windows_core::Result<()>;
    fn GetPalette(this: &Self::This) -> ::windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(this: &Self::This, param0: *mut DDPIXELFORMAT) -> ::windows_core::Result<()>;
    fn GetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, param0: ::core::option::Option<&IDirectDraw>, param1: *mut DDSURFACEDESC2) -> ::windows_core::Result<()>;
    fn IsLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn ReleaseDC(this: &Self::This, param0: super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetClipper(this: &Self::This, param0: ::core::option::Option<&IDirectDrawClipper>) -> ::windows_core::Result<()>;
    fn SetColorKey(this: &Self::This, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::Result<()>;
    fn SetOverlayPosition(this: &Self::This, param0: i32, param1: i32) -> ::windows_core::Result<()>;
    fn SetPalette(this: &Self::This, param0: ::core::option::Option<&IDirectDrawPalette>) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This, param0: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn UpdateOverlay(this: &Self::This, param0: *mut super::super::Foundation::RECT, param1: ::core::option::Option<&IDirectDrawSurface7>, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::Result<()>;
    fn UpdateOverlayDisplay(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn UpdateOverlayZOrder(this: &Self::This, param0: u32, param1: ::core::option::Option<&IDirectDrawSurface7>) -> ::windows_core::Result<()>;
    fn GetDDInterface(this: &Self::This, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PageLock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn PageUnlock(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn SetSurfaceDesc(this: &Self::This, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::Result<()>;
    fn GetPrivateData(this: &Self::This, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::Result<()>;
    fn FreePrivateData(this: &Self::This, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetUniquenessValue(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn ChangeUniquenessValue(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPriority(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SetLOD(this: &Self::This, param0: u32) -> ::windows_core::Result<()>;
    fn GetLOD(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirectDrawSurface7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawSurface7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAttachedSurface(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOverlayDirtyRect(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Blt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Blt(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn BltBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltBatch(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn BltFast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BltFast(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows_core::from_raw_borrowed(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttachedSurfaces(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumOverlayZOrders(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn Flip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flip(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttachedSurface(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetBltStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBltStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipper(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetFlipStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlipStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPalette(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelFormat(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSurfaceDesc(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn IsLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLost(this).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        unsafe extern "system" fn SetClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipper(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn SetColorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorKey(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverlayPosition(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlay(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayDisplay(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOverlayZOrder(this, ::core::mem::transmute_copy(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn GetDDInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDDInterface(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageLock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn PageUnlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PageUnlock(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSurfaceDesc(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreePrivateData(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUniquenessValue(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeUniquenessValue(this).into())
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPriority(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetLOD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLOD(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetLOD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurface7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLOD(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectDrawSurface7_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAttachedSurface: AddAttachedSurface::<Identity, Impl, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, Impl, OFFSET>,
            Blt: Blt::<Identity, Impl, OFFSET>,
            BltBatch: BltBatch::<Identity, Impl, OFFSET>,
            BltFast: BltFast::<Identity, Impl, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, Impl, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, Impl, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, Impl, OFFSET>,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, Impl, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetClipper: GetClipper::<Identity, Impl, OFFSET>,
            GetColorKey: GetColorKey::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, Impl, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, Impl, OFFSET>,
            GetPalette: GetPalette::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsLost: IsLost::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetClipper: SetClipper::<Identity, Impl, OFFSET>,
            SetColorKey: SetColorKey::<Identity, Impl, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, Impl, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, Impl, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, Impl, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, Impl, OFFSET>,
            PageLock: PageLock::<Identity, Impl, OFFSET>,
            PageUnlock: PageUnlock::<Identity, Impl, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, Impl, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetLOD: SetLOD::<Identity, Impl, OFFSET>,
            GetLOD: GetLOD::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectDrawSurfaceKernel_Impl: ::windows_core::BaseImpl {
    fn GetKernelHandle(this: &Self::This, param0: *mut usize) -> ::windows_core::Result<()>;
    fn ReleaseKernelHandle(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectDrawSurfaceKernel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawSurfaceKernel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetKernelHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKernelHandle(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn ReleaseKernelHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawSurfaceKernel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseKernelHandle(this).into())
        }
        IDirectDrawSurfaceKernel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetKernelHandle: GetKernelHandle::<Identity, Impl, OFFSET>,
            ReleaseKernelHandle: ReleaseKernelHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectDrawVideoPort_Impl: ::windows_core::BaseImpl {
    fn Flip(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface>, param1: u32) -> ::windows_core::Result<()>;
    fn GetBandwidthInfo(this: &Self::This, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows_core::Result<()>;
    fn GetColorControls(this: &Self::This, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
    fn GetInputFormats(this: &Self::This, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows_core::Result<()>;
    fn GetOutputFormats(this: &Self::This, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows_core::Result<()>;
    fn GetFieldPolarity(this: &Self::This, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetVideoLine(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn GetVideoSignalStatus(this: &Self::This, param0: *mut u32) -> ::windows_core::Result<()>;
    fn SetColorControls(this: &Self::This, param0: *mut DDCOLORCONTROL) -> ::windows_core::Result<()>;
    fn SetTargetSurface(this: &Self::This, param0: ::core::option::Option<&IDirectDrawSurface>, param1: u32) -> ::windows_core::Result<()>;
    fn StartVideo(this: &Self::This, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::Result<()>;
    fn StopVideo(this: &Self::This) -> ::windows_core::Result<()>;
    fn UpdateVideo(this: &Self::This, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::Result<()>;
    fn WaitForSync(this: &Self::This, param0: u32, param1: u32, param2: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectDrawVideoPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawVideoPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Flip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flip(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetBandwidthInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBandwidthInfo(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into())
        }
        unsafe extern "system" fn GetColorControls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorControls(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetInputFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputFormats(this, ::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn GetOutputFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputFormats(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&lpnumformats), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn GetFieldPolarity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFieldPolarity(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetVideoLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoLine(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetVideoSignalStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoSignalStatus(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetColorControls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorControls(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn SetTargetSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetSurface(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn StartVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartVideo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn StopVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopVideo(this).into())
        }
        unsafe extern "system" fn UpdateVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateVideo(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn WaitForSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForSync(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into())
        }
        IDirectDrawVideoPort_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Flip: Flip::<Identity, Impl, OFFSET>,
            GetBandwidthInfo: GetBandwidthInfo::<Identity, Impl, OFFSET>,
            GetColorControls: GetColorControls::<Identity, Impl, OFFSET>,
            GetInputFormats: GetInputFormats::<Identity, Impl, OFFSET>,
            GetOutputFormats: GetOutputFormats::<Identity, Impl, OFFSET>,
            GetFieldPolarity: GetFieldPolarity::<Identity, Impl, OFFSET>,
            GetVideoLine: GetVideoLine::<Identity, Impl, OFFSET>,
            GetVideoSignalStatus: GetVideoSignalStatus::<Identity, Impl, OFFSET>,
            SetColorControls: SetColorControls::<Identity, Impl, OFFSET>,
            SetTargetSurface: SetTargetSurface::<Identity, Impl, OFFSET>,
            StartVideo: StartVideo::<Identity, Impl, OFFSET>,
            StopVideo: StopVideo::<Identity, Impl, OFFSET>,
            UpdateVideo: UpdateVideo::<Identity, Impl, OFFSET>,
            WaitForSync: WaitForSync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectDrawVideoPortNotify_Impl: ::windows_core::BaseImpl {
    fn AcquireNotification(this: &Self::This, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows_core::Result<()>;
    fn ReleaseNotification(this: &Self::This, param0: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectDrawVideoPortNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectDrawVideoPortNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcquireNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireNotification(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn ReleaseNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectDrawVideoPortNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseNotification(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IDirectDrawVideoPortNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcquireNotification: AcquireNotification::<Identity, Impl, OFFSET>,
            ReleaseNotification: ReleaseNotification::<Identity, Impl, OFFSET>,
        }
    };
}
