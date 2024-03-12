#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Http_Headers\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
pub trait IHttpContent_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl {
    fn Headers(this: &Self::This) -> ::windows_core::Result<Headers::HttpContentHeaderCollection>;
    fn BufferAllAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn ReadAsBufferAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u64>>;
    fn ReadAsInputStreamAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, u64>>;
    fn ReadAsStringAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>;
    fn TryComputeLength(this: &Self::This, length: &mut u64) -> ::windows_core::Result<bool>;
    fn WriteToStreamAsync(this: &Self::This, outputstream: ::core::option::Option<&super::super::Storage::Streams::IOutputStream>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl ::windows_core::Iids for IHttpContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http_Headers"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHttpContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Headers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Headers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BufferAllAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferAllAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadAsBufferAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadAsBufferAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadAsInputStreamAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadAsInputStreamAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadAsStringAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadAsStringAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TryComputeLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut u64, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TryComputeLength(this, ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteToStreamAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHttpContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteToStreamAsync(this, ::windows_core::from_raw_borrowed(&outputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHttpContent_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Headers: Headers::<Identity, Impl, OFFSET>,
            BufferAllAsync: BufferAllAsync::<Identity, Impl, OFFSET>,
            ReadAsBufferAsync: ReadAsBufferAsync::<Identity, Impl, OFFSET>,
            ReadAsInputStreamAsync: ReadAsInputStreamAsync::<Identity, Impl, OFFSET>,
            ReadAsStringAsync: ReadAsStringAsync::<Identity, Impl, OFFSET>,
            TryComputeLength: TryComputeLength::<Identity, Impl, OFFSET>,
            WriteToStreamAsync: WriteToStreamAsync::<Identity, Impl, OFFSET>,
        }
    };
}
