#[doc = "Required features: `\"Security_Authentication_Web_Core\"`"]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderBaseReportOperation_Impl: ::windows_core::BaseImpl {
    fn ReportCompleted(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReportError(this: &Self::This, value: ::core::option::Option<&super::Core::WebProviderError>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows_core::Iids for IWebAccountProviderBaseReportOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderBaseReportOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportCompleted(this).into())
        }
        unsafe extern "system" fn ReportError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderBaseReportOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportError(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        IWebAccountProviderBaseReportOperation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportCompleted: ReportCompleted::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWebAccountProviderOperation_Impl: ::windows_core::BaseImpl {
    fn Kind(this: &Self::This) -> ::windows_core::Result<WebAccountProviderOperationKind>;
}
impl ::windows_core::Iids for IWebAccountProviderOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Kind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebAccountProviderOperation_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Kind: Kind::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Security_Authentication_Web_Core\"`"]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderSilentReportOperation_Impl: ::windows_core::BaseImpl + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserInteractionRequired(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReportUserInteractionRequiredWithError(this: &Self::This, value: ::core::option::Option<&super::Core::WebProviderError>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows_core::Iids for IWebAccountProviderSilentReportOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IWebAccountProviderBaseReportOperation as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderSilentReportOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportUserInteractionRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportUserInteractionRequired(this).into())
        }
        unsafe extern "system" fn ReportUserInteractionRequiredWithError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderSilentReportOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportUserInteractionRequiredWithError(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        IWebAccountProviderSilentReportOperation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportUserInteractionRequired: ReportUserInteractionRequired::<Identity, Impl, OFFSET>,
            ReportUserInteractionRequiredWithError: ReportUserInteractionRequiredWithError::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWebAccountProviderTokenObjects_Impl: ::windows_core::BaseImpl {
    fn Operation(this: &Self::This) -> ::windows_core::Result<IWebAccountProviderOperation>;
}
impl ::windows_core::Iids for IWebAccountProviderTokenObjects {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderTokenObjects {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Operation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenObjects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Operation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebAccountProviderTokenObjects_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Operation: Operation::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"System\"`"]
#[cfg(feature = "System")]
pub trait IWebAccountProviderTokenObjects2_Impl: ::windows_core::BaseImpl + IWebAccountProviderTokenObjects_Impl {
    fn User(this: &Self::This) -> ::windows_core::Result<super::super::super::super::System::User>;
}
#[cfg(feature = "System")]
impl ::windows_core::Iids for IWebAccountProviderTokenObjects2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IWebAccountProviderTokenObjects as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "System")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderTokenObjects2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenObjects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebAccountProviderTokenObjects2_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, User: User::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IWebAccountProviderTokenOperation_Impl: ::windows_core::BaseImpl + IWebAccountProviderOperation_Impl {
    fn ProviderRequest(this: &Self::This) -> ::windows_core::Result<WebProviderTokenRequest>;
    fn ProviderResponses(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>;
    fn SetCacheExpirationTime(this: &Self::This, value: &super::super::super::super::Foundation::DateTime) -> ::windows_core::Result<()>;
    fn CacheExpirationTime(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IWebAccountProviderTokenOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IWebAccountProviderOperation as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderTokenOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProviderRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProviderRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProviderResponses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProviderResponses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCacheExpirationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCacheExpirationTime(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn CacheExpirationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderTokenOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CacheExpirationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebAccountProviderTokenOperation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProviderRequest: ProviderRequest::<Identity, Impl, OFFSET>,
            ProviderResponses: ProviderResponses::<Identity, Impl, OFFSET>,
            SetCacheExpirationTime: SetCacheExpirationTime::<Identity, Impl, OFFSET>,
            CacheExpirationTime: CacheExpirationTime::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Security_Authentication_Web_Core\"`"]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub trait IWebAccountProviderUIReportOperation_Impl: ::windows_core::BaseImpl + IWebAccountProviderBaseReportOperation_Impl {
    fn ReportUserCanceled(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl ::windows_core::Iids for IWebAccountProviderUIReportOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IWebAccountProviderBaseReportOperation as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Security_Authentication_Web_Core")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAccountProviderUIReportOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportUserCanceled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAccountProviderUIReportOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportUserCanceled(this).into())
        }
        IWebAccountProviderUIReportOperation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportUserCanceled: ReportUserCanceled::<Identity, Impl, OFFSET>,
        }
    };
}
