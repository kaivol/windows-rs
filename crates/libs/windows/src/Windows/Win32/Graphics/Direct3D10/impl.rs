pub trait ID3D10Asynchronous_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {
    fn Begin(this: &Self::This);
    fn End(this: &Self::This);
    fn GetData(this: &Self::This, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows_core::Result<()>;
    fn GetDataSize(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for ID3D10Asynchronous {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Asynchronous {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin(this))
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this))
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, datasize: u32, getdataflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&getdataflags)).into())
        }
        unsafe extern "system" fn GetDataSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Asynchronous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataSize(this))
        }
        ID3D10Asynchronous_Vtbl {
            base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataSize: GetDataSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendState_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_BLEND_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10BlendState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10BlendState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10BlendState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10BlendState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10BlendState_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10BlendState1_Impl: ::windows_core::BaseImpl + ID3D10BlendState_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut D3D10_BLEND_DESC1);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10BlendState1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10BlendState);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10BlendState1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10BlendState1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10BlendState1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10BlendState1_Vtbl { base__: <ID3D10BlendState as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D10Buffer_Impl: ::windows_core::BaseImpl + ID3D10Resource_Impl {
    fn Map(this: &Self::This, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Unmap(this: &Self::This);
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_BUFFER_DESC);
}
impl ::windows_core::Iids for ID3D10Buffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Buffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Map(this, ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags), ::core::mem::transmute_copy(&ppdata)).into())
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Buffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10Buffer_Vtbl {
            base__: <ID3D10Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D10Counter_Impl: ::windows_core::BaseImpl + ID3D10Asynchronous_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_COUNTER_DESC);
}
impl ::windows_core::Iids for ID3D10Counter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Asynchronous);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Counter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Counter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Counter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10Counter_Vtbl { base__: <ID3D10Asynchronous as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D10Debug_Impl: ::windows_core::BaseImpl {
    fn SetFeatureMask(this: &Self::This, mask: u32) -> ::windows_core::Result<()>;
    fn GetFeatureMask(this: &Self::This) -> u32;
    fn SetPresentPerRenderOpDelay(this: &Self::This, milliseconds: u32) -> ::windows_core::Result<()>;
    fn GetPresentPerRenderOpDelay(this: &Self::This) -> u32;
    fn SetSwapChain(this: &Self::This, pswapchain: ::core::option::Option<&super::Dxgi::IDXGISwapChain>) -> ::windows_core::Result<()>;
    fn GetSwapChain(this: &Self::This) -> ::windows_core::Result<super::Dxgi::IDXGISwapChain>;
    fn Validate(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::windows_core::Iids for ID3D10Debug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Debug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFeatureMask(this, ::core::mem::transmute_copy(&mask)).into())
        }
        unsafe extern "system" fn GetFeatureMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureMask(this))
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, milliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPresentPerRenderOpDelay(this, ::core::mem::transmute_copy(&milliseconds)).into())
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresentPerRenderOpDelay(this))
        }
        unsafe extern "system" fn SetSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pswapchain: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSwapChain(this, ::windows_core::from_raw_borrowed(&pswapchain)).into())
        }
        unsafe extern "system" fn GetSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSwapChain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Debug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Validate(this).into())
        }
        ID3D10Debug_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10DepthStencilState_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_DEPTH_STENCIL_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10DepthStencilState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DepthStencilState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10DepthStencilState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DepthStencilState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10DepthStencilState_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10DepthStencilView_Impl: ::windows_core::BaseImpl + ID3D10View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D10DepthStencilView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10View);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DepthStencilView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10DepthStencilView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DepthStencilView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10DepthStencilView_Vtbl { base__: <ID3D10View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device_Impl: ::windows_core::BaseImpl {
    fn VSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn PSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn PSSetShader(this: &Self::This, ppixelshader: ::core::option::Option<&ID3D10PixelShader>);
    fn PSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn VSSetShader(this: &Self::This, pvertexshader: ::core::option::Option<&ID3D10VertexShader>);
    fn DrawIndexed(this: &Self::This, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(this: &Self::This, vertexcount: u32, startvertexlocation: u32);
    fn PSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn IASetInputLayout(this: &Self::This, pinputlayout: ::core::option::Option<&ID3D10InputLayout>);
    fn IASetVertexBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppvertexbuffers: *const ::core::option::Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(this: &Self::This, pindexbuffer: ::core::option::Option<&ID3D10Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(this: &Self::This, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(this: &Self::This, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *const ::core::option::Option<ID3D10Buffer>);
    fn GSSetShader(this: &Self::This, pshader: ::core::option::Option<&ID3D10GeometryShader>);
    fn IASetPrimitiveTopology(this: &Self::This, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn VSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn SetPredication(this: &Self::This, ppredicate: ::core::option::Option<&ID3D10Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *const ::core::option::Option<ID3D10ShaderResourceView>);
    fn GSSetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *const ::core::option::Option<ID3D10SamplerState>);
    fn OMSetRenderTargets(this: &Self::This, numviews: u32, pprendertargetviews: *const ::core::option::Option<ID3D10RenderTargetView>, pdepthstencilview: ::core::option::Option<&ID3D10DepthStencilView>);
    fn OMSetBlendState(this: &Self::This, pblendstate: ::core::option::Option<&ID3D10BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(this: &Self::This, pdepthstencilstate: ::core::option::Option<&ID3D10DepthStencilState>, stencilref: u32);
    fn SOSetTargets(this: &Self::This, numbuffers: u32, ppsotargets: *const ::core::option::Option<ID3D10Buffer>, poffsets: *const u32);
    fn DrawAuto(this: &Self::This);
    fn RSSetState(this: &Self::This, prasterizerstate: ::core::option::Option<&ID3D10RasterizerState>);
    fn RSSetViewports(this: &Self::This, numviewports: u32, pviewports: *const D3D10_VIEWPORT);
    fn RSSetScissorRects(this: &Self::This, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(this: &Self::This, pdstresource: ::core::option::Option<&ID3D10Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: ::core::option::Option<&ID3D10Resource>, srcsubresource: u32, psrcbox: *const D3D10_BOX);
    fn CopyResource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D10Resource>, psrcresource: ::core::option::Option<&ID3D10Resource>);
    fn UpdateSubresource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D10Resource>, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ClearRenderTargetView(this: &Self::This, prendertargetview: ::core::option::Option<&ID3D10RenderTargetView>, colorrgba: *const f32);
    fn ClearDepthStencilView(this: &Self::This, pdepthstencilview: ::core::option::Option<&ID3D10DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(this: &Self::This, pshaderresourceview: ::core::option::Option<&ID3D10ShaderResourceView>);
    fn ResolveSubresource(this: &Self::This, pdstresource: ::core::option::Option<&ID3D10Resource>, dstsubresource: u32, psrcresource: ::core::option::Option<&ID3D10Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn VSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn PSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn PSGetShader(this: &Self::This, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>);
    fn PSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn VSGetShader(this: &Self::This, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>);
    fn PSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn IAGetInputLayout(this: &Self::This, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>);
    fn IAGetVertexBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut ::core::option::Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(this: &Self::This, pindexbuffer: *mut ::core::option::Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(this: &Self::This, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut ::core::option::Option<ID3D10Buffer>);
    fn GSGetShader(this: &Self::This, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>);
    fn IAGetPrimitiveTopology(this: &Self::This, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn VSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn GetPredication(this: &Self::This, pppredicate: *mut ::core::option::Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(this: &Self::This, startslot: u32, numviews: u32, ppshaderresourceviews: *mut ::core::option::Option<ID3D10ShaderResourceView>);
    fn GSGetSamplers(this: &Self::This, startslot: u32, numsamplers: u32, ppsamplers: *mut ::core::option::Option<ID3D10SamplerState>);
    fn OMGetRenderTargets(this: &Self::This, numviews: u32, pprendertargetviews: *mut ::core::option::Option<ID3D10RenderTargetView>, ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>);
    fn OMGetBlendState(this: &Self::This, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(this: &Self::This, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(this: &Self::This, numbuffers: u32, ppsotargets: *mut ::core::option::Option<ID3D10Buffer>, poffsets: *mut u32);
    fn RSGetState(this: &Self::This, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>);
    fn RSGetViewports(this: &Self::This, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT);
    fn RSGetScissorRects(this: &Self::This, numrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn GetDeviceRemovedReason(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetExceptionMode(this: &Self::This, raiseflags: u32) -> ::windows_core::Result<()>;
    fn GetExceptionMode(this: &Self::This) -> u32;
    fn GetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, guid: *const ::windows_core::GUID, pdata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ClearState(this: &Self::This);
    fn Flush(this: &Self::This);
    fn CreateBuffer(this: &Self::This, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut ::core::option::Option<ID3D10Buffer>) -> ::windows_core::Result<()>;
    fn CreateTexture1D(this: &Self::This, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows_core::Result<ID3D10Texture1D>;
    fn CreateTexture2D(this: &Self::This, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows_core::Result<ID3D10Texture2D>;
    fn CreateTexture3D(this: &Self::This, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> ::windows_core::Result<ID3D10Texture3D>;
    fn CreateShaderResourceView(this: &Self::This, presource: ::core::option::Option<&ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut ::core::option::Option<ID3D10ShaderResourceView>) -> ::windows_core::Result<()>;
    fn CreateRenderTargetView(this: &Self::This, presource: ::core::option::Option<&ID3D10Resource>, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut ::core::option::Option<ID3D10RenderTargetView>) -> ::windows_core::Result<()>;
    fn CreateDepthStencilView(this: &Self::This, presource: ::core::option::Option<&ID3D10Resource>, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut ::core::option::Option<ID3D10DepthStencilView>) -> ::windows_core::Result<()>;
    fn CreateInputLayout(this: &Self::This, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut ::core::option::Option<ID3D10InputLayout>) -> ::windows_core::Result<()>;
    fn CreateVertexShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut ::core::option::Option<ID3D10VertexShader>) -> ::windows_core::Result<()>;
    fn CreateGeometryShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>) -> ::windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut ::core::option::Option<ID3D10GeometryShader>) -> ::windows_core::Result<()>;
    fn CreatePixelShader(this: &Self::This, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut ::core::option::Option<ID3D10PixelShader>) -> ::windows_core::Result<()>;
    fn CreateBlendState(this: &Self::This, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut ::core::option::Option<ID3D10BlendState>) -> ::windows_core::Result<()>;
    fn CreateDepthStencilState(this: &Self::This, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut ::core::option::Option<ID3D10DepthStencilState>) -> ::windows_core::Result<()>;
    fn CreateRasterizerState(this: &Self::This, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut ::core::option::Option<ID3D10RasterizerState>) -> ::windows_core::Result<()>;
    fn CreateSamplerState(this: &Self::This, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut ::core::option::Option<ID3D10SamplerState>) -> ::windows_core::Result<()>;
    fn CreateQuery(this: &Self::This, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut ::core::option::Option<ID3D10Query>) -> ::windows_core::Result<()>;
    fn CreatePredicate(this: &Self::This, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut ::core::option::Option<ID3D10Predicate>) -> ::windows_core::Result<()>;
    fn CreateCounter(this: &Self::This, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut ::core::option::Option<ID3D10Counter>) -> ::windows_core::Result<()>;
    fn CheckFormatSupport(this: &Self::This, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(this: &Self::This, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows_core::Result<u32>;
    fn CheckCounterInfo(this: &Self::This, pcounterinfo: *mut D3D10_COUNTER_INFO);
    fn CheckCounter(this: &Self::This, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows_core::PSTR, pnamelength: *mut u32, szunits: ::windows_core::PSTR, punitslength: *mut u32, szdescription: ::windows_core::PSTR, pdescriptionlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetCreationFlags(this: &Self::This) -> u32;
    fn OpenSharedResource(this: &Self::This, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetTextFilterSize(this: &Self::This, width: u32, height: u32);
    fn GetTextFilterSize(this: &Self::This, pwidth: *mut u32, pheight: *mut u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D10Device {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Device {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn PSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppixelshader: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetShader(this, ::windows_core::from_raw_borrowed(&ppixelshader)))
        }
        unsafe extern "system" fn PSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn VSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvertexshader: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetShader(this, ::windows_core::from_raw_borrowed(&pvertexshader)))
        }
        unsafe extern "system" fn DrawIndexed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexed(this, ::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation)))
        }
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::core::mem::transmute_copy(&vertexcount), ::core::mem::transmute_copy(&startvertexlocation)))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn IASetInputLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputlayout: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetInputLayout(this, ::windows_core::from_raw_borrowed(&pinputlayout)))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut ::core::ffi::c_void, pstrides: *const u32, poffsets: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetVertexBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets)))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetIndexBuffer(this, ::windows_core::from_raw_borrowed(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset)))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawIndexedInstanced(this, ::core::mem::transmute_copy(&indexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startindexlocation), ::core::mem::transmute_copy(&basevertexlocation), ::core::mem::transmute_copy(&startinstancelocation)))
        }
        unsafe extern "system" fn DrawInstanced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInstanced(this, ::core::mem::transmute_copy(&vertexcountperinstance), ::core::mem::transmute_copy(&instancecount), ::core::mem::transmute_copy(&startvertexlocation), ::core::mem::transmute_copy(&startinstancelocation)))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn GSSetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetShader(this, ::windows_core::from_raw_borrowed(&pshader)))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IASetPrimitiveTopology(this, ::core::mem::transmute_copy(&topology)))
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn VSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn SetPredication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppredicate: *mut ::core::ffi::c_void, predicatevalue: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPredication(this, ::windows_core::from_raw_borrowed(&ppredicate), ::core::mem::transmute_copy(&predicatevalue)))
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn GSSetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSSetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetRenderTargets(this, ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::windows_core::from_raw_borrowed(&pdepthstencilview)))
        }
        unsafe extern "system" fn OMSetBlendState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblendstate: *mut ::core::ffi::c_void, blendfactor: *const f32, samplemask: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetBlendState(this, ::windows_core::from_raw_borrowed(&pblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&samplemask)))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdepthstencilstate: *mut ::core::ffi::c_void, stencilref: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMSetDepthStencilState(this, ::windows_core::from_raw_borrowed(&pdepthstencilstate), ::core::mem::transmute_copy(&stencilref)))
        }
        unsafe extern "system" fn SOSetTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut ::core::ffi::c_void, poffsets: *const u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SOSetTargets(this, ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets)))
        }
        unsafe extern "system" fn DrawAuto<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawAuto(this))
        }
        unsafe extern "system" fn RSSetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterizerstate: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetState(this, ::windows_core::from_raw_borrowed(&prasterizerstate)))
        }
        unsafe extern "system" fn RSSetViewports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetViewports(this, ::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports)))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSSetScissorRects(this, ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D10_BOX) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopySubresourceRegion(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&dstx), ::core::mem::transmute_copy(&dsty), ::core::mem::transmute_copy(&dstz), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&psrcbox)))
        }
        unsafe extern "system" fn CopyResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, psrcresource: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyResource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::windows_core::from_raw_borrowed(&psrcresource)))
        }
        unsafe extern "system" fn UpdateSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateSubresource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::core::mem::transmute_copy(&pdstbox), ::core::mem::transmute_copy(&psrcdata), ::core::mem::transmute_copy(&srcrowpitch), ::core::mem::transmute_copy(&srcdepthpitch)))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prendertargetview: *mut ::core::ffi::c_void, colorrgba: *const f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRenderTargetView(this, ::windows_core::from_raw_borrowed(&prendertargetview), ::core::mem::transmute_copy(&colorrgba)))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdepthstencilview: *mut ::core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearDepthStencilView(this, ::windows_core::from_raw_borrowed(&pdepthstencilview), ::core::mem::transmute_copy(&clearflags), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&stencil)))
        }
        unsafe extern "system" fn GenerateMips<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderresourceview: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateMips(this, ::windows_core::from_raw_borrowed(&pshaderresourceview)))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdstresource: *mut ::core::ffi::c_void, dstsubresource: u32, psrcresource: *mut ::core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveSubresource(this, ::windows_core::from_raw_borrowed(&pdstresource), ::core::mem::transmute_copy(&dstsubresource), ::windows_core::from_raw_borrowed(&psrcresource), ::core::mem::transmute_copy(&srcsubresource), ::core::mem::transmute_copy(&format)))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn PSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppixelshader: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetShader(this, ::core::mem::transmute_copy(&pppixelshader)))
        }
        unsafe extern "system" fn PSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn VSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvertexshader: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetShader(this, ::core::mem::transmute_copy(&ppvertexshader)))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinputlayout: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetInputLayout(this, ::core::mem::transmute_copy(&ppinputlayout)))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut ::core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetVertexBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppvertexbuffers), ::core::mem::transmute_copy(&pstrides), ::core::mem::transmute_copy(&poffsets)))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexbuffer: *mut *mut ::core::ffi::c_void, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetIndexBuffer(this, ::core::mem::transmute_copy(&pindexbuffer), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&offset)))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetConstantBuffers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppconstantbuffers)))
        }
        unsafe extern "system" fn GSGetShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgeometryshader: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetShader(this, ::core::mem::transmute_copy(&ppgeometryshader)))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IAGetPrimitiveTopology(this, ::core::mem::transmute_copy(&ptopology)))
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn VSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn GetPredication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppredicate: *mut *mut ::core::ffi::c_void, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPredication(this, ::core::mem::transmute_copy(&pppredicate), ::core::mem::transmute_copy(&ppredicatevalue)))
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetShaderResources(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&ppshaderresourceviews)))
        }
        unsafe extern "system" fn GSGetSamplers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GSGetSamplers(this, ::core::mem::transmute_copy(&startslot), ::core::mem::transmute_copy(&numsamplers), ::core::mem::transmute_copy(&ppsamplers)))
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut ::core::ffi::c_void, ppdepthstencilview: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMGetRenderTargets(this, ::core::mem::transmute_copy(&numviews), ::core::mem::transmute_copy(&pprendertargetviews), ::core::mem::transmute_copy(&ppdepthstencilview)))
        }
        unsafe extern "system" fn OMGetBlendState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppblendstate: *mut *mut ::core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMGetBlendState(this, ::core::mem::transmute_copy(&ppblendstate), ::core::mem::transmute_copy(&blendfactor), ::core::mem::transmute_copy(&psamplemask)))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdepthstencilstate: *mut *mut ::core::ffi::c_void, pstencilref: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OMGetDepthStencilState(this, ::core::mem::transmute_copy(&ppdepthstencilstate), ::core::mem::transmute_copy(&pstencilref)))
        }
        unsafe extern "system" fn SOGetTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut ::core::ffi::c_void, poffsets: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SOGetTargets(this, ::core::mem::transmute_copy(&numbuffers), ::core::mem::transmute_copy(&ppsotargets), ::core::mem::transmute_copy(&poffsets)))
        }
        unsafe extern "system" fn RSGetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprasterizerstate: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSGetState(this, ::core::mem::transmute_copy(&pprasterizerstate)))
        }
        unsafe extern "system" fn RSGetViewports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSGetViewports(this, ::core::mem::transmute_copy(&numviewports), ::core::mem::transmute_copy(&pviewports)))
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RSGetScissorRects(this, ::core::mem::transmute_copy(&numrects), ::core::mem::transmute_copy(&prects)))
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceRemovedReason(this).into())
        }
        unsafe extern "system" fn SetExceptionMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, raiseflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExceptionMode(this, ::core::mem::transmute_copy(&raiseflags)).into())
        }
        unsafe extern "system" fn GetExceptionMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExceptionMode(this))
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&guid), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        unsafe extern "system" fn ClearState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearState(this))
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this))
        }
        unsafe extern "system" fn CreateBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBuffer(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata), ::core::mem::transmute_copy(&ppbuffer)).into())
        }
        unsafe extern "system" fn CreateTexture1D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTexture1D(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture1d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTexture2D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTexture2D(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture2d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTexture3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTexture3D(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pinitialdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexture3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateShaderResourceView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppsrview)).into())
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRenderTargetView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pprtview)).into())
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDepthStencilView(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppdepthstencilview)).into())
        }
        unsafe extern "system" fn CreateInputLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const ::core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInputLayout(this, ::core::mem::transmute_copy(&pinputelementdescs), ::core::mem::transmute_copy(&numelements), ::core::mem::transmute_copy(&pshaderbytecodewithinputsignature), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&ppinputlayout)).into())
        }
        unsafe extern "system" fn CreateVertexShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVertexShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&ppvertexshader)).into())
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateGeometryShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&ppgeometryshader)).into())
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateGeometryShaderWithStreamOutput(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&psodeclaration), ::core::mem::transmute_copy(&numentries), ::core::mem::transmute_copy(&outputstreamstride), ::core::mem::transmute_copy(&ppgeometryshader)).into())
        }
        unsafe extern "system" fn CreatePixelShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePixelShader(this, ::core::mem::transmute_copy(&pshaderbytecode), ::core::mem::transmute_copy(&bytecodelength), ::core::mem::transmute_copy(&pppixelshader)).into())
        }
        unsafe extern "system" fn CreateBlendState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBlendState(this, ::core::mem::transmute_copy(&pblendstatedesc), ::core::mem::transmute_copy(&ppblendstate)).into())
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDepthStencilState(this, ::core::mem::transmute_copy(&pdepthstencildesc), ::core::mem::transmute_copy(&ppdepthstencilstate)).into())
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRasterizerState(this, ::core::mem::transmute_copy(&prasterizerdesc), ::core::mem::transmute_copy(&pprasterizerstate)).into())
        }
        unsafe extern "system" fn CreateSamplerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSamplerState(this, ::core::mem::transmute_copy(&psamplerdesc), ::core::mem::transmute_copy(&ppsamplerstate)).into())
        }
        unsafe extern "system" fn CreateQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateQuery(this, ::core::mem::transmute_copy(&pquerydesc), ::core::mem::transmute_copy(&ppquery)).into())
        }
        unsafe extern "system" fn CreatePredicate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePredicate(this, ::core::mem::transmute_copy(&ppredicatedesc), ::core::mem::transmute_copy(&pppredicate)).into())
        }
        unsafe extern "system" fn CreateCounter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCounter(this, ::core::mem::transmute_copy(&pcounterdesc), ::core::mem::transmute_copy(&ppcounter)).into())
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckFormatSupport(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckMultisampleQualityLevels(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&samplecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumqualitylevels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckCounterInfo(this, ::core::mem::transmute_copy(&pcounterinfo)))
        }
        unsafe extern "system" fn CheckCounter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows_core::PSTR, pnamelength: *mut u32, szunits: ::windows_core::PSTR, punitslength: *mut u32, szdescription: ::windows_core::PSTR, pdescriptionlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckCounter(this, ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pactivecounters), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pnamelength), ::core::mem::transmute_copy(&szunits), ::core::mem::transmute_copy(&punitslength), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pdescriptionlength)).into())
        }
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationFlags(this))
        }
        unsafe extern "system" fn OpenSharedResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const ::windows_core::GUID, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenSharedResource(this, ::core::mem::transmute_copy(&hresource), ::core::mem::transmute_copy(&returnedinterface), ::core::mem::transmute_copy(&ppresource)).into())
        }
        unsafe extern "system" fn SetTextFilterSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextFilterSize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)))
        }
        unsafe extern "system" fn GetTextFilterSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextFilterSize(this, ::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)))
        }
        ID3D10Device_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, Impl, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, Impl, OFFSET>,
            PSSetShader: PSSetShader::<Identity, Impl, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, Impl, OFFSET>,
            VSSetShader: VSSetShader::<Identity, Impl, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
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
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, Impl, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, Impl, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            DrawAuto: DrawAuto::<Identity, Impl, OFFSET>,
            RSSetState: RSSetState::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            GenerateMips: GenerateMips::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
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
            OMGetBlendState: OMGetBlendState::<Identity, Impl, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, Impl, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, Impl, OFFSET>,
            RSGetState: RSGetState::<Identity, Impl, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, Impl, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, Impl, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            CreateBuffer: CreateBuffer::<Identity, Impl, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, Impl, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, Impl, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, Impl, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, Impl, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, Impl, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, Impl, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, Impl, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, Impl, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, Impl, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, Impl, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, Impl, OFFSET>,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, Impl, OFFSET>,
            CreateCounter: CreateCounter::<Identity, Impl, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, Impl, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, Impl, OFFSET>,
            CheckCounter: CheckCounter::<Identity, Impl, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, Impl, OFFSET>,
            SetTextFilterSize: SetTextFilterSize::<Identity, Impl, OFFSET>,
            GetTextFilterSize: GetTextFilterSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device1_Impl: ::windows_core::BaseImpl + ID3D10Device_Impl {
    fn CreateShaderResourceView1(this: &Self::This, presource: ::core::option::Option<&ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut ::core::option::Option<ID3D10ShaderResourceView1>) -> ::windows_core::Result<()>;
    fn CreateBlendState1(this: &Self::This, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut ::core::option::Option<ID3D10BlendState1>) -> ::windows_core::Result<()>;
    fn GetFeatureLevel(this: &Self::This) -> D3D10_FEATURE_LEVEL1;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D10Device1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Device);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Device1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateShaderResourceView1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateShaderResourceView1(this, ::windows_core::from_raw_borrowed(&presource), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppsrview)).into())
        }
        unsafe extern "system" fn CreateBlendState1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBlendState1(this, ::core::mem::transmute_copy(&pblendstatedesc), ::core::mem::transmute_copy(&ppblendstate)).into())
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D3D10_FEATURE_LEVEL1 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureLevel(this))
        }
        ID3D10Device1_Vtbl {
            base__: <ID3D10Device as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, Impl, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, Impl, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D10DeviceChild_Impl: ::windows_core::BaseImpl {
    fn GetDevice(this: &Self::This, ppdevice: *mut ::core::option::Option<ID3D10Device>);
    fn GetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateData(this: &Self::This, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetPrivateDataInterface(this: &Self::This, guid: *const ::windows_core::GUID, pdata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3D10DeviceChild {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10DeviceChild {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevice(this, ::core::mem::transmute_copy(&ppdevice)))
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10DeviceChild_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivateDataInterface(this, ::core::mem::transmute_copy(&guid), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        ID3D10DeviceChild_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10Effect_Impl: ::windows_core::BaseImpl {
    fn IsValid(this: &Self::This) -> super::super::Foundation::BOOL;
    fn IsPool(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<ID3D10Device>;
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows_core::Result<()>;
    fn GetConstantBufferByIndex(this: &Self::This, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn GetConstantBufferByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn GetVariableByIndex(this: &Self::This, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetVariableByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetVariableBySemantic(this: &Self::This, semantic: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetTechniqueByIndex(this: &Self::This, index: u32) -> ::core::option::Option<ID3D10EffectTechnique>;
    fn GetTechniqueByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectTechnique>;
    fn Optimize(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsOptimized(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10Effect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Effect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsValid(this))
        }
        unsafe extern "system" fn IsPool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPool(this))
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVariableByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVariableByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetVariableBySemantic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, semantic: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVariableBySemantic(this, ::core::mem::transmute(&semantic)))
        }
        unsafe extern "system" fn GetTechniqueByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectTechnique> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTechniqueByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetTechniqueByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectTechnique> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTechniqueByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn Optimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Optimize(this).into())
        }
        unsafe extern "system" fn IsOptimized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsOptimized(this))
        }
        ID3D10Effect_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            IsPool: IsPool::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetVariableBySemantic: GetVariableBySemantic::<Identity, Impl, OFFSET>,
            GetTechniqueByIndex: GetTechniqueByIndex::<Identity, Impl, OFFSET>,
            GetTechniqueByName: GetTechniqueByName::<Identity, Impl, OFFSET>,
            Optimize: Optimize::<Identity, Impl, OFFSET>,
            IsOptimized: IsOptimized::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectBlendVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetBlendState(&self, index: u32) -> ::windows_core::Result<ID3D10BlendState>;
    fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectBlendVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectBlendVariable_Impl>() -> ID3D10EffectBlendVariable_Vtbl {
        unsafe extern "system" fn GetBlendState<Impl: ID3D10EffectBlendVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, ppblendstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetBlendState(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppblendstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectBlendVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBackingStore(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pblenddesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetBlendState: GetBlendState::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectBlendVariable_ImplVtbl<T: ID3D10EffectBlendVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectBlendVariable_Impl> ID3D10EffectBlendVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectBlendVariable_Vtbl = ID3D10EffectBlendVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectBlendVariable {
    pub fn new<'a, T: ID3D10EffectBlendVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectBlendVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectConstantBuffer_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetConstantBuffer(&self, pconstantbuffer: ::core::option::Option<&ID3D10Buffer>) -> ::windows_core::Result<()>;
    fn GetConstantBuffer(&self) -> ::windows_core::Result<ID3D10Buffer>;
    fn SetTextureBuffer(&self, ptexturebuffer: ::core::option::Option<&ID3D10ShaderResourceView>) -> ::windows_core::Result<()>;
    fn GetTextureBuffer(&self) -> ::windows_core::Result<ID3D10ShaderResourceView>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D10EffectConstantBuffer_Impl>() -> ID3D10EffectConstantBuffer_Vtbl {
        unsafe extern "system" fn SetConstantBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, pconstantbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetConstantBuffer(this, ::windows_core::from_raw_borrowed(&pconstantbuffer)).into()
        }
        unsafe extern "system" fn GetConstantBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, ppconstantbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetConstantBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconstantbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, ptexturebuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetTextureBuffer(this, ::windows_core::from_raw_borrowed(&ptexturebuffer)).into()
        }
        unsafe extern "system" fn GetTextureBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, pptexturebuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetTextureBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptexturebuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetConstantBuffer: SetConstantBuffer::<Impl>,
            GetConstantBuffer: GetConstantBuffer::<Impl>,
            SetTextureBuffer: SetTextureBuffer::<Impl>,
            GetTextureBuffer: GetTextureBuffer::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectConstantBuffer_ImplVtbl<T: ID3D10EffectConstantBuffer_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectConstantBuffer_Impl> ID3D10EffectConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10EffectConstantBuffer_Vtbl = ID3D10EffectConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectConstantBuffer {
    pub fn new<'a, T: ID3D10EffectConstantBuffer_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetDepthStencilState(&self, index: u32) -> ::windows_core::Result<ID3D10DepthStencilState>;
    fn GetBackingStore(&self, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectDepthStencilVariable_Impl>() -> ID3D10EffectDepthStencilVariable_Vtbl {
        unsafe extern "system" fn GetDepthStencilState<Impl: ID3D10EffectDepthStencilVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, ppdepthstencilstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetDepthStencilState(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdepthstencilstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectDepthStencilVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBackingStore(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pdepthstencildesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetDepthStencilState: GetDepthStencilState::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectDepthStencilVariable_ImplVtbl<T: ID3D10EffectDepthStencilVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectDepthStencilVariable_Impl> ID3D10EffectDepthStencilVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilVariable_Vtbl = ID3D10EffectDepthStencilVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectDepthStencilViewVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetDepthStencil(&self, presource: ::core::option::Option<&ID3D10DepthStencilView>) -> ::windows_core::Result<()>;
    fn GetDepthStencil(&self) -> ::windows_core::Result<ID3D10DepthStencilView>;
    fn SetDepthStencilArray(&self, ppresources: *const ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetDepthStencilArray(&self, ppresources: *mut ::core::option::Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectDepthStencilViewVariable_Impl>() -> ID3D10EffectDepthStencilViewVariable_Vtbl {
        unsafe extern "system" fn SetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetDepthStencil(this, ::windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn GetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetDepthStencil(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetDepthStencilArray(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDepthStencilArray(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetDepthStencil: SetDepthStencil::<Impl>,
            GetDepthStencil: GetDepthStencil::<Impl>,
            SetDepthStencilArray: SetDepthStencilArray::<Impl>,
            GetDepthStencilArray: GetDepthStencilArray::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectDepthStencilViewVariable_ImplVtbl<T: ID3D10EffectDepthStencilViewVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectDepthStencilViewVariable_Impl> ID3D10EffectDepthStencilViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilViewVariable_Vtbl = ID3D10EffectDepthStencilViewVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectDepthStencilViewVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilViewVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectMatrixVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetMatrix(&self, pdata: *mut f32) -> ::windows_core::Result<()>;
    fn GetMatrix(&self, pdata: *mut f32) -> ::windows_core::Result<()>;
    fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn SetMatrixTranspose(&self, pdata: *mut f32) -> ::windows_core::Result<()>;
    fn GetMatrixTranspose(&self, pdata: *mut f32) -> ::windows_core::Result<()>;
    fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectMatrixVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectMatrixVariable_Impl>() -> ID3D10EffectMatrixVariable_Vtbl {
        unsafe extern "system" fn SetMatrix<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetMatrix(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrix<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMatrix(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetMatrixArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMatrixArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetMatrixTranspose<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetMatrixTranspose(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrixTranspose<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMatrixTranspose(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetMatrixTransposeArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMatrixTransposeArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetMatrix: SetMatrix::<Impl>,
            GetMatrix: GetMatrix::<Impl>,
            SetMatrixArray: SetMatrixArray::<Impl>,
            GetMatrixArray: GetMatrixArray::<Impl>,
            SetMatrixTranspose: SetMatrixTranspose::<Impl>,
            GetMatrixTranspose: GetMatrixTranspose::<Impl>,
            SetMatrixTransposeArray: SetMatrixTransposeArray::<Impl>,
            GetMatrixTransposeArray: GetMatrixTransposeArray::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectMatrixVariable_ImplVtbl<T: ID3D10EffectMatrixVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectMatrixVariable_Impl> ID3D10EffectMatrixVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectMatrixVariable_Vtbl = ID3D10EffectMatrixVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectMatrixVariable {
    pub fn new<'a, T: ID3D10EffectMatrixVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectMatrixVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectPass_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> ::windows_core::Result<()>;
    fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn Apply(&self, flags: u32) -> ::windows_core::Result<()>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectPass_Vtbl {
    pub const fn new<Impl: ID3D10EffectPass_Impl>() -> ID3D10EffectPass_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsValid(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVertexShaderDesc<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVertexShaderDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetGeometryShaderDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetPixelShaderDesc<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetPixelShaderDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetAnnotationByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetAnnotationByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn Apply<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::Apply(this, ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectPass_Impl>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::ComputeStateBlockMask(this, ::core::mem::transmute_copy(&pstateblockmask)).into()
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetVertexShaderDesc: GetVertexShaderDesc::<Impl>,
            GetGeometryShaderDesc: GetGeometryShaderDesc::<Impl>,
            GetPixelShaderDesc: GetPixelShaderDesc::<Impl>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl>,
            GetAnnotationByName: GetAnnotationByName::<Impl>,
            Apply: Apply::<Impl>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectPass_ImplVtbl<T: ID3D10EffectPass_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectPass_Impl> ID3D10EffectPass_ImplVtbl<T> {
    const VTABLE: ID3D10EffectPass_Vtbl = ID3D10EffectPass_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectPass {
    pub fn new<'a, T: ID3D10EffectPass_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectPass_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait ID3D10EffectPool_Impl: ::windows_core::BaseImpl {
    fn AsEffect(this: &Self::This) -> ::core::option::Option<ID3D10Effect>;
}
impl ::windows_core::Iids for ID3D10EffectPool {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10EffectPool_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10EffectPool {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10EffectPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10Effect> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsEffect(this))
        }
        ID3D10EffectPool_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AsEffect: AsEffect::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRasterizerVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetRasterizerState(&self, index: u32) -> ::windows_core::Result<ID3D10RasterizerState>;
    fn GetBackingStore(&self, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRasterizerVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectRasterizerVariable_Impl>() -> ID3D10EffectRasterizerVariable_Vtbl {
        unsafe extern "system" fn GetRasterizerState<Impl: ID3D10EffectRasterizerVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, pprasterizerstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetRasterizerState(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprasterizerstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectRasterizerVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBackingStore(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&prasterizerdesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetRasterizerState: GetRasterizerState::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectRasterizerVariable_ImplVtbl<T: ID3D10EffectRasterizerVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectRasterizerVariable_Impl> ID3D10EffectRasterizerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRasterizerVariable_Vtbl = ID3D10EffectRasterizerVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRasterizerVariable {
    pub fn new<'a, T: ID3D10EffectRasterizerVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectRasterizerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectRenderTargetViewVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetRenderTarget(&self, presource: ::core::option::Option<&ID3D10RenderTargetView>) -> ::windows_core::Result<()>;
    fn GetRenderTarget(&self) -> ::windows_core::Result<ID3D10RenderTargetView>;
    fn SetRenderTargetArray(&self, ppresources: *const ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetRenderTargetArray(&self, ppresources: *mut ::core::option::Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectRenderTargetViewVariable_Impl>() -> ID3D10EffectRenderTargetViewVariable_Vtbl {
        unsafe extern "system" fn SetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetRenderTarget(this, ::windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn GetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetRenderTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetRenderTargetArray(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetRenderTargetArray(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetRenderTarget: SetRenderTarget::<Impl>,
            GetRenderTarget: GetRenderTarget::<Impl>,
            SetRenderTargetArray: SetRenderTargetArray::<Impl>,
            GetRenderTargetArray: GetRenderTargetArray::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectRenderTargetViewVariable_ImplVtbl<T: ID3D10EffectRenderTargetViewVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectRenderTargetViewVariable_Impl> ID3D10EffectRenderTargetViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRenderTargetViewVariable_Vtbl = ID3D10EffectRenderTargetViewVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectRenderTargetViewVariable {
    pub fn new<'a, T: ID3D10EffectRenderTargetViewVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectRenderTargetViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectSamplerVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetSampler(&self, index: u32) -> ::windows_core::Result<ID3D10SamplerState>;
    fn GetBackingStore(&self, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectSamplerVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectSamplerVariable_Impl>() -> ID3D10EffectSamplerVariable_Vtbl {
        unsafe extern "system" fn GetSampler<Impl: ID3D10EffectSamplerVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, ppsampler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetSampler(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsampler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectSamplerVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBackingStore(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&psamplerdesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetSampler: GetSampler::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectSamplerVariable_ImplVtbl<T: ID3D10EffectSamplerVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectSamplerVariable_Impl> ID3D10EffectSamplerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectSamplerVariable_Vtbl = ID3D10EffectSamplerVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectSamplerVariable {
    pub fn new<'a, T: ID3D10EffectSamplerVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectSamplerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectScalarVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetFloat(&self, value: f32) -> ::windows_core::Result<()>;
    fn GetFloat(&self) -> ::windows_core::Result<f32>;
    fn SetFloatArray(&self, pdata: *const f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetFloatArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn SetInt(&self, value: i32) -> ::windows_core::Result<()>;
    fn GetInt(&self) -> ::windows_core::Result<i32>;
    fn SetIntArray(&self, pdata: *const i32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetIntArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn SetBool(&self, value: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBool(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBoolArray(&self, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetBoolArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectScalarVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectScalarVariable_Impl>() -> ID3D10EffectScalarVariable_Vtbl {
        unsafe extern "system" fn SetFloat<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetFloat(this, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFloat<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetFloat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetFloatArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetFloatArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetInt<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetInt(this, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetInt<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetInt(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetIntArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetIntArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBool<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetBool(this, ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBool<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetBool(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetBoolArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBoolArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetFloat: SetFloat::<Impl>,
            GetFloat: GetFloat::<Impl>,
            SetFloatArray: SetFloatArray::<Impl>,
            GetFloatArray: GetFloatArray::<Impl>,
            SetInt: SetInt::<Impl>,
            GetInt: GetInt::<Impl>,
            SetIntArray: SetIntArray::<Impl>,
            GetIntArray: GetIntArray::<Impl>,
            SetBool: SetBool::<Impl>,
            GetBool: GetBool::<Impl>,
            SetBoolArray: SetBoolArray::<Impl>,
            GetBoolArray: GetBoolArray::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectScalarVariable_ImplVtbl<T: ID3D10EffectScalarVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectScalarVariable_Impl> ID3D10EffectScalarVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectScalarVariable_Vtbl = ID3D10EffectScalarVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectScalarVariable {
    pub fn new<'a, T: ID3D10EffectScalarVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectScalarVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectShaderResourceVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetResource(&self, presource: ::core::option::Option<&ID3D10ShaderResourceView>) -> ::windows_core::Result<()>;
    fn GetResource(&self) -> ::windows_core::Result<ID3D10ShaderResourceView>;
    fn SetResourceArray(&self, ppresources: *const ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetResourceArray(&self, ppresources: *mut ::core::option::Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectShaderResourceVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectShaderResourceVariable_Impl>() -> ID3D10EffectShaderResourceVariable_Vtbl {
        unsafe extern "system" fn SetResource<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut ::core::ffi::c_void, presource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetResource(this, ::windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn GetResource<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceArray<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetResourceArray(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetResourceArray<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetResourceArray(this, ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetResource: SetResource::<Impl>,
            GetResource: GetResource::<Impl>,
            SetResourceArray: SetResourceArray::<Impl>,
            GetResourceArray: GetResourceArray::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectShaderResourceVariable_ImplVtbl<T: ID3D10EffectShaderResourceVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectShaderResourceVariable_Impl> ID3D10EffectShaderResourceVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderResourceVariable_Vtbl = ID3D10EffectShaderResourceVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectShaderResourceVariable {
    pub fn new<'a, T: ID3D10EffectShaderResourceVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectShaderResourceVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectShaderVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetShaderDesc(&self, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetVertexShader(&self, shaderindex: u32) -> ::windows_core::Result<ID3D10VertexShader>;
    fn GetGeometryShader(&self, shaderindex: u32) -> ::windows_core::Result<ID3D10GeometryShader>;
    fn GetPixelShader(&self, shaderindex: u32) -> ::windows_core::Result<ID3D10PixelShader>;
    fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectShaderVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectShaderVariable_Impl>() -> ID3D10EffectShaderVariable_Vtbl {
        unsafe extern "system" fn GetShaderDesc<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut ::core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetShaderDesc(this, ::core::mem::transmute_copy(&shaderindex), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVertexShader<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppvs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetVertexShader(this, ::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryShader<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppgs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetGeometryShader(this, ::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShader<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut ::core::ffi::c_void, shaderindex: u32, ppps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetPixelShader(this, ::core::mem::transmute_copy(&shaderindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetInputSignatureElementDesc(this, ::core::mem::transmute_copy(&shaderindex), ::core::mem::transmute_copy(&element), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut ::core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetOutputSignatureElementDesc(this, ::core::mem::transmute_copy(&shaderindex), ::core::mem::transmute_copy(&element), ::core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            GetShaderDesc: GetShaderDesc::<Impl>,
            GetVertexShader: GetVertexShader::<Impl>,
            GetGeometryShader: GetGeometryShader::<Impl>,
            GetPixelShader: GetPixelShader::<Impl>,
            GetInputSignatureElementDesc: GetInputSignatureElementDesc::<Impl>,
            GetOutputSignatureElementDesc: GetOutputSignatureElementDesc::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
struct ID3D10EffectShaderVariable_ImplVtbl<T: ID3D10EffectShaderVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl<T: ID3D10EffectShaderVariable_Impl> ID3D10EffectShaderVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderVariable_Vtbl = ID3D10EffectShaderVariable_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectShaderVariable {
    pub fn new<'a, T: ID3D10EffectShaderVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectShaderVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectStringVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetString(&self) -> ::windows_core::Result<::windows_core::PCSTR>;
    fn GetStringArray(&self, ppstrings: *mut ::windows_core::PCSTR, offset: u32, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectStringVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectStringVariable_Impl>() -> ID3D10EffectStringVariable_Vtbl {
        unsafe extern "system" fn GetString<Impl: ID3D10EffectStringVariable_Impl>(this: *mut ::core::ffi::c_void, ppstring: *mut ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringArray<Impl: ID3D10EffectStringVariable_Impl>(this: *mut ::core::ffi::c_void, ppstrings: *mut ::windows_core::PCSTR, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetStringArray(this, ::core::mem::transmute_copy(&ppstrings), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetString: GetString::<Impl>, GetStringArray: GetStringArray::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectStringVariable_ImplVtbl<T: ID3D10EffectStringVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectStringVariable_Impl> ID3D10EffectStringVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectStringVariable_Vtbl = ID3D10EffectStringVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectStringVariable {
    pub fn new<'a, T: ID3D10EffectStringVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectStringVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectTechnique_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetPassByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectPass>;
    fn GetPassByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectPass>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectTechnique_Vtbl {
    pub const fn new<Impl: ID3D10EffectTechnique_Impl>() -> ID3D10EffectTechnique_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTechnique_Impl>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsValid(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTechnique_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectTechnique_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetAnnotationByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectTechnique_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetAnnotationByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetPassByIndex<Impl: ID3D10EffectTechnique_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetPassByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetPassByName<Impl: ID3D10EffectTechnique_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectPass> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetPassByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectTechnique_Impl>(this: *mut ::core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::ComputeStateBlockMask(this, ::core::mem::transmute_copy(&pstateblockmask)).into()
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl>,
            GetAnnotationByName: GetAnnotationByName::<Impl>,
            GetPassByIndex: GetPassByIndex::<Impl>,
            GetPassByName: GetPassByName::<Impl>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectTechnique_ImplVtbl<T: ID3D10EffectTechnique_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectTechnique_Impl> ID3D10EffectTechnique_ImplVtbl<T> {
    const VTABLE: ID3D10EffectTechnique_Vtbl = ID3D10EffectTechnique_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectTechnique {
    pub fn new<'a, T: ID3D10EffectTechnique_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectTechnique_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10EffectType_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberTypeByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberTypeBySemantic(&self, semantic: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectType>;
    fn GetMemberName(&self, index: u32) -> ::windows_core::PCSTR;
    fn GetMemberSemantic(&self, index: u32) -> ::windows_core::PCSTR;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectType_Vtbl {
    pub const fn new<Impl: ID3D10EffectType_Impl>() -> ID3D10EffectType_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectType_Impl>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsValid(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectType_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10EffectType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10EffectType_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Impl: ID3D10EffectType_Impl>(this: *mut ::core::ffi::c_void, semantic: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeBySemantic(this, ::core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetMemberName<Impl: ID3D10EffectType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::PCSTR {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberName(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberSemantic<Impl: ID3D10EffectType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::PCSTR {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberSemantic(this, ::core::mem::transmute_copy(&index))
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeBySemantic: GetMemberTypeBySemantic::<Impl>,
            GetMemberName: GetMemberName::<Impl>,
            GetMemberSemantic: GetMemberSemantic::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
struct ID3D10EffectType_ImplVtbl<T: ID3D10EffectType_Impl>(::std::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl<T: ID3D10EffectType_Impl> ID3D10EffectType_ImplVtbl<T> {
    const VTABLE: ID3D10EffectType_Vtbl = ID3D10EffectType_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ID3D10EffectType {
    pub fn new<'a, T: ID3D10EffectType_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVariable_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetType(&self) -> ::core::option::Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectVariable_Impl>() -> ID3D10EffectVariable_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::IsValid(this)
        }
        unsafe extern "system" fn GetType<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetType(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetAnnotationByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetAnnotationByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberByIndex<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberByName<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberBySemantic<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, semantic: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberBySemantic(this, ::core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetElement<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetElement(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetParentConstantBuffer<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetParentConstantBuffer(this)
        }
        unsafe extern "system" fn AsScalar<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectScalarVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsScalar(this)
        }
        unsafe extern "system" fn AsVector<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectVectorVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsVector(this)
        }
        unsafe extern "system" fn AsMatrix<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsMatrix(this)
        }
        unsafe extern "system" fn AsString<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectStringVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsString(this)
        }
        unsafe extern "system" fn AsShaderResource<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsShaderResource(this)
        }
        unsafe extern "system" fn AsRenderTargetView<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsRenderTargetView(this)
        }
        unsafe extern "system" fn AsDepthStencilView<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsDepthStencilView(this)
        }
        unsafe extern "system" fn AsConstantBuffer<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsConstantBuffer(this)
        }
        unsafe extern "system" fn AsShader<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectShaderVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsShader(this)
        }
        unsafe extern "system" fn AsBlend<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectBlendVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsBlend(this)
        }
        unsafe extern "system" fn AsDepthStencil<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsDepthStencil(this)
        }
        unsafe extern "system" fn AsRasterizer<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsRasterizer(this)
        }
        unsafe extern "system" fn AsSampler<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AsSampler(this)
        }
        unsafe extern "system" fn SetRawValue<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetRawValue(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&bytecount)).into()
        }
        unsafe extern "system" fn GetRawValue<Impl: ID3D10EffectVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetRawValue(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&bytecount)).into()
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetType: GetType::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl>,
            GetAnnotationByName: GetAnnotationByName::<Impl>,
            GetMemberByIndex: GetMemberByIndex::<Impl>,
            GetMemberByName: GetMemberByName::<Impl>,
            GetMemberBySemantic: GetMemberBySemantic::<Impl>,
            GetElement: GetElement::<Impl>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Impl>,
            AsScalar: AsScalar::<Impl>,
            AsVector: AsVector::<Impl>,
            AsMatrix: AsMatrix::<Impl>,
            AsString: AsString::<Impl>,
            AsShaderResource: AsShaderResource::<Impl>,
            AsRenderTargetView: AsRenderTargetView::<Impl>,
            AsDepthStencilView: AsDepthStencilView::<Impl>,
            AsConstantBuffer: AsConstantBuffer::<Impl>,
            AsShader: AsShader::<Impl>,
            AsBlend: AsBlend::<Impl>,
            AsDepthStencil: AsDepthStencil::<Impl>,
            AsRasterizer: AsRasterizer::<Impl>,
            AsSampler: AsSampler::<Impl>,
            SetRawValue: SetRawValue::<Impl>,
            GetRawValue: GetRawValue::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectVariable_ImplVtbl<T: ID3D10EffectVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectVariable_Impl> ID3D10EffectVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVariable_Vtbl = ID3D10EffectVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVariable {
    pub fn new<'a, T: ID3D10EffectVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10EffectVectorVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetIntVector(&self, pdata: *mut i32) -> ::windows_core::Result<()>;
    fn SetFloatVector(&self, pdata: *mut f32) -> ::windows_core::Result<()>;
    fn GetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetIntVector(&self, pdata: *mut i32) -> ::windows_core::Result<()>;
    fn GetFloatVector(&self, pdata: *mut f32) -> ::windows_core::Result<()>;
    fn SetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVectorVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectVectorVariable_Impl>() -> ID3D10EffectVectorVariable_Vtbl {
        unsafe extern "system" fn SetBoolVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetBoolVector(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetIntVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetIntVector(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetFloatVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetFloatVector(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetBoolVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBoolVector(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetIntVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetIntVector(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetFloatVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetFloatVector(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetBoolVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetBoolVectorArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetIntVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetIntVectorArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetFloatVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetFloatVectorArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetBoolVectorArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetIntVectorArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut ::core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetFloatVectorArray(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetBoolVector: SetBoolVector::<Impl>,
            SetIntVector: SetIntVector::<Impl>,
            SetFloatVector: SetFloatVector::<Impl>,
            GetBoolVector: GetBoolVector::<Impl>,
            GetIntVector: GetIntVector::<Impl>,
            GetFloatVector: GetFloatVector::<Impl>,
            SetBoolVectorArray: SetBoolVectorArray::<Impl>,
            SetIntVectorArray: SetIntVectorArray::<Impl>,
            SetFloatVectorArray: SetFloatVectorArray::<Impl>,
            GetBoolVectorArray: GetBoolVectorArray::<Impl>,
            GetIntVectorArray: GetIntVectorArray::<Impl>,
            GetFloatVectorArray: GetFloatVectorArray::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ID3D10EffectVectorVariable_ImplVtbl<T: ID3D10EffectVectorVariable_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ID3D10EffectVectorVariable_Impl> ID3D10EffectVectorVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVectorVariable_Vtbl = ID3D10EffectVectorVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ID3D10EffectVectorVariable {
    pub fn new<'a, T: ID3D10EffectVectorVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10EffectVectorVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait ID3D10GeometryShader_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D10GeometryShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10GeometryShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10GeometryShader {
    const VTABLE: Self::Vtable = { ID3D10GeometryShader_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10InfoQueue_Impl: ::windows_core::BaseImpl {
    fn SetMessageCountLimit(this: &Self::This, messagecountlimit: u64) -> ::windows_core::Result<()>;
    fn ClearStoredMessages(this: &Self::This);
    fn GetMessage(this: &Self::This, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(this: &Self::This) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(this: &Self::This) -> u64;
    fn GetNumStoredMessages(this: &Self::This) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(this: &Self::This) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(this: &Self::This) -> u64;
    fn GetMessageCountLimit(this: &Self::This) -> u64;
    fn AddStorageFilterEntries(this: &Self::This, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetStorageFilter(this: &Self::This, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearStorageFilter(this: &Self::This);
    fn PushEmptyStorageFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushCopyOfStorageFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushStorageFilter(this: &Self::This, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopStorageFilter(this: &Self::This);
    fn GetStorageFilterStackSize(this: &Self::This) -> u32;
    fn AddRetrievalFilterEntries(this: &Self::This, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn GetRetrievalFilter(this: &Self::This, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::Result<()>;
    fn ClearRetrievalFilter(this: &Self::This);
    fn PushEmptyRetrievalFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(this: &Self::This) -> ::windows_core::Result<()>;
    fn PushRetrievalFilter(this: &Self::This, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::Result<()>;
    fn PopRetrievalFilter(this: &Self::This);
    fn GetRetrievalFilterStackSize(this: &Self::This) -> u32;
    fn AddMessage(this: &Self::This, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn AddApplicationMessage(this: &Self::This, severity: D3D10_MESSAGE_SEVERITY, pdescription: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetBreakOnCategory(this: &Self::This, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnSeverity(this: &Self::This, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBreakOnID(this: &Self::This, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBreakOnCategory(this: &Self::This, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(this: &Self::This, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(this: &Self::This, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(this: &Self::This, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10InfoQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10InfoQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagecountlimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageCountLimit(this, ::core::mem::transmute_copy(&messagecountlimit)).into())
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStoredMessages(this))
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessage(this, ::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into())
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesAllowedByStorageFilter(this))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDeniedByStorageFilter(this))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessages(this))
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumStoredMessagesAllowedByRetrievalFilter(this))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumMessagesDiscardedByMessageCountLimit(this))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessageCountLimit(this))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStorageFilterEntries(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilter(this, ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearStorageFilter(this))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyStorageFilter(this).into())
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfStorageFilter(this).into())
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushStorageFilter(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopStorageFilter(this))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStorageFilterStackSize(this))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRetrievalFilterEntries(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilter(this, ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into())
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRetrievalFilter(this))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushEmptyRetrievalFilter(this).into())
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushCopyOfRetrievalFilter(this).into())
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushRetrievalFilter(this, ::core::mem::transmute_copy(&pfilter)).into())
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopRetrievalFilter(this))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetrievalFilterStackSize(this))
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMessage(this, ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddApplicationMessage(this, ::core::mem::transmute_copy(&severity), ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnCategory(this, ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnSeverity(this, ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakOnID(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnCategory(this, ::core::mem::transmute_copy(&category)))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnSeverity(this, ::core::mem::transmute_copy(&severity)))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakOnID(this, ::core::mem::transmute_copy(&id)))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMuteDebugOutput(this, ::core::mem::transmute_copy(&bmute)))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InfoQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMuteDebugOutput(this))
        }
        ID3D10InfoQueue_Vtbl {
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
pub trait ID3D10InputLayout_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D10InputLayout {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10InputLayout_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10InputLayout {
    const VTABLE: Self::Vtable = { ID3D10InputLayout_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10Multithread_Impl: ::windows_core::BaseImpl {
    fn Enter(this: &Self::This);
    fn Leave(this: &Self::This);
    fn SetMultithreadProtected(this: &Self::This, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10Multithread {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Multithread {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enter(this))
        }
        unsafe extern "system" fn Leave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Leave(this))
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultithreadProtected(this, ::core::mem::transmute_copy(&bmtprotect)))
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMultithreadProtected(this))
        }
        ID3D10Multithread_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, Impl, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D10PixelShader_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D10PixelShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10PixelShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10PixelShader {
    const VTABLE: Self::Vtable = { ID3D10PixelShader_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D10Predicate_Impl: ::windows_core::BaseImpl + ID3D10Query_Impl {}
impl ::windows_core::Iids for ID3D10Predicate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Query);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Predicate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Predicate {
    const VTABLE: Self::Vtable = { ID3D10Predicate_Vtbl { base__: <ID3D10Query as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D10Query_Impl: ::windows_core::BaseImpl + ID3D10Asynchronous_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_QUERY_DESC);
}
impl ::windows_core::Iids for ID3D10Query {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Asynchronous);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Query_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Query {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Query_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10Query_Vtbl { base__: <ID3D10Asynchronous as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10RasterizerState_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_RASTERIZER_DESC);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10RasterizerState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10RasterizerState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10RasterizerState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10RasterizerState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10RasterizerState_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10RenderTargetView_Impl: ::windows_core::BaseImpl + ID3D10View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D10RenderTargetView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10View);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10RenderTargetView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10RenderTargetView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10RenderTargetView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10RenderTargetView_Vtbl { base__: <ID3D10View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D10Resource_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {
    fn GetType(this: &Self::This, rtype: *mut D3D10_RESOURCE_DIMENSION);
    fn SetEvictionPriority(this: &Self::This, evictionpriority: u32);
    fn GetEvictionPriority(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for ID3D10Resource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Resource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this, ::core::mem::transmute_copy(&rtype)))
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, evictionpriority: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEvictionPriority(this, ::core::mem::transmute_copy(&evictionpriority)))
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEvictionPriority(this))
        }
        ID3D10Resource_Vtbl {
            base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D10SamplerState_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_SAMPLER_DESC);
}
impl ::windows_core::Iids for ID3D10SamplerState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10SamplerState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10SamplerState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10SamplerState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10SamplerState_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflection_Impl: ::windows_core::BaseImpl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetConstantBufferByIndex(this: &Self::This, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(this: &Self::This, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetInputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetOutputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::Iids for ID3D10ShaderReflection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10ShaderReflection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceBindingDesc(this, ::core::mem::transmute_copy(&resourceindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        ID3D10ShaderReflection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub trait ID3D10ShaderReflection1_Impl: ::windows_core::BaseImpl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_SHADER_DESC) -> ::windows_core::Result<()>;
    fn GetConstantBufferByIndex(this: &Self::This, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(this: &Self::This, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetInputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetOutputParameterDesc(this: &Self::This, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByName(this: &Self::This, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(this: &Self::This, name: &::windows_core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows_core::Result<()>;
    fn GetMovInstructionCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMovcInstructionCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetConversionInstructionCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetBitwiseInstructionCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetGSInputPrimitive(this: &Self::This) -> ::windows_core::Result<super::Direct3D::D3D_PRIMITIVE>;
    fn IsLevel9Shader(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsSampleFrequencyShader(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::windows_core::Iids for ID3D10ShaderReflection1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10ShaderReflection1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionConstantBuffer> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstantBufferByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceBindingDesc(this, ::core::mem::transmute_copy(&resourceindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputParameterDesc(this, ::core::mem::transmute_copy(&parameterindex), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetVariableByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVariableByName(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceBindingDescByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&pdesc)).into())
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMovInstructionCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMovcInstructionCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConversionInstructionCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBitwiseInstructionCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGSInputPrimitive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLevel9Shader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLevel9Shader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblevel9shader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderReflection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSampleFrequencyShader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsamplefrequency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D10ShaderReflection1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsLevel9Shader: IsLevel9Shader::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>() -> ID3D10ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetVariableByName(this, ::core::mem::transmute(&name))
        }
        Self { GetDesc: GetDesc::<Impl>, GetVariableByIndex: GetVariableByIndex::<Impl>, GetVariableByName: GetVariableByName::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D10ShaderReflectionConstantBuffer_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10ShaderReflectionConstantBuffer_Impl> ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionConstantBuffer_Vtbl = ID3D10ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D10ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> ::windows_core::PCSTR;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionType_Vtbl {
    pub const fn new<Impl: ID3D10ShaderReflectionType_Impl>() -> ID3D10ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByIndex(this, ::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCSTR) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeByName(this, ::core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::PCSTR {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetMemberTypeName(this, ::core::mem::transmute_copy(&index))
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeName: GetMemberTypeName::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10ShaderReflectionType_ImplVtbl<T: ID3D10ShaderReflectionType_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10ShaderReflectionType_Impl> ID3D10ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionType_Vtbl = ID3D10ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionType {
    pub fn new<'a, T: ID3D10ShaderReflectionType_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait ID3D10ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows_core::Result<()>;
    fn GetType(&self) -> ::core::option::Option<ID3D10ShaderReflectionType>;
}
impl ID3D10ShaderReflectionVariable_Vtbl {
    pub const fn new<Impl: ID3D10ShaderReflectionVariable_Impl>() -> ID3D10ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetType<Impl: ID3D10ShaderReflectionVariable_Impl>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetType(this)
        }
        Self { GetDesc: GetDesc::<Impl>, GetType: GetType::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D10ShaderReflectionVariable_ImplVtbl<T: ID3D10ShaderReflectionVariable_Impl>(::std::marker::PhantomData<T>);
impl<T: ID3D10ShaderReflectionVariable_Impl> ID3D10ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionVariable_Vtbl = ID3D10ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D10ShaderReflectionVariable {
    pub fn new<'a, T: ID3D10ShaderReflectionVariable_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView_Impl: ::windows_core::BaseImpl + ID3D10View_Impl {
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D10ShaderResourceView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10View);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderResourceView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10ShaderResourceView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderResourceView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10ShaderResourceView_Vtbl { base__: <ID3D10View as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView1_Impl: ::windows_core::BaseImpl + ID3D10ShaderResourceView_Impl {
    fn GetDesc1(this: &Self::This, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID3D10ShaderResourceView1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10ShaderResourceView);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderResourceView1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10ShaderResourceView1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesc1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10ShaderResourceView1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc1(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10ShaderResourceView1_Vtbl { base__: <ID3D10ShaderResourceView as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    };
}
pub trait ID3D10StateBlock_Impl: ::windows_core::BaseImpl {
    fn Capture(this: &Self::This) -> ::windows_core::Result<()>;
    fn Apply(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReleaseAllDeviceObjects(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDevice(this: &Self::This) -> ::windows_core::Result<ID3D10Device>;
}
impl ::windows_core::Iids for ID3D10StateBlock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10StateBlock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Capture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Capture(this).into())
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Apply(this).into())
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseAllDeviceObjects(this).into())
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10StateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID3D10StateBlock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Capture: Capture::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            ReleaseAllDeviceObjects: ReleaseAllDeviceObjects::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID3D10SwitchToRef_Impl: ::windows_core::BaseImpl {
    fn SetUseRef(this: &Self::This, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID3D10SwitchToRef {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10SwitchToRef_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10SwitchToRef {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetUseRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10SwitchToRef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseRef(this, ::core::mem::transmute_copy(&useref)))
        }
        unsafe extern "system" fn GetUseRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10SwitchToRef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUseRef(this))
        }
        ID3D10SwitchToRef_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetUseRef: SetUseRef::<Identity, Impl, OFFSET>,
            GetUseRef: GetUseRef::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture1D_Impl: ::windows_core::BaseImpl + ID3D10Resource_Impl {
    fn Map(this: &Self::This, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Unmap(this: &Self::This, subresource: u32);
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D10Texture1D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Resource);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Texture1D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Map(this, ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags), ::core::mem::transmute_copy(&ppdata)).into())
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this, ::core::mem::transmute_copy(&subresource)))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture1D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10Texture1D_Vtbl {
            base__: <ID3D10Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture2D_Impl: ::windows_core::BaseImpl + ID3D10Resource_Impl {
    fn Map(this: &Self::This, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows_core::Result<D3D10_MAPPED_TEXTURE2D>;
    fn Unmap(this: &Self::This, subresource: u32);
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D10Texture2D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Resource);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Texture2D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Map(this, ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmappedtex2d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this, ::core::mem::transmute_copy(&subresource)))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture2D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10Texture2D_Vtbl {
            base__: <ID3D10Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture3D_Impl: ::windows_core::BaseImpl + ID3D10Resource_Impl {
    fn Map(this: &Self::This, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> ::windows_core::Result<D3D10_MAPPED_TEXTURE3D>;
    fn Unmap(this: &Self::This, subresource: u32);
    fn GetDesc(this: &Self::This, pdesc: *mut D3D10_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ID3D10Texture3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10Resource);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10Texture3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Map(this, ::core::mem::transmute_copy(&subresource), ::core::mem::transmute_copy(&maptype), ::core::mem::transmute_copy(&mapflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmappedtex3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subresource: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this, ::core::mem::transmute_copy(&subresource)))
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10Texture3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesc(this, ::core::mem::transmute_copy(&pdesc)))
        }
        ID3D10Texture3D_Vtbl {
            base__: <ID3D10Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ID3D10VertexShader_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {}
impl ::windows_core::Iids for ID3D10VertexShader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10VertexShader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10VertexShader {
    const VTABLE: Self::Vtable = { ID3D10VertexShader_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ID3D10View_Impl: ::windows_core::BaseImpl + ID3D10DeviceChild_Impl {
    fn GetResource(this: &Self::This, ppresource: *mut ::core::option::Option<ID3D10Resource>);
}
impl ::windows_core::Iids for ID3D10View {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID3D10DeviceChild);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10View_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3D10View {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3D10View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresource: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResource(this, ::core::mem::transmute_copy(&ppresource)))
        }
        ID3D10View_Vtbl { base__: <ID3D10DeviceChild as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetResource: GetResource::<Identity, Impl, OFFSET> }
    };
}
