#[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemInformation_Impl: ::windows_core::BaseImpl {
    fn MusicProperties(this: &Self::This) -> ::windows_core::Result<super::FileProperties::MusicProperties>;
    fn VideoProperties(this: &Self::This) -> ::windows_core::Result<super::FileProperties::VideoProperties>;
    fn ImageProperties(this: &Self::This) -> ::windows_core::Result<super::FileProperties::ImageProperties>;
    fn DocumentProperties(this: &Self::This) -> ::windows_core::Result<super::FileProperties::DocumentProperties>;
    fn BasicProperties(this: &Self::This) -> ::windows_core::Result<super::FileProperties::BasicProperties>;
    fn Thumbnail(this: &Self::This) -> ::windows_core::Result<super::FileProperties::StorageItemThumbnail>;
    fn ThumbnailUpdated(this: &Self::This, changedhandler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThumbnailUpdated(this: &Self::This, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PropertiesUpdated(this: &Self::This, changedhandler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesUpdated(this: &Self::This, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IStorageItemInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageItemInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MusicProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MusicProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VideoProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VideoProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImageProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DocumentProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BasicProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BasicProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Thumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Thumbnail(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ThumbnailUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ThumbnailUpdated(this, ::windows_core::from_raw_borrowed(&changedhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveThumbnailUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveThumbnailUpdated(this, ::core::mem::transmute(&eventcookie)).into())
        }
        unsafe extern "system" fn PropertiesUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertiesUpdated(this, ::windows_core::from_raw_borrowed(&changedhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePropertiesUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageItemInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePropertiesUpdated(this, ::core::mem::transmute(&eventcookie)).into())
        }
        IStorageItemInformation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MusicProperties: MusicProperties::<Identity, Impl, OFFSET>,
            VideoProperties: VideoProperties::<Identity, Impl, OFFSET>,
            ImageProperties: ImageProperties::<Identity, Impl, OFFSET>,
            DocumentProperties: DocumentProperties::<Identity, Impl, OFFSET>,
            BasicProperties: BasicProperties::<Identity, Impl, OFFSET>,
            Thumbnail: Thumbnail::<Identity, Impl, OFFSET>,
            ThumbnailUpdated: ThumbnailUpdated::<Identity, Impl, OFFSET>,
            RemoveThumbnailUpdated: RemoveThumbnailUpdated::<Identity, Impl, OFFSET>,
            PropertiesUpdated: PropertiesUpdated::<Identity, Impl, OFFSET>,
            RemovePropertiesUpdated: RemovePropertiesUpdated::<Identity, Impl, OFFSET>,
        }
    };
}
