#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IUriToStreamResolver_Impl: ::windows_core::BaseImpl {
    fn UriToStreamAsync(this: &Self::This, uri: ::core::option::Option<&super::Foundation::Uri>) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IInputStream>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IUriToStreamResolver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriToStreamResolver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUriToStreamResolver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UriToStreamAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUriToStreamResolver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UriToStreamAsync(this, ::windows_core::from_raw_borrowed(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUriToStreamResolver_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UriToStreamAsync: UriToStreamAsync::<Identity, Impl, OFFSET>,
        }
    };
}
