#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IHttpFilter_Impl: ::windows_core::BaseImpl + super::super::super::Foundation::IClosable_Impl {
    fn SendRequestAsync(this: &Self::This, request: ::core::option::Option<&super::HttpRequestMessage>) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IHttpFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHttpFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendRequestAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendRequestAsync(this, ::windows_core::from_raw_borrowed(&request)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHttpFilter_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendRequestAsync: SendRequestAsync::<Identity, Impl, OFFSET>,
        }
    };
}
