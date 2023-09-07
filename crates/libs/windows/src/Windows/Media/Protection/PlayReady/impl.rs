#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDClosedCaptionDataReceivedEventArgs_Impl: ::windows_core::BaseImpl {
    fn ClosedCaptionDataFormat(this: &Self::This) -> ::windows_core::Result<NDClosedCaptionFormat>;
    fn PresentationTimestamp(this: &Self::This) -> ::windows_core::Result<i64>;
    fn ClosedCaptionData(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDClosedCaptionDataReceivedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDClosedCaptionDataReceivedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClosedCaptionDataFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClosedCaptionDataFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PresentationTimestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PresentationTimestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClosedCaptionData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClosedCaptionData(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDClosedCaptionDataReceivedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClosedCaptionDataFormat: ClosedCaptionDataFormat::<Identity, Impl, OFFSET>,
            PresentationTimestamp: PresentationTimestamp::<Identity, Impl, OFFSET>,
            ClosedCaptionData: ClosedCaptionData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDCustomData_Impl: ::windows_core::BaseImpl {
    fn CustomDataTypeID(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
    fn CustomData(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDCustomData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDCustomData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDCustomData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CustomDataTypeID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDCustomData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CustomDataTypeID(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDCustomData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CustomData(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDCustomData_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CustomDataTypeID: CustomDataTypeID::<Identity, Impl, OFFSET>,
            CustomData: CustomData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDDownloadEngine_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, uri: ::core::option::Option<&super::super::super::Foundation::Uri>, sessionidbytes: &[u8]) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, startposition: &super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()>;
    fn CanSeek(this: &Self::This) -> ::windows_core::Result<bool>;
    fn BufferFullMinThresholdInSamples(this: &Self::This) -> ::windows_core::Result<u32>;
    fn BufferFullMaxThresholdInSamples(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Notifier(this: &Self::This) -> ::windows_core::Result<NDDownloadEngineNotifier>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows_core::Iids for INDDownloadEngine {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDDownloadEngine {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&uri), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute(&startposition)).into())
        }
        unsafe extern "system" fn CanSeek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanSeek(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BufferFullMinThresholdInSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferFullMinThresholdInSamples(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BufferFullMaxThresholdInSamples<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferFullMaxThresholdInSamples(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Notifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Notifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDDownloadEngine_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            CanSeek: CanSeek::<Identity, Impl, OFFSET>,
            BufferFullMinThresholdInSamples: BufferFullMinThresholdInSamples::<Identity, Impl, OFFSET>,
            BufferFullMaxThresholdInSamples: BufferFullMaxThresholdInSamples::<Identity, Impl, OFFSET>,
            Notifier: Notifier::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineNotifier_Impl: ::windows_core::BaseImpl {
    fn OnStreamOpened(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnPlayReadyObjectReceived(this: &Self::This, databytes: &[u8]) -> ::windows_core::Result<()>;
    fn OnContentIDReceived(this: &Self::This, licensefetchdescriptor: ::core::option::Option<&INDLicenseFetchDescriptor>) -> ::windows_core::Result<()>;
    fn OnDataReceived(this: &Self::This, databytes: &[u8], bytesreceived: u32) -> ::windows_core::Result<()>;
    fn OnEndOfStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnNetworkError(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDDownloadEngineNotifier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDDownloadEngineNotifier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStreamOpened<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStreamOpened(this).into())
        }
        unsafe extern "system" fn OnPlayReadyObjectReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPlayReadyObjectReceived(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as usize)).into())
        }
        unsafe extern "system" fn OnContentIDReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnContentIDReceived(this, ::windows_core::from_raw_borrowed(&licensefetchdescriptor)).into())
        }
        unsafe extern "system" fn OnDataReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataReceived(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as usize), bytesreceived).into())
        }
        unsafe extern "system" fn OnEndOfStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndOfStream(this).into())
        }
        unsafe extern "system" fn OnNetworkError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNetworkError(this).into())
        }
        INDDownloadEngineNotifier_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStreamOpened: OnStreamOpened::<Identity, Impl, OFFSET>,
            OnPlayReadyObjectReceived: OnPlayReadyObjectReceived::<Identity, Impl, OFFSET>,
            OnContentIDReceived: OnContentIDReceived::<Identity, Impl, OFFSET>,
            OnDataReceived: OnDataReceived::<Identity, Impl, OFFSET>,
            OnEndOfStream: OnEndOfStream::<Identity, Impl, OFFSET>,
            OnNetworkError: OnNetworkError::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchCompletedEventArgs_Impl: ::windows_core::BaseImpl {
    fn ResponseCustomData(this: &Self::This) -> ::windows_core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDLicenseFetchCompletedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDLicenseFetchCompletedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseCustomData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDLicenseFetchCompletedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchDescriptor_Impl: ::windows_core::BaseImpl {
    fn ContentIDType(this: &Self::This) -> ::windows_core::Result<NDContentIDType>;
    fn ContentID(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
    fn LicenseFetchChallengeCustomData(this: &Self::This) -> ::windows_core::Result<INDCustomData>;
    fn SetLicenseFetchChallengeCustomData(this: &Self::This, licensefetchchallengecustomdata: ::core::option::Option<&INDCustomData>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDLicenseFetchDescriptor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDLicenseFetchDescriptor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContentIDType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NDContentIDType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentIDType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContentID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentID(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LicenseFetchChallengeCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LicenseFetchChallengeCustomData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLicenseFetchChallengeCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, licensefetchchallengecustomdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLicenseFetchChallengeCustomData(this, ::windows_core::from_raw_borrowed(&licensefetchchallengecustomdata)).into())
        }
        INDLicenseFetchDescriptor_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContentIDType: ContentIDType::<Identity, Impl, OFFSET>,
            ContentID: ContentID::<Identity, Impl, OFFSET>,
            LicenseFetchChallengeCustomData: LicenseFetchChallengeCustomData::<Identity, Impl, OFFSET>,
            SetLicenseFetchChallengeCustomData: SetLicenseFetchChallengeCustomData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchResult_Impl: ::windows_core::BaseImpl {
    fn ResponseCustomData(this: &Self::This) -> ::windows_core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDLicenseFetchResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDLicenseFetchResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDLicenseFetchResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseCustomData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDLicenseFetchResult_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDMessenger_Impl: ::windows_core::BaseImpl {
    fn SendRegistrationRequestAsync(this: &Self::This, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionStartAsync(this: &Self::This, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionResponseAsync(this: &Self::This, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendLicenseFetchRequestAsync(this: &Self::This, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows_core::Iids for INDMessenger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDMessenger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendRegistrationRequestAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendRegistrationRequestAsync(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendProximityDetectionStartAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendProximityDetectionStartAsync(this, pdtype, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as usize), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendProximityDetectionResponseAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendProximityDetectionResponseAsync(this, pdtype, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as usize), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsedatabytes), responseDataBytes_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendLicenseFetchRequestAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendLicenseFetchRequestAsync(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDMessenger_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendRegistrationRequestAsync: SendRegistrationRequestAsync::<Identity, Impl, OFFSET>,
            SendProximityDetectionStartAsync: SendProximityDetectionStartAsync::<Identity, Impl, OFFSET>,
            SendProximityDetectionResponseAsync: SendProximityDetectionResponseAsync::<Identity, Impl, OFFSET>,
            SendLicenseFetchRequestAsync: SendLicenseFetchRequestAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDProximityDetectionCompletedEventArgs_Impl: ::windows_core::BaseImpl {
    fn ProximityDetectionRetryCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDProximityDetectionCompletedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDProximityDetectionCompletedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProximityDetectionRetryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProximityDetectionRetryCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDProximityDetectionCompletedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProximityDetectionRetryCount: ProximityDetectionRetryCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDRegistrationCompletedEventArgs_Impl: ::windows_core::BaseImpl {
    fn ResponseCustomData(this: &Self::This) -> ::windows_core::Result<INDCustomData>;
    fn TransmitterProperties(this: &Self::This) -> ::windows_core::Result<INDTransmitterProperties>;
    fn TransmitterCertificateAccepted(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetTransmitterCertificateAccepted(this: &Self::This, accept: bool) -> ::windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDRegistrationCompletedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDRegistrationCompletedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseCustomData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmitterProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmitterProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmitterCertificateAccepted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmitterCertificateAccepted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransmitterCertificateAccepted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accept: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransmitterCertificateAccepted(this, accept).into())
        }
        INDRegistrationCompletedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
            TransmitterProperties: TransmitterProperties::<Identity, Impl, OFFSET>,
            TransmitterCertificateAccepted: TransmitterCertificateAccepted::<Identity, Impl, OFFSET>,
            SetTransmitterCertificateAccepted: SetTransmitterCertificateAccepted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait INDSendResult_Impl: ::windows_core::BaseImpl {
    fn Response(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for INDSendResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDSendResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDSendResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Response<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDSendResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Response(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDSendResult_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Response: Response::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStartResult_Impl: ::windows_core::BaseImpl {
    fn MediaStreamSource(this: &Self::This) -> ::windows_core::Result<super::super::Core::MediaStreamSource>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl ::windows_core::Iids for INDStartResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStartResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDStartResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MediaStreamSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStartResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaStreamSource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDStartResult_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MediaStreamSource: MediaStreamSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait INDStorageFileHelper_Impl: ::windows_core::BaseImpl {
    fn GetFileURLs(this: &Self::This, file: ::core::option::Option<&super::super::super::Storage::IStorageFile>) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl ::windows_core::Iids for INDStorageFileHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStorageFileHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDStorageFileHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFileURLs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStorageFileHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileURLs(this, ::windows_core::from_raw_borrowed(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDStorageFileHelper_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetFileURLs: GetFileURLs::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParser_Impl: ::windows_core::BaseImpl {
    fn ParseData(this: &Self::This, databytes: &[u8]) -> ::windows_core::Result<()>;
    fn GetStreamInformation(this: &Self::This, descriptor: ::core::option::Option<&super::super::Core::IMediaStreamDescriptor>, streamtype: &mut NDMediaStreamType) -> ::windows_core::Result<u32>;
    fn BeginOfStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndOfStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn Notifier(this: &Self::This) -> ::windows_core::Result<NDStreamParserNotifier>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl ::windows_core::Iids for INDStreamParser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDStreamParser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseData(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&databytes), dataBytes_array_size as usize)).into())
        }
        unsafe extern "system" fn GetStreamInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamInformation(this, ::windows_core::from_raw_borrowed(&descriptor), ::core::mem::transmute_copy(&streamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginOfStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginOfStream(this).into())
        }
        unsafe extern "system" fn EndOfStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndOfStream(this).into())
        }
        unsafe extern "system" fn Notifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Notifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDStreamParser_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseData: ParseData::<Identity, Impl, OFFSET>,
            GetStreamInformation: GetStreamInformation::<Identity, Impl, OFFSET>,
            BeginOfStream: BeginOfStream::<Identity, Impl, OFFSET>,
            EndOfStream: EndOfStream::<Identity, Impl, OFFSET>,
            Notifier: Notifier::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParserNotifier_Impl: ::windows_core::BaseImpl {
    fn OnContentIDReceived(this: &Self::This, licensefetchdescriptor: ::core::option::Option<&INDLicenseFetchDescriptor>) -> ::windows_core::Result<()>;
    fn OnMediaStreamDescriptorCreated(this: &Self::This, audiostreamdescriptors: ::core::option::Option<&super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, videostreamdescriptors: ::core::option::Option<&super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>) -> ::windows_core::Result<()>;
    fn OnSampleParsed(this: &Self::This, streamid: u32, streamtype: NDMediaStreamType, streamsample: ::core::option::Option<&super::super::Core::MediaStreamSample>, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows_core::Result<()>;
    fn OnBeginSetupDecryptor(this: &Self::This, descriptor: ::core::option::Option<&super::super::Core::IMediaStreamDescriptor>, keyid: &::windows_core::GUID, probytes: &[u8]) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl ::windows_core::Iids for INDStreamParserNotifier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDStreamParserNotifier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnContentIDReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnContentIDReceived(this, ::windows_core::from_raw_borrowed(&licensefetchdescriptor)).into())
        }
        unsafe extern "system" fn OnMediaStreamDescriptorCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiostreamdescriptors: *mut ::core::ffi::c_void, videostreamdescriptors: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMediaStreamDescriptorCreated(this, ::windows_core::from_raw_borrowed(&audiostreamdescriptors), ::windows_core::from_raw_borrowed(&videostreamdescriptors)).into())
        }
        unsafe extern "system" fn OnSampleParsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: *mut ::core::ffi::c_void, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSampleParsed(this, streamid, streamtype, ::windows_core::from_raw_borrowed(&streamsample), pts, ccformat, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&ccdatabytes), ccDataBytes_array_size as usize)).into())
        }
        unsafe extern "system" fn OnBeginSetupDecryptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, keyid: ::windows_core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBeginSetupDecryptor(this, ::windows_core::from_raw_borrowed(&descriptor), ::core::mem::transmute(&keyid), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&probytes), proBytes_array_size as usize)).into())
        }
        INDStreamParserNotifier_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnContentIDReceived: OnContentIDReceived::<Identity, Impl, OFFSET>,
            OnMediaStreamDescriptorCreated: OnMediaStreamDescriptorCreated::<Identity, Impl, OFFSET>,
            OnSampleParsed: OnSampleParsed::<Identity, Impl, OFFSET>,
            OnBeginSetupDecryptor: OnBeginSetupDecryptor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait INDTransmitterProperties_Impl: ::windows_core::BaseImpl {
    fn CertificateType(this: &Self::This) -> ::windows_core::Result<NDCertificateType>;
    fn PlatformIdentifier(this: &Self::This) -> ::windows_core::Result<NDCertificatePlatformID>;
    fn SupportedFeatures(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<NDCertificateFeature>>;
    fn SecurityLevel(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SecurityVersion(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ExpirationDate(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::DateTime>;
    fn ClientID(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
    fn ModelDigest(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
    fn ModelManufacturerName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ModelName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ModelNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows_core::Iids for INDTransmitterProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INDTransmitterProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CertificateType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificateType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CertificateType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PlatformIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlatformIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedFeatures(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SecurityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecurityLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SecurityVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecurityVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpirationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpirationDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientID(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModelDigest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModelDigest(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModelManufacturerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModelManufacturerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModelName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModelName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModelNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModelNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INDTransmitterProperties_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CertificateType: CertificateType::<Identity, Impl, OFFSET>,
            PlatformIdentifier: PlatformIdentifier::<Identity, Impl, OFFSET>,
            SupportedFeatures: SupportedFeatures::<Identity, Impl, OFFSET>,
            SecurityLevel: SecurityLevel::<Identity, Impl, OFFSET>,
            SecurityVersion: SecurityVersion::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            ClientID: ClientID::<Identity, Impl, OFFSET>,
            ModelDigest: ModelDigest::<Identity, Impl, OFFSET>,
            ModelManufacturerName: ModelManufacturerName::<Identity, Impl, OFFSET>,
            ModelName: ModelName::<Identity, Impl, OFFSET>,
            ModelNumber: ModelNumber::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPlayReadyDomain_Impl: ::windows_core::BaseImpl {
    fn AccountId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn ServiceId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Revision(this: &Self::This) -> ::windows_core::Result<u32>;
    fn FriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn DomainJoinUrl(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Uri>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPlayReadyDomain {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayReadyDomain {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccountId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccountId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Revision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Revision(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainJoinUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainJoinUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPlayReadyDomain_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AccountId: AccountId::<Identity, Impl, OFFSET>,
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            Revision: Revision::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            DomainJoinUrl: DomainJoinUrl::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPlayReadyLicense_Impl: ::windows_core::BaseImpl {
    fn FullyEvaluated(this: &Self::This) -> ::windows_core::Result<bool>;
    fn UsableForPlay(this: &Self::This) -> ::windows_core::Result<bool>;
    fn ExpirationDate(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ExpireAfterFirstPlay(this: &Self::This) -> ::windows_core::Result<u32>;
    fn DomainAccountID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn ChainDepth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetKIDAtChainDepth(this: &Self::This, chaindepth: u32) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPlayReadyLicense {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayReadyLicense {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FullyEvaluated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FullyEvaluated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UsableForPlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UsableForPlay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpirationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpirationDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpireAfterFirstPlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpireAfterFirstPlay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainAccountID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainAccountID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChainDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChainDepth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKIDAtChainDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, chaindepth: u32, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKIDAtChainDepth(this, chaindepth) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPlayReadyLicense_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FullyEvaluated: FullyEvaluated::<Identity, Impl, OFFSET>,
            UsableForPlay: UsableForPlay::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            ExpireAfterFirstPlay: ExpireAfterFirstPlay::<Identity, Impl, OFFSET>,
            DomainAccountID: DomainAccountID::<Identity, Impl, OFFSET>,
            ChainDepth: ChainDepth::<Identity, Impl, OFFSET>,
            GetKIDAtChainDepth: GetKIDAtChainDepth::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPlayReadyLicenseAcquisitionServiceRequest_Impl: ::windows_core::BaseImpl + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn ContentHeader(this: &Self::This) -> ::windows_core::Result<PlayReadyContentHeader>;
    fn SetContentHeader(this: &Self::This, value: ::core::option::Option<&PlayReadyContentHeader>) -> ::windows_core::Result<()>;
    fn DomainServiceId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetDomainServiceId(this: &Self::This, value: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPlayReadyLicenseAcquisitionServiceRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IMediaProtectionServiceRequest as ::windows_core::ComInterface>::IID, <IPlayReadyServiceRequest as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayReadyLicenseAcquisitionServiceRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContentHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentHeader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContentHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContentHeader(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn DomainServiceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainServiceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDomainServiceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDomainServiceId(this, ::core::mem::transmute(&value)).into())
        }
        IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContentHeader: ContentHeader::<Identity, Impl, OFFSET>,
            SetContentHeader: SetContentHeader::<Identity, Impl, OFFSET>,
            DomainServiceId: DomainServiceId::<Identity, Impl, OFFSET>,
            SetDomainServiceId: SetDomainServiceId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPlayReadyLicenseSession_Impl: ::windows_core::BaseImpl {
    fn CreateLAServiceRequest(this: &Self::This) -> ::windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest>;
    fn ConfigureMediaProtectionManager(this: &Self::This, mpm: ::core::option::Option<&super::MediaProtectionManager>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPlayReadyLicenseSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayReadyLicenseSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLAServiceRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLAServiceRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConfigureMediaProtectionManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mpm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureMediaProtectionManager(this, ::windows_core::from_raw_borrowed(&mpm)).into())
        }
        IPlayReadyLicenseSession_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateLAServiceRequest: CreateLAServiceRequest::<Identity, Impl, OFFSET>,
            ConfigureMediaProtectionManager: ConfigureMediaProtectionManager::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPlayReadyLicenseSession2_Impl: ::windows_core::BaseImpl + IPlayReadyLicenseSession_Impl {
    fn CreateLicenseIterable(this: &Self::This, contentheader: ::core::option::Option<&PlayReadyContentHeader>, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IPlayReadyLicenseSession2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPlayReadyLicenseSession as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseSession2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayReadyLicenseSession2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLicenseIterable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyLicenseSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLicenseIterable(this, ::windows_core::from_raw_borrowed(&contentheader), fullyevaluated) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPlayReadyLicenseSession2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateLicenseIterable: CreateLicenseIterable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPlayReadySecureStopServiceRequest_Impl: ::windows_core::BaseImpl + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn SessionID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::DateTime>;
    fn UpdateTime(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::DateTime>;
    fn Stopped(this: &Self::This) -> ::windows_core::Result<bool>;
    fn PublisherCertificate(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPlayReadySecureStopServiceRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IMediaProtectionServiceRequest as ::windows_core::ComInterface>::IID, <IPlayReadyServiceRequest as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayReadySecureStopServiceRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SessionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UpdateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stopped<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Stopped(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PublisherCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PublisherCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPlayReadySecureStopServiceRequest_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SessionID: SessionID::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            UpdateTime: UpdateTime::<Identity, Impl, OFFSET>,
            Stopped: Stopped::<Identity, Impl, OFFSET>,
            PublisherCertificate: PublisherCertificate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPlayReadyServiceRequest_Impl: ::windows_core::BaseImpl + super::IMediaProtectionServiceRequest_Impl {
    fn Uri(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(this: &Self::This, value: ::core::option::Option<&super::super::super::Foundation::Uri>) -> ::windows_core::Result<()>;
    fn ResponseCustomData(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ChallengeCustomData(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetChallengeCustomData(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn BeginServiceRequest(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>;
    fn NextServiceRequest(this: &Self::This) -> ::windows_core::Result<IPlayReadyServiceRequest>;
    fn GenerateManualEnablingChallenge(this: &Self::This) -> ::windows_core::Result<PlayReadySoapMessage>;
    fn ProcessManualEnablingResponse(this: &Self::This, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPlayReadyServiceRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IMediaProtectionServiceRequest as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayReadyServiceRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Uri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Uri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUri(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn ResponseCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseCustomData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChallengeCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChallengeCustomData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChallengeCustomData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChallengeCustomData(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn BeginServiceRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginServiceRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextServiceRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextServiceRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GenerateManualEnablingChallenge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateManualEnablingChallenge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProcessManualEnablingResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProcessManualEnablingResponse(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&responsebytes), responseBytes_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPlayReadyServiceRequest_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Uri: Uri::<Identity, Impl, OFFSET>,
            SetUri: SetUri::<Identity, Impl, OFFSET>,
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
            ChallengeCustomData: ChallengeCustomData::<Identity, Impl, OFFSET>,
            SetChallengeCustomData: SetChallengeCustomData::<Identity, Impl, OFFSET>,
            BeginServiceRequest: BeginServiceRequest::<Identity, Impl, OFFSET>,
            NextServiceRequest: NextServiceRequest::<Identity, Impl, OFFSET>,
            GenerateManualEnablingChallenge: GenerateManualEnablingChallenge::<Identity, Impl, OFFSET>,
            ProcessManualEnablingResponse: ProcessManualEnablingResponse::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
