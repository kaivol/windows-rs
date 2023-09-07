#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBindCallbackRedirect_Impl: ::windows_core::BaseImpl {
    fn Redirect(this: &Self::This, lpcurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBindCallbackRedirect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCallbackRedirect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindCallbackRedirect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Redirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindCallbackRedirect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcurl: ::windows_core::PCWSTR, vbcancel: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Redirect(this, ::core::mem::transmute(&lpcurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vbcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBindCallbackRedirect_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Redirect: Redirect::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBindHttpSecurity_Impl: ::windows_core::BaseImpl {
    fn GetIgnoreCertMask(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IBindHttpSecurity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindHttpSecurity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindHttpSecurity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIgnoreCertMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindHttpSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwignorecertmask: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIgnoreCertMask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwignorecertmask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBindHttpSecurity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIgnoreCertMask: GetIgnoreCertMask::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBindProtocol_Impl: ::windows_core::BaseImpl {
    fn CreateBinding(this: &Self::This, szurl: &::windows_core::PCWSTR, pbc: ::core::option::Option<&super::IBindCtx>) -> ::windows_core::Result<super::IBinding>;
}
impl ::windows_core::Iids for IBindProtocol {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindProtocol_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindProtocol {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBinding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szurl: ::windows_core::PCWSTR, pbc: *mut ::core::ffi::c_void, ppb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBinding(this, ::core::mem::transmute(&szurl), ::windows_core::from_raw_borrowed(&pbc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBindProtocol_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateBinding: CreateBinding::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICatalogFileInfo_Impl: ::windows_core::BaseImpl {
    fn GetCatalogFile(this: &Self::This) -> ::windows_core::Result<::windows_core::PSTR>;
    fn GetJavaTrust(this: &Self::This, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICatalogFileInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogFileInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICatalogFileInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCatalogFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogFileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcatalogfile: *mut ::windows_core::PSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCatalogFile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcatalogfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetJavaTrust<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogFileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetJavaTrust(this, ::core::mem::transmute_copy(&ppjavatrust)).into())
        }
        ICatalogFileInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCatalogFile: GetCatalogFile::<Identity, Impl, OFFSET>,
            GetJavaTrust: GetJavaTrust::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICodeInstall_Impl: ::windows_core::BaseImpl + IWindowForBindingUI_Impl {
    fn OnCodeInstallProblem(this: &Self::This, ulstatuscode: u32, szdestination: &::windows_core::PCWSTR, szsource: &::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICodeInstall {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWindowForBindingUI);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICodeInstall_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICodeInstall {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCodeInstallProblem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICodeInstall_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szdestination: ::windows_core::PCWSTR, szsource: ::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCodeInstallProblem(this, ::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute(&szdestination), ::core::mem::transmute(&szsource), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        ICodeInstall_Vtbl {
            base__: <IWindowForBindingUI as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCodeInstallProblem: OnCodeInstallProblem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDataFilter_Impl: ::windows_core::BaseImpl {
    fn DoEncode(this: &Self::This, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn DoDecode(this: &Self::This, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SetEncodingLevel(this: &Self::This, dwenclevel: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDataFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoEncode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoEncode(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn DoDecode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoDecode(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn SetEncodingLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwenclevel: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncodingLevel(this, ::core::mem::transmute_copy(&dwenclevel)).into())
        }
        IDataFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DoEncode: DoEncode::<Identity, Impl, OFFSET>,
            DoDecode: DoDecode::<Identity, Impl, OFFSET>,
            SetEncodingLevel: SetEncodingLevel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEncodingFilterFactory_Impl: ::windows_core::BaseImpl {
    fn FindBestFilter(this: &Self::This, pwzcodein: &::windows_core::PCWSTR, pwzcodeout: &::windows_core::PCWSTR, info: &DATAINFO) -> ::windows_core::Result<IDataFilter>;
    fn GetDefaultFilter(this: &Self::This, pwzcodein: &::windows_core::PCWSTR, pwzcodeout: &::windows_core::PCWSTR) -> ::windows_core::Result<IDataFilter>;
}
impl ::windows_core::Iids for IEncodingFilterFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEncodingFilterFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEncodingFilterFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindBestFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEncodingFilterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzcodein: ::windows_core::PCWSTR, pwzcodeout: ::windows_core::PCWSTR, info: DATAINFO, ppdf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindBestFilter(this, ::core::mem::transmute(&pwzcodein), ::core::mem::transmute(&pwzcodeout), ::core::mem::transmute(&info)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEncodingFilterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzcodein: ::windows_core::PCWSTR, pwzcodeout: ::windows_core::PCWSTR, ppdf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultFilter(this, ::core::mem::transmute(&pwzcodein), ::core::mem::transmute(&pwzcodeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEncodingFilterFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindBestFilter: FindBestFilter::<Identity, Impl, OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGetBindHandle_Impl: ::windows_core::BaseImpl {
    fn GetBindHandle(this: &Self::This, enumrequestedhandle: BINDHANDLETYPES) -> ::windows_core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IGetBindHandle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetBindHandle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetBindHandle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBindHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetBindHandle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBindHandle(this, ::core::mem::transmute_copy(&enumrequestedhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prethandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetBindHandle_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBindHandle: GetBindHandle::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHttpNegotiate_Impl: ::windows_core::BaseImpl {
    fn BeginningTransaction(this: &Self::This, szurl: &::windows_core::PCWSTR, szheaders: &::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn OnResponse(this: &Self::This, dwresponsecode: u32, szresponseheaders: &::windows_core::PCWSTR, szrequestheaders: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IHttpNegotiate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpNegotiate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHttpNegotiate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginningTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpNegotiate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szurl: ::windows_core::PCWSTR, szheaders: ::windows_core::PCWSTR, dwreserved: u32, pszadditionalheaders: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginningTransaction(this, ::core::mem::transmute(&szurl), ::core::mem::transmute(&szheaders), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszadditionalheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpNegotiate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwresponsecode: u32, szresponseheaders: ::windows_core::PCWSTR, szrequestheaders: ::windows_core::PCWSTR, pszadditionalrequestheaders: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnResponse(this, ::core::mem::transmute_copy(&dwresponsecode), ::core::mem::transmute(&szresponseheaders), ::core::mem::transmute(&szrequestheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszadditionalrequestheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHttpNegotiate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginningTransaction: BeginningTransaction::<Identity, Impl, OFFSET>,
            OnResponse: OnResponse::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHttpNegotiate2_Impl: ::windows_core::BaseImpl + IHttpNegotiate_Impl {
    fn GetRootSecurityId(this: &Self::This, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHttpNegotiate2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IHttpNegotiate);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpNegotiate2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHttpNegotiate2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRootSecurityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpNegotiate2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRootSecurityId(this, ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IHttpNegotiate2_Vtbl { base__: <IHttpNegotiate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetRootSecurityId: GetRootSecurityId::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHttpNegotiate3_Impl: ::windows_core::BaseImpl + IHttpNegotiate2_Impl {
    fn GetSerializedClientCertContext(this: &Self::This, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHttpNegotiate3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IHttpNegotiate2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpNegotiate3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHttpNegotiate3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSerializedClientCertContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpNegotiate3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSerializedClientCertContext(this, ::core::mem::transmute_copy(&ppbcert), ::core::mem::transmute_copy(&pcbcert)).into())
        }
        IHttpNegotiate3_Vtbl {
            base__: <IHttpNegotiate2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSerializedClientCertContext: GetSerializedClientCertContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpSecurity_Impl: ::windows_core::BaseImpl + IWindowForBindingUI_Impl {
    fn OnSecurityProblem(this: &Self::This, dwproblem: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHttpSecurity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWindowForBindingUI);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpSecurity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHttpSecurity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSecurityProblem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpSecurity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproblem: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSecurityProblem(this, ::core::mem::transmute_copy(&dwproblem)).into())
        }
        IHttpSecurity_Vtbl {
            base__: <IWindowForBindingUI as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSecurityProblem: OnSecurityProblem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternet_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IInternet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternet {
    const VTABLE: Self::Vtable = { IInternet_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfo_Impl: ::windows_core::BaseImpl {
    fn GetBindInfo(this: &Self::This, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows_core::Result<()>;
    fn GetBindString(this: &Self::This, ulstringtype: u32, ppwzstr: *mut ::windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IInternetBindInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetBindInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetBindInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBindInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetBindInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindInfo(this, ::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo)).into())
        }
        unsafe extern "system" fn GetBindString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetBindInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut ::windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindString(this, ::core::mem::transmute_copy(&ulstringtype), ::core::mem::transmute_copy(&ppwzstr), ::core::mem::transmute_copy(&cel), ::core::mem::transmute_copy(&pcelfetched)).into())
        }
        IInternetBindInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBindInfo: GetBindInfo::<Identity, Impl, OFFSET>,
            GetBindString: GetBindString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfoEx_Impl: ::windows_core::BaseImpl + IInternetBindInfo_Impl {
    fn GetBindInfoEx(this: &Self::This, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IInternetBindInfoEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IInternetBindInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetBindInfoEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetBindInfoEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBindInfoEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetBindInfoEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindInfoEx(this, ::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        IInternetBindInfoEx_Vtbl { base__: <IInternetBindInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBindInfoEx: GetBindInfoEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetHostSecurityManager_Impl: ::windows_core::BaseImpl {
    fn GetSecurityId(this: &Self::This, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::Result<()>;
    fn ProcessUrlAction(this: &Self::This, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn QueryCustomPolicy(this: &Self::This, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetHostSecurityManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetHostSecurityManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSecurityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSecurityId(this, ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessUrlAction(this, ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryCustomPolicy(this, ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IInternetHostSecurityManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSecurityId: GetSecurityId::<Identity, Impl, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, Impl, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetPriority_Impl: ::windows_core::BaseImpl {
    fn SetPriority(this: &Self::This, npriority: i32) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IInternetPriority {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetPriority_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetPriority {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetPriority_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&npriority)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetPriority_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInternetPriority_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocol_Impl: ::windows_core::BaseImpl + IInternetProtocolRoot_Impl {
    fn Read(this: &Self::This, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, dlibmove: i64, dworigin: u32) -> ::windows_core::Result<u64>;
    fn LockRequest(this: &Self::This, dwoptions: u32) -> ::windows_core::Result<()>;
    fn UnlockRequest(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInternetProtocol {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IInternetProtocolRoot);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetProtocol {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Seek(this, ::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plibnewposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockRequest(this, ::core::mem::transmute_copy(&dwoptions)).into())
        }
        unsafe extern "system" fn UnlockRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockRequest(this).into())
        }
        IInternetProtocol_Vtbl {
            base__: <IInternetProtocolRoot as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Read: Read::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            LockRequest: LockRequest::<Identity, Impl, OFFSET>,
            UnlockRequest: UnlockRequest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolEx_Impl: ::windows_core::BaseImpl + IInternetProtocol_Impl {
    fn StartEx(this: &Self::This, puri: ::core::option::Option<&super::IUri>, poiprotsink: ::core::option::Option<&IInternetProtocolSink>, poibindinfo: ::core::option::Option<&IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInternetProtocolEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IInternetProtocol);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetProtocolEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, poiprotsink: *mut ::core::ffi::c_void, poibindinfo: *mut ::core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartEx(this, ::windows_core::from_raw_borrowed(&puri), ::windows_core::from_raw_borrowed(&poiprotsink), ::windows_core::from_raw_borrowed(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IInternetProtocolEx_Vtbl { base__: <IInternetProtocol as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, StartEx: StartEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetProtocolInfo_Impl: ::windows_core::BaseImpl {
    fn ParseUrl(this: &Self::This, pwzurl: &::windows_core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: ::windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn CombineUrl(this: &Self::This, pwzbaseurl: &::windows_core::PCWSTR, pwzrelativeurl: &::windows_core::PCWSTR, dwcombineflags: u32, pwzresult: &::windows_core::PCWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn CompareUrl(this: &Self::This, pwzurl1: &::windows_core::PCWSTR, pwzurl2: &::windows_core::PCWSTR, dwcompareflags: u32) -> ::windows_core::Result<()>;
    fn QueryInfo(this: &Self::This, pwzurl: &::windows_core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetProtocolInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetProtocolInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzurl: ::windows_core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: ::windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseUrl(this, ::core::mem::transmute(&pwzurl), ::core::mem::transmute_copy(&parseaction), ::core::mem::transmute_copy(&dwparseflags), ::core::mem::transmute_copy(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn CombineUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzbaseurl: ::windows_core::PCWSTR, pwzrelativeurl: ::windows_core::PCWSTR, dwcombineflags: u32, pwzresult: ::windows_core::PCWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CombineUrl(this, ::core::mem::transmute(&pwzbaseurl), ::core::mem::transmute(&pwzrelativeurl), ::core::mem::transmute_copy(&dwcombineflags), ::core::mem::transmute(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn CompareUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzurl1: ::windows_core::PCWSTR, pwzurl2: ::windows_core::PCWSTR, dwcompareflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompareUrl(this, ::core::mem::transmute(&pwzurl1), ::core::mem::transmute(&pwzurl2), ::core::mem::transmute_copy(&dwcompareflags)).into())
        }
        unsafe extern "system" fn QueryInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzurl: ::windows_core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryInfo(this, ::core::mem::transmute(&pwzurl), ::core::mem::transmute_copy(&oueryoption), ::core::mem::transmute_copy(&dwqueryflags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcbbuf), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IInternetProtocolInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseUrl: ParseUrl::<Identity, Impl, OFFSET>,
            CombineUrl: CombineUrl::<Identity, Impl, OFFSET>,
            CompareUrl: CompareUrl::<Identity, Impl, OFFSET>,
            QueryInfo: QueryInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolRoot_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This, szurl: &::windows_core::PCWSTR, poiprotsink: ::core::option::Option<&IInternetProtocolSink>, poibindinfo: ::core::option::Option<&IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()>;
    fn Continue(this: &Self::This, pprotocoldata: *const PROTOCOLDATA) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This, hrreason: ::windows_core::HRESULT, dwoptions: u32) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This, dwoptions: u32) -> ::windows_core::Result<()>;
    fn Suspend(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInternetProtocolRoot {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetProtocolRoot {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szurl: ::windows_core::PCWSTR, poiprotsink: *mut ::core::ffi::c_void, poibindinfo: *mut ::core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute(&szurl), ::windows_core::from_raw_borrowed(&poiprotsink), ::windows_core::from_raw_borrowed(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn Continue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Continue(this, ::core::mem::transmute_copy(&pprotocoldata)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrreason: ::windows_core::HRESULT, dwoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this, ::core::mem::transmute_copy(&hrreason), ::core::mem::transmute_copy(&dwoptions)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this, ::core::mem::transmute_copy(&dwoptions)).into())
        }
        unsafe extern "system" fn Suspend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Suspend(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        IInternetProtocolRoot_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetProtocolSink_Impl: ::windows_core::BaseImpl {
    fn Switch(this: &Self::This, pprotocoldata: *const PROTOCOLDATA) -> ::windows_core::Result<()>;
    fn ReportProgress(this: &Self::This, ulstatuscode: u32, szstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReportData(this: &Self::This, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::Result<()>;
    fn ReportResult(this: &Self::This, hrresult: ::windows_core::HRESULT, dwerror: u32, szresult: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetProtocolSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetProtocolSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Switch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Switch(this, ::core::mem::transmute_copy(&pprotocoldata)).into())
        }
        unsafe extern "system" fn ReportProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportProgress(this, ::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute(&szstatustext)).into())
        }
        unsafe extern "system" fn ReportData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportData(this, ::core::mem::transmute_copy(&grfbscf), ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into())
        }
        unsafe extern "system" fn ReportResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT, dwerror: u32, szresult: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportResult(this, ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&dwerror), ::core::mem::transmute(&szresult)).into())
        }
        IInternetProtocolSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Switch: Switch::<Identity, Impl, OFFSET>,
            ReportProgress: ReportProgress::<Identity, Impl, OFFSET>,
            ReportData: ReportData::<Identity, Impl, OFFSET>,
            ReportResult: ReportResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetProtocolSinkStackable_Impl: ::windows_core::BaseImpl {
    fn SwitchSink(this: &Self::This, poiprotsink: ::core::option::Option<&IInternetProtocolSink>) -> ::windows_core::Result<()>;
    fn CommitSwitch(this: &Self::This) -> ::windows_core::Result<()>;
    fn RollbackSwitch(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetProtocolSinkStackable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetProtocolSinkStackable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SwitchSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poiprotsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchSink(this, ::windows_core::from_raw_borrowed(&poiprotsink)).into())
        }
        unsafe extern "system" fn CommitSwitch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitSwitch(this).into())
        }
        unsafe extern "system" fn RollbackSwitch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RollbackSwitch(this).into())
        }
        IInternetProtocolSinkStackable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SwitchSink: SwitchSink::<Identity, Impl, OFFSET>,
            CommitSwitch: CommitSwitch::<Identity, Impl, OFFSET>,
            RollbackSwitch: RollbackSwitch::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetSecurityManager_Impl: ::windows_core::BaseImpl {
    fn SetSecuritySite(this: &Self::This, psite: ::core::option::Option<&IInternetSecurityMgrSite>) -> ::windows_core::Result<()>;
    fn GetSecuritySite(this: &Self::This) -> ::windows_core::Result<IInternetSecurityMgrSite>;
    fn MapUrlToZone(this: &Self::This, pwszurl: &::windows_core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetSecurityId(this: &Self::This, pwszurl: &::windows_core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::Result<()>;
    fn ProcessUrlAction(this: &Self::This, pwszurl: &::windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn QueryCustomPolicy(this: &Self::This, pwszurl: &::windows_core::PCWSTR, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SetZoneMapping(this: &Self::This, dwzone: u32, lpszpattern: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetZoneMappings(this: &Self::This, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetSecurityManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetSecurityManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSecuritySite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecuritySite(this, ::windows_core::from_raw_borrowed(&psite)).into())
        }
        unsafe extern "system" fn GetSecuritySite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecuritySite(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MapUrlToZone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapUrlToZone(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetSecurityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSecurityId(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessUrlAction(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryCustomPolicy(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn SetZoneMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, lpszpattern: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZoneMapping(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute(&lpszpattern), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetZoneMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, ppenumstring: *mut *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetZoneMappings(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&ppenumstring), ::core::mem::transmute_copy(&dwflags)).into())
        }
        IInternetSecurityManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSecuritySite: SetSecuritySite::<Identity, Impl, OFFSET>,
            GetSecuritySite: GetSecuritySite::<Identity, Impl, OFFSET>,
            MapUrlToZone: MapUrlToZone::<Identity, Impl, OFFSET>,
            GetSecurityId: GetSecurityId::<Identity, Impl, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, Impl, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, Impl, OFFSET>,
            SetZoneMapping: SetZoneMapping::<Identity, Impl, OFFSET>,
            GetZoneMappings: GetZoneMappings::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetSecurityManagerEx_Impl: ::windows_core::BaseImpl + IInternetSecurityManager_Impl {
    fn ProcessUrlActionEx(this: &Self::This, pwszurl: &::windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetSecurityManagerEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IInternetSecurityManager);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManagerEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetSecurityManagerEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessUrlActionEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManagerEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessUrlActionEx(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into())
        }
        IInternetSecurityManagerEx_Vtbl {
            base__: <IInternetSecurityManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProcessUrlActionEx: ProcessUrlActionEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetSecurityManagerEx2_Impl: ::windows_core::BaseImpl + IInternetSecurityManagerEx_Impl {
    fn MapUrlToZoneEx2(this: &Self::This, puri: ::core::option::Option<&super::IUri>, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut ::windows_core::PWSTR, pdwoutflags: *mut u32) -> ::windows_core::Result<()>;
    fn ProcessUrlActionEx2(this: &Self::This, puri: ::core::option::Option<&super::IUri>, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows_core::Result<()>;
    fn GetSecurityIdEx2(this: &Self::This, puri: ::core::option::Option<&super::IUri>, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::Result<()>;
    fn QueryCustomPolicyEx2(this: &Self::This, puri: ::core::option::Option<&super::IUri>, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetSecurityManagerEx2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IInternetSecurityManagerEx);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetSecurityManagerEx2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MapUrlToZoneEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut ::windows_core::PWSTR, pdwoutflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapUrlToZoneEx2(this, ::windows_core::from_raw_borrowed(&puri), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppwszmappedurl), ::core::mem::transmute_copy(&pdwoutflags)).into())
        }
        unsafe extern "system" fn ProcessUrlActionEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessUrlActionEx2(this, ::windows_core::from_raw_borrowed(&puri), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into())
        }
        unsafe extern "system" fn GetSecurityIdEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSecurityIdEx2(this, ::windows_core::from_raw_borrowed(&puri), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn QueryCustomPolicyEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryCustomPolicyEx2(this, ::windows_core::from_raw_borrowed(&puri), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IInternetSecurityManagerEx2_Vtbl {
            base__: <IInternetSecurityManagerEx as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MapUrlToZoneEx2: MapUrlToZoneEx2::<Identity, Impl, OFFSET>,
            ProcessUrlActionEx2: ProcessUrlActionEx2::<Identity, Impl, OFFSET>,
            GetSecurityIdEx2: GetSecurityIdEx2::<Identity, Impl, OFFSET>,
            QueryCustomPolicyEx2: QueryCustomPolicyEx2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSecurityMgrSite_Impl: ::windows_core::BaseImpl {
    fn GetWindow(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
    fn EnableModeless(this: &Self::This, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInternetSecurityMgrSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityMgrSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetSecurityMgrSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityMgrSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSecurityMgrSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableModeless(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        IInternetSecurityMgrSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetSession_Impl: ::windows_core::BaseImpl {
    fn RegisterNameSpace(this: &Self::This, pcf: ::core::option::Option<&super::IClassFactory>, rclsid: *const ::windows_core::GUID, pwzprotocol: &::windows_core::PCWSTR, cpatterns: u32, ppwzpatterns: *const ::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::Result<()>;
    fn UnregisterNameSpace(this: &Self::This, pcf: ::core::option::Option<&super::IClassFactory>, pszprotocol: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RegisterMimeFilter(this: &Self::This, pcf: ::core::option::Option<&super::IClassFactory>, rclsid: *const ::windows_core::GUID, pwztype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UnregisterMimeFilter(this: &Self::This, pcf: ::core::option::Option<&super::IClassFactory>, pwztype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CreateBinding(this: &Self::This, pbc: ::core::option::Option<&super::IBindCtx>, szurl: &::windows_core::PCWSTR, punkouter: ::core::option::Option<&::windows_core::IUnknown>, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>, ppoinetprot: *mut ::core::option::Option<IInternetProtocol>, dwoption: u32) -> ::windows_core::Result<()>;
    fn SetSessionOption(this: &Self::This, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn GetSessionOption(this: &Self::This, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterNameSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, pwzprotocol: ::windows_core::PCWSTR, cpatterns: u32, ppwzpatterns: *const ::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterNameSpace(this, ::windows_core::from_raw_borrowed(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&pwzprotocol), ::core::mem::transmute_copy(&cpatterns), ::core::mem::transmute_copy(&ppwzpatterns), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn UnregisterNameSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, pszprotocol: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterNameSpace(this, ::windows_core::from_raw_borrowed(&pcf), ::core::mem::transmute(&pszprotocol)).into())
        }
        unsafe extern "system" fn RegisterMimeFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, pwztype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterMimeFilter(this, ::windows_core::from_raw_borrowed(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&pwztype)).into())
        }
        unsafe extern "system" fn UnregisterMimeFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, pwztype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterMimeFilter(this, ::windows_core::from_raw_borrowed(&pcf), ::core::mem::transmute(&pwztype)).into())
        }
        unsafe extern "system" fn CreateBinding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, szurl: ::windows_core::PCWSTR, punkouter: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void, ppoinetprot: *mut *mut ::core::ffi::c_void, dwoption: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBinding(this, ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute(&szurl), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&ppoinetprot), ::core::mem::transmute_copy(&dwoption)).into())
        }
        unsafe extern "system" fn SetSessionOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSessionOption(this, ::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbufferlength), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn GetSessionOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSessionOption(this, ::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pdwbufferlength), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IInternetSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterNameSpace: RegisterNameSpace::<Identity, Impl, OFFSET>,
            UnregisterNameSpace: UnregisterNameSpace::<Identity, Impl, OFFSET>,
            RegisterMimeFilter: RegisterMimeFilter::<Identity, Impl, OFFSET>,
            UnregisterMimeFilter: UnregisterMimeFilter::<Identity, Impl, OFFSET>,
            CreateBinding: CreateBinding::<Identity, Impl, OFFSET>,
            SetSessionOption: SetSessionOption::<Identity, Impl, OFFSET>,
            GetSessionOption: GetSessionOption::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInternetThreadSwitch_Impl: ::windows_core::BaseImpl {
    fn Prepare(this: &Self::This) -> ::windows_core::Result<()>;
    fn Continue(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetThreadSwitch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetThreadSwitch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetThreadSwitch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Prepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetThreadSwitch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Prepare(this).into())
        }
        unsafe extern "system" fn Continue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetThreadSwitch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Continue(this).into())
        }
        IInternetThreadSwitch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManager_Impl: ::windows_core::BaseImpl {
    fn GetZoneAttributes(this: &Self::This, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows_core::Result<()>;
    fn SetZoneAttributes(this: &Self::This, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows_core::Result<()>;
    fn GetZoneCustomPolicy(this: &Self::This, dwzone: u32, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows_core::Result<()>;
    fn SetZoneCustomPolicy(this: &Self::This, dwzone: u32, guidkey: *const ::windows_core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows_core::Result<()>;
    fn GetZoneActionPolicy(this: &Self::This, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows_core::Result<()>;
    fn SetZoneActionPolicy(this: &Self::This, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows_core::Result<()>;
    fn PromptAction(this: &Self::This, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: &::windows_core::PCWSTR, pwsztext: &::windows_core::PCWSTR, dwpromptflags: u32) -> ::windows_core::Result<()>;
    fn LogAction(this: &Self::This, dwaction: u32, pwszurl: &::windows_core::PCWSTR, pwsztext: &::windows_core::PCWSTR, dwlogflags: u32) -> ::windows_core::Result<()>;
    fn CreateZoneEnumerator(this: &Self::This, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetZoneAt(this: &Self::This, dwenum: u32, dwindex: u32) -> ::windows_core::Result<u32>;
    fn DestroyZoneEnumerator(this: &Self::This, dwenum: u32) -> ::windows_core::Result<()>;
    fn CopyTemplatePoliciesToZone(this: &Self::This, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInternetZoneManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetZoneManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetZoneAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetZoneAttributes(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into())
        }
        unsafe extern "system" fn SetZoneAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZoneAttributes(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into())
        }
        unsafe extern "system" fn GetZoneCustomPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetZoneCustomPolicy(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into())
        }
        unsafe extern "system" fn SetZoneCustomPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows_core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZoneCustomPolicy(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into())
        }
        unsafe extern "system" fn GetZoneActionPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetZoneActionPolicy(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into())
        }
        unsafe extern "system" fn SetZoneActionPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZoneActionPolicy(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into())
        }
        unsafe extern "system" fn PromptAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: ::windows_core::PCWSTR, pwsztext: ::windows_core::PCWSTR, dwpromptflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PromptAction(this, ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwsztext), ::core::mem::transmute_copy(&dwpromptflags)).into())
        }
        unsafe extern "system" fn LogAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaction: u32, pwszurl: ::windows_core::PCWSTR, pwsztext: ::windows_core::PCWSTR, dwlogflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogAction(this, ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwsztext), ::core::mem::transmute_copy(&dwlogflags)).into())
        }
        unsafe extern "system" fn CreateZoneEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateZoneEnumerator(this, ::core::mem::transmute_copy(&pdwenum), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetZoneAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetZoneAt(this, ::core::mem::transmute_copy(&dwenum), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwzone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestroyZoneEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwenum: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyZoneEnumerator(this, ::core::mem::transmute_copy(&dwenum)).into())
        }
        unsafe extern "system" fn CopyTemplatePoliciesToZone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTemplatePoliciesToZone(this, ::core::mem::transmute_copy(&dwtemplate), ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IInternetZoneManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetZoneAttributes: GetZoneAttributes::<Identity, Impl, OFFSET>,
            SetZoneAttributes: SetZoneAttributes::<Identity, Impl, OFFSET>,
            GetZoneCustomPolicy: GetZoneCustomPolicy::<Identity, Impl, OFFSET>,
            SetZoneCustomPolicy: SetZoneCustomPolicy::<Identity, Impl, OFFSET>,
            GetZoneActionPolicy: GetZoneActionPolicy::<Identity, Impl, OFFSET>,
            SetZoneActionPolicy: SetZoneActionPolicy::<Identity, Impl, OFFSET>,
            PromptAction: PromptAction::<Identity, Impl, OFFSET>,
            LogAction: LogAction::<Identity, Impl, OFFSET>,
            CreateZoneEnumerator: CreateZoneEnumerator::<Identity, Impl, OFFSET>,
            GetZoneAt: GetZoneAt::<Identity, Impl, OFFSET>,
            DestroyZoneEnumerator: DestroyZoneEnumerator::<Identity, Impl, OFFSET>,
            CopyTemplatePoliciesToZone: CopyTemplatePoliciesToZone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerEx_Impl: ::windows_core::BaseImpl + IInternetZoneManager_Impl {
    fn GetZoneActionPolicyEx(this: &Self::This, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows_core::Result<()>;
    fn SetZoneActionPolicyEx(this: &Self::This, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInternetZoneManagerEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IInternetZoneManager);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetZoneManagerEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetZoneActionPolicyEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetZoneActionPolicyEx(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn SetZoneActionPolicyEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZoneActionPolicyEx(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into())
        }
        IInternetZoneManagerEx_Vtbl {
            base__: <IInternetZoneManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetZoneActionPolicyEx: GetZoneActionPolicyEx::<Identity, Impl, OFFSET>,
            SetZoneActionPolicyEx: SetZoneActionPolicyEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerEx2_Impl: ::windows_core::BaseImpl + IInternetZoneManagerEx_Impl {
    fn GetZoneAttributesEx(this: &Self::This, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetZoneSecurityState(this: &Self::This, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetIESecurityState(this: &Self::This, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FixUnsecureSettings(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInternetZoneManagerEx2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IInternetZoneManagerEx);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetZoneManagerEx2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetZoneAttributesEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetZoneAttributesEx(this, ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetZoneSecurityState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetZoneSecurityState(this, ::core::mem::transmute_copy(&dwzoneindex), ::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered)).into())
        }
        unsafe extern "system" fn GetIESecurityState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIESecurityState(this, ::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered), ::core::mem::transmute_copy(&fnocache)).into())
        }
        unsafe extern "system" fn FixUnsecureSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FixUnsecureSettings(this).into())
        }
        IInternetZoneManagerEx2_Vtbl {
            base__: <IInternetZoneManagerEx as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetZoneAttributesEx: GetZoneAttributesEx::<Identity, Impl, OFFSET>,
            GetZoneSecurityState: GetZoneSecurityState::<Identity, Impl, OFFSET>,
            GetIESecurityState: GetIESecurityState::<Identity, Impl, OFFSET>,
            FixUnsecureSettings: FixUnsecureSettings::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMonikerProp_Impl: ::windows_core::BaseImpl {
    fn PutProperty(this: &Self::This, mkp: MONIKERPROPERTY, val: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMonikerProp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonikerProp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMonikerProp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PutProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMonikerProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mkp: MONIKERPROPERTY, val: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutProperty(this, ::core::mem::transmute_copy(&mkp), ::core::mem::transmute(&val)).into())
        }
        IMonikerProp_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, PutProperty: PutProperty::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistMoniker_Impl: ::windows_core::BaseImpl {
    fn GetClassID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn IsDirty(this: &Self::This) -> ::windows_core::HRESULT;
    fn Load(this: &Self::This, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: ::core::option::Option<&super::IMoniker>, pibc: ::core::option::Option<&super::IBindCtx>, grfmode: u32) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, pimkname: ::core::option::Option<&super::IMoniker>, pbc: ::core::option::Option<&super::IBindCtx>, fremember: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SaveCompleted(this: &Self::This, pimkname: ::core::option::Option<&super::IMoniker>, pibc: ::core::option::Option<&super::IBindCtx>) -> ::windows_core::Result<()>;
    fn GetCurMoniker(this: &Self::This) -> ::windows_core::Result<super::IMoniker>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPersistMoniker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistMoniker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this))
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: *mut ::core::ffi::c_void, pibc: *mut ::core::ffi::c_void, grfmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::core::mem::transmute_copy(&ffullyavailable), ::windows_core::from_raw_borrowed(&pimkname), ::windows_core::from_raw_borrowed(&pibc), ::core::mem::transmute_copy(&grfmode)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimkname: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, fremember: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::windows_core::from_raw_borrowed(&pimkname), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&fremember)).into())
        }
        unsafe extern "system" fn SaveCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimkname: *mut ::core::ffi::c_void, pibc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveCompleted(this, ::windows_core::from_raw_borrowed(&pimkname), ::windows_core::from_raw_borrowed(&pibc)).into())
        }
        unsafe extern "system" fn GetCurMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimkname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurMoniker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimkname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPersistMoniker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClassID: GetClassID::<Identity, Impl, OFFSET>,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            GetCurMoniker: GetCurMoniker::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`"]
#[cfg(feature = "Win32_Data_Xml_MsXml")]
pub trait ISoftDistExt_Impl: ::windows_core::BaseImpl {
    fn ProcessSoftDist(this: &Self::This, szcdfurl: &::windows_core::PCWSTR, psoftdistelement: ::core::option::Option<&super::super::super::Data::Xml::MsXml::IXMLElement>, lpsdi: *mut SOFTDISTINFO) -> ::windows_core::Result<()>;
    fn GetFirstCodeBase(this: &Self::This, szcodebase: *const ::windows_core::PCWSTR, dwmaxsize: *const u32) -> ::windows_core::Result<()>;
    fn GetNextCodeBase(this: &Self::This, szcodebase: *const ::windows_core::PCWSTR, dwmaxsize: *const u32) -> ::windows_core::Result<()>;
    fn AsyncInstallDistributionUnit(this: &Self::This, pbc: ::core::option::Option<&super::IBindCtx>, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
impl ::windows_core::Iids for ISoftDistExt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISoftDistExt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessSoftDist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szcdfurl: ::windows_core::PCWSTR, psoftdistelement: *mut ::core::ffi::c_void, lpsdi: *mut SOFTDISTINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessSoftDist(this, ::core::mem::transmute(&szcdfurl), ::windows_core::from_raw_borrowed(&psoftdistelement), ::core::mem::transmute_copy(&lpsdi)).into())
        }
        unsafe extern "system" fn GetFirstCodeBase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szcodebase: *const ::windows_core::PCWSTR, dwmaxsize: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFirstCodeBase(this, ::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into())
        }
        unsafe extern "system" fn GetNextCodeBase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szcodebase: *const ::windows_core::PCWSTR, dwmaxsize: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextCodeBase(this, ::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into())
        }
        unsafe extern "system" fn AsyncInstallDistributionUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncInstallDistributionUnit(this, ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&lpcbh)).into())
        }
        ISoftDistExt_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProcessSoftDist: ProcessSoftDist::<Identity, Impl, OFFSET>,
            GetFirstCodeBase: GetFirstCodeBase::<Identity, Impl, OFFSET>,
            GetNextCodeBase: GetNextCodeBase::<Identity, Impl, OFFSET>,
            AsyncInstallDistributionUnit: AsyncInstallDistributionUnit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUriBuilderFactory_Impl: ::windows_core::BaseImpl {
    fn CreateIUriBuilder(this: &Self::This, dwflags: u32, dwreserved: usize) -> ::windows_core::Result<super::IUriBuilder>;
    fn CreateInitializedIUriBuilder(this: &Self::This, dwflags: u32, dwreserved: usize) -> ::windows_core::Result<super::IUriBuilder>;
}
impl ::windows_core::Iids for IUriBuilderFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilderFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUriBuilderFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateIUriBuilder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateIUriBuilder(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuribuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInitializedIUriBuilder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriBuilderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInitializedIUriBuilder(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuribuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUriBuilderFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateIUriBuilder: CreateIUriBuilder::<Identity, Impl, OFFSET>,
            CreateInitializedIUriBuilder: CreateInitializedIUriBuilder::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUriContainer_Impl: ::windows_core::BaseImpl {
    fn GetIUri(this: &Self::This) -> ::windows_core::Result<super::IUri>;
}
impl ::windows_core::Iids for IUriContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUriContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUriContainer_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetIUri: GetIUri::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWinInetCacheHints_Impl: ::windows_core::BaseImpl {
    fn SetCacheExtension(this: &Self::This, pwzext: &::windows_core::PCWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWinInetCacheHints {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetCacheHints_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinInetCacheHints {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCacheExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetCacheHints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzext: ::windows_core::PCWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCacheExtension(this, ::core::mem::transmute(&pwzext), ::core::mem::transmute_copy(&pszcachefile), ::core::mem::transmute_copy(&pcbcachefile), ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        IWinInetCacheHints_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCacheExtension: SetCacheExtension::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWinInetCacheHints2_Impl: ::windows_core::BaseImpl + IWinInetCacheHints_Impl {
    fn SetCacheExtension2(this: &Self::This, pwzext: &::windows_core::PCWSTR, pwzcachefile: ::windows_core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWinInetCacheHints2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWinInetCacheHints);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetCacheHints2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinInetCacheHints2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCacheExtension2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetCacheHints2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzext: ::windows_core::PCWSTR, pwzcachefile: ::windows_core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCacheExtension2(this, ::core::mem::transmute(&pwzext), ::core::mem::transmute_copy(&pwzcachefile), ::core::mem::transmute_copy(&pcchcachefile), ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        IWinInetCacheHints2_Vtbl {
            base__: <IWinInetCacheHints as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCacheExtension2: SetCacheExtension2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWinInetFileStream_Impl: ::windows_core::BaseImpl {
    fn SetHandleForUnlock(this: &Self::This, hwininetlockhandle: usize, dwreserved: usize) -> ::windows_core::Result<()>;
    fn SetDeleteFile(this: &Self::This, dwreserved: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWinInetFileStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetFileStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinInetFileStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHandleForUnlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHandleForUnlock(this, ::core::mem::transmute_copy(&hwininetlockhandle), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn SetDeleteFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeleteFile(this, ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IWinInetFileStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetHandleForUnlock: SetHandleForUnlock::<Identity, Impl, OFFSET>,
            SetDeleteFile: SetDeleteFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWinInetHttpInfo_Impl: ::windows_core::BaseImpl + IWinInetInfo_Impl {
    fn QueryInfo(this: &Self::This, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWinInetHttpInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWinInetInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetHttpInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinInetHttpInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetHttpInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryInfo(this, ::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbbuf), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        IWinInetHttpInfo_Vtbl { base__: <IWinInetInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, QueryInfo: QueryInfo::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWinInetHttpTimeouts_Impl: ::windows_core::BaseImpl {
    fn GetRequestTimeouts(this: &Self::This, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWinInetHttpTimeouts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetHttpTimeouts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinInetHttpTimeouts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRequestTimeouts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetHttpTimeouts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRequestTimeouts(this, ::core::mem::transmute_copy(&pdwconnecttimeout), ::core::mem::transmute_copy(&pdwsendtimeout), ::core::mem::transmute_copy(&pdwreceivetimeout)).into())
        }
        IWinInetHttpTimeouts_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRequestTimeouts: GetRequestTimeouts::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWinInetInfo_Impl: ::windows_core::BaseImpl {
    fn QueryOption(this: &Self::This, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWinInetInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWinInetInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWinInetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryOption(this, ::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbbuf)).into())
        }
        IWinInetInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, QueryOption: QueryOption::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowForBindingUI_Impl: ::windows_core::BaseImpl {
    fn GetWindow(this: &Self::This, rguidreason: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWindowForBindingUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowForBindingUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowForBindingUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowForBindingUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidreason: *const ::windows_core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindow(this, ::core::mem::transmute_copy(&rguidreason)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowForBindingUI_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetWindow: GetWindow::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWrappedProtocol_Impl: ::windows_core::BaseImpl {
    fn GetWrapperCode(this: &Self::This, pncode: *mut i32, dwreserved: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWrappedProtocol {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWrappedProtocol_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWrappedProtocol {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWrapperCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWrappedProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWrapperCode(this, ::core::mem::transmute_copy(&pncode), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IWrappedProtocol_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetWrapperCode: GetWrapperCode::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IZoneIdentifier_Impl: ::windows_core::BaseImpl {
    fn GetId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetId(this: &Self::This, dwzone: u32) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IZoneIdentifier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IZoneIdentifier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwzone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwzone: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute_copy(&dwzone)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this).into())
        }
        IZoneIdentifier_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetId: GetId::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IZoneIdentifier2_Impl: ::windows_core::BaseImpl + IZoneIdentifier_Impl {
    fn GetLastWriterPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLastWriterPackageFamilyName(this: &Self::This, packagefamilyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoveLastWriterPackageFamilyName(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAppZoneId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetAppZoneId(this: &Self::This, zone: u32) -> ::windows_core::Result<()>;
    fn RemoveAppZoneId(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IZoneIdentifier2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IZoneIdentifier);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IZoneIdentifier2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastWriterPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastWriterPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastWriterPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastWriterPackageFamilyName(this, ::core::mem::transmute(&packagefamilyname)).into())
        }
        unsafe extern "system" fn RemoveLastWriterPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLastWriterPackageFamilyName(this).into())
        }
        unsafe extern "system" fn GetAppZoneId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zone: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAppZoneId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(zone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAppZoneId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zone: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppZoneId(this, ::core::mem::transmute_copy(&zone)).into())
        }
        unsafe extern "system" fn RemoveAppZoneId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAppZoneId(this).into())
        }
        IZoneIdentifier2_Vtbl {
            base__: <IZoneIdentifier as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLastWriterPackageFamilyName: GetLastWriterPackageFamilyName::<Identity, Impl, OFFSET>,
            SetLastWriterPackageFamilyName: SetLastWriterPackageFamilyName::<Identity, Impl, OFFSET>,
            RemoveLastWriterPackageFamilyName: RemoveLastWriterPackageFamilyName::<Identity, Impl, OFFSET>,
            GetAppZoneId: GetAppZoneId::<Identity, Impl, OFFSET>,
            SetAppZoneId: SetAppZoneId::<Identity, Impl, OFFSET>,
            RemoveAppZoneId: RemoveAppZoneId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
