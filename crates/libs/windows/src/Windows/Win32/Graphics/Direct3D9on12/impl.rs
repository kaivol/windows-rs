#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Direct3D9\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
pub trait IDirect3DDevice9On12_Impl: ::windows_core::BaseImpl {
    fn GetD3D12Device(this: &Self::This, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UnwrapUnderlyingResource(this: &Self::This, presource: ::core::option::Option<&super::Direct3D9::IDirect3DResource9>, pcommandqueue: ::core::option::Option<&super::Direct3D12::ID3D12CommandQueue>, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReturnUnderlyingResource(this: &Self::This, presource: ::core::option::Option<&super::Direct3D9::IDirect3DResource9>, numsync: u32, psignalvalues: *mut u64, ppfences: *mut ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
impl ::windows_core::Iids for IDirect3DDevice9On12 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DDevice9On12 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetD3D12Device<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetD3D12Device(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into())
        }
        unsafe extern "system" fn UnwrapUnderlyingResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnwrapUnderlyingResource(this, ::windows_core::from_raw_borrowed(&presource), ::windows_core::from_raw_borrowed(&pcommandqueue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource12)).into())
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDevice9On12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *mut u64, ppfences: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReturnUnderlyingResource(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&numsync), ::core::mem::transmute_copy(&psignalvalues), ::core::mem::transmute_copy(&ppfences)).into())
        }
        IDirect3DDevice9On12_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetD3D12Device: GetD3D12Device::<Identity, Impl, OFFSET>,
            UnwrapUnderlyingResource: UnwrapUnderlyingResource::<Identity, Impl, OFFSET>,
            ReturnUnderlyingResource: ReturnUnderlyingResource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
