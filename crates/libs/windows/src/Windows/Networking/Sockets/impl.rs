pub trait IControlChannelTriggerEventDetails_Impl: ::windows_core::BaseImpl {
    fn ControlChannelTrigger(this: &Self::This) -> ::windows_core::Result<ControlChannelTrigger>;
}
impl ::windows_core::Iids for IControlChannelTriggerEventDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChannelTriggerEventDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IControlChannelTriggerEventDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ControlChannelTrigger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChannelTriggerEventDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControlChannelTrigger(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IControlChannelTriggerEventDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ControlChannelTrigger: ControlChannelTrigger::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IControlChannelTriggerResetEventDetails_Impl: ::windows_core::BaseImpl {
    fn ResetReason(this: &Self::This) -> ::windows_core::Result<ControlChannelTriggerResetReason>;
    fn HardwareSlotReset(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SoftwareSlotReset(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IControlChannelTriggerResetEventDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IControlChannelTriggerResetEventDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResetReason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerResetReason) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResetReason(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HardwareSlotReset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HardwareSlotReset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SoftwareSlotReset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SoftwareSlotReset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IControlChannelTriggerResetEventDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResetReason: ResetReason::<Identity, Impl, OFFSET>,
            HardwareSlotReset: HardwareSlotReset::<Identity, Impl, OFFSET>,
            SoftwareSlotReset: SoftwareSlotReset::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IWebSocket_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl {
    fn OutputStream(this: &Self::This) -> ::windows_core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(this: &Self::This, uri: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRequestHeader(this: &Self::This, headername: &::windows_core::HSTRING, headervalue: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Closed(this: &Self::This, eventhandler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(this: &Self::This, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn CloseWithStatus(this: &Self::This, code: u16, reason: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IWebSocket {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocket_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebSocket {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OutputStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutputStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectAsync(this, ::windows_core::from_raw_borrowed(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, headername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, headervalue: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestHeader(this, ::core::mem::transmute(&headername), ::core::mem::transmute(&headervalue)).into())
        }
        unsafe extern "system" fn Closed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Closed(this, ::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveClosed(this, ::core::mem::transmute(&eventcookie)).into())
        }
        unsafe extern "system" fn CloseWithStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseWithStatus(this, code, ::core::mem::transmute(&reason)).into())
        }
        IWebSocket_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OutputStream: OutputStream::<Identity, Impl, OFFSET>,
            ConnectAsync: ConnectAsync::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
            CloseWithStatus: CloseWithStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
pub trait IWebSocketControl_Impl: ::windows_core::BaseImpl {
    fn OutboundBufferSizeInBytes(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn ServerCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(this: &Self::This, value: ::core::option::Option<&super::super::Security::Credentials::PasswordCredential>) -> ::windows_core::Result<()>;
    fn ProxyCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(this: &Self::This, value: ::core::option::Option<&super::super::Security::Credentials::PasswordCredential>) -> ::windows_core::Result<()>;
    fn SupportedProtocols(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl ::windows_core::Iids for IWebSocketControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebSocketControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OutboundBufferSizeInBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OutboundBufferSizeInBytes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutboundBufferSizeInBytes(this, value).into())
        }
        unsafe extern "system" fn ServerCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerCredential(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn ProxyCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxyCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxyCredential(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn SupportedProtocols<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedProtocols(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebSocketControl_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OutboundBufferSizeInBytes: OutboundBufferSizeInBytes::<Identity, Impl, OFFSET>,
            SetOutboundBufferSizeInBytes: SetOutboundBufferSizeInBytes::<Identity, Impl, OFFSET>,
            ServerCredential: ServerCredential::<Identity, Impl, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, Impl, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, Impl, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, Impl, OFFSET>,
            SupportedProtocols: SupportedProtocols::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"Security_Cryptography_Certificates\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketControl2_Impl: ::windows_core::BaseImpl + IWebSocketControl_Impl {
    fn IgnorableServerCertificateErrors(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows_core::Iids for IWebSocketControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IWebSocketControl as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebSocketControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IgnorableServerCertificateErrors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IgnorableServerCertificateErrors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebSocketControl2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IgnorableServerCertificateErrors: IgnorableServerCertificateErrors::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWebSocketInformation_Impl: ::windows_core::BaseImpl {
    fn LocalAddress(this: &Self::This) -> ::windows_core::Result<super::HostName>;
    fn BandwidthStatistics(this: &Self::This) -> ::windows_core::Result<BandwidthStatistics>;
    fn Protocol(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IWebSocketInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebSocketInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LocalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BandwidthStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BandwidthStatistics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebSocketInformation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LocalAddress: LocalAddress::<Identity, Impl, OFFSET>,
            BandwidthStatistics: BandwidthStatistics::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketInformation2_Impl: ::windows_core::BaseImpl + IWebSocketInformation_Impl {
    fn ServerCertificate(this: &Self::This) -> ::windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(this: &Self::This) -> ::windows_core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl ::windows_core::Iids for IWebSocketInformation2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IWebSocketInformation as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebSocketInformation2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServerCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerCertificateErrorSeverity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerCertificateErrorSeverity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerCertificateErrors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerCertificateErrors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebSocketInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerIntermediateCertificates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebSocketInformation2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServerCertificate: ServerCertificate::<Identity, Impl, OFFSET>,
            ServerCertificateErrorSeverity: ServerCertificateErrorSeverity::<Identity, Impl, OFFSET>,
            ServerCertificateErrors: ServerCertificateErrors::<Identity, Impl, OFFSET>,
            ServerIntermediateCertificates: ServerIntermediateCertificates::<Identity, Impl, OFFSET>,
        }
    };
}
