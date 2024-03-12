#[doc = "Required features: `\"Win32_Graphics_Direct3D11\"`, `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device_Impl: ::windows_core::BaseImpl {
    fn CreateWrappedResource(this: &Self::This, presource12: ::core::option::Option<&::windows_core::IUnknown>, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows_core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReleaseWrappedResources(this: &Self::This, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
    fn AcquireWrappedResources(this: &Self::This, ppresources: *const ::core::option::Option<super::Direct3D11::ID3D11Resource>, numresources: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ::windows_core::Iids for ID3D11On12Device {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11On12Device {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateWrappedResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource12: *mut ::core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows_core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateWrappedResource(this, ::windows_core::from_raw_borrowed(&presource12), ::core::mem::transmute_copy(&pflags11), ::core::mem::transmute_copy(&instate), ::core::mem::transmute_copy(&outstate), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresource11)).into())
        }
        unsafe extern "system" fn ReleaseWrappedResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, numresources: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseWrappedResources(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&numresources)))
        }
        unsafe extern "system" fn AcquireWrappedResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, numresources: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireWrappedResources(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&numresources)))
        }
        ID3D11On12Device_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateWrappedResource: CreateWrappedResource::<Identity, Impl, OFFSET>,
            ReleaseWrappedResources: ReleaseWrappedResources::<Identity, Impl, OFFSET>,
            AcquireWrappedResources: AcquireWrappedResources::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D11\"`, `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device1_Impl: ::windows_core::BaseImpl + ID3D11On12Device_Impl {
    fn GetD3D12Device(this: &Self::This, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ::windows_core::Iids for ID3D11On12Device1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11On12Device);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11On12Device1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetD3D12Device<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetD3D12Device(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvdevice)).into())
        }
        ID3D11On12Device1_Vtbl { base__: <ID3D11On12Device as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetD3D12Device: GetD3D12Device::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D11\"`, `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
pub trait ID3D11On12Device2_Impl: ::windows_core::BaseImpl + ID3D11On12Device1_Impl {
    fn UnwrapUnderlyingResource(this: &Self::This, presource11: ::core::option::Option<&super::Direct3D11::ID3D11Resource>, pcommandqueue: ::core::option::Option<&super::Direct3D12::ID3D12CommandQueue>, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReturnUnderlyingResource(this: &Self::This, presource11: ::core::option::Option<&super::Direct3D11::ID3D11Resource>, numsync: u32, psignalvalues: *const u64, ppfences: *const ::core::option::Option<super::Direct3D12::ID3D12Fence>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl ::windows_core::Iids for ID3D11On12Device2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11On12Device1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11On12Device2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UnwrapUnderlyingResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource11: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnwrapUnderlyingResource(this, ::windows_core::from_raw_borrowed(&presource11), ::windows_core::from_raw_borrowed(&pcommandqueue), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvresource12)).into())
        }
        unsafe extern "system" fn ReturnUnderlyingResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11On12Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource11: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *const u64, ppfences: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReturnUnderlyingResource(this, ::windows_core::from_raw_borrowed(&presource11), ::core::mem::transmute_copy(&numsync), ::core::mem::transmute_copy(&psignalvalues), ::core::mem::transmute_copy(&ppfences)).into())
        }
        ID3D11On12Device2_Vtbl {
            base__: <ID3D11On12Device1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UnwrapUnderlyingResource: UnwrapUnderlyingResource::<Identity, Impl, OFFSET>,
            ReturnUnderlyingResource: ReturnUnderlyingResource::<Identity, Impl, OFFSET>,
        }
    };
}
