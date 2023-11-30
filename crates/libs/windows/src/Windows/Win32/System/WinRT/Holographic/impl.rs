#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicCameraInterop_Impl: ::windows_core::BaseImpl {
    fn CreateDirect3D12BackBufferResource(this: &Self::This, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedBackBufferResource(this: &Self::This, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(this: &Self::This, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows_core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(this: &Self::This, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows_core::Result<()>;
    fn UnacquireDirect3D12BufferResource(this: &Self::This, presourcetounacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IHolographicCameraInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHolographicCameraInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDirect3D12BackBufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDirect3D12BackBufferResource(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcreatedtexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedBackBufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDirect3D12HardwareProtectedBackBufferResource(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::windows_core::from_raw_borrowed(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcreatedtexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireDirect3D12BufferResource(this, ::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue)).into())
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireDirect3D12BufferResourceWithTimeout(this, ::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into())
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnacquireDirect3D12BufferResource(this, ::windows_core::from_raw_borrowed(&presourcetounacquire)).into())
        }
        IHolographicCameraInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDirect3D12BackBufferResource: CreateDirect3D12BackBufferResource::<Identity, Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedBackBufferResource: CreateDirect3D12HardwareProtectedBackBufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, Impl, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicCameraRenderingParametersInterop_Impl: ::windows_core::BaseImpl {
    fn CommitDirect3D12Resource(this: &Self::This, pcolorresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()>;
    fn CommitDirect3D12ResourceWithDepthData(this: &Self::This, pcolorresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pdepthresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, depthresourcefencesignalvalue: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IHolographicCameraRenderingParametersInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHolographicCameraRenderingParametersInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitDirect3D12Resource(this, ::windows_core::from_raw_borrowed(&pcolorresourcetocommit), ::windows_core::from_raw_borrowed(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into())
        }
        unsafe extern "system" fn CommitDirect3D12ResourceWithDepthData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: *mut ::core::ffi::c_void, pdepthresourcefence: *mut ::core::ffi::c_void, depthresourcefencesignalvalue: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitDirect3D12ResourceWithDepthData(this, ::windows_core::from_raw_borrowed(&pcolorresourcetocommit), ::windows_core::from_raw_borrowed(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue), ::windows_core::from_raw_borrowed(&pdepthresourcetocommit), ::windows_core::from_raw_borrowed(&pdepthresourcefence), ::core::mem::transmute_copy(&depthresourcefencesignalvalue)).into())
        }
        IHolographicCameraRenderingParametersInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, Impl, OFFSET>,
            CommitDirect3D12ResourceWithDepthData: CommitDirect3D12ResourceWithDepthData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicQuadLayerInterop_Impl: ::windows_core::BaseImpl {
    fn CreateDirect3D12ContentBufferResource(this: &Self::This, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedContentBufferResource(this: &Self::This, pdevice: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(this: &Self::This, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> ::windows_core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(this: &Self::This, presourcetoacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> ::windows_core::Result<()>;
    fn UnacquireDirect3D12BufferResource(this: &Self::This, presourcetounacquire: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IHolographicQuadLayerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHolographicQuadLayerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDirect3D12ContentBufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDirect3D12ContentBufferResource(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedContentBufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDirect3D12HardwareProtectedContentBufferResource(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&ptexture2ddesc), ::windows_core::from_raw_borrowed(&pprotectedresourcesession)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcreatedtexture2dresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireDirect3D12BufferResource(this, ::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue)).into())
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireDirect3D12BufferResourceWithTimeout(this, ::windows_core::from_raw_borrowed(&presourcetoacquire), ::windows_core::from_raw_borrowed(&pcommandqueue), ::core::mem::transmute_copy(&duration)).into())
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnacquireDirect3D12BufferResource(this, ::windows_core::from_raw_borrowed(&presourcetounacquire)).into())
        }
        IHolographicQuadLayerInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDirect3D12ContentBufferResource: CreateDirect3D12ContentBufferResource::<Identity, Impl, OFFSET>,
            CreateDirect3D12HardwareProtectedContentBufferResource: CreateDirect3D12HardwareProtectedContentBufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, Impl, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicQuadLayerUpdateParametersInterop_Impl: ::windows_core::BaseImpl {
    fn CommitDirect3D12Resource(this: &Self::This, pcolorresourcetocommit: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: ::core::option::Option<&super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows_core::Iids for IHolographicQuadLayerUpdateParametersInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHolographicQuadLayerUpdateParametersInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitDirect3D12Resource(this, ::windows_core::from_raw_borrowed(&pcolorresourcetocommit), ::windows_core::from_raw_borrowed(&pcolorresourcefence), ::core::mem::transmute_copy(&colorresourcefencesignalvalue)).into())
        }
        IHolographicQuadLayerUpdateParametersInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
