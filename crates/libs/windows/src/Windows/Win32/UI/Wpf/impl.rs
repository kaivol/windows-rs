#[doc = "Required features: `\"Win32_Graphics_Imaging\"`"]
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffect_Impl: ::windows_core::BaseImpl {
    fn GetOutput(this: &Self::This, uiindex: u32, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetParentEffect(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffectGroup>;
    fn SetInputSource(this: &Self::This, uiindex: u32, pbitmapsource: ::core::option::Option<&super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl ::windows_core::Iids for IMILBitmapEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutput(this, ::core::mem::transmute_copy(&uiindex), ::windows_core::from_raw_borrowed(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbitmapsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParentEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparenteffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparenteffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInputSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputSource(this, ::core::mem::transmute_copy(&uiindex), ::windows_core::from_raw_borrowed(&pbitmapsource)).into())
        }
        IMILBitmapEffect_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            GetParentEffect: GetParentEffect::<Identity, Impl, OFFSET>,
            SetInputSource: SetInputSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectConnections_Impl: ::windows_core::BaseImpl {
    fn GetInputConnector(this: &Self::This, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector>;
    fn GetOutputConnector(this: &Self::This, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl ::windows_core::Iids for IMILBitmapEffectConnections {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectConnections {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputConnector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputConnector(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputConnector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputConnector(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectConnections_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputConnector: GetInputConnector::<Identity, Impl, OFFSET>,
            GetOutputConnector: GetOutputConnector::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectConnectionsInfo_Impl: ::windows_core::BaseImpl {
    fn GetNumberInputs(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetNumberOutputs(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetInputConnectorInfo(this: &Self::This, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectConnectorInfo>;
    fn GetOutputConnectorInfo(this: &Self::This, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectConnectorInfo>;
}
impl ::windows_core::Iids for IMILBitmapEffectConnectionsInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectConnectionsInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNumberInputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberInputs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinuminputs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumberOutputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberOutputs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinumoutputs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputConnectorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputConnectorInfo(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnectorinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputConnectorInfo(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnectorinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectConnectionsInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNumberInputs: GetNumberInputs::<Identity, Impl, OFFSET>,
            GetNumberOutputs: GetNumberOutputs::<Identity, Impl, OFFSET>,
            GetInputConnectorInfo: GetInputConnectorInfo::<Identity, Impl, OFFSET>,
            GetOutputConnectorInfo: GetOutputConnectorInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectConnector_Impl: ::windows_core::BaseImpl + IMILBitmapEffectConnectorInfo_Impl {
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetBitmapEffect(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffect>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMILBitmapEffectConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMILBitmapEffectConnectorInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBitmapEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBitmapEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectConnector_Vtbl {
            base__: <IMILBitmapEffectConnectorInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetBitmapEffect: GetBitmapEffect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectConnectorInfo_Impl: ::windows_core::BaseImpl {
    fn GetIndex(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetOptimalFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetNumberFormats(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFormat(this: &Self::This, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IMILBitmapEffectConnectorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectConnectorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOptimalFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptimalFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumberFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberFormats(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulnumberformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormat(this, ::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectConnectorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetOptimalFormat: GetOptimalFormat::<Identity, Impl, OFFSET>,
            GetNumberFormats: GetNumberFormats::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectEvents_Impl: ::windows_core::BaseImpl {
    fn PropertyChange(this: &Self::This, peffect: ::core::option::Option<&IMILBitmapEffect>, bstrpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DirtyRegion(this: &Self::This, peffect: ::core::option::Option<&IMILBitmapEffect>, prect: *const MilRectD) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMILBitmapEffectEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropertyChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PropertyChange(this, ::windows_core::from_raw_borrowed(&peffect), ::core::mem::transmute(&bstrpropertyname)).into())
        }
        unsafe extern "system" fn DirtyRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DirtyRegion(this, ::windows_core::from_raw_borrowed(&peffect), ::core::mem::transmute_copy(&prect)).into())
        }
        IMILBitmapEffectEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropertyChange: PropertyChange::<Identity, Impl, OFFSET>,
            DirtyRegion: DirtyRegion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectFactory_Impl: ::windows_core::BaseImpl {
    fn CreateEffect(this: &Self::This, pguideffect: *const ::windows_core::GUID) -> ::windows_core::Result<IMILBitmapEffect>;
    fn CreateContext(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffectRenderContext>;
    fn CreateEffectOuter(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffect>;
}
impl ::windows_core::Iids for IMILBitmapEffectFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows_core::GUID, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEffect(this, ::core::mem::transmute_copy(&pguideffect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEffectOuter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEffectOuter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            CreateEffectOuter: CreateEffectOuter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectGroup_Impl: ::windows_core::BaseImpl {
    fn GetInteriorInputConnector(this: &Self::This, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectOutputConnector>;
    fn GetInteriorOutputConnector(this: &Self::This, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector>;
    fn Add(this: &Self::This, peffect: ::core::option::Option<&IMILBitmapEffect>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMILBitmapEffectGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInteriorInputConnector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInteriorInputConnector(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInteriorOutputConnector(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&peffect)).into())
        }
        IMILBitmapEffectGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInteriorInputConnector: GetInteriorInputConnector::<Identity, Impl, OFFSET>,
            GetInteriorOutputConnector: GetInteriorOutputConnector::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectGroupImpl_Impl: ::windows_core::BaseImpl {
    fn Preprocess(this: &Self::This, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>) -> ::windows_core::Result<()>;
    fn GetNumberChildren(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetChildren(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffects>;
}
impl ::windows_core::Iids for IMILBitmapEffectGroupImpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectGroupImpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Preprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Preprocess(this, ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        unsafe extern "system" fn GetNumberChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinumberchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectGroupImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectGroupImpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Preprocess: Preprocess::<Identity, Impl, OFFSET>,
            GetNumberChildren: GetNumberChildren::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectImpl_Impl: ::windows_core::BaseImpl {
    fn IsInPlaceModificationAllowed(this: &Self::This, poutputconnector: ::core::option::Option<&IMILBitmapEffectOutputConnector>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetParentEffect(this: &Self::This, pparenteffect: ::core::option::Option<&IMILBitmapEffectGroup>) -> ::windows_core::Result<()>;
    fn GetInputSource(this: &Self::This, uiindex: u32) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetInputSourceBounds(this: &Self::This, uiindex: u32, prect: *mut MilRectD) -> ::windows_core::Result<()>;
    fn GetInputBitmapSource(this: &Self::This, uiindex: u32, prendercontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn GetOutputBitmapSource(this: &Self::This, uiindex: u32, prendercontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, pinner: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for IMILBitmapEffectImpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectImpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInPlaceModificationAllowed(this, ::windows_core::from_raw_borrowed(&poutputconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmodifyinplace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParentEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparenteffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParentEffect(this, ::windows_core::from_raw_borrowed(&pparenteffect)).into())
        }
        unsafe extern "system" fn GetInputSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputSource(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbitmapsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputSourceBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputSourceBounds(this, ::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&prect)).into())
        }
        unsafe extern "system" fn GetInputBitmapSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputBitmapSource(this, ::core::mem::transmute_copy(&uiindex), ::windows_core::from_raw_borrowed(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into())
        }
        unsafe extern "system" fn GetOutputBitmapSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputBitmapSource(this, ::core::mem::transmute_copy(&uiindex), ::windows_core::from_raw_borrowed(&prendercontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pinner)).into())
        }
        IMILBitmapEffectImpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsInPlaceModificationAllowed: IsInPlaceModificationAllowed::<Identity, Impl, OFFSET>,
            SetParentEffect: SetParentEffect::<Identity, Impl, OFFSET>,
            GetInputSource: GetInputSource::<Identity, Impl, OFFSET>,
            GetInputSourceBounds: GetInputSourceBounds::<Identity, Impl, OFFSET>,
            GetInputBitmapSource: GetInputBitmapSource::<Identity, Impl, OFFSET>,
            GetOutputBitmapSource: GetOutputBitmapSource::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectInputConnector_Impl: ::windows_core::BaseImpl + IMILBitmapEffectConnector_Impl {
    fn ConnectTo(this: &Self::This, pconnector: ::core::option::Option<&IMILBitmapEffectOutputConnector>) -> ::windows_core::Result<()>;
    fn GetConnection(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffectOutputConnector>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMILBitmapEffectInputConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMILBitmapEffectConnector);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectInputConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnector: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectTo(this, ::windows_core::from_raw_borrowed(&pconnector)).into())
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectInputConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectInputConnector_Vtbl {
            base__: <IMILBitmapEffectConnector as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectTo: ConnectTo::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectInteriorInputConnector_Impl: ::windows_core::BaseImpl {
    fn GetInputConnector(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffectInputConnector>;
}
impl ::windows_core::Iids for IMILBitmapEffectInteriorInputConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectInteriorInputConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputConnector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputConnector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinputconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectInteriorInputConnector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputConnector: GetInputConnector::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectInteriorOutputConnector_Impl: ::windows_core::BaseImpl {
    fn GetOutputConnector(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl ::windows_core::Iids for IMILBitmapEffectInteriorOutputConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectInteriorOutputConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOutputConnector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputConnector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutputconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectInteriorOutputConnector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOutputConnector: GetOutputConnector::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectOutputConnector_Impl: ::windows_core::BaseImpl + IMILBitmapEffectConnector_Impl {
    fn GetNumberConnections(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetConnection(this: &Self::This, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMILBitmapEffectOutputConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMILBitmapEffectConnector);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectOutputConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNumberConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puinumberconnections, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectOutputConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnection(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectOutputConnector_Vtbl {
            base__: <IMILBitmapEffectConnector as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNumberConnections: GetNumberConnections::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffectOutputConnectorImpl_Impl: ::windows_core::BaseImpl {
    fn AddBackLink(this: &Self::This, pconnection: ::core::option::Option<&IMILBitmapEffectInputConnector>) -> ::windows_core::Result<()>;
    fn RemoveBackLink(this: &Self::This, pconnection: ::core::option::Option<&IMILBitmapEffectInputConnector>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMILBitmapEffectOutputConnectorImpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectOutputConnectorImpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddBackLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddBackLink(this, ::windows_core::from_raw_borrowed(&pconnection)).into())
        }
        unsafe extern "system" fn RemoveBackLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveBackLink(this, ::windows_core::from_raw_borrowed(&pconnection)).into())
        }
        IMILBitmapEffectOutputConnectorImpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddBackLink: AddBackLink::<Identity, Impl, OFFSET>,
            RemoveBackLink: RemoveBackLink::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dwm\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectPrimitive_Impl: ::windows_core::BaseImpl {
    fn GetOutput(this: &Self::This, uiindex: u32, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn TransformPoint(this: &Self::This, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn TransformRect(this: &Self::This, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: ::core::option::Option<&IMILBitmapEffectRenderContext>) -> ::windows_core::Result<()>;
    fn HasAffineTransform(this: &Self::This, uiindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn HasInverseTransform(this: &Self::This, uiindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAffineMatrix(this: &Self::This, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for IMILBitmapEffectPrimitive {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectPrimitive {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutput(this, ::core::mem::transmute_copy(&uiindex), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pfmodifyinplace), ::core::mem::transmute_copy(&ppbitmapsource)).into())
        }
        unsafe extern "system" fn TransformPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransformPoint(this, ::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pfpointtransformed)).into())
        }
        unsafe extern "system" fn TransformRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransformRect(this, ::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&p), ::core::mem::transmute_copy(&fforwardtransform), ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        unsafe extern "system" fn HasAffineTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasAffineTransform(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaffine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasInverseTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasInverseTransform(this, ::core::mem::transmute_copy(&uiindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasinverse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAffineMatrix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAffineMatrix(this, ::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&pmatrix)).into())
        }
        IMILBitmapEffectPrimitive_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            TransformPoint: TransformPoint::<Identity, Impl, OFFSET>,
            TransformRect: TransformRect::<Identity, Impl, OFFSET>,
            HasAffineTransform: HasAffineTransform::<Identity, Impl, OFFSET>,
            HasInverseTransform: HasInverseTransform::<Identity, Impl, OFFSET>,
            GetAffineMatrix: GetAffineMatrix::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectPrimitiveImpl_Impl: ::windows_core::BaseImpl {
    fn IsDirty(this: &Self::This, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT;
    fn IsVolatile(this: &Self::This, uioutputindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMILBitmapEffectPrimitiveImpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectPrimitiveImpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this, ::core::mem::transmute_copy(&uioutputindex), ::core::mem::transmute_copy(&pfdirty)))
        }
        unsafe extern "system" fn IsVolatile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsVolatile(this, ::core::mem::transmute_copy(&uioutputindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfvolatile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffectPrimitiveImpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            IsVolatile: IsVolatile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectRenderContext_Impl: ::windows_core::BaseImpl {
    fn SetOutputPixelFormat(this: &Self::This, format: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetOutputPixelFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetUseSoftwareRenderer(this: &Self::This, fsoftware: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetInitialTransform(this: &Self::This, pmatrix: *const MILMatrixF) -> ::windows_core::Result<()>;
    fn GetFinalTransform(this: &Self::This, pmatrix: *mut MILMatrixF) -> ::windows_core::Result<()>;
    fn SetOutputDPI(this: &Self::This, dbldpix: f64, dbldpiy: f64) -> ::windows_core::Result<()>;
    fn GetOutputDPI(this: &Self::This, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows_core::Result<()>;
    fn SetRegionOfInterest(this: &Self::This, prect: *const MilRectD) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMILBitmapEffectRenderContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectRenderContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOutputPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputPixelFormat(this, ::core::mem::transmute_copy(&format)).into())
        }
        unsafe extern "system" fn GetOutputPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputPixelFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsoftware: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseSoftwareRenderer(this, ::core::mem::transmute_copy(&fsoftware)).into())
        }
        unsafe extern "system" fn SetInitialTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialTransform(this, ::core::mem::transmute_copy(&pmatrix)).into())
        }
        unsafe extern "system" fn GetFinalTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFinalTransform(this, ::core::mem::transmute_copy(&pmatrix)).into())
        }
        unsafe extern "system" fn SetOutputDPI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputDPI(this, ::core::mem::transmute_copy(&dbldpix), ::core::mem::transmute_copy(&dbldpiy)).into())
        }
        unsafe extern "system" fn GetOutputDPI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputDPI(this, ::core::mem::transmute_copy(&pdbldpix), ::core::mem::transmute_copy(&pdbldpiy)).into())
        }
        unsafe extern "system" fn SetRegionOfInterest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRegionOfInterest(this, ::core::mem::transmute_copy(&prect)).into())
        }
        IMILBitmapEffectRenderContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOutputPixelFormat: SetOutputPixelFormat::<Identity, Impl, OFFSET>,
            GetOutputPixelFormat: GetOutputPixelFormat::<Identity, Impl, OFFSET>,
            SetUseSoftwareRenderer: SetUseSoftwareRenderer::<Identity, Impl, OFFSET>,
            SetInitialTransform: SetInitialTransform::<Identity, Impl, OFFSET>,
            GetFinalTransform: GetFinalTransform::<Identity, Impl, OFFSET>,
            SetOutputDPI: SetOutputDPI::<Identity, Impl, OFFSET>,
            GetOutputDPI: GetOutputDPI::<Identity, Impl, OFFSET>,
            SetRegionOfInterest: SetRegionOfInterest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMILBitmapEffectRenderContextImpl_Impl: ::windows_core::BaseImpl {
    fn GetUseSoftwareRenderer(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransform(this: &Self::This, pmatrix: *mut MILMatrixF) -> ::windows_core::Result<()>;
    fn UpdateTransform(this: &Self::This, pmatrix: *const MILMatrixF) -> ::windows_core::Result<()>;
    fn GetOutputBounds(this: &Self::This, prect: *mut MilRectD) -> ::windows_core::Result<()>;
    fn UpdateOutputBounds(this: &Self::This, prect: *const MilRectD) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMILBitmapEffectRenderContextImpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffectRenderContextImpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUseSoftwareRenderer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsoftware: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUseSoftwareRenderer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsoftware, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransform(this, ::core::mem::transmute_copy(&pmatrix)).into())
        }
        unsafe extern "system" fn UpdateTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateTransform(this, ::core::mem::transmute_copy(&pmatrix)).into())
        }
        unsafe extern "system" fn GetOutputBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputBounds(this, ::core::mem::transmute_copy(&prect)).into())
        }
        unsafe extern "system" fn UpdateOutputBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateOutputBounds(this, ::core::mem::transmute_copy(&prect)).into())
        }
        IMILBitmapEffectRenderContextImpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUseSoftwareRenderer: GetUseSoftwareRenderer::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            UpdateTransform: UpdateTransform::<Identity, Impl, OFFSET>,
            GetOutputBounds: GetOutputBounds::<Identity, Impl, OFFSET>,
            UpdateOutputBounds: UpdateOutputBounds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMILBitmapEffects_Impl: ::windows_core::BaseImpl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<IMILBitmapEffectGroup>;
    fn Item(this: &Self::This, uindex: u32) -> ::windows_core::Result<IMILBitmapEffect>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMILBitmapEffects {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMILBitmapEffects {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiureturn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMILBitmapEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puicount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMILBitmapEffects_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
