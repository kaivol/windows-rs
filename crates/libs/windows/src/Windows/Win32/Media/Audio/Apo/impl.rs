pub trait IApoAcousticEchoCancellation_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IApoAcousticEchoCancellation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApoAcousticEchoCancellation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApoAcousticEchoCancellation {
    const VTABLE: Self::Vtable = { IApoAcousticEchoCancellation_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IApoAuxiliaryInputConfiguration_Impl: ::windows_core::BaseImpl {
    fn AddAuxiliaryInput(this: &Self::This, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::Result<()>;
    fn RemoveAuxiliaryInput(this: &Self::This, dwinputid: u32) -> ::windows_core::Result<()>;
    fn IsInputFormatSupported(this: &Self::This, prequestedinputformat: ::core::option::Option<&IAudioMediaType>) -> ::windows_core::Result<IAudioMediaType>;
}
impl ::windows_core::Iids for IApoAuxiliaryInputConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApoAuxiliaryInputConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAuxiliaryInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAuxiliaryInput(this, ::core::mem::transmute_copy(&dwinputid), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pbydata), ::core::mem::transmute_copy(&pinputconnection)).into())
        }
        unsafe extern "system" fn RemoveAuxiliaryInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAuxiliaryInput(this, ::core::mem::transmute_copy(&dwinputid)).into())
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequestedinputformat: *mut ::core::ffi::c_void, ppsupportedinputformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInputFormatSupported(this, ::windows_core::from_raw_borrowed(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsupportedinputformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IApoAuxiliaryInputConfiguration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAuxiliaryInput: AddAuxiliaryInput::<Identity, Impl, OFFSET>,
            RemoveAuxiliaryInput: RemoveAuxiliaryInput::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IApoAuxiliaryInputRT_Impl: ::windows_core::BaseImpl {
    fn AcceptInput(this: &Self::This, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY);
}
impl ::windows_core::Iids for IApoAuxiliaryInputRT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApoAuxiliaryInputRT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApoAuxiliaryInputRT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcceptInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApoAuxiliaryInputRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcceptInput(this, ::core::mem::transmute_copy(&dwinputid), ::core::mem::transmute_copy(&pinputconnection)))
        }
        IApoAuxiliaryInputRT_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AcceptInput: AcceptInput::<Identity, Impl, OFFSET> }
    };
}
pub trait IAudioDeviceModulesClient_Impl: ::windows_core::BaseImpl {
    fn SetAudioDeviceModulesManager(this: &Self::This, paudiodevicemodulesmanager: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioDeviceModulesClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioDeviceModulesClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioDeviceModulesClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAudioDeviceModulesManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioDeviceModulesClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAudioDeviceModulesManager(this, ::windows_core::from_raw_borrowed(&paudiodevicemodulesmanager)).into())
        }
        IAudioDeviceModulesClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAudioDeviceModulesManager: SetAudioDeviceModulesManager::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioMediaType_Impl: ::windows_core::BaseImpl {
    fn IsCompressedFormat(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn IsEqual(this: &Self::This, piaudiotype: ::core::option::Option<&IAudioMediaType>) -> ::windows_core::Result<u32>;
    fn GetAudioFormat(this: &Self::This) -> *mut super::WAVEFORMATEX;
    fn GetUncompressedAudioFormat(this: &Self::This, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioMediaType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioMediaType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsCompressedFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCompressedFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcompressed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piaudiotype: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&piaudiotype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAudioFormat(this))
        }
        unsafe extern "system" fn GetUncompressedAudioFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUncompressedAudioFormat(this, ::core::mem::transmute_copy(&puncompressedaudioformat)).into())
        }
        IAudioMediaType_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsCompressedFormat: IsCompressedFormat::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            GetAudioFormat: GetAudioFormat::<Identity, Impl, OFFSET>,
            GetUncompressedAudioFormat: GetUncompressedAudioFormat::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioProcessingObject_Impl: ::windows_core::BaseImpl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetLatency(this: &Self::This) -> ::windows_core::Result<i64>;
    fn GetRegistrationProperties(this: &Self::This) -> ::windows_core::Result<*mut APO_REG_PROPERTIES>;
    fn Initialize(this: &Self::This, cbdatasize: u32, pbydata: *const u8) -> ::windows_core::Result<()>;
    fn IsInputFormatSupported(this: &Self::This, poppositeformat: ::core::option::Option<&IAudioMediaType>, prequestedinputformat: ::core::option::Option<&IAudioMediaType>) -> ::windows_core::Result<IAudioMediaType>;
    fn IsOutputFormatSupported(this: &Self::This, poppositeformat: ::core::option::Option<&IAudioMediaType>, prequestedoutputformat: ::core::option::Option<&IAudioMediaType>) -> ::windows_core::Result<IAudioMediaType>;
    fn GetInputChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAudioProcessingObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn GetLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLatency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRegistrationProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegistrationProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pbydata)).into())
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poppositeformat: *mut ::core::ffi::c_void, prequestedinputformat: *mut ::core::ffi::c_void, ppsupportedinputformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInputFormatSupported(this, ::windows_core::from_raw_borrowed(&poppositeformat), ::windows_core::from_raw_borrowed(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsupportedinputformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOutputFormatSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poppositeformat: *mut ::core::ffi::c_void, prequestedoutputformat: *mut ::core::ffi::c_void, ppsupportedoutputformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOutputFormatSupported(this, ::windows_core::from_raw_borrowed(&poppositeformat), ::windows_core::from_raw_borrowed(&prequestedoutputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsupportedoutputformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu32channelcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioProcessingObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetLatency: GetLatency::<Identity, Impl, OFFSET>,
            GetRegistrationProperties: GetRegistrationProperties::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
            IsOutputFormatSupported: IsOutputFormatSupported::<Identity, Impl, OFFSET>,
            GetInputChannelCount: GetInputChannelCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioProcessingObjectConfiguration_Impl: ::windows_core::BaseImpl {
    fn LockForProcess(this: &Self::This, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::Result<()>;
    fn UnlockForProcess(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioProcessingObjectConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObjectConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LockForProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockForProcess(this, ::core::mem::transmute_copy(&u32numinputconnections), ::core::mem::transmute_copy(&ppinputconnections), ::core::mem::transmute_copy(&u32numoutputconnections), ::core::mem::transmute_copy(&ppoutputconnections)).into())
        }
        unsafe extern "system" fn UnlockForProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockForProcess(this).into())
        }
        IAudioProcessingObjectConfiguration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LockForProcess: LockForProcess::<Identity, Impl, OFFSET>,
            UnlockForProcess: UnlockForProcess::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioProcessingObjectLoggingService_Impl: ::windows_core::BaseImpl {
    fn ApoLog(this: &Self::This, level: APO_LOG_LEVEL, format: &::windows_core::PCWSTR);
}
impl ::windows_core::Iids for IAudioProcessingObjectLoggingService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectLoggingService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObjectLoggingService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApoLog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectLoggingService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: ::windows_core::PCWSTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApoLog(this, ::core::mem::transmute_copy(&level), ::core::mem::transmute(&format)))
        }
        IAudioProcessingObjectLoggingService_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ApoLog: ApoLog::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAudioProcessingObjectNotifications_Impl: ::windows_core::BaseImpl {
    fn GetApoNotificationRegistrationInfo(this: &Self::This, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows_core::Result<()>;
    fn HandleNotification(this: &Self::This, aponotification: *const APO_NOTIFICATION);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IAudioProcessingObjectNotifications {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObjectNotifications {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApoNotificationRegistrationInfo(this, ::core::mem::transmute_copy(&aponotifications), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn HandleNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleNotification(this, ::core::mem::transmute_copy(&aponotification)))
        }
        IAudioProcessingObjectNotifications_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetApoNotificationRegistrationInfo: GetApoNotificationRegistrationInfo::<Identity, Impl, OFFSET>,
            HandleNotification: HandleNotification::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAudioProcessingObjectNotifications2_Impl: ::windows_core::BaseImpl + IAudioProcessingObjectNotifications_Impl {
    fn GetApoNotificationRegistrationInfo2(this: &Self::This, maxaponotificationtypesupported: APO_NOTIFICATION_TYPE, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IAudioProcessingObjectNotifications2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioProcessingObjectNotifications);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectNotifications2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObjectNotifications2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectNotifications2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxaponotificationtypesupported: APO_NOTIFICATION_TYPE, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApoNotificationRegistrationInfo2(this, ::core::mem::transmute_copy(&maxaponotificationtypesupported), ::core::mem::transmute_copy(&aponotifications), ::core::mem::transmute_copy(&count)).into())
        }
        IAudioProcessingObjectNotifications2_Vtbl {
            base__: <IAudioProcessingObjectNotifications as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetApoNotificationRegistrationInfo2: GetApoNotificationRegistrationInfo2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioProcessingObjectRT_Impl: ::windows_core::BaseImpl {
    fn APOProcess(this: &Self::This, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY);
    fn CalcInputFrames(this: &Self::This, u32outputframecount: u32) -> u32;
    fn CalcOutputFrames(this: &Self::This, u32inputframecount: u32) -> u32;
}
impl ::windows_core::Iids for IAudioProcessingObjectRT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObjectRT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn APOProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::APOProcess(this, ::core::mem::transmute_copy(&u32numinputconnections), ::core::mem::transmute_copy(&ppinputconnections), ::core::mem::transmute_copy(&u32numoutputconnections), ::core::mem::transmute_copy(&ppoutputconnections)))
        }
        unsafe extern "system" fn CalcInputFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CalcInputFrames(this, ::core::mem::transmute_copy(&u32outputframecount)))
        }
        unsafe extern "system" fn CalcOutputFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CalcOutputFrames(this, ::core::mem::transmute_copy(&u32inputframecount)))
        }
        IAudioProcessingObjectRT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            APOProcess: APOProcess::<Identity, Impl, OFFSET>,
            CalcInputFrames: CalcInputFrames::<Identity, Impl, OFFSET>,
            CalcOutputFrames: CalcOutputFrames::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioProcessingObjectRTQueueService_Impl: ::windows_core::BaseImpl {
    fn GetRealTimeWorkQueue(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAudioProcessingObjectRTQueueService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectRTQueueService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObjectRTQueueService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRealTimeWorkQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectRTQueueService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRealTimeWorkQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(workqueueid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioProcessingObjectRTQueueService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRealTimeWorkQueue: GetRealTimeWorkQueue::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioProcessingObjectVBR_Impl: ::windows_core::BaseImpl {
    fn CalcMaxInputFrames(this: &Self::This, u32maxoutputframecount: u32) -> ::windows_core::Result<u32>;
    fn CalcMaxOutputFrames(this: &Self::This, u32maxinputframecount: u32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAudioProcessingObjectVBR {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioProcessingObjectVBR {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CalcMaxInputFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CalcMaxInputFrames(this, ::core::mem::transmute_copy(&u32maxoutputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu32inputframecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CalcMaxOutputFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CalcMaxOutputFrames(this, ::core::mem::transmute_copy(&u32maxinputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu32outputframecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioProcessingObjectVBR_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CalcMaxInputFrames: CalcMaxInputFrames::<Identity, Impl, OFFSET>,
            CalcMaxOutputFrames: CalcMaxOutputFrames::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioSystemEffects_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IAudioSystemEffects {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffects_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSystemEffects {
    const VTABLE: Self::Vtable = { IAudioSystemEffects_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects2_Impl: ::windows_core::BaseImpl + IAudioSystemEffects_Impl {
    fn GetEffectsList(this: &Self::This, ppeffectsids: *mut *mut ::windows_core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioSystemEffects2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioSystemEffects);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffects2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSystemEffects2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEffectsList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows_core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectsList(this, ::core::mem::transmute_copy(&ppeffectsids), ::core::mem::transmute_copy(&pceffects), ::core::mem::transmute_copy(&event)).into())
        }
        IAudioSystemEffects2_Vtbl { base__: <IAudioSystemEffects as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetEffectsList: GetEffectsList::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects3_Impl: ::windows_core::BaseImpl + IAudioSystemEffects2_Impl {
    fn GetControllableSystemEffectsList(this: &Self::This, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetAudioSystemEffectState(this: &Self::This, effectid: &::windows_core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioSystemEffects3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAudioSystemEffects2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffects3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSystemEffects3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetControllableSystemEffectsList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffects3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetControllableSystemEffectsList(this, ::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&numeffects), ::core::mem::transmute_copy(&event)).into())
        }
        unsafe extern "system" fn SetAudioSystemEffectState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffects3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: ::windows_core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAudioSystemEffectState(this, ::core::mem::transmute(&effectid), ::core::mem::transmute_copy(&state)).into())
        }
        IAudioSystemEffects3_Vtbl {
            base__: <IAudioSystemEffects2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetControllableSystemEffectsList: GetControllableSystemEffectsList::<Identity, Impl, OFFSET>,
            SetAudioSystemEffectState: SetAudioSystemEffectState::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioSystemEffectsCustomFormats_Impl: ::windows_core::BaseImpl {
    fn GetFormatCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFormat(this: &Self::This, nformat: u32) -> ::windows_core::Result<IAudioMediaType>;
    fn GetFormatRepresentation(this: &Self::This, nformat: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IAudioSystemEffectsCustomFormats {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSystemEffectsCustomFormats {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFormatCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormat(this, ::core::mem::transmute_copy(&nformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatRepresentation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatRepresentation(this, ::core::mem::transmute_copy(&nformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrformatrep, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAudioSystemEffectsCustomFormats_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFormatCount: GetFormatCount::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetFormatRepresentation: GetFormatRepresentation::<Identity, Impl, OFFSET>,
        }
    };
}
