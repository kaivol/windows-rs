#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub trait ISmsBinaryMessage_Impl: ::windows_core::BaseImpl + ISmsMessage_Impl {
    fn Format(this: &Self::This) -> ::windows_core::Result<SmsDataFormat>;
    fn SetFormat(this: &Self::This, value: SmsDataFormat) -> ::windows_core::Result<()>;
    fn GetData(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<u8>>;
    fn SetData(this: &Self::This, value: &[u8]) -> ::windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::Iids for ISmsBinaryMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<ISmsMessage as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "deprecated")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISmsBinaryMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SmsDataFormat) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Format(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SmsDataFormat) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, value).into())
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetData(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsBinaryMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as usize)).into())
        }
        ISmsBinaryMessage_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait ISmsDevice_Impl: ::windows_core::BaseImpl {
    fn SendMessageAsync(this: &Self::This, message: ::core::option::Option<&ISmsMessage>) -> ::windows_core::Result<SendSmsMessageOperation>;
    fn CalculateLength(this: &Self::This, message: ::core::option::Option<&SmsTextMessage>) -> ::windows_core::Result<SmsEncodedLength>;
    fn AccountPhoneNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn CellularClass(this: &Self::This) -> ::windows_core::Result<CellularClass>;
    fn MessageStore(this: &Self::This) -> ::windows_core::Result<SmsDeviceMessageStore>;
    fn DeviceStatus(this: &Self::This) -> ::windows_core::Result<SmsDeviceStatus>;
    fn SmsMessageReceived(this: &Self::This, eventhandler: ::core::option::Option<&SmsMessageReceivedEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsMessageReceived(this: &Self::This, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn SmsDeviceStatusChanged(this: &Self::This, eventhandler: ::core::option::Option<&SmsDeviceStatusChangedEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSmsDeviceStatusChanged(this: &Self::This, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows_core::Iids for ISmsDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISmsDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendMessageAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendMessageAsync(this, ::windows_core::from_raw_borrowed(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CalculateLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut SmsEncodedLength) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CalculateLength(this, ::windows_core::from_raw_borrowed(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AccountPhoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccountPhoneNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CellularClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CellularClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MessageStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SmsDeviceStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmsMessageReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsMessageReceived(this, ::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSmsMessageReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSmsMessageReceived(this, ::core::mem::transmute(&eventcookie)).into())
        }
        unsafe extern "system" fn SmsDeviceStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsDeviceStatusChanged(this, ::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSmsDeviceStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSmsDeviceStatusChanged(this, ::core::mem::transmute(&eventcookie)).into())
        }
        ISmsDevice_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendMessageAsync: SendMessageAsync::<Identity, Impl, OFFSET>,
            CalculateLength: CalculateLength::<Identity, Impl, OFFSET>,
            AccountPhoneNumber: AccountPhoneNumber::<Identity, Impl, OFFSET>,
            CellularClass: CellularClass::<Identity, Impl, OFFSET>,
            MessageStore: MessageStore::<Identity, Impl, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, Impl, OFFSET>,
            SmsMessageReceived: SmsMessageReceived::<Identity, Impl, OFFSET>,
            RemoveSmsMessageReceived: RemoveSmsMessageReceived::<Identity, Impl, OFFSET>,
            SmsDeviceStatusChanged: SmsDeviceStatusChanged::<Identity, Impl, OFFSET>,
            RemoveSmsDeviceStatusChanged: RemoveSmsDeviceStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISmsMessage_Impl: ::windows_core::BaseImpl {
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MessageClass(this: &Self::This) -> ::windows_core::Result<SmsMessageClass>;
}
impl ::windows_core::Iids for ISmsMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISmsMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MessageClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISmsMessage_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            MessageClass: MessageClass::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISmsMessageBase_Impl: ::windows_core::BaseImpl {
    fn MessageType(this: &Self::This) -> ::windows_core::Result<SmsMessageType>;
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn CellularClass(this: &Self::This) -> ::windows_core::Result<CellularClass>;
    fn MessageClass(this: &Self::This) -> ::windows_core::Result<SmsMessageClass>;
    fn SimIccId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for ISmsMessageBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISmsMessageBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MessageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CellularClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CellularClass) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CellularClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MessageClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SmsMessageClass) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SimIccId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsMessageBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SimIccId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISmsMessageBase_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MessageType: MessageType::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            CellularClass: CellularClass::<Identity, Impl, OFFSET>,
            MessageClass: MessageClass::<Identity, Impl, OFFSET>,
            SimIccId: SimIccId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"deprecated\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
pub trait ISmsTextMessage_Impl: ::windows_core::BaseImpl + ISmsMessage_Impl {
    fn Timestamp(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DateTime>;
    fn PartReferenceId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn PartNumber(this: &Self::This) -> ::windows_core::Result<u32>;
    fn PartCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn To(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetTo(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn From(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetFrom(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Body(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetBody(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Encoding(this: &Self::This) -> ::windows_core::Result<SmsEncoding>;
    fn SetEncoding(this: &Self::This, value: SmsEncoding) -> ::windows_core::Result<()>;
    fn ToBinaryMessages(this: &Self::This, format: SmsDataFormat) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows_core::Iids for ISmsTextMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<ISmsMessage as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISmsTextMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PartReferenceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PartReferenceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PartNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PartNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PartCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PartCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn To<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::To(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTo(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn From<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::From(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrom(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Body<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Body(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Encoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SmsEncoding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Encoding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SmsEncoding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncoding(this, value).into())
        }
        unsafe extern "system" fn ToBinaryMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISmsTextMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: SmsDataFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToBinaryMessages(this, format) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISmsTextMessage_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            PartReferenceId: PartReferenceId::<Identity, Impl, OFFSET>,
            PartNumber: PartNumber::<Identity, Impl, OFFSET>,
            PartCount: PartCount::<Identity, Impl, OFFSET>,
            To: To::<Identity, Impl, OFFSET>,
            SetTo: SetTo::<Identity, Impl, OFFSET>,
            From: From::<Identity, Impl, OFFSET>,
            SetFrom: SetFrom::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            Encoding: Encoding::<Identity, Impl, OFFSET>,
            SetEncoding: SetEncoding::<Identity, Impl, OFFSET>,
            ToBinaryMessages: ToBinaryMessages::<Identity, Impl, OFFSET>,
        }
    };
}
