#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ISysStorageProviderEventSource_Impl: ::windows_core::BaseImpl {
    fn EventReceived(this: &Self::This, handler: ::core::option::Option<&super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEventReceived(this: &Self::This, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ISysStorageProviderEventSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISysStorageProviderEventSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventReceived(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveEventReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderEventSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveEventReceived(this, ::core::mem::transmute(&token)).into())
        }
        ISysStorageProviderEventSource_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventReceived: EventReceived::<Identity, Impl, OFFSET>,
            RemoveEventReceived: RemoveEventReceived::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISysStorageProviderHandlerFactory_Impl: ::windows_core::BaseImpl {
    fn GetHttpRequestProvider(this: &Self::This, syncrootid: &::windows_core::HSTRING) -> ::windows_core::Result<ISysStorageProviderHttpRequestProvider>;
    fn GetEventSource(this: &Self::This, syncrootid: &::windows_core::HSTRING, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<ISysStorageProviderEventSource>;
}
impl ::windows_core::Iids for ISysStorageProviderHandlerFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISysStorageProviderHandlerFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHttpRequestProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHttpRequestProvider(this, ::core::mem::transmute(&syncrootid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderHandlerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventSource(this, ::core::mem::transmute(&syncrootid), ::core::mem::transmute(&eventname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISysStorageProviderHandlerFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHttpRequestProvider: GetHttpRequestProvider::<Identity, Impl, OFFSET>,
            GetEventSource: GetEventSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Web_Http\"`"]
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
pub trait ISysStorageProviderHttpRequestProvider_Impl: ::windows_core::BaseImpl {
    fn SendRequestAsync(this: &Self::This, request: ::core::option::Option<&super::super::super::Web::Http::HttpRequestMessage>) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl ::windows_core::Iids for ISysStorageProviderHttpRequestProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Web_Http"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISysStorageProviderHttpRequestProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendRequestAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISysStorageProviderHttpRequestProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendRequestAsync(this, ::windows_core::from_raw_borrowed(&request)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISysStorageProviderHttpRequestProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendRequestAsync: SendRequestAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
