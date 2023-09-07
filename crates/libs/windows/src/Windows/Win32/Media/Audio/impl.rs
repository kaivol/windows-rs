pub trait IAcousticEchoCancellationControl_Impl: ::windows_core::BaseImpl {
    fn SetEchoCancellationRenderEndpoint(this: &Self::This, endpointid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAcousticEchoCancellationControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAcousticEchoCancellationControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAcousticEchoCancellationControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEchoCancellationRenderEndpoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAcousticEchoCancellationControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpointid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEchoCancellationRenderEndpoint(this, ::core::mem::transmute(&endpointid)).into())
        }
        IAcousticEchoCancellationControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetEchoCancellationRenderEndpoint: SetEchoCancellationRenderEndpoint::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IActivateAudioInterfaceAsyncOperation_Impl: ::windows_core::BaseImpl {
    fn GetActivateResult(this: &Self::This, activateresult: *mut ::windows_core::HRESULT, activatedinterface: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActivateAudioInterfaceAsyncOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActivateAudioInterfaceAsyncOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActivateResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activateresult: *mut ::windows_core::HRESULT, activatedinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActivateResult(this, ::core::mem::transmute_copy(&activateresult), ::core::mem::transmute_copy(&activatedinterface)).into())
        }
        IActivateAudioInterfaceAsyncOperation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetActivateResult: GetActivateResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IActivateAudioInterfaceCompletionHandler_Impl: ::windows_core::BaseImpl {
    fn ActivateCompleted(this: &Self::This, activateoperation: ::core::option::Option<&IActivateAudioInterfaceAsyncOperation>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActivateAudioInterfaceCompletionHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActivateAudioInterfaceCompletionHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activateoperation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateCompleted(this, ::windows_core::from_raw_borrowed(&activateoperation)).into())
        }
        IActivateAudioInterfaceCompletionHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateCompleted: ActivateCompleted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioAmbisonicsControl_Impl: ::windows_core::BaseImpl {
    fn SetData(this: &Self::This, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows_core::Result<()>;
    fn SetHeadTracking(this: &Self::This, benableheadtracking: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetHeadTracking(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetRotation(this: &Self::This, x: f32, y: f32, z: f32, w: f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioAmbisonicsControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioAmbisonicsControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute_copy(&pambisonicsparams), ::core::mem::transmute_copy(&cbambisonicsparams)).into())
        }
        unsafe extern "system" fn SetHeadTracking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benableheadtracking: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHeadTracking(this, ::core::mem::transmute_copy(&benableheadtracking)).into())
        }
        unsafe extern "system" fn GetHeadTracking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHeadTracking(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenableheadtracking, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRotation(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&w)).into())
        }
        IAudioAmbisonicsControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetData: SetData::<Identity, Impl, OFFSET>,
            SetHeadTracking: SetHeadTracking::<Identity, Impl, OFFSET>,
            GetHeadTracking: GetHeadTracking::<Identity, Impl, OFFSET>,
            SetRotation: SetRotation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioAutoGainControl_Impl: ::windows_core::BaseImpl {
    fn GetEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(this: &Self::This, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioAutoGainControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAutoGainControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioAutoGainControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAutoGainControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioAutoGainControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&benable), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        IAudioAutoGainControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEnabled: GetEnabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioBass_Impl: ::windows_core::BaseImpl + IPerChannelDbLevel_Impl {}
impl ::windows_core::Iids for IAudioBass {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPerChannelDbLevel);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioBass_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioBass {
    const VTABLE: Self::Vtable = { IAudioBass_Vtbl { base__: <IPerChannelDbLevel as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioCaptureClient_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::Result<()>;
    fn ReleaseBuffer(this: &Self::This, numframesread: u32) -> ::windows_core::Result<()>;
    fn GetNextPacketSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAudioCaptureClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioCaptureClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pnumframestoread), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pu64deviceposition), ::core::mem::transmute_copy(&pu64qpcposition)).into())
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseBuffer(this, ::core::mem::transmute_copy(&numframesread)).into())
        }
        unsafe extern "system" fn GetNextPacketSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextPacketSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumframesinnextpacket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioCaptureClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, Impl, OFFSET>,
            GetNextPacketSize: GetNextPacketSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioChannelConfig_Impl: ::windows_core::BaseImpl {
    fn SetChannelConfig(this: &Self::This, dwconfig: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetChannelConfig(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAudioChannelConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioChannelConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioChannelConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetChannelConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioChannelConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconfig: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelConfig(this, ::core::mem::transmute_copy(&dwconfig), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn GetChannelConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioChannelConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwconfig: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelConfig(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconfig, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioChannelConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetChannelConfig: SetChannelConfig::<Identity, Impl, OFFSET>,
            GetChannelConfig: GetChannelConfig::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetBufferSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetStreamLatency(this: &Self::This) -> ::windows_core::Result<i64>;
    fn GetCurrentPadding(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsFormatSupported(this: &Self::This, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows_core::HRESULT;
    fn GetMixFormat(this: &Self::This) -> ::windows_core::Result<*mut WAVEFORMATEX>;
    fn GetDevicePeriod(this: &Self::This, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows_core::Result<()>;
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetEventHandle(this: &Self::This, eventhandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetService(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&sharemode), ::core::mem::transmute_copy(&streamflags), ::core::mem::transmute_copy(&hnsbufferduration), ::core::mem::transmute_copy(&hnsperiodicity), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&audiosessionguid)).into())
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumbufferframes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBufferSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumbufferframes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStreamLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phnslatency: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamLatency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phnslatency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentPadding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumpaddingframes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentPadding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumpaddingframes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFormatSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsFormatSupported(this, ::core::mem::transmute_copy(&sharemode), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&ppclosestmatch)))
        }
        unsafe extern "system" fn GetMixFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMixFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdeviceformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDevicePeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevicePeriod(this, ::core::mem::transmute_copy(&phnsdefaultdeviceperiod), ::core::mem::transmute_copy(&phnsminimumdeviceperiod)).into())
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn SetEventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventHandle(this, ::core::mem::transmute_copy(&eventhandle)).into())
        }
        unsafe extern "system" fn GetService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetService(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IAudioClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
            GetStreamLatency: GetStreamLatency::<Identity, Impl, OFFSET>,
            GetCurrentPadding: GetCurrentPadding::<Identity, Impl, OFFSET>,
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
            GetMixFormat: GetMixFormat::<Identity, Impl, OFFSET>,
            GetDevicePeriod: GetDevicePeriod::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetEventHandle: SetEventHandle::<Identity, Impl, OFFSET>,
            GetService: GetService::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient2_Impl: ::windows_core::BaseImpl + IAudioClient_Impl {
    fn IsOffloadCapable(this: &Self::This, category: AUDIO_STREAM_CATEGORY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetClientProperties(this: &Self::This, pproperties: *const AudioClientProperties) -> ::windows_core::Result<()>;
    fn GetBufferSizeLimits(this: &Self::This, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioClient2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioClient);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioClient2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsOffloadCapable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOffloadCapable(this, ::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboffloadcapable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproperties: *const AudioClientProperties) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientProperties(this, ::core::mem::transmute_copy(&pproperties)).into())
        }
        unsafe extern "system" fn GetBufferSizeLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferSizeLimits(this, ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&beventdriven), ::core::mem::transmute_copy(&phnsminbufferduration), ::core::mem::transmute_copy(&phnsmaxbufferduration)).into())
        }
        IAudioClient2_Vtbl {
            base__: <IAudioClient as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsOffloadCapable: IsOffloadCapable::<Identity, Impl, OFFSET>,
            SetClientProperties: SetClientProperties::<Identity, Impl, OFFSET>,
            GetBufferSizeLimits: GetBufferSizeLimits::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient3_Impl: ::windows_core::BaseImpl + IAudioClient2_Impl {
    fn GetSharedModeEnginePeriod(this: &Self::This, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows_core::Result<()>;
    fn GetCurrentSharedModeEnginePeriod(this: &Self::This, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows_core::Result<()>;
    fn InitializeSharedAudioStream(this: &Self::This, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioClient3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioClient2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioClient3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSharedModeEnginePeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSharedModeEnginePeriod(this, ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pdefaultperiodinframes), ::core::mem::transmute_copy(&pfundamentalperiodinframes), ::core::mem::transmute_copy(&pminperiodinframes), ::core::mem::transmute_copy(&pmaxperiodinframes)).into())
        }
        unsafe extern "system" fn GetCurrentSharedModeEnginePeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentSharedModeEnginePeriod(this, ::core::mem::transmute_copy(&ppformat), ::core::mem::transmute_copy(&pcurrentperiodinframes)).into())
        }
        unsafe extern "system" fn InitializeSharedAudioStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeSharedAudioStream(this, ::core::mem::transmute_copy(&streamflags), ::core::mem::transmute_copy(&periodinframes), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&audiosessionguid)).into())
        }
        IAudioClient3_Vtbl {
            base__: <IAudioClient2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSharedModeEnginePeriod: GetSharedModeEnginePeriod::<Identity, Impl, OFFSET>,
            GetCurrentSharedModeEnginePeriod: GetCurrentSharedModeEnginePeriod::<Identity, Impl, OFFSET>,
            InitializeSharedAudioStream: InitializeSharedAudioStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioClientDuckingControl_Impl: ::windows_core::BaseImpl {
    fn SetDuckingOptionsForCurrentStream(this: &Self::This, options: AUDIO_DUCKING_OPTIONS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioClientDuckingControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClientDuckingControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioClientDuckingControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDuckingOptionsForCurrentStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClientDuckingControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuckingOptionsForCurrentStream(this, ::core::mem::transmute_copy(&options)).into())
        }
        IAudioClientDuckingControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDuckingOptionsForCurrentStream: SetDuckingOptionsForCurrentStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioClock_Impl: ::windows_core::BaseImpl {
    fn GetFrequency(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetPosition(this: &Self::This, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::Result<()>;
    fn GetCharacteristics(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAudioClock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioClock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pu64frequency: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrequency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu64frequency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPosition(this, ::core::mem::transmute_copy(&pu64position), ::core::mem::transmute_copy(&pu64qpcposition)).into())
        }
        unsafe extern "system" fn GetCharacteristics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCharacteristics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcharacteristics, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioClock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFrequency: GetFrequency::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioClock2_Impl: ::windows_core::BaseImpl {
    fn GetDevicePosition(this: &Self::This, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioClock2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClock2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioClock2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevicePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClock2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevicePosition(this, ::core::mem::transmute_copy(&deviceposition), ::core::mem::transmute_copy(&qpcposition)).into())
        }
        IAudioClock2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevicePosition: GetDevicePosition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioClockAdjustment_Impl: ::windows_core::BaseImpl {
    fn SetSampleRate(this: &Self::This, flsamplerate: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioClockAdjustment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClockAdjustment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioClockAdjustment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSampleRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioClockAdjustment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flsamplerate: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSampleRate(this, ::core::mem::transmute_copy(&flsamplerate)).into())
        }
        IAudioClockAdjustment_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetSampleRate: SetSampleRate::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioEffectsChangedNotificationClient_Impl: ::windows_core::BaseImpl {
    fn OnAudioEffectsChanged(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioEffectsChangedNotificationClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEffectsChangedNotificationClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnAudioEffectsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAudioEffectsChanged(this).into())
        }
        IAudioEffectsChangedNotificationClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnAudioEffectsChanged: OnAudioEffectsChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEffectsManager_Impl: ::windows_core::BaseImpl {
    fn RegisterAudioEffectsChangedNotificationCallback(this: &Self::This, client: ::core::option::Option<&IAudioEffectsChangedNotificationClient>) -> ::windows_core::Result<()>;
    fn UnregisterAudioEffectsChangedNotificationCallback(this: &Self::This, client: ::core::option::Option<&IAudioEffectsChangedNotificationClient>) -> ::windows_core::Result<()>;
    fn GetAudioEffects(this: &Self::This, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows_core::Result<()>;
    fn SetAudioEffectState(this: &Self::This, effectid: &::windows_core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioEffectsManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEffectsManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterAudioEffectsChangedNotificationCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, client: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterAudioEffectsChangedNotificationCallback(this, ::windows_core::from_raw_borrowed(&client)).into())
        }
        unsafe extern "system" fn UnregisterAudioEffectsChangedNotificationCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, client: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterAudioEffectsChangedNotificationCallback(this, ::windows_core::from_raw_borrowed(&client)).into())
        }
        unsafe extern "system" fn GetAudioEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAudioEffects(this, ::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&numeffects)).into())
        }
        unsafe extern "system" fn SetAudioEffectState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: ::windows_core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAudioEffectState(this, ::core::mem::transmute(&effectid), ::core::mem::transmute_copy(&state)).into())
        }
        IAudioEffectsManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterAudioEffectsChangedNotificationCallback: RegisterAudioEffectsChangedNotificationCallback::<Identity, Impl, OFFSET>,
            UnregisterAudioEffectsChangedNotificationCallback: UnregisterAudioEffectsChangedNotificationCallback::<Identity, Impl, OFFSET>,
            GetAudioEffects: GetAudioEffects::<Identity, Impl, OFFSET>,
            SetAudioEffectState: SetAudioEffectState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioFormatEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFormat(this: &Self::This, index: u32) -> ::windows_core::Result<*mut WAVEFORMATEX>;
}
impl ::windows_core::Iids for IAudioFormatEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioFormatEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioFormatEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioFormatEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioFormatEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormat(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioFormatEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioInputSelector_Impl: ::windows_core::BaseImpl {
    fn GetSelection(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetSelection(this: &Self::This, nidselect: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioInputSelector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputSelector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioInputSelector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnidselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelection(this, ::core::mem::transmute_copy(&nidselect), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        IAudioInputSelector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioLoudness_Impl: ::windows_core::BaseImpl {
    fn GetEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(this: &Self::This, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioLoudness {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioLoudness_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioLoudness {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioLoudness_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioLoudness_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&benable), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        IAudioLoudness_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEnabled: GetEnabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioMidrange_Impl: ::windows_core::BaseImpl + IPerChannelDbLevel_Impl {}
impl ::windows_core::Iids for IAudioMidrange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPerChannelDbLevel);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMidrange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioMidrange {
    const VTABLE: Self::Vtable = { IAudioMidrange_Vtbl { base__: <IPerChannelDbLevel as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioMute_Impl: ::windows_core::BaseImpl {
    fn SetMute(this: &Self::This, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetMute(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioMute {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMute_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioMute {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMute(this, ::core::mem::transmute_copy(&bmuted), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn GetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmuted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioMute_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMute: SetMute::<Identity, Impl, OFFSET>,
            GetMute: GetMute::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioOutputSelector_Impl: ::windows_core::BaseImpl {
    fn GetSelection(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetSelection(this: &Self::This, nidselect: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioOutputSelector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioOutputSelector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioOutputSelector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioOutputSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnidselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioOutputSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelection(this, ::core::mem::transmute_copy(&nidselect), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        IAudioOutputSelector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioPeakMeter_Impl: ::windows_core::BaseImpl {
    fn GetChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLevel(this: &Self::This, nchannel: u32) -> ::windows_core::Result<f32>;
}
impl ::windows_core::Iids for IAudioPeakMeter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioPeakMeter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioPeakMeter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioPeakMeter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchannels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioPeakMeter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLevel(this, ::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioPeakMeter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            GetLevel: GetLevel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioRenderClient_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, numframesrequested: u32) -> ::windows_core::Result<*mut u8>;
    fn ReleaseBuffer(this: &Self::This, numframeswritten: u32, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioRenderClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioRenderClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioRenderClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioRenderClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBuffer(this, ::core::mem::transmute_copy(&numframesrequested)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioRenderClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseBuffer(this, ::core::mem::transmute_copy(&numframeswritten), ::core::mem::transmute_copy(&dwflags)).into())
        }
        IAudioRenderClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioSessionControl_Impl: ::windows_core::BaseImpl {
    fn GetState(this: &Self::This) -> ::windows_core::Result<AudioSessionState>;
    fn GetDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDisplayName(this: &Self::This, value: &::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetIconPath(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetIconPath(this: &Self::This, value: &::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetGroupingParam(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetGroupingParam(this: &Self::This, r#override: *const ::windows_core::GUID, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RegisterAudioSessionNotification(this: &Self::This, newnotifications: ::core::option::Option<&IAudioSessionEvents>) -> ::windows_core::Result<()>;
    fn UnregisterAudioSessionNotification(this: &Self::This, newnotifications: ::core::option::Option<&IAudioSessionEvents>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioSessionControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut AudioSessionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&value), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn GetIconPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIconPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIconPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIconPath(this, ::core::mem::transmute(&value), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn GetGroupingParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGroupingParam(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroupingParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#override: *const ::windows_core::GUID, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroupingParam(this, ::core::mem::transmute_copy(&r#override), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn RegisterAudioSessionNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newnotifications: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterAudioSessionNotification(this, ::windows_core::from_raw_borrowed(&newnotifications)).into())
        }
        unsafe extern "system" fn UnregisterAudioSessionNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newnotifications: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterAudioSessionNotification(this, ::windows_core::from_raw_borrowed(&newnotifications)).into())
        }
        IAudioSessionControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetIconPath: GetIconPath::<Identity, Impl, OFFSET>,
            SetIconPath: SetIconPath::<Identity, Impl, OFFSET>,
            GetGroupingParam: GetGroupingParam::<Identity, Impl, OFFSET>,
            SetGroupingParam: SetGroupingParam::<Identity, Impl, OFFSET>,
            RegisterAudioSessionNotification: RegisterAudioSessionNotification::<Identity, Impl, OFFSET>,
            UnregisterAudioSessionNotification: UnregisterAudioSessionNotification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionControl2_Impl: ::windows_core::BaseImpl + IAudioSessionControl_Impl {
    fn GetSessionIdentifier(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSessionInstanceIdentifier(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetProcessId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsSystemSoundsSession(this: &Self::This) -> ::windows_core::HRESULT;
    fn SetDuckingPreference(this: &Self::This, optout: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioSessionControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioSessionControl);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSessionIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSessionIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSessionInstanceIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSessionInstanceIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProcessId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProcessId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSystemSoundsSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSystemSoundsSession(this))
        }
        unsafe extern "system" fn SetDuckingPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optout: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuckingPreference(this, ::core::mem::transmute_copy(&optout)).into())
        }
        IAudioSessionControl2_Vtbl {
            base__: <IAudioSessionControl as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSessionIdentifier: GetSessionIdentifier::<Identity, Impl, OFFSET>,
            GetSessionInstanceIdentifier: GetSessionInstanceIdentifier::<Identity, Impl, OFFSET>,
            GetProcessId: GetProcessId::<Identity, Impl, OFFSET>,
            IsSystemSoundsSession: IsSystemSoundsSession::<Identity, Impl, OFFSET>,
            SetDuckingPreference: SetDuckingPreference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioSessionEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetSession(this: &Self::This, sessioncount: i32) -> ::windows_core::Result<IAudioSessionControl>;
}
impl ::windows_core::Iids for IAudioSessionEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessioncount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sessioncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessioncount: i32, session: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSession(this, ::core::mem::transmute_copy(&sessioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(session, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioSessionEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetSession: GetSession::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionEvents_Impl: ::windows_core::BaseImpl {
    fn OnDisplayNameChanged(this: &Self::This, newdisplayname: &::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnIconPathChanged(this: &Self::This, newiconpath: &::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnSimpleVolumeChanged(this: &Self::This, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnChannelVolumeChanged(this: &Self::This, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnGroupingParamChanged(this: &Self::This, newgroupingparam: *const ::windows_core::GUID, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnStateChanged(this: &Self::This, newstate: AudioSessionState) -> ::windows_core::Result<()>;
    fn OnSessionDisconnected(this: &Self::This, disconnectreason: AudioSessionDisconnectReason) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioSessionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDisplayNameChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newdisplayname: ::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDisplayNameChanged(this, ::core::mem::transmute(&newdisplayname), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn OnIconPathChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newiconpath: ::windows_core::PCWSTR, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIconPathChanged(this, ::core::mem::transmute(&newiconpath), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn OnSimpleVolumeChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSimpleVolumeChanged(this, ::core::mem::transmute_copy(&newvolume), ::core::mem::transmute_copy(&newmute), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn OnChannelVolumeChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChannelVolumeChanged(this, ::core::mem::transmute_copy(&channelcount), ::core::mem::transmute_copy(&newchannelvolumearray), ::core::mem::transmute_copy(&changedchannel), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn OnGroupingParamChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newgroupingparam: *const ::windows_core::GUID, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnGroupingParamChanged(this, ::core::mem::transmute_copy(&newgroupingparam), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn OnStateChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: AudioSessionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStateChanged(this, ::core::mem::transmute_copy(&newstate)).into())
        }
        unsafe extern "system" fn OnSessionDisconnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSessionDisconnected(this, ::core::mem::transmute_copy(&disconnectreason)).into())
        }
        IAudioSessionEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDisplayNameChanged: OnDisplayNameChanged::<Identity, Impl, OFFSET>,
            OnIconPathChanged: OnIconPathChanged::<Identity, Impl, OFFSET>,
            OnSimpleVolumeChanged: OnSimpleVolumeChanged::<Identity, Impl, OFFSET>,
            OnChannelVolumeChanged: OnChannelVolumeChanged::<Identity, Impl, OFFSET>,
            OnGroupingParamChanged: OnGroupingParamChanged::<Identity, Impl, OFFSET>,
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnSessionDisconnected: OnSessionDisconnected::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioSessionManager_Impl: ::windows_core::BaseImpl {
    fn GetAudioSessionControl(this: &Self::This, audiosessionguid: *const ::windows_core::GUID, streamflags: u32) -> ::windows_core::Result<IAudioSessionControl>;
    fn GetSimpleAudioVolume(this: &Self::This, audiosessionguid: *const ::windows_core::GUID, streamflags: u32) -> ::windows_core::Result<ISimpleAudioVolume>;
}
impl ::windows_core::Iids for IAudioSessionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAudioSessionControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows_core::GUID, streamflags: u32, sessioncontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudioSessionControl(this, ::core::mem::transmute_copy(&audiosessionguid), ::core::mem::transmute_copy(&streamflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sessioncontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSimpleAudioVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows_core::GUID, streamflags: u32, audiovolume: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSimpleAudioVolume(this, ::core::mem::transmute_copy(&audiosessionguid), ::core::mem::transmute_copy(&streamflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiovolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioSessionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAudioSessionControl: GetAudioSessionControl::<Identity, Impl, OFFSET>,
            GetSimpleAudioVolume: GetSimpleAudioVolume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioSessionManager2_Impl: ::windows_core::BaseImpl + IAudioSessionManager_Impl {
    fn GetSessionEnumerator(this: &Self::This) -> ::windows_core::Result<IAudioSessionEnumerator>;
    fn RegisterSessionNotification(this: &Self::This, sessionnotification: ::core::option::Option<&IAudioSessionNotification>) -> ::windows_core::Result<()>;
    fn UnregisterSessionNotification(this: &Self::This, sessionnotification: ::core::option::Option<&IAudioSessionNotification>) -> ::windows_core::Result<()>;
    fn RegisterDuckNotification(this: &Self::This, sessionid: &::windows_core::PCWSTR, ducknotification: ::core::option::Option<&IAudioVolumeDuckNotification>) -> ::windows_core::Result<()>;
    fn UnregisterDuckNotification(this: &Self::This, ducknotification: ::core::option::Option<&IAudioVolumeDuckNotification>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioSessionManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioSessionManager);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSessionEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSessionEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sessionenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterSessionNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionnotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterSessionNotification(this, ::windows_core::from_raw_borrowed(&sessionnotification)).into())
        }
        unsafe extern "system" fn UnregisterSessionNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionnotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterSessionNotification(this, ::windows_core::from_raw_borrowed(&sessionnotification)).into())
        }
        unsafe extern "system" fn RegisterDuckNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: ::windows_core::PCWSTR, ducknotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterDuckNotification(this, ::core::mem::transmute(&sessionid), ::windows_core::from_raw_borrowed(&ducknotification)).into())
        }
        unsafe extern "system" fn UnregisterDuckNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ducknotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterDuckNotification(this, ::windows_core::from_raw_borrowed(&ducknotification)).into())
        }
        IAudioSessionManager2_Vtbl {
            base__: <IAudioSessionManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSessionEnumerator: GetSessionEnumerator::<Identity, Impl, OFFSET>,
            RegisterSessionNotification: RegisterSessionNotification::<Identity, Impl, OFFSET>,
            UnregisterSessionNotification: UnregisterSessionNotification::<Identity, Impl, OFFSET>,
            RegisterDuckNotification: RegisterDuckNotification::<Identity, Impl, OFFSET>,
            UnregisterDuckNotification: UnregisterDuckNotification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioSessionNotification_Impl: ::windows_core::BaseImpl {
    fn OnSessionCreated(this: &Self::This, newsession: ::core::option::Option<&IAudioSessionControl>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioSessionNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSessionCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newsession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSessionCreated(this, ::windows_core::from_raw_borrowed(&newsession)).into())
        }
        IAudioSessionNotification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSessionCreated: OnSessionCreated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioStateMonitor_Impl: ::windows_core::BaseImpl {
    fn RegisterCallback(this: &Self::This, callback: PAudioStateMonitorCallback, context: *const ::core::ffi::c_void) -> ::windows_core::Result<i64>;
    fn UnregisterCallback(this: &Self::This, registration: i64);
    fn GetSoundLevel(this: &Self::This) -> AudioStateMonitorSoundLevel;
}
impl ::windows_core::Iids for IAudioStateMonitor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioStateMonitor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: PAudioStateMonitorCallback, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterCallback(this, ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registration: i64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterCallback(this, ::core::mem::transmute_copy(&registration)))
        }
        unsafe extern "system" fn GetSoundLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> AudioStateMonitorSoundLevel {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSoundLevel(this))
        }
        IAudioStateMonitor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterCallback: RegisterCallback::<Identity, Impl, OFFSET>,
            UnregisterCallback: UnregisterCallback::<Identity, Impl, OFFSET>,
            GetSoundLevel: GetSoundLevel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioStreamVolume_Impl: ::windows_core::BaseImpl {
    fn GetChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetChannelVolume(this: &Self::This, dwindex: u32, flevel: f32) -> ::windows_core::Result<()>;
    fn GetChannelVolume(this: &Self::This, dwindex: u32) -> ::windows_core::Result<f32>;
    fn SetAllVolumes(this: &Self::This, dwcount: u32, pfvolumes: *const f32) -> ::windows_core::Result<()>;
    fn GetAllVolumes(this: &Self::This, dwcount: u32, pfvolumes: *mut f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioStreamVolume {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioStreamVolume {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChannelVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelVolume(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&flevel)).into())
        }
        unsafe extern "system" fn GetChannelVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelVolume(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllVolumes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllVolumes(this, ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into())
        }
        unsafe extern "system" fn GetAllVolumes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllVolumes(this, ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into())
        }
        IAudioStreamVolume_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, Impl, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, Impl, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, Impl, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyChangeNotificationClient_Impl: ::windows_core::BaseImpl {
    fn OnPropertyChanged(this: &Self::This, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IAudioSystemEffectsPropertyChangeNotificationClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSystemEffectsPropertyChangeNotificationClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnPropertyChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPropertyChanged(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&key)).into())
        }
        IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnPropertyChanged: OnPropertyChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyStore_Impl: ::windows_core::BaseImpl {
    fn OpenDefaultPropertyStore(this: &Self::This, stgmaccess: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenUserPropertyStore(this: &Self::This, stgmaccess: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenVolatilePropertyStore(this: &Self::This, stgmaccess: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn ResetUserPropertyStore(this: &Self::This) -> ::windows_core::Result<()>;
    fn ResetVolatilePropertyStore(this: &Self::This) -> ::windows_core::Result<()>;
    fn RegisterPropertyChangeNotification(this: &Self::This, callback: ::core::option::Option<&IAudioSystemEffectsPropertyChangeNotificationClient>) -> ::windows_core::Result<()>;
    fn UnregisterPropertyChangeNotification(this: &Self::This, callback: ::core::option::Option<&IAudioSystemEffectsPropertyChangeNotificationClient>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IAudioSystemEffectsPropertyStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSystemEffectsPropertyStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenDefaultPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenDefaultPropertyStore(this, ::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenUserPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenUserPropertyStore(this, ::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenVolatilePropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenVolatilePropertyStore(this, ::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResetUserPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetUserPropertyStore(this).into())
        }
        unsafe extern "system" fn ResetVolatilePropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetVolatilePropertyStore(this).into())
        }
        unsafe extern "system" fn RegisterPropertyChangeNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterPropertyChangeNotification(this, ::windows_core::from_raw_borrowed(&callback)).into())
        }
        unsafe extern "system" fn UnregisterPropertyChangeNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterPropertyChangeNotification(this, ::windows_core::from_raw_borrowed(&callback)).into())
        }
        IAudioSystemEffectsPropertyStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenDefaultPropertyStore: OpenDefaultPropertyStore::<Identity, Impl, OFFSET>,
            OpenUserPropertyStore: OpenUserPropertyStore::<Identity, Impl, OFFSET>,
            OpenVolatilePropertyStore: OpenVolatilePropertyStore::<Identity, Impl, OFFSET>,
            ResetUserPropertyStore: ResetUserPropertyStore::<Identity, Impl, OFFSET>,
            ResetVolatilePropertyStore: ResetVolatilePropertyStore::<Identity, Impl, OFFSET>,
            RegisterPropertyChangeNotification: RegisterPropertyChangeNotification::<Identity, Impl, OFFSET>,
            UnregisterPropertyChangeNotification: UnregisterPropertyChangeNotification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioTreble_Impl: ::windows_core::BaseImpl + IPerChannelDbLevel_Impl {}
impl ::windows_core::Iids for IAudioTreble {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPerChannelDbLevel);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioTreble_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioTreble {
    const VTABLE: Self::Vtable = { IAudioTreble_Vtbl { base__: <IPerChannelDbLevel as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioViewManagerService_Impl: ::windows_core::BaseImpl {
    fn SetAudioStreamWindow(this: &Self::This, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioViewManagerService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioViewManagerService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioViewManagerService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAudioStreamWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioViewManagerService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAudioStreamWindow(this, ::core::mem::transmute_copy(&hwnd)).into())
        }
        IAudioViewManagerService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAudioStreamWindow: SetAudioStreamWindow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioVolumeDuckNotification_Impl: ::windows_core::BaseImpl {
    fn OnVolumeDuckNotification(this: &Self::This, sessionid: &::windows_core::PCWSTR, countcommunicationsessions: u32) -> ::windows_core::Result<()>;
    fn OnVolumeUnduckNotification(this: &Self::This, sessionid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioVolumeDuckNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioVolumeDuckNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnVolumeDuckNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: ::windows_core::PCWSTR, countcommunicationsessions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnVolumeDuckNotification(this, ::core::mem::transmute(&sessionid), ::core::mem::transmute_copy(&countcommunicationsessions)).into())
        }
        unsafe extern "system" fn OnVolumeUnduckNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnVolumeUnduckNotification(this, ::core::mem::transmute(&sessionid)).into())
        }
        IAudioVolumeDuckNotification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnVolumeDuckNotification: OnVolumeDuckNotification::<Identity, Impl, OFFSET>,
            OnVolumeUnduckNotification: OnVolumeUnduckNotification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioVolumeLevel_Impl: ::windows_core::BaseImpl + IPerChannelDbLevel_Impl {}
impl ::windows_core::Iids for IAudioVolumeLevel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPerChannelDbLevel);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioVolumeLevel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioVolumeLevel {
    const VTABLE: Self::Vtable = { IAudioVolumeLevel_Vtbl { base__: <IPerChannelDbLevel as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IChannelAudioVolume_Impl: ::windows_core::BaseImpl {
    fn GetChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetChannelVolume(this: &Self::This, dwindex: u32, flevel: f32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetChannelVolume(this: &Self::This, dwindex: u32) -> ::windows_core::Result<f32>;
    fn SetAllVolumes(this: &Self::This, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetAllVolumes(this: &Self::This, dwcount: u32, pfvolumes: *mut f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IChannelAudioVolume {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChannelAudioVolume {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChannelVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelVolume(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn GetChannelVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelVolume(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllVolumes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllVolumes(this, ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn GetAllVolumes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllVolumes(this, ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into())
        }
        IChannelAudioVolume_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, Impl, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, Impl, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, Impl, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IConnector_Impl: ::windows_core::BaseImpl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<ConnectorType>;
    fn GetDataFlow(this: &Self::This) -> ::windows_core::Result<DataFlow>;
    fn ConnectTo(this: &Self::This, pconnectto: ::core::option::Option<&IConnector>) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetConnectedTo(this: &Self::This) -> ::windows_core::Result<IConnector>;
    fn GetConnectorIdConnectedTo(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDeviceIdConnectedTo(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut ConnectorType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataFlow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflow: *mut DataFlow) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataFlow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectto: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectTo(this, ::windows_core::from_raw_borrowed(&pconnectto)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconto: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectedTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconto, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwstrconnectorid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectorIdConnectedTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrconnectorid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceIdConnectedTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrdeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IConnector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDataFlow: GetDataFlow::<Identity, Impl, OFFSET>,
            ConnectTo: ConnectTo::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectedTo: GetConnectedTo::<Identity, Impl, OFFSET>,
            GetConnectorIdConnectedTo: GetConnectorIdConnectedTo::<Identity, Impl, OFFSET>,
            GetDeviceIdConnectedTo: GetDeviceIdConnectedTo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IControlChangeNotify_Impl: ::windows_core::BaseImpl {
    fn OnNotify(this: &Self::This, dwsenderprocessid: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IControlChangeNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChangeNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IControlChangeNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChangeNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNotify(this, ::core::mem::transmute_copy(&dwsenderprocessid), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        IControlChangeNotify_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IControlInterface_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetIID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IControlInterface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlInterface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IControlInterface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IControlInterface_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetIID: GetIID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDeviceSpecificProperty_Impl: ::windows_core::BaseImpl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetValue(this: &Self::This, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Get4BRange(this: &Self::This, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDeviceSpecificProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeviceSpecificProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvtype: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValue(this, ::core::mem::transmute_copy(&pvvalue), ::core::mem::transmute_copy(&pcbvalue)).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&pvvalue), ::core::mem::transmute_copy(&cbvalue), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn Get4BRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get4BRange(this, ::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plstepping)).into())
        }
        IDeviceSpecificProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Get4BRange: Get4BRange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDeviceTopology_Impl: ::windows_core::BaseImpl {
    fn GetConnectorCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetConnector(this: &Self::This, nindex: u32) -> ::windows_core::Result<IConnector>;
    fn GetSubunitCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSubunit(this: &Self::This, nindex: u32) -> ::windows_core::Result<ISubunit>;
    fn GetPartById(this: &Self::This, nid: u32) -> ::windows_core::Result<IPart>;
    fn GetDeviceId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSignalPath(this: &Self::This, pipartfrom: ::core::option::Option<&IPart>, pipartto: ::core::option::Option<&IPart>, brejectmixedpaths: super::super::Foundation::BOOL) -> ::windows_core::Result<IPartsList>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDeviceTopology {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeviceTopology {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnectorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectorCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnector(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubunitCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubunitCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubunit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, ppsubunit: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubunit(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubunit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nid: u32, pppart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartById(this, ::core::mem::transmute_copy(&nid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrdeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignalPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipartfrom: *mut ::core::ffi::c_void, pipartto: *mut ::core::ffi::c_void, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignalPath(this, ::windows_core::from_raw_borrowed(&pipartfrom), ::windows_core::from_raw_borrowed(&pipartto), ::core::mem::transmute_copy(&brejectmixedpaths)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDeviceTopology_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnectorCount: GetConnectorCount::<Identity, Impl, OFFSET>,
            GetConnector: GetConnector::<Identity, Impl, OFFSET>,
            GetSubunitCount: GetSubunitCount::<Identity, Impl, OFFSET>,
            GetSubunit: GetSubunit::<Identity, Impl, OFFSET>,
            GetPartById: GetPartById::<Identity, Impl, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            GetSignalPath: GetSignalPath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMMDevice_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This, iid: *const ::windows_core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn OpenPropertyStore(this: &Self::This, stgmaccess: super::super::System::Com::STGM) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetState(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IMMDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMMDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute_copy(&pactivationparams), ::core::mem::transmute_copy(&ppinterface)).into())
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stgmaccess: super::super::System::Com::STGM, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenPropertyStore(this, ::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstrid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMMDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Activate: Activate::<Identity, Impl, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IMMDeviceActivator_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This, iid: *const ::windows_core::GUID, pdevice: ::core::option::Option<&IMMDevice>, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMMDeviceActivator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceActivator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMMDeviceActivator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceActivator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, pdevice: *mut ::core::ffi::c_void, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&iid), ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pactivationparams), ::core::mem::transmute_copy(&ppinterface)).into())
        }
        IMMDeviceActivator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Activate: Activate::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMMDeviceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Item(this: &Self::This, ndevice: u32) -> ::windows_core::Result<IMMDevice>;
}
impl ::windows_core::Iids for IMMDeviceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMMDeviceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdevices: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcdevices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ndevice: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&ndevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMMDeviceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMMDeviceEnumerator_Impl: ::windows_core::BaseImpl {
    fn EnumAudioEndpoints(this: &Self::This, dataflow: EDataFlow, dwstatemask: u32) -> ::windows_core::Result<IMMDeviceCollection>;
    fn GetDefaultAudioEndpoint(this: &Self::This, dataflow: EDataFlow, role: ERole) -> ::windows_core::Result<IMMDevice>;
    fn GetDevice(this: &Self::This, pwstrid: &::windows_core::PCWSTR) -> ::windows_core::Result<IMMDevice>;
    fn RegisterEndpointNotificationCallback(this: &Self::This, pclient: ::core::option::Option<&IMMNotificationClient>) -> ::windows_core::Result<()>;
    fn UnregisterEndpointNotificationCallback(this: &Self::This, pclient: ::core::option::Option<&IMMNotificationClient>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMMDeviceEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMMDeviceEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumAudioEndpoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumAudioEndpoints(this, ::core::mem::transmute_copy(&dataflow), ::core::mem::transmute_copy(&dwstatemask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultAudioEndpoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultAudioEndpoint(this, ::core::mem::transmute_copy(&dataflow), ::core::mem::transmute_copy(&role)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppendpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrid: ::windows_core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this, ::core::mem::transmute(&pwstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterEndpointNotificationCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclient: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterEndpointNotificationCallback(this, ::windows_core::from_raw_borrowed(&pclient)).into())
        }
        unsafe extern "system" fn UnregisterEndpointNotificationCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclient: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterEndpointNotificationCallback(this, ::windows_core::from_raw_borrowed(&pclient)).into())
        }
        IMMDeviceEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumAudioEndpoints: EnumAudioEndpoints::<Identity, Impl, OFFSET>,
            GetDefaultAudioEndpoint: GetDefaultAudioEndpoint::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            RegisterEndpointNotificationCallback: RegisterEndpointNotificationCallback::<Identity, Impl, OFFSET>,
            UnregisterEndpointNotificationCallback: UnregisterEndpointNotificationCallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMMEndpoint_Impl: ::windows_core::BaseImpl {
    fn GetDataFlow(this: &Self::This) -> ::windows_core::Result<EDataFlow>;
}
impl ::windows_core::Iids for IMMEndpoint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMEndpoint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMMEndpoint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDataFlow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataflow: *mut EDataFlow) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataFlow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdataflow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMMEndpoint_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDataFlow: GetDataFlow::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IMMNotificationClient_Impl: ::windows_core::BaseImpl {
    fn OnDeviceStateChanged(this: &Self::This, pwstrdeviceid: &::windows_core::PCWSTR, dwnewstate: u32) -> ::windows_core::Result<()>;
    fn OnDeviceAdded(this: &Self::This, pwstrdeviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnDeviceRemoved(this: &Self::This, pwstrdeviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnDefaultDeviceChanged(this: &Self::This, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnPropertyValueChanged(this: &Self::This, pwstrdeviceid: &::windows_core::PCWSTR, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IMMNotificationClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMMNotificationClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDeviceStateChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows_core::PCWSTR, dwnewstate: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDeviceStateChanged(this, ::core::mem::transmute(&pwstrdeviceid), ::core::mem::transmute_copy(&dwnewstate)).into())
        }
        unsafe extern "system" fn OnDeviceAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDeviceAdded(this, ::core::mem::transmute(&pwstrdeviceid)).into())
        }
        unsafe extern "system" fn OnDeviceRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDeviceRemoved(this, ::core::mem::transmute(&pwstrdeviceid)).into())
        }
        unsafe extern "system" fn OnDefaultDeviceChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDefaultDeviceChanged(this, ::core::mem::transmute_copy(&flow), ::core::mem::transmute_copy(&role), ::core::mem::transmute(&pwstrdefaultdeviceid)).into())
        }
        unsafe extern "system" fn OnPropertyValueChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows_core::PCWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPropertyValueChanged(this, ::core::mem::transmute(&pwstrdeviceid), ::core::mem::transmute(&key)).into())
        }
        IMMNotificationClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDeviceStateChanged: OnDeviceStateChanged::<Identity, Impl, OFFSET>,
            OnDeviceAdded: OnDeviceAdded::<Identity, Impl, OFFSET>,
            OnDeviceRemoved: OnDeviceRemoved::<Identity, Impl, OFFSET>,
            OnDefaultDeviceChanged: OnDefaultDeviceChanged::<Identity, Impl, OFFSET>,
            OnPropertyValueChanged: OnPropertyValueChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageFilter_Impl: ::windows_core::BaseImpl {
    fn HandleInComingCall(this: &Self::This, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32;
    fn RetryRejectedCall(this: &Self::This, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32;
    fn MessagePending(this: &Self::This, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMessageFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMessageFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleInComingCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleInComingCall(this, ::core::mem::transmute_copy(&dwcalltype), ::core::mem::transmute_copy(&htaskcaller), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&lpinterfaceinfo)))
        }
        unsafe extern "system" fn RetryRejectedCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RetryRejectedCall(this, ::core::mem::transmute_copy(&htaskcallee), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&dwrejecttype)))
        }
        unsafe extern "system" fn MessagePending<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MessagePending(this, ::core::mem::transmute_copy(&htaskcallee), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&dwpendingtype)))
        }
        IMessageFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleInComingCall: HandleInComingCall::<Identity, Impl, OFFSET>,
            RetryRejectedCall: RetryRejectedCall::<Identity, Impl, OFFSET>,
            MessagePending: MessagePending::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPart_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetLocalId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetGlobalId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPartType(this: &Self::This) -> ::windows_core::Result<PartType>;
    fn GetSubType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetControlInterfaceCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetControlInterface(this: &Self::This, nindex: u32) -> ::windows_core::Result<IControlInterface>;
    fn EnumPartsIncoming(this: &Self::This) -> ::windows_core::Result<IPartsList>;
    fn EnumPartsOutgoing(this: &Self::This) -> ::windows_core::Result<IPartsList>;
    fn GetTopologyObject(this: &Self::This) -> ::windows_core::Result<IDeviceTopology>;
    fn Activate(this: &Self::This, dwclscontext: u32, refiid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RegisterControlChangeCallback(this: &Self::This, riid: *const ::windows_core::GUID, pnotify: ::core::option::Option<&IControlChangeNotify>) -> ::windows_core::Result<()>;
    fn UnregisterControlChangeCallback(this: &Self::This, pnotify: ::core::option::Option<&IControlChangeNotify>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPart {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPart {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlobalId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwstrglobalid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlobalId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrglobalid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparttype: *mut PartType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubtype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psubtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetControlInterfaceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetControlInterfaceCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetControlInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetControlInterface(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinterfacedesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumPartsIncoming<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumPartsIncoming(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumPartsOutgoing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumPartsOutgoing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTopologyObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptopology: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTopologyObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptopology, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, refiid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        unsafe extern "system" fn RegisterControlChangeCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pnotify: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterControlChangeCallback(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&pnotify)).into())
        }
        unsafe extern "system" fn UnregisterControlChangeCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterControlChangeCallback(this, ::windows_core::from_raw_borrowed(&pnotify)).into())
        }
        IPart_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetLocalId: GetLocalId::<Identity, Impl, OFFSET>,
            GetGlobalId: GetGlobalId::<Identity, Impl, OFFSET>,
            GetPartType: GetPartType::<Identity, Impl, OFFSET>,
            GetSubType: GetSubType::<Identity, Impl, OFFSET>,
            GetControlInterfaceCount: GetControlInterfaceCount::<Identity, Impl, OFFSET>,
            GetControlInterface: GetControlInterface::<Identity, Impl, OFFSET>,
            EnumPartsIncoming: EnumPartsIncoming::<Identity, Impl, OFFSET>,
            EnumPartsOutgoing: EnumPartsOutgoing::<Identity, Impl, OFFSET>,
            GetTopologyObject: GetTopologyObject::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            RegisterControlChangeCallback: RegisterControlChangeCallback::<Identity, Impl, OFFSET>,
            UnregisterControlChangeCallback: UnregisterControlChangeCallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPartsList_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPart(this: &Self::This, nindex: u32) -> ::windows_core::Result<IPart>;
}
impl ::windows_core::Iids for IPartsList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartsList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartsList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartsList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartsList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, pppart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPart(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPartsList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetPart: GetPart::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPerChannelDbLevel_Impl: ::windows_core::BaseImpl {
    fn GetChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLevelRange(this: &Self::This, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows_core::Result<()>;
    fn GetLevel(this: &Self::This, nchannel: u32) -> ::windows_core::Result<f32>;
    fn SetLevel(this: &Self::This, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetLevelUniform(this: &Self::This, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetLevelAllChannels(this: &Self::This, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPerChannelDbLevel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPerChannelDbLevel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchannels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLevelRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLevelRange(this, ::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&pfminleveldb), ::core::mem::transmute_copy(&pfmaxleveldb), ::core::mem::transmute_copy(&pfstepping)).into())
        }
        unsafe extern "system" fn GetLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLevel(this, ::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfleveldb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLevel(this, ::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn SetLevelUniform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLevelUniform(this, ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn SetLevelAllChannels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLevelAllChannels(this, ::core::mem::transmute_copy(&alevelsdb), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        IPerChannelDbLevel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            GetLevelRange: GetLevelRange::<Identity, Impl, OFFSET>,
            GetLevel: GetLevel::<Identity, Impl, OFFSET>,
            SetLevel: SetLevel::<Identity, Impl, OFFSET>,
            SetLevelUniform: SetLevelUniform::<Identity, Impl, OFFSET>,
            SetLevelAllChannels: SetLevelAllChannels::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleAudioVolume_Impl: ::windows_core::BaseImpl {
    fn SetMasterVolume(this: &Self::This, flevel: f32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetMasterVolume(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetMute(this: &Self::This, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetMute(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISimpleAudioVolume {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimpleAudioVolume {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMasterVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flevel: f32, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMasterVolume(this, ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn GetMasterVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMasterVolume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMute(this, ::core::mem::transmute_copy(&bmute), ::core::mem::transmute_copy(&eventcontext)).into())
        }
        unsafe extern "system" fn GetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISimpleAudioVolume_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMasterVolume: SetMasterVolume::<Identity, Impl, OFFSET>,
            GetMasterVolume: GetMasterVolume::<Identity, Impl, OFFSET>,
            SetMute: SetMute::<Identity, Impl, OFFSET>,
            GetMute: GetMute::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISpatialAudioClient_Impl: ::windows_core::BaseImpl {
    fn GetStaticObjectPosition(this: &Self::This, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows_core::Result<()>;
    fn GetNativeStaticObjectTypeMask(this: &Self::This) -> ::windows_core::Result<AudioObjectType>;
    fn GetMaxDynamicObjectCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSupportedAudioObjectFormatEnumerator(this: &Self::This) -> ::windows_core::Result<IAudioFormatEnumerator>;
    fn GetMaxFrameCount(this: &Self::This, objectformat: *const WAVEFORMATEX) -> ::windows_core::Result<u32>;
    fn IsAudioObjectFormatSupported(this: &Self::This, objectformat: *const WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn IsSpatialAudioStreamAvailable(this: &Self::This, streamuuid: *const ::windows_core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn ActivateSpatialAudioStream(this: &Self::This, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpatialAudioClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStaticObjectPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStaticObjectPosition(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into())
        }
        unsafe extern "system" fn GetNativeStaticObjectTypeMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: *mut AudioObjectType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNativeStaticObjectTypeMask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxDynamicObjectCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxDynamicObjectCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedAudioObjectFormatEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedAudioObjectFormatEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxFrameCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxFrameCount(this, ::core::mem::transmute_copy(&objectformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(framecountperbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAudioObjectFormatSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsAudioObjectFormatSupported(this, ::core::mem::transmute_copy(&objectformat)).into())
        }
        unsafe extern "system" fn IsSpatialAudioStreamAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamuuid: *const ::windows_core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSpatialAudioStreamAvailable(this, ::core::mem::transmute_copy(&streamuuid), ::core::mem::transmute_copy(&auxiliaryinfo)).into())
        }
        unsafe extern "system" fn ActivateSpatialAudioStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows_core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateSpatialAudioStream(this, ::core::mem::transmute_copy(&activationparams), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&stream)).into())
        }
        ISpatialAudioClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStaticObjectPosition: GetStaticObjectPosition::<Identity, Impl, OFFSET>,
            GetNativeStaticObjectTypeMask: GetNativeStaticObjectTypeMask::<Identity, Impl, OFFSET>,
            GetMaxDynamicObjectCount: GetMaxDynamicObjectCount::<Identity, Impl, OFFSET>,
            GetSupportedAudioObjectFormatEnumerator: GetSupportedAudioObjectFormatEnumerator::<Identity, Impl, OFFSET>,
            GetMaxFrameCount: GetMaxFrameCount::<Identity, Impl, OFFSET>,
            IsAudioObjectFormatSupported: IsAudioObjectFormatSupported::<Identity, Impl, OFFSET>,
            IsSpatialAudioStreamAvailable: IsSpatialAudioStreamAvailable::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioStream: ActivateSpatialAudioStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISpatialAudioClient2_Impl: ::windows_core::BaseImpl + ISpatialAudioClient_Impl {
    fn IsOffloadCapable(this: &Self::This, category: AUDIO_STREAM_CATEGORY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetMaxFrameCountForCategory(this: &Self::This, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISpatialAudioClient2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioClient);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioClient2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsOffloadCapable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOffloadCapable(this, ::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isoffloadcapable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxFrameCountForCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxFrameCountForCategory(this, ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&offloadenabled), ::core::mem::transmute_copy(&objectformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(framecountperbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioClient2_Vtbl {
            base__: <ISpatialAudioClient as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsOffloadCapable: IsOffloadCapable::<Identity, Impl, OFFSET>,
            GetMaxFrameCountForCategory: GetMaxFrameCountForCategory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioMetadataClient_Impl: ::windows_core::BaseImpl {
    fn ActivateSpatialAudioMetadataItems(this: &Self::This, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::core::option::Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut ::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows_core::Result<()>;
    fn GetSpatialAudioMetadataItemsBufferLength(this: &Self::This, maxitemcount: u16) -> ::windows_core::Result<u32>;
    fn ActivateSpatialAudioMetadataWriter(this: &Self::This, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> ::windows_core::Result<ISpatialAudioMetadataWriter>;
    fn ActivateSpatialAudioMetadataCopier(this: &Self::This) -> ::windows_core::Result<ISpatialAudioMetadataCopier>;
    fn ActivateSpatialAudioMetadataReader(this: &Self::This) -> ::windows_core::Result<ISpatialAudioMetadataReader>;
}
impl ::windows_core::Iids for ISpatialAudioMetadataClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioMetadataClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateSpatialAudioMetadataItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut *mut ::core::ffi::c_void, metadataitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateSpatialAudioMetadataItems(this, ::core::mem::transmute_copy(&maxitemcount), ::core::mem::transmute_copy(&framecount), ::core::mem::transmute_copy(&metadataitemsbuffer), ::core::mem::transmute_copy(&metadataitems)).into())
        }
        unsafe extern "system" fn GetSpatialAudioMetadataItemsBufferLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpatialAudioMetadataItemsBufferLength(this, ::core::mem::transmute_copy(&maxitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateSpatialAudioMetadataWriter(this, ::core::mem::transmute_copy(&overflowmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadatawriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataCopier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadatacopier: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateSpatialAudioMetadataCopier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadatacopier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadatareader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateSpatialAudioMetadataReader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadatareader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioMetadataClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateSpatialAudioMetadataItems: ActivateSpatialAudioMetadataItems::<Identity, Impl, OFFSET>,
            GetSpatialAudioMetadataItemsBufferLength: GetSpatialAudioMetadataItemsBufferLength::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioMetadataWriter: ActivateSpatialAudioMetadataWriter::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioMetadataCopier: ActivateSpatialAudioMetadataCopier::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioMetadataReader: ActivateSpatialAudioMetadataReader::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioMetadataCopier_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, metadataitems: ::core::option::Option<&ISpatialAudioMetadataItems>) -> ::windows_core::Result<()>;
    fn CopyMetadataForFrames(this: &Self::This, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: ::core::option::Option<&ISpatialAudioMetadataItems>) -> ::windows_core::Result<u16>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpatialAudioMetadataCopier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioMetadataCopier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&metadataitems)).into())
        }
        unsafe extern "system" fn CopyMetadataForFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: *mut ::core::ffi::c_void, itemscopied: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyMetadataForFrames(this, ::core::mem::transmute_copy(&copyframecount), ::core::mem::transmute_copy(&copymode), ::windows_core::from_raw_borrowed(&dstmetadataitems)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemscopied, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ISpatialAudioMetadataCopier_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            CopyMetadataForFrames: CopyMetadataForFrames::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioMetadataItems_Impl: ::windows_core::BaseImpl {
    fn GetFrameCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetItemCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetMaxItemCount(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetMaxValueBufferLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetInfo(this: &Self::This) -> ::windows_core::Result<SpatialAudioMetadataItemsInfo>;
}
impl ::windows_core::Iids for ISpatialAudioMetadataItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioMetadataItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFrameCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framecount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(framecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemcount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxitemcount: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxitemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxValueBufferLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxvaluebufferlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxValueBufferLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxvaluebufferlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(info, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioMetadataItems_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFrameCount: GetFrameCount::<Identity, Impl, OFFSET>,
            GetItemCount: GetItemCount::<Identity, Impl, OFFSET>,
            GetMaxItemCount: GetMaxItemCount::<Identity, Impl, OFFSET>,
            GetMaxValueBufferLength: GetMaxValueBufferLength::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioMetadataItemsBuffer_Impl: ::windows_core::BaseImpl {
    fn AttachToBuffer(this: &Self::This, buffer: *mut u8, bufferlength: u32) -> ::windows_core::Result<()>;
    fn AttachToPopulatedBuffer(this: &Self::This, buffer: *mut u8, bufferlength: u32) -> ::windows_core::Result<()>;
    fn DetachBuffer(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpatialAudioMetadataItemsBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioMetadataItemsBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AttachToBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachToBuffer(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into())
        }
        unsafe extern "system" fn AttachToPopulatedBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachToPopulatedBuffer(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into())
        }
        unsafe extern "system" fn DetachBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetachBuffer(this).into())
        }
        ISpatialAudioMetadataItemsBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AttachToBuffer: AttachToBuffer::<Identity, Impl, OFFSET>,
            AttachToPopulatedBuffer: AttachToPopulatedBuffer::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioMetadataReader_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, metadataitems: ::core::option::Option<&ISpatialAudioMetadataItems>) -> ::windows_core::Result<()>;
    fn ReadNextItem(this: &Self::This, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows_core::Result<()>;
    fn ReadNextItemCommand(this: &Self::This, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpatialAudioMetadataReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioMetadataReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&metadataitems)).into())
        }
        unsafe extern "system" fn ReadNextItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadNextItem(this, ::core::mem::transmute_copy(&commandcount), ::core::mem::transmute_copy(&frameoffset)).into())
        }
        unsafe extern "system" fn ReadNextItemCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadNextItemCommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&maxvaluebufferlength), ::core::mem::transmute_copy(&valuebufferlength)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ISpatialAudioMetadataReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            ReadNextItem: ReadNextItem::<Identity, Impl, OFFSET>,
            ReadNextItemCommand: ReadNextItemCommand::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioMetadataWriter_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, metadataitems: ::core::option::Option<&ISpatialAudioMetadataItems>) -> ::windows_core::Result<()>;
    fn WriteNextItem(this: &Self::This, frameoffset: u16) -> ::windows_core::Result<()>;
    fn WriteNextItemCommand(this: &Self::This, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpatialAudioMetadataWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioMetadataWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&metadataitems)).into())
        }
        unsafe extern "system" fn WriteNextItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameoffset: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNextItem(this, ::core::mem::transmute_copy(&frameoffset)).into())
        }
        unsafe extern "system" fn WriteNextItemCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNextItemCommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&valuebufferlength)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ISpatialAudioMetadataWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            WriteNextItem: WriteNextItem::<Identity, Impl, OFFSET>,
            WriteNextItemCommand: WriteNextItemCommand::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObject_Impl: ::windows_core::BaseImpl + ISpatialAudioObjectBase_Impl {
    fn SetPosition(this: &Self::This, x: f32, y: f32, z: f32) -> ::windows_core::Result<()>;
    fn SetVolume(this: &Self::This, volume: f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpatialAudioObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioObjectBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPosition(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into())
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, volume: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolume(this, ::core::mem::transmute_copy(&volume)).into())
        }
        ISpatialAudioObject_Vtbl {
            base__: <ISpatialAudioObjectBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectBase_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows_core::Result<()>;
    fn SetEndOfStream(this: &Self::This, framecount: u32) -> ::windows_core::Result<()>;
    fn IsActive(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetAudioObjectType(this: &Self::This) -> ::windows_core::Result<AudioObjectType>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpatialAudioObjectBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into())
        }
        unsafe extern "system" fn SetEndOfStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEndOfStream(this, ::core::mem::transmute_copy(&framecount)).into())
        }
        unsafe extern "system" fn IsActive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsActive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isactive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAudioObjectType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudioObjectType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobjecttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioObjectBase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            SetEndOfStream: SetEndOfStream::<Identity, Impl, OFFSET>,
            IsActive: IsActive::<Identity, Impl, OFFSET>,
            GetAudioObjectType: GetAudioObjectType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForHrtf_Impl: ::windows_core::BaseImpl + ISpatialAudioObjectBase_Impl {
    fn SetPosition(this: &Self::This, x: f32, y: f32, z: f32) -> ::windows_core::Result<()>;
    fn SetGain(this: &Self::This, gain: f32) -> ::windows_core::Result<()>;
    fn SetOrientation(this: &Self::This, orientation: *const *const f32) -> ::windows_core::Result<()>;
    fn SetEnvironment(this: &Self::This, environment: SpatialAudioHrtfEnvironmentType) -> ::windows_core::Result<()>;
    fn SetDistanceDecay(this: &Self::This, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows_core::Result<()>;
    fn SetDirectivity(this: &Self::This, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpatialAudioObjectForHrtf {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioObjectBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectForHrtf {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPosition(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into())
        }
        unsafe extern "system" fn SetGain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGain(this, ::core::mem::transmute_copy(&gain)).into())
        }
        unsafe extern "system" fn SetOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, orientation: *const *const f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOrientation(this, ::core::mem::transmute_copy(&orientation)).into())
        }
        unsafe extern "system" fn SetEnvironment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnvironment(this, ::core::mem::transmute_copy(&environment)).into())
        }
        unsafe extern "system" fn SetDistanceDecay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDistanceDecay(this, ::core::mem::transmute_copy(&distancedecay)).into())
        }
        unsafe extern "system" fn SetDirectivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDirectivity(this, ::core::mem::transmute_copy(&directivity)).into())
        }
        ISpatialAudioObjectForHrtf_Vtbl {
            base__: <ISpatialAudioObjectBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetGain: SetGain::<Identity, Impl, OFFSET>,
            SetOrientation: SetOrientation::<Identity, Impl, OFFSET>,
            SetEnvironment: SetEnvironment::<Identity, Impl, OFFSET>,
            SetDistanceDecay: SetDistanceDecay::<Identity, Impl, OFFSET>,
            SetDirectivity: SetDirectivity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForMetadataCommands_Impl: ::windows_core::BaseImpl + ISpatialAudioObjectBase_Impl {
    fn WriteNextMetadataCommand(this: &Self::This, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpatialAudioObjectForMetadataCommands {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioObjectBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectForMetadataCommands {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteNextMetadataCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNextMetadataCommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&valuebufferlength)).into())
        }
        ISpatialAudioObjectForMetadataCommands_Vtbl {
            base__: <ISpatialAudioObjectBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteNextMetadataCommand: WriteNextMetadataCommand::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForMetadataItems_Impl: ::windows_core::BaseImpl + ISpatialAudioObjectBase_Impl {
    fn GetSpatialAudioMetadataItems(this: &Self::This) -> ::windows_core::Result<ISpatialAudioMetadataItems>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpatialAudioObjectForMetadataItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioObjectBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectForMetadataItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSpatialAudioMetadataItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadataitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpatialAudioMetadataItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadataitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioObjectForMetadataItems_Vtbl {
            base__: <ISpatialAudioObjectBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSpatialAudioMetadataItems: GetSpatialAudioMetadataItems::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioObjectRenderStream_Impl: ::windows_core::BaseImpl + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObject(this: &Self::This, r#type: AudioObjectType) -> ::windows_core::Result<ISpatialAudioObject>;
}
impl ::windows_core::Iids for ISpatialAudioObjectRenderStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioObjectRenderStreamBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectRenderStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateSpatialAudioObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateSpatialAudioObject(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioObjectRenderStream_Vtbl {
            base__: <ISpatialAudioObjectRenderStreamBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateSpatialAudioObject: ActivateSpatialAudioObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioObjectRenderStreamBase_Impl: ::windows_core::BaseImpl {
    fn GetAvailableDynamicObjectCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetService(this: &Self::This, riid: *const ::windows_core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn BeginUpdatingAudioObjects(this: &Self::This, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows_core::Result<()>;
    fn EndUpdatingAudioObjects(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpatialAudioObjectRenderStreamBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectRenderStreamBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAvailableDynamicObjectCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableDynamicObjectCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetService(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&service)).into())
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn BeginUpdatingAudioObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUpdatingAudioObjects(this, ::core::mem::transmute_copy(&availabledynamicobjectcount), ::core::mem::transmute_copy(&framecountperbuffer)).into())
        }
        unsafe extern "system" fn EndUpdatingAudioObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndUpdatingAudioObjects(this).into())
        }
        ISpatialAudioObjectRenderStreamBase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAvailableDynamicObjectCount: GetAvailableDynamicObjectCount::<Identity, Impl, OFFSET>,
            GetService: GetService::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            BeginUpdatingAudioObjects: BeginUpdatingAudioObjects::<Identity, Impl, OFFSET>,
            EndUpdatingAudioObjects: EndUpdatingAudioObjects::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioObjectRenderStreamForHrtf_Impl: ::windows_core::BaseImpl + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForHrtf(this: &Self::This, r#type: AudioObjectType) -> ::windows_core::Result<ISpatialAudioObjectForHrtf>;
}
impl ::windows_core::Iids for ISpatialAudioObjectRenderStreamForHrtf {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioObjectRenderStreamBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForHrtf_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectRenderStreamForHrtf {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateSpatialAudioObjectForHrtf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForHrtf_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateSpatialAudioObjectForHrtf(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
            base__: <ISpatialAudioObjectRenderStreamBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateSpatialAudioObjectForHrtf: ActivateSpatialAudioObjectForHrtf::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioObjectRenderStreamForMetadata_Impl: ::windows_core::BaseImpl + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForMetadataCommands(this: &Self::This, r#type: AudioObjectType) -> ::windows_core::Result<ISpatialAudioObjectForMetadataCommands>;
    fn ActivateSpatialAudioObjectForMetadataItems(this: &Self::This, r#type: AudioObjectType) -> ::windows_core::Result<ISpatialAudioObjectForMetadataItems>;
}
impl ::windows_core::Iids for ISpatialAudioObjectRenderStreamForMetadata {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpatialAudioObjectRenderStreamBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectRenderStreamForMetadata {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateSpatialAudioObjectForMetadataCommands(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateSpatialAudioObjectForMetadataItems(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
            base__: <ISpatialAudioObjectRenderStreamBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateSpatialAudioObjectForMetadataCommands: ActivateSpatialAudioObjectForMetadataCommands::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioObjectForMetadataItems: ActivateSpatialAudioObjectForMetadataItems::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpatialAudioObjectRenderStreamNotify_Impl: ::windows_core::BaseImpl {
    fn OnAvailableDynamicObjectCountChange(this: &Self::This, sender: ::core::option::Option<&ISpatialAudioObjectRenderStreamBase>, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpatialAudioObjectRenderStreamNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialAudioObjectRenderStreamNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnAvailableDynamicObjectCountChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAvailableDynamicObjectCountChange(this, ::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&hnscompliancedeadlinetime), ::core::mem::transmute_copy(&availabledynamicobjectcountchange)).into())
        }
        ISpatialAudioObjectRenderStreamNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnAvailableDynamicObjectCountChange: OnAvailableDynamicObjectCountChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISubunit_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ISubunit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubunit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISubunit {
    const VTABLE: Self::Vtable = { ISubunit_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
