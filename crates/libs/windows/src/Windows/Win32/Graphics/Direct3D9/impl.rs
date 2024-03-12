#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9_Impl: ::windows_core::BaseImpl {
    fn RegisterSoftwareDevice(this: &Self::This, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAdapterCount(this: &Self::This) -> u32;
    fn GetAdapterIdentifier(this: &Self::This, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows_core::Result<()>;
    fn GetAdapterModeCount(this: &Self::This, adapter: u32, format: D3DFORMAT) -> u32;
    fn EnumAdapterModes(this: &Self::This, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::Result<()>;
    fn GetAdapterDisplayMode(this: &Self::This, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::Result<()>;
    fn CheckDeviceType(this: &Self::This, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CheckDeviceFormat(this: &Self::This, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows_core::Result<()>;
    fn CheckDeviceMultiSampleType(this: &Self::This, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows_core::Result<()>;
    fn CheckDepthStencilMatch(this: &Self::This, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows_core::Result<()>;
    fn CheckDeviceFormatConversion(this: &Self::This, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows_core::Result<()>;
    fn GetDeviceCaps(this: &Self::This, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows_core::Result<()>;
    fn GetAdapterMonitor(this: &Self::This, adapter: u32) -> super::Gdi::HMONITOR;
    fn CreateDevice(this: &Self::This, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirect3D9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3D9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterSoftwareDevice(this, ::core::mem::transmute_copy(&pinitializefunction)).into())
        }
        unsafe extern "system" fn GetAdapterCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterCount(this))
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterIdentifier(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pidentifier)).into())
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterModeCount(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&format)))
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAdapterModes(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&pmode)).into())
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterDisplayMode(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pmode)).into())
        }
        unsafe extern "system" fn CheckDeviceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckDeviceType(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devtype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&backbufferformat), ::core::mem::transmute_copy(&bwindowed)).into())
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckDeviceFormat(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&rtype), ::core::mem::transmute_copy(&checkformat)).into())
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: super::super::Foundation::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckDeviceMultiSampleType(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&surfaceformat), ::core::mem::transmute_copy(&windowed), ::core::mem::transmute_copy(&multisampletype), ::core::mem::transmute_copy(&pqualitylevels)).into())
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckDepthStencilMatch(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&adapterformat), ::core::mem::transmute_copy(&rendertargetformat), ::core::mem::transmute_copy(&depthstencilformat)).into())
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckDeviceFormatConversion(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&sourceformat), ::core::mem::transmute_copy(&targetformat)).into())
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceCaps(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&pcaps)).into())
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterMonitor(this, ::core::mem::transmute_copy(&adapter)))
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDevice(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&hfocuswindow), ::core::mem::transmute_copy(&behaviorflags), ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&ppreturneddeviceinterface)).into())
        }
        IDirect3D9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterSoftwareDevice: RegisterSoftwareDevice::<Identity, Impl, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, Impl, OFFSET>,
            GetAdapterIdentifier: GetAdapterIdentifier::<Identity, Impl, OFFSET>,
            GetAdapterModeCount: GetAdapterModeCount::<Identity, Impl, OFFSET>,
            EnumAdapterModes: EnumAdapterModes::<Identity, Impl, OFFSET>,
            GetAdapterDisplayMode: GetAdapterDisplayMode::<Identity, Impl, OFFSET>,
            CheckDeviceType: CheckDeviceType::<Identity, Impl, OFFSET>,
            CheckDeviceFormat: CheckDeviceFormat::<Identity, Impl, OFFSET>,
            CheckDeviceMultiSampleType: CheckDeviceMultiSampleType::<Identity, Impl, OFFSET>,
            CheckDepthStencilMatch: CheckDepthStencilMatch::<Identity, Impl, OFFSET>,
            CheckDeviceFormatConversion: CheckDeviceFormatConversion::<Identity, Impl, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, Impl, OFFSET>,
            GetAdapterMonitor: GetAdapterMonitor::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3D9Ex_Impl: ::windows_core::BaseImpl + IDirect3D9_Impl {
    fn GetAdapterModeCountEx(this: &Self::This, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32;
    fn EnumAdapterModesEx(this: &Self::This, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows_core::Result<()>;
    fn GetAdapterDisplayModeEx(this: &Self::This, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows_core::Result<()>;
    fn CreateDeviceEx(this: &Self::This, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9Ex>) -> ::windows_core::Result<()>;
    fn GetAdapterLUID(this: &Self::This, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirect3D9Ex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3D9);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3D9Ex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAdapterModeCountEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterModeCountEx(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pfilter)))
        }
        unsafe extern "system" fn EnumAdapterModesEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAdapterModesEx(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&pmode)).into())
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterDisplayModeEx(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into())
        }
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeviceEx(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&devicetype), ::core::mem::transmute_copy(&hfocuswindow), ::core::mem::transmute_copy(&behaviorflags), ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pfullscreendisplaymode), ::core::mem::transmute_copy(&ppreturneddeviceinterface)).into())
        }
        unsafe extern "system" fn GetAdapterLUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3D9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterLUID(this, ::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&pluid)).into())
        }
        IDirect3D9Ex_Vtbl {
            base__: <IDirect3D9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAdapterModeCountEx: GetAdapterModeCountEx::<Identity, Impl, OFFSET>,
            EnumAdapterModesEx: EnumAdapterModesEx::<Identity, Impl, OFFSET>,
            GetAdapterDisplayModeEx: GetAdapterDisplayModeEx::<Identity, Impl, OFFSET>,
            CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET>,
            GetAdapterLUID: GetAdapterLUID::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DBaseTexture9_Impl: ::windows_core::BaseImpl + IDirect3DResource9_Impl {
    fn SetLOD(this: &Self::This, lodnew: u32) -> u32;
    fn GetLOD(this: &Self::This) -> u32;
    fn GetLevelCount(this: &Self::This) -> u32;
    fn SetAutoGenFilterType(this: &Self::This, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows_core::Result<()>;
    fn GetAutoGenFilterType(this: &Self::This) -> D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(this: &Self::This);
}
impl ::windows_core::Iids for IDirect3DBaseTexture9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DResource9);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DBaseTexture9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLOD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lodnew: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLOD(this, ::core::mem::transmute_copy(&lodnew)))
        }
        unsafe extern "system" fn GetLOD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLOD(this))
        }
        unsafe extern "system" fn GetLevelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLevelCount(this))
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoGenFilterType(this, ::core::mem::transmute_copy(&filtertype)).into())
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAutoGenFilterType(this))
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DBaseTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateMipSubLevels(this))
        }
        IDirect3DBaseTexture9_Vtbl {
            base__: <IDirect3DResource9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLOD: SetLOD::<Identity, Impl, OFFSET>,
            GetLOD: GetLOD::<Identity, Impl, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, Impl, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, Impl, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, Impl, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DCubeTexture9_Impl: ::windows_core::BaseImpl + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(this: &Self::This, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows_core::Result<()>;
    fn GetCubeMapSurface(this: &Self::This, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows_core::Result<IDirect3DSurface9>;
    fn LockRect(this: &Self::This, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows_core::Result<()>;
    fn UnlockRect(this: &Self::This, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows_core::Result<()>;
    fn AddDirtyRect(this: &Self::This, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirect3DCubeTexture9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DBaseTexture9);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DCubeTexture9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLevelDesc(this, ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetCubeMapSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCubeMapSurface(this, ::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcubemapsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockRect(this, ::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockRect(this, ::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&level)).into())
        }
        unsafe extern "system" fn AddDirtyRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DCubeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDirtyRect(this, ::core::mem::transmute_copy(&facetype), ::core::mem::transmute_copy(&pdirtyrect)).into())
        }
        IDirect3DCubeTexture9_Vtbl {
            base__: <IDirect3DBaseTexture9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetCubeMapSurface: GetCubeMapSurface::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9_Impl: ::windows_core::BaseImpl {
    fn TestCooperativeLevel(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAvailableTextureMem(this: &Self::This) -> u32;
    fn EvictManagedResources(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDirect3D(this: &Self::This) -> ::windows_core::Result<IDirect3D9>;
    fn GetDeviceCaps(this: &Self::This, pcaps: *mut D3DCAPS9) -> ::windows_core::Result<()>;
    fn GetDisplayMode(this: &Self::This, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::Result<()>;
    fn GetCreationParameters(this: &Self::This, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows_core::Result<()>;
    fn SetCursorProperties(this: &Self::This, xhotspot: u32, yhotspot: u32, pcursorbitmap: ::core::option::Option<&IDirect3DSurface9>) -> ::windows_core::Result<()>;
    fn SetCursorPosition(this: &Self::This, x: i32, y: i32, flags: u32);
    fn ShowCursor(this: &Self::This, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn CreateAdditionalSwapChain(this: &Self::This, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::core::option::Option<IDirect3DSwapChain9>) -> ::windows_core::Result<()>;
    fn GetSwapChain(this: &Self::This, iswapchain: u32) -> ::windows_core::Result<IDirect3DSwapChain9>;
    fn GetNumberOfSwapChains(this: &Self::This) -> u32;
    fn Reset(this: &Self::This, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows_core::Result<()>;
    fn Present(this: &Self::This, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows_core::Result<()>;
    fn GetBackBuffer(this: &Self::This, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(this: &Self::This, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows_core::Result<()>;
    fn SetDialogBoxMode(this: &Self::This, benabledialogs: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetGammaRamp(this: &Self::This, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP);
    fn GetGammaRamp(this: &Self::This, iswapchain: u32, pramp: *mut D3DGAMMARAMP);
    fn CreateTexture(this: &Self::This, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::core::option::Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn CreateVolumeTexture(this: &Self::This, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::core::option::Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn CreateCubeTexture(this: &Self::This, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::core::option::Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn CreateVertexBuffer(this: &Self::This, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::core::option::Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn CreateIndexBuffer(this: &Self::This, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::core::option::Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn CreateRenderTarget(this: &Self::This, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn CreateDepthStencilSurface(this: &Self::This, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn UpdateSurface(this: &Self::This, psourcesurface: ::core::option::Option<&IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: ::core::option::Option<&IDirect3DSurface9>, pdestpoint: *const super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn UpdateTexture(this: &Self::This, psourcetexture: ::core::option::Option<&IDirect3DBaseTexture9>, pdestinationtexture: ::core::option::Option<&IDirect3DBaseTexture9>) -> ::windows_core::Result<()>;
    fn GetRenderTargetData(this: &Self::This, prendertarget: ::core::option::Option<&IDirect3DSurface9>, pdestsurface: ::core::option::Option<&IDirect3DSurface9>) -> ::windows_core::Result<()>;
    fn GetFrontBufferData(this: &Self::This, iswapchain: u32, pdestsurface: ::core::option::Option<&IDirect3DSurface9>) -> ::windows_core::Result<()>;
    fn StretchRect(this: &Self::This, psourcesurface: ::core::option::Option<&IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestsurface: ::core::option::Option<&IDirect3DSurface9>, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows_core::Result<()>;
    fn ColorFill(this: &Self::This, psurface: ::core::option::Option<&IDirect3DSurface9>, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows_core::Result<()>;
    fn CreateOffscreenPlainSurface(this: &Self::This, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetRenderTarget(this: &Self::This, rendertargetindex: u32, prendertarget: ::core::option::Option<&IDirect3DSurface9>) -> ::windows_core::Result<()>;
    fn GetRenderTarget(this: &Self::This, rendertargetindex: u32) -> ::windows_core::Result<IDirect3DSurface9>;
    fn SetDepthStencilSurface(this: &Self::This, pnewzstencil: ::core::option::Option<&IDirect3DSurface9>) -> ::windows_core::Result<()>;
    fn GetDepthStencilSurface(this: &Self::This) -> ::windows_core::Result<IDirect3DSurface9>;
    fn BeginScene(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndScene(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows_core::Result<()>;
    fn SetTransform(this: &Self::This, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::Result<()>;
    fn GetTransform(this: &Self::This, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::Result<()>;
    fn MultiplyTransform(this: &Self::This, param0: D3DTRANSFORMSTATETYPE, param1: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::Result<()>;
    fn SetViewport(this: &Self::This, pviewport: *const D3DVIEWPORT9) -> ::windows_core::Result<()>;
    fn GetViewport(this: &Self::This, pviewport: *mut D3DVIEWPORT9) -> ::windows_core::Result<()>;
    fn SetMaterial(this: &Self::This, pmaterial: *const D3DMATERIAL9) -> ::windows_core::Result<()>;
    fn GetMaterial(this: &Self::This, pmaterial: *mut D3DMATERIAL9) -> ::windows_core::Result<()>;
    fn SetLight(this: &Self::This, index: u32, param1: *const D3DLIGHT9) -> ::windows_core::Result<()>;
    fn GetLight(this: &Self::This, index: u32, param1: *mut D3DLIGHT9) -> ::windows_core::Result<()>;
    fn LightEnable(this: &Self::This, index: u32, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLightEnable(this: &Self::This, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetClipPlane(this: &Self::This, index: u32, pplane: *const f32) -> ::windows_core::Result<()>;
    fn GetClipPlane(this: &Self::This, index: u32, pplane: *mut f32) -> ::windows_core::Result<()>;
    fn SetRenderState(this: &Self::This, state: D3DRENDERSTATETYPE, value: u32) -> ::windows_core::Result<()>;
    fn GetRenderState(this: &Self::This, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows_core::Result<()>;
    fn CreateStateBlock(this: &Self::This, r#type: D3DSTATEBLOCKTYPE) -> ::windows_core::Result<IDirect3DStateBlock9>;
    fn BeginStateBlock(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndStateBlock(this: &Self::This) -> ::windows_core::Result<IDirect3DStateBlock9>;
    fn SetClipStatus(this: &Self::This, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows_core::Result<()>;
    fn GetClipStatus(this: &Self::This, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows_core::Result<()>;
    fn GetTexture(this: &Self::This, stage: u32) -> ::windows_core::Result<IDirect3DBaseTexture9>;
    fn SetTexture(this: &Self::This, stage: u32, ptexture: ::core::option::Option<&IDirect3DBaseTexture9>) -> ::windows_core::Result<()>;
    fn GetTextureStageState(this: &Self::This, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows_core::Result<()>;
    fn SetTextureStageState(this: &Self::This, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows_core::Result<()>;
    fn GetSamplerState(this: &Self::This, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows_core::Result<()>;
    fn SetSamplerState(this: &Self::This, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows_core::Result<()>;
    fn ValidateDevice(this: &Self::This, pnumpasses: *mut u32) -> ::windows_core::Result<()>;
    fn SetPaletteEntries(this: &Self::This, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
    fn GetPaletteEntries(this: &Self::This, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::Result<()>;
    fn SetCurrentTexturePalette(this: &Self::This, palettenumber: u32) -> ::windows_core::Result<()>;
    fn GetCurrentTexturePalette(this: &Self::This, palettenumber: *mut u32) -> ::windows_core::Result<()>;
    fn SetScissorRect(this: &Self::This, prect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetScissorRect(this: &Self::This, prect: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetSoftwareVertexProcessing(this: &Self::This, bsoftware: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSoftwareVertexProcessing(this: &Self::This) -> super::super::Foundation::BOOL;
    fn SetNPatchMode(this: &Self::This, nsegments: f32) -> ::windows_core::Result<()>;
    fn GetNPatchMode(this: &Self::This) -> f32;
    fn DrawPrimitive(this: &Self::This, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows_core::Result<()>;
    fn DrawIndexedPrimitive(this: &Self::This, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows_core::Result<()>;
    fn DrawPrimitiveUP(this: &Self::This, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows_core::Result<()>;
    fn DrawIndexedPrimitiveUP(this: &Self::This, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows_core::Result<()>;
    fn ProcessVertices(this: &Self::This, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: ::core::option::Option<&IDirect3DVertexBuffer9>, pvertexdecl: ::core::option::Option<&IDirect3DVertexDeclaration9>, flags: u32) -> ::windows_core::Result<()>;
    fn CreateVertexDeclaration(this: &Self::This, pvertexelements: *const D3DVERTEXELEMENT9) -> ::windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetVertexDeclaration(this: &Self::This, pdecl: ::core::option::Option<&IDirect3DVertexDeclaration9>) -> ::windows_core::Result<()>;
    fn GetVertexDeclaration(this: &Self::This) -> ::windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetFVF(this: &Self::This, fvf: u32) -> ::windows_core::Result<()>;
    fn GetFVF(this: &Self::This, pfvf: *mut u32) -> ::windows_core::Result<()>;
    fn CreateVertexShader(this: &Self::This, pfunction: *const u32) -> ::windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShader(this: &Self::This, pshader: ::core::option::Option<&IDirect3DVertexShader9>) -> ::windows_core::Result<()>;
    fn GetVertexShader(this: &Self::This) -> ::windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShaderConstantF(this: &Self::This, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows_core::Result<()>;
    fn GetVertexShaderConstantF(this: &Self::This, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows_core::Result<()>;
    fn SetVertexShaderConstantI(this: &Self::This, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows_core::Result<()>;
    fn GetVertexShaderConstantI(this: &Self::This, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows_core::Result<()>;
    fn SetVertexShaderConstantB(this: &Self::This, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::Result<()>;
    fn GetVertexShaderConstantB(this: &Self::This, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::Result<()>;
    fn SetStreamSource(this: &Self::This, streamnumber: u32, pstreamdata: ::core::option::Option<&IDirect3DVertexBuffer9>, offsetinbytes: u32, stride: u32) -> ::windows_core::Result<()>;
    fn GetStreamSource(this: &Self::This, streamnumber: u32, ppstreamdata: *mut ::core::option::Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows_core::Result<()>;
    fn SetStreamSourceFreq(this: &Self::This, streamnumber: u32, setting: u32) -> ::windows_core::Result<()>;
    fn GetStreamSourceFreq(this: &Self::This, streamnumber: u32, psetting: *mut u32) -> ::windows_core::Result<()>;
    fn SetIndices(this: &Self::This, pindexdata: ::core::option::Option<&IDirect3DIndexBuffer9>) -> ::windows_core::Result<()>;
    fn GetIndices(this: &Self::This) -> ::windows_core::Result<IDirect3DIndexBuffer9>;
    fn CreatePixelShader(this: &Self::This, pfunction: *const u32) -> ::windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShader(this: &Self::This, pshader: ::core::option::Option<&IDirect3DPixelShader9>) -> ::windows_core::Result<()>;
    fn GetPixelShader(this: &Self::This) -> ::windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShaderConstantF(this: &Self::This, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows_core::Result<()>;
    fn GetPixelShaderConstantF(this: &Self::This, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows_core::Result<()>;
    fn SetPixelShaderConstantI(this: &Self::This, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows_core::Result<()>;
    fn GetPixelShaderConstantI(this: &Self::This, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows_core::Result<()>;
    fn SetPixelShaderConstantB(this: &Self::This, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::Result<()>;
    fn GetPixelShaderConstantB(this: &Self::This, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::Result<()>;
    fn DrawRectPatch(this: &Self::This, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows_core::Result<()>;
    fn DrawTriPatch(this: &Self::This, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows_core::Result<()>;
    fn DeletePatch(this: &Self::This, handle: u32) -> ::windows_core::Result<()>;
    fn CreateQuery(this: &Self::This, r#type: D3DQUERYTYPE) -> ::windows_core::Result<IDirect3DQuery9>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirect3DDevice9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DDevice9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TestCooperativeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TestCooperativeLevel(this).into())
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAvailableTextureMem(this))
        }
        unsafe extern "system" fn EvictManagedResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EvictManagedResources(this).into())
        }
        unsafe extern "system" fn GetDirect3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppd3d9: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDirect3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppd3d9, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3DCAPS9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceCaps(this, ::core::mem::transmute_copy(&pcaps)).into())
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayMode(this, ::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pmode)).into())
        }
        unsafe extern "system" fn GetCreationParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationParameters(this, ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn SetCursorProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCursorProperties(this, ::core::mem::transmute_copy(&xhotspot), ::core::mem::transmute_copy(&yhotspot), ::windows_core::from_raw_borrowed(&pcursorbitmap)).into())
        }
        unsafe extern "system" fn SetCursorPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, flags: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCursorPosition(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn ShowCursor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowCursor(this, ::core::mem::transmute_copy(&bshow)))
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAdditionalSwapChain(this, ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pswapchain)).into())
        }
        unsafe extern "system" fn GetSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSwapChain(this, ::core::mem::transmute_copy(&iswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumberOfSwapChains(this))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this, ::core::mem::transmute_copy(&ppresentationparameters)).into())
        }
        unsafe extern "system" fn Present<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Present(this, ::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion)).into())
        }
        unsafe extern "system" fn GetBackBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackBuffer(this, ::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&ibackbuffer), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRasterStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRasterStatus(this, ::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&prasterstatus)).into())
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabledialogs: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDialogBoxMode(this, ::core::mem::transmute_copy(&benabledialogs)).into())
        }
        unsafe extern "system" fn SetGammaRamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGammaRamp(this, ::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pramp)))
        }
        unsafe extern "system" fn GetGammaRamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGammaRamp(this, ::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pramp)))
        }
        unsafe extern "system" fn CreateTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTexture(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&pptexture), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVolumeTexture(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppvolumetexture), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCubeTexture(this, ::core::mem::transmute_copy(&edgelength), ::core::mem::transmute_copy(&levels), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppcubetexture), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVertexBuffer(this, ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&fvf), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppvertexbuffer), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateIndexBuffer(this, ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppindexbuffer), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRenderTarget(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&lockable), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDepthStencilSurface(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&discard), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn UpdateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcesurface: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: *mut ::core::ffi::c_void, pdestpoint: *const super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateSurface(this, ::windows_core::from_raw_borrowed(&psourcesurface), ::core::mem::transmute_copy(&psourcerect), ::windows_core::from_raw_borrowed(&pdestinationsurface), ::core::mem::transmute_copy(&pdestpoint)).into())
        }
        unsafe extern "system" fn UpdateTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcetexture: *mut ::core::ffi::c_void, pdestinationtexture: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateTexture(this, ::windows_core::from_raw_borrowed(&psourcetexture), ::windows_core::from_raw_borrowed(&pdestinationtexture)).into())
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prendertarget: *mut ::core::ffi::c_void, pdestsurface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRenderTargetData(this, ::windows_core::from_raw_borrowed(&prendertarget), ::windows_core::from_raw_borrowed(&pdestsurface)).into())
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pdestsurface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrontBufferData(this, ::core::mem::transmute_copy(&iswapchain), ::windows_core::from_raw_borrowed(&pdestsurface)).into())
        }
        unsafe extern "system" fn StretchRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcesurface: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestsurface: *mut ::core::ffi::c_void, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StretchRect(this, ::windows_core::from_raw_borrowed(&psourcesurface), ::core::mem::transmute_copy(&psourcerect), ::windows_core::from_raw_borrowed(&pdestsurface), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&filter)).into())
        }
        unsafe extern "system" fn ColorFill<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ColorFill(this, ::windows_core::from_raw_borrowed(&psurface), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&color)).into())
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateOffscreenPlainSurface(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle)).into())
        }
        unsafe extern "system" fn SetRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, prendertarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRenderTarget(this, ::core::mem::transmute_copy(&rendertargetindex), ::windows_core::from_raw_borrowed(&prendertarget)).into())
        }
        unsafe extern "system" fn GetRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRenderTarget(this, ::core::mem::transmute_copy(&rendertargetindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprendertarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewzstencil: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDepthStencilSurface(this, ::windows_core::from_raw_borrowed(&pnewzstencil)).into())
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppzstencilsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDepthStencilSurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppzstencilsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginScene<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginScene(this).into())
        }
        unsafe extern "system" fn EndScene<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndScene(this).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prects), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&stencil)).into())
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform(this, ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pmatrix)).into())
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransform(this, ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pmatrix)).into())
        }
        unsafe extern "system" fn MultiplyTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MultiplyTransform(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn SetViewport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewport(this, ::core::mem::transmute_copy(&pviewport)).into())
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetViewport(this, ::core::mem::transmute_copy(&pviewport)).into())
        }
        unsafe extern "system" fn SetMaterial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaterial(this, ::core::mem::transmute_copy(&pmaterial)).into())
        }
        unsafe extern "system" fn GetMaterial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaterial(this, ::core::mem::transmute_copy(&pmaterial)).into())
        }
        unsafe extern "system" fn SetLight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLight(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn GetLight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLight(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&param1)).into())
        }
        unsafe extern "system" fn LightEnable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LightEnable(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&enable)).into())
        }
        unsafe extern "system" fn GetLightEnable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLightEnable(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&penable)).into())
        }
        unsafe extern "system" fn SetClipPlane<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *const f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipPlane(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pplane)).into())
        }
        unsafe extern "system" fn GetClipPlane<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pplane: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClipPlane(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pplane)).into())
        }
        unsafe extern "system" fn SetRenderState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRenderState(this, ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetRenderState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRenderState(this, ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn CreateStateBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStateBlock(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginStateBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginStateBlock(this).into())
        }
        unsafe extern "system" fn EndStateBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndStateBlock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClipStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipStatus(this, ::core::mem::transmute_copy(&pclipstatus)).into())
        }
        unsafe extern "system" fn GetClipStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClipStatus(this, ::core::mem::transmute_copy(&pclipstatus)).into())
        }
        unsafe extern "system" fn GetTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, pptexture: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTexture(this, ::core::mem::transmute_copy(&stage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, ptexture: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTexture(this, ::core::mem::transmute_copy(&stage), ::windows_core::from_raw_borrowed(&ptexture)).into())
        }
        unsafe extern "system" fn GetTextureStageState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextureStageState(this, ::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn SetTextureStageState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextureStageState(this, ::core::mem::transmute_copy(&stage), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetSamplerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSamplerState(this, ::core::mem::transmute_copy(&sampler), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn SetSamplerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSamplerState(this, ::core::mem::transmute_copy(&sampler), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ValidateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumpasses: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateDevice(this, ::core::mem::transmute_copy(&pnumpasses)).into())
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPaletteEntries(this, ::core::mem::transmute_copy(&palettenumber), ::core::mem::transmute_copy(&pentries)).into())
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPaletteEntries(this, ::core::mem::transmute_copy(&palettenumber), ::core::mem::transmute_copy(&pentries)).into())
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentTexturePalette(this, ::core::mem::transmute_copy(&palettenumber)).into())
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palettenumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentTexturePalette(this, ::core::mem::transmute_copy(&palettenumber)).into())
        }
        unsafe extern "system" fn SetScissorRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScissorRect(this, ::core::mem::transmute_copy(&prect)).into())
        }
        unsafe extern "system" fn GetScissorRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScissorRect(this, ::core::mem::transmute_copy(&prect)).into())
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsoftware: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSoftwareVertexProcessing(this, ::core::mem::transmute_copy(&bsoftware)).into())
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSoftwareVertexProcessing(this))
        }
        unsafe extern "system" fn SetNPatchMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nsegments: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNPatchMode(this, ::core::mem::transmute_copy(&nsegments)).into())
        }
        unsafe extern "system" fn GetNPatchMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNPatchMode(this))
        }
        unsafe extern "system" fn DrawPrimitive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawPrimitive(this, ::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&startvertex), ::core::mem::transmute_copy(&primitivecount)).into())
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexedPrimitive(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&basevertexindex), ::core::mem::transmute_copy(&minvertexindex), ::core::mem::transmute_copy(&numvertices), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&primcount)).into())
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawPrimitiveUP(this, ::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&primitivecount), ::core::mem::transmute_copy(&pvertexstreamzerodata), ::core::mem::transmute_copy(&vertexstreamzerostride)).into())
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexedPrimitiveUP(this, ::core::mem::transmute_copy(&primitivetype), ::core::mem::transmute_copy(&minvertexindex), ::core::mem::transmute_copy(&numvertices), ::core::mem::transmute_copy(&primitivecount), ::core::mem::transmute_copy(&pindexdata), ::core::mem::transmute_copy(&indexdataformat), ::core::mem::transmute_copy(&pvertexstreamzerodata), ::core::mem::transmute_copy(&vertexstreamzerostride)).into())
        }
        unsafe extern "system" fn ProcessVertices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: *mut ::core::ffi::c_void, pvertexdecl: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessVertices(this, ::core::mem::transmute_copy(&srcstartindex), ::core::mem::transmute_copy(&destindex), ::core::mem::transmute_copy(&vertexcount), ::windows_core::from_raw_borrowed(&pdestbuffer), ::windows_core::from_raw_borrowed(&pvertexdecl), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVertexDeclaration(this, ::core::mem::transmute_copy(&pvertexelements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdecl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVertexDeclaration(this, ::windows_core::from_raw_borrowed(&pdecl)).into())
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdecl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVertexDeclaration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdecl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFVF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvf: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFVF(this, ::core::mem::transmute_copy(&fvf)).into())
        }
        unsafe extern "system" fn GetFVF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfvf: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFVF(this, ::core::mem::transmute_copy(&pfvf)).into())
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVertexShader(this, ::core::mem::transmute_copy(&pfunction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVertexShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVertexShader(this, ::windows_core::from_raw_borrowed(&pshader)).into())
        }
        unsafe extern "system" fn GetVertexShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVertexShader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVertexShaderConstantF(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into())
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVertexShaderConstantF(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into())
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVertexShaderConstantI(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into())
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVertexShaderConstantI(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into())
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVertexShaderConstantB(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into())
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVertexShaderConstantB(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into())
        }
        unsafe extern "system" fn SetStreamSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, pstreamdata: *mut ::core::ffi::c_void, offsetinbytes: u32, stride: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamSource(this, ::core::mem::transmute_copy(&streamnumber), ::windows_core::from_raw_borrowed(&pstreamdata), ::core::mem::transmute_copy(&offsetinbytes), ::core::mem::transmute_copy(&stride)).into())
        }
        unsafe extern "system" fn GetStreamSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut *mut ::core::ffi::c_void, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamSource(this, ::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&ppstreamdata), ::core::mem::transmute_copy(&poffsetinbytes), ::core::mem::transmute_copy(&pstride)).into())
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, setting: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamSourceFreq(this, ::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&setting)).into())
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamSourceFreq(this, ::core::mem::transmute_copy(&streamnumber), ::core::mem::transmute_copy(&psetting)).into())
        }
        unsafe extern "system" fn SetIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIndices(this, ::windows_core::from_raw_borrowed(&pindexdata)).into())
        }
        unsafe extern "system" fn GetIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppindexdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppindexdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePixelShader(this, ::core::mem::transmute_copy(&pfunction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPixelShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelShader(this, ::windows_core::from_raw_borrowed(&pshader)).into())
        }
        unsafe extern "system" fn GetPixelShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPixelShader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelShaderConstantF(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into())
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelShaderConstantF(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4fcount)).into())
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelShaderConstantI(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into())
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelShaderConstantI(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&vector4icount)).into())
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelShaderConstantB(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into())
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelShaderConstantB(this, ::core::mem::transmute_copy(&startregister), ::core::mem::transmute_copy(&pconstantdata), ::core::mem::transmute_copy(&boolcount)).into())
        }
        unsafe extern "system" fn DrawRectPatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawRectPatch(this, ::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&pnumsegs), ::core::mem::transmute_copy(&prectpatchinfo)).into())
        }
        unsafe extern "system" fn DrawTriPatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawTriPatch(this, ::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&pnumsegs), ::core::mem::transmute_copy(&ptripatchinfo)).into())
        }
        unsafe extern "system" fn DeletePatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePatch(this, ::core::mem::transmute_copy(&handle)).into())
        }
        unsafe extern "system" fn CreateQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQuery(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirect3DDevice9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, Impl, OFFSET>,
            GetAvailableTextureMem: GetAvailableTextureMem::<Identity, Impl, OFFSET>,
            EvictManagedResources: EvictManagedResources::<Identity, Impl, OFFSET>,
            GetDirect3D: GetDirect3D::<Identity, Impl, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetCreationParameters: GetCreationParameters::<Identity, Impl, OFFSET>,
            SetCursorProperties: SetCursorProperties::<Identity, Impl, OFFSET>,
            SetCursorPosition: SetCursorPosition::<Identity, Impl, OFFSET>,
            ShowCursor: ShowCursor::<Identity, Impl, OFFSET>,
            CreateAdditionalSwapChain: CreateAdditionalSwapChain::<Identity, Impl, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, Impl, OFFSET>,
            GetNumberOfSwapChains: GetNumberOfSwapChains::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Present: Present::<Identity, Impl, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, Impl, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, Impl, OFFSET>,
            SetDialogBoxMode: SetDialogBoxMode::<Identity, Impl, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, Impl, OFFSET>,
            GetGammaRamp: GetGammaRamp::<Identity, Impl, OFFSET>,
            CreateTexture: CreateTexture::<Identity, Impl, OFFSET>,
            CreateVolumeTexture: CreateVolumeTexture::<Identity, Impl, OFFSET>,
            CreateCubeTexture: CreateCubeTexture::<Identity, Impl, OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Identity, Impl, OFFSET>,
            CreateIndexBuffer: CreateIndexBuffer::<Identity, Impl, OFFSET>,
            CreateRenderTarget: CreateRenderTarget::<Identity, Impl, OFFSET>,
            CreateDepthStencilSurface: CreateDepthStencilSurface::<Identity, Impl, OFFSET>,
            UpdateSurface: UpdateSurface::<Identity, Impl, OFFSET>,
            UpdateTexture: UpdateTexture::<Identity, Impl, OFFSET>,
            GetRenderTargetData: GetRenderTargetData::<Identity, Impl, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, Impl, OFFSET>,
            StretchRect: StretchRect::<Identity, Impl, OFFSET>,
            ColorFill: ColorFill::<Identity, Impl, OFFSET>,
            CreateOffscreenPlainSurface: CreateOffscreenPlainSurface::<Identity, Impl, OFFSET>,
            SetRenderTarget: SetRenderTarget::<Identity, Impl, OFFSET>,
            GetRenderTarget: GetRenderTarget::<Identity, Impl, OFFSET>,
            SetDepthStencilSurface: SetDepthStencilSurface::<Identity, Impl, OFFSET>,
            GetDepthStencilSurface: GetDepthStencilSurface::<Identity, Impl, OFFSET>,
            BeginScene: BeginScene::<Identity, Impl, OFFSET>,
            EndScene: EndScene::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            MultiplyTransform: MultiplyTransform::<Identity, Impl, OFFSET>,
            SetViewport: SetViewport::<Identity, Impl, OFFSET>,
            GetViewport: GetViewport::<Identity, Impl, OFFSET>,
            SetMaterial: SetMaterial::<Identity, Impl, OFFSET>,
            GetMaterial: GetMaterial::<Identity, Impl, OFFSET>,
            SetLight: SetLight::<Identity, Impl, OFFSET>,
            GetLight: GetLight::<Identity, Impl, OFFSET>,
            LightEnable: LightEnable::<Identity, Impl, OFFSET>,
            GetLightEnable: GetLightEnable::<Identity, Impl, OFFSET>,
            SetClipPlane: SetClipPlane::<Identity, Impl, OFFSET>,
            GetClipPlane: GetClipPlane::<Identity, Impl, OFFSET>,
            SetRenderState: SetRenderState::<Identity, Impl, OFFSET>,
            GetRenderState: GetRenderState::<Identity, Impl, OFFSET>,
            CreateStateBlock: CreateStateBlock::<Identity, Impl, OFFSET>,
            BeginStateBlock: BeginStateBlock::<Identity, Impl, OFFSET>,
            EndStateBlock: EndStateBlock::<Identity, Impl, OFFSET>,
            SetClipStatus: SetClipStatus::<Identity, Impl, OFFSET>,
            GetClipStatus: GetClipStatus::<Identity, Impl, OFFSET>,
            GetTexture: GetTexture::<Identity, Impl, OFFSET>,
            SetTexture: SetTexture::<Identity, Impl, OFFSET>,
            GetTextureStageState: GetTextureStageState::<Identity, Impl, OFFSET>,
            SetTextureStageState: SetTextureStageState::<Identity, Impl, OFFSET>,
            GetSamplerState: GetSamplerState::<Identity, Impl, OFFSET>,
            SetSamplerState: SetSamplerState::<Identity, Impl, OFFSET>,
            ValidateDevice: ValidateDevice::<Identity, Impl, OFFSET>,
            SetPaletteEntries: SetPaletteEntries::<Identity, Impl, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, Impl, OFFSET>,
            SetCurrentTexturePalette: SetCurrentTexturePalette::<Identity, Impl, OFFSET>,
            GetCurrentTexturePalette: GetCurrentTexturePalette::<Identity, Impl, OFFSET>,
            SetScissorRect: SetScissorRect::<Identity, Impl, OFFSET>,
            GetScissorRect: GetScissorRect::<Identity, Impl, OFFSET>,
            SetSoftwareVertexProcessing: SetSoftwareVertexProcessing::<Identity, Impl, OFFSET>,
            GetSoftwareVertexProcessing: GetSoftwareVertexProcessing::<Identity, Impl, OFFSET>,
            SetNPatchMode: SetNPatchMode::<Identity, Impl, OFFSET>,
            GetNPatchMode: GetNPatchMode::<Identity, Impl, OFFSET>,
            DrawPrimitive: DrawPrimitive::<Identity, Impl, OFFSET>,
            DrawIndexedPrimitive: DrawIndexedPrimitive::<Identity, Impl, OFFSET>,
            DrawPrimitiveUP: DrawPrimitiveUP::<Identity, Impl, OFFSET>,
            DrawIndexedPrimitiveUP: DrawIndexedPrimitiveUP::<Identity, Impl, OFFSET>,
            ProcessVertices: ProcessVertices::<Identity, Impl, OFFSET>,
            CreateVertexDeclaration: CreateVertexDeclaration::<Identity, Impl, OFFSET>,
            SetVertexDeclaration: SetVertexDeclaration::<Identity, Impl, OFFSET>,
            GetVertexDeclaration: GetVertexDeclaration::<Identity, Impl, OFFSET>,
            SetFVF: SetFVF::<Identity, Impl, OFFSET>,
            GetFVF: GetFVF::<Identity, Impl, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, Impl, OFFSET>,
            SetVertexShader: SetVertexShader::<Identity, Impl, OFFSET>,
            GetVertexShader: GetVertexShader::<Identity, Impl, OFFSET>,
            SetVertexShaderConstantF: SetVertexShaderConstantF::<Identity, Impl, OFFSET>,
            GetVertexShaderConstantF: GetVertexShaderConstantF::<Identity, Impl, OFFSET>,
            SetVertexShaderConstantI: SetVertexShaderConstantI::<Identity, Impl, OFFSET>,
            GetVertexShaderConstantI: GetVertexShaderConstantI::<Identity, Impl, OFFSET>,
            SetVertexShaderConstantB: SetVertexShaderConstantB::<Identity, Impl, OFFSET>,
            GetVertexShaderConstantB: GetVertexShaderConstantB::<Identity, Impl, OFFSET>,
            SetStreamSource: SetStreamSource::<Identity, Impl, OFFSET>,
            GetStreamSource: GetStreamSource::<Identity, Impl, OFFSET>,
            SetStreamSourceFreq: SetStreamSourceFreq::<Identity, Impl, OFFSET>,
            GetStreamSourceFreq: GetStreamSourceFreq::<Identity, Impl, OFFSET>,
            SetIndices: SetIndices::<Identity, Impl, OFFSET>,
            GetIndices: GetIndices::<Identity, Impl, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, Impl, OFFSET>,
            SetPixelShader: SetPixelShader::<Identity, Impl, OFFSET>,
            GetPixelShader: GetPixelShader::<Identity, Impl, OFFSET>,
            SetPixelShaderConstantF: SetPixelShaderConstantF::<Identity, Impl, OFFSET>,
            GetPixelShaderConstantF: GetPixelShaderConstantF::<Identity, Impl, OFFSET>,
            SetPixelShaderConstantI: SetPixelShaderConstantI::<Identity, Impl, OFFSET>,
            GetPixelShaderConstantI: GetPixelShaderConstantI::<Identity, Impl, OFFSET>,
            SetPixelShaderConstantB: SetPixelShaderConstantB::<Identity, Impl, OFFSET>,
            GetPixelShaderConstantB: GetPixelShaderConstantB::<Identity, Impl, OFFSET>,
            DrawRectPatch: DrawRectPatch::<Identity, Impl, OFFSET>,
            DrawTriPatch: DrawTriPatch::<Identity, Impl, OFFSET>,
            DeletePatch: DeletePatch::<Identity, Impl, OFFSET>,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9Ex_Impl: ::windows_core::BaseImpl + IDirect3DDevice9_Impl {
    fn SetConvolutionMonoKernel(this: &Self::This, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows_core::Result<()>;
    fn ComposeRects(this: &Self::This, psrc: ::core::option::Option<&IDirect3DSurface9>, pdst: ::core::option::Option<&IDirect3DSurface9>, psrcrectdescs: ::core::option::Option<&IDirect3DVertexBuffer9>, numrects: u32, pdstrectdescs: ::core::option::Option<&IDirect3DVertexBuffer9>, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows_core::Result<()>;
    fn PresentEx(this: &Self::This, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetGPUThreadPriority(this: &Self::This, ppriority: *mut i32) -> ::windows_core::Result<()>;
    fn SetGPUThreadPriority(this: &Self::This, priority: i32) -> ::windows_core::Result<()>;
    fn WaitForVBlank(this: &Self::This, iswapchain: u32) -> ::windows_core::Result<()>;
    fn CheckResourceResidency(this: &Self::This, presourcearray: *mut ::core::option::Option<IDirect3DResource9>, numresources: u32) -> ::windows_core::Result<()>;
    fn SetMaximumFrameLatency(this: &Self::This, maxlatency: u32) -> ::windows_core::Result<()>;
    fn GetMaximumFrameLatency(this: &Self::This, pmaxlatency: *mut u32) -> ::windows_core::Result<()>;
    fn CheckDeviceState(this: &Self::This, hdestinationwindow: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn CreateRenderTargetEx(this: &Self::This, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows_core::Result<()>;
    fn CreateOffscreenPlainSurfaceEx(this: &Self::This, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows_core::Result<()>;
    fn CreateDepthStencilSurfaceEx(this: &Self::This, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows_core::Result<()>;
    fn ResetEx(this: &Self::This, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows_core::Result<()>;
    fn GetDisplayModeEx(this: &Self::This, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirect3DDevice9Ex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DDevice9);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DDevice9Ex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetConvolutionMonoKernel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConvolutionMonoKernel(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&rows), ::core::mem::transmute_copy(&columns)).into())
        }
        unsafe extern "system" fn ComposeRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrc: *mut ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, psrcrectdescs: *mut ::core::ffi::c_void, numrects: u32, pdstrectdescs: *mut ::core::ffi::c_void, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ComposeRects(this, ::windows_core::from_raw_borrowed(&psrc), ::windows_core::from_raw_borrowed(&pdst), ::windows_core::from_raw_borrowed(&psrcrectdescs), ::core::mem::transmute_copy(&numrects), ::windows_core::from_raw_borrowed(&pdstrectdescs), ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&xoffset), ::core::mem::transmute_copy(&yoffset)).into())
        }
        unsafe extern "system" fn PresentEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PresentEx(this, ::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGPUThreadPriority(this, ::core::mem::transmute_copy(&ppriority)).into())
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGPUThreadPriority(this, ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn WaitForVBlank<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForVBlank(this, ::core::mem::transmute_copy(&iswapchain)).into())
        }
        unsafe extern "system" fn CheckResourceResidency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcearray: *mut *mut ::core::ffi::c_void, numresources: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckResourceResidency(this, ::core::mem::transmute_copy(&presourcearray), ::core::mem::transmute_copy(&numresources)).into())
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumFrameLatency(this, ::core::mem::transmute_copy(&maxlatency)).into())
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaximumFrameLatency(this, ::core::mem::transmute_copy(&pmaxlatency)).into())
        }
        unsafe extern "system" fn CheckDeviceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckDeviceState(this, ::core::mem::transmute_copy(&hdestinationwindow)).into())
        }
        unsafe extern "system" fn CreateRenderTargetEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRenderTargetEx(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&lockable), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into())
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateOffscreenPlainSurfaceEx(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&pool), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into())
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: super::super::Foundation::BOOL, ppsurface: *mut *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDepthStencilSurfaceEx(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&multisample), ::core::mem::transmute_copy(&multisamplequality), ::core::mem::transmute_copy(&discard), ::core::mem::transmute_copy(&ppsurface), ::core::mem::transmute_copy(&psharedhandle), ::core::mem::transmute_copy(&usage)).into())
        }
        unsafe extern "system" fn ResetEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetEx(this, ::core::mem::transmute_copy(&ppresentationparameters), ::core::mem::transmute_copy(&pfullscreendisplaymode)).into())
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayModeEx(this, ::core::mem::transmute_copy(&iswapchain), ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into())
        }
        IDirect3DDevice9Ex_Vtbl {
            base__: <IDirect3DDevice9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetConvolutionMonoKernel: SetConvolutionMonoKernel::<Identity, Impl, OFFSET>,
            ComposeRects: ComposeRects::<Identity, Impl, OFFSET>,
            PresentEx: PresentEx::<Identity, Impl, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, Impl, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, Impl, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, Impl, OFFSET>,
            CheckResourceResidency: CheckResourceResidency::<Identity, Impl, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, Impl, OFFSET>,
            CreateRenderTargetEx: CreateRenderTargetEx::<Identity, Impl, OFFSET>,
            CreateOffscreenPlainSurfaceEx: CreateOffscreenPlainSurfaceEx::<Identity, Impl, OFFSET>,
            CreateDepthStencilSurfaceEx: CreateDepthStencilSurfaceEx::<Identity, Impl, OFFSET>,
            ResetEx: ResetEx::<Identity, Impl, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DIndexBuffer9_Impl: ::windows_core::BaseImpl + IDirect3DResource9_Impl {
    fn Lock(this: &Self::This, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDesc(this: &Self::This, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DIndexBuffer9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DResource9);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DIndexBuffer9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&offsettolock), ::core::mem::transmute_copy(&sizetolock), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this).into())
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DIndexBuffer9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        IDirect3DIndexBuffer9_Vtbl {
            base__: <IDirect3DResource9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DPixelShader9_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(this: &Self::This, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DPixelShader9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DPixelShader9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DPixelShader9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DPixelShader9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DPixelShader9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunction(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&psizeofdata)).into())
        }
        IDirect3DPixelShader9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DQuery9_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn GetType(this: &Self::This) -> D3DQUERYTYPE;
    fn GetDataSize(this: &Self::This) -> u32;
    fn Issue(this: &Self::This, dwissueflags: u32) -> ::windows_core::Result<()>;
    fn GetData(this: &Self::This, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DQuery9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DQuery9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3DQUERYTYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this))
        }
        unsafe extern "system" fn GetDataSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataSize(this))
        }
        unsafe extern "system" fn Issue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwissueflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Issue(this, ::core::mem::transmute_copy(&dwissueflags)).into())
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DQuery9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwgetdataflags)).into())
        }
        IDirect3DQuery9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDataSize: GetDataSize::<Identity, Impl, OFFSET>,
            Issue: Issue::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DResource9_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(this: &Self::This, refguid: *const ::windows_core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows_core::Result<()>;
    fn GetPrivateData(this: &Self::This, refguid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::Result<()>;
    fn FreePrivateData(this: &Self::This, refguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetPriority(this: &Self::This, prioritynew: u32) -> u32;
    fn GetPriority(this: &Self::This) -> u32;
    fn PreLoad(this: &Self::This);
    fn GetType(this: &Self::This) -> D3DRESOURCETYPE;
}
impl ::windows_core::Iids for IDirect3DResource9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DResource9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows_core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&sizeofdata), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&psizeofdata)).into())
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreePrivateData(this, ::core::mem::transmute_copy(&refguid)).into())
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prioritynew: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&prioritynew)))
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPriority(this))
        }
        unsafe extern "system" fn PreLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreLoad(this))
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DResource9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3DRESOURCETYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this))
        }
        IDirect3DResource9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            PreLoad: PreLoad::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DStateBlock9_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn Capture(this: &Self::This) -> ::windows_core::Result<()>;
    fn Apply(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DStateBlock9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DStateBlock9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Capture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Capture(this).into())
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DStateBlock9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Apply(this).into())
        }
        IDirect3DStateBlock9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            Capture: Capture::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSurface9_Impl: ::windows_core::BaseImpl + IDirect3DResource9_Impl {
    fn GetContainer(this: &Self::This, riid: *const ::windows_core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDesc(this: &Self::This, pdesc: *mut D3DSURFACE_DESC) -> ::windows_core::Result<()>;
    fn LockRect(this: &Self::This, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows_core::Result<()>;
    fn UnlockRect(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDC(this: &Self::This, phdc: *mut super::Gdi::HDC) -> ::windows_core::Result<()>;
    fn ReleaseDC(this: &Self::This, hdc: super::Gdi::HDC) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirect3DSurface9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DResource9);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DSurface9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContainer(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcontainer)).into())
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn LockRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockRect(this, ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockRect(this).into())
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDC(this, ::core::mem::transmute_copy(&phdc)).into())
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSurface9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&hdc)).into())
        }
        IDirect3DSurface9_Vtbl {
            base__: <IDirect3DResource9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9_Impl: ::windows_core::BaseImpl {
    fn Present(this: &Self::This, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetFrontBufferData(this: &Self::This, pdestsurface: ::core::option::Option<&IDirect3DSurface9>) -> ::windows_core::Result<()>;
    fn GetBackBuffer(this: &Self::This, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(this: &Self::This, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows_core::Result<()>;
    fn GetDisplayMode(this: &Self::This, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::Result<()>;
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn GetPresentParameters(this: &Self::This, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirect3DSwapChain9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DSwapChain9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Present<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Present(this, ::core::mem::transmute_copy(&psourcerect), ::core::mem::transmute_copy(&pdestrect), ::core::mem::transmute_copy(&hdestwindowoverride), ::core::mem::transmute_copy(&pdirtyregion), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestsurface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrontBufferData(this, ::windows_core::from_raw_borrowed(&pdestsurface)).into())
        }
        unsafe extern "system" fn GetBackBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackBuffer(this, ::core::mem::transmute_copy(&ibackbuffer), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRasterStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRasterStatus(this, ::core::mem::transmute_copy(&prasterstatus)).into())
        }
        unsafe extern "system" fn GetDisplayMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayMode(this, ::core::mem::transmute_copy(&pmode)).into())
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPresentParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresentParameters(this, ::core::mem::transmute_copy(&ppresentationparameters)).into())
        }
        IDirect3DSwapChain9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Present: Present::<Identity, Impl, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, Impl, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, Impl, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, Impl, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetPresentParameters: GetPresentParameters::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DSwapChain9Ex_Impl: ::windows_core::BaseImpl + IDirect3DSwapChain9_Impl {
    fn GetLastPresentCount(this: &Self::This, plastpresentcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetPresentStats(this: &Self::This, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows_core::Result<()>;
    fn GetDisplayModeEx(this: &Self::This, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDirect3DSwapChain9Ex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DSwapChain9);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DSwapChain9Ex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastPresentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastPresentCount(this, ::core::mem::transmute_copy(&plastpresentcount)).into())
        }
        unsafe extern "system" fn GetPresentStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresentStats(this, ::core::mem::transmute_copy(&ppresentationstatistics)).into())
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DSwapChain9Ex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayModeEx(this, ::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&protation)).into())
        }
        IDirect3DSwapChain9Ex_Vtbl {
            base__: <IDirect3DSwapChain9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLastPresentCount: GetLastPresentCount::<Identity, Impl, OFFSET>,
            GetPresentStats: GetPresentStats::<Identity, Impl, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirect3DTexture9_Impl: ::windows_core::BaseImpl + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(this: &Self::This, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows_core::Result<()>;
    fn GetSurfaceLevel(this: &Self::This, level: u32) -> ::windows_core::Result<IDirect3DSurface9>;
    fn LockRect(this: &Self::This, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows_core::Result<()>;
    fn UnlockRect(this: &Self::This, level: u32) -> ::windows_core::Result<()>;
    fn AddDirtyRect(this: &Self::This, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirect3DTexture9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DBaseTexture9);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DTexture9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLevelDesc(this, ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetSurfaceLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, ppsurfacelevel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSurfaceLevel(this, ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsurfacelevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockRect(this, ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn UnlockRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockRect(this, ::core::mem::transmute_copy(&level)).into())
        }
        unsafe extern "system" fn AddDirtyRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDirtyRect(this, ::core::mem::transmute_copy(&pdirtyrect)).into())
        }
        IDirect3DTexture9_Vtbl {
            base__: <IDirect3DBaseTexture9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetSurfaceLevel: GetSurfaceLevel::<Identity, Impl, OFFSET>,
            LockRect: LockRect::<Identity, Impl, OFFSET>,
            UnlockRect: UnlockRect::<Identity, Impl, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DVertexBuffer9_Impl: ::windows_core::BaseImpl + IDirect3DResource9_Impl {
    fn Lock(this: &Self::This, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::Result<()>;
    fn Unlock(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDesc(this: &Self::This, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DVertexBuffer9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DResource9);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DVertexBuffer9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&offsettolock), ::core::mem::transmute_copy(&sizetolock), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlock(this).into())
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexBuffer9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        IDirect3DVertexBuffer9_Vtbl {
            base__: <IDirect3DResource9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DVertexDeclaration9_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn GetDeclaration(this: &Self::This, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DVertexDeclaration9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DVertexDeclaration9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeclaration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexDeclaration9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeclaration(this, ::core::mem::transmute_copy(&pelement), ::core::mem::transmute_copy(&pnumelements)).into())
        }
        IDirect3DVertexDeclaration9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetDeclaration: GetDeclaration::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DVertexShader9_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(this: &Self::This, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DVertexShader9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexShader9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DVertexShader9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexShader9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVertexShader9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunction(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&psizeofdata)).into())
        }
        IDirect3DVertexShader9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DVolume9_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(this: &Self::This, refguid: *const ::windows_core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows_core::Result<()>;
    fn GetPrivateData(this: &Self::This, refguid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::Result<()>;
    fn FreePrivateData(this: &Self::This, refguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetContainer(this: &Self::This, riid: *const ::windows_core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetDesc(this: &Self::This, pdesc: *mut D3DVOLUME_DESC) -> ::windows_core::Result<()>;
    fn LockBox(this: &Self::This, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows_core::Result<()>;
    fn UnlockBox(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DVolume9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DVolume9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows_core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&sizeofdata), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&refguid), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&psizeofdata)).into())
        }
        unsafe extern "system" fn FreePrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreePrivateData(this, ::core::mem::transmute_copy(&refguid)).into())
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContainer(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppcontainer)).into())
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn LockBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockBox(this, ::core::mem::transmute_copy(&plockedvolume), ::core::mem::transmute_copy(&pbox), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn UnlockBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolume9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockBox(this).into())
        }
        IDirect3DVolume9_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            LockBox: LockBox::<Identity, Impl, OFFSET>,
            UnlockBox: UnlockBox::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirect3DVolumeTexture9_Impl: ::windows_core::BaseImpl + IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(this: &Self::This, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows_core::Result<()>;
    fn GetVolumeLevel(this: &Self::This, level: u32) -> ::windows_core::Result<IDirect3DVolume9>;
    fn LockBox(this: &Self::This, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows_core::Result<()>;
    fn UnlockBox(this: &Self::This, level: u32) -> ::windows_core::Result<()>;
    fn AddDirtyBox(this: &Self::This, pdirtybox: *const D3DBOX) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DVolumeTexture9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirect3DBaseTexture9);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DVolumeTexture9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLevelDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLevelDesc(this, ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, ppvolumelevel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVolumeLevel(this, ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvolumelevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockBox(this, ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&plockedvolume), ::core::mem::transmute_copy(&pbox), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn UnlockBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockBox(this, ::core::mem::transmute_copy(&level)).into())
        }
        unsafe extern "system" fn AddDirtyBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DVolumeTexture9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirtybox: *const D3DBOX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDirtyBox(this, ::core::mem::transmute_copy(&pdirtybox)).into())
        }
        IDirect3DVolumeTexture9_Vtbl {
            base__: <IDirect3DBaseTexture9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLevelDesc: GetLevelDesc::<Identity, Impl, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, Impl, OFFSET>,
            LockBox: LockBox::<Identity, Impl, OFFSET>,
            UnlockBox: UnlockBox::<Identity, Impl, OFFSET>,
            AddDirtyBox: AddDirtyBox::<Identity, Impl, OFFSET>,
        }
    };
}
