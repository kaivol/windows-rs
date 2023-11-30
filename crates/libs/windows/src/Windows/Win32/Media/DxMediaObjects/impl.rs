pub trait IDMOQualityControl_Impl: ::windows_core::BaseImpl {
    fn SetNow(this: &Self::This, rtnow: i64) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IDMOQualityControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMOQualityControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNow(this, ::core::mem::transmute_copy(&rtnow)).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDMOQualityControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNow: SetNow::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDMOVideoOutputOptimizations_Impl: ::windows_core::BaseImpl {
    fn QueryOperationModePreferences(this: &Self::This, uloutputstreamindex: u32) -> ::windows_core::Result<u32>;
    fn SetOperationMode(this: &Self::This, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows_core::Result<()>;
    fn GetCurrentOperationMode(this: &Self::This, uloutputstreamindex: u32) -> ::windows_core::Result<u32>;
    fn GetCurrentSampleRequirements(this: &Self::This, uloutputstreamindex: u32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IDMOVideoOutputOptimizations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDMOVideoOutputOptimizations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryOperationModePreferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryOperationModePreferences(this, ::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrequestedcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOperationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOperationMode(this, ::core::mem::transmute_copy(&uloutputstreamindex), ::core::mem::transmute_copy(&dwenabledfeatures)).into())
        }
        unsafe extern "system" fn GetCurrentOperationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentOperationMode(this, ::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwenabledfeatures, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentSampleRequirements(this, ::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrequestedfeatures, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDMOVideoOutputOptimizations_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryOperationModePreferences: QueryOperationModePreferences::<Identity, Impl, OFFSET>,
            SetOperationMode: SetOperationMode::<Identity, Impl, OFFSET>,
            GetCurrentOperationMode: GetCurrentOperationMode::<Identity, Impl, OFFSET>,
            GetCurrentSampleRequirements: GetCurrentSampleRequirements::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumDMO_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, citemstofetch: u32, pclsid: *mut ::windows_core::GUID, names: *mut ::windows_core::PWSTR, pcitemsfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, citemstoskip: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDMO>;
}
impl ::windows_core::Iids for IEnumDMO {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDMO {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, citemstofetch: u32, pclsid: *mut ::windows_core::GUID, names: *mut ::windows_core::PWSTR, pcitemsfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&citemstofetch), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&names), ::core::mem::transmute_copy(&pcitemsfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, citemstoskip: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&citemstoskip)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDMO_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMediaBuffer_Impl: ::windows_core::BaseImpl {
    fn SetLength(this: &Self::This, cblength: u32) -> ::windows_core::Result<()>;
    fn GetMaxLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetBufferAndLength(this: &Self::This, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMediaBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLength(this, ::core::mem::transmute_copy(&cblength)).into())
        }
        unsafe extern "system" fn GetMaxLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmaxlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferAndLength(this, ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pcblength)).into())
        }
        IMediaBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMediaObject_Impl: ::windows_core::BaseImpl {
    fn GetStreamCount(this: &Self::This, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows_core::Result<()>;
    fn GetInputStreamInfo(this: &Self::This, dwinputstreamindex: u32) -> ::windows_core::Result<u32>;
    fn GetOutputStreamInfo(this: &Self::This, dwoutputstreamindex: u32) -> ::windows_core::Result<u32>;
    fn GetInputType(this: &Self::This, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()>;
    fn GetOutputType(this: &Self::This, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()>;
    fn SetInputType(this: &Self::This, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_core::Result<()>;
    fn SetOutputType(this: &Self::This, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetInputCurrentType(this: &Self::This, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()>;
    fn GetOutputCurrentType(this: &Self::This, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()>;
    fn GetInputSizeInfo(this: &Self::This, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows_core::Result<()>;
    fn GetOutputSizeInfo(this: &Self::This, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows_core::Result<()>;
    fn GetInputMaxLatency(this: &Self::This, dwinputstreamindex: u32) -> ::windows_core::Result<i64>;
    fn SetInputMaxLatency(this: &Self::This, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
    fn Discontinuity(this: &Self::This, dwinputstreamindex: u32) -> ::windows_core::Result<()>;
    fn AllocateStreamingResources(this: &Self::This) -> ::windows_core::Result<()>;
    fn FreeStreamingResources(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetInputStatus(this: &Self::This, dwinputstreamindex: u32) -> ::windows_core::Result<u32>;
    fn ProcessInput(this: &Self::This, dwinputstreamindex: u32, pbuffer: ::core::option::Option<&IMediaBuffer>, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows_core::Result<()>;
    fn ProcessOutput(this: &Self::This, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows_core::Result<()>;
    fn Lock(this: &Self::This, block: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMediaObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStreamCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamCount(this, ::core::mem::transmute_copy(&pcinputstreams), ::core::mem::transmute_copy(&pcoutputstreams)).into())
        }
        unsafe extern "system" fn GetInputStreamInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputStreamInfo(this, ::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputStreamInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputStreamInfo(this, ::core::mem::transmute_copy(&dwoutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputType(this, ::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&dwtypeindex), ::core::mem::transmute_copy(&pmt)).into())
        }
        unsafe extern "system" fn GetOutputType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputType(this, ::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&dwtypeindex), ::core::mem::transmute_copy(&pmt)).into())
        }
        unsafe extern "system" fn SetInputType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputType(this, ::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&pmt), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn SetOutputType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputType(this, ::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&pmt), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputCurrentType(this, ::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&pmt)).into())
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputCurrentType(this, ::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&pmt)).into())
        }
        unsafe extern "system" fn GetInputSizeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputSizeInfo(this, ::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbmaxlookahead), ::core::mem::transmute_copy(&pcbalignment)).into())
        }
        unsafe extern "system" fn GetOutputSizeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputSizeInfo(this, ::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbalignment)).into())
        }
        unsafe extern "system" fn GetInputMaxLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputMaxLatency(this, ::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prtmaxlatency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInputMaxLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputMaxLatency(this, ::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&rtmaxlatency)).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        unsafe extern "system" fn Discontinuity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Discontinuity(this, ::core::mem::transmute_copy(&dwinputstreamindex)).into())
        }
        unsafe extern "system" fn AllocateStreamingResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateStreamingResources(this).into())
        }
        unsafe extern "system" fn FreeStreamingResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeStreamingResources(this).into())
        }
        unsafe extern "system" fn GetInputStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputStatus(this, ::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProcessInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pbuffer: *mut ::core::ffi::c_void, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessInput(this, ::core::mem::transmute_copy(&dwinputstreamindex), ::windows_core::from_raw_borrowed(&pbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&rttimestamp), ::core::mem::transmute_copy(&rttimelength)).into())
        }
        unsafe extern "system" fn ProcessOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessOutput(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&coutputbuffercount), ::core::mem::transmute_copy(&poutputbuffers), ::core::mem::transmute_copy(&pdwstatus)).into())
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, block: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lock(this, ::core::mem::transmute_copy(&block)).into())
        }
        IMediaObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStreamCount: GetStreamCount::<Identity, Impl, OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Identity, Impl, OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Identity, Impl, OFFSET>,
            GetInputType: GetInputType::<Identity, Impl, OFFSET>,
            GetOutputType: GetOutputType::<Identity, Impl, OFFSET>,
            SetInputType: SetInputType::<Identity, Impl, OFFSET>,
            SetOutputType: SetOutputType::<Identity, Impl, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, Impl, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, Impl, OFFSET>,
            GetInputSizeInfo: GetInputSizeInfo::<Identity, Impl, OFFSET>,
            GetOutputSizeInfo: GetOutputSizeInfo::<Identity, Impl, OFFSET>,
            GetInputMaxLatency: GetInputMaxLatency::<Identity, Impl, OFFSET>,
            SetInputMaxLatency: SetInputMaxLatency::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            Discontinuity: Discontinuity::<Identity, Impl, OFFSET>,
            AllocateStreamingResources: AllocateStreamingResources::<Identity, Impl, OFFSET>,
            FreeStreamingResources: FreeStreamingResources::<Identity, Impl, OFFSET>,
            GetInputStatus: GetInputStatus::<Identity, Impl, OFFSET>,
            ProcessInput: ProcessInput::<Identity, Impl, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMediaObjectInPlace_Impl: ::windows_core::BaseImpl {
    fn Process(this: &Self::This, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IMediaObjectInPlace>;
    fn GetLatency(this: &Self::This) -> ::windows_core::Result<i64>;
}
impl ::windows_core::Iids for IMediaObjectInPlace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaObjectInPlace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Process<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Process(this, ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&reftimestart), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmediaobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLatency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(platencytime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaObjectInPlace_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Process: Process::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetLatency: GetLatency::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
