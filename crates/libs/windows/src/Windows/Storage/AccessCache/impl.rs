#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageItemAccessList_Impl: ::windows_core::BaseImpl {
    fn AddOverloadDefaultMetadata(this: &Self::This, file: ::core::option::Option<&super::IStorageItem>) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Add(this: &Self::This, file: ::core::option::Option<&super::IStorageItem>, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AddOrReplaceOverloadDefaultMetadata(this: &Self::This, token: &::windows_core::HSTRING, file: ::core::option::Option<&super::IStorageItem>) -> ::windows_core::Result<()>;
    fn AddOrReplace(this: &Self::This, token: &::windows_core::HSTRING, file: ::core::option::Option<&super::IStorageItem>, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GetItemAsync(this: &Self::This, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileAsync(this: &Self::This, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderAsync(this: &Self::This, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn GetItemWithOptionsAsync(this: &Self::This, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>>;
    fn GetFileWithOptionsAsync(this: &Self::This, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>;
    fn GetFolderWithOptionsAsync(this: &Self::This, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>>;
    fn Remove(this: &Self::This, token: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn ContainsItem(this: &Self::This, token: &::windows_core::HSTRING) -> ::windows_core::Result<bool>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn CheckAccess(this: &Self::This, file: ::core::option::Option<&super::IStorageItem>) -> ::windows_core::Result<bool>;
    fn Entries(this: &Self::This) -> ::windows_core::Result<AccessListEntryView>;
    fn MaximumItemsAllowed(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IStorageItemAccessList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItemAccessList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddOverloadDefaultMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddOverloadDefaultMetadata(this, ::windows_core::from_raw_borrowed(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::windows_core::from_raw_borrowed(&file), ::core::mem::transmute(&metadata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddOrReplaceOverloadDefaultMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, file: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOrReplaceOverloadDefaultMetadata(this, ::core::mem::transmute(&token), ::windows_core::from_raw_borrowed(&file)).into())
        }
        unsafe extern "system" fn AddOrReplace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddOrReplace(this, ::core::mem::transmute(&token), ::windows_core::from_raw_borrowed(&file), ::core::mem::transmute(&metadata)).into())
        }
        unsafe extern "system" fn GetItemAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemAsync(this, ::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileAsync(this, ::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFolderAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFolderAsync(this, ::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemWithOptionsAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemWithOptionsAsync(this, ::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileWithOptionsAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileWithOptionsAsync(this, ::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFolderWithOptionsAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFolderWithOptionsAsync(this, ::core::mem::transmute(&token), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn ContainsItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContainsItem(this, ::core::mem::transmute(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn CheckAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckAccess(this, ::windows_core::from_raw_borrowed(&file)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Entries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Entries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaximumItemsAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemAccessList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaximumItemsAllowed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItemAccessList_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddOverloadDefaultMetadata: AddOverloadDefaultMetadata::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddOrReplaceOverloadDefaultMetadata: AddOrReplaceOverloadDefaultMetadata::<Identity, Impl, OFFSET>,
            AddOrReplace: AddOrReplace::<Identity, Impl, OFFSET>,
            GetItemAsync: GetItemAsync::<Identity, Impl, OFFSET>,
            GetFileAsync: GetFileAsync::<Identity, Impl, OFFSET>,
            GetFolderAsync: GetFolderAsync::<Identity, Impl, OFFSET>,
            GetItemWithOptionsAsync: GetItemWithOptionsAsync::<Identity, Impl, OFFSET>,
            GetFileWithOptionsAsync: GetFileWithOptionsAsync::<Identity, Impl, OFFSET>,
            GetFolderWithOptionsAsync: GetFolderWithOptionsAsync::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            ContainsItem: ContainsItem::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            CheckAccess: CheckAccess::<Identity, Impl, OFFSET>,
            Entries: Entries::<Identity, Impl, OFFSET>,
            MaximumItemsAllowed: MaximumItemsAllowed::<Identity, Impl, OFFSET>,
        }
    };
}
