#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAsyncGetSendNotificationCookie_Impl: ::windows_core::BaseImpl + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(this: &Self::This, param0: ::core::option::Option<&IPrintAsyncNotifyDataObject>, param1: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAsyncGetSendNotificationCookie {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintAsyncCookie);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncGetSendNotificationCookie_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsyncGetSendNotificationCookie {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncGetSendNotificationCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishAsyncCallWithData(this, ::windows_core::from_raw_borrowed(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IAsyncGetSendNotificationCookie_Vtbl {
            base__: <IPrintAsyncCookie as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAsyncGetSrvReferralCookie_Impl: ::windows_core::BaseImpl {
    fn FinishAsyncCall(this: &Self::This, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn CancelAsyncCall(this: &Self::This, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn FinishAsyncCallWithData(this: &Self::This, param0: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAsyncGetSrvReferralCookie {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsyncGetSrvReferralCookie {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FinishAsyncCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishAsyncCall(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncCall(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncGetSrvReferralCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishAsyncCallWithData(this, ::core::mem::transmute(&param0)).into())
        }
        IAsyncGetSrvReferralCookie_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FinishAsyncCall: FinishAsyncCall::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IBidiAsyncNotifyChannel_Impl: ::windows_core::BaseImpl + IPrintAsyncNotifyChannel_Impl {
    fn CreateNotificationChannel(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetPrintName(this: &Self::This, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn GetChannelNotificationType(this: &Self::This, param0: *const ::core::option::Option<IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn AsyncGetNotificationSendResponse(this: &Self::This, param0: ::core::option::Option<&IPrintAsyncNotifyDataObject>, param1: ::core::option::Option<&IAsyncGetSendNotificationCookie>) -> ::windows_core::Result<()>;
    fn AsyncCloseChannel(this: &Self::This, param0: ::core::option::Option<&IPrintAsyncNotifyDataObject>, param1: ::core::option::Option<&IPrintAsyncCookie>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBidiAsyncNotifyChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintAsyncNotifyChannel);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBidiAsyncNotifyChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateNotificationChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNotificationChannel(this).into())
        }
        unsafe extern "system" fn GetPrintName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrintName(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn GetChannelNotificationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChannelNotificationType(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn AsyncGetNotificationSendResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncGetNotificationSendResponse(this, ::windows_core::from_raw_borrowed(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        unsafe extern "system" fn AsyncCloseChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiAsyncNotifyChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncCloseChannel(this, ::windows_core::from_raw_borrowed(&param0), ::windows_core::from_raw_borrowed(&param1)).into())
        }
        IBidiAsyncNotifyChannel_Vtbl {
            base__: <IPrintAsyncNotifyChannel as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateNotificationChannel: CreateNotificationChannel::<Identity, Impl, OFFSET>,
            GetPrintName: GetPrintName::<Identity, Impl, OFFSET>,
            GetChannelNotificationType: GetChannelNotificationType::<Identity, Impl, OFFSET>,
            AsyncGetNotificationSendResponse: AsyncGetNotificationSendResponse::<Identity, Impl, OFFSET>,
            AsyncCloseChannel: AsyncCloseChannel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IBidiRequest_Impl: ::windows_core::BaseImpl {
    fn SetSchema(this: &Self::This, pszschema: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetInputData(this: &Self::This, dwtype: u32, pdata: *const u8, usize: u32) -> ::windows_core::Result<()>;
    fn GetResult(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetOutputData(this: &Self::This, dwindex: u32, ppszschema: *mut ::windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> ::windows_core::Result<()>;
    fn GetEnumCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IBidiRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBidiRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszschema: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSchema(this, ::core::mem::transmute(&pszschema)).into())
        }
        unsafe extern "system" fn SetInputData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtype: u32, pdata: *const u8, usize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputData(this, ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&usize)).into())
        }
        unsafe extern "system" fn GetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszschema: *mut ::windows_core::PWSTR, pdwtype: *mut u32, ppdata: *mut *mut u8, usize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputData(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ppszschema), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&usize)).into())
        }
        unsafe extern "system" fn GetEnumCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtotal: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtotal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBidiRequest_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSchema: SetSchema::<Identity, Impl, OFFSET>,
            SetInputData: SetInputData::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
            GetOutputData: GetOutputData::<Identity, Impl, OFFSET>,
            GetEnumCount: GetEnumCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiRequestContainer_Impl: ::windows_core::BaseImpl {
    fn AddRequest(this: &Self::This, prequest: ::core::option::Option<&IBidiRequest>) -> ::windows_core::Result<()>;
    fn GetEnumObject(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn GetRequestCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IBidiRequestContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBidiRequestContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRequest(this, ::windows_core::from_raw_borrowed(&prequest)).into())
        }
        unsafe extern "system" fn GetEnumObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRequestCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiRequestContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequestCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBidiRequestContainer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRequest: AddRequest::<Identity, Impl, OFFSET>,
            GetEnumObject: GetEnumObject::<Identity, Impl, OFFSET>,
            GetRequestCount: GetRequestCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IBidiSpl_Impl: ::windows_core::BaseImpl {
    fn BindDevice(this: &Self::This, pszdevicename: &::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::Result<()>;
    fn UnbindDevice(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendRecv(this: &Self::This, pszaction: &::windows_core::PCWSTR, prequest: ::core::option::Option<&IBidiRequest>) -> ::windows_core::Result<()>;
    fn MultiSendRecv(this: &Self::This, pszaction: &::windows_core::PCWSTR, prequestcontainer: ::core::option::Option<&IBidiRequestContainer>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBidiSpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBidiSpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdevicename: ::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindDevice(this, ::core::mem::transmute(&pszdevicename), ::core::mem::transmute_copy(&dwaccess)).into())
        }
        unsafe extern "system" fn UnbindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnbindDevice(this).into())
        }
        unsafe extern "system" fn SendRecv<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszaction: ::windows_core::PCWSTR, prequest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendRecv(this, ::core::mem::transmute(&pszaction), ::windows_core::from_raw_borrowed(&prequest)).into())
        }
        unsafe extern "system" fn MultiSendRecv<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszaction: ::windows_core::PCWSTR, prequestcontainer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MultiSendRecv(this, ::core::mem::transmute(&pszaction), ::windows_core::from_raw_borrowed(&prequestcontainer)).into())
        }
        IBidiSpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BindDevice: BindDevice::<Identity, Impl, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, Impl, OFFSET>,
            SendRecv: SendRecv::<Identity, Impl, OFFSET>,
            MultiSendRecv: MultiSendRecv::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBidiSpl2_Impl: ::windows_core::BaseImpl {
    fn BindDevice(this: &Self::This, pszdevicename: &::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::Result<()>;
    fn UnbindDevice(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendRecvXMLString(this: &Self::This, bstrrequest: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SendRecvXMLStream(this: &Self::This, psrequest: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IBidiSpl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBidiSpl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdevicename: ::windows_core::PCWSTR, dwaccess: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindDevice(this, ::core::mem::transmute(&pszdevicename), ::core::mem::transmute_copy(&dwaccess)).into())
        }
        unsafe extern "system" fn UnbindDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnbindDevice(this).into())
        }
        unsafe extern "system" fn SendRecvXMLString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrequest: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrresponse: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendRecvXMLString(this, ::core::mem::transmute(&bstrrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendRecvXMLStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBidiSpl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrequest: *mut ::core::ffi::c_void, ppsresponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendRecvXMLStream(this, ::windows_core::from_raw_borrowed(&psrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBidiSpl2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BindDevice: BindDevice::<Identity, Impl, OFFSET>,
            UnbindDevice: UnbindDevice::<Identity, Impl, OFFSET>,
            SendRecvXMLString: SendRecvXMLString::<Identity, Impl, OFFSET>,
            SendRecvXMLStream: SendRecvXMLStream::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFixedDocument_Impl: ::windows_core::BaseImpl {
    fn GetUri(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPrintTicket(this: &Self::This) -> ::windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(this: &Self::This, pprintticket: ::core::option::Option<&IPartPrintTicket>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFixedDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFixedDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintTicket(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprintticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintTicket(this, ::windows_core::from_raw_borrowed(&pprintticket)).into())
        }
        IFixedDocument_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, Impl, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFixedDocumentSequence_Impl: ::windows_core::BaseImpl {
    fn GetUri(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPrintTicket(this: &Self::This) -> ::windows_core::Result<IPartPrintTicket>;
    fn SetPrintTicket(this: &Self::This, pprintticket: ::core::option::Option<&IPartPrintTicket>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFixedDocumentSequence {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFixedDocumentSequence {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintTicket(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprintticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedDocumentSequence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintTicket(this, ::windows_core::from_raw_borrowed(&pprintticket)).into())
        }
        IFixedDocumentSequence_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetPrintTicket: GetPrintTicket::<Identity, Impl, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFixedPage_Impl: ::windows_core::BaseImpl + IPartBase_Impl {
    fn GetPrintTicket(this: &Self::This) -> ::windows_core::Result<IPartPrintTicket>;
    fn GetPagePart(this: &Self::This, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetWriteStream(this: &Self::This) -> ::windows_core::Result<IPrintWriteStream>;
    fn SetPrintTicket(this: &Self::This, ppprintticket: ::core::option::Option<&IPartPrintTicket>) -> ::windows_core::Result<()>;
    fn SetPagePart(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DeleteResource(this: &Self::This, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetXpsPartIterator(this: &Self::This) -> ::windows_core::Result<IXpsPartIterator>;
}
impl ::windows_core::Iids for IFixedPage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFixedPage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintTicket(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprintticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPagePart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPagePart(this, ::core::mem::transmute(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwritestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWriteStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwritestream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintTicket(this, ::windows_core::from_raw_borrowed(&ppprintticket)).into())
        }
        unsafe extern "system" fn SetPagePart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPagePart(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn DeleteResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteResource(this, ::core::mem::transmute(&uri)).into())
        }
        unsafe extern "system" fn GetXpsPartIterator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFixedPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxpspartit: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXpsPartIterator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxpspartit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFixedPage_Vtbl {
            base__: <IPartBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrintTicket: GetPrintTicket::<Identity, Impl, OFFSET>,
            GetPagePart: GetPagePart::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
            SetPrintTicket: SetPrintTicket::<Identity, Impl, OFFSET>,
            SetPagePart: SetPagePart::<Identity, Impl, OFFSET>,
            DeleteResource: DeleteResource::<Identity, Impl, OFFSET>,
            GetXpsPartIterator: GetXpsPartIterator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IImgCreateErrorInfo_Impl: ::windows_core::BaseImpl + super::super::System::Ole::ICreateErrorInfo_Impl {
    fn AttachToErrorInfo(this: &Self::This, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::Iids for IImgCreateErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Ole::ICreateErrorInfo);
}
#[cfg(feature = "Win32_System_Ole")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgCreateErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImgCreateErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AttachToErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgCreateErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachToErrorInfo(this, ::core::mem::transmute_copy(&perrorinfo)).into())
        }
        IImgCreateErrorInfo_Vtbl {
            base__: <super::super::System::Ole::ICreateErrorInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AttachToErrorInfo: AttachToErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IImgErrorInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IErrorInfo_Impl {
    fn GetDeveloperDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUserErrorId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetUserParameterCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetUserParameter(this: &Self::This, cparam: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUserFallback(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetExceptionId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn DetachErrorInfo(this: &Self::This, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IImgErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IErrorInfo);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImgErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeveloperDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeveloperDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserErrorId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserErrorId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrorid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserParameterCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcuserparams: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserParameterCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcuserparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cparam: u32, pbstrparam: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserParameter(this, ::core::mem::transmute_copy(&cparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrparam, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfallback: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserFallback(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExceptionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexceptionid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExceptionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexceptionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DetachErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImgErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorinfo: *mut ImgErrorInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetachErrorInfo(this, ::core::mem::transmute_copy(&perrorinfo)).into())
        }
        IImgErrorInfo_Vtbl {
            base__: <super::super::System::Com::IErrorInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeveloperDescription: GetDeveloperDescription::<Identity, Impl, OFFSET>,
            GetUserErrorId: GetUserErrorId::<Identity, Impl, OFFSET>,
            GetUserParameterCount: GetUserParameterCount::<Identity, Impl, OFFSET>,
            GetUserParameter: GetUserParameter::<Identity, Impl, OFFSET>,
            GetUserFallback: GetUserFallback::<Identity, Impl, OFFSET>,
            GetExceptionId: GetExceptionId::<Identity, Impl, OFFSET>,
            DetachErrorInfo: DetachErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInterFilterCommunicator_Impl: ::windows_core::BaseImpl {
    fn RequestReader(this: &Self::This, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestWriter(this: &Self::This, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInterFilterCommunicator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterFilterCommunicator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInterFilterCommunicator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterFilterCommunicator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestReader(this, ::core::mem::transmute_copy(&ppireader)).into())
        }
        unsafe extern "system" fn RequestWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterFilterCommunicator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestWriter(this, ::core::mem::transmute_copy(&ppiwriter)).into())
        }
        IInterFilterCommunicator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestReader: RequestReader::<Identity, Impl, OFFSET>,
            RequestWriter: RequestWriter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPartBase_Impl: ::windows_core::BaseImpl {
    fn GetUri(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<IPrintReadStream>;
    fn GetPartCompression(this: &Self::This) -> ::windows_core::Result<EXpsCompressionOptions>;
    fn SetPartCompression(this: &Self::This, compression: EXpsCompressionOptions) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPartBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartCompression<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompression: *mut EXpsCompressionOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartCompression(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcompression, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPartCompression<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compression: EXpsCompressionOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPartCompression(this, ::core::mem::transmute_copy(&compression)).into())
        }
        IPartBase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUri: GetUri::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            GetPartCompression: GetPartCompression::<Identity, Impl, OFFSET>,
            SetPartCompression: SetPartCompression::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPartColorProfile_Impl: ::windows_core::BaseImpl + IPartBase_Impl {}
impl ::windows_core::Iids for IPartColorProfile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartColorProfile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartColorProfile {
    const VTABLE: Self::Vtable = { IPartColorProfile_Vtbl { base__: <IPartBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IPartDiscardControl_Impl: ::windows_core::BaseImpl {
    fn GetDiscardProperties(this: &Self::This, urisentinelpage: *mut ::windows_core::BSTR, uriparttodiscard: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPartDiscardControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartDiscardControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartDiscardControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDiscardProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartDiscardControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urisentinelpage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, uriparttodiscard: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDiscardProperties(this, ::core::mem::transmute_copy(&urisentinelpage), ::core::mem::transmute_copy(&uriparttodiscard)).into())
        }
        IPartDiscardControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDiscardProperties: GetDiscardProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPartFont_Impl: ::windows_core::BaseImpl + IPartBase_Impl {
    fn GetFontProperties(this: &Self::This, pcontenttype: *mut ::windows_core::BSTR, pfontoptions: *mut EXpsFontOptions) -> ::windows_core::Result<()>;
    fn SetFontContent(this: &Self::This, pcontenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFontOptions(this: &Self::This, options: EXpsFontOptions) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPartFont {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartFont {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfontoptions: *mut EXpsFontOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontProperties(this, ::core::mem::transmute_copy(&pcontenttype), ::core::mem::transmute_copy(&pfontoptions)).into())
        }
        unsafe extern "system" fn SetFontContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontContent(this, ::core::mem::transmute(&pcontenttype)).into())
        }
        unsafe extern "system" fn SetFontOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: EXpsFontOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontOptions(this, ::core::mem::transmute_copy(&options)).into())
        }
        IPartFont_Vtbl {
            base__: <IPartBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontProperties: GetFontProperties::<Identity, Impl, OFFSET>,
            SetFontContent: SetFontContent::<Identity, Impl, OFFSET>,
            SetFontOptions: SetFontOptions::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPartFont2_Impl: ::windows_core::BaseImpl + IPartFont_Impl {
    fn GetFontRestriction(this: &Self::This) -> ::windows_core::Result<EXpsFontRestriction>;
}
impl ::windows_core::Iids for IPartFont2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartFont);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartFont2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartFont2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontRestriction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartFont2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prestriction: *mut EXpsFontRestriction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontRestriction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prestriction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPartFont2_Vtbl { base__: <IPartFont as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetFontRestriction: GetFontRestriction::<Identity, Impl, OFFSET> }
    };
}
pub trait IPartImage_Impl: ::windows_core::BaseImpl + IPartBase_Impl {
    fn GetImageProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetImageContent(this: &Self::This, pcontenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPartImage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartImage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartImage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImageProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetImageContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImageContent(this, ::core::mem::transmute(&pcontenttype)).into())
        }
        IPartImage_Vtbl {
            base__: <IPartBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImageProperties: GetImageProperties::<Identity, Impl, OFFSET>,
            SetImageContent: SetImageContent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPartPrintTicket_Impl: ::windows_core::BaseImpl + IPartBase_Impl {}
impl ::windows_core::Iids for IPartPrintTicket {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartPrintTicket_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartPrintTicket {
    const VTABLE: Self::Vtable = { IPartPrintTicket_Vtbl { base__: <IPartBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IPartResourceDictionary_Impl: ::windows_core::BaseImpl + IPartBase_Impl {}
impl ::windows_core::Iids for IPartResourceDictionary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartResourceDictionary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartResourceDictionary {
    const VTABLE: Self::Vtable = { IPartResourceDictionary_Vtbl { base__: <IPartBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IPartThumbnail_Impl: ::windows_core::BaseImpl + IPartBase_Impl {
    fn GetThumbnailProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetThumbnailContent(this: &Self::This, pcontenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPartThumbnail {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPartBase);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartThumbnail_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPartThumbnail {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetThumbnailProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartThumbnail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnailProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetThumbnailContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPartThumbnail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThumbnailContent(this, ::core::mem::transmute(&pcontenttype)).into())
        }
        IPartThumbnail_Vtbl {
            base__: <IPartBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetThumbnailProperties: GetThumbnailProperties::<Identity, Impl, OFFSET>,
            SetThumbnailContent: SetThumbnailContent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncCookie_Impl: ::windows_core::BaseImpl {
    fn FinishAsyncCall(this: &Self::This, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn CancelAsyncCall(this: &Self::This, param0: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintAsyncCookie {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncCookie_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncCookie {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FinishAsyncCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishAsyncCall(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncCall(this, ::core::mem::transmute_copy(&param0)).into())
        }
        IPrintAsyncCookie_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FinishAsyncCall: FinishAsyncCall::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncNewChannelCookie_Impl: ::windows_core::BaseImpl + IPrintAsyncCookie_Impl {
    fn FinishAsyncCallWithData(this: &Self::This, param0: *const ::core::option::Option<IPrintAsyncNotifyChannel>, param1: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintAsyncNewChannelCookie {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintAsyncCookie);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNewChannelCookie_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncNewChannelCookie {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FinishAsyncCallWithData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNewChannelCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const *mut ::core::ffi::c_void, param1: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishAsyncCallWithData(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into())
        }
        IPrintAsyncNewChannelCookie_Vtbl {
            base__: <IPrintAsyncCookie as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FinishAsyncCallWithData: FinishAsyncCallWithData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncNotify_Impl: ::windows_core::BaseImpl {
    fn CreatePrintAsyncNotifyChannel(this: &Self::This, param0: u32, param1: *const ::windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: ::core::option::Option<&IPrintAsyncNotifyCallback>) -> ::windows_core::Result<IPrintAsyncNotifyChannel>;
    fn CreatePrintAsyncNotifyRegistration(this: &Self::This, param0: *const ::windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: ::core::option::Option<&IPrintAsyncNotifyCallback>) -> ::windows_core::Result<IPrintAsyncNotifyRegistration>;
}
impl ::windows_core::Iids for IPrintAsyncNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePrintAsyncNotifyChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *const ::windows_core::GUID, param2: PrintAsyncNotifyUserFilter, param3: PrintAsyncNotifyConversationStyle, param4: *mut ::core::ffi::c_void, param5: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePrintAsyncNotifyChannel(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::windows_core::from_raw_borrowed(&param4)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param5, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePrintAsyncNotifyRegistration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: PrintAsyncNotifyUserFilter, param2: PrintAsyncNotifyConversationStyle, param3: *mut ::core::ffi::c_void, param4: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePrintAsyncNotifyRegistration(this, ::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows_core::from_raw_borrowed(&param3)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param4, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintAsyncNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePrintAsyncNotifyChannel: CreatePrintAsyncNotifyChannel::<Identity, Impl, OFFSET>,
            CreatePrintAsyncNotifyRegistration: CreatePrintAsyncNotifyRegistration::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncNotifyCallback_Impl: ::windows_core::BaseImpl {
    fn OnEventNotify(this: &Self::This, pchannel: ::core::option::Option<&IPrintAsyncNotifyChannel>, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn ChannelClosed(this: &Self::This, pchannel: ::core::option::Option<&IPrintAsyncNotifyChannel>, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintAsyncNotifyCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncNotifyCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnEventNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEventNotify(this, ::windows_core::from_raw_borrowed(&pchannel), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        unsafe extern "system" fn ChannelClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChannelClosed(this, ::windows_core::from_raw_borrowed(&pchannel), ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        IPrintAsyncNotifyCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnEventNotify: OnEventNotify::<Identity, Impl, OFFSET>,
            ChannelClosed: ChannelClosed::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncNotifyChannel_Impl: ::windows_core::BaseImpl {
    fn SendNotification(this: &Self::This, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
    fn CloseChannel(this: &Self::This, pdata: ::core::option::Option<&IPrintAsyncNotifyDataObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintAsyncNotifyChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncNotifyChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendNotification(this, ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        unsafe extern "system" fn CloseChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseChannel(this, ::windows_core::from_raw_borrowed(&pdata)).into())
        }
        IPrintAsyncNotifyChannel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendNotification: SendNotification::<Identity, Impl, OFFSET>,
            CloseChannel: CloseChannel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncNotifyDataObject_Impl: ::windows_core::BaseImpl {
    fn AcquireData(this: &Self::This, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ReleaseData(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintAsyncNotifyDataObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncNotifyDataObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcquireData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnotificationdata: *mut *mut u8, psize: *mut u32, ppschema: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireData(this, ::core::mem::transmute_copy(&ppnotificationdata), ::core::mem::transmute_copy(&psize), ::core::mem::transmute_copy(&ppschema)).into())
        }
        unsafe extern "system" fn ReleaseData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyDataObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseData(this).into())
        }
        IPrintAsyncNotifyDataObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcquireData: AcquireData::<Identity, Impl, OFFSET>,
            ReleaseData: ReleaseData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncNotifyRegistration_Impl: ::windows_core::BaseImpl {
    fn RegisterForNotifications(this: &Self::This) -> ::windows_core::Result<()>;
    fn UnregisterForNotifications(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintAsyncNotifyRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncNotifyRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterForNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterForNotifications(this).into())
        }
        unsafe extern "system" fn UnregisterForNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterForNotifications(this).into())
        }
        IPrintAsyncNotifyRegistration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterForNotifications: RegisterForNotifications::<Identity, Impl, OFFSET>,
            UnregisterForNotifications: UnregisterForNotifications::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintAsyncNotifyServerReferral_Impl: ::windows_core::BaseImpl {
    fn GetServerReferral(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn AsyncGetServerReferral(this: &Self::This, param0: ::core::option::Option<&IAsyncGetSrvReferralCookie>) -> ::windows_core::Result<()>;
    fn SetServerReferral(this: &Self::This, prmtserverreferral: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintAsyncNotifyServerReferral {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintAsyncNotifyServerReferral {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetServerReferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServerReferral(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param0, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AsyncGetServerReferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncGetServerReferral(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        unsafe extern "system" fn SetServerReferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintAsyncNotifyServerReferral_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prmtserverreferral: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerReferral(this, ::core::mem::transmute(&prmtserverreferral)).into())
        }
        IPrintAsyncNotifyServerReferral_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetServerReferral: GetServerReferral::<Identity, Impl, OFFSET>,
            AsyncGetServerReferral: AsyncGetServerReferral::<Identity, Impl, OFFSET>,
            SetServerReferral: SetServerReferral::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintBidiAsyncNotifyRegistration_Impl: ::windows_core::BaseImpl + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNewChannel(this: &Self::This, param0: ::core::option::Option<&IPrintAsyncNewChannelCookie>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintBidiAsyncNotifyRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintAsyncNotifyRegistration);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintBidiAsyncNotifyRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncGetNewChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintBidiAsyncNotifyRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncGetNewChannel(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        IPrintBidiAsyncNotifyRegistration_Vtbl {
            base__: <IPrintAsyncNotifyRegistration as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncGetNewChannel: AsyncGetNewChannel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintClassObjectFactory_Impl: ::windows_core::BaseImpl {
    fn GetPrintClassObject(this: &Self::This, pszprintername: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintClassObjectFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintClassObjectFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintClassObjectFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrintClassObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintClassObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszprintername: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrintClassObject(this, ::core::mem::transmute(&pszprintername), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppnewobject)).into())
        }
        IPrintClassObjectFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrintClassObject: GetPrintClassObject::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelper_Impl: ::windows_core::BaseImpl {
    fn GetOption(this: &Self::This, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: &::windows_core::PCSTR) -> ::windows_core::Result<::windows_core::PCSTR>;
    fn SetOptions(this: &Self::This, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn EnumConstrainedOptions(this: &Self::This, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &::windows_core::PCSTR, pconstrainedoptionlist: *const *const *const ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::Result<()>;
    fn WhyConstrained(this: &Self::This, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows_core::Result<()>;
    fn EnumFeatures(this: &Self::This, pfeaturelist: *mut *mut *mut ::windows_core::PCSTR, pdwnumfeatures: *mut u32) -> ::windows_core::Result<()>;
    fn EnumOptions(this: &Self::This, pszfeaturekeyword: &::windows_core::PCSTR, poptionlist: *mut *mut *mut ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::Result<()>;
    fn GetFontSubstitution(this: &Self::This, psztruetypefontname: &::windows_core::PCWSTR, ppszdevfontname: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFontSubstitution(this: &Self::This, psztruetypefontname: &::windows_core::PCWSTR, pszdevfontname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CreateInstanceOfMSXMLObject(this: &Self::This, rclsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IPrintCoreHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintCoreHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturerequested: ::windows_core::PCSTR, ppszoption: *mut ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOption(this, ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszfeaturerequested)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, bresolveconflicts: super::super::Foundation::BOOL, pfopairs: *const PRINT_FEATURE_OPTION, cpairs: u32, pcpairswritten: *mut u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOptions(this, ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&bresolveconflicts), ::core::mem::transmute_copy(&pfopairs), ::core::mem::transmute_copy(&cpairs), ::core::mem::transmute_copy(&pcpairswritten), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: ::windows_core::PCSTR, pconstrainedoptionlist: *const *const *const ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumConstrainedOptions(this, ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&pconstrainedoptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into())
        }
        unsafe extern "system" fn WhyConstrained<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, ppfoconstraints: *mut *mut PRINT_FEATURE_OPTION, pdwnumoptions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WhyConstrained(this, ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute_copy(&ppfoconstraints), ::core::mem::transmute_copy(&pdwnumoptions)).into())
        }
        unsafe extern "system" fn EnumFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfeaturelist: *mut *mut *mut ::windows_core::PCSTR, pdwnumfeatures: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumFeatures(this, ::core::mem::transmute_copy(&pfeaturelist), ::core::mem::transmute_copy(&pdwnumfeatures)).into())
        }
        unsafe extern "system" fn EnumOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows_core::PCSTR, poptionlist: *mut *mut *mut ::windows_core::PCSTR, pdwnumoptions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumOptions(this, ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&poptionlist), ::core::mem::transmute_copy(&pdwnumoptions)).into())
        }
        unsafe extern "system" fn GetFontSubstitution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztruetypefontname: ::windows_core::PCWSTR, ppszdevfontname: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSubstitution(this, ::core::mem::transmute(&psztruetypefontname), ::core::mem::transmute_copy(&ppszdevfontname)).into())
        }
        unsafe extern "system" fn SetFontSubstitution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztruetypefontname: ::windows_core::PCWSTR, pszdevfontname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontSubstitution(this, ::core::mem::transmute(&psztruetypefontname), ::core::mem::transmute(&pszdevfontname)).into())
        }
        unsafe extern "system" fn CreateInstanceOfMSXMLObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstanceOfMSXMLObject(this, ::core::mem::transmute_copy(&rclsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IPrintCoreHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, Impl, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, Impl, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, Impl, OFFSET>,
            EnumOptions: EnumOptions::<Identity, Impl, OFFSET>,
            GetFontSubstitution: GetFontSubstitution::<Identity, Impl, OFFSET>,
            SetFontSubstitution: SetFontSubstitution::<Identity, Impl, OFFSET>,
            CreateInstanceOfMSXMLObject: CreateInstanceOfMSXMLObject::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintCoreHelperPS_Impl: ::windows_core::BaseImpl + IPrintCoreHelper_Impl {
    fn GetGlobalAttribute(this: &Self::This, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetFeatureAttribute(this: &Self::This, pszfeaturekeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetOptionAttribute(this: &Self::This, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IPrintCoreHelperPS {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintCoreHelper);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintCoreHelperPS {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGlobalAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlobalAttribute(this, ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into())
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureAttribute(this, ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into())
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, ppbdata: *mut *mut u8, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptionAttribute(this, ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbsize)).into())
        }
        IPrintCoreHelperPS_Vtbl {
            base__: <IPrintCoreHelper as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGlobalAttribute: GetGlobalAttribute::<Identity, Impl, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, Impl, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni_Impl: ::windows_core::BaseImpl + IPrintCoreHelper_Impl {
    fn CreateGDLSnapshot(this: &Self::This, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CreateDefaultGDLSnapshot(this: &Self::This, dwflags: u32) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPrintCoreHelperUni {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintCoreHelper);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperUni_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintCoreHelperUni {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateGDLSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperUni_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::Gdi::DEVMODEA, cbsize: u32, dwflags: u32, ppsnapshotstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateGDLSnapshot(this, ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppsnapshotstream)).into())
        }
        unsafe extern "system" fn CreateDefaultGDLSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperUni_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppsnapshotstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDefaultGDLSnapshot(this, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsnapshotstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintCoreHelperUni_Vtbl {
            base__: <IPrintCoreHelper as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateGDLSnapshot: CreateGDLSnapshot::<Identity, Impl, OFFSET>,
            CreateDefaultGDLSnapshot: CreateDefaultGDLSnapshot::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintCoreHelperUni2_Impl: ::windows_core::BaseImpl + IPrintCoreHelperUni_Impl {
    fn GetNamedCommand(this: &Self::This, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: &::windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPrintCoreHelperUni2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintCoreHelperUni);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperUni2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintCoreHelperUni2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNamedCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreHelperUni2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *const super::Gdi::DEVMODEA, cbsize: u32, pszcommandname: ::windows_core::PCWSTR, ppcommandbytes: *mut *mut u8, pcbcommandsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNamedCommand(this, ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute(&pszcommandname), ::core::mem::transmute_copy(&ppcommandbytes), ::core::mem::transmute_copy(&pcbcommandsize)).into())
        }
        IPrintCoreHelperUni2_Vtbl { base__: <IPrintCoreHelperUni as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetNamedCommand: GetNamedCommand::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintCoreUI2_Impl: ::windows_core::BaseImpl + IPrintOemDriverUI_Impl {
    fn GetOptions(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn SetOptions(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32) -> ::windows_core::Result<u32>;
    fn EnumConstrainedOptions(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pmszconstrainedoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn WhyConstrained(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, pmszreasonlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn GetGlobalAttribute(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn GetFeatureAttribute(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn GetOptionAttribute(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pszoptionkeyword: &::windows_core::PCSTR, pszattribute: &::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn EnumFeatures(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn EnumOptions(this: &Self::This, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: &::windows_core::PCSTR, pmszoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn QuerySimulationSupport(this: &Self::This, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPrintCoreUI2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintOemDriverUI);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintCoreUI2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturesrequested: *const i8, cbin: u32, pmszfeatureoptionbuf: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptions(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturesrequested), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeatureoptionbuf: *const i8, cbin: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetOptions(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeatureoptionbuf), ::core::mem::transmute_copy(&cbin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumConstrainedOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pmszconstrainedoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumConstrainedOptions(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszconstrainedoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn WhyConstrained<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, pmszreasonlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WhyConstrained(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute_copy(&pmszreasonlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn GetGlobalAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlobalAttribute(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn GetFeatureAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeatureAttribute(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn GetOptionAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pszoptionkeyword: ::windows_core::PCSTR, pszattribute: ::windows_core::PCSTR, pdwdatatype: *mut u32, pbdata: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptionAttribute(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute(&pszoptionkeyword), ::core::mem::transmute(&pszattribute), ::core::mem::transmute_copy(&pdwdatatype), ::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn EnumFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pmszfeaturelist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumFeatures(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pmszfeaturelist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn EnumOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, dwflags: u32, pszfeaturekeyword: ::windows_core::PCSTR, pmszoptionlist: ::windows_core::PSTR, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumOptions(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszfeaturekeyword), ::core::mem::transmute_copy(&pmszoptionlist), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn QuerySimulationSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCoreUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, dwlevel: u32, pcaps: *mut u8, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QuerySimulationSupport(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pcaps), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        IPrintCoreUI2_Vtbl {
            base__: <IPrintOemDriverUI as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            EnumConstrainedOptions: EnumConstrainedOptions::<Identity, Impl, OFFSET>,
            WhyConstrained: WhyConstrained::<Identity, Impl, OFFSET>,
            GetGlobalAttribute: GetGlobalAttribute::<Identity, Impl, OFFSET>,
            GetFeatureAttribute: GetFeatureAttribute::<Identity, Impl, OFFSET>,
            GetOptionAttribute: GetOptionAttribute::<Identity, Impl, OFFSET>,
            EnumFeatures: EnumFeatures::<Identity, Impl, OFFSET>,
            EnumOptions: EnumOptions::<Identity, Impl, OFFSET>,
            QuerySimulationSupport: QuerySimulationSupport::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintJob_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn PrintedPages(this: &Self::This) -> ::windows_core::Result<u32>;
    fn TotalPages(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Status(this: &Self::This) -> ::windows_core::Result<PrintJobStatus>;
    fn SubmissionTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn RequestCancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrintedPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrintedPages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulpages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalPages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut PrintJobStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubmissionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubmissiontime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubmissionTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psubmissiontime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestCancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestCancel(this).into())
        }
        IPrintJob_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            PrintedPages: PrintedPages::<Identity, Impl, OFFSET>,
            TotalPages: TotalPages::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            RequestCancel: RequestCancel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintJobCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, ulindex: u32) -> ::windows_core::Result<IPrintJob>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintJobCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintJobCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintJobCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintJobCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemCommon_Impl: ::windows_core::BaseImpl {
    fn GetInfo(this: &Self::This, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::Result<()>;
    fn DevMode(this: &Self::This, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IPrintOemCommon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemCommon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintOemCommon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemCommon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, pbuffer: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfo(this, ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded)).into())
        }
        unsafe extern "system" fn DevMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemCommon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemdmparam: *mut OEMDMPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DevMode(this, ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemdmparam)).into())
        }
        IPrintOemCommon_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            DevMode: DevMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintOemDriverUI_Impl: ::windows_core::BaseImpl {
    fn DrvGetDriverSetting(this: &Self::This, pci: *mut ::core::ffi::c_void, feature: &::windows_core::PCSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows_core::Result<()>;
    fn DrvUpgradeRegistrySetting(this: &Self::This, hprinter: super::super::Foundation::HANDLE, pfeature: &::windows_core::PCSTR, poption: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn DrvUpdateUISetting(this: &Self::This, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPrintOemDriverUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintOemDriverUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DrvGetDriverSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, feature: ::windows_core::PCSTR, poutput: *mut ::core::ffi::c_void, cbsize: u32, pcbneeded: *mut u32, pdwoptionsreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrvGetDriverSetting(this, ::core::mem::transmute_copy(&pci), ::core::mem::transmute(&feature), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pcbneeded), ::core::mem::transmute_copy(&pdwoptionsreturned)).into())
        }
        unsafe extern "system" fn DrvUpgradeRegistrySetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pfeature: ::windows_core::PCSTR, poption: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrvUpgradeRegistrySetting(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute(&pfeature), ::core::mem::transmute(&poption)).into())
        }
        unsafe extern "system" fn DrvUpdateUISetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemDriverUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pci: *mut ::core::ffi::c_void, poptitem: *mut ::core::ffi::c_void, dwpreviousselection: u32, dwmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrvUpdateUISetting(this, ::core::mem::transmute_copy(&pci), ::core::mem::transmute_copy(&poptitem), ::core::mem::transmute_copy(&dwpreviousselection), ::core::mem::transmute_copy(&dwmode)).into())
        }
        IPrintOemDriverUI_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DrvGetDriverSetting: DrvGetDriverSetting::<Identity, Impl, OFFSET>,
            DrvUpgradeRegistrySetting: DrvUpgradeRegistrySetting::<Identity, Impl, OFFSET>,
            DrvUpdateUISetting: DrvUpdateUISetting::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI_Impl: ::windows_core::BaseImpl + IPrintOemCommon_Impl {
    fn PublishDriverInterface(this: &Self::This, piunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CommonUIProp(this: &Self::This, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows_core::Result<()>;
    fn DocumentPropertySheets(this: &Self::This, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn DevicePropertySheets(this: &Self::This, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn DevQueryPrintEx(this: &Self::This, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DeviceCapabilitiesA(this: &Self::This, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: &::windows_core::PCWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows_core::Result<()>;
    fn UpgradePrinter(this: &Self::This, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows_core::Result<()>;
    fn PrinterEvent(this: &Self::This, pprintername: &::windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn DriverEvent(this: &Self::This, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn QueryColorProfile(this: &Self::This, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows_core::Result<()>;
    fn FontInstallerDlgProc(this: &Self::This, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn UpdateExternalFonts(this: &Self::This, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IPrintOemUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintOemCommon);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintOemUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PublishDriverInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PublishDriverInterface(this, ::windows_core::from_raw_borrowed(&piunknown)).into())
        }
        unsafe extern "system" fn CommonUIProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, poemcuipparam: *const OEMCUIPPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommonUIProp(this, ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&poemcuipparam)).into())
        }
        unsafe extern "system" fn DocumentPropertySheets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DocumentPropertySheets(this, ::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn DevicePropertySheets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsuiinfo: *const PROPSHEETUI_INFO, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DevicePropertySheets(this, ::core::mem::transmute_copy(&ppsuiinfo), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn DevQueryPrintEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *const OEMUIOBJ, pdqpinfo: *const DEVQUERYPRINT_INFO, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DevQueryPrintEx(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&pdqpinfo), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm)).into())
        }
        unsafe extern "system" fn DeviceCapabilitiesA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poemuiobj: *mut OEMUIOBJ, hprinter: super::super::Foundation::HANDLE, pdevicename: ::windows_core::PCWSTR, wcapability: u16, poutput: *mut ::core::ffi::c_void, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, dwold: u32, dwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceCapabilitiesA(this, ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute(&pdevicename), ::core::mem::transmute_copy(&wcapability), ::core::mem::transmute_copy(&poutput), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&dwold), ::core::mem::transmute_copy(&dwresult)).into())
        }
        unsafe extern "system" fn UpgradePrinter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlevel: u32, pdriverupgradeinfo: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpgradePrinter(this, ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverupgradeinfo)).into())
        }
        unsafe extern "system" fn PrinterEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintername: ::windows_core::PCWSTR, idriverevent: i32, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrinterEvent(this, ::core::mem::transmute(&pprintername), ::core::mem::transmute_copy(&idriverevent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn DriverEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdriverevent: u32, dwlevel: u32, pdriverinfo: *const u8, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DriverEvent(this, ::core::mem::transmute_copy(&dwdriverevent), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&pdriverinfo), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn QueryColorProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, poemuiobj: *const OEMUIOBJ, ppublicdm: *const super::Gdi::DEVMODEA, poemdm: *const ::core::ffi::c_void, ulquerymode: u32, pvprofiledata: *mut ::core::ffi::c_void, pcbprofiledata: *mut u32, pflprofiledata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryColorProfile(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&poemuiobj), ::core::mem::transmute_copy(&ppublicdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&ulquerymode), ::core::mem::transmute_copy(&pvprofiledata), ::core::mem::transmute_copy(&pcbprofiledata), ::core::mem::transmute_copy(&pflprofiledata)).into())
        }
        unsafe extern "system" fn FontInstallerDlgProc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, usmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FontInstallerDlgProc(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&usmsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn UpdateExternalFonts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hheap: super::super::Foundation::HANDLE, pwstrcartridges: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateExternalFonts(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hheap), ::core::mem::transmute(&pwstrcartridges)).into())
        }
        IPrintOemUI_Vtbl {
            base__: <IPrintOemCommon as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PublishDriverInterface: PublishDriverInterface::<Identity, Impl, OFFSET>,
            CommonUIProp: CommonUIProp::<Identity, Impl, OFFSET>,
            DocumentPropertySheets: DocumentPropertySheets::<Identity, Impl, OFFSET>,
            DevicePropertySheets: DevicePropertySheets::<Identity, Impl, OFFSET>,
            DevQueryPrintEx: DevQueryPrintEx::<Identity, Impl, OFFSET>,
            DeviceCapabilitiesA: DeviceCapabilitiesA::<Identity, Impl, OFFSET>,
            UpgradePrinter: UpgradePrinter::<Identity, Impl, OFFSET>,
            PrinterEvent: PrinterEvent::<Identity, Impl, OFFSET>,
            DriverEvent: DriverEvent::<Identity, Impl, OFFSET>,
            QueryColorProfile: QueryColorProfile::<Identity, Impl, OFFSET>,
            FontInstallerDlgProc: FontInstallerDlgProc::<Identity, Impl, OFFSET>,
            UpdateExternalFonts: UpdateExternalFonts::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPrintOemUI2_Impl: ::windows_core::BaseImpl + IPrintOemUI_Impl {
    fn QueryJobAttributes(this: &Self::This, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows_core::Result<()>;
    fn HideStandardUI(this: &Self::This, dwmode: u32) -> ::windows_core::Result<()>;
    fn DocumentEvent(this: &Self::This, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IPrintOemUI2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintOemUI);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintOemUI2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryJobAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, pdevmode: *const super::Gdi::DEVMODEA, dwlevel: u32, lpattributeinfo: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryJobAttributes(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&lpattributeinfo)).into())
        }
        unsafe extern "system" fn HideStandardUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HideStandardUI(this, ::core::mem::transmute_copy(&dwmode)).into())
        }
        unsafe extern "system" fn DocumentEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, hdc: super::Gdi::HDC, iesc: i32, cbin: u32, pvin: *mut ::core::ffi::c_void, cbout: u32, pvout: *mut ::core::ffi::c_void, piresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DocumentEvent(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&iesc), ::core::mem::transmute_copy(&cbin), ::core::mem::transmute_copy(&pvin), ::core::mem::transmute_copy(&cbout), ::core::mem::transmute_copy(&pvout), ::core::mem::transmute_copy(&piresult)).into())
        }
        IPrintOemUI2_Vtbl {
            base__: <IPrintOemUI as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryJobAttributes: QueryJobAttributes::<Identity, Impl, OFFSET>,
            HideStandardUI: HideStandardUI::<Identity, Impl, OFFSET>,
            DocumentEvent: DocumentEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintOemUIMXDC_Impl: ::windows_core::BaseImpl {
    fn AdjustImageableArea(this: &Self::This, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows_core::Result<()>;
    fn AdjustImageCompression(this: &Self::This, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows_core::Result<()>;
    fn AdjustDPI(this: &Self::This, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IPrintOemUIMXDC {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintOemUIMXDC {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdjustImageableArea<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, prclimageablearea: *mut super::super::Foundation::RECTL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdjustImageableArea(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&prclimageablearea)).into())
        }
        unsafe extern "system" fn AdjustImageCompression<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pcompressionmode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdjustImageCompression(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pcompressionmode)).into())
        }
        unsafe extern "system" fn AdjustDPI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOemUIMXDC_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, cbdevmode: u32, pdevmode: *const super::Gdi::DEVMODEA, cboemdm: u32, poemdm: *const ::core::ffi::c_void, pdpi: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdjustDPI(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&cboemdm), ::core::mem::transmute_copy(&poemdm), ::core::mem::transmute_copy(&pdpi)).into())
        }
        IPrintOemUIMXDC_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdjustImageableArea: AdjustImageableArea::<Identity, Impl, OFFSET>,
            AdjustImageCompression: AdjustImageCompression::<Identity, Impl, OFFSET>,
            AdjustDPI: AdjustDPI::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintPipelineFilter_Impl: ::windows_core::BaseImpl {
    fn InitializeFilter(this: &Self::This, pinegotiation: ::core::option::Option<&IInterFilterCommunicator>, pipropertybag: ::core::option::Option<&IPrintPipelinePropertyBag>, pipipelinecontrol: ::core::option::Option<&IPrintPipelineManagerControl>) -> ::windows_core::Result<()>;
    fn ShutdownOperation(this: &Self::This) -> ::windows_core::Result<()>;
    fn StartOperation(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintPipelineFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintPipelineFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinegotiation: *mut ::core::ffi::c_void, pipropertybag: *mut ::core::ffi::c_void, pipipelinecontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFilter(this, ::windows_core::from_raw_borrowed(&pinegotiation), ::windows_core::from_raw_borrowed(&pipropertybag), ::windows_core::from_raw_borrowed(&pipipelinecontrol)).into())
        }
        unsafe extern "system" fn ShutdownOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownOperation(this).into())
        }
        unsafe extern "system" fn StartOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartOperation(this).into())
        }
        IPrintPipelineFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeFilter: InitializeFilter::<Identity, Impl, OFFSET>,
            ShutdownOperation: ShutdownOperation::<Identity, Impl, OFFSET>,
            StartOperation: StartOperation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintPipelineManagerControl_Impl: ::windows_core::BaseImpl {
    fn RequestShutdown(this: &Self::This, hrreason: ::windows_core::HRESULT, preason: ::core::option::Option<&IImgErrorInfo>) -> ::windows_core::Result<()>;
    fn FilterFinished(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IPrintPipelineManagerControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineManagerControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintPipelineManagerControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestShutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineManagerControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrreason: ::windows_core::HRESULT, preason: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestShutdown(this, ::core::mem::transmute_copy(&hrreason), ::windows_core::from_raw_borrowed(&preason)).into())
        }
        unsafe extern "system" fn FilterFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineManagerControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FilterFinished(this).into())
        }
        IPrintPipelineManagerControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestShutdown: RequestShutdown::<Identity, Impl, OFFSET>,
            FilterFinished: FilterFinished::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintPipelineProgressReport_Impl: ::windows_core::BaseImpl {
    fn ReportProgress(this: &Self::This, update: EXpsJobConsumption) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintPipelineProgressReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineProgressReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintPipelineProgressReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelineProgressReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, update: EXpsJobConsumption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportProgress(this, ::core::mem::transmute_copy(&update)).into())
        }
        IPrintPipelineProgressReport_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ReportProgress: ReportProgress::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintPipelinePropertyBag_Impl: ::windows_core::BaseImpl {
    fn AddProperty(this: &Self::This, pszname: &::windows_core::PCWSTR, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DeleteProperty(this: &Self::This, pszname: &::windows_core::PCWSTR) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintPipelinePropertyBag {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintPipelinePropertyBag {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddProperty(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pvar)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPipelinePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteProperty(this, ::core::mem::transmute(&pszname)))
        }
        IPrintPipelinePropertyBag_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddProperty: AddProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait IPrintPreviewDxgiPackageTarget_Impl: ::windows_core::BaseImpl {
    fn SetJobPageCount(this: &Self::This, counttype: PageCountType, count: u32) -> ::windows_core::Result<()>;
    fn DrawPage(this: &Self::This, jobpagenumber: u32, pageimage: ::core::option::Option<&super::Dxgi::IDXGISurface>, dpix: f32, dpiy: f32) -> ::windows_core::Result<()>;
    fn InvalidatePreview(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::windows_core::Iids for IPrintPreviewDxgiPackageTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintPreviewDxgiPackageTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetJobPageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, counttype: PageCountType, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJobPageCount(this, ::core::mem::transmute_copy(&counttype), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn DrawPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, pageimage: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawPage(this, ::core::mem::transmute_copy(&jobpagenumber), ::windows_core::from_raw_borrowed(&pageimage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into())
        }
        unsafe extern "system" fn InvalidatePreview<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintPreviewDxgiPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidatePreview(this).into())
        }
        IPrintPreviewDxgiPackageTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetJobPageCount: SetJobPageCount::<Identity, Impl, OFFSET>,
            DrawPage: DrawPage::<Identity, Impl, OFFSET>,
            InvalidatePreview: InvalidatePreview::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintReadStream_Impl: ::windows_core::BaseImpl {
    fn Seek(this: &Self::This, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::Result<()>;
    fn ReadBytes(this: &Self::This, pvbuffer: *mut ::core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPrintReadStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintReadStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintReadStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintReadStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin), ::core::mem::transmute_copy(&plibnewposition)).into())
        }
        unsafe extern "system" fn ReadBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintReadStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbrequested: u32, pcbread: *mut u32, pbendoffile: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadBytes(this, ::core::mem::transmute_copy(&pvbuffer), ::core::mem::transmute_copy(&cbrequested), ::core::mem::transmute_copy(&pcbread), ::core::mem::transmute_copy(&pbendoffile)).into())
        }
        IPrintReadStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Seek: Seek::<Identity, Impl, OFFSET>,
            ReadBytes: ReadBytes::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintReadStreamFactory_Impl: ::windows_core::BaseImpl {
    fn GetStream(this: &Self::This) -> ::windows_core::Result<IPrintReadStream>;
}
impl ::windows_core::Iids for IPrintReadStreamFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintReadStreamFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintReadStreamFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintReadStreamFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintReadStreamFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetStream: GetStream::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaAsyncOperation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaAsyncOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaAsyncOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IPrintSchemaAsyncOperation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaAsyncOperationEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Completed(this: &Self::This, pticket: ::core::option::Option<&IPrintSchemaTicket>, hroperation: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaAsyncOperationEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaAsyncOperationEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Completed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaAsyncOperationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pticket: *mut ::core::ffi::c_void, hroperation: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Completed(this, ::windows_core::from_raw_borrowed(&pticket), ::core::mem::transmute_copy(&hroperation)).into())
        }
        IPrintSchemaAsyncOperationEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Completed: Completed::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaCapabilities_Impl: ::windows_core::BaseImpl + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(this: &Self::This, bstrkeyname: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(this: &Self::This, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn PageImageableSize(this: &Self::This) -> ::windows_core::Result<IPrintSchemaPageImageableSize>;
    fn JobCopiesAllDocumentsMinValue(this: &Self::This) -> ::windows_core::Result<u32>;
    fn JobCopiesAllDocumentsMaxValue(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSelectedOptionInPrintTicket(this: &Self::This, pfeature: ::core::option::Option<&IPrintSchemaFeature>) -> ::windows_core::Result<IPrintSchemaOption>;
    fn GetOptions(this: &Self::This, pfeature: ::core::option::Option<&IPrintSchemaFeature>) -> ::windows_core::Result<IPrintSchemaOptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFeatureByKeyName(this, ::core::mem::transmute(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFeature(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PageImageableSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppageimageablesize: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PageImageableSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppageimageablesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMinValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsminvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JobCopiesAllDocumentsMinValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljobcopiesalldocumentsminvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JobCopiesAllDocumentsMaxValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocumentsmaxvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JobCopiesAllDocumentsMaxValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljobcopiesalldocumentsmaxvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSelectedOptionInPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfeature: *mut ::core::ffi::c_void, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelectedOptionInPrintTicket(this, ::windows_core::from_raw_borrowed(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfeature: *mut ::core::ffi::c_void, ppoptioncollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptions(this, ::windows_core::from_raw_borrowed(&pfeature)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptioncollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaCapabilities_Vtbl {
            base__: <IPrintSchemaElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, Impl, OFFSET>,
            GetFeature: GetFeature::<Identity, Impl, OFFSET>,
            PageImageableSize: PageImageableSize::<Identity, Impl, OFFSET>,
            JobCopiesAllDocumentsMinValue: JobCopiesAllDocumentsMinValue::<Identity, Impl, OFFSET>,
            JobCopiesAllDocumentsMaxValue: JobCopiesAllDocumentsMaxValue::<Identity, Impl, OFFSET>,
            GetSelectedOptionInPrintTicket: GetSelectedOptionInPrintTicket::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaCapabilities2_Impl: ::windows_core::BaseImpl + IPrintSchemaCapabilities_Impl {
    fn GetParameterDefinition(this: &Self::This, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaParameterDefinition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaCapabilities2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaCapabilities);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaCapabilities2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParameterDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaCapabilities2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppparameterdefinition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParameterDefinition(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparameterdefinition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaCapabilities2_Vtbl {
            base__: <IPrintSchemaCapabilities as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParameterDefinition: GetParameterDefinition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaDisplayableElement_Impl: ::windows_core::BaseImpl + IPrintSchemaElement_Impl {
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaDisplayableElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaDisplayableElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaDisplayableElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaDisplayableElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaDisplayableElement_Vtbl { base__: <IPrintSchemaElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DisplayName: DisplayName::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaElement_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn XmlNode(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NamespaceUri(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn XmlNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::XmlNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxmlnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NamespaceUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnamespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamespaceUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnamespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaElement_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            XmlNode: XmlNode::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaFeature_Impl: ::windows_core::BaseImpl + IPrintSchemaDisplayableElement_Impl {
    fn SelectedOption(this: &Self::This) -> ::windows_core::Result<IPrintSchemaOption>;
    fn SetSelectedOption(this: &Self::This, poption: ::core::option::Option<&IPrintSchemaOption>) -> ::windows_core::Result<()>;
    fn SelectionType(this: &Self::This) -> ::windows_core::Result<PrintSchemaSelectionType>;
    fn GetOption(this: &Self::This, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaOption>;
    fn DisplayUI(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaFeature {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaDisplayableElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaFeature {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SelectedOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectedOption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSelectedOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poption: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelectedOption(this, ::windows_core::from_raw_borrowed(&poption)).into())
        }
        unsafe extern "system" fn SelectionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pselectiontype: *mut PrintSchemaSelectionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pselectiontype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOption(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaFeature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayUI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbshow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaFeature_Vtbl {
            base__: <IPrintSchemaDisplayableElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SelectedOption: SelectedOption::<Identity, Impl, OFFSET>,
            SetSelectedOption: SetSelectedOption::<Identity, Impl, OFFSET>,
            SelectionType: SelectionType::<Identity, Impl, OFFSET>,
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaNUpOption_Impl: ::windows_core::BaseImpl + IPrintSchemaOption_Impl {
    fn PagesPerSheet(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaNUpOption {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaOption);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaNUpOption_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaNUpOption {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PagesPerSheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaNUpOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulpagespersheet: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PagesPerSheet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpagespersheet, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaNUpOption_Vtbl { base__: <IPrintSchemaOption as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, PagesPerSheet: PagesPerSheet::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaOption_Impl: ::windows_core::BaseImpl + IPrintSchemaDisplayableElement_Impl {
    fn Selected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Constrained(this: &Self::This) -> ::windows_core::Result<PrintSchemaConstrainedSetting>;
    fn GetPropertyValue(this: &Self::This, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaOption {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaDisplayableElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaOption {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Selected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Selected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Constrained<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psetting: *mut PrintSchemaConstrainedSetting) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Constrained(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psetting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppxmlvaluenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyValue(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxmlvaluenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaOption_Vtbl {
            base__: <IPrintSchemaDisplayableElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Selected: Selected::<Identity, Impl, OFFSET>,
            Constrained: Constrained::<Identity, Impl, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaOptionCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, ulindex: u32) -> ::windows_core::Result<IPrintSchemaOption>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaOptionCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaOptionCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppoption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaOptionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaOptionCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaPageImageableSize_Impl: ::windows_core::BaseImpl + IPrintSchemaElement_Impl {
    fn ImageableSizeWidthInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ImageableSizeHeightInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OriginWidthInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OriginHeightInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ExtentWidthInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ExtentHeightInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaPageImageableSize {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaPageImageableSize {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ImageableSizeWidthInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulimageablesizewidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageableSizeWidthInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulimageablesizewidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImageableSizeHeightInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulimageablesizeheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageableSizeHeightInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulimageablesizeheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OriginWidthInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puloriginwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OriginWidthInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puloriginwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OriginHeightInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puloriginheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OriginHeightInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puloriginheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtentWidthInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulextentwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtentWidthInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulextentwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtentHeightInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageImageableSize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulextentheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtentHeightInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulextentheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaPageImageableSize_Vtbl {
            base__: <IPrintSchemaElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ImageableSizeWidthInMicrons: ImageableSizeWidthInMicrons::<Identity, Impl, OFFSET>,
            ImageableSizeHeightInMicrons: ImageableSizeHeightInMicrons::<Identity, Impl, OFFSET>,
            OriginWidthInMicrons: OriginWidthInMicrons::<Identity, Impl, OFFSET>,
            OriginHeightInMicrons: OriginHeightInMicrons::<Identity, Impl, OFFSET>,
            ExtentWidthInMicrons: ExtentWidthInMicrons::<Identity, Impl, OFFSET>,
            ExtentHeightInMicrons: ExtentHeightInMicrons::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaPageMediaSizeOption_Impl: ::windows_core::BaseImpl + IPrintSchemaOption_Impl {
    fn WidthInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
    fn HeightInMicrons(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaPageMediaSizeOption {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaOption);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaPageMediaSizeOption {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WidthInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WidthInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HeightInMicrons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaPageMediaSizeOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HeightInMicrons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaPageMediaSizeOption_Vtbl {
            base__: <IPrintSchemaOption as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WidthInMicrons: WidthInMicrons::<Identity, Impl, OFFSET>,
            HeightInMicrons: HeightInMicrons::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaParameterDefinition_Impl: ::windows_core::BaseImpl + IPrintSchemaDisplayableElement_Impl {
    fn UserInputRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn UnitType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DataType(this: &Self::This) -> ::windows_core::Result<PrintSchemaParameterDataType>;
    fn RangeMin(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RangeMax(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaParameterDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaDisplayableElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaParameterDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UserInputRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisrequired: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserInputRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisrequired, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnitType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrunittype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnitType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrunittype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatatype: *mut PrintSchemaParameterDataType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RangeMin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangemin: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeMin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prangemin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RangeMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangemax: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeMax(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prangemax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaParameterDefinition_Vtbl {
            base__: <IPrintSchemaDisplayableElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UserInputRequired: UserInputRequired::<Identity, Impl, OFFSET>,
            UnitType: UnitType::<Identity, Impl, OFFSET>,
            DataType: DataType::<Identity, Impl, OFFSET>,
            RangeMin: RangeMin::<Identity, Impl, OFFSET>,
            RangeMax: RangeMax::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaParameterInitializer_Impl: ::windows_core::BaseImpl + IPrintSchemaElement_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(this: &Self::This, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaParameterInitializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaParameterInitializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaParameterInitializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&pvar)).into())
        }
        IPrintSchemaParameterInitializer_Vtbl {
            base__: <IPrintSchemaElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaTicket_Impl: ::windows_core::BaseImpl + IPrintSchemaElement_Impl {
    fn GetFeatureByKeyName(this: &Self::This, bstrkeyname: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn GetFeature(this: &Self::This, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaFeature>;
    fn ValidateAsync(this: &Self::This) -> ::windows_core::Result<IPrintSchemaAsyncOperation>;
    fn CommitAsync(this: &Self::This, pprintticketcommit: ::core::option::Option<&IPrintSchemaTicket>) -> ::windows_core::Result<IPrintSchemaAsyncOperation>;
    fn NotifyXmlChanged(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCapabilities(this: &Self::This) -> ::windows_core::Result<IPrintSchemaCapabilities>;
    fn JobCopiesAllDocuments(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetJobCopiesAllDocuments(this: &Self::This, uljobcopiesalldocuments: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaTicket {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaTicket {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFeatureByKeyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFeatureByKeyName(this, ::core::mem::transmute(&bstrkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfeature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFeature(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValidateAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValidateAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommitAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticketcommit: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitAsync(this, ::windows_core::from_raw_borrowed(&pprintticketcommit)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyXmlChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyXmlChanged(this).into())
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JobCopiesAllDocuments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puljobcopiesalldocuments: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JobCopiesAllDocuments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljobcopiesalldocuments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJobCopiesAllDocuments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uljobcopiesalldocuments: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJobCopiesAllDocuments(this, ::core::mem::transmute_copy(&uljobcopiesalldocuments)).into())
        }
        IPrintSchemaTicket_Vtbl {
            base__: <IPrintSchemaElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFeatureByKeyName: GetFeatureByKeyName::<Identity, Impl, OFFSET>,
            GetFeature: GetFeature::<Identity, Impl, OFFSET>,
            ValidateAsync: ValidateAsync::<Identity, Impl, OFFSET>,
            CommitAsync: CommitAsync::<Identity, Impl, OFFSET>,
            NotifyXmlChanged: NotifyXmlChanged::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            JobCopiesAllDocuments: JobCopiesAllDocuments::<Identity, Impl, OFFSET>,
            SetJobCopiesAllDocuments: SetJobCopiesAllDocuments::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintSchemaTicket2_Impl: ::windows_core::BaseImpl + IPrintSchemaTicket_Impl {
    fn GetParameterInitializer(this: &Self::This, bstrname: &::windows_core::BSTR, bstrnamespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IPrintSchemaParameterInitializer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintSchemaTicket2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintSchemaTicket);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintSchemaTicket2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParameterInitializer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintSchemaTicket2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnamespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppparameterinitializer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParameterInitializer(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrnamespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparameterinitializer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintSchemaTicket2_Vtbl {
            base__: <IPrintSchemaTicket as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParameterInitializer: GetParameterInitializer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider_Impl: ::windows_core::BaseImpl {
    fn GetSupportedVersions(this: &Self::This, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows_core::Result<()>;
    fn BindPrinter(this: &Self::This, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryDeviceNamespace(this: &Self::This, pdefaultnamespace: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ConvertPrintTicketToDevMode(this: &Self::This, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows_core::Result<()>;
    fn ConvertDevModeToPrintTicket(this: &Self::This, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<()>;
    fn GetPrintCapabilities(this: &Self::This, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn ValidatePrintTicket(this: &Self::This, pbaseticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPrintTicketProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTicketProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSupportedVersions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, ppversions: *mut *mut i32, cversions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSupportedVersions(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&ppversions), ::core::mem::transmute_copy(&cversions)).into())
        }
        unsafe extern "system" fn BindPrinter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hprinter: super::super::Foundation::HANDLE, version: i32, poptions: *mut SHIMOPTS, pdevmodeflags: *mut u32, cnamespaces: *mut i32, ppnamespaces: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindPrinter(this, ::core::mem::transmute_copy(&hprinter), ::core::mem::transmute_copy(&version), ::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&pdevmodeflags), ::core::mem::transmute_copy(&cnamespaces), ::core::mem::transmute_copy(&ppnamespaces)).into())
        }
        unsafe extern "system" fn QueryDeviceNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdefaultnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryDeviceNamespace(this, ::core::mem::transmute_copy(&pdefaultnamespace)).into())
        }
        unsafe extern "system" fn ConvertPrintTicketToDevMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, cbdevmodein: u32, pdevmodein: *mut super::Gdi::DEVMODEA, pcbdevmodeout: *mut u32, ppdevmodeout: *mut *mut super::Gdi::DEVMODEA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertPrintTicketToDevMode(this, ::windows_core::from_raw_borrowed(&pprintticket), ::core::mem::transmute_copy(&cbdevmodein), ::core::mem::transmute_copy(&pdevmodein), ::core::mem::transmute_copy(&pcbdevmodeout), ::core::mem::transmute_copy(&ppdevmodeout)).into())
        }
        unsafe extern "system" fn ConvertDevModeToPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbdevmode: u32, pdevmode: *mut super::Gdi::DEVMODEA, pprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertDevModeToPrintTicket(this, ::core::mem::transmute_copy(&cbdevmode), ::core::mem::transmute_copy(&pdevmode), ::windows_core::from_raw_borrowed(&pprintticket)).into())
        }
        unsafe extern "system" fn GetPrintCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintCapabilities(this, ::windows_core::from_raw_borrowed(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValidatePrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbaseticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidatePrintTicket(this, ::windows_core::from_raw_borrowed(&pbaseticket)).into())
        }
        IPrintTicketProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSupportedVersions: GetSupportedVersions::<Identity, Impl, OFFSET>,
            BindPrinter: BindPrinter::<Identity, Impl, OFFSET>,
            QueryDeviceNamespace: QueryDeviceNamespace::<Identity, Impl, OFFSET>,
            ConvertPrintTicketToDevMode: ConvertPrintTicketToDevMode::<Identity, Impl, OFFSET>,
            ConvertDevModeToPrintTicket: ConvertDevModeToPrintTicket::<Identity, Impl, OFFSET>,
            GetPrintCapabilities: GetPrintCapabilities::<Identity, Impl, OFFSET>,
            ValidatePrintTicket: ValidatePrintTicket::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPrintTicketProvider2_Impl: ::windows_core::BaseImpl + IPrintTicketProvider_Impl {
    fn GetPrintDeviceCapabilities(this: &Self::This, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
    fn GetPrintDeviceResources(this: &Self::This, pszlocalename: &::windows_core::PCWSTR, pprintticket: ::core::option::Option<&super::super::Data::Xml::MsXml::IXMLDOMDocument2>) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument2>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPrintTicketProvider2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintTicketProvider);
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTicketProvider2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrintDeviceCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintticket: *mut ::core::ffi::c_void, ppdevicecapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintDeviceCapabilities(this, ::windows_core::from_raw_borrowed(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevicecapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrintDeviceResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTicketProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlocalename: ::windows_core::PCWSTR, pprintticket: *mut ::core::ffi::c_void, ppdeviceresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintDeviceResources(this, ::core::mem::transmute(&pszlocalename), ::windows_core::from_raw_borrowed(&pprintticket)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdeviceresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintTicketProvider2_Vtbl {
            base__: <IPrintTicketProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrintDeviceCapabilities: GetPrintDeviceCapabilities::<Identity, Impl, OFFSET>,
            GetPrintDeviceResources: GetPrintDeviceResources::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintUnidiAsyncNotifyRegistration_Impl: ::windows_core::BaseImpl + IPrintAsyncNotifyRegistration_Impl {
    fn AsyncGetNotification(this: &Self::This, param0: ::core::option::Option<&IAsyncGetSendNotificationCookie>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintUnidiAsyncNotifyRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintAsyncNotifyRegistration);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintUnidiAsyncNotifyRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncGetNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintUnidiAsyncNotifyRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncGetNotification(this, ::windows_core::from_raw_borrowed(&param0)).into())
        }
        IPrintUnidiAsyncNotifyRegistration_Vtbl {
            base__: <IPrintAsyncNotifyRegistration as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncGetNotification: AsyncGetNotification::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintWriteStream_Impl: ::windows_core::BaseImpl {
    fn WriteBytes(this: &Self::This, pvbuffer: *const ::core::ffi::c_void, cbbuffer: u32) -> ::windows_core::Result<u32>;
    fn Close(this: &Self::This);
}
impl ::windows_core::Iids for IPrintWriteStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWriteStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintWriteStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWriteStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteBytes(this, ::core::mem::transmute_copy(&pvbuffer), ::core::mem::transmute_copy(&cbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWriteStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this))
        }
        IPrintWriteStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteBytes: WriteBytes::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintWriteStreamFlush_Impl: ::windows_core::BaseImpl {
    fn FlushData(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintWriteStreamFlush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWriteStreamFlush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintWriteStreamFlush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FlushData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWriteStreamFlush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushData(this).into())
        }
        IPrintWriteStreamFlush_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FlushData: FlushData::<Identity, Impl, OFFSET> }
    };
}
pub trait IPrinterBidiSetRequestCallback_Impl: ::windows_core::BaseImpl {
    fn Completed(this: &Self::This, bstrresponse: &::windows_core::BSTR, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrinterBidiSetRequestCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterBidiSetRequestCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterBidiSetRequestCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Completed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterBidiSetRequestCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresponse: ::std::mem::MaybeUninit<::windows_core::BSTR>, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Completed(this, ::core::mem::transmute(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        IPrinterBidiSetRequestCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Completed: Completed::<Identity, Impl, OFFSET> }
    };
}
pub trait IPrinterExtensionAsyncOperation_Impl: ::windows_core::BaseImpl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrinterExtensionAsyncOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionAsyncOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterExtensionAsyncOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IPrinterExtensionAsyncOperation_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Cancel: Cancel::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionContext_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn PrinterQueue(this: &Self::This) -> ::windows_core::Result<IPrinterQueue>;
    fn PrintSchemaTicket(this: &Self::This) -> ::windows_core::Result<IPrintSchemaTicket>;
    fn DriverProperties(this: &Self::This) -> ::windows_core::Result<IPrinterPropertyBag>;
    fn UserProperties(this: &Self::This) -> ::windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterExtensionContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterExtensionContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrinterQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrinterQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrintSchemaTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppticket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrintSchemaTicket(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppticket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterExtensionContext_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrinterQueue: PrinterQueue::<Identity, Impl, OFFSET>,
            PrintSchemaTicket: PrintSchemaTicket::<Identity, Impl, OFFSET>,
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionContextCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, ulindex: u32) -> ::windows_core::Result<IPrinterExtensionContext>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterExtensionContextCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterExtensionContextCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionContextCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterExtensionContextCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OnDriverEvent(this: &Self::This, peventargs: ::core::option::Option<&IPrinterExtensionEventArgs>) -> ::windows_core::Result<()>;
    fn OnPrinterQueuesEnumerated(this: &Self::This, pcontextcollection: ::core::option::Option<&IPrinterExtensionContextCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterExtensionEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterExtensionEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDriverEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDriverEvent(this, ::windows_core::from_raw_borrowed(&peventargs)).into())
        }
        unsafe extern "system" fn OnPrinterQueuesEnumerated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontextcollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPrinterQueuesEnumerated(this, ::windows_core::from_raw_borrowed(&pcontextcollection)).into())
        }
        IPrinterExtensionEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDriverEvent: OnDriverEvent::<Identity, Impl, OFFSET>,
            OnPrinterQueuesEnumerated: OnPrinterQueuesEnumerated::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionEventArgs_Impl: ::windows_core::BaseImpl + IPrinterExtensionContext_Impl {
    fn BidiNotification(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ReasonId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Request(this: &Self::This) -> ::windows_core::Result<IPrinterExtensionRequest>;
    fn SourceApplication(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DetailedReasonId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn WindowModal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn WindowParent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterExtensionEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrinterExtensionContext);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterExtensionEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BidiNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbidinotification: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BidiNotification(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbidinotification, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReasonId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preasonid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReasonId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preasonid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Request<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Request(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SourceApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrapplication: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourceApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DetailedReasonId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdetailedreasonid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DetailedReasonId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdetailedreasonid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WindowModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmodal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WindowModal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmodal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WindowParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WindowParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwndparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterExtensionEventArgs_Vtbl {
            base__: <IPrinterExtensionContext as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BidiNotification: BidiNotification::<Identity, Impl, OFFSET>,
            ReasonId: ReasonId::<Identity, Impl, OFFSET>,
            Request: Request::<Identity, Impl, OFFSET>,
            SourceApplication: SourceApplication::<Identity, Impl, OFFSET>,
            DetailedReasonId: DetailedReasonId::<Identity, Impl, OFFSET>,
            WindowModal: WindowModal::<Identity, Impl, OFFSET>,
            WindowParent: WindowParent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrinterExtensionManager_Impl: ::windows_core::BaseImpl {
    fn EnableEvents(this: &Self::This, printerdriverid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DisableEvents(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrinterExtensionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterExtensionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printerdriverid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableEvents(this, ::core::mem::transmute(&printerdriverid)).into())
        }
        unsafe extern "system" fn DisableEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableEvents(this).into())
        }
        IPrinterExtensionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableEvents: EnableEvents::<Identity, Impl, OFFSET>,
            DisableEvents: DisableEvents::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterExtensionRequest_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Cancel(this: &Self::This, hrstatus: ::windows_core::HRESULT, bstrlogmessage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Complete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterExtensionRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterExtensionRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, bstrlogmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this, ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute(&bstrlogmessage)).into())
        }
        unsafe extern "system" fn Complete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterExtensionRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Complete(this).into())
        }
        IPrinterExtensionRequest_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterPropertyBag_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetBool(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBool(this: &Self::This, bstrname: &::windows_core::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetInt32(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn SetInt32(this: &Self::This, bstrname: &::windows_core::BSTR, nvalue: i32) -> ::windows_core::Result<()>;
    fn GetString(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetString(this: &Self::This, bstrname: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBytes(this: &Self::This, bstrname: &::windows_core::BSTR, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows_core::Result<()>;
    fn SetBytes(this: &Self::This, bstrname: &::windows_core::BSTR, cbvalue: u32, pvalue: *const u8) -> ::windows_core::Result<()>;
    fn GetReadStream(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn GetWriteStream(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterPropertyBag {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterPropertyBag {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBool(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBool(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&bvalue)).into())
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pnvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInt32(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInt32(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&nvalue)).into())
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetString(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into())
        }
        unsafe extern "system" fn GetBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcbvalue: *mut u32, ppvalue: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBytes(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&pcbvalue), ::core::mem::transmute_copy(&ppvalue)).into())
        }
        unsafe extern "system" fn SetBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, cbvalue: u32, pvalue: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBytes(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&cbvalue), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetReadStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReadStream(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWriteStream(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterPropertyBag_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBool: GetBool::<Identity, Impl, OFFSET>,
            SetBool: SetBool::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            SetInt32: SetInt32::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            GetBytes: GetBytes::<Identity, Impl, OFFSET>,
            SetBytes: SetBytes::<Identity, Impl, OFFSET>,
            GetReadStream: GetReadStream::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Handle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SendBidiQuery(this: &Self::This, bstrbidiquery: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetProperties(this: &Self::This) -> ::windows_core::Result<IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phprinter: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phprinter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendBidiQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbidiquery: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendBidiQuery(this, ::core::mem::transmute(&bstrbidiquery)).into())
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterQueue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Handle: Handle::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SendBidiQuery: SendBidiQuery::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueue2_Impl: ::windows_core::BaseImpl + IPrinterQueue_Impl {
    fn SendBidiSetRequestAsync(this: &Self::This, bstrbidirequest: &::windows_core::BSTR, pcallback: ::core::option::Option<&IPrinterBidiSetRequestCallback>) -> ::windows_core::Result<IPrinterExtensionAsyncOperation>;
    fn GetPrinterQueueView(this: &Self::This, ulviewoffset: u32, ulviewsize: u32) -> ::windows_core::Result<IPrinterQueueView>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterQueue2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrinterQueue);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterQueue2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendBidiSetRequestAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbidirequest: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcallback: *mut ::core::ffi::c_void, ppasyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendBidiSetRequestAsync(this, ::core::mem::transmute(&bstrbidirequest), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrinterQueueView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ppjobview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrinterQueueView(this, ::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppjobview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterQueue2_Vtbl {
            base__: <IPrinterQueue as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendBidiSetRequestAsync: SendBidiSetRequestAsync::<Identity, Impl, OFFSET>,
            GetPrinterQueueView: GetPrinterQueueView::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OnBidiResponseReceived(this: &Self::This, bstrresponse: &::windows_core::BSTR, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterQueueEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueueEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterQueueEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnBidiResponseReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueueEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrresponse: ::std::mem::MaybeUninit<::windows_core::BSTR>, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBidiResponseReceived(this, ::core::mem::transmute(&bstrresponse), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        IPrinterQueueEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnBidiResponseReceived: OnBidiResponseReceived::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueView_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetViewRange(this: &Self::This, ulviewoffset: u32, ulviewsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterQueueView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueueView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterQueueView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetViewRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueueView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewRange(this, ::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize)).into())
        }
        IPrinterQueueView_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetViewRange: SetViewRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterQueueViewEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OnChanged(this: &Self::This, pcollection: ::core::option::Option<&IPrintJobCollection>, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterQueueViewEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueueViewEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterQueueViewEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterQueueViewEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcollection: *mut ::core::ffi::c_void, ulviewoffset: u32, ulviewsize: u32, ulcountjobsinprintqueue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChanged(this, ::windows_core::from_raw_borrowed(&pcollection), ::core::mem::transmute_copy(&ulviewoffset), ::core::mem::transmute_copy(&ulviewsize), ::core::mem::transmute_copy(&ulcountjobsinprintqueue)).into())
        }
        IPrinterQueueViewEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnChanged: OnChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptContext_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DriverProperties(this: &Self::This) -> ::windows_core::Result<IPrinterScriptablePropertyBag>;
    fn QueueProperties(this: &Self::This) -> ::windows_core::Result<IPrinterScriptablePropertyBag>;
    fn UserProperties(this: &Self::This) -> ::windows_core::Result<IPrinterScriptablePropertyBag>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterScriptContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterScriptContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DriverProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterScriptContext_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            QueueProperties: QueueProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptablePropertyBag_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetBool(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBool(this: &Self::This, bstrname: &::windows_core::BSTR, bvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetInt32(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn SetInt32(this: &Self::This, bstrname: &::windows_core::BSTR, nvalue: i32) -> ::windows_core::Result<()>;
    fn GetString(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetString(this: &Self::This, bstrname: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBytes(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetBytes(this: &Self::This, bstrname: &::windows_core::BSTR, parray: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn GetReadStream(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IPrinterScriptableStream>;
    fn GetWriteStream(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IPrinterScriptableStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterScriptablePropertyBag {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterScriptablePropertyBag {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBool(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bvalue: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBool(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&bvalue)).into())
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pnvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInt32(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInt32(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&nvalue)).into())
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetString(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into())
        }
        unsafe extern "system" fn GetBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pparray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBytes(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, parray: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBytes(this, ::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&parray)).into())
        }
        unsafe extern "system" fn GetReadStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReadStream(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWriteStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWriteStream(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterScriptablePropertyBag_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBool: GetBool::<Identity, Impl, OFFSET>,
            SetBool: SetBool::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            SetInt32: SetInt32::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            GetBytes: GetBytes::<Identity, Impl, OFFSET>,
            SetBytes: SetBytes::<Identity, Impl, OFFSET>,
            GetReadStream: GetReadStream::<Identity, Impl, OFFSET>,
            GetWriteStream: GetWriteStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptablePropertyBag2_Impl: ::windows_core::BaseImpl + IPrinterScriptablePropertyBag_Impl {
    fn GetReadStreamAsXML(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterScriptablePropertyBag2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrinterScriptablePropertyBag);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterScriptablePropertyBag2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetReadStreamAsXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptablePropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppxmlnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReadStreamAsXML(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxmlnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterScriptablePropertyBag2_Vtbl {
            base__: <IPrinterScriptablePropertyBag as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetReadStreamAsXML: GetReadStreamAsXML::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptableSequentialStream_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Read(this: &Self::This, cbread: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Write(this: &Self::This, parray: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterScriptableSequentialStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterScriptableSequentialStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbread: i32, pparray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Read(this, ::core::mem::transmute_copy(&cbread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptableSequentialStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parray: *mut ::core::ffi::c_void, pcbwritten: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Write(this, ::windows_core::from_raw_borrowed(&parray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrinterScriptableSequentialStream_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrinterScriptableStream_Impl: ::windows_core::BaseImpl + IPrinterScriptableSequentialStream_Impl {
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK) -> ::windows_core::Result<i32>;
    fn SetSize(this: &Self::This, lsize: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrinterScriptableStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrinterScriptableSequentialStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinterScriptableStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loffset: i32, streamseek: super::super::System::Com::STREAM_SEEK, plposition: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Seek(this, ::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&streamseek)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinterScriptableStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, ::core::mem::transmute_copy(&lsize)).into())
        }
        IPrinterScriptableStream_Vtbl {
            base__: <IPrinterScriptableSequentialStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXpsDocument_Impl: ::windows_core::BaseImpl {
    fn GetThumbnail(this: &Self::This) -> ::windows_core::Result<IPartThumbnail>;
    fn SetThumbnail(this: &Self::This, pthumbnail: ::core::option::Option<&IPartThumbnail>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppthumbnail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnail(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppthumbnail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pthumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThumbnail(this, ::windows_core::from_raw_borrowed(&pthumbnail)).into())
        }
        IXpsDocument_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXpsDocumentConsumer_Impl: ::windows_core::BaseImpl {
    fn SendXpsUnknown(this: &Self::This, punknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SendXpsDocument(this: &Self::This, pixpsdocument: ::core::option::Option<&IXpsDocument>) -> ::windows_core::Result<()>;
    fn SendFixedDocumentSequence(this: &Self::This, pifixeddocumentsequence: ::core::option::Option<&IFixedDocumentSequence>) -> ::windows_core::Result<()>;
    fn SendFixedDocument(this: &Self::This, pifixeddocument: ::core::option::Option<&IFixedDocument>) -> ::windows_core::Result<()>;
    fn SendFixedPage(this: &Self::This, pifixedpage: ::core::option::Option<&IFixedPage>) -> ::windows_core::Result<()>;
    fn CloseSender(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetNewEmptyPart(this: &Self::This, uri: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void, ppwritestream: *mut ::core::option::Option<IPrintWriteStream>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsDocumentConsumer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsDocumentConsumer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendXpsUnknown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendXpsUnknown(this, ::windows_core::from_raw_borrowed(&punknown)).into())
        }
        unsafe extern "system" fn SendXpsDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixpsdocument: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendXpsDocument(this, ::windows_core::from_raw_borrowed(&pixpsdocument)).into())
        }
        unsafe extern "system" fn SendFixedDocumentSequence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifixeddocumentsequence: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendFixedDocumentSequence(this, ::windows_core::from_raw_borrowed(&pifixeddocumentsequence)).into())
        }
        unsafe extern "system" fn SendFixedDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifixeddocument: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendFixedDocument(this, ::windows_core::from_raw_borrowed(&pifixeddocument)).into())
        }
        unsafe extern "system" fn SendFixedPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifixedpage: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendFixedPage(this, ::windows_core::from_raw_borrowed(&pifixedpage)).into())
        }
        unsafe extern "system" fn CloseSender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseSender(this).into())
        }
        unsafe extern "system" fn GetNewEmptyPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentConsumer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppnewobject: *mut *mut ::core::ffi::c_void, ppwritestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNewEmptyPart(this, ::core::mem::transmute(&uri), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppnewobject), ::core::mem::transmute_copy(&ppwritestream)).into())
        }
        IXpsDocumentConsumer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendXpsUnknown: SendXpsUnknown::<Identity, Impl, OFFSET>,
            SendXpsDocument: SendXpsDocument::<Identity, Impl, OFFSET>,
            SendFixedDocumentSequence: SendFixedDocumentSequence::<Identity, Impl, OFFSET>,
            SendFixedDocument: SendFixedDocument::<Identity, Impl, OFFSET>,
            SendFixedPage: SendFixedPage::<Identity, Impl, OFFSET>,
            CloseSender: CloseSender::<Identity, Impl, OFFSET>,
            GetNewEmptyPart: GetNewEmptyPart::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXpsDocumentProvider_Impl: ::windows_core::BaseImpl {
    fn GetXpsPart(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IXpsDocumentProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsDocumentProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXpsPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppixpspart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXpsPart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpspart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsDocumentProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetXpsPart: GetXpsPart::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsPartIterator_Impl: ::windows_core::BaseImpl {
    fn Reset(this: &Self::This);
    fn Current(this: &Self::This, puri: *mut ::windows_core::BSTR, ppxpspart: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn IsDone(this: &Self::This) -> super::super::Foundation::BOOL;
    fn Next(this: &Self::This);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXpsPartIterator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsPartIterator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this))
        }
        unsafe extern "system" fn Current<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppxpspart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Current(this, ::core::mem::transmute_copy(&puri), ::core::mem::transmute_copy(&ppxpspart)).into())
        }
        unsafe extern "system" fn IsDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDone(this))
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPartIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this))
        }
        IXpsPartIterator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Current: Current::<Identity, Impl, OFFSET>,
            IsDone: IsDone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory_Impl: ::windows_core::BaseImpl {
    fn CreateRasterizer(this: &Self::This, xpspage: ::core::option::Option<&super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE) -> ::windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::Iids for IXpsRasterizationFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_Xps")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizationFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsRasterizationFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizationFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRasterizer(this, ::windows_core::from_raw_borrowed(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpsrasterizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsRasterizationFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory1_Impl: ::windows_core::BaseImpl {
    fn CreateRasterizer(this: &Self::This, xpspage: ::core::option::Option<&super::super::Storage::Xps::IXpsOMPage>, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT) -> ::windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::Iids for IXpsRasterizationFactory1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_Xps")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizationFactory1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsRasterizationFactory1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizationFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpi: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRasterizer(this, ::windows_core::from_raw_borrowed(&xpspage), ::core::mem::transmute_copy(&dpi), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpsrasterizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsRasterizationFactory1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IXpsRasterizationFactory2_Impl: ::windows_core::BaseImpl {
    fn CreateRasterizer(this: &Self::This, xpspage: ::core::option::Option<&super::super::Storage::Xps::IXpsOMPage>, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR) -> ::windows_core::Result<IXpsRasterizer>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::Iids for IXpsRasterizationFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_Xps")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizationFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsRasterizationFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateRasterizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizationFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpspage: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32, nontextrenderingmode: XPSRAS_RENDERING_MODE, textrenderingmode: XPSRAS_RENDERING_MODE, pixelformat: XPSRAS_PIXEL_FORMAT, backgroundcolor: XPSRAS_BACKGROUND_COLOR, ppixpsrasterizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRasterizer(this, ::windows_core::from_raw_borrowed(&xpspage), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&nontextrenderingmode), ::core::mem::transmute_copy(&textrenderingmode), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&backgroundcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixpsrasterizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsRasterizationFactory2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateRasterizer: CreateRasterizer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Imaging\"`"]
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IXpsRasterizer_Impl: ::windows_core::BaseImpl {
    fn RasterizeRect(this: &Self::This, x: i32, y: i32, width: i32, height: i32, notificationcallback: ::core::option::Option<&IXpsRasterizerNotificationCallback>) -> ::windows_core::Result<super::Imaging::IWICBitmap>;
    fn SetMinimalLineWidth(this: &Self::This, width: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl ::windows_core::Iids for IXpsRasterizer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsRasterizer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RasterizeRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, height: i32, notificationcallback: *mut ::core::ffi::c_void, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RasterizeRect(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::windows_core::from_raw_borrowed(&notificationcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinimalLineWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinimalLineWidth(this, ::core::mem::transmute_copy(&width)).into())
        }
        IXpsRasterizer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RasterizeRect: RasterizeRect::<Identity, Impl, OFFSET>,
            SetMinimalLineWidth: SetMinimalLineWidth::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXpsRasterizerNotificationCallback_Impl: ::windows_core::BaseImpl {
    fn Continue(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsRasterizerNotificationCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizerNotificationCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsRasterizerNotificationCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Continue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsRasterizerNotificationCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Continue(this).into())
        }
        IXpsRasterizerNotificationCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Continue: Continue::<Identity, Impl, OFFSET> }
    };
}
