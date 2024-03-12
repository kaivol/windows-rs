#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IIndexableContent_Impl: ::windows_core::BaseImpl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetId(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>;
    fn Stream(this: &Self::This) -> ::windows_core::Result<super::Streams::IRandomAccessStream>;
    fn SetStream(this: &Self::This, value: ::core::option::Option<&super::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn StreamContentType(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetStreamContentType(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IIndexableContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIndexableContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Stream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStream(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn StreamContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StreamContentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStreamContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexableContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamContentType(this, ::core::mem::transmute(&value)).into())
        }
        IIndexableContent_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            SetStream: SetStream::<Identity, Impl, OFFSET>,
            StreamContentType: StreamContentType::<Identity, Impl, OFFSET>,
            SetStreamContentType: SetStreamContentType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageFolderQueryOperations_Impl: ::windows_core::BaseImpl {
    fn GetIndexedStateAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IndexedState>>;
    fn CreateFileQueryOverloadDefault(this: &Self::This) -> ::windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQuery(this: &Self::This, query: CommonFileQuery) -> ::windows_core::Result<StorageFileQueryResult>;
    fn CreateFileQueryWithOptions(this: &Self::This, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<StorageFileQueryResult>;
    fn CreateFolderQueryOverloadDefault(this: &Self::This) -> ::windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQuery(this: &Self::This, query: CommonFolderQuery) -> ::windows_core::Result<StorageFolderQueryResult>;
    fn CreateFolderQueryWithOptions(this: &Self::This, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<StorageFolderQueryResult>;
    fn CreateItemQuery(this: &Self::This) -> ::windows_core::Result<StorageItemQueryResult>;
    fn CreateItemQueryWithOptions(this: &Self::This, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<StorageItemQueryResult>;
    fn GetFilesAsync(this: &Self::This, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFilesAsyncOverloadDefaultStartAndCount(this: &Self::This, query: CommonFileQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>;
    fn GetFoldersAsync(this: &Self::This, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetFoldersAsyncOverloadDefaultStartAndCount(this: &Self::This, query: CommonFolderQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>;
    fn GetItemsAsync(this: &Self::This, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>;
    fn AreQueryOptionsSupported(this: &Self::This, queryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<bool>;
    fn IsCommonFolderQuerySupported(this: &Self::This, query: CommonFolderQuery) -> ::windows_core::Result<bool>;
    fn IsCommonFileQuerySupported(this: &Self::This, query: CommonFileQuery) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IStorageFolderQueryOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageFolderQueryOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIndexedStateAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndexedStateAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileQueryOverloadDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileQueryOverloadDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileQuery(this, query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileQueryWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileQueryWithOptions(this, ::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFolderQueryOverloadDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFolderQueryOverloadDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFolderQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFolderQuery(this, query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFolderQueryWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFolderQueryWithOptions(this, ::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItemQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItemQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateItemQueryWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateItemQueryWithOptions(this, ::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilesAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilesAsync(this, query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultStartAndCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilesAsyncOverloadDefaultStartAndCount(this, query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFoldersAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFoldersAsync(this, query, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultStartAndCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFoldersAsyncOverloadDefaultStartAndCount(this, query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemsAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemsAsync(this, startindex, maxitemstoretrieve) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AreQueryOptionsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queryoptions: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AreQueryOptionsSupported(this, ::windows_core::from_raw_borrowed(&queryoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCommonFolderQuerySupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCommonFolderQuerySupported(this, query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCommonFileQuerySupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolderQueryOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCommonFileQuerySupported(this, query) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageFolderQueryOperations_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIndexedStateAsync: GetIndexedStateAsync::<Identity, Impl, OFFSET>,
            CreateFileQueryOverloadDefault: CreateFileQueryOverloadDefault::<Identity, Impl, OFFSET>,
            CreateFileQuery: CreateFileQuery::<Identity, Impl, OFFSET>,
            CreateFileQueryWithOptions: CreateFileQueryWithOptions::<Identity, Impl, OFFSET>,
            CreateFolderQueryOverloadDefault: CreateFolderQueryOverloadDefault::<Identity, Impl, OFFSET>,
            CreateFolderQuery: CreateFolderQuery::<Identity, Impl, OFFSET>,
            CreateFolderQueryWithOptions: CreateFolderQueryWithOptions::<Identity, Impl, OFFSET>,
            CreateItemQuery: CreateItemQuery::<Identity, Impl, OFFSET>,
            CreateItemQueryWithOptions: CreateItemQueryWithOptions::<Identity, Impl, OFFSET>,
            GetFilesAsync: GetFilesAsync::<Identity, Impl, OFFSET>,
            GetFilesAsyncOverloadDefaultStartAndCount: GetFilesAsyncOverloadDefaultStartAndCount::<Identity, Impl, OFFSET>,
            GetFoldersAsync: GetFoldersAsync::<Identity, Impl, OFFSET>,
            GetFoldersAsyncOverloadDefaultStartAndCount: GetFoldersAsyncOverloadDefaultStartAndCount::<Identity, Impl, OFFSET>,
            GetItemsAsync: GetItemsAsync::<Identity, Impl, OFFSET>,
            AreQueryOptionsSupported: AreQueryOptionsSupported::<Identity, Impl, OFFSET>,
            IsCommonFolderQuerySupported: IsCommonFolderQuerySupported::<Identity, Impl, OFFSET>,
            IsCommonFileQuerySupported: IsCommonFileQuerySupported::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IStorageQueryResultBase_Impl: ::windows_core::BaseImpl {
    fn GetItemCountAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn Folder(this: &Self::This) -> ::windows_core::Result<super::StorageFolder>;
    fn ContentsChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentsChanged(this: &Self::This, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn OptionsChanged(this: &Self::This, changedhandler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionsChanged(this: &Self::This, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FindStartIndexAsync(this: &Self::This, value: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetCurrentQueryOptions(this: &Self::This) -> ::windows_core::Result<QueryOptions>;
    fn ApplyNewQueryOptions(this: &Self::This, newqueryoptions: ::core::option::Option<&QueryOptions>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IStorageQueryResultBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageQueryResultBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemCountAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemCountAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Folder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Folder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContentsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentsChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveContentsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveContentsChanged(this, ::core::mem::transmute(&eventcookie)).into())
        }
        unsafe extern "system" fn OptionsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OptionsChanged(this, ::windows_core::from_raw_borrowed(&changedhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveOptionsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveOptionsChanged(this, ::core::mem::transmute(&eventcookie)).into())
        }
        unsafe extern "system" fn FindStartIndexAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindStartIndexAsync(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentQueryOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentQueryOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplyNewQueryOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageQueryResultBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newqueryoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyNewQueryOptions(this, ::windows_core::from_raw_borrowed(&newqueryoptions)).into())
        }
        IStorageQueryResultBase_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemCountAsync: GetItemCountAsync::<Identity, Impl, OFFSET>,
            Folder: Folder::<Identity, Impl, OFFSET>,
            ContentsChanged: ContentsChanged::<Identity, Impl, OFFSET>,
            RemoveContentsChanged: RemoveContentsChanged::<Identity, Impl, OFFSET>,
            OptionsChanged: OptionsChanged::<Identity, Impl, OFFSET>,
            RemoveOptionsChanged: RemoveOptionsChanged::<Identity, Impl, OFFSET>,
            FindStartIndexAsync: FindStartIndexAsync::<Identity, Impl, OFFSET>,
            GetCurrentQueryOptions: GetCurrentQueryOptions::<Identity, Impl, OFFSET>,
            ApplyNewQueryOptions: ApplyNewQueryOptions::<Identity, Impl, OFFSET>,
        }
    };
}
