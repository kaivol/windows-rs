#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageItemExtraProperties_Impl: ::windows_core::BaseImpl {
    fn RetrievePropertiesAsync(this: &Self::This, propertiestoretrieve: ::core::option::Option<&super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>>;
    fn SavePropertiesAsync(this: &Self::This, propertiestosave: ::core::option::Option<&super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn SavePropertiesAsyncOverloadDefault(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IStorageItemExtraProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItemExtraProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RetrievePropertiesAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrievePropertiesAsync(this, ::windows_core::from_raw_borrowed(&propertiestoretrieve)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SavePropertiesAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiestosave: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SavePropertiesAsync(this, ::windows_core::from_raw_borrowed(&propertiestosave)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SavePropertiesAsyncOverloadDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemExtraProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SavePropertiesAsyncOverloadDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItemExtraProperties_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RetrievePropertiesAsync: RetrievePropertiesAsync::<Identity, Impl, OFFSET>,
            SavePropertiesAsync: SavePropertiesAsync::<Identity, Impl, OFFSET>,
            SavePropertiesAsyncOverloadDefault: SavePropertiesAsyncOverloadDefault::<Identity, Impl, OFFSET>,
        }
    };
}
