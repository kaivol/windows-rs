#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWinHttpRequest_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetProxy(this: &Self::This, proxysetting: i32, proxyserver: &super::super::System::Variant::VARIANT, bypasslist: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetCredentials(this: &Self::This, username: &::windows_core::BSTR, password: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, method: &::windows_core::BSTR, url: &::windows_core::BSTR, r#async: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetRequestHeader(this: &Self::This, header: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetResponseHeader(this: &Self::This, header: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAllResponseHeaders(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Send(this: &Self::This, body: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Status(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ResponseText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ResponseBody(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ResponseStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_Option(this: &Self::This, option: WinHttpRequestOption) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn put_Option(this: &Self::This, option: WinHttpRequestOption, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn WaitForResponse(this: &Self::This, timeout: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetTimeouts(this: &Self::This, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::Result<()>;
    fn SetClientCertificate(this: &Self::This, clientcertificate: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetAutoLogonPolicy(this: &Self::This, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWinHttpRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinHttpRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, proxysetting: i32, proxyserver: super::super::System::Variant::VARIANT, bypasslist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxy(this, ::core::mem::transmute_copy(&proxysetting), ::core::mem::transmute(&proxyserver), ::core::mem::transmute(&bypasslist)).into())
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentials(this, ::core::mem::transmute(&username), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: ::std::mem::MaybeUninit<::windows_core::BSTR>, url: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#async: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&method), ::core::mem::transmute(&url), ::core::mem::transmute(&r#async)).into())
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, header: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestHeader(this, ::core::mem::transmute(&header), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetResponseHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, header: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResponseHeader(this, ::core::mem::transmute(&header)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, headers: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllResponseHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(headers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, body: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::core::mem::transmute(&body)).into())
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, body: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(body, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseBody(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(body, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, body: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(body, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Option<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: WinHttpRequestOption, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Option(this, ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_Option<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: WinHttpRequestOption, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_Option(this, ::core::mem::transmute_copy(&option), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn WaitForResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: super::super::System::Variant::VARIANT, succeeded: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitForResponse(this, ::core::mem::transmute(&timeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(succeeded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn SetTimeouts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimeouts(this, ::core::mem::transmute_copy(&resolvetimeout), ::core::mem::transmute_copy(&connecttimeout), ::core::mem::transmute_copy(&sendtimeout), ::core::mem::transmute_copy(&receivetimeout)).into())
        }
        unsafe extern "system" fn SetClientCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientcertificate: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCertificate(this, ::core::mem::transmute(&clientcertificate)).into())
        }
        unsafe extern "system" fn SetAutoLogonPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, autologonpolicy: WinHttpRequestAutoLogonPolicy) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoLogonPolicy(this, ::core::mem::transmute_copy(&autologonpolicy)).into())
        }
        IWinHttpRequest_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProxy: SetProxy::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            GetResponseHeader: GetResponseHeader::<Identity, Impl, OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            ResponseText: ResponseText::<Identity, Impl, OFFSET>,
            ResponseBody: ResponseBody::<Identity, Impl, OFFSET>,
            ResponseStream: ResponseStream::<Identity, Impl, OFFSET>,
            get_Option: get_Option::<Identity, Impl, OFFSET>,
            put_Option: put_Option::<Identity, Impl, OFFSET>,
            WaitForResponse: WaitForResponse::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            SetTimeouts: SetTimeouts::<Identity, Impl, OFFSET>,
            SetClientCertificate: SetClientCertificate::<Identity, Impl, OFFSET>,
            SetAutoLogonPolicy: SetAutoLogonPolicy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWinHttpRequestEvents_Impl: ::windows_core::BaseImpl {
    fn OnResponseStart(this: &Self::This, status: i32, contenttype: &::windows_core::BSTR);
    fn OnResponseDataAvailable(this: &Self::This, data: *const *const super::super::System::Com::SAFEARRAY);
    fn OnResponseFinished(this: &Self::This);
    fn OnError(this: &Self::This, errornumber: i32, errordescription: &::windows_core::BSTR);
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWinHttpRequestEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinHttpRequestEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnResponseStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: i32, contenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResponseStart(this, ::core::mem::transmute_copy(&status), ::core::mem::transmute(&contenttype)))
        }
        unsafe extern "system" fn OnResponseDataAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *const *const super::super::System::Com::SAFEARRAY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResponseDataAvailable(this, ::core::mem::transmute_copy(&data)))
        }
        unsafe extern "system" fn OnResponseFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResponseFinished(this))
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinHttpRequestEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errornumber: i32, errordescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnError(this, ::core::mem::transmute_copy(&errornumber), ::core::mem::transmute(&errordescription)))
        }
        IWinHttpRequestEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnResponseStart: OnResponseStart::<Identity, Impl, OFFSET>,
            OnResponseDataAvailable: OnResponseDataAvailable::<Identity, Impl, OFFSET>,
            OnResponseFinished: OnResponseFinished::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    };
}
