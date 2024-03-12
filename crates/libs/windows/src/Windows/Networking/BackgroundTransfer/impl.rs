#[doc = "Required features: `\"Security_Credentials\"`"]
#[cfg(feature = "Security_Credentials")]
pub trait IBackgroundTransferBase_Impl: ::windows_core::BaseImpl {
    fn SetRequestHeader(this: &Self::This, headername: &::windows_core::HSTRING, headervalue: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn ServerCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(this: &Self::This, credential: ::core::option::Option<&super::super::Security::Credentials::PasswordCredential>) -> ::windows_core::Result<()>;
    fn ProxyCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(this: &Self::This, credential: ::core::option::Option<&super::super::Security::Credentials::PasswordCredential>) -> ::windows_core::Result<()>;
    fn Method(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetMethod(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Group(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetGroup(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn CostPolicy(this: &Self::This) -> ::windows_core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(this: &Self::This, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Security_Credentials")]
impl ::windows_core::Iids for IBackgroundTransferBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Security_Credentials")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTransferBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, headername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, headervalue: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestHeader(this, ::core::mem::transmute(&headername), ::core::mem::transmute(&headervalue)).into())
        }
        unsafe extern "system" fn ServerCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credential: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerCredential(this, ::windows_core::from_raw_borrowed(&credential)).into())
        }
        unsafe extern "system" fn ProxyCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxyCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credential: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxyCredential(this, ::windows_core::from_raw_borrowed(&credential)).into())
        }
        unsafe extern "system" fn Method<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Method(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMethod(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Group(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroup(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn CostPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CostPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCostPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCostPolicy(this, value).into())
        }
        IBackgroundTransferBase_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            ServerCredential: ServerCredential::<Identity, Impl, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, Impl, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, Impl, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, Impl, OFFSET>,
            Method: Method::<Identity, Impl, OFFSET>,
            SetMethod: SetMethod::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            CostPolicy: CostPolicy::<Identity, Impl, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IBackgroundTransferContentPartFactory_Impl: ::windows_core::BaseImpl {
    fn CreateWithName(this: &Self::This, name: &::windows_core::HSTRING) -> ::windows_core::Result<BackgroundTransferContentPart>;
    fn CreateWithNameAndFileName(this: &Self::This, name: &::windows_core::HSTRING, filename: &::windows_core::HSTRING) -> ::windows_core::Result<BackgroundTransferContentPart>;
}
impl ::windows_core::Iids for IBackgroundTransferContentPartFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTransferContentPartFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateWithName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateWithName(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateWithNameAndFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateWithNameAndFileName(this, ::core::mem::transmute(&name), ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundTransferContentPartFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateWithName: CreateWithName::<Identity, Impl, OFFSET>,
            CreateWithNameAndFileName: CreateWithNameAndFileName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBackgroundTransferOperation_Impl: ::windows_core::BaseImpl {
    fn Guid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn RequestedUri(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Uri>;
    fn Method(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Group(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn CostPolicy(this: &Self::This) -> ::windows_core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(this: &Self::This, value: BackgroundTransferCostPolicy) -> ::windows_core::Result<()>;
    fn GetResultStreamAt(this: &Self::This, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetResponseInformation(this: &Self::This) -> ::windows_core::Result<ResponseInformation>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IBackgroundTransferOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTransferOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Guid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Guid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestedUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestedUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Method<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Method(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Group(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CostPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CostPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCostPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCostPolicy(this, value).into())
        }
        unsafe extern "system" fn GetResultStreamAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResultStreamAt(this, position) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResponseInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResponseInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundTransferOperation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Guid: Guid::<Identity, Impl, OFFSET>,
            RequestedUri: RequestedUri::<Identity, Impl, OFFSET>,
            Method: Method::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            CostPolicy: CostPolicy::<Identity, Impl, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, Impl, OFFSET>,
            GetResultStreamAt: GetResultStreamAt::<Identity, Impl, OFFSET>,
            GetResponseInformation: GetResponseInformation::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IBackgroundTransferOperationPriority_Impl: ::windows_core::BaseImpl {
    fn Priority(this: &Self::This) -> ::windows_core::Result<BackgroundTransferPriority>;
    fn SetPriority(this: &Self::This, value: BackgroundTransferPriority) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundTransferOperationPriority {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundTransferOperationPriority {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferPriority) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, value).into())
        }
        IBackgroundTransferOperationPriority_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
        }
    };
}
