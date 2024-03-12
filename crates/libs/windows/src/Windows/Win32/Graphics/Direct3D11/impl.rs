pub trait ID3D11Asynchronous_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetDataSize(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for ID3D11Asynchronous {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Asynchronous_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Asynchronous {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDataSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Asynchronous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataSize(this))
        }
        ID3D11Asynchronous_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDataSize: GetDataSize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11AuthenticatedChannel_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetCertificateSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCertificate(this: &Self::This, certificatesize: u32, pcertificate: *mut u8) -> ::windows_core::Result<()>;
    fn GetChannelHandle(this: &Self::This, pchannelhandle: *mut super::super::Foundation::HANDLE);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11AuthenticatedChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11AuthenticatedChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCertificateSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertificateSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcertificatesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCertificate(this, ::core::mem::transmute_copy(&certificatesize), ::core::mem::transmute_copy(&pcertificate)).into())
        }
        unsafe extern "system" fn GetChannelHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannelhandle: *mut super::super::Foundation::HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChannelHandle(this, ::core::mem::transmute_copy(&pchannelhandle)))
        }
        ID3D11AuthenticatedChannel_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCertificateSize: GetCertificateSize::<Identity, Impl, OFFSET>,
            GetCertificate: GetCertificate::<Identity, Impl, OFFSET>,
            GetChannelHandle: GetChannelHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendState_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_BLEND_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11BlendState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11BlendState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11BlendState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11BlendState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11BlendState_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11BlendState1_Impl: ::windows_core::BaseImpl + ID3D11BlendState_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut D3D11_BLEND_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11BlendState1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11BlendState);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11BlendState1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11BlendState1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11BlendState1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11BlendState1_Vtbl { base__: <ID3D11BlendState as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11Buffer_Impl: ::windows_core::BaseImpl + ID3D11Resource_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_BUFFER_DESC);
}
impl ::windows_core::Iids for ID3D11Buffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Buffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Buffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Buffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_BUFFER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Buffer_Vtbl { base__: <ID3D11Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ClassInstance_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetClassLinkage(this: &Self::This, pplinkage: *mut ::core::option::Option<ID3D11ClassLinkage>);
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_CLASS_INSTANCE_DESC);
    fn GetInstanceName(this: &Self::This, pinstancename: ::windows_core::PSTR, pbufferlength: *mut usize);
    fn GetTypeName(this: &Self::This, ptypename: ::windows_core::PSTR, pbufferlength: *mut usize);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11ClassInstance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ClassInstance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassLinkage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplinkage: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassLinkage(this, ::core::mem::transmute_copy(&pplinkage)))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        unsafe extern "system" fn GetInstanceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstancename: ::windows_core::PSTR, pbufferlength: *mut usize) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInstanceName(this, ::core::mem::transmute_copy(&pinstancename), ::core::mem::transmute_copy(&pbufferlength)))
        }
        unsafe extern "system" fn GetTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptypename: ::windows_core::PSTR, pbufferlength: *mut usize) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTypeName(this, ::core::mem::transmute_copy(&ptypename), ::core::mem::transmute_copy(&pbufferlength)))
        }
        ID3D11ClassInstance_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClassLinkage: GetClassLinkage::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetInstanceName: GetInstanceName::<Identity, Impl, OFFSET>,
            GetTypeName: GetTypeName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11ClassLinkage_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetClassInstance(this: &Self::This, pclassinstancename: &::windows_core::PCSTR, instanceindex: u32) -> ::windows_core::Result<ID3D11ClassInstance>;
    fn CreateClassInstance(this: &Self::This, pclasstypename: &::windows_core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32) -> ::windows_core::Result<ID3D11ClassInstance>;
}
impl ::windows_core::Iids for ID3D11ClassLinkage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassLinkage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ClassLinkage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassLinkage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclassinstancename: ::windows_core::PCSTR, instanceindex: u32, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassInstance(this, ::core::mem::transmute(&pclassinstancename), ::core::mem::transmute_copy(&instanceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateClassInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ClassLinkage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclasstypename: ::windows_core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateClassInstance(this, ::core::mem::transmute(&pclasstypename), ::core::mem::transmute_copy(&constantbufferoffset), ::core::mem::transmute_copy(&constantvectoroffset), ::core::mem::transmute_copy(&textureoffset), ::core::mem::transmute_copy(&sampleroffset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11ClassLinkage_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClassInstance: GetClassInstance::<Identity, Impl, OFFSET>,
            CreateClassInstance: CreateClassInstance::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11CommandList_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetContextFlags(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for ID3D11CommandList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CommandList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11CommandList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContextFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContextFlags(this))
        }
        ID3D11CommandList_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetContextFlags: GetContextFlags::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11ComputeShader_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D11ComputeShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ComputeShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ComputeShader {
    const VTABLE: Self::Vtable = { ID3D11ComputeShader_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D11Counter_Impl: ::windows_core::BaseImpl + ID3D11Asynchronous_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_COUNTER_DESC);
}
impl ::windows_core::Iids for ID3D11Counter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Asynchronous);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Counter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Counter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Counter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_COUNTER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Counter_Vtbl { base__: <ID3D11Asynchronous as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11CryptoSession_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetCryptoType(this: &Self::This, pcryptotype: *mut ::windows_core::GUID);
    fn GetDecoderProfile(this: &Self::This, pdecoderprofile: *mut ::windows_core::GUID);
    fn GetCertificateSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCertificate(this: &Self::This, certificatesize: u32, pcertificate: *mut u8) -> ::windows_core::Result<()>;
    fn GetCryptoSessionHandle(this: &Self::This, pcryptosessionhandle: *mut super::super::Foundation::HANDLE);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11CryptoSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11CryptoSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCryptoType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptotype: *mut ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCryptoType(this, ::core::mem::transmute_copy(&pcryptotype)))
        }
        unsafe extern "system" fn GetDecoderProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *mut ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDecoderProfile(this, ::core::mem::transmute_copy(&pdecoderprofile)))
        }
        unsafe extern "system" fn GetCertificateSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcertificatesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertificateSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcertificatesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCertificate(this, ::core::mem::transmute_copy(&certificatesize), ::core::mem::transmute_copy(&pcertificate)).into())
        }
        unsafe extern "system" fn GetCryptoSessionHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosessionhandle: *mut super::super::Foundation::HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCryptoSessionHandle(this, ::core::mem::transmute_copy(&pcryptosessionhandle)))
        }
        ID3D11CryptoSession_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCryptoType: GetCryptoType::<Identity, Impl, OFFSET>,
            GetDecoderProfile: GetDecoderProfile::<Identity, Impl, OFFSET>,
            GetCertificateSize: GetCertificateSize::<Identity, Impl, OFFSET>,
            GetCertificate: GetCertificate::<Identity, Impl, OFFSET>,
            GetCryptoSessionHandle: GetCryptoSessionHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D11Debug_Impl: ::windows_core::BaseImpl {
    fn SetFeatureMask(this: &Self::This, mask: u32) -> ::windows_core::Result<()>;
    fn GetFeatureMask(this: &Self::This) -> u32;
    fn SetPresentPerRenderOpDelay(this: &Self::This, milliseconds: u32) -> ::windows_core::Result<()>;
    fn GetPresentPerRenderOpDelay(this: &Self::This) -> u32;
    fn SetSwapChain(this: &Self::This, pswapchain: ::core::option::Option<&super::Dxgi::IDXGISwapChain>) -> ::windows_core::Result<()>;
    fn GetSwapChain(this: &Self::This) -> ::windows_core::Result<super::Dxgi::IDXGISwapChain>;
    fn ValidateContext(this: &Self::This, pcontext: ::core::option::Option<&ID3D11DeviceContext>) -> ::windows_core::Result<()>;
    fn ReportLiveDeviceObjects(this: &Self::This, flags: D3D11_RLDO_FLAGS) -> ::windows_core::Result<()>;
    fn ValidateContextForDispatch(this: &Self::This, pcontext: ::core::option::Option<&ID3D11DeviceContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::windows_core::Iids for ID3D11Debug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Debug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFeatureMask(this, ::core::mem::transmute_copy(&mask)).into())
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureMask(this))
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPresentPerRenderOpDelay(this, ::core::mem::transmute_copy(&milliseconds)).into())
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresentPerRenderOpDelay(this))
        }
        unsafe extern "system" fn SetSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pswapchain: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSwapChain(this, ::windows_core::from_raw_borrowed(&pswapchain)).into())
        }
        unsafe extern "system" fn GetSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSwapChain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValidateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateContext(this, ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: D3D11_RLDO_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportLiveDeviceObjects(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn ValidateContextForDispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateContextForDispatch(this, ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        ID3D11Debug_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, Impl, OFFSET>,
            ValidateContext: ValidateContext::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
            ValidateContextForDispatch: ValidateContextForDispatch::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11DepthStencilState_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_DEPTH_STENCIL_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11DepthStencilState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DepthStencilState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DepthStencilState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DepthStencilState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11DepthStencilState_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11DepthStencilView_Impl: ::windows_core::BaseImpl + ID3D11View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11DepthStencilView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11View);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DepthStencilView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DepthStencilView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DepthStencilView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11DepthStencilView_Vtbl { base__: <ID3D11View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device_Impl: ::windows_core::BaseImpl {
    fn CreateBuffer(this: &Self::This, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut ::core::option::Option<ID3D11Buffer>) -> ::windows_core::Result<()>;
    fn CreateTexture1D(this: &Self::This, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut ::core::option::Option<ID3D11Texture1D>) -> ::windows_core::Result<()>;
    fn CreateTexture2D(this: &Self::This, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::core::option::Option<ID3D11Texture2D>) -> ::windows_core::Result<()>;
    fn CreateTexture3D(this: &Self::This, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::core::option::Option<ID3D11Texture3D>) -> ::windows_core::Result<()>;
    fn CreateShaderResourceView(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::core::option::Option<ID3D11ShaderResourceView>) -> ::windows_core::Result<()>;
    fn CreateUnorderedAccessView(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows_core::Result<()>;
    fn CreateRenderTargetView(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::core::option::Option<ID3D11RenderTargetView>) -> ::windows_core::Result<()>;
    fn CreateDepthStencilView(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::core::option::Option<ID3D11DepthStencilView>) -> ::windows_core::Result<()>;
    fn CreateInputLayout(this: &Self::This, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::core::option::Option<ID3D11InputLayout>) -> ::windows_core::Result<()>;
    fn CreateVertexShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::core::option::Option<&ID3D11ClassLinkage>, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>) -> ::windows_core::Result<()>;
    fn CreateGeometryShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::core::option::Option<&ID3D11ClassLinkage>, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>) -> ::windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: ::core::option::Option<&ID3D11ClassLinkage>, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>) -> ::windows_core::Result<()>;
    fn CreatePixelShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::core::option::Option<&ID3D11ClassLinkage>, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>) -> ::windows_core::Result<()>;
    fn CreateHullShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::core::option::Option<&ID3D11ClassLinkage>, pphullshader: *mut ::core::option::Option<ID3D11HullShader>) -> ::windows_core::Result<()>;
    fn CreateDomainShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::core::option::Option<&ID3D11ClassLinkage>, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>) -> ::windows_core::Result<()>;
    fn CreateComputeShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: ::core::option::Option<&ID3D11ClassLinkage>, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>) -> ::windows_core::Result<()>;
    fn CreateClassLinkage(this: &Self::This) -> ::windows_core::Result<ID3D11ClassLinkage>;
    fn CreateBlendState(this: &Self::This, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut ::core::option::Option<ID3D11BlendState>) -> ::windows_core::Result<()>;
    fn CreateDepthStencilState(this: &Self::This, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::core::option::Option<ID3D11DepthStencilState>) -> ::windows_core::Result<()>;
    fn CreateRasterizerState(this: &Self::This, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut ::core::option::Option<ID3D11RasterizerState>) -> ::windows_core::Result<()>;
    fn CreateSamplerState(this: &Self::This, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut ::core::option::Option<ID3D11SamplerState>) -> ::windows_core::Result<()>;
    fn CreateQuery(this: &Self::This, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut ::core::option::Option<ID3D11Query>) -> ::windows_core::Result<()>;
    fn CreatePredicate(this: &Self::This, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut ::core::option::Option<ID3D11Predicate>) -> ::windows_core::Result<()>;
    fn CreateCounter(this: &Self::This, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut ::core::option::Option<ID3D11Counter>) -> ::windows_core::Result<()>;
    fn CreateDeferredContext(this: &Self::This, contextflags: u32, ppdeferredcontext: *mut ::core::option::Option<ID3D11DeviceContext>) -> ::windows_core::Result<()>;
    fn OpenSharedResource(this: &Self::This, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CheckFormatSupport(this: &Self::This, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(this: &Self::This, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows_core::Result<u32>;
    fn CheckCounterInfo(this: &Self::This, pcounterinfo: *mut D3D11_COUNTER_INFO);
    fn CheckCounter(this: &Self::This, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows_core::PSTR, pnamelength: *mut u32, szunits: ::windows_core::PSTR, punitslength: *mut u32, szdescription: ::windows_core::PSTR, pdescriptionlength: *mut u32) -> ::windows_core::Result<()>;
    fn CheckFeatureSupport(this: &Self::This, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()>;
    fn GetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, guid: *const ::windows_core::GUID, pdata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetFeatureLevel(this: &Self::This) -> super::Direct3D::D3D_FEATURE_LEVEL;
    fn GetCreationFlags(this: &Self::This) -> u32;
    fn GetDeviceRemovedReason(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetImmediateContext(this: &Self::This, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext>);
    fn SetExceptionMode(this: &Self::This, raiseflags: u32) -> ::windows_core::Result<()>;
    fn GetExceptionMode(this: &Self::This) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11Device {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Device {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBuffer(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata), ::core::mem::transmute_copy(&ppbuffer)).into())
        }
        unsafe extern "system" fn CreateTexture1D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTexture1D(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata), ::core::mem::transmute_copy(&pptexture1d)).into())
        }
        unsafe extern "system" fn CreateTexture2D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTexture2D(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata), ::core::mem::transmute_copy(&pptexture2d)).into())
        }
        unsafe extern "system" fn CreateTexture3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTexture3D(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata), ::core::mem::transmute_copy(&pptexture3d)).into())
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateShaderResourceView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppsrview)).into())
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateUnorderedAccessView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppuaview)).into())
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRenderTargetView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pprtview)).into())
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDepthStencilView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppdepthstencilview)).into())
        }
        unsafe extern "system" fn CreateInputLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInputLayout(this, ::core::mem::transmute_copy(&pinputelementdescs), ::core::mem::transmute_copy(&numelements), ::core::mem::transmute_copy(&pshaderbytecodewithinputsignature), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&ppinputlayout)).into())
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut ::core::ffi::c_void, ppvertexshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVertexShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::windows_core::from_raw_borrowed(&pclasslinkage), ::core::mem::transmute_copy(&ppvertexshader)).into())
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut ::core::ffi::c_void, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateGeometryShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::windows_core::from_raw_borrowed(&pclasslinkage), ::core::mem::transmute_copy(&ppgeometryshader)).into())
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: *mut ::core::ffi::c_void, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::CreateGeometryShaderWithStreamOutput(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&psodeclaration), ::core::mem::transmute_copy(&numentries), ::core::mem::transmute_copy(&pbufferstrides), ::core::mem::transmute_copy(&numstrides), ::core::mem::transmute_copy(&rasterizedstream), ::windows_core::from_raw_borrowed(&pclasslinkage), ::core::mem::transmute_copy(&ppgeometryshader)).into()
            })
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut ::core::ffi::c_void, pppixelshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePixelShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::windows_core::from_raw_borrowed(&pclasslinkage), ::core::mem::transmute_copy(&pppixelshader)).into())
        }
        unsafe extern "system" fn CreateHullShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut ::core::ffi::c_void, pphullshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateHullShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::windows_core::from_raw_borrowed(&pclasslinkage), ::core::mem::transmute_copy(&pphullshader)).into())
        }
        unsafe extern "system" fn CreateDomainShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut ::core::ffi::c_void, ppdomainshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDomainShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::windows_core::from_raw_borrowed(&pclasslinkage), ::core::mem::transmute_copy(&ppdomainshader)).into())
        }
        unsafe extern "system" fn CreateComputeShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut ::core::ffi::c_void, ppcomputeshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateComputeShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::windows_core::from_raw_borrowed(&pclasslinkage), ::core::mem::transmute_copy(&ppcomputeshader)).into())
        }
        unsafe extern "system" fn CreateClassLinkage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplinkage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateClassLinkage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplinkage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlendState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBlendState(this, ::core::mem::transmute_copy(&pblendstatedesc), ::core::mem::transmute_copy(&ppblendstate)).into())
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDepthStencilState(this, ::core::mem::transmute_copy(&pdepthstencildesc), ::core::mem::transmute_copy(&ppdepthstencilstate)).into())
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRasterizerState(this, ::core::mem::transmute_copy(&prasterizerdesc), ::core::mem::transmute_copy(&pprasterizerstate)).into())
        }
        unsafe extern "system" fn CreateSamplerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSamplerState(this, ::core::mem::transmute_copy(&psamplerdesc), ::core::mem::transmute_copy(&ppsamplerstate)).into())
        }
        unsafe extern "system" fn CreateQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateQuery(this, ::core::mem::transmute_copy(&pquerydesc), ::core::mem::transmute_copy(&ppquery)).into())
        }
        unsafe extern "system" fn CreatePredicate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePredicate(this, ::core::mem::transmute_copy(&ppredicatedesc), ::core::mem::transmute_copy(&pppredicate)).into())
        }
        unsafe extern "system" fn CreateCounter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCounter(this, ::core::mem::transmute_copy(&pcounterdesc), ::core::mem::transmute_copy(&ppcounter)).into())
        }
        unsafe extern "system" fn CreateDeferredContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeferredContext(this, ::core::mem::transmute_copy(&contextflags), ::core::mem::transmute_copy(&ppdeferredcontext)).into())
        }
        unsafe extern "system" fn OpenSharedResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenSharedResource(this, ::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into())
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckFormatSupport(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckMultisampleQualityLevels(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumqualitylevels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D11_COUNTER_INFO) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckCounterInfo(this, ::core::mem::transmute_copy(&pcounterinfo)))
        }
        unsafe extern "system" fn CheckCounter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows_core::PSTR, pnamelength: *mut u32, szunits: ::windows_core::PSTR, punitslength: *mut u32, szdescription: ::windows_core::PSTR, pdescriptionlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckCounter(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pnamelength), ::core::mem::transmute_copy(&szunits), ::core::mem::transmute_copy(&punitslength), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pdescriptionlength)).into())
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckFeatureSupport(this, ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into())
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&guid), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureLevel(this))
        }
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationFlags(this))
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceRemovedReason(this).into())
        }
        unsafe extern "system" fn GetImmediateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImmediateContext(this, ::core::mem::transmute_copy(&ppimmediatecontext)))
        }
        unsafe extern "system" fn SetExceptionMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExceptionMode(this, ::core::mem::transmute_copy(&raiseflags)).into())
        }
        unsafe extern "system" fn GetExceptionMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExceptionMode(this))
        }
        ID3D11Device_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateBuffer: CreateBuffer::<Identity, Impl, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, Impl, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, Impl, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, Impl, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, Impl, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, Impl, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, Impl, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, Impl, OFFSET>,
            CreateHullShader: CreateHullShader::<Identity, Impl, OFFSET>,
            CreateDomainShader: CreateDomainShader::<Identity, Impl, OFFSET>,
            CreateComputeShader: CreateComputeShader::<Identity, Impl, OFFSET>,
            CreateClassLinkage: CreateClassLinkage::<Identity, Impl, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, Impl, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, Impl, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, Impl, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, Impl, OFFSET>,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, Impl, OFFSET>,
            CreateCounter: CreateCounter::<Identity, Impl, OFFSET>,
            CreateDeferredContext: CreateDeferredContext::<Identity, Impl, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, Impl, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, Impl, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, Impl, OFFSET>,
            CheckCounter: CheckCounter::<Identity, Impl, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, Impl, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetImmediateContext: GetImmediateContext::<Identity, Impl, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, Impl, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device1_Impl: ::windows_core::BaseImpl + ID3D11Device_Impl {
    fn GetImmediateContext1(this: &Self::This, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext1>);
    fn CreateDeferredContext1(this: &Self::This, contextflags: u32, ppdeferredcontext: *mut ::core::option::Option<ID3D11DeviceContext1>) -> ::windows_core::Result<()>;
    fn CreateBlendState1(this: &Self::This, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut ::core::option::Option<ID3D11BlendState1>) -> ::windows_core::Result<()>;
    fn CreateRasterizerState1(this: &Self::This, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut ::core::option::Option<ID3D11RasterizerState1>) -> ::windows_core::Result<()>;
    fn CreateDeviceContextState(this: &Self::This, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows_core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut ::core::option::Option<ID3DDeviceContextState>) -> ::windows_core::Result<()>;
    fn OpenSharedResource1(this: &Self::This, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OpenSharedResourceByName(this: &Self::This, lpname: &::windows_core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11Device1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Device);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Device1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImmediateContext1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImmediateContext1(this, ::core::mem::transmute_copy(&ppimmediatecontext)))
        }
        unsafe extern "system" fn CreateDeferredContext1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeferredContext1(this, ::core::mem::transmute_copy(&contextflags), ::core::mem::transmute_copy(&ppdeferredcontext)).into())
        }
        unsafe extern "system" fn CreateBlendState1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBlendState1(this, ::core::mem::transmute_copy(&pblendstatedesc), ::core::mem::transmute_copy(&ppblendstate)).into())
        }
        unsafe extern "system" fn CreateRasterizerState1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRasterizerState1(this, ::core::mem::transmute_copy(&prasterizerdesc), ::core::mem::transmute_copy(&pprasterizerstate)).into())
        }
        unsafe extern "system" fn CreateDeviceContextState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const ::windows_core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeviceContextState(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pfeaturelevels), ::core::mem::transmute_copy(&featurelevels), ::core::mem::transmute_copy(&sdkversion), ::core::mem::transmute_copy(&emulatedinterface), ::core::mem::transmute_copy(&pchosenfeaturelevel), ::core::mem::transmute_copy(&ppcontextstate)).into())
        }
        unsafe extern "system" fn OpenSharedResource1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenSharedResource1(this, ::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into())
        }
        unsafe extern "system" fn OpenSharedResourceByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpname: ::windows_core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenSharedResourceByName(this, ::core::mem::transmute(&lpname), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into())
        }
        ID3D11Device1_Vtbl {
            base__: <ID3D11Device as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImmediateContext1: GetImmediateContext1::<Identity, Impl, OFFSET>,
            CreateDeferredContext1: CreateDeferredContext1::<Identity, Impl, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, Impl, OFFSET>,
            CreateRasterizerState1: CreateRasterizerState1::<Identity, Impl, OFFSET>,
            CreateDeviceContextState: CreateDeviceContextState::<Identity, Impl, OFFSET>,
            OpenSharedResource1: OpenSharedResource1::<Identity, Impl, OFFSET>,
            OpenSharedResourceByName: OpenSharedResourceByName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device2_Impl: ::windows_core::BaseImpl + ID3D11Device1_Impl {
    fn GetImmediateContext2(this: &Self::This, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext2>);
    fn CreateDeferredContext2(this: &Self::This, contextflags: u32, ppdeferredcontext: *mut ::core::option::Option<ID3D11DeviceContext2>) -> ::windows_core::Result<()>;
    fn GetResourceTiling(this: &Self::This, ptiledresource: ::core::option::Option<&ID3D11Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING);
    fn CheckMultisampleQualityLevels1(this: &Self::This, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11Device2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Device1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Device2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImmediateContext2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImmediateContext2(this, ::core::mem::transmute_copy(&ppimmediatecontext)))
        }
        unsafe extern "system" fn CreateDeferredContext2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeferredContext2(this, ::core::mem::transmute_copy(&contextflags), ::core::mem::transmute_copy(&ppdeferredcontext)).into())
        }
        unsafe extern "system" fn GetResourceTiling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptiledresource: *mut ::core::ffi::c_void, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceTiling(this, ::windows_core::from_raw_borrowed(&ptiledresource), ::core::mem::transmute_copy(&pnumtilesforentireresource), ::core::mem::transmute_copy(&ppackedmipdesc), ::core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), ::core::mem::transmute_copy(&pnumsubresourcetilings), ::core::mem::transmute_copy(&firstsubresourcetilingtoget), ::core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips)))
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32, pnumqualitylevels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckMultisampleQualityLevels1(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumqualitylevels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11Device2_Vtbl {
            base__: <ID3D11Device1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImmediateContext2: GetImmediateContext2::<Identity, Impl, OFFSET>,
            CreateDeferredContext2: CreateDeferredContext2::<Identity, Impl, OFFSET>,
            GetResourceTiling: GetResourceTiling::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels1: CheckMultisampleQualityLevels1::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device3_Impl: ::windows_core::BaseImpl + ID3D11Device2_Impl {
    fn CreateTexture2D1(this: &Self::This, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut ::core::option::Option<ID3D11Texture2D1>) -> ::windows_core::Result<()>;
    fn CreateTexture3D1(this: &Self::This, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut ::core::option::Option<ID3D11Texture3D1>) -> ::windows_core::Result<()>;
    fn CreateRasterizerState2(this: &Self::This, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut ::core::option::Option<ID3D11RasterizerState2>) -> ::windows_core::Result<()>;
    fn CreateShaderResourceView1(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut ::core::option::Option<ID3D11ShaderResourceView1>) -> ::windows_core::Result<()>;
    fn CreateUnorderedAccessView1(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut ::core::option::Option<ID3D11UnorderedAccessView1>) -> ::windows_core::Result<()>;
    fn CreateRenderTargetView1(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut ::core::option::Option<ID3D11RenderTargetView1>) -> ::windows_core::Result<()>;
    fn CreateQuery1(this: &Self::This, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut ::core::option::Option<ID3D11Query1>) -> ::windows_core::Result<()>;
    fn GetImmediateContext3(this: &Self::This, ppimmediatecontext: *mut ::core::option::Option<ID3D11DeviceContext3>);
    fn CreateDeferredContext3(this: &Self::This, contextflags: u32, ppdeferredcontext: *mut ::core::option::Option<ID3D11DeviceContext3>) -> ::windows_core::Result<()>;
    fn WriteToSubresource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ReadFromSubresource(this: &Self::This, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: ::core::option::Option<&ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11Device3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Device2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Device3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTexture2D1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTexture2D1(this, ::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&pinitialdata), ::core::mem::transmute_copy(&pptexture2d)).into())
        }
        unsafe extern "system" fn CreateTexture3D1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTexture3D1(this, ::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&pinitialdata), ::core::mem::transmute_copy(&pptexture3d)).into())
        }
        unsafe extern "system" fn CreateRasterizerState2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRasterizerState2(this, ::core::mem::transmute_copy(&prasterizerdesc), ::core::mem::transmute_copy(&pprasterizerstate)).into())
        }
        unsafe extern "system" fn CreateShaderResourceView1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateShaderResourceView1(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&ppsrview1)).into())
        }
        unsafe extern "system" fn CreateUnorderedAccessView1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateUnorderedAccessView1(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&ppuaview1)).into())
        }
        unsafe extern "system" fn CreateRenderTargetView1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRenderTargetView1(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc1), ::core::mem::transmute_copy(&pprtview1)).into())
        }
        unsafe extern "system" fn CreateQuery1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateQuery1(this, ::core::mem::transmute_copy(&pquerydesc1), ::core::mem::transmute_copy(&ppquery1)).into())
        }
        unsafe extern "system" fn GetImmediateContext3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimmediatecontext: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImmediateContext3(this, ::core::mem::transmute_copy(&ppimmediatecontext)))
        }
        unsafe extern "system" fn CreateDeferredContext3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeferredContext3(this, ::core::mem::transmute_copy(&contextflags), ::core::mem::transmute_copy(&ppdeferredcontext)).into())
        }
        unsafe extern "system" fn WriteToSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToSubresource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch)))
        }
        unsafe extern "system" fn ReadFromSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadFromSubresource(this, ::core::mem::transmute_copy(&pdstdata), ::core::mem::transmute_copy(&dstrowpitch), ::core::mem::transmute_copy(&dstdepthpitch), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox)))
        }
        ID3D11Device3_Vtbl {
            base__: <ID3D11Device2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTexture2D1: CreateTexture2D1::<Identity, Impl, OFFSET>,
            CreateTexture3D1: CreateTexture3D1::<Identity, Impl, OFFSET>,
            CreateRasterizerState2: CreateRasterizerState2::<Identity, Impl, OFFSET>,
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView1: CreateUnorderedAccessView1::<Identity, Impl, OFFSET>,
            CreateRenderTargetView1: CreateRenderTargetView1::<Identity, Impl, OFFSET>,
            CreateQuery1: CreateQuery1::<Identity, Impl, OFFSET>,
            GetImmediateContext3: GetImmediateContext3::<Identity, Impl, OFFSET>,
            CreateDeferredContext3: CreateDeferredContext3::<Identity, Impl, OFFSET>,
            WriteToSubresource: WriteToSubresource::<Identity, Impl, OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device4_Impl: ::windows_core::BaseImpl + ID3D11Device3_Impl {
    fn RegisterDeviceRemovedEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
    fn UnregisterDeviceRemoved(this: &Self::This, dwcookie: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11Device4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Device3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Device4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterDeviceRemovedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterDeviceRemovedEvent(this, ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterDeviceRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterDeviceRemoved(this, ::core::mem::transmute_copy(&dwcookie)))
        }
        ID3D11Device4_Vtbl {
            base__: <ID3D11Device3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterDeviceRemovedEvent: RegisterDeviceRemovedEvent::<Identity, Impl, OFFSET>,
            UnregisterDeviceRemoved: UnregisterDeviceRemoved::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device5_Impl: ::windows_core::BaseImpl + ID3D11Device4_Impl {
    fn OpenSharedFence(this: &Self::This, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateFence(this: &Self::This, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11Device5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Device4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Device5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenSharedFence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfence: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenSharedFence(this, ::core::mem::transmute_copy(&hfence), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppfence)).into())
        }
        unsafe extern "system" fn CreateFence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const ::windows_core::GUID, ppfence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFence(this, ::core::mem::transmute_copy(&initialvalue), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppfence)).into())
        }
        ID3D11Device5_Vtbl {
            base__: <ID3D11Device4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenSharedFence: OpenSharedFence::<Identity, Impl, OFFSET>,
            CreateFence: CreateFence::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11DeviceChild_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This, ppdevice: *mut ::core::option::Option<ID3D11Device>);
    fn GetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, guid: *const ::windows_core::GUID, pdata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D11DeviceChild {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DeviceChild {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevice(this, ::core::mem::transmute_copy(&ppdevice)))
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&guid), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        ID3D11DeviceChild_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn VSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn PSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn PSSetShader(this: &Self::This, ppixelshader: ::core::option::Option<&ID3D11PixelShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn PSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn VSSetShader(this: &Self::This, pvertexshader: ::core::option::Option<&ID3D11VertexShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DrawIndexed(this: &Self::This, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(this: &Self::This, vertexcount: u32, startvertexlocation: u32);
    fn Map(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> ::windows_core::Result<()>;
    fn Unmap(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, subresource: u32);
    fn PSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn IASetInputLayout(this: &Self::This, pinputlayout: ::core::option::Option<&ID3D11InputLayout>);
    fn IASetVertexBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D11Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(this: &Self::This, pindexbuffer: ::core::option::Option<&ID3D11Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(this: &Self::This, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(this: &Self::This, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn GSSetShader(this: &Self::This, pshader: ::core::option::Option<&ID3D11GeometryShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn IASetPrimitiveTopology(this: &Self::This, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn VSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn Begin(this: &Self::This, pasync: ::core::option::Option<&ID3D11Asynchronous>);
    fn End(this: &Self::This, pasync: ::core::option::Option<&ID3D11Asynchronous>);
    fn GetData(this: &Self::This, pasync: ::core::option::Option<&ID3D11Asynchronous>, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows_core::Result<()>;
    fn SetPredication(this: &Self::This, ppredicate: ::core::option::Option<&ID3D11Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn GSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn OMSetRenderTargets(this: &Self::This, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D11RenderTargetView>, pdepthstencilview: ::core::option::Option<&ID3D11DepthStencilView>);
    fn OMSetRenderTargetsAndUnorderedAccessViews(this: &Self::This, numrtvs: u32, pprendertargetviews: *const ::core::option::Option<ID3D11RenderTargetView>, pdepthstencilview: ::core::option::Option<&ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::core::option::Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn OMSetBlendState(this: &Self::This, pblendstate: ::core::option::Option<&ID3D11BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(this: &Self::This, pdepthstencilstate: ::core::option::Option<&ID3D11DepthStencilState>, stencilref: u32);
    fn SOSetTargets(this: &Self::This, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D11Buffer>, poffsets: *const u32);
    fn DrawAuto(this: &Self::This);
    fn DrawIndexedInstancedIndirect(this: &Self::This, pbufferforargs: ::core::option::Option<&ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn DrawInstancedIndirect(this: &Self::This, pbufferforargs: ::core::option::Option<&ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn Dispatch(this: &Self::This, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn DispatchIndirect(this: &Self::This, pbufferforargs: ::core::option::Option<&ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn RSSetState(this: &Self::This, prasterizerstate: ::core::option::Option<&ID3D11RasterizerState>);
    fn RSSetViewports(this: &Self::This, numviewports: u32, pviewports: *const D3D11_VIEWPORT);
    fn RSSetScissorRects(this: &Self::This, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(this: &Self::This, pdstresource: ::core::option::Option<&ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::core::option::Option<&ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
    fn CopyResource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D11Resource>, psrcresource: ::core::option::Option<&ID3D11Resource>);
    fn UpdateSubresource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn CopyStructureCount(this: &Self::This, pdstbuffer: ::core::option::Option<&ID3D11Buffer>, dstalignedbyteoffset: u32, psrcview: ::core::option::Option<&ID3D11UnorderedAccessView>);
    fn ClearRenderTargetView(this: &Self::This, prendertargetview: ::core::option::Option<&ID3D11RenderTargetView>, colorrgba: *const f32);
    fn ClearUnorderedAccessViewUint(this: &Self::This, punorderedaccessview: ::core::option::Option<&ID3D11UnorderedAccessView>, values: *const u32);
    fn ClearUnorderedAccessViewFloat(this: &Self::This, punorderedaccessview: ::core::option::Option<&ID3D11UnorderedAccessView>, values: *const f32);
    fn ClearDepthStencilView(this: &Self::This, pdepthstencilview: ::core::option::Option<&ID3D11DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(this: &Self::This, pshaderresourceview: ::core::option::Option<&ID3D11ShaderResourceView>);
    fn SetResourceMinLOD(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, minlod: f32);
    fn GetResourceMinLOD(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>) -> f32;
    fn ResolveSubresource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D11Resource>, dstsubresource: u32, psrcresource: ::core::option::Option<&ID3D11Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn ExecuteCommandList(this: &Self::This, pcommandlist: ::core::option::Option<&ID3D11CommandList>, restorecontextstate: super::super::Foundation::BOOL);
    fn HSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn HSSetShader(this: &Self::This, phullshader: ::core::option::Option<&ID3D11HullShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn HSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn HSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn DSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn DSSetShader(this: &Self::This, pdomainshader: ::core::option::Option<&ID3D11DomainShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn DSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn CSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D11ShaderResourceView>);
    fn CSSetUnorderedAccessViews(this: &Self::This, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const ::core::option::Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn CSSetShader(this: &Self::This, pcomputeshader: ::core::option::Option<&ID3D11ComputeShader>, ppclassinstances: *const ::core::option::Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn CSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D11SamplerState>);
    fn CSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>);
    fn VSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn PSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn PSGetShader(this: &Self::This, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn VSGetShader(this: &Self::This, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn IAGetInputLayout(this: &Self::This, ppinputlayout: *mut ::core::option::Option<ID3D11InputLayout>);
    fn IAGetVertexBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D11Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(this: &Self::This, pindexbuffer: *mut ::core::option::Option<ID3D11Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn GSGetShader(this: &Self::This, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn IAGetPrimitiveTopology(this: &Self::This, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn VSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn GetPredication(this: &Self::This, pppredicate: *mut ::core::option::Option<ID3D11Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn GSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn OMGetRenderTargets(this: &Self::This, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D11DepthStencilView>);
    fn OMGetRenderTargetsAndUnorderedAccessViews(this: &Self::This, numrtvs: u32, pprendertargetviews: *mut ::core::option::Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::core::option::Option<ID3D11UnorderedAccessView>);
    fn OMGetBlendState(this: &Self::This, ppblendstate: *mut ::core::option::Option<ID3D11BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(this: &Self::This, ppdepthstencilstate: *mut ::core::option::Option<ID3D11DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(this: &Self::This, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D11Buffer>);
    fn RSGetState(this: &Self::This, pprasterizerstate: *mut ::core::option::Option<ID3D11RasterizerState>);
    fn RSGetViewports(this: &Self::This, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT);
    fn RSGetScissorRects(this: &Self::This, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn HSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn HSGetShader(this: &Self::This, pphullshader: *mut ::core::option::Option<ID3D11HullShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn HSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn HSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn DSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn DSGetShader(this: &Self::This, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn DSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn DSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn CSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D11ShaderResourceView>);
    fn CSGetUnorderedAccessViews(this: &Self::This, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut ::core::option::Option<ID3D11UnorderedAccessView>);
    fn CSGetShader(this: &Self::This, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>, ppclassinstances: *mut ::core::option::Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn CSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D11SamplerState>);
    fn CSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>);
    fn ClearState(this: &Self::This);
    fn Flush(this: &Self::This);
    fn GetType(this: &Self::This) -> D3D11_DEVICE_CONTEXT_TYPE;
    fn GetContextFlags(this: &Self::This) -> u32;
    fn FinishCommandList(this: &Self::This, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut ::core::option::Option<ID3D11CommandList>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11DeviceContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DeviceContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn PSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppixelshader: *mut ::core::ffi::c_void, ppclassinstances: *const *mut ::core::ffi::c_void, numclassinstances: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetShader(this, ::windows_core::from_raw_borrowed(&ppixelshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances)))
        }
        unsafe extern "system" fn PSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn VSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvertexshader: *mut ::core::ffi::c_void, ppclassinstances: *const *mut ::core::ffi::c_void, numclassinstances: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetShader(this, ::windows_core::from_raw_borrowed(&pvertexshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances)))
        }
        unsafe extern "system" fn DrawIndexed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexed(this, ::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation)))
        }
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::core::mem::transmute_copy(&vertexcount), ::core::mem::transmute_copy(&startvertexlocation)))
        }
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Map(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags), ::core::mem::transmute_copy(&pmappedresource)).into())
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, subresource: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&subresource)))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn IASetInputLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputlayout: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetInputLayout(this, ::windows_core::from_raw_borrowed(&pinputlayout)))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut ::core::ffi::c_void, pstrides: *const u32, poffsets: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetVertexBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets)))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetIndexBuffer(this, ::windows_core::from_raw_borrowed(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset)))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexedInstanced(this, ::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation)))
        }
        unsafe extern "system" fn DrawInstanced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInstanced(this, ::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation)))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn GSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ppclassinstances: *const *mut ::core::ffi::c_void, numclassinstances: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetShader(this, ::windows_core::from_raw_borrowed(&pshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances)))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetPrimitiveTopology(this, ::core::mem::transmute_copy(&topology)))
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn VSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn Begin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasync: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin(this, ::windows_core::from_raw_borrowed(&pasync)))
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasync: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this, ::windows_core::from_raw_borrowed(&pasync)))
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasync: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::windows_core::from_raw_borrowed(&pasync), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&getdataflags)).into())
        }
        unsafe extern "system" fn SetPredication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppredicate: *mut ::core::ffi::c_void, predicatevalue: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPredication(this, ::windows_core::from_raw_borrowed(&ppredicate), ::core::mem::transmute_copy(&predicatevalue)))
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn GSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetRenderTargets(this, ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::windows_core::from_raw_borrowed(&pdepthstencilview)))
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *const *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const *mut ::core::ffi::c_void, puavinitialcounts: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetRenderTargetsAndUnorderedAccessViews(this, ::core::mem::transmute_copy(&numrtvs), ::core::mem::transmute_copy(&pprendertargetviews), ::windows_core::from_raw_borrowed(&pdepthstencilview), ::core::mem::transmute_copy(&uavstartslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews), ::core::mem::transmute_copy(&puavinitialcounts)))
        }
        unsafe extern "system" fn OMSetBlendState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblendstate: *mut ::core::ffi::c_void, blendfactor: *const f32, samplemask: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetBlendState(this, ::windows_core::from_raw_borrowed(&pblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&samplemask)))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: *mut ::core::ffi::c_void, stencilref: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetDepthStencilState(this, ::windows_core::from_raw_borrowed(&pdepthstencilstate), ::core::mem::transmute_copy(&stencilref)))
        }
        unsafe extern "system" fn SOSetTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut ::core::ffi::c_void, poffsets: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SOSetTargets(this, ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets)))
        }
        unsafe extern "system" fn DrawAuto<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawAuto(this))
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbufferforargs: *mut ::core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexedInstancedIndirect(this, ::windows_core::from_raw_borrowed(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs)))
        }
        unsafe extern "system" fn DrawInstancedIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbufferforargs: *mut ::core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInstancedIndirect(this, ::windows_core::from_raw_borrowed(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs)))
        }
        unsafe extern "system" fn Dispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Dispatch(this, ::core::mem::transmute_copy(&threadgroupcountx), ::core::mem::transmute_copy(&threadgroupcounty), ::core::mem::transmute_copy(&threadgroupcountz)))
        }
        unsafe extern "system" fn DispatchIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbufferforargs: *mut ::core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DispatchIndirect(this, ::windows_core::from_raw_borrowed(&pbufferforargs), ::core::mem::transmute_copy(&alignedbyteoffsetforargs)))
        }
        unsafe extern "system" fn RSSetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterizerstate: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetState(this, ::windows_core::from_raw_borrowed(&prasterizerstate)))
        }
        unsafe extern "system" fn RSSetViewports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D11_VIEWPORT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetViewports(this, ::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports)))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetScissorRects(this, ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopySubresourceRegion(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox)))
        }
        unsafe extern "system" fn CopyResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, psrcresource: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyResource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::windows_core::from_raw_borrowed(&psrcresource)))
        }
        unsafe extern "system" fn UpdateSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateSubresource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch)))
        }
        unsafe extern "system" fn CopyStructureCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstbuffer: *mut ::core::ffi::c_void, dstalignedbyteoffset: u32, psrcview: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyStructureCount(this, ::windows_core::from_raw_borrowed(&pdstbuffer), ::core::mem::transmute_copy(&dstalignedbyteoffset), ::windows_core::from_raw_borrowed(&psrcview)))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prendertargetview: *mut ::core::ffi::c_void, colorrgba: *const f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRenderTargetView(this, ::windows_core::from_raw_borrowed(&prendertargetview), ::core::mem::transmute_copy(&colorrgba)))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punorderedaccessview: *mut ::core::ffi::c_void, values: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearUnorderedAccessViewUint(this, ::windows_core::from_raw_borrowed(&punorderedaccessview), ::core::mem::transmute_copy(&values)))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punorderedaccessview: *mut ::core::ffi::c_void, values: *const f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearUnorderedAccessViewFloat(this, ::windows_core::from_raw_borrowed(&punorderedaccessview), ::core::mem::transmute_copy(&values)))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearDepthStencilView(this, ::windows_core::from_raw_borrowed(&pdepthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil)))
        }
        unsafe extern "system" fn GenerateMips<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderresourceview: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateMips(this, ::windows_core::from_raw_borrowed(&pshaderresourceview)))
        }
        unsafe extern "system" fn SetResourceMinLOD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, minlod: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResourceMinLOD(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&minlod)))
        }
        unsafe extern "system" fn GetResourceMinLOD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceMinLOD(this, ::windows_core::from_raw_borrowed(&presource)))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveSubresource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format)))
        }
        unsafe extern "system" fn ExecuteCommandList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandlist: *mut ::core::ffi::c_void, restorecontextstate: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteCommandList(this, ::windows_core::from_raw_borrowed(&pcommandlist), ::core::mem::transmute_copy(&restorecontextstate)))
        }
        unsafe extern "system" fn HSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn HSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phullshader: *mut ::core::ffi::c_void, ppclassinstances: *const *mut ::core::ffi::c_void, numclassinstances: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSSetShader(this, ::windows_core::from_raw_borrowed(&phullshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances)))
        }
        unsafe extern "system" fn HSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn HSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn DSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn DSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdomainshader: *mut ::core::ffi::c_void, ppclassinstances: *const *mut ::core::ffi::c_void, numclassinstances: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSSetShader(this, ::windows_core::from_raw_borrowed(&pdomainshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances)))
        }
        unsafe extern "system" fn DSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn DSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn CSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const *mut ::core::ffi::c_void, puavinitialcounts: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSSetUnorderedAccessViews(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews), ::core::mem::transmute_copy(&puavinitialcounts)))
        }
        unsafe extern "system" fn CSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomputeshader: *mut ::core::ffi::c_void, ppclassinstances: *const *mut ::core::ffi::c_void, numclassinstances: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSSetShader(this, ::windows_core::from_raw_borrowed(&pcomputeshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&numclassinstances)))
        }
        unsafe extern "system" fn CSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn CSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn PSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut *mut ::core::ffi::c_void, ppclassinstances: *mut *mut ::core::ffi::c_void, pnumclassinstances: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetShader(this, ::core::mem::transmute_copy(&pppixelshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances)))
        }
        unsafe extern "system" fn PSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn VSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut *mut ::core::ffi::c_void, ppclassinstances: *mut *mut ::core::ffi::c_void, pnumclassinstances: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetShader(this, ::core::mem::transmute_copy(&ppvertexshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances)))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetInputLayout(this, ::core::mem::transmute_copy(&ppinputlayout)))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut ::core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetVertexBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets)))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut *mut ::core::ffi::c_void, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetIndexBuffer(this, ::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset)))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn GSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut *mut ::core::ffi::c_void, ppclassinstances: *mut *mut ::core::ffi::c_void, pnumclassinstances: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetShader(this, ::core::mem::transmute_copy(&ppgeometryshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances)))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetPrimitiveTopology(this, ::core::mem::transmute_copy(&ptopology)))
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn VSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn GetPredication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppredicate: *mut *mut ::core::ffi::c_void, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPredication(this, ::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue)))
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn GSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut ::core::ffi::c_void, ppdepthstencilview: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMGetRenderTargets(this, ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview)))
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrtvs: u32, pprendertargetviews: *mut *mut ::core::ffi::c_void, ppdepthstencilview: *mut *mut ::core::ffi::c_void, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMGetRenderTargetsAndUnorderedAccessViews(this, ::core::mem::transmute_copy(&numrtvs), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview), ::core::mem::transmute_copy(&uavstartslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews)))
        }
        unsafe extern "system" fn OMGetBlendState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut *mut ::core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMGetBlendState(this, ::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask)))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut *mut ::core::ffi::c_void, pstencilref: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMGetDepthStencilState(this, ::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref)))
        }
        unsafe extern "system" fn SOGetTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SOGetTargets(this, ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets)))
        }
        unsafe extern "system" fn RSGetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSGetState(this, ::core::mem::transmute_copy(&pprasterizerstate)))
        }
        unsafe extern "system" fn RSGetViewports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSGetViewports(this, ::core::mem::transmute_copy(&pnumviewports), ::core::mem::transmute_copy(&pviewports)))
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSGetScissorRects(this, ::core::mem::transmute_copy(&pnumrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn HSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn HSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphullshader: *mut *mut ::core::ffi::c_void, ppclassinstances: *mut *mut ::core::ffi::c_void, pnumclassinstances: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSGetShader(this, ::core::mem::transmute_copy(&pphullshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances)))
        }
        unsafe extern "system" fn HSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn HSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn DSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn DSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdomainshader: *mut *mut ::core::ffi::c_void, ppclassinstances: *mut *mut ::core::ffi::c_void, pnumclassinstances: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSGetShader(this, ::core::mem::transmute_copy(&ppdomainshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances)))
        }
        unsafe extern "system" fn DSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn DSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn CSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSGetUnorderedAccessViews(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numuavs), ::core::mem::transmute_copy(&ppunorderedaccessviews)))
        }
        unsafe extern "system" fn CSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomputeshader: *mut *mut ::core::ffi::c_void, ppclassinstances: *mut *mut ::core::ffi::c_void, pnumclassinstances: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSGetShader(this, ::core::mem::transmute_copy(&ppcomputeshader), ::core::mem::transmute_copy(&ppclassinstances), ::core::mem::transmute_copy(&pnumclassinstances)))
        }
        unsafe extern "system" fn CSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn CSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn ClearState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearState(this))
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this))
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this))
        }
        unsafe extern "system" fn GetContextFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContextFlags(this))
        }
        unsafe extern "system" fn FinishCommandList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishCommandList(this, ::core::mem::transmute_copy(&restoredeferredcontextstate), ::core::mem::transmute_copy(&ppcommandlist)).into())
        }
        ID3D11DeviceContext_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, Impl, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, Impl, OFFSET>,
            PSSetShader: PSSetShader::<Identity, Impl, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, Impl, OFFSET>,
            VSSetShader: VSSetShader::<Identity, Impl, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, Impl, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, Impl, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, Impl, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, Impl, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, Impl, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, Impl, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, Impl, OFFSET>,
            GSSetShader: GSSetShader::<Identity, Impl, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, Impl, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, Impl, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews: OMSetRenderTargetsAndUnorderedAccessViews::<Identity, Impl, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, Impl, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            DrawAuto: DrawAuto::<Identity, Impl, OFFSET>,
            DrawIndexedInstancedIndirect: DrawIndexedInstancedIndirect::<Identity, Impl, OFFSET>,
            DrawInstancedIndirect: DrawInstancedIndirect::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
            DispatchIndirect: DispatchIndirect::<Identity, Impl, OFFSET>,
            RSSetState: RSSetState::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, Impl, OFFSET>,
            CopyStructureCount: CopyStructureCount::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            GenerateMips: GenerateMips::<Identity, Impl, OFFSET>,
            SetResourceMinLOD: SetResourceMinLOD::<Identity, Impl, OFFSET>,
            GetResourceMinLOD: GetResourceMinLOD::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
            ExecuteCommandList: ExecuteCommandList::<Identity, Impl, OFFSET>,
            HSSetShaderResources: HSSetShaderResources::<Identity, Impl, OFFSET>,
            HSSetShader: HSSetShader::<Identity, Impl, OFFSET>,
            HSSetSamplers: HSSetSamplers::<Identity, Impl, OFFSET>,
            HSSetConstantBuffers: HSSetConstantBuffers::<Identity, Impl, OFFSET>,
            DSSetShaderResources: DSSetShaderResources::<Identity, Impl, OFFSET>,
            DSSetShader: DSSetShader::<Identity, Impl, OFFSET>,
            DSSetSamplers: DSSetSamplers::<Identity, Impl, OFFSET>,
            DSSetConstantBuffers: DSSetConstantBuffers::<Identity, Impl, OFFSET>,
            CSSetShaderResources: CSSetShaderResources::<Identity, Impl, OFFSET>,
            CSSetUnorderedAccessViews: CSSetUnorderedAccessViews::<Identity, Impl, OFFSET>,
            CSSetShader: CSSetShader::<Identity, Impl, OFFSET>,
            CSSetSamplers: CSSetSamplers::<Identity, Impl, OFFSET>,
            CSSetConstantBuffers: CSSetConstantBuffers::<Identity, Impl, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, Impl, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, Impl, OFFSET>,
            PSGetShader: PSGetShader::<Identity, Impl, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, Impl, OFFSET>,
            VSGetShader: VSGetShader::<Identity, Impl, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, Impl, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, Impl, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, Impl, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, Impl, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, Impl, OFFSET>,
            GSGetShader: GSGetShader::<Identity, Impl, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, Impl, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, Impl, OFFSET>,
            GetPredication: GetPredication::<Identity, Impl, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, Impl, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, Impl, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, Impl, OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews: OMGetRenderTargetsAndUnorderedAccessViews::<Identity, Impl, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, Impl, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, Impl, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, Impl, OFFSET>,
            RSGetState: RSGetState::<Identity, Impl, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, Impl, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, Impl, OFFSET>,
            HSGetShaderResources: HSGetShaderResources::<Identity, Impl, OFFSET>,
            HSGetShader: HSGetShader::<Identity, Impl, OFFSET>,
            HSGetSamplers: HSGetSamplers::<Identity, Impl, OFFSET>,
            HSGetConstantBuffers: HSGetConstantBuffers::<Identity, Impl, OFFSET>,
            DSGetShaderResources: DSGetShaderResources::<Identity, Impl, OFFSET>,
            DSGetShader: DSGetShader::<Identity, Impl, OFFSET>,
            DSGetSamplers: DSGetSamplers::<Identity, Impl, OFFSET>,
            DSGetConstantBuffers: DSGetConstantBuffers::<Identity, Impl, OFFSET>,
            CSGetShaderResources: CSGetShaderResources::<Identity, Impl, OFFSET>,
            CSGetUnorderedAccessViews: CSGetUnorderedAccessViews::<Identity, Impl, OFFSET>,
            CSGetShader: CSGetShader::<Identity, Impl, OFFSET>,
            CSGetSamplers: CSGetSamplers::<Identity, Impl, OFFSET>,
            CSGetConstantBuffers: CSGetConstantBuffers::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetContextFlags: GetContextFlags::<Identity, Impl, OFFSET>,
            FinishCommandList: FinishCommandList::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext1_Impl: ::windows_core::BaseImpl + ID3D11DeviceContext_Impl {
    fn CopySubresourceRegion1(this: &Self::This, pdstresource: ::core::option::Option<&ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::core::option::Option<&ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32);
    fn UpdateSubresource1(this: &Self::This, pdstresource: ::core::option::Option<&ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32);
    fn DiscardResource(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>);
    fn DiscardView(this: &Self::This, presourceview: ::core::option::Option<&ID3D11View>);
    fn VSSetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn HSSetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn DSSetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn GSSetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn PSSetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn CSSetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn VSGetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn HSGetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn DSGetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn GSGetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn PSGetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn CSGetConstantBuffers1(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn SwapDeviceContextState(this: &Self::This, pstate: ::core::option::Option<&ID3DDeviceContextState>, pppreviousstate: *mut ::core::option::Option<ID3DDeviceContextState>);
    fn ClearView(this: &Self::This, pview: ::core::option::Option<&ID3D11View>, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32);
    fn DiscardView1(this: &Self::This, presourceview: ::core::option::Option<&ID3D11View>, prects: *const super::super::Foundation::RECT, numrects: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11DeviceContext1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceContext);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DeviceContext1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CopySubresourceRegion1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopySubresourceRegion1(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox), ::core::mem::transmute_copy(&copyflags)))
        }
        unsafe extern "system" fn UpdateSubresource1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateSubresource1(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch), ::core::mem::transmute_copy(&copyflags)))
        }
        unsafe extern "system" fn DiscardResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardResource(this, ::windows_core::from_raw_borrowed(&presource)))
        }
        unsafe extern "system" fn DiscardView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourceview: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardView(this, ::windows_core::from_raw_borrowed(&presourceview)))
        }
        unsafe extern "system" fn VSSetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn HSSetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSSetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn DSSetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSSetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn GSSetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn PSSetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn CSSetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSSetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn VSGetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn HSGetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HSGetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn DSGetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DSGetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn GSGetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn PSGetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn CSGetConstantBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CSGetConstantBuffers1(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers), ::core::mem::transmute_copy(&pfirstconstant), ::core::mem::transmute_copy(&pnumconstants)))
        }
        unsafe extern "system" fn SwapDeviceContextState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut ::core::ffi::c_void, pppreviousstate: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwapDeviceContextState(this, ::windows_core::from_raw_borrowed(&pstate), ::core::mem::transmute_copy(&pppreviousstate)))
        }
        unsafe extern "system" fn ClearView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pview: *mut ::core::ffi::c_void, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearView(this, ::windows_core::from_raw_borrowed(&pview), ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&numrects)))
        }
        unsafe extern "system" fn DiscardView1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourceview: *mut ::core::ffi::c_void, prects: *const super::super::Foundation::RECT, numrects: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardView1(this, ::windows_core::from_raw_borrowed(&presourceview), ::core::mem::transmute_copy(&prects), ::core::mem::transmute_copy(&numrects)))
        }
        ID3D11DeviceContext1_Vtbl {
            base__: <ID3D11DeviceContext as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CopySubresourceRegion1: CopySubresourceRegion1::<Identity, Impl, OFFSET>,
            UpdateSubresource1: UpdateSubresource1::<Identity, Impl, OFFSET>,
            DiscardResource: DiscardResource::<Identity, Impl, OFFSET>,
            DiscardView: DiscardView::<Identity, Impl, OFFSET>,
            VSSetConstantBuffers1: VSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            HSSetConstantBuffers1: HSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            DSSetConstantBuffers1: DSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            GSSetConstantBuffers1: GSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            PSSetConstantBuffers1: PSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            CSSetConstantBuffers1: CSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            VSGetConstantBuffers1: VSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            HSGetConstantBuffers1: HSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            DSGetConstantBuffers1: DSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            GSGetConstantBuffers1: GSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            PSGetConstantBuffers1: PSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            CSGetConstantBuffers1: CSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            SwapDeviceContextState: SwapDeviceContextState::<Identity, Impl, OFFSET>,
            ClearView: ClearView::<Identity, Impl, OFFSET>,
            DiscardView1: DiscardView1::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext2_Impl: ::windows_core::BaseImpl + ID3D11DeviceContext1_Impl {
    fn UpdateTileMappings(this: &Self::This, ptiledresource: ::core::option::Option<&ID3D11Resource>, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: ::core::option::Option<&ID3D11Buffer>, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows_core::Result<()>;
    fn CopyTileMappings(this: &Self::This, pdesttiledresource: ::core::option::Option<&ID3D11Resource>, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: ::core::option::Option<&ID3D11Resource>, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows_core::Result<()>;
    fn CopyTiles(this: &Self::This, ptiledresource: ::core::option::Option<&ID3D11Resource>, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: ::core::option::Option<&ID3D11Buffer>, bufferstartoffsetinbytes: u64, flags: u32);
    fn UpdateTiles(this: &Self::This, pdesttiledresource: ::core::option::Option<&ID3D11Resource>, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32);
    fn ResizeTilePool(this: &Self::This, ptilepool: ::core::option::Option<&ID3D11Buffer>, newsizeinbytes: u64) -> ::windows_core::Result<()>;
    fn TiledResourceBarrier(this: &Self::This, ptiledresourceorviewaccessbeforebarrier: ::core::option::Option<&ID3D11DeviceChild>, ptiledresourceorviewaccessafterbarrier: ::core::option::Option<&ID3D11DeviceChild>);
    fn IsAnnotationEnabled(this: &Self::This) -> super::super::Foundation::BOOL;
    fn SetMarkerInt(this: &Self::This, plabel: &::windows_core::PCWSTR, data: i32);
    fn BeginEventInt(this: &Self::This, plabel: &::windows_core::PCWSTR, data: i32);
    fn EndEvent(this: &Self::This);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11DeviceContext2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceContext1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DeviceContext2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateTileMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptiledresource: *mut ::core::ffi::c_void, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: *mut ::core::ffi::c_void, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::UpdateTileMappings(
                    this,
                    ::windows_core::from_raw_borrowed(&ptiledresource),
                    ::core::mem::transmute_copy(&numtiledresourceregions),
                    ::core::mem::transmute_copy(&ptiledresourceregionstartcoordinates),
                    ::core::mem::transmute_copy(&ptiledresourceregionsizes),
                    ::windows_core::from_raw_borrowed(&ptilepool),
                    ::core::mem::transmute_copy(&numranges),
                    ::core::mem::transmute_copy(&prangeflags),
                    ::core::mem::transmute_copy(&ptilepoolstartoffsets),
                    ::core::mem::transmute_copy(&prangetilecounts),
                    ::core::mem::transmute_copy(&flags),
                )
                .into()
            })
        }
        unsafe extern "system" fn CopyTileMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesttiledresource: *mut ::core::ffi::c_void, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: *mut ::core::ffi::c_void, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTileMappings(this, ::windows_core::from_raw_borrowed(&pdesttiledresource), ::core::mem::transmute_copy(&pdestregionstartcoordinate), ::windows_core::from_raw_borrowed(&psourcetiledresource), ::core::mem::transmute_copy(&psourceregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn CopyTiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptiledresource: *mut ::core::ffi::c_void, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: *mut ::core::ffi::c_void, bufferstartoffsetinbytes: u64, flags: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTiles(this, ::windows_core::from_raw_borrowed(&ptiledresource), ::core::mem::transmute_copy(&ptileregionstartcoordinate), ::core::mem::transmute_copy(&ptileregionsize), ::windows_core::from_raw_borrowed(&pbuffer), ::core::mem::transmute_copy(&bufferstartoffsetinbytes), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn UpdateTiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesttiledresource: *mut ::core::ffi::c_void, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateTiles(this, ::windows_core::from_raw_borrowed(&pdesttiledresource), ::core::mem::transmute_copy(&pdesttileregionstartcoordinate), ::core::mem::transmute_copy(&pdesttileregionsize), ::core::mem::transmute_copy(&psourcetiledata), ::core::mem::transmute_copy(&flags)))
        }
        unsafe extern "system" fn ResizeTilePool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptilepool: *mut ::core::ffi::c_void, newsizeinbytes: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResizeTilePool(this, ::windows_core::from_raw_borrowed(&ptilepool), ::core::mem::transmute_copy(&newsizeinbytes)).into())
        }
        unsafe extern "system" fn TiledResourceBarrier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptiledresourceorviewaccessbeforebarrier: *mut ::core::ffi::c_void, ptiledresourceorviewaccessafterbarrier: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TiledResourceBarrier(this, ::windows_core::from_raw_borrowed(&ptiledresourceorviewaccessbeforebarrier), ::windows_core::from_raw_borrowed(&ptiledresourceorviewaccessafterbarrier)))
        }
        unsafe extern "system" fn IsAnnotationEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsAnnotationEnabled(this))
        }
        unsafe extern "system" fn SetMarkerInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plabel: ::windows_core::PCWSTR, data: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMarkerInt(this, ::core::mem::transmute(&plabel), ::core::mem::transmute_copy(&data)))
        }
        unsafe extern "system" fn BeginEventInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plabel: ::windows_core::PCWSTR, data: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEventInt(this, ::core::mem::transmute(&plabel), ::core::mem::transmute_copy(&data)))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEvent(this))
        }
        ID3D11DeviceContext2_Vtbl {
            base__: <ID3D11DeviceContext1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateTileMappings: UpdateTileMappings::<Identity, Impl, OFFSET>,
            CopyTileMappings: CopyTileMappings::<Identity, Impl, OFFSET>,
            CopyTiles: CopyTiles::<Identity, Impl, OFFSET>,
            UpdateTiles: UpdateTiles::<Identity, Impl, OFFSET>,
            ResizeTilePool: ResizeTilePool::<Identity, Impl, OFFSET>,
            TiledResourceBarrier: TiledResourceBarrier::<Identity, Impl, OFFSET>,
            IsAnnotationEnabled: IsAnnotationEnabled::<Identity, Impl, OFFSET>,
            SetMarkerInt: SetMarkerInt::<Identity, Impl, OFFSET>,
            BeginEventInt: BeginEventInt::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext3_Impl: ::windows_core::BaseImpl + ID3D11DeviceContext2_Impl {
    fn Flush1(this: &Self::This, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE);
    fn SetHardwareProtectionState(this: &Self::This, hwprotectionenable: super::super::Foundation::BOOL);
    fn GetHardwareProtectionState(this: &Self::This, phwprotectionenable: *mut super::super::Foundation::BOOL);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11DeviceContext3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceContext2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DeviceContext3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Flush1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush1(this, ::core::mem::transmute_copy(&contexttype), ::core::mem::transmute_copy(&hevent)))
        }
        unsafe extern "system" fn SetHardwareProtectionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwprotectionenable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHardwareProtectionState(this, ::core::mem::transmute_copy(&hwprotectionenable)))
        }
        unsafe extern "system" fn GetHardwareProtectionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwprotectionenable: *mut super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHardwareProtectionState(this, ::core::mem::transmute_copy(&phwprotectionenable)))
        }
        ID3D11DeviceContext3_Vtbl {
            base__: <ID3D11DeviceContext2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Flush1: Flush1::<Identity, Impl, OFFSET>,
            SetHardwareProtectionState: SetHardwareProtectionState::<Identity, Impl, OFFSET>,
            GetHardwareProtectionState: GetHardwareProtectionState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext4_Impl: ::windows_core::BaseImpl + ID3D11DeviceContext3_Impl {
    fn Signal(this: &Self::This, pfence: ::core::option::Option<&ID3D11Fence>, value: u64) -> ::windows_core::Result<()>;
    fn Wait(this: &Self::This, pfence: ::core::option::Option<&ID3D11Fence>, value: u64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11DeviceContext4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceContext3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DeviceContext4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Signal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Signal(this, ::windows_core::from_raw_borrowed(&pfence), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfence: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::windows_core::from_raw_borrowed(&pfence), ::core::mem::transmute_copy(&value)).into())
        }
        ID3D11DeviceContext4_Vtbl {
            base__: <ID3D11DeviceContext3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Signal: Signal::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11DomainShader_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D11DomainShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11DomainShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11DomainShader {
    const VTABLE: Self::Vtable = { ID3D11DomainShader_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait ID3D11Fence_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn CreateSharedHandle(this: &Self::This, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetCompletedValue(this: &Self::This) -> u64;
    fn SetEventOnCompletion(this: &Self::This, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::Iids for ID3D11Fence {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Fence {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSharedHandle(this, ::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&dwaccess), ::core::mem::transmute(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCompletedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompletedValue(this))
        }
        unsafe extern "system" fn SetEventOnCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventOnCompletion(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&hevent)).into())
        }
        ID3D11Fence_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            GetCompletedValue: GetCompletedValue::<Identity, Impl, OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11FunctionLinkingGraph_Impl: ::windows_core::BaseImpl {
    fn CreateModuleInstance(this: &Self::This, ppmoduleinstance: *mut ::core::option::Option<ID3D11ModuleInstance>, pperrorbuffer: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows_core::Result<()>;
    fn SetInputSignature(this: &Self::This, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32) -> ::windows_core::Result<ID3D11LinkingNode>;
    fn SetOutputSignature(this: &Self::This, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32) -> ::windows_core::Result<ID3D11LinkingNode>;
    fn CallFunction(this: &Self::This, pmoduleinstancenamespace: &::windows_core::PCSTR, pmodulewithfunctionprototype: ::core::option::Option<&ID3D11Module>, pfunctionname: &::windows_core::PCSTR) -> ::windows_core::Result<ID3D11LinkingNode>;
    fn PassValue(this: &Self::This, psrcnode: ::core::option::Option<&ID3D11LinkingNode>, srcparameterindex: i32, pdstnode: ::core::option::Option<&ID3D11LinkingNode>, dstparameterindex: i32) -> ::windows_core::Result<()>;
    fn PassValueWithSwizzle(this: &Self::This, psrcnode: ::core::option::Option<&ID3D11LinkingNode>, srcparameterindex: i32, psrcswizzle: &::windows_core::PCSTR, pdstnode: ::core::option::Option<&ID3D11LinkingNode>, dstparameterindex: i32, pdstswizzle: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn GetLastError(this: &Self::This, pperrorbuffer: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows_core::Result<()>;
    fn GenerateHlsl(this: &Self::This, uflags: u32) -> ::windows_core::Result<super::Direct3D::ID3DBlob>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for ID3D11FunctionLinkingGraph {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11FunctionLinkingGraph {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateModuleInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmoduleinstance: *mut *mut ::core::ffi::c_void, pperrorbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateModuleInstance(this, ::core::mem::transmute_copy(&ppmoduleinstance), ::core::mem::transmute_copy(&pperrorbuffer)).into())
        }
        unsafe extern "system" fn SetInputSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32, ppinputnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetInputSignature(this, ::core::mem::transmute_copy(&pinputparameters), ::core::mem::transmute_copy(&cinputparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinputnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutputSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32, ppoutputnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetOutputSignature(this, ::core::mem::transmute_copy(&poutputparameters), ::core::mem::transmute_copy(&coutputparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutputnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmoduleinstancenamespace: ::windows_core::PCSTR, pmodulewithfunctionprototype: *mut ::core::ffi::c_void, pfunctionname: ::windows_core::PCSTR, ppcallnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallFunction(this, ::core::mem::transmute(&pmoduleinstancenamespace), ::windows_core::from_raw_borrowed(&pmodulewithfunctionprototype), ::core::mem::transmute(&pfunctionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PassValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrcnode: *mut ::core::ffi::c_void, srcparameterindex: i32, pdstnode: *mut ::core::ffi::c_void, dstparameterindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PassValue(this, ::windows_core::from_raw_borrowed(&psrcnode), ::core::mem::transmute_copy(&srcparameterindex), ::windows_core::from_raw_borrowed(&pdstnode), ::core::mem::transmute_copy(&dstparameterindex)).into())
        }
        unsafe extern "system" fn PassValueWithSwizzle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrcnode: *mut ::core::ffi::c_void, srcparameterindex: i32, psrcswizzle: ::windows_core::PCSTR, pdstnode: *mut ::core::ffi::c_void, dstparameterindex: i32, pdstswizzle: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PassValueWithSwizzle(this, ::windows_core::from_raw_borrowed(&psrcnode), ::core::mem::transmute_copy(&srcparameterindex), ::core::mem::transmute(&psrcswizzle), ::windows_core::from_raw_borrowed(&pdstnode), ::core::mem::transmute_copy(&dstparameterindex), ::core::mem::transmute(&pdstswizzle)).into())
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperrorbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastError(this, ::core::mem::transmute_copy(&pperrorbuffer)).into())
        }
        unsafe extern "system" fn GenerateHlsl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uflags: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateHlsl(this, ::core::mem::transmute_copy(&uflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11FunctionLinkingGraph_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateModuleInstance: CreateModuleInstance::<Identity, Impl, OFFSET>,
            SetInputSignature: SetInputSignature::<Identity, Impl, OFFSET>,
            SetOutputSignature: SetOutputSignature::<Identity, Impl, OFFSET>,
            CallFunction: CallFunction::<Identity, Impl, OFFSET>,
            PassValue: PassValue::<Identity, Impl, OFFSET>,
            PassValueWithSwizzle: PassValueWithSwizzle::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GenerateHlsl: GenerateHlsl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11FunctionParameterReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_PARAMETER_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionParameterReflection_Vtbl {
    pub const fn new<Impl: ID3D11FunctionParameterReflection_Impl>() -> ID3D11FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionParameterReflection_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_PARAMETER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        Self { GetDesc: GetDesc::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D11FunctionParameterReflection_ImplVtbl<T: ID3D11FunctionParameterReflection_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D11FunctionParameterReflection_Impl> ID3D11FunctionParameterReflection_ImplVtbl<T> {
    const VTABLE: ID3D11FunctionParameterReflection_Vtbl = ID3D11FunctionParameterReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionParameterReflection {
    pub fn new<'a, T: ID3D11FunctionParameterReflection_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D11FunctionParameterReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11FunctionReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_FUNCTION_DESC) -> ::windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &::windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetFunctionParameter(&self, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionReflection_Vtbl {
    pub const fn new<Impl: ID3D11FunctionReflection_Impl>() -> ID3D11FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_FUNCTION_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, bufferindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetConstantBufferByIndex(this, ::core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetConstantBufferByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetResourceBindingDesc(this, ::core::mem::transmute_copy(&resourceindex), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetResourceBindingDescByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D11FunctionReflection_Impl>(this: *mut ::core::ffi::c_void, parameterindex: i32) -> ::core::option::Option<ID3D11FunctionParameterReflection> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetFunctionParameter(this, ::core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl>,
            GetVariableByName: GetVariableByName::<Impl>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl>,
            GetFunctionParameter: GetFunctionParameter::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
struct ID3D11FunctionReflection_ImplVtbl<T: ID3D11FunctionReflection_Impl>(::std::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl<T: ID3D11FunctionReflection_Impl> ID3D11FunctionReflection_ImplVtbl<T> {
    const VTABLE: ID3D11FunctionReflection_Vtbl = ID3D11FunctionReflection_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D11FunctionReflection {
    pub fn new<'a, T: ID3D11FunctionReflection_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D11FunctionReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait ID3D11GeometryShader_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D11GeometryShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11GeometryShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11GeometryShader {
    const VTABLE: Self::Vtable = { ID3D11GeometryShader_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D11HullShader_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D11HullShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11HullShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11HullShader {
    const VTABLE: Self::Vtable = { ID3D11HullShader_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11InfoQueue_Impl: ::windows_core::BaseImpl {
    fn SetMessageCountLimit(this: &Self::This, messagecountlimit: u64) -> ::windows_core::Result<()>;
    fn ClearStoredMessages(this: &Self::This);
    fn GetMessage(this: &Self::This, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(this: &Self::This) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(this: &Self::This) -> u64;
    fn GetNumStoredMessages(this: &Self::This) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(this: &Self::This) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(this: &Self::This) -> u64;
    fn GetMessageCountLimit(this: &Self::This) -> u64;
    fn AddStorageFilterEntries(this: &Self::This, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetStorageFilter(this: &Self::This, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearStorageFilter(this: &Self::This);
    fn PushEmptyStorageFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushCopyOfStorageFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushStorageFilter(this: &Self::This, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopStorageFilter(this: &Self::This);
    fn GetStorageFilterStackSize(this: &Self::This) -> u32;
    fn AddRetrievalFilterEntries(this: &Self::This, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetRetrievalFilter(this: &Self::This, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearRetrievalFilter(this: &Self::This);
    fn PushEmptyRetrievalFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushRetrievalFilter(this: &Self::This, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopRetrievalFilter(this: &Self::This);
    fn GetRetrievalFilterStackSize(this: &Self::This) -> u32;
    fn AddMessage(this: &Self::This, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn AddApplicationMessage(this: &Self::This, severity: D3D11_MESSAGE_SEVERITY, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetBreakOnCategory(this: &Self::This, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnSeverity(this: &Self::This, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnID(this: &Self::This, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBreakOnCategory(this: &Self::This, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(this: &Self::This, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(this: &Self::This, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(this: &Self::This, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11InfoQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11InfoQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageCountLimit(this, ::core::mem::transmute_copy(&messagecountlimit)).into())
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStoredMessages(this))
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessage(this, ::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into())
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesAllowedByStorageFilter(this))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDeniedByStorageFilter(this))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessages(this))
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessagesAllowedByRetrievalFilter(this))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDiscardedByMessageCountLimit(this))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessageCountLimit(this))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStorageFilterEntries(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilter(this, ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStorageFilter(this))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyStorageFilter(this).into())
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfStorageFilter(this).into())
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushStorageFilter(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopStorageFilter(this))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilterStackSize(this))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRetrievalFilterEntries(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilter(this, ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRetrievalFilter(this))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyRetrievalFilter(this).into())
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfRetrievalFilter(this).into())
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushRetrievalFilter(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopRetrievalFilter(this))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilterStackSize(this))
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMessage(this, ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddApplicationMessage(this, ::core::mem::transmute_copy(&severity), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnCategory(this, ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnSeverity(this, ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnID(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnCategory(this, ::core::mem::transmute_copy(&category)))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnSeverity(this, ::core::mem::transmute_copy(&severity)))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnID(this, ::core::mem::transmute_copy(&id)))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMuteDebugOutput(this, ::core::mem::transmute_copy(&bmute)))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMuteDebugOutput(this))
        }
        ID3D11InfoQueue_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
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
pub trait ID3D11InputLayout_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D11InputLayout {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11InputLayout_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11InputLayout {
    const VTABLE: Self::Vtable = { ID3D11InputLayout_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D11LibraryReflection_Impl: ::windows_core::BaseImpl {
    fn GetDesc(this: &Self::This) -> ::windows_core::Result<D3D11_LIBRARY_DESC>;
    fn GetFunctionByIndex(this: &Self::This, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection>;
}
impl ::windows_core::Iids for ID3D11LibraryReflection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11LibraryReflection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11LibraryReflection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11LibraryReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_LIBRARY_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDesc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11LibraryReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionindex: i32) -> ::core::option::Option<ID3D11FunctionReflection> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionByIndex(this, ::core::mem::transmute_copy(&functionindex)))
        }
        ID3D11LibraryReflection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11Linker_Impl: ::windows_core::BaseImpl {
    fn Link(this: &Self::This, pentry: ::core::option::Option<&ID3D11ModuleInstance>, pentryname: &::windows_core::PCSTR, ptargetname: &::windows_core::PCSTR, uflags: u32, ppshaderblob: *mut ::core::option::Option<super::Direct3D::ID3DBlob>, pperrorbuffer: *mut ::core::option::Option<super::Direct3D::ID3DBlob>) -> ::windows_core::Result<()>;
    fn UseLibrary(this: &Self::This, plibrarymi: ::core::option::Option<&ID3D11ModuleInstance>) -> ::windows_core::Result<()>;
    fn AddClipPlaneFromCBuffer(this: &Self::This, ucbufferslot: u32, ucbufferentry: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for ID3D11Linker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Linker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pentry: *mut ::core::ffi::c_void, pentryname: ::windows_core::PCSTR, ptargetname: ::windows_core::PCSTR, uflags: u32, ppshaderblob: *mut *mut ::core::ffi::c_void, pperrorbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Link(this, ::windows_core::from_raw_borrowed(&pentry), ::core::mem::transmute(&pentryname), ::core::mem::transmute(&ptargetname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ppshaderblob), ::core::mem::transmute_copy(&pperrorbuffer)).into())
        }
        unsafe extern "system" fn UseLibrary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plibrarymi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UseLibrary(this, ::windows_core::from_raw_borrowed(&plibrarymi)).into())
        }
        unsafe extern "system" fn AddClipPlaneFromCBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ucbufferslot: u32, ucbufferentry: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddClipPlaneFromCBuffer(this, ::core::mem::transmute_copy(&ucbufferslot), ::core::mem::transmute_copy(&ucbufferentry)).into())
        }
        ID3D11Linker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Link: Link::<Identity, Impl, OFFSET>,
            UseLibrary: UseLibrary::<Identity, Impl, OFFSET>,
            AddClipPlaneFromCBuffer: AddClipPlaneFromCBuffer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11LinkingNode_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ID3D11LinkingNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11LinkingNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11LinkingNode {
    const VTABLE: Self::Vtable = { ID3D11LinkingNode_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D11Module_Impl: ::windows_core::BaseImpl {
    fn CreateInstance(this: &Self::This, pnamespace: &::windows_core::PCSTR) -> ::windows_core::Result<ID3D11ModuleInstance>;
}
impl ::windows_core::Iids for ID3D11Module {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Module_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Module {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Module_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows_core::PCSTR, ppmoduleinstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this, ::core::mem::transmute(&pnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmoduleinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11Module_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11ModuleInstance_Impl: ::windows_core::BaseImpl {
    fn BindConstantBuffer(this: &Self::This, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows_core::Result<()>;
    fn BindConstantBufferByName(this: &Self::This, pname: &::windows_core::PCSTR, udstslot: u32, cbdstoffset: u32) -> ::windows_core::Result<()>;
    fn BindResource(this: &Self::This, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows_core::Result<()>;
    fn BindResourceByName(this: &Self::This, pname: &::windows_core::PCSTR, udstslot: u32, ucount: u32) -> ::windows_core::Result<()>;
    fn BindSampler(this: &Self::This, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows_core::Result<()>;
    fn BindSamplerByName(this: &Self::This, pname: &::windows_core::PCSTR, udstslot: u32, ucount: u32) -> ::windows_core::Result<()>;
    fn BindUnorderedAccessView(this: &Self::This, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows_core::Result<()>;
    fn BindUnorderedAccessViewByName(this: &Self::This, pname: &::windows_core::PCSTR, udstslot: u32, ucount: u32) -> ::windows_core::Result<()>;
    fn BindResourceAsUnorderedAccessView(this: &Self::This, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows_core::Result<()>;
    fn BindResourceAsUnorderedAccessViewByName(this: &Self::This, psrvname: &::windows_core::PCSTR, udstuavslot: u32, ucount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D11ModuleInstance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ModuleInstance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindConstantBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindConstantBuffer(this, ::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&cbdstoffset)).into())
        }
        unsafe extern "system" fn BindConstantBufferByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCSTR, udstslot: u32, cbdstoffset: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindConstantBufferByName(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&cbdstoffset)).into())
        }
        unsafe extern "system" fn BindResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindResource(this, ::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        unsafe extern "system" fn BindResourceByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCSTR, udstslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindResourceByName(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        unsafe extern "system" fn BindSampler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindSampler(this, ::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        unsafe extern "system" fn BindSamplerByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCSTR, udstslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindSamplerByName(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        unsafe extern "system" fn BindUnorderedAccessView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindUnorderedAccessView(this, ::core::mem::transmute_copy(&usrcslot), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        unsafe extern "system" fn BindUnorderedAccessViewByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCSTR, udstslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindUnorderedAccessViewByName(this, ::core::mem::transmute(&pname), ::core::mem::transmute_copy(&udstslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindResourceAsUnorderedAccessView(this, ::core::mem::transmute_copy(&usrcsrvslot), ::core::mem::transmute_copy(&udstuavslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessViewByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrvname: ::windows_core::PCSTR, udstuavslot: u32, ucount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindResourceAsUnorderedAccessViewByName(this, ::core::mem::transmute(&psrvname), ::core::mem::transmute_copy(&udstuavslot), ::core::mem::transmute_copy(&ucount)).into())
        }
        ID3D11ModuleInstance_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BindConstantBuffer: BindConstantBuffer::<Identity, Impl, OFFSET>,
            BindConstantBufferByName: BindConstantBufferByName::<Identity, Impl, OFFSET>,
            BindResource: BindResource::<Identity, Impl, OFFSET>,
            BindResourceByName: BindResourceByName::<Identity, Impl, OFFSET>,
            BindSampler: BindSampler::<Identity, Impl, OFFSET>,
            BindSamplerByName: BindSamplerByName::<Identity, Impl, OFFSET>,
            BindUnorderedAccessView: BindUnorderedAccessView::<Identity, Impl, OFFSET>,
            BindUnorderedAccessViewByName: BindUnorderedAccessViewByName::<Identity, Impl, OFFSET>,
            BindResourceAsUnorderedAccessView: BindResourceAsUnorderedAccessView::<Identity, Impl, OFFSET>,
            BindResourceAsUnorderedAccessViewByName: BindResourceAsUnorderedAccessViewByName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11Multithread_Impl: ::windows_core::BaseImpl {
    fn Enter(this: &Self::This);
    fn Leave(this: &Self::This);
    fn SetMultithreadProtected(this: &Self::This, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11Multithread {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Multithread {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enter(this))
        }
        unsafe extern "system" fn Leave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Leave(this))
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultithreadProtected(this, ::core::mem::transmute_copy(&bmtprotect)))
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMultithreadProtected(this))
        }
        ID3D11Multithread_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, Impl, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11PixelShader_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D11PixelShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11PixelShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11PixelShader {
    const VTABLE: Self::Vtable = { ID3D11PixelShader_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D11Predicate_Impl: ::windows_core::BaseImpl + ID3D11Query_Impl {}
impl ::windows_core::Iids for ID3D11Predicate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Query);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Predicate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Predicate {
    const VTABLE: Self::Vtable = { ID3D11Predicate_Vtbl { base__: <ID3D11Query as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D11Query_Impl: ::windows_core::BaseImpl + ID3D11Asynchronous_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_QUERY_DESC);
}
impl ::windows_core::Iids for ID3D11Query {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Asynchronous);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Query_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Query {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Query_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_QUERY_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Query_Vtbl { base__: <ID3D11Asynchronous as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11Query1_Impl: ::windows_core::BaseImpl + ID3D11Query_Impl {
    fn GetDesc1(this: &Self::This, pdesc1: *mut D3D11_QUERY_DESC1);
}
impl ::windows_core::Iids for ID3D11Query1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Query);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Query1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Query1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Query1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_QUERY_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc1)))
        }
        ID3D11Query1_Vtbl { base__: <ID3D11Query as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_RASTERIZER_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11RasterizerState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RasterizerState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11RasterizerState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RasterizerState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11RasterizerState_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState1_Impl: ::windows_core::BaseImpl + ID3D11RasterizerState_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut D3D11_RASTERIZER_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11RasterizerState1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11RasterizerState);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RasterizerState1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11RasterizerState1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RasterizerState1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11RasterizerState1_Vtbl { base__: <ID3D11RasterizerState as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11RasterizerState2_Impl: ::windows_core::BaseImpl + ID3D11RasterizerState1_Impl {
    fn GetDesc2(this: &Self::This, pdesc: *mut D3D11_RASTERIZER_DESC2);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11RasterizerState2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11RasterizerState1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RasterizerState2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11RasterizerState2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RasterizerState2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc2(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11RasterizerState2_Vtbl { base__: <ID3D11RasterizerState1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc2: GetDesc2::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11RefDefaultTrackingOptions_Impl: ::windows_core::BaseImpl {
    fn SetTrackingOptions(this: &Self::This, resourcetypeflags: u32, options: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D11RefDefaultTrackingOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RefDefaultTrackingOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11RefDefaultTrackingOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTrackingOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RefDefaultTrackingOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrackingOptions(this, ::core::mem::transmute_copy(&resourcetypeflags), ::core::mem::transmute_copy(&options)).into())
        }
        ID3D11RefDefaultTrackingOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTrackingOptions: SetTrackingOptions::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11RefTrackingOptions_Impl: ::windows_core::BaseImpl {
    fn SetTrackingOptions(this: &Self::This, uoptions: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D11RefTrackingOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RefTrackingOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11RefTrackingOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTrackingOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RefTrackingOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrackingOptions(this, ::core::mem::transmute_copy(&uoptions)).into())
        }
        ID3D11RefTrackingOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTrackingOptions: SetTrackingOptions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView_Impl: ::windows_core::BaseImpl + ID3D11View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11RenderTargetView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11View);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RenderTargetView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11RenderTargetView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RenderTargetView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11RenderTargetView_Vtbl { base__: <ID3D11View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView1_Impl: ::windows_core::BaseImpl + ID3D11RenderTargetView_Impl {
    fn GetDesc1(this: &Self::This, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11RenderTargetView1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11RenderTargetView);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RenderTargetView1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11RenderTargetView1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11RenderTargetView1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc1)))
        }
        ID3D11RenderTargetView1_Vtbl { base__: <ID3D11RenderTargetView as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11Resource_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetType(this: &Self::This, presourcedimension: *mut D3D11_RESOURCE_DIMENSION);
    fn SetEvictionPriority(this: &Self::This, evictionpriority: u32);
    fn GetEvictionPriority(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for ID3D11Resource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Resource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourcedimension: *mut D3D11_RESOURCE_DIMENSION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this, ::core::mem::transmute_copy(&presourcedimension)))
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEvictionPriority(this, ::core::mem::transmute_copy(&evictionpriority)))
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEvictionPriority(this))
        }
        ID3D11Resource_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11SamplerState_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_SAMPLER_DESC);
}
impl ::windows_core::Iids for ID3D11SamplerState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11SamplerState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11SamplerState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11SamplerState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SAMPLER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11SamplerState_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D11ShaderReflection_Impl: ::windows_core::BaseImpl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetConstantBufferByIndex(this: &Self::This, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(this: &Self::This, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetInputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetOutputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetPatchConstantParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(this: &Self::This, name: &::windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetMovInstructionCount(this: &Self::This) -> u32;
    fn GetMovcInstructionCount(this: &Self::This) -> u32;
    fn GetConversionInstructionCount(this: &Self::This) -> u32;
    fn GetBitwiseInstructionCount(this: &Self::This) -> u32;
    fn GetGSInputPrimitive(this: &Self::This) -> super::Direct3D::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetNumInterfaceSlots(this: &Self::This) -> u32;
    fn GetMinFeatureLevel(this: &Self::This) -> ::windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(this: &Self::This, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(this: &Self::This) -> u64;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::windows_core::Iids for ID3D11ShaderReflection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ShaderReflection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceBindingDesc(this, ::core::mem::transmute_copy(&resourceindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatchConstantParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVariableByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceBindingDescByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMovInstructionCount(this))
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMovcInstructionCount(this))
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionInstructionCount(this))
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBitwiseInstructionCount(this))
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGSInputPrimitive(this))
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSampleFrequencyShader(this))
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumInterfaceSlots(this))
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinFeatureLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetThreadGroupSize(this, ::core::mem::transmute_copy(&psizex), ::core::mem::transmute_copy(&psizey), ::core::mem::transmute_copy(&psizez)))
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRequiresFlags(this))
        }
        ID3D11ShaderReflection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Identity, Impl, OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Identity, Impl, OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Identity, Impl, OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>() -> ID3D11ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByName(this, ::core::mem::transmute(&name))
        }
        Self { GetDesc: GetDesc::<Impl>, GetVariableByIndex: GetVariableByIndex::<Impl>, GetVariableByName: GetVariableByName::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D11ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D11ShaderReflectionConstantBuffer_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D11ShaderReflectionConstantBuffer_Impl> ID3D11ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionConstantBuffer_Vtbl = ID3D11ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D11ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> ::windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> ::windows_core::PCSTR;
    fn IsEqual(&self, ptype: ::core::option::Option<&ID3D11ShaderReflectionType>) -> ::windows_core::Result<()>;
    fn GetSubType(&self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetBaseClass(&self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetNumInterfaces(&self) -> u32;
    fn GetInterfaceByIndex(&self, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn IsOfType(&self, ptype: ::core::option::Option<&ID3D11ShaderReflectionType>) -> ::windows_core::Result<()>;
    fn ImplementsInterface(&self, pbase: ::core::option::Option<&ID3D11ShaderReflectionType>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionType_Vtbl {
    pub const fn new<Impl: ID3D11ShaderReflectionType_Impl>() -> ID3D11ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::PCSTR {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeName(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, ptype: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetSubType(this)
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBaseClass(this)
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetNumInterfaces(this)
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetInterfaceByIndex(this, ::core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, ptype: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsOfType(this, ::windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, pbase: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::ImplementsInterface(this, ::windows_core::from_raw_borrowed(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeName: GetMemberTypeName::<Impl>,
            IsEqual: IsEqual::<Impl>,
            GetSubType: GetSubType::<Impl>,
            GetBaseClass: GetBaseClass::<Impl>,
            GetNumInterfaces: GetNumInterfaces::<Impl>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Impl>,
            IsOfType: IsOfType::<Impl>,
            ImplementsInterface: ImplementsInterface::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D11ShaderReflectionType_ImplVtbl<T: ID3D11ShaderReflectionType_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D11ShaderReflectionType_Impl> ID3D11ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionType_Vtbl = ID3D11ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionType {
    pub fn new<'a, T: ID3D11ShaderReflectionType_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait ID3D11ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> ::windows_core::Result<()>;
    fn GetType(&self) -> ::core::option::Option<ID3D11ShaderReflectionType>;
    fn GetBuffer(&self) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32;
}
impl ID3D11ShaderReflectionVariable_Vtbl {
    pub const fn new<Impl: ID3D11ShaderReflectionVariable_Impl>() -> ID3D11ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetType<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetType(this)
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBuffer(this)
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetInterfaceSlot(this, ::core::mem::transmute_copy(&uarrayindex))
        }
        Self { GetDesc: GetDesc::<Impl>, GetType: GetType::<Impl>, GetBuffer: GetBuffer::<Impl>, GetInterfaceSlot: GetInterfaceSlot::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D11ShaderReflectionVariable_ImplVtbl<T: ID3D11ShaderReflectionVariable_Impl>(::std::marker::PhantomData<T>);
impl<T: ID3D11ShaderReflectionVariable_Impl> ID3D11ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionVariable_Vtbl = ID3D11ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D11ShaderReflectionVariable {
    pub fn new<'a, T: ID3D11ShaderReflectionVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView_Impl: ::windows_core::BaseImpl + ID3D11View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11ShaderResourceView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11View);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderResourceView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ShaderResourceView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderResourceView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11ShaderResourceView_Vtbl { base__: <ID3D11View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView1_Impl: ::windows_core::BaseImpl + ID3D11ShaderResourceView_Impl {
    fn GetDesc1(this: &Self::This, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11ShaderResourceView1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11ShaderResourceView);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderResourceView1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ShaderResourceView1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderResourceView1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc1)))
        }
        ID3D11ShaderResourceView1_Vtbl { base__: <ID3D11ShaderResourceView as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11ShaderTrace_Impl: ::windows_core::BaseImpl {
    fn TraceReady(this: &Self::This, ptestcount: *mut u64) -> ::windows_core::Result<()>;
    fn ResetTrace(this: &Self::This);
    fn GetTraceStats(this: &Self::This, ptracestats: *mut D3D11_TRACE_STATS) -> ::windows_core::Result<()>;
    fn PSSelectStamp(this: &Self::This, stampindex: u32) -> ::windows_core::Result<()>;
    fn GetInitialRegisterContents(this: &Self::This, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows_core::Result<()>;
    fn GetStep(this: &Self::This, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> ::windows_core::Result<()>;
    fn GetWrittenRegister(this: &Self::This, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows_core::Result<()>;
    fn GetReadRegister(this: &Self::This, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11ShaderTrace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ShaderTrace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TraceReady<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptestcount: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TraceReady(this, ::core::mem::transmute_copy(&ptestcount)).into())
        }
        unsafe extern "system" fn ResetTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetTrace(this))
        }
        unsafe extern "system" fn GetTraceStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptracestats: *mut D3D11_TRACE_STATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTraceStats(this, ::core::mem::transmute_copy(&ptracestats)).into())
        }
        unsafe extern "system" fn PSSelectStamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stampindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSelectStamp(this, ::core::mem::transmute_copy(&stampindex)).into())
        }
        unsafe extern "system" fn GetInitialRegisterContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInitialRegisterContents(this, ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetStep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStep(this, ::core::mem::transmute_copy(&stepindex), ::core::mem::transmute_copy(&ptracestep)).into())
        }
        unsafe extern "system" fn GetWrittenRegister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWrittenRegister(this, ::core::mem::transmute_copy(&stepindex), ::core::mem::transmute_copy(&writtenregisterindex), ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetReadRegister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReadRegister(this, ::core::mem::transmute_copy(&stepindex), ::core::mem::transmute_copy(&readregisterindex), ::core::mem::transmute_copy(&pregister), ::core::mem::transmute_copy(&pvalue)).into())
        }
        ID3D11ShaderTrace_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TraceReady: TraceReady::<Identity, Impl, OFFSET>,
            ResetTrace: ResetTrace::<Identity, Impl, OFFSET>,
            GetTraceStats: GetTraceStats::<Identity, Impl, OFFSET>,
            PSSelectStamp: PSSelectStamp::<Identity, Impl, OFFSET>,
            GetInitialRegisterContents: GetInitialRegisterContents::<Identity, Impl, OFFSET>,
            GetStep: GetStep::<Identity, Impl, OFFSET>,
            GetWrittenRegister: GetWrittenRegister::<Identity, Impl, OFFSET>,
            GetReadRegister: GetReadRegister::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11ShaderTraceFactory_Impl: ::windows_core::BaseImpl {
    fn CreateShaderTrace(this: &Self::This, pshader: ::core::option::Option<&::windows_core::IUnknown>, ptracedesc: *const D3D11_SHADER_TRACE_DESC) -> ::windows_core::Result<ID3D11ShaderTrace>;
}
impl ::windows_core::Iids for ID3D11ShaderTraceFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTraceFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11ShaderTraceFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateShaderTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11ShaderTraceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ptracedesc: *const D3D11_SHADER_TRACE_DESC, ppshadertrace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateShaderTrace(this, ::windows_core::from_raw_borrowed(&pshader), ::core::mem::transmute_copy(&ptracedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshadertrace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11ShaderTraceFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateShaderTrace: CreateShaderTrace::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D11SwitchToRef_Impl: ::windows_core::BaseImpl {
    fn SetUseRef(this: &Self::This, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D11SwitchToRef {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11SwitchToRef_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11SwitchToRef {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetUseRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11SwitchToRef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseRef(this, ::core::mem::transmute_copy(&useref)))
        }
        unsafe extern "system" fn GetUseRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11SwitchToRef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUseRef(this))
        }
        ID3D11SwitchToRef_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetUseRef: SetUseRef::<Identity, Impl, OFFSET>,
            GetUseRef: GetUseRef::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture1D_Impl: ::windows_core::BaseImpl + ID3D11Resource_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11Texture1D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Resource);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture1D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Texture1D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture1D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE1D_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Texture1D_Vtbl { base__: <ID3D11Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D_Impl: ::windows_core::BaseImpl + ID3D11Resource_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11Texture2D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Resource);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture2D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Texture2D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture2D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Texture2D_Vtbl { base__: <ID3D11Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D1_Impl: ::windows_core::BaseImpl + ID3D11Texture2D_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut D3D11_TEXTURE2D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11Texture2D1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Texture2D);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture2D1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Texture2D1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture2D1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Texture2D1_Vtbl { base__: <ID3D11Texture2D as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D_Impl: ::windows_core::BaseImpl + ID3D11Resource_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11Texture3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Resource);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Texture3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Texture3D_Vtbl { base__: <ID3D11Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D1_Impl: ::windows_core::BaseImpl + ID3D11Texture3D_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut D3D11_TEXTURE3D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11Texture3D1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11Texture3D);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture3D1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11Texture3D1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11Texture3D1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11Texture3D1_Vtbl { base__: <ID3D11Texture3D as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11TracingDevice_Impl: ::windows_core::BaseImpl {
    fn SetShaderTrackingOptionsByType(this: &Self::This, resourcetypeflags: u32, options: u32) -> ::windows_core::Result<()>;
    fn SetShaderTrackingOptions(this: &Self::This, pshader: ::core::option::Option<&::windows_core::IUnknown>, options: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D11TracingDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11TracingDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11TracingDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetShaderTrackingOptionsByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11TracingDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcetypeflags: u32, options: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetShaderTrackingOptionsByType(this, ::core::mem::transmute_copy(&resourcetypeflags), ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn SetShaderTrackingOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11TracingDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, options: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetShaderTrackingOptions(this, ::windows_core::from_raw_borrowed(&pshader), ::core::mem::transmute_copy(&options)).into())
        }
        ID3D11TracingDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetShaderTrackingOptionsByType: SetShaderTrackingOptionsByType::<Identity, Impl, OFFSET>,
            SetShaderTrackingOptions: SetShaderTrackingOptions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView_Impl: ::windows_core::BaseImpl + ID3D11View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11UnorderedAccessView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11View);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11UnorderedAccessView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11UnorderedAccessView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11UnorderedAccessView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11UnorderedAccessView_Vtbl { base__: <ID3D11View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView1_Impl: ::windows_core::BaseImpl + ID3D11UnorderedAccessView_Impl {
    fn GetDesc1(this: &Self::This, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11UnorderedAccessView1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11UnorderedAccessView);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11UnorderedAccessView1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11UnorderedAccessView1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11UnorderedAccessView1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc1)))
        }
        ID3D11UnorderedAccessView1_Vtbl { base__: <ID3D11UnorderedAccessView as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11VertexShader_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D11VertexShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VertexShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VertexShader {
    const VTABLE: Self::Vtable = { ID3D11VertexShader_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetDecoderBuffer(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReleaseDecoderBuffer(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows_core::Result<()>;
    fn DecoderBeginFrame(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, pview: ::core::option::Option<&ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DecoderEndFrame(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>) -> ::windows_core::Result<()>;
    fn SubmitDecoderBuffers(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows_core::Result<()>;
    fn DecoderExtension(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32;
    fn VideoProcessorSetOutputTargetRect(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetOutputBackgroundColor(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR);
    fn VideoProcessorSetOutputColorSpace(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetOutputAlphaFillMode(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32);
    fn VideoProcessorSetOutputConstriction(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, size: &super::super::Foundation::SIZE);
    fn VideoProcessorSetOutputStereoMode(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetOutputExtension(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32;
    fn VideoProcessorGetOutputTargetRect(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetOutputBackgroundColor(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR);
    fn VideoProcessorGetOutputColorSpace(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetOutputAlphaFillMode(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32);
    fn VideoProcessorGetOutputConstriction(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE);
    fn VideoProcessorGetOutputStereoMode(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputExtension(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32;
    fn VideoProcessorSetStreamFrameFormat(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorSetStreamColorSpace(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetStreamOutputRate(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamSourceRect(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamDestRect(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamAlpha(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32);
    fn VideoProcessorSetStreamPalette(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *const u32);
    fn VideoProcessorSetStreamPixelAspectRatio(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamLumaKey(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32);
    fn VideoProcessorSetStreamStereoFormat(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32);
    fn VideoProcessorSetStreamAutoProcessingMode(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamFilter(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32);
    fn VideoProcessorSetStreamExtension(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32;
    fn VideoProcessorGetStreamFrameFormat(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorGetStreamColorSpace(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetStreamOutputRate(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamSourceRect(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamDestRect(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamAlpha(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32);
    fn VideoProcessorGetStreamPalette(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *mut u32);
    fn VideoProcessorGetStreamPixelAspectRatio(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamLumaKey(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32);
    fn VideoProcessorGetStreamStereoFormat(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32);
    fn VideoProcessorGetStreamAutoProcessingMode(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamFilter(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32);
    fn VideoProcessorGetStreamExtension(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32;
    fn VideoProcessorBlt(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pview: ::core::option::Option<&ID3D11VideoProcessorOutputView>, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows_core::Result<()>;
    fn NegotiateCryptoSessionKeyExchange(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EncryptionBlt(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>, psrcsurface: ::core::option::Option<&ID3D11Texture2D>, pdstsurface: ::core::option::Option<&ID3D11Texture2D>, ivsize: u32, piv: *mut ::core::ffi::c_void);
    fn DecryptionBlt(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>, psrcsurface: ::core::option::Option<&ID3D11Texture2D>, pdstsurface: ::core::option::Option<&ID3D11Texture2D>, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *mut ::core::ffi::c_void);
    fn StartSessionKeyRefresh(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void);
    fn FinishSessionKeyRefresh(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>);
    fn GetEncryptionBltKey(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn NegotiateAuthenticatedChannelKeyExchange(this: &Self::This, pchannel: ::core::option::Option<&ID3D11AuthenticatedChannel>, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn QueryAuthenticatedChannel(this: &Self::This, pchannel: ::core::option::Option<&ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ConfigureAuthenticatedChannel(this: &Self::This, pchannel: ::core::option::Option<&ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows_core::Result<()>;
    fn VideoProcessorSetStreamRotation(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION);
    fn VideoProcessorGetStreamRotation(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDecoderBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDecoderBuffer(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pbuffersize), ::core::mem::transmute_copy(&ppbuffer)).into())
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDecoderBuffer(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn DecoderBeginFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, pview: *mut ::core::ffi::c_void, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecoderBeginFrame(this, ::windows_core::from_raw_borrowed(&pdecoder), ::windows_core::from_raw_borrowed(&pview), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey)).into())
        }
        unsafe extern "system" fn DecoderEndFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecoderEndFrame(this, ::windows_core::from_raw_borrowed(&pdecoder)).into())
        }
        unsafe extern "system" fn SubmitDecoderBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubmitDecoderBuffers(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into())
        }
        unsafe extern "system" fn DecoderExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecoderExtension(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&pextensiondata)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputTargetRect(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputBackgroundColor(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&ycbcr), ::core::mem::transmute_copy(&pcolor)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputColorSpace(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputAlphaFillMode(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&alphafillmode), ::core::mem::transmute_copy(&streamindex)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL, size: super::super::Foundation::SIZE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputConstriction(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&enable), ::core::mem::transmute(&size)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputStereoMode(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&enable)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputExtension(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputTargetRect(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&enabled), ::core::mem::transmute_copy(&prect)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputBackgroundColor(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&pycbcr), ::core::mem::transmute_copy(&pcolor)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputColorSpace(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputAlphaFillMode(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&palphafillmode), ::core::mem::transmute_copy(&pstreamindex)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputConstriction(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psize)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, penabled: *mut super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputStereoMode(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&penabled)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputExtension(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamFrameFormat(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&frameformat)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamColorSpace(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamOutputRate(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&outputrate), ::core::mem::transmute_copy(&repeatframe), ::core::mem::transmute_copy(&pcustomrate)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamSourceRect(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamDestRect(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&prect)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamAlpha(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&alpha)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, count: u32, pentries: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamPalette(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pentries)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamPixelAspectRatio(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&psourceaspectratio), ::core::mem::transmute_copy(&pdestinationaspectratio)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamLumaKey(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&lower), ::core::mem::transmute_copy(&upper)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamStereoFormat(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&leftviewframe0), ::core::mem::transmute_copy(&baseviewframe0), ::core::mem::transmute_copy(&flipmode), ::core::mem::transmute_copy(&monooffset)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamAutoProcessingMode(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamFilter(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&level)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamExtension(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamFrameFormat(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pframeformat)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamColorSpace(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamOutputRate(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&poutputrate), ::core::mem::transmute_copy(&prepeatframe), ::core::mem::transmute_copy(&pcustomrate)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamSourceRect(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamDestRect(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&prect)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamAlpha(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&palpha)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, count: u32, pentries: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamPalette(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&pentries)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamPixelAspectRatio(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&psourceaspectratio), ::core::mem::transmute_copy(&pdestinationaspectratio)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamLumaKey(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plower), ::core::mem::transmute_copy(&pupper)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamStereoFormat(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pleftviewframe0), ::core::mem::transmute_copy(&pbaseviewframe0), ::core::mem::transmute_copy(&pflipmode), ::core::mem::transmute_copy(&monooffset)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamAutoProcessingMode(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penabled)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamFilter(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&penabled), ::core::mem::transmute_copy(&plevel)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, pextensionguid: *const ::windows_core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamExtension(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pextensionguid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)))
        }
        unsafe extern "system" fn VideoProcessorBlt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pview: *mut ::core::ffi::c_void, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorBlt(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::windows_core::from_raw_borrowed(&pview), ::core::mem::transmute_copy(&outputframe), ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams)).into())
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NegotiateCryptoSessionKeyExchange(this, ::windows_core::from_raw_borrowed(&pcryptosession), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn EncryptionBlt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, psrcsurface: *mut ::core::ffi::c_void, pdstsurface: *mut ::core::ffi::c_void, ivsize: u32, piv: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncryptionBlt(this, ::windows_core::from_raw_borrowed(&pcryptosession), ::windows_core::from_raw_borrowed(&psrcsurface), ::windows_core::from_raw_borrowed(&pdstsurface), ::core::mem::transmute_copy(&ivsize), ::core::mem::transmute_copy(&piv)))
        }
        unsafe extern "system" fn DecryptionBlt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, psrcsurface: *mut ::core::ffi::c_void, pdstsurface: *mut ::core::ffi::c_void, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, ivsize: u32, piv: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecryptionBlt(this, ::windows_core::from_raw_borrowed(&pcryptosession), ::windows_core::from_raw_borrowed(&psrcsurface), ::windows_core::from_raw_borrowed(&pdstsurface), ::core::mem::transmute_copy(&pencryptedblockinfo), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey), ::core::mem::transmute_copy(&ivsize), ::core::mem::transmute_copy(&piv)))
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartSessionKeyRefresh(this, ::windows_core::from_raw_borrowed(&pcryptosession), ::core::mem::transmute_copy(&randomnumbersize), ::core::mem::transmute_copy(&prandomnumber)))
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishSessionKeyRefresh(this, ::windows_core::from_raw_borrowed(&pcryptosession)))
        }
        unsafe extern "system" fn GetEncryptionBltKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEncryptionBltKey(this, ::windows_core::from_raw_borrowed(&pcryptosession), ::core::mem::transmute_copy(&keysize), ::core::mem::transmute_copy(&preadbackkey)).into())
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NegotiateAuthenticatedChannelKeyExchange(this, ::windows_core::from_raw_borrowed(&pchannel), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryAuthenticatedChannel(this, ::windows_core::from_raw_borrowed(&pchannel), ::core::mem::transmute_copy(&inputsize), ::core::mem::transmute_copy(&pinput), ::core::mem::transmute_copy(&outputsize), ::core::mem::transmute_copy(&poutput)).into())
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureAuthenticatedChannel(this, ::windows_core::from_raw_borrowed(&pchannel), ::core::mem::transmute_copy(&inputsize), ::core::mem::transmute_copy(&pinput), ::core::mem::transmute_copy(&poutput)).into())
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamRotation(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&rotation)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamRotation(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&protation)))
        }
        ID3D11VideoContext_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDecoderBuffer: GetDecoderBuffer::<Identity, Impl, OFFSET>,
            ReleaseDecoderBuffer: ReleaseDecoderBuffer::<Identity, Impl, OFFSET>,
            DecoderBeginFrame: DecoderBeginFrame::<Identity, Impl, OFFSET>,
            DecoderEndFrame: DecoderEndFrame::<Identity, Impl, OFFSET>,
            SubmitDecoderBuffers: SubmitDecoderBuffers::<Identity, Impl, OFFSET>,
            DecoderExtension: DecoderExtension::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputTargetRect: VideoProcessorSetOutputTargetRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputBackgroundColor: VideoProcessorSetOutputBackgroundColor::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace: VideoProcessorSetOutputColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputAlphaFillMode: VideoProcessorSetOutputAlphaFillMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputConstriction: VideoProcessorSetOutputConstriction::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputStereoMode: VideoProcessorSetOutputStereoMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputExtension: VideoProcessorSetOutputExtension::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputTargetRect: VideoProcessorGetOutputTargetRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputBackgroundColor: VideoProcessorGetOutputBackgroundColor::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace: VideoProcessorGetOutputColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputAlphaFillMode: VideoProcessorGetOutputAlphaFillMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputConstriction: VideoProcessorGetOutputConstriction::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputStereoMode: VideoProcessorGetOutputStereoMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputExtension: VideoProcessorGetOutputExtension::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamFrameFormat: VideoProcessorSetStreamFrameFormat::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace: VideoProcessorSetStreamColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamOutputRate: VideoProcessorSetStreamOutputRate::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamSourceRect: VideoProcessorSetStreamSourceRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamDestRect: VideoProcessorSetStreamDestRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamAlpha: VideoProcessorSetStreamAlpha::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamPalette: VideoProcessorSetStreamPalette::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio: VideoProcessorSetStreamPixelAspectRatio::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamLumaKey: VideoProcessorSetStreamLumaKey::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamStereoFormat: VideoProcessorSetStreamStereoFormat::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode: VideoProcessorSetStreamAutoProcessingMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamFilter: VideoProcessorSetStreamFilter::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamExtension: VideoProcessorSetStreamExtension::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamFrameFormat: VideoProcessorGetStreamFrameFormat::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace: VideoProcessorGetStreamColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamOutputRate: VideoProcessorGetStreamOutputRate::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamSourceRect: VideoProcessorGetStreamSourceRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamDestRect: VideoProcessorGetStreamDestRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamAlpha: VideoProcessorGetStreamAlpha::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamPalette: VideoProcessorGetStreamPalette::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio: VideoProcessorGetStreamPixelAspectRatio::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamLumaKey: VideoProcessorGetStreamLumaKey::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamStereoFormat: VideoProcessorGetStreamStereoFormat::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode: VideoProcessorGetStreamAutoProcessingMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamFilter: VideoProcessorGetStreamFilter::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamExtension: VideoProcessorGetStreamExtension::<Identity, Impl, OFFSET>,
            VideoProcessorBlt: VideoProcessorBlt::<Identity, Impl, OFFSET>,
            NegotiateCryptoSessionKeyExchange: NegotiateCryptoSessionKeyExchange::<Identity, Impl, OFFSET>,
            EncryptionBlt: EncryptionBlt::<Identity, Impl, OFFSET>,
            DecryptionBlt: DecryptionBlt::<Identity, Impl, OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Identity, Impl, OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Identity, Impl, OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Identity, Impl, OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange: NegotiateAuthenticatedChannelKeyExchange::<Identity, Impl, OFFSET>,
            QueryAuthenticatedChannel: QueryAuthenticatedChannel::<Identity, Impl, OFFSET>,
            ConfigureAuthenticatedChannel: ConfigureAuthenticatedChannel::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamRotation: VideoProcessorSetStreamRotation::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamRotation: VideoProcessorGetStreamRotation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext1_Impl: ::windows_core::BaseImpl + ID3D11VideoContext_Impl {
    fn SubmitDecoderBuffers1(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows_core::Result<()>;
    fn GetDataForNewHardwareKey(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void) -> ::windows_core::Result<u64>;
    fn CheckCryptoSessionStatus(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>) -> ::windows_core::Result<D3D11_CRYPTO_SESSION_STATUS>;
    fn DecoderEnableDownsampling(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows_core::Result<()>;
    fn DecoderUpdateDownsampling(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows_core::Result<()>;
    fn VideoProcessorSetOutputColorSpace1(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetOutputShaderUsage(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, shaderusage: super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputColorSpace1(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetOutputShaderUsage(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, pshaderusage: *mut super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamColorSpace1(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetStreamMirror(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamColorSpace1(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetStreamMirror(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetBehaviorHints(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoContext1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11VideoContext);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoContext1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SubmitDecoderBuffers1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubmitDecoderBuffers1(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into())
        }
        unsafe extern "system" fn GetDataForNewHardwareKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, privateinputsize: u32, pprivatinputdata: *const ::core::ffi::c_void, pprivateoutputdata: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataForNewHardwareKey(this, ::windows_core::from_raw_borrowed(&pcryptosession), ::core::mem::transmute_copy(&privateinputsize), ::core::mem::transmute_copy(&pprivatinputdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprivateoutputdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckCryptoSessionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, pstatus: *mut D3D11_CRYPTO_SESSION_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckCryptoSessionStatus(this, ::windows_core::from_raw_borrowed(&pcryptosession)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DecoderEnableDownsampling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecoderEnableDownsampling(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&poutputdesc), ::core::mem::transmute_copy(&referenceframecount)).into())
        }
        unsafe extern "system" fn DecoderUpdateDownsampling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecoderUpdateDownsampling(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&poutputdesc)).into())
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputColorSpace1(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&colorspace)))
        }
        unsafe extern "system" fn VideoProcessorSetOutputShaderUsage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, shaderusage: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputShaderUsage(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&shaderusage)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputColorSpace1(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&pcolorspace)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputShaderUsage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, pshaderusage: *mut super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputShaderUsage(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&pshaderusage)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamColorSpace1(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&colorspace)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamMirror<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamMirror(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&enable), ::core::mem::transmute_copy(&fliphorizontal), ::core::mem::transmute_copy(&flipvertical)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamColorSpace1(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&pcolorspace)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamMirror<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamMirror(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&penable), ::core::mem::transmute_copy(&pfliphorizontal), ::core::mem::transmute_copy(&pflipvertical)))
        }
        unsafe extern "system" fn VideoProcessorGetBehaviorHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, pbehaviorhints: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VideoProcessorGetBehaviorHints(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&outputwidth), ::core::mem::transmute_copy(&outputheight), ::core::mem::transmute_copy(&outputformat), ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbehaviorhints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11VideoContext1_Vtbl {
            base__: <ID3D11VideoContext as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SubmitDecoderBuffers1: SubmitDecoderBuffers1::<Identity, Impl, OFFSET>,
            GetDataForNewHardwareKey: GetDataForNewHardwareKey::<Identity, Impl, OFFSET>,
            CheckCryptoSessionStatus: CheckCryptoSessionStatus::<Identity, Impl, OFFSET>,
            DecoderEnableDownsampling: DecoderEnableDownsampling::<Identity, Impl, OFFSET>,
            DecoderUpdateDownsampling: DecoderUpdateDownsampling::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace1: VideoProcessorSetOutputColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputShaderUsage: VideoProcessorSetOutputShaderUsage::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace1: VideoProcessorGetOutputColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputShaderUsage: VideoProcessorGetOutputShaderUsage::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace1: VideoProcessorSetStreamColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamMirror: VideoProcessorSetStreamMirror::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace1: VideoProcessorGetStreamColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamMirror: VideoProcessorGetStreamMirror::<Identity, Impl, OFFSET>,
            VideoProcessorGetBehaviorHints: VideoProcessorGetBehaviorHints::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext2_Impl: ::windows_core::BaseImpl + ID3D11VideoContext1_Impl {
    fn VideoProcessorSetOutputHDRMetaData(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void);
    fn VideoProcessorGetOutputHDRMetaData(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void);
    fn VideoProcessorSetStreamHDRMetaData(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void);
    fn VideoProcessorGetStreamHDRMetaData(this: &Self::This, pvideoprocessor: ::core::option::Option<&ID3D11VideoProcessor>, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoContext2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11VideoContext1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoContext2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VideoProcessorSetOutputHDRMetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetOutputHDRMetaData(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&phdrmetadata)))
        }
        unsafe extern "system" fn VideoProcessorGetOutputHDRMetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetOutputHDRMetaData(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata)))
        }
        unsafe extern "system" fn VideoProcessorSetStreamHDRMetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorSetStreamHDRMetaData(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&phdrmetadata)))
        }
        unsafe extern "system" fn VideoProcessorGetStreamHDRMetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideoprocessor: *mut ::core::ffi::c_void, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VideoProcessorGetStreamHDRMetaData(this, ::windows_core::from_raw_borrowed(&pvideoprocessor), ::core::mem::transmute_copy(&streamindex), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata)))
        }
        ID3D11VideoContext2_Vtbl {
            base__: <ID3D11VideoContext1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            VideoProcessorSetOutputHDRMetaData: VideoProcessorSetOutputHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputHDRMetaData: VideoProcessorGetOutputHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamHDRMetaData: VideoProcessorSetStreamHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamHDRMetaData: VideoProcessorGetStreamHDRMetaData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoContext3_Impl: ::windows_core::BaseImpl + ID3D11VideoContext2_Impl {
    fn DecoderBeginFrame1(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, pview: ::core::option::Option<&ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const ::core::option::Option<ID3D11Buffer>) -> ::windows_core::Result<()>;
    fn SubmitDecoderBuffers2(this: &Self::This, pdecoder: ::core::option::Option<&ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoContext3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11VideoContext2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoContext3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DecoderBeginFrame1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, pview: *mut ::core::ffi::c_void, contentkeysize: u32, pcontentkey: *const ::core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecoderBeginFrame1(this, ::windows_core::from_raw_borrowed(&pdecoder), ::windows_core::from_raw_borrowed(&pview), ::core::mem::transmute_copy(&contentkeysize), ::core::mem::transmute_copy(&pcontentkey), ::core::mem::transmute_copy(&numcomponenthistograms), ::core::mem::transmute_copy(&phistogramoffsets), ::core::mem::transmute_copy(&pphistogrambuffers)).into())
        }
        unsafe extern "system" fn SubmitDecoderBuffers2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoder: *mut ::core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubmitDecoderBuffers2(this, ::windows_core::from_raw_borrowed(&pdecoder), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&pbufferdesc)).into())
        }
        ID3D11VideoContext3_Vtbl {
            base__: <ID3D11VideoContext2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DecoderBeginFrame1: DecoderBeginFrame1::<Identity, Impl, OFFSET>,
            SubmitDecoderBuffers2: SubmitDecoderBuffers2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDecoder_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetCreationParameters(this: &Self::This, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows_core::Result<()>;
    fn GetDriverHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoDecoder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDecoder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoDecoder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCreationParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationParameters(this, ::core::mem::transmute_copy(&pvideodesc), ::core::mem::transmute_copy(&pconfig)).into())
        }
        unsafe extern "system" fn GetDriverHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdriverhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDriverHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdriverhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11VideoDecoder_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCreationParameters: GetCreationParameters::<Identity, Impl, OFFSET>,
            GetDriverHandle: GetDriverHandle::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11VideoDecoderOutputView_Impl: ::windows_core::BaseImpl + ID3D11View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC);
}
impl ::windows_core::Iids for ID3D11VideoDecoderOutputView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11View);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDecoderOutputView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoDecoderOutputView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDecoderOutputView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11VideoDecoderOutputView_Vtbl { base__: <ID3D11View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice_Impl: ::windows_core::BaseImpl {
    fn CreateVideoDecoder(this: &Self::This, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> ::windows_core::Result<ID3D11VideoDecoder>;
    fn CreateVideoProcessor(this: &Self::This, penum: ::core::option::Option<&ID3D11VideoProcessorEnumerator>, rateconversionindex: u32) -> ::windows_core::Result<ID3D11VideoProcessor>;
    fn CreateAuthenticatedChannel(this: &Self::This, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> ::windows_core::Result<ID3D11AuthenticatedChannel>;
    fn CreateCryptoSession(this: &Self::This, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, pkeyexchangetype: *const ::windows_core::GUID) -> ::windows_core::Result<ID3D11CryptoSession>;
    fn CreateVideoDecoderOutputView(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut ::core::option::Option<ID3D11VideoDecoderOutputView>) -> ::windows_core::Result<()>;
    fn CreateVideoProcessorInputView(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, penum: ::core::option::Option<&ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut ::core::option::Option<ID3D11VideoProcessorInputView>) -> ::windows_core::Result<()>;
    fn CreateVideoProcessorOutputView(this: &Self::This, presource: ::core::option::Option<&ID3D11Resource>, penum: ::core::option::Option<&ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut ::core::option::Option<ID3D11VideoProcessorOutputView>) -> ::windows_core::Result<()>;
    fn CreateVideoProcessorEnumerator(this: &Self::This, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows_core::Result<ID3D11VideoProcessorEnumerator>;
    fn GetVideoDecoderProfileCount(this: &Self::This) -> u32;
    fn GetVideoDecoderProfile(this: &Self::This, index: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn CheckVideoDecoderFormat(this: &Self::This, pdecoderprofile: *const ::windows_core::GUID, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetVideoDecoderConfigCount(this: &Self::This, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> ::windows_core::Result<u32>;
    fn GetVideoDecoderConfig(this: &Self::This, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows_core::Result<()>;
    fn GetContentProtectionCaps(this: &Self::This, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows_core::Result<()>;
    fn CheckCryptoKeyExchange(this: &Self::This, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, index: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, guid: *const ::windows_core::GUID, pdata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVideoDecoder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG, ppdecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVideoDecoder(this, ::core::mem::transmute_copy(&pvideodesc), ::core::mem::transmute_copy(&pconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVideoProcessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penum: *mut ::core::ffi::c_void, rateconversionindex: u32, ppvideoprocessor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVideoProcessor(this, ::windows_core::from_raw_borrowed(&penum), ::core::mem::transmute_copy(&rateconversionindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvideoprocessor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppauthenticatedchannel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAuthenticatedChannel(this, ::core::mem::transmute_copy(&channeltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppauthenticatedchannel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCryptoSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, pkeyexchangetype: *const ::windows_core::GUID, ppcryptosession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCryptoSession(this, ::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&pkeyexchangetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcryptosession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVideoDecoderOutputView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppvdovview)).into())
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, penum: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVideoProcessorInputView(this, ::windows_core::from_raw_borrowed(&presource), ::windows_core::from_raw_borrowed(&penum), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppvpiview)).into())
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, penum: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVideoProcessorOutputView(this, ::windows_core::from_raw_borrowed(&presource), ::windows_core::from_raw_borrowed(&penum), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppvpoview)).into())
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVideoProcessorEnumerator(this, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoDecoderProfileCount(this))
        }
        unsafe extern "system" fn GetVideoDecoderProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pdecoderprofile: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVideoDecoderProfile(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdecoderprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows_core::GUID, format: super::Dxgi::Common::DXGI_FORMAT, psupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckVideoDecoderFormat(this, ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVideoDecoderConfigCount(this, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVideoDecoderConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoDecoderConfig(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pconfig)).into())
        }
        unsafe extern "system" fn GetContentProtectionCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContentProtectionCaps(this, ::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&pcaps)).into())
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, index: u32, pkeyexchangetype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckCryptoKeyExchange(this, ::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pkeyexchangetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&guid), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        ID3D11VideoDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVideoDecoder: CreateVideoDecoder::<Identity, Impl, OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Identity, Impl, OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Identity, Impl, OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Identity, Impl, OFFSET>,
            CreateVideoDecoderOutputView: CreateVideoDecoderOutputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorInputView: CreateVideoProcessorInputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorOutputView: CreateVideoProcessorOutputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorEnumerator: CreateVideoProcessorEnumerator::<Identity, Impl, OFFSET>,
            GetVideoDecoderProfileCount: GetVideoDecoderProfileCount::<Identity, Impl, OFFSET>,
            GetVideoDecoderProfile: GetVideoDecoderProfile::<Identity, Impl, OFFSET>,
            CheckVideoDecoderFormat: CheckVideoDecoderFormat::<Identity, Impl, OFFSET>,
            GetVideoDecoderConfigCount: GetVideoDecoderConfigCount::<Identity, Impl, OFFSET>,
            GetVideoDecoderConfig: GetVideoDecoderConfig::<Identity, Impl, OFFSET>,
            GetContentProtectionCaps: GetContentProtectionCaps::<Identity, Impl, OFFSET>,
            CheckCryptoKeyExchange: CheckCryptoKeyExchange::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice1_Impl: ::windows_core::BaseImpl + ID3D11VideoDevice_Impl {
    fn GetCryptoSessionPrivateDataSize(this: &Self::This, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, pkeyexchangetype: *const ::windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetVideoDecoderCaps(this: &Self::This, pdecoderprofile: *const ::windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn CheckVideoDecoderDownsampling(this: &Self::This, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RecommendVideoDecoderDownsampleParameters(this: &Self::This, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL) -> ::windows_core::Result<D3D11_VIDEO_SAMPLE_DESC>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoDevice1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11VideoDevice);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoDevice1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCryptoSessionPrivateDataSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptotype: *const ::windows_core::GUID, pdecoderprofile: *const ::windows_core::GUID, pkeyexchangetype: *const ::windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCryptoSessionPrivateDataSize(this, ::core::mem::transmute_copy(&pcryptotype), ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&pkeyexchangetype), ::core::mem::transmute_copy(&pprivateinputsize), ::core::mem::transmute_copy(&pprivateoutputsize)).into())
        }
        unsafe extern "system" fn GetVideoDecoderCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecoderprofile: *const ::windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const ::windows_core::GUID, pdecodercaps: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVideoDecoderCaps(this, ::core::mem::transmute_copy(&pdecoderprofile), ::core::mem::transmute_copy(&samplewidth), ::core::mem::transmute_copy(&sampleheight), ::core::mem::transmute_copy(&pframerate), ::core::mem::transmute_copy(&bitrate), ::core::mem::transmute_copy(&pcryptotype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdecodercaps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckVideoDecoderDownsampling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckVideoDecoderDownsampling(this, ::core::mem::transmute_copy(&pinputdesc), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&pinputconfig), ::core::mem::transmute_copy(&pframerate), ::core::mem::transmute_copy(&poutputdesc), ::core::mem::transmute_copy(&psupported), ::core::mem::transmute_copy(&prealtimehint)).into())
        }
        unsafe extern "system" fn RecommendVideoDecoderDownsampleParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, precommendedoutputdesc: *mut D3D11_VIDEO_SAMPLE_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecommendVideoDecoderDownsampleParameters(this, ::core::mem::transmute_copy(&pinputdesc), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&pinputconfig), ::core::mem::transmute_copy(&pframerate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(precommendedoutputdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11VideoDevice1_Vtbl {
            base__: <ID3D11VideoDevice as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCryptoSessionPrivateDataSize: GetCryptoSessionPrivateDataSize::<Identity, Impl, OFFSET>,
            GetVideoDecoderCaps: GetVideoDecoderCaps::<Identity, Impl, OFFSET>,
            CheckVideoDecoderDownsampling: CheckVideoDecoderDownsampling::<Identity, Impl, OFFSET>,
            RecommendVideoDecoderDownsampleParameters: RecommendVideoDecoderDownsampleParameters::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoDevice2_Impl: ::windows_core::BaseImpl + ID3D11VideoDevice1_Impl {
    fn CheckFeatureSupport(this: &Self::This, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()>;
    fn NegotiateCryptoSessionKeyExchangeMT(this: &Self::This, pcryptosession: ::core::option::Option<&ID3D11CryptoSession>, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoDevice2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11VideoDevice1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoDevice2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckFeatureSupport(this, ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into())
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchangeMT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcryptosession: *mut ::core::ffi::c_void, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NegotiateCryptoSessionKeyExchangeMT(this, ::windows_core::from_raw_borrowed(&pcryptosession), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        ID3D11VideoDevice2_Vtbl {
            base__: <ID3D11VideoDevice1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            NegotiateCryptoSessionKeyExchangeMT: NegotiateCryptoSessionKeyExchangeMT::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoProcessor_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetContentDesc(this: &Self::This, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC);
    fn GetRateConversionCaps(this: &Self::This, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D11VideoProcessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoProcessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContentDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContentDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        unsafe extern "system" fn GetRateConversionCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRateConversionCaps(this, ::core::mem::transmute_copy(&pcaps)))
        }
        ID3D11VideoProcessor_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContentDesc: GetContentDesc::<Identity, Impl, OFFSET>,
            GetRateConversionCaps: GetRateConversionCaps::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumerator_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetVideoProcessorContentDesc(this: &Self::This, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows_core::Result<()>;
    fn CheckVideoProcessorFormat(this: &Self::This, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows_core::Result<u32>;
    fn GetVideoProcessorCaps(this: &Self::This, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> ::windows_core::Result<()>;
    fn GetVideoProcessorRateConversionCaps(this: &Self::This, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> ::windows_core::Result<()>;
    fn GetVideoProcessorCustomRate(this: &Self::This, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> ::windows_core::Result<()>;
    fn GetVideoProcessorFilterRange(this: &Self::This, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> ::windows_core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoProcessorEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoProcessorEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVideoProcessorContentDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoProcessorContentDesc(this, ::core::mem::transmute_copy(&pcontentdesc)).into())
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckVideoProcessorFormat(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoProcessorCaps(this, ::core::mem::transmute_copy(&pcaps)).into())
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoProcessorRateConversionCaps(this, ::core::mem::transmute_copy(&typeindex), ::core::mem::transmute_copy(&pcaps)).into())
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVideoProcessorCustomRate(this, ::core::mem::transmute_copy(&typeindex), ::core::mem::transmute_copy(&customrateindex), ::core::mem::transmute_copy(&prate)).into())
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: D3D11_VIDEO_PROCESSOR_FILTER, prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVideoProcessorFilterRange(this, ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11VideoProcessorEnumerator_Vtbl {
            base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVideoProcessorContentDesc: GetVideoProcessorContentDesc::<Identity, Impl, OFFSET>,
            CheckVideoProcessorFormat: CheckVideoProcessorFormat::<Identity, Impl, OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Identity, Impl, OFFSET>,
            GetVideoProcessorRateConversionCaps: GetVideoProcessorRateConversionCaps::<Identity, Impl, OFFSET>,
            GetVideoProcessorCustomRate: GetVideoProcessorCustomRate::<Identity, Impl, OFFSET>,
            GetVideoProcessorFilterRange: GetVideoProcessorFilterRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11VideoProcessorEnumerator1_Impl: ::windows_core::BaseImpl + ID3D11VideoProcessorEnumerator_Impl {
    fn CheckVideoProcessorFormatConversion(this: &Self::This, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D11VideoProcessorEnumerator1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11VideoProcessorEnumerator);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoProcessorEnumerator1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckVideoProcessorFormatConversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, psupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckVideoProcessorFormatConversion(this, ::core::mem::transmute_copy(&inputformat), ::core::mem::transmute_copy(&inputcolorspace), ::core::mem::transmute_copy(&outputformat), ::core::mem::transmute_copy(&outputcolorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D11VideoProcessorEnumerator1_Vtbl {
            base__: <ID3D11VideoProcessorEnumerator as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CheckVideoProcessorFormatConversion: CheckVideoProcessorFormatConversion::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D11VideoProcessorInputView_Impl: ::windows_core::BaseImpl + ID3D11View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC);
}
impl ::windows_core::Iids for ID3D11VideoProcessorInputView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11View);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorInputView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoProcessorInputView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorInputView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11VideoProcessorInputView_Vtbl { base__: <ID3D11View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11VideoProcessorOutputView_Impl: ::windows_core::BaseImpl + ID3D11View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC);
}
impl ::windows_core::Iids for ID3D11VideoProcessorOutputView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11View);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorOutputView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11VideoProcessorOutputView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11VideoProcessorOutputView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D11VideoProcessorOutputView_Vtbl { base__: <ID3D11View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D11View_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {
    fn GetResource(this: &Self::This, ppresource: *mut ::core::option::Option<ID3D11Resource>);
}
impl ::windows_core::Iids for ID3D11View {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11View_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D11View {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D11View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResource(this, ::core::mem::transmute_copy(&ppresource)))
        }
        ID3D11View_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetResource: GetResource::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3DDeviceContextState_Impl: ::windows_core::BaseImpl + ID3D11DeviceChild_Impl {}
impl ::windows_core::Iids for ID3DDeviceContextState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D11DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DDeviceContextState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3DDeviceContextState {
    const VTABLE: Self::Vtable = { ID3DDeviceContextState_Vtbl { base__: <ID3D11DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3DUserDefinedAnnotation_Impl: ::windows_core::BaseImpl {
    fn BeginEvent(this: &Self::This, name: &::windows_core::PCWSTR) -> i32;
    fn EndEvent(this: &Self::This) -> i32;
    fn SetMarker(this: &Self::This, name: &::windows_core::PCWSTR);
    fn GetStatus(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3DUserDefinedAnnotation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3DUserDefinedAnnotation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEvent(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn EndEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndEvent(this))
        }
        unsafe extern "system" fn SetMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMarker(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this))
        }
        ID3DUserDefinedAnnotation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3DX11FFT_Impl: ::windows_core::BaseImpl {
    fn SetForwardScale(this: &Self::This, forwardscale: f32) -> ::windows_core::Result<()>;
    fn GetForwardScale(this: &Self::This) -> f32;
    fn SetInverseScale(this: &Self::This, inversescale: f32) -> ::windows_core::Result<()>;
    fn GetInverseScale(this: &Self::This) -> f32;
    fn AttachBuffersAndPrecompute(this: &Self::This, numtempbuffers: u32, pptempbuffers: *const ::core::option::Option<ID3D11UnorderedAccessView>, numprecomputebuffers: u32, ppprecomputebuffersizes: *const ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows_core::Result<()>;
    fn ForwardTransform(this: &Self::This, pinputbuffer: ::core::option::Option<&ID3D11UnorderedAccessView>, ppoutputbuffer: *mut ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows_core::Result<()>;
    fn InverseTransform(this: &Self::This, pinputbuffer: ::core::option::Option<&ID3D11UnorderedAccessView>, ppoutputbuffer: *mut ::core::option::Option<ID3D11UnorderedAccessView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3DX11FFT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3DX11FFT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetForwardScale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardscale: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForwardScale(this, ::core::mem::transmute_copy(&forwardscale)).into())
        }
        unsafe extern "system" fn GetForwardScale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForwardScale(this))
        }
        unsafe extern "system" fn SetInverseScale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inversescale: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInverseScale(this, ::core::mem::transmute_copy(&inversescale)).into())
        }
        unsafe extern "system" fn GetInverseScale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInverseScale(this))
        }
        unsafe extern "system" fn AttachBuffersAndPrecompute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numtempbuffers: u32, pptempbuffers: *const *mut ::core::ffi::c_void, numprecomputebuffers: u32, ppprecomputebuffersizes: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachBuffersAndPrecompute(this, ::core::mem::transmute_copy(&numtempbuffers), ::core::mem::transmute_copy(&pptempbuffers), ::core::mem::transmute_copy(&numprecomputebuffers), ::core::mem::transmute_copy(&ppprecomputebuffersizes)).into())
        }
        unsafe extern "system" fn ForwardTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputbuffer: *mut ::core::ffi::c_void, ppoutputbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForwardTransform(this, ::windows_core::from_raw_borrowed(&pinputbuffer), ::core::mem::transmute_copy(&ppoutputbuffer)).into())
        }
        unsafe extern "system" fn InverseTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputbuffer: *mut ::core::ffi::c_void, ppoutputbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InverseTransform(this, ::windows_core::from_raw_borrowed(&pinputbuffer), ::core::mem::transmute_copy(&ppoutputbuffer)).into())
        }
        ID3DX11FFT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetForwardScale: SetForwardScale::<Identity, Impl, OFFSET>,
            GetForwardScale: GetForwardScale::<Identity, Impl, OFFSET>,
            SetInverseScale: SetInverseScale::<Identity, Impl, OFFSET>,
            GetInverseScale: GetInverseScale::<Identity, Impl, OFFSET>,
            AttachBuffersAndPrecompute: AttachBuffersAndPrecompute::<Identity, Impl, OFFSET>,
            ForwardTransform: ForwardTransform::<Identity, Impl, OFFSET>,
            InverseTransform: InverseTransform::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3DX11Scan_Impl: ::windows_core::BaseImpl {
    fn SetScanDirection(this: &Self::This, direction: D3DX11_SCAN_DIRECTION) -> ::windows_core::Result<()>;
    fn Scan(this: &Self::This, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::core::option::Option<&ID3D11UnorderedAccessView>, pdst: ::core::option::Option<&ID3D11UnorderedAccessView>) -> ::windows_core::Result<()>;
    fn Multiscan(this: &Self::This, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: ::core::option::Option<&ID3D11UnorderedAccessView>, pdst: ::core::option::Option<&ID3D11UnorderedAccessView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3DX11Scan {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3DX11Scan {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetScanDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScanDirection(this, ::core::mem::transmute_copy(&direction)).into())
        }
        unsafe extern "system" fn Scan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: *mut ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Scan(this, ::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::windows_core::from_raw_borrowed(&psrc), ::windows_core::from_raw_borrowed(&pdst)).into())
        }
        unsafe extern "system" fn Multiscan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: *mut ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Multiscan(this, ::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::core::mem::transmute_copy(&elementscanpitch), ::core::mem::transmute_copy(&scancount), ::windows_core::from_raw_borrowed(&psrc), ::windows_core::from_raw_borrowed(&pdst)).into())
        }
        ID3DX11Scan_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetScanDirection: SetScanDirection::<Identity, Impl, OFFSET>,
            Scan: Scan::<Identity, Impl, OFFSET>,
            Multiscan: Multiscan::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3DX11SegmentedScan_Impl: ::windows_core::BaseImpl {
    fn SetScanDirection(this: &Self::This, direction: D3DX11_SCAN_DIRECTION) -> ::windows_core::Result<()>;
    fn SegScan(this: &Self::This, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: ::core::option::Option<&ID3D11UnorderedAccessView>, psrcelementflags: ::core::option::Option<&ID3D11UnorderedAccessView>, pdst: ::core::option::Option<&ID3D11UnorderedAccessView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3DX11SegmentedScan {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3DX11SegmentedScan {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetScanDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScanDirection(this, ::core::mem::transmute_copy(&direction)).into())
        }
        unsafe extern "system" fn SegScan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: *mut ::core::ffi::c_void, psrcelementflags: *mut ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SegScan(this, ::core::mem::transmute_copy(&elementtype), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&elementscansize), ::windows_core::from_raw_borrowed(&psrc), ::windows_core::from_raw_borrowed(&psrcelementflags), ::windows_core::from_raw_borrowed(&pdst)).into())
        }
        ID3DX11SegmentedScan_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetScanDirection: SetScanDirection::<Identity, Impl, OFFSET>,
            SegScan: SegScan::<Identity, Impl, OFFSET>,
        }
    };
}
