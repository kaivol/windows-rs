pub trait IAudioEndpointFormatControl_Impl: ::windows_core::BaseImpl {
    fn ResetToDefault(this: &Self::This, resetflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioEndpointFormatControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointFormatControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointFormatControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResetToDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointFormatControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resetflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetToDefault(this, ::core::mem::transmute_copy(&resetflags)).into())
        }
        IAudioEndpointFormatControl_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ResetToDefault: ResetToDefault::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_Apo\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
pub trait IAudioEndpointLastBufferControl_Impl: ::windows_core::BaseImpl {
    fn IsLastBufferControlSupported(this: &Self::This) -> super::super::super::Foundation::BOOL;
    fn ReleaseOutputDataPointerForLastBuffer(this: &Self::This, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
impl ::windows_core::Iids for IAudioEndpointLastBufferControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointLastBufferControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsLastBufferControlSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsLastBufferControlSupported(this))
        }
        unsafe extern "system" fn ReleaseOutputDataPointerForLastBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseOutputDataPointerForLastBuffer(this, ::core::mem::transmute_copy(&pconnectionproperty)))
        }
        IAudioEndpointLastBufferControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsLastBufferControlSupported: IsLastBufferControlSupported::<Identity, Impl, OFFSET>,
            ReleaseOutputDataPointerForLastBuffer: ReleaseOutputDataPointerForLastBuffer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioEndpointOffloadStreamMeter_Impl: ::windows_core::BaseImpl {
    fn GetMeterChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMeteringData(this: &Self::This, u32channelcount: u32) -> ::windows_core::Result<f32>;
}
impl ::windows_core::Iids for IAudioEndpointOffloadStreamMeter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointOffloadStreamMeter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMeterChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMeterChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu32channelcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMeteringData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMeteringData(this, ::core::mem::transmute_copy(&u32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pf32peakvalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioEndpointOffloadStreamMeter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMeterChannelCount: GetMeterChannelCount::<Identity, Impl, OFFSET>,
            GetMeteringData: GetMeteringData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioEndpointOffloadStreamMute_Impl: ::windows_core::BaseImpl {
    fn SetMute(this: &Self::This, bmuted: u8) -> ::windows_core::Result<()>;
    fn GetMute(this: &Self::This) -> ::windows_core::Result<u8>;
}
impl ::windows_core::Iids for IAudioEndpointOffloadStreamMute {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointOffloadStreamMute {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmuted: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMute(this, ::core::mem::transmute_copy(&bmuted)).into())
        }
        unsafe extern "system" fn GetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmuted: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmuted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioEndpointOffloadStreamMute_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMute: SetMute::<Identity, Impl, OFFSET>,
            GetMute: GetMute::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Media_KernelStreaming\"`"]
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub trait IAudioEndpointOffloadStreamVolume_Impl: ::windows_core::BaseImpl {
    fn GetVolumeChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetChannelVolumes(this: &Self::This, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows_core::Result<()>;
    fn GetChannelVolumes(this: &Self::This, u32channelcount: u32) -> ::windows_core::Result<f32>;
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl ::windows_core::Iids for IAudioEndpointOffloadStreamVolume {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointOffloadStreamVolume {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVolumeChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVolumeChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu32channelcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChannelVolumes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelVolumes(this, ::core::mem::transmute_copy(&u32channelcount), ::core::mem::transmute_copy(&pf32volumes), ::core::mem::transmute_copy(&u32curvetype), ::core::mem::transmute_copy(&pcurveduration)).into())
        }
        unsafe extern "system" fn GetChannelVolumes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelVolumes(this, ::core::mem::transmute_copy(&u32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pf32volumes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioEndpointOffloadStreamVolume_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVolumeChannelCount: GetVolumeChannelCount::<Identity, Impl, OFFSET>,
            SetChannelVolumes: SetChannelVolumes::<Identity, Impl, OFFSET>,
            GetChannelVolumes: GetChannelVolumes::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolume_Impl: ::windows_core::BaseImpl {
    fn RegisterControlChangeNotify(this: &Self::This, pnotify: ::core::option::Option<&IAudioEndpointVolumeCallback>) -> ::windows_core::Result<()>;
    fn UnregisterControlChangeNotify(this: &Self::This, pnotify: ::core::option::Option<&IAudioEndpointVolumeCallback>) -> ::windows_core::Result<()>;
    fn GetChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMasterVolumeLevel(this: &Self::This, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetMasterVolumeLevelScalar(this: &Self::This, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetMasterVolumeLevel(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetMasterVolumeLevelScalar(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetChannelVolumeLevel(this: &Self::This, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetChannelVolumeLevelScalar(this: &Self::This, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetChannelVolumeLevel(this: &Self::This, nchannel: u32) -> ::windows_core::Result<f32>;
    fn GetChannelVolumeLevelScalar(this: &Self::This, nchannel: u32) -> ::windows_core::Result<f32>;
    fn SetMute(this: &Self::This, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetMute(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetVolumeStepInfo(this: &Self::This, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows_core::Result<()>;
    fn VolumeStepUp(this: &Self::This, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn VolumeStepDown(this: &Self::This, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn QueryHardwareSupport(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetVolumeRange(this: &Self::This, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioEndpointVolume {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointVolume {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterControlChangeNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterControlChangeNotify(this, ::windows_core::from_raw_borrowed(&pnotify)).into())
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterControlChangeNotify(this, ::windows_core::from_raw_borrowed(&pnotify)).into())
        }
        unsafe extern "system" fn GetChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnchannelcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMasterVolumeLevel(this, ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMasterVolumeLevelScalar(this, ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfleveldb: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMasterVolumeLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfleveldb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMasterVolumeLevelScalar(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelVolumeLevel(this, ::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelVolumeLevelScalar(this, ::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelVolumeLevel(this, ::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfleveldb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelVolumeLevelScalar(this, ::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMute(this, ::core::mem::transmute_copy(&bmute), ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn GetMute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVolumeStepInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVolumeStepInfo(this, ::core::mem::transmute_copy(&pnstep), ::core::mem::transmute_copy(&pnstepcount)).into())
        }
        unsafe extern "system" fn VolumeStepUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VolumeStepUp(this, ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn VolumeStepDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VolumeStepDown(this, ::core::mem::transmute_copy(&pguideventcontext)).into())
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryHardwareSupport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhardwaresupportmask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVolumeRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVolumeRange(this, ::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)).into())
        }
        IAudioEndpointVolume_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterControlChangeNotify: RegisterControlChangeNotify::<Identity, Impl, OFFSET>,
            UnregisterControlChangeNotify: UnregisterControlChangeNotify::<Identity, Impl, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            SetMasterVolumeLevel: SetMasterVolumeLevel::<Identity, Impl, OFFSET>,
            SetMasterVolumeLevelScalar: SetMasterVolumeLevelScalar::<Identity, Impl, OFFSET>,
            GetMasterVolumeLevel: GetMasterVolumeLevel::<Identity, Impl, OFFSET>,
            GetMasterVolumeLevelScalar: GetMasterVolumeLevelScalar::<Identity, Impl, OFFSET>,
            SetChannelVolumeLevel: SetChannelVolumeLevel::<Identity, Impl, OFFSET>,
            SetChannelVolumeLevelScalar: SetChannelVolumeLevelScalar::<Identity, Impl, OFFSET>,
            GetChannelVolumeLevel: GetChannelVolumeLevel::<Identity, Impl, OFFSET>,
            GetChannelVolumeLevelScalar: GetChannelVolumeLevelScalar::<Identity, Impl, OFFSET>,
            SetMute: SetMute::<Identity, Impl, OFFSET>,
            GetMute: GetMute::<Identity, Impl, OFFSET>,
            GetVolumeStepInfo: GetVolumeStepInfo::<Identity, Impl, OFFSET>,
            VolumeStepUp: VolumeStepUp::<Identity, Impl, OFFSET>,
            VolumeStepDown: VolumeStepDown::<Identity, Impl, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, Impl, OFFSET>,
            GetVolumeRange: GetVolumeRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeCallback_Impl: ::windows_core::BaseImpl {
    fn OnNotify(this: &Self::This, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioEndpointVolumeCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolumeCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointVolumeCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolumeCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNotify(this, ::core::mem::transmute_copy(&pnotify)).into())
        }
        IAudioEndpointVolumeCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeEx_Impl: ::windows_core::BaseImpl + IAudioEndpointVolume_Impl {
    fn GetVolumeRangeChannel(this: &Self::This, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioEndpointVolumeEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioEndpointVolume);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolumeEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointVolumeEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVolumeRangeChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointVolumeEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVolumeRangeChannel(this, ::core::mem::transmute_copy(&ichannel), ::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)).into())
        }
        IAudioEndpointVolumeEx_Vtbl {
            base__: <IAudioEndpointVolume as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVolumeRangeChannel: GetVolumeRangeChannel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioLfxControl_Impl: ::windows_core::BaseImpl {
    fn SetLocalEffectsState(this: &Self::This, benabled: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLocalEffectsState(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioLfxControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioLfxControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioLfxControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLocalEffectsState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioLfxControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalEffectsState(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn GetLocalEffectsState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioLfxControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalEffectsState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioLfxControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLocalEffectsState: SetLocalEffectsState::<Identity, Impl, OFFSET>,
            GetLocalEffectsState: GetLocalEffectsState::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioMeterInformation_Impl: ::windows_core::BaseImpl {
    fn GetPeakValue(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetMeteringChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetChannelsPeakValues(this: &Self::This, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows_core::Result<()>;
    fn QueryHardwareSupport(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAudioMeterInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioMeterInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPeakValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfpeak: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPeakValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfpeak, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMeteringChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMeteringChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnchannelcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChannelsPeakValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChannelsPeakValues(this, ::core::mem::transmute_copy(&u32channelcount), ::core::mem::transmute_copy(&afpeakvalues)).into())
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryHardwareSupport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhardwaresupportmask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioMeterInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPeakValue: GetPeakValue::<Identity, Impl, OFFSET>,
            GetMeteringChannelCount: GetMeteringChannelCount::<Identity, Impl, OFFSET>,
            GetChannelsPeakValues: GetChannelsPeakValues::<Identity, Impl, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHardwareAudioEngineBase_Impl: ::windows_core::BaseImpl {
    fn GetAvailableOffloadConnectorCount(this: &Self::This, _pwstrdeviceid: &::windows_core::PCWSTR, _uconnectorid: u32) -> ::windows_core::Result<u32>;
    fn GetEngineFormat(this: &Self::This, pdevice: ::core::option::Option<&super::IMMDevice>, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn SetEngineDeviceFormat(this: &Self::This, pdevice: ::core::option::Option<&super::IMMDevice>, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn SetGfxState(this: &Self::This, pdevice: ::core::option::Option<&super::IMMDevice>, _benable: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetGfxState(this: &Self::This, pdevice: ::core::option::Option<&super::IMMDevice>) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHardwareAudioEngineBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHardwareAudioEngineBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAvailableOffloadConnectorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, _pwstrdeviceid: ::windows_core::PCWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableOffloadConnectorCount(this, ::core::mem::transmute(&_pwstrdeviceid), ::core::mem::transmute_copy(&_uconnectorid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_pavailableconnectorinstancecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEngineFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEngineFormat(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&_brequestdeviceformat), ::core::mem::transmute_copy(&_ppwfxformat)).into())
        }
        unsafe extern "system" fn SetEngineDeviceFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEngineDeviceFormat(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&_pwfxformat)).into())
        }
        unsafe extern "system" fn SetGfxState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _benable: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGfxState(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&_benable)).into())
        }
        unsafe extern "system" fn GetGfxState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGfxState(this, ::windows_core::from_raw_borrowed(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_pbenable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHardwareAudioEngineBase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAvailableOffloadConnectorCount: GetAvailableOffloadConnectorCount::<Identity, Impl, OFFSET>,
            GetEngineFormat: GetEngineFormat::<Identity, Impl, OFFSET>,
            SetEngineDeviceFormat: SetEngineDeviceFormat::<Identity, Impl, OFFSET>,
            SetGfxState: SetGfxState::<Identity, Impl, OFFSET>,
            GetGfxState: GetGfxState::<Identity, Impl, OFFSET>,
        }
    };
}
