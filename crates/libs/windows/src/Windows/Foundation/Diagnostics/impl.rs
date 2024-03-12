pub trait IErrorReportingSettings_Impl: ::windows_core::BaseImpl {
    fn SetErrorOptions(this: &Self::This, value: ErrorOptions) -> ::windows_core::Result<()>;
    fn GetErrorOptions(this: &Self::This) -> ::windows_core::Result<ErrorOptions>;
}
impl ::windows_core::Iids for IErrorReportingSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IErrorReportingSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetErrorOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorOptions(this, value).into())
        }
        unsafe extern "system" fn GetErrorOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IErrorReportingSettings_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetErrorOptions: SetErrorOptions::<Identity, Impl, OFFSET>,
            GetErrorOptions: GetErrorOptions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Storage\"`"]
#[cfg(feature = "Storage")]
pub trait IFileLoggingSession_Impl: ::windows_core::BaseImpl + super::IClosable_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AddLoggingChannel(this: &Self::This, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
    fn AddLoggingChannelWithLevel(this: &Self::This, loggingchannel: ::core::option::Option<&ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows_core::Result<()>;
    fn RemoveLoggingChannel(this: &Self::This, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
    fn CloseAndSaveToFileAsync(this: &Self::This) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(this: &Self::This, handler: ::core::option::Option<&super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> ::windows_core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(this: &Self::This, token: &super::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows_core::Iids for IFileLoggingSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Storage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFileLoggingSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLoggingChannel(this, ::windows_core::from_raw_borrowed(&loggingchannel)).into())
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLoggingChannelWithLevel(this, ::windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into())
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLoggingChannel(this, ::windows_core::from_raw_borrowed(&loggingchannel)).into())
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CloseAndSaveToFileAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LogFileGenerated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LogFileGenerated(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveLogFileGenerated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLogFileGenerated(this, ::core::mem::transmute(&token)).into())
        }
        IFileLoggingSession_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
            CloseAndSaveToFileAsync: CloseAndSaveToFileAsync::<Identity, Impl, OFFSET>,
            LogFileGenerated: LogFileGenerated::<Identity, Impl, OFFSET>,
            RemoveLogFileGenerated: RemoveLogFileGenerated::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILoggingChannel_Impl: ::windows_core::BaseImpl + super::IClosable_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<bool>;
    fn Level(this: &Self::This) -> ::windows_core::Result<LoggingLevel>;
    fn LogMessage(this: &Self::This, eventstring: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LogMessageWithLevel(this: &Self::This, eventstring: &::windows_core::HSTRING, level: LoggingLevel) -> ::windows_core::Result<()>;
    fn LogValuePair(this: &Self::This, value1: &::windows_core::HSTRING, value2: i32) -> ::windows_core::Result<()>;
    fn LogValuePairWithLevel(this: &Self::This, value1: &::windows_core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows_core::Result<()>;
    fn LoggingEnabled(this: &Self::This, handler: ::core::option::Option<&super::TypedEventHandler<ILoggingChannel, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(this: &Self::This, token: &super::EventRegistrationToken) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILoggingChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IClosable as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILoggingChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Level<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Level(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LogMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogMessage(this, ::core::mem::transmute(&eventstring)).into())
        }
        unsafe extern "system" fn LogMessageWithLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>, level: LoggingLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogMessageWithLevel(this, ::core::mem::transmute(&eventstring), level).into())
        }
        unsafe extern "system" fn LogValuePair<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value2: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogValuePair(this, ::core::mem::transmute(&value1), value2).into())
        }
        unsafe extern "system" fn LogValuePairWithLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogValuePairWithLevel(this, ::core::mem::transmute(&value1), value2, level).into())
        }
        unsafe extern "system" fn LoggingEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoggingEnabled(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveLoggingEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLoggingEnabled(this, ::core::mem::transmute(&token)).into())
        }
        ILoggingChannel_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            LogMessage: LogMessage::<Identity, Impl, OFFSET>,
            LogMessageWithLevel: LogMessageWithLevel::<Identity, Impl, OFFSET>,
            LogValuePair: LogValuePair::<Identity, Impl, OFFSET>,
            LogValuePairWithLevel: LogValuePairWithLevel::<Identity, Impl, OFFSET>,
            LoggingEnabled: LoggingEnabled::<Identity, Impl, OFFSET>,
            RemoveLoggingEnabled: RemoveLoggingEnabled::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Storage\"`"]
#[cfg(feature = "Storage")]
pub trait ILoggingSession_Impl: ::windows_core::BaseImpl + super::IClosable_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SaveToFileAsync(this: &Self::This, folder: ::core::option::Option<&super::super::Storage::IStorageFolder>, filename: &::windows_core::HSTRING) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(this: &Self::This, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
    fn AddLoggingChannelWithLevel(this: &Self::This, loggingchannel: ::core::option::Option<&ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows_core::Result<()>;
    fn RemoveLoggingChannel(this: &Self::This, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows_core::Iids for ILoggingSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Storage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILoggingSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveToFileAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveToFileAsync(this, ::windows_core::from_raw_borrowed(&folder), ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLoggingChannel(this, ::windows_core::from_raw_borrowed(&loggingchannel)).into())
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLoggingChannelWithLevel(this, ::windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into())
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLoggingChannel(this, ::windows_core::from_raw_borrowed(&loggingchannel)).into())
        }
        ILoggingSession_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SaveToFileAsync: SaveToFileAsync::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILoggingTarget_Impl: ::windows_core::BaseImpl {
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsEnabledWithLevel(this: &Self::This, level: LoggingLevel) -> ::windows_core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(this: &Self::This, level: LoggingLevel, keywords: i64) -> ::windows_core::Result<bool>;
    fn LogEvent(this: &Self::This, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LogEventWithFields(this: &Self::This, eventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>) -> ::windows_core::Result<()>;
    fn LogEventWithFieldsAndLevel(this: &Self::This, eventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel) -> ::windows_core::Result<()>;
    fn LogEventWithFieldsAndOptions(this: &Self::This, eventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel, options: ::core::option::Option<&LoggingOptions>) -> ::windows_core::Result<()>;
    fn StartActivity(this: &Self::This, starteventname: &::windows_core::HSTRING) -> ::windows_core::Result<LoggingActivity>;
    fn StartActivityWithFields(this: &Self::This, starteventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>) -> ::windows_core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(this: &Self::This, starteventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel) -> ::windows_core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(this: &Self::This, starteventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel, options: ::core::option::Option<&LoggingOptions>) -> ::windows_core::Result<LoggingActivity>;
}
impl ::windows_core::Iids for ILoggingTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILoggingTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEnabledWithLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabledWithLevel(this, level) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabledWithLevelAndKeywords(this, level, keywords) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LogEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogEvent(this, ::core::mem::transmute(&eventname)).into())
        }
        unsafe extern "system" fn LogEventWithFields<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogEventWithFields(this, ::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&fields)).into())
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogEventWithFieldsAndLevel(this, ::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&fields), level).into())
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogEventWithFieldsAndOptions(this, ::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&fields), level, ::windows_core::from_raw_borrowed(&options)).into())
        }
        unsafe extern "system" fn StartActivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartActivity(this, ::core::mem::transmute(&starteventname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartActivityWithFields<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartActivityWithFields(this, ::core::mem::transmute(&starteventname), ::windows_core::from_raw_borrowed(&fields)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartActivityWithFieldsAndLevel(this, ::core::mem::transmute(&starteventname), ::windows_core::from_raw_borrowed(&fields), level) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartActivityWithFieldsAndOptions(this, ::core::mem::transmute(&starteventname), ::windows_core::from_raw_borrowed(&fields), level, ::windows_core::from_raw_borrowed(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILoggingTarget_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            IsEnabledWithLevel: IsEnabledWithLevel::<Identity, Impl, OFFSET>,
            IsEnabledWithLevelAndKeywords: IsEnabledWithLevelAndKeywords::<Identity, Impl, OFFSET>,
            LogEvent: LogEvent::<Identity, Impl, OFFSET>,
            LogEventWithFields: LogEventWithFields::<Identity, Impl, OFFSET>,
            LogEventWithFieldsAndLevel: LogEventWithFieldsAndLevel::<Identity, Impl, OFFSET>,
            LogEventWithFieldsAndOptions: LogEventWithFieldsAndOptions::<Identity, Impl, OFFSET>,
            StartActivity: StartActivity::<Identity, Impl, OFFSET>,
            StartActivityWithFields: StartActivityWithFields::<Identity, Impl, OFFSET>,
            StartActivityWithFieldsAndLevel: StartActivityWithFieldsAndLevel::<Identity, Impl, OFFSET>,
            StartActivityWithFieldsAndOptions: StartActivityWithFieldsAndOptions::<Identity, Impl, OFFSET>,
        }
    };
}
