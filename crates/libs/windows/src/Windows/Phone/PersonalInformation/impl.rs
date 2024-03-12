#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IContactInformation_Impl: ::windows_core::BaseImpl {
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetDisplayName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn FamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetFamilyName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GivenName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetGivenName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn HonorificPrefix(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetHonorificPrefix(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn HonorificSuffix(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetHonorificSuffix(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GetDisplayPictureAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn SetDisplayPictureAsync(this: &Self::This, stream: ::core::option::Option<&super::super::Storage::Streams::IInputStream>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn DisplayPicture(this: &Self::This) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetPropertiesAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>>;
    fn ToVcardAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ToVcardWithOptionsAsync(this: &Self::This, format: VCardFormat) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IContactInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn FamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFamilyName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GivenName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GivenName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGivenName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGivenName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn HonorificPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HonorificPrefix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHonorificPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHonorificPrefix(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn HonorificSuffix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HonorificSuffix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHonorificSuffix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHonorificSuffix(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetDisplayPictureAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayPictureAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayPictureAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetDisplayPictureAsync(this, ::windows_core::from_raw_borrowed(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayPicture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayPicture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertiesAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertiesAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ToVcardAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToVcardAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ToVcardWithOptionsAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: VCardFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToVcardWithOptionsAsync(this, format) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactInformation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            FamilyName: FamilyName::<Identity, Impl, OFFSET>,
            SetFamilyName: SetFamilyName::<Identity, Impl, OFFSET>,
            GivenName: GivenName::<Identity, Impl, OFFSET>,
            SetGivenName: SetGivenName::<Identity, Impl, OFFSET>,
            HonorificPrefix: HonorificPrefix::<Identity, Impl, OFFSET>,
            SetHonorificPrefix: SetHonorificPrefix::<Identity, Impl, OFFSET>,
            HonorificSuffix: HonorificSuffix::<Identity, Impl, OFFSET>,
            SetHonorificSuffix: SetHonorificSuffix::<Identity, Impl, OFFSET>,
            GetDisplayPictureAsync: GetDisplayPictureAsync::<Identity, Impl, OFFSET>,
            SetDisplayPictureAsync: SetDisplayPictureAsync::<Identity, Impl, OFFSET>,
            DisplayPicture: DisplayPicture::<Identity, Impl, OFFSET>,
            GetPropertiesAsync: GetPropertiesAsync::<Identity, Impl, OFFSET>,
            ToVcardAsync: ToVcardAsync::<Identity, Impl, OFFSET>,
            ToVcardWithOptionsAsync: ToVcardWithOptionsAsync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IContactInformation2_Impl: ::windows_core::BaseImpl {
    fn DisplayPictureDate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DateTime>;
    fn SetDisplayPictureDate(this: &Self::This, returnvalue: &super::super::Foundation::DateTime) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IContactInformation2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactInformation2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayPictureDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayPictureDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayPictureDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, returnvalue: super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayPictureDate(this, ::core::mem::transmute(&returnvalue)).into())
        }
        IContactInformation2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayPictureDate: DisplayPictureDate::<Identity, Impl, OFFSET>,
            SetDisplayPictureDate: SetDisplayPictureDate::<Identity, Impl, OFFSET>,
        }
    };
}
