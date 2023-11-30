#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageFile_Impl: ::windows_core::BaseImpl + Streams::IInputStreamReference_Impl + Streams::IRandomAccessStreamReference_Impl + IStorageItem_Impl {
    fn FileType(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ContentType(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn OpenAsync(this: &Self::This, accessmode: FileAccessMode) -> ::windows_core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(this: &Self::This) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
    fn CopyOverloadDefaultNameAndOptions(this: &Self::This, destinationfolder: ::core::option::Option<&IStorageFolder>) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverloadDefaultOptions(this: &Self::This, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverload(this: &Self::This, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING, option: NameCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyAndReplaceAsync(this: &Self::This, filetoreplace: ::core::option::Option<&IStorageFile>) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultNameAndOptions(this: &Self::This, destinationfolder: ::core::option::Option<&IStorageFolder>) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultOptions(this: &Self::This, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverload(this: &Self::This, destinationfolder: ::core::option::Option<&IStorageFolder>, desirednewname: &::windows_core::HSTRING, option: NameCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn MoveAndReplaceAsync(this: &Self::This, filetoreplace: ::core::option::Option<&IStorageFile>) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IStorageFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<Streams::IInputStreamReference as ::windows_core::ComInterface>::IID, <Streams::IRandomAccessStreamReference as ::windows_core::ComInterface>::IID, <IStorageItem as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FileType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenAsync(this, accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenTransactedWriteAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenTransactedWriteAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyOverloadDefaultNameAndOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyOverloadDefaultNameAndOptions(this, ::windows_core::from_raw_borrowed(&destinationfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyOverloadDefaultOptions(this, ::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyOverload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyOverload(this, ::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname), option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyAndReplaceAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetoreplace: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyAndReplaceAsync(this, ::windows_core::from_raw_borrowed(&filetoreplace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveOverloadDefaultNameAndOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveOverloadDefaultNameAndOptions(this, ::windows_core::from_raw_borrowed(&destinationfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveOverloadDefaultOptions(this, ::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveOverload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationfolder: *mut ::core::ffi::c_void, desirednewname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveOverload(this, ::windows_core::from_raw_borrowed(&destinationfolder), ::core::mem::transmute(&desirednewname), option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveAndReplaceAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetoreplace: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveAndReplaceAsync(this, ::windows_core::from_raw_borrowed(&filetoreplace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageFile_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FileType: FileType::<Identity, Impl, OFFSET>,
            ContentType: ContentType::<Identity, Impl, OFFSET>,
            OpenAsync: OpenAsync::<Identity, Impl, OFFSET>,
            OpenTransactedWriteAsync: OpenTransactedWriteAsync::<Identity, Impl, OFFSET>,
            CopyOverloadDefaultNameAndOptions: CopyOverloadDefaultNameAndOptions::<Identity, Impl, OFFSET>,
            CopyOverloadDefaultOptions: CopyOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            CopyOverload: CopyOverload::<Identity, Impl, OFFSET>,
            CopyAndReplaceAsync: CopyAndReplaceAsync::<Identity, Impl, OFFSET>,
            MoveOverloadDefaultNameAndOptions: MoveOverloadDefaultNameAndOptions::<Identity, Impl, OFFSET>,
            MoveOverloadDefaultOptions: MoveOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            MoveOverload: MoveOverload::<Identity, Impl, OFFSET>,
            MoveAndReplaceAsync: MoveAndReplaceAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IStorageFile2_Impl: ::windows_core::BaseImpl {
    fn OpenWithOptionsAsync(this: &Self::This, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteWithOptionsAsync(this: &Self::This, options: StorageOpenOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IStorageFile2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageFile2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenWithOptionsAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, options: StorageOpenOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenWithOptionsAsync(this, accessmode, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenTransactedWriteWithOptionsAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: StorageOpenOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenTransactedWriteWithOptionsAsync(this, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageFile2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenWithOptionsAsync: OpenWithOptionsAsync::<Identity, Impl, OFFSET>,
            OpenTransactedWriteWithOptionsAsync: OpenTransactedWriteWithOptionsAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStorageFilePropertiesWithAvailability_Impl: ::windows_core::BaseImpl {
    fn IsAvailable(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IStorageFilePropertiesWithAvailability {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageFilePropertiesWithAvailability {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAvailable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageFilePropertiesWithAvailability_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsAvailable: IsAvailable::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_FileProperties\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
pub trait IStorageFolder_Impl: ::windows_core::BaseImpl + IStorageItem_Impl {
    fn CreateFileAsyncOverloadDefaultOptions(this: &Self::This, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFileAsync(this: &Self::This, desiredname: &::windows_core::HSTRING, options: CreationCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsyncOverloadDefaultOptions(this: &Self::This, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFolderAsync(this: &Self::This, desiredname: &::windows_core::HSTRING, options: CreationCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetFileAsync(this: &Self::This, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFolderAsync(this: &Self::This, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetItemAsync(this: &Self::This, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(this: &Self::This) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>;
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(this: &Self::This) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>;
    fn GetItemsAsyncOverloadDefaultStartAndCount(this: &Self::This) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
impl ::windows_core::Iids for IStorageFolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IStorageItem as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageFolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFileAsyncOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileAsyncOverloadDefaultOptions(this, ::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFileAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFileAsync(this, ::core::mem::transmute(&desiredname), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFolderAsyncOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFolderAsyncOverloadDefaultOptions(this, ::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFolderAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFolderAsync(this, ::core::mem::transmute(&desiredname), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileAsync(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFolderAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFolderAsync(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemAsync(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultOptionsStartAndCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilesAsyncOverloadDefaultOptionsStartAndCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFoldersAsyncOverloadDefaultOptionsStartAndCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemsAsyncOverloadDefaultStartAndCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemsAsyncOverloadDefaultStartAndCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageFolder_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFileAsyncOverloadDefaultOptions: CreateFileAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            CreateFileAsync: CreateFileAsync::<Identity, Impl, OFFSET>,
            CreateFolderAsyncOverloadDefaultOptions: CreateFolderAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            CreateFolderAsync: CreateFolderAsync::<Identity, Impl, OFFSET>,
            GetFileAsync: GetFileAsync::<Identity, Impl, OFFSET>,
            GetFolderAsync: GetFolderAsync::<Identity, Impl, OFFSET>,
            GetItemAsync: GetItemAsync::<Identity, Impl, OFFSET>,
            GetFilesAsyncOverloadDefaultOptionsStartAndCount: GetFilesAsyncOverloadDefaultOptionsStartAndCount::<Identity, Impl, OFFSET>,
            GetFoldersAsyncOverloadDefaultOptionsStartAndCount: GetFoldersAsyncOverloadDefaultOptionsStartAndCount::<Identity, Impl, OFFSET>,
            GetItemsAsyncOverloadDefaultStartAndCount: GetItemsAsyncOverloadDefaultStartAndCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IStorageFolder2_Impl: ::windows_core::BaseImpl {
    fn TryGetItemAsync(this: &Self::This, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IStorageFolder2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageFolder2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TryGetItemAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageFolder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TryGetItemAsync(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageFolder2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TryGetItemAsync: TryGetItemAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
pub trait IStorageItem_Impl: ::windows_core::BaseImpl {
    fn RenameAsyncOverloadDefaultOptions(this: &Self::This, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn RenameAsync(this: &Self::This, desiredname: &::windows_core::HSTRING, option: NameCollisionOption) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsyncOverloadDefaultOptions(this: &Self::This) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsync(this: &Self::This, option: StorageDeleteOption) -> ::windows_core::Result<super::Foundation::IAsyncAction>;
    fn GetBasicPropertiesAsync(this: &Self::This) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Attributes(this: &Self::This) -> ::windows_core::Result<FileAttributes>;
    fn DateCreated(this: &Self::This) -> ::windows_core::Result<super::Foundation::DateTime>;
    fn IsOfType(this: &Self::This, r#type: StorageItemTypes) -> ::windows_core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl ::windows_core::Iids for IStorageItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RenameAsyncOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RenameAsyncOverloadDefaultOptions(this, ::core::mem::transmute(&desiredname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RenameAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RenameAsync(this, ::core::mem::transmute(&desiredname), option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteAsyncOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeleteAsyncOverloadDefaultOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: StorageDeleteOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeleteAsync(this, option) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBasicPropertiesAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBasicPropertiesAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FileAttributes) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DateCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DateCreated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: StorageItemTypes, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOfType(this, r#type) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItem_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RenameAsyncOverloadDefaultOptions: RenameAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            RenameAsync: RenameAsync::<Identity, Impl, OFFSET>,
            DeleteAsyncOverloadDefaultOptions: DeleteAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, Impl, OFFSET>,
            GetBasicPropertiesAsync: GetBasicPropertiesAsync::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            DateCreated: DateCreated::<Identity, Impl, OFFSET>,
            IsOfType: IsOfType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
pub trait IStorageItem2_Impl: ::windows_core::BaseImpl + IStorageItem_Impl {
    fn GetParentAsync(this: &Self::This) -> ::windows_core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn IsEqual(this: &Self::This, item: ::core::option::Option<&IStorageItem>) -> ::windows_core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl ::windows_core::Iids for IStorageItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IStorageItem as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParentAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItem2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParentAsync: GetParentAsync::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemProperties_Impl: ::windows_core::BaseImpl {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(this: &Self::This, mode: FileProperties::ThumbnailMode) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsyncOverloadDefaultOptions(this: &Self::This, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsync(this: &Self::This, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn DisplayType(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn FolderRelativeId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<FileProperties::StorageItemContentProperties>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IStorageItemProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItemProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(this, mode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnailAsyncOverloadDefaultOptions(this, mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThumbnailAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnailAsync(this, mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FolderRelativeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FolderRelativeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItemProperties_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Identity, Impl, OFFSET>,
            GetThumbnailAsyncOverloadDefaultOptions: GetThumbnailAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            GetThumbnailAsync: GetThumbnailAsync::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            DisplayType: DisplayType::<Identity, Impl, OFFSET>,
            FolderRelativeId: FolderRelativeId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemProperties2_Impl: ::windows_core::BaseImpl + IStorageItemProperties_Impl {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(this: &Self::This, mode: FileProperties::ThumbnailMode) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(this: &Self::This, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsync(this: &Self::This, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IStorageItemProperties2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IStorageItemProperties as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItemProperties2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(this, mode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(this, mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemProperties2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScaledImageAsThumbnailAsync(this, mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItemProperties2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Identity, Impl, OFFSET>,
            GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultOptions::<Identity, Impl, OFFSET>,
            GetScaledImageAsThumbnailAsync: GetScaledImageAsThumbnailAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemPropertiesWithProvider_Impl: ::windows_core::BaseImpl + IStorageItemProperties_Impl {
    fn Provider(this: &Self::This) -> ::windows_core::Result<StorageProvider>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IStorageItemPropertiesWithProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IStorageItemProperties as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemPropertiesWithProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItemPropertiesWithProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Provider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemPropertiesWithProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Provider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageItemPropertiesWithProvider_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Provider: Provider::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStreamedFileDataRequest_Impl: ::windows_core::BaseImpl {
    fn FailAndClose(this: &Self::This, failuremode: StreamedFileFailureMode) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IStreamedFileDataRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamedFileDataRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStreamedFileDataRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FailAndClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStreamedFileDataRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, failuremode: StreamedFileFailureMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FailAndClose(this, failuremode).into())
        }
        IStreamedFileDataRequest_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FailAndClose: FailAndClose::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
