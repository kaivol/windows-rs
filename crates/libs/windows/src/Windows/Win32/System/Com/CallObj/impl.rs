#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICallFrame_Impl: ::windows_core::BaseImpl {
    fn GetInfo(this: &Self::This, pinfo: *mut CALLFRAMEINFO) -> ::windows_core::Result<()>;
    fn GetIIDAndMethod(this: &Self::This, piid: *mut ::windows_core::GUID, pimethod: *mut u32) -> ::windows_core::Result<()>;
    fn GetNames(this: &Self::This, pwszinterface: *mut ::windows_core::PWSTR, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetStackLocation(this: &Self::This) -> *mut ::core::ffi::c_void;
    fn SetStackLocation(this: &Self::This, pvstack: *const ::core::ffi::c_void);
    fn SetReturnValue(this: &Self::This, hr: ::windows_core::HRESULT);
    fn GetReturnValue(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetParamInfo(this: &Self::This, iparam: u32) -> ::windows_core::Result<CALLFRAMEPARAMINFO>;
    fn SetParam(this: &Self::This, iparam: u32, pvar: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetParam(this: &Self::This, iparam: u32) -> ::windows_core::Result<super::super::Variant::VARIANT>;
    fn Copy(this: &Self::This, copycontrol: CALLFRAME_COPY, pwalker: ::core::option::Option<&ICallFrameWalker>) -> ::windows_core::Result<ICallFrame>;
    fn Free(this: &Self::This, pframeargsdest: ::core::option::Option<&ICallFrame>, pwalkerdestfree: ::core::option::Option<&ICallFrameWalker>, pwalkercopy: ::core::option::Option<&ICallFrameWalker>, freeflags: u32, pwalkerfree: ::core::option::Option<&ICallFrameWalker>, nullflags: u32) -> ::windows_core::Result<()>;
    fn FreeParam(this: &Self::This, iparam: u32, freeflags: u32, pwalkerfree: ::core::option::Option<&ICallFrameWalker>, nullflags: u32) -> ::windows_core::Result<()>;
    fn WalkFrame(this: &Self::This, walkwhat: u32, pwalker: ::core::option::Option<&ICallFrameWalker>) -> ::windows_core::Result<()>;
    fn GetMarshalSizeMax(this: &Self::This, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> ::windows_core::Result<u32>;
    fn Marshal(this: &Self::This, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows_core::Result<()>;
    fn Unmarshal(this: &Self::This, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::Result<u32>;
    fn ReleaseMarshalData(this: &Self::This, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::Result<()>;
    fn Invoke(this: &Self::This, pvreceiver: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICallFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICallFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfo(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn GetIIDAndMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID, pimethod: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIIDAndMethod(this, ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pimethod)).into())
        }
        unsafe extern "system" fn GetNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinterface: *mut ::windows_core::PWSTR, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNames(this, ::core::mem::transmute_copy(&pwszinterface), ::core::mem::transmute_copy(&pwszmethod)).into())
        }
        unsafe extern "system" fn GetStackLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStackLocation(this))
        }
        unsafe extern "system" fn SetStackLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvstack: *const ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStackLocation(this, ::core::mem::transmute_copy(&pvstack)))
        }
        unsafe extern "system" fn SetReturnValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReturnValue(this, ::core::mem::transmute_copy(&hr)))
        }
        unsafe extern "system" fn GetReturnValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReturnValue(this).into())
        }
        unsafe extern "system" fn GetParamInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParamInfo(this, ::core::mem::transmute_copy(&iparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParam(this, ::core::mem::transmute_copy(&iparam), ::core::mem::transmute_copy(&pvar)).into())
        }
        unsafe extern "system" fn GetParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParam(this, ::core::mem::transmute_copy(&iparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Copy(this, ::core::mem::transmute_copy(&copycontrol), ::windows_core::from_raw_borrowed(&pwalker)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Free<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pframeargsdest: *mut ::core::ffi::c_void, pwalkerdestfree: *mut ::core::ffi::c_void, pwalkercopy: *mut ::core::ffi::c_void, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Free(this, ::windows_core::from_raw_borrowed(&pframeargsdest), ::windows_core::from_raw_borrowed(&pwalkerdestfree), ::windows_core::from_raw_borrowed(&pwalkercopy), ::core::mem::transmute_copy(&freeflags), ::windows_core::from_raw_borrowed(&pwalkerfree), ::core::mem::transmute_copy(&nullflags)).into())
        }
        unsafe extern "system" fn FreeParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeParam(this, ::core::mem::transmute_copy(&iparam), ::core::mem::transmute_copy(&freeflags), ::windows_core::from_raw_borrowed(&pwalkerfree), ::core::mem::transmute_copy(&nullflags)).into())
        }
        unsafe extern "system" fn WalkFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, walkwhat: u32, pwalker: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WalkFrame(this, ::core::mem::transmute_copy(&walkwhat), ::windows_core::from_raw_borrowed(&pwalker)).into())
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMarshalSizeMax(this, ::core::mem::transmute_copy(&pmshlcontext), ::core::mem::transmute_copy(&mshlflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbufferneeded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Marshal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Marshal(this, ::core::mem::transmute_copy(&pmshlcontext), ::core::mem::transmute_copy(&mshlflags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcbbufferused), ::core::mem::transmute_copy(&pdatarep), ::core::mem::transmute_copy(&prpcflags)).into())
        }
        unsafe extern "system" fn Unmarshal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Unmarshal(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbunmarshalled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMarshalData(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ibfirstrelease), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvreceiver: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::core::mem::transmute_copy(&pvreceiver)).into())
        }
        ICallFrame_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetIIDAndMethod: GetIIDAndMethod::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            GetStackLocation: GetStackLocation::<Identity, Impl, OFFSET>,
            SetStackLocation: SetStackLocation::<Identity, Impl, OFFSET>,
            SetReturnValue: SetReturnValue::<Identity, Impl, OFFSET>,
            GetReturnValue: GetReturnValue::<Identity, Impl, OFFSET>,
            GetParamInfo: GetParamInfo::<Identity, Impl, OFFSET>,
            SetParam: SetParam::<Identity, Impl, OFFSET>,
            GetParam: GetParam::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
            FreeParam: FreeParam::<Identity, Impl, OFFSET>,
            WalkFrame: WalkFrame::<Identity, Impl, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, Impl, OFFSET>,
            Marshal: Marshal::<Identity, Impl, OFFSET>,
            Unmarshal: Unmarshal::<Identity, Impl, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICallFrameEvents_Impl: ::windows_core::BaseImpl {
    fn OnCall(this: &Self::This, pframe: ::core::option::Option<&ICallFrame>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICallFrameEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrameEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICallFrameEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrameEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCall(this, ::windows_core::from_raw_borrowed(&pframe)).into())
        }
        ICallFrameEvents_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnCall: OnCall::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallFrameWalker_Impl: ::windows_core::BaseImpl {
    fn OnWalkInterface(this: &Self::This, iid: *const ::windows_core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICallFrameWalker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrameWalker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICallFrameWalker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnWalkInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallFrameWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWalkInterface(this, ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvinterface), ::core::mem::transmute_copy(&fin), ::core::mem::transmute_copy(&fout)).into())
        }
        ICallFrameWalker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnWalkInterface: OnWalkInterface::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallIndirect_Impl: ::windows_core::BaseImpl {
    fn CallIndirect(this: &Self::This, phrreturn: *mut ::windows_core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows_core::Result<()>;
    fn GetMethodInfo(this: &Self::This, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetStackSize(this: &Self::This, imethod: u32) -> ::windows_core::Result<u32>;
    fn GetIID(this: &Self::This, piid: *mut ::windows_core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICallIndirect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICallIndirect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CallIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrreturn: *mut ::windows_core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CallIndirect(this, ::core::mem::transmute_copy(&phrreturn), ::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pvargs), ::core::mem::transmute_copy(&cbargs)).into())
        }
        unsafe extern "system" fn GetMethodInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMethodInfo(this, ::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pwszmethod)).into())
        }
        unsafe extern "system" fn GetStackSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStackSize(this, ::core::mem::transmute_copy(&imethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cbargs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallIndirect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIID(this, ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pfderivesfromidispatch), ::core::mem::transmute_copy(&pcmethod), ::core::mem::transmute_copy(&pwszinterface)).into())
        }
        ICallIndirect_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CallIndirect: CallIndirect::<Identity, Impl, OFFSET>,
            GetMethodInfo: GetMethodInfo::<Identity, Impl, OFFSET>,
            GetStackSize: GetStackSize::<Identity, Impl, OFFSET>,
            GetIID: GetIID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallInterceptor_Impl: ::windows_core::BaseImpl + ICallIndirect_Impl {
    fn RegisterSink(this: &Self::This, psink: ::core::option::Option<&ICallFrameEvents>) -> ::windows_core::Result<()>;
    fn GetRegisteredSink(this: &Self::This) -> ::windows_core::Result<ICallFrameEvents>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICallInterceptor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICallIndirect);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallInterceptor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICallInterceptor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallInterceptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterSink(this, ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn GetRegisteredSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallInterceptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegisteredSink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICallInterceptor_Vtbl {
            base__: <ICallIndirect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterSink: RegisterSink::<Identity, Impl, OFFSET>,
            GetRegisteredSink: GetRegisteredSink::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICallUnmarshal_Impl: ::windows_core::BaseImpl {
    fn Unmarshal(this: &Self::This, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::core::option::Option<ICallFrame>) -> ::windows_core::Result<()>;
    fn ReleaseMarshalData(this: &Self::This, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICallUnmarshal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallUnmarshal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICallUnmarshal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Unmarshal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallUnmarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmarshal(this, ::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&fforcebuffercopy), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcbunmarshalled), ::core::mem::transmute_copy(&ppframe)).into())
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICallUnmarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMarshalData(this, ::core::mem::transmute_copy(&imethod), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ibfirstrelease), ::core::mem::transmute_copy(&datarep), ::core::mem::transmute_copy(&pcontext)).into())
        }
        ICallUnmarshal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Unmarshal: Unmarshal::<Identity, Impl, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInterfaceRelated_Impl: ::windows_core::BaseImpl {
    fn SetIID(this: &Self::This, iid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetIID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IInterfaceRelated {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterfaceRelated_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInterfaceRelated {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterfaceRelated_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIID(this, ::core::mem::transmute_copy(&iid)).into())
        }
        unsafe extern "system" fn GetIID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterfaceRelated_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInterfaceRelated_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIID: SetIID::<Identity, Impl, OFFSET>,
            GetIID: GetIID::<Identity, Impl, OFFSET>,
        }
    };
}
