pub trait ITraceEvent_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<ITraceEvent>;
    fn GetUserContext(this: &Self::This) -> ::windows_core::Result<*mut ::core::ffi::c_void>;
    fn GetEventRecord(this: &Self::This) -> ::windows_core::Result<*mut EVENT_RECORD>;
    fn SetPayload(this: &Self::This, payload: *const u8, payloadsize: u32) -> ::windows_core::Result<()>;
    fn SetEventDescriptor(this: &Self::This, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows_core::Result<()>;
    fn SetProcessId(this: &Self::This, processid: u32) -> ::windows_core::Result<()>;
    fn SetProcessorIndex(this: &Self::This, processorindex: u32) -> ::windows_core::Result<()>;
    fn SetThreadId(this: &Self::This, threadid: u32) -> ::windows_core::Result<()>;
    fn SetThreadTimes(this: &Self::This, kerneltime: u32, usertime: u32) -> ::windows_core::Result<()>;
    fn SetActivityId(this: &Self::This, activityid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetTimeStamp(this: &Self::This, timestamp: *const i64) -> ::windows_core::Result<()>;
    fn SetProviderId(this: &Self::This, providerid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITraceEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITraceEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usercontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventRecord(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPayload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, payload: *const u8, payloadsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPayload(this, ::core::mem::transmute_copy(&payload), ::core::mem::transmute_copy(&payloadsize)).into())
        }
        unsafe extern "system" fn SetEventDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventDescriptor(this, ::core::mem::transmute_copy(&eventdescriptor)).into())
        }
        unsafe extern "system" fn SetProcessId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProcessId(this, ::core::mem::transmute_copy(&processid)).into())
        }
        unsafe extern "system" fn SetProcessorIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processorindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProcessorIndex(this, ::core::mem::transmute_copy(&processorindex)).into())
        }
        unsafe extern "system" fn SetThreadId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThreadId(this, ::core::mem::transmute_copy(&threadid)).into())
        }
        unsafe extern "system" fn SetThreadTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kerneltime: u32, usertime: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThreadTimes(this, ::core::mem::transmute_copy(&kerneltime), ::core::mem::transmute_copy(&usertime)).into())
        }
        unsafe extern "system" fn SetActivityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activityid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActivityId(this, ::core::mem::transmute_copy(&activityid)).into())
        }
        unsafe extern "system" fn SetTimeStamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: *const i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimeStamp(this, ::core::mem::transmute_copy(&timestamp)).into())
        }
        unsafe extern "system" fn SetProviderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProviderId(this, ::core::mem::transmute_copy(&providerid)).into())
        }
        ITraceEvent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetUserContext: GetUserContext::<Identity, Impl, OFFSET>,
            GetEventRecord: GetEventRecord::<Identity, Impl, OFFSET>,
            SetPayload: SetPayload::<Identity, Impl, OFFSET>,
            SetEventDescriptor: SetEventDescriptor::<Identity, Impl, OFFSET>,
            SetProcessId: SetProcessId::<Identity, Impl, OFFSET>,
            SetProcessorIndex: SetProcessorIndex::<Identity, Impl, OFFSET>,
            SetThreadId: SetThreadId::<Identity, Impl, OFFSET>,
            SetThreadTimes: SetThreadTimes::<Identity, Impl, OFFSET>,
            SetActivityId: SetActivityId::<Identity, Impl, OFFSET>,
            SetTimeStamp: SetTimeStamp::<Identity, Impl, OFFSET>,
            SetProviderId: SetProviderId::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITraceEventCallback_Impl: ::windows_core::BaseImpl {
    fn OnBeginProcessTrace(this: &Self::This, headerevent: ::core::option::Option<&ITraceEvent>, relogger: ::core::option::Option<&ITraceRelogger>) -> ::windows_core::Result<()>;
    fn OnFinalizeProcessTrace(this: &Self::This, relogger: ::core::option::Option<&ITraceRelogger>) -> ::windows_core::Result<()>;
    fn OnEvent(this: &Self::This, event: ::core::option::Option<&ITraceEvent>, relogger: ::core::option::Option<&ITraceRelogger>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITraceEventCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITraceEventCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnBeginProcessTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, headerevent: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBeginProcessTrace(this, ::windows_core::from_raw_borrowed(&headerevent), ::windows_core::from_raw_borrowed(&relogger)).into())
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFinalizeProcessTrace(this, ::windows_core::from_raw_borrowed(&relogger)).into())
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEvent(this, ::windows_core::from_raw_borrowed(&event), ::windows_core::from_raw_borrowed(&relogger)).into())
        }
        ITraceEventCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnBeginProcessTrace: OnBeginProcessTrace::<Identity, Impl, OFFSET>,
            OnFinalizeProcessTrace: OnFinalizeProcessTrace::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITraceRelogger_Impl: ::windows_core::BaseImpl {
    fn AddLogfileTraceStream(this: &Self::This, logfilename: &::windows_core::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows_core::Result<RELOGSTREAM_HANDLE>;
    fn AddRealtimeTraceStream(this: &Self::This, loggername: &::windows_core::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows_core::Result<RELOGSTREAM_HANDLE>;
    fn RegisterCallback(this: &Self::This, callback: ::core::option::Option<&ITraceEventCallback>) -> ::windows_core::Result<()>;
    fn Inject(this: &Self::This, event: ::core::option::Option<&ITraceEvent>) -> ::windows_core::Result<()>;
    fn CreateEventInstance(this: &Self::This, tracehandle: &RELOGSTREAM_HANDLE, flags: u32) -> ::windows_core::Result<ITraceEvent>;
    fn ProcessTrace(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetOutputFilename(this: &Self::This, logfilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetCompressionMode(this: &Self::This, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITraceRelogger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITraceRelogger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddLogfileTraceStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddLogfileTraceStream(this, ::core::mem::transmute(&logfilename), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tracehandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggername: ::std::mem::MaybeUninit<::windows_core::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddRealtimeTraceStream(this, ::core::mem::transmute(&loggername), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tracehandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterCallback(this, ::windows_core::from_raw_borrowed(&callback)).into())
        }
        unsafe extern "system" fn Inject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Inject(this, ::windows_core::from_raw_borrowed(&event)).into())
        }
        unsafe extern "system" fn CreateEventInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tracehandle: RELOGSTREAM_HANDLE, flags: u32, event: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEventInstance(this, ::core::mem::transmute(&tracehandle), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(event, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProcessTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessTrace(this).into())
        }
        unsafe extern "system" fn SetOutputFilename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputFilename(this, ::core::mem::transmute(&logfilename)).into())
        }
        unsafe extern "system" fn SetCompressionMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompressionMode(this, ::core::mem::transmute_copy(&compressionmode)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        ITraceRelogger_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddLogfileTraceStream: AddLogfileTraceStream::<Identity, Impl, OFFSET>,
            AddRealtimeTraceStream: AddRealtimeTraceStream::<Identity, Impl, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, Impl, OFFSET>,
            Inject: Inject::<Identity, Impl, OFFSET>,
            CreateEventInstance: CreateEventInstance::<Identity, Impl, OFFSET>,
            ProcessTrace: ProcessTrace::<Identity, Impl, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, Impl, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
