#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IStorageProviderItemPropertySource_Impl: ::windows_core::BaseImpl {
    fn GetItemProperties(this: &Self::This, itempath: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IStorageProviderItemPropertySource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderItemPropertySource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageProviderItemPropertySource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderItemPropertySource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itempath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemProperties(this, ::core::mem::transmute(&itempath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageProviderItemPropertySource_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemProperties: GetItemProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStorageProviderPropertyCapabilities_Impl: ::windows_core::BaseImpl {
    fn IsPropertySupported(this: &Self::This, propertycanonicalname: &::windows_core::HSTRING) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IStorageProviderPropertyCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderPropertyCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageProviderPropertyCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsPropertySupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderPropertyCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertycanonicalname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPropertySupported(this, ::core::mem::transmute(&propertycanonicalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageProviderPropertyCapabilities_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsPropertySupported: IsPropertySupported::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IStorageProviderStatusUISource_Impl: ::windows_core::BaseImpl {
    fn GetStatusUI(this: &Self::This) -> ::windows_core::Result<StorageProviderStatusUI>;
    fn StatusUIChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IStorageProviderStatusUISource, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUIChanged(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IStorageProviderStatusUISource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageProviderStatusUISource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatusUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatusUI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusUIChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusUIChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveStatusUIChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderStatusUISource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStatusUIChanged(this, ::core::mem::transmute(&token)).into())
        }
        IStorageProviderStatusUISource_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatusUI: GetStatusUI::<Identity, Impl, OFFSET>,
            StatusUIChanged: StatusUIChanged::<Identity, Impl, OFFSET>,
            RemoveStatusUIChanged: RemoveStatusUIChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStorageProviderStatusUISourceFactory_Impl: ::windows_core::BaseImpl {
    fn GetStatusUISource(this: &Self::This, syncrootid: &::windows_core::HSTRING) -> ::windows_core::Result<IStorageProviderStatusUISource>;
}
impl ::windows_core::Iids for IStorageProviderStatusUISourceFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageProviderStatusUISourceFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatusUISource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderStatusUISourceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatusUISource(this, ::core::mem::transmute(&syncrootid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStorageProviderStatusUISourceFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatusUISource: GetStatusUISource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IStorageProviderUICommand_Impl: ::windows_core::BaseImpl {
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Icon(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Uri>;
    fn State(this: &Self::This) -> ::windows_core::Result<StorageProviderUICommandState>;
    fn Invoke(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IStorageProviderUICommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageProviderUICommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Icon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Icon(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUICommandState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this).into())
        }
        IStorageProviderUICommand_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Label: Label::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Icon: Icon::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStorageProviderUriSource_Impl: ::windows_core::BaseImpl {
    fn GetPathForContentUri(this: &Self::This, contenturi: &::windows_core::HSTRING, result: ::core::option::Option<&StorageProviderGetPathForContentUriResult>) -> ::windows_core::Result<()>;
    fn GetContentInfoForPath(this: &Self::This, path: &::windows_core::HSTRING, result: ::core::option::Option<&StorageProviderGetContentInfoForPathResult>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IStorageProviderUriSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUriSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorageProviderUriSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPathForContentUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUriSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenturi: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPathForContentUri(this, ::core::mem::transmute(&contenturi), ::windows_core::from_raw_borrowed(&result)).into())
        }
        unsafe extern "system" fn GetContentInfoForPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorageProviderUriSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContentInfoForPath(this, ::core::mem::transmute(&path), ::windows_core::from_raw_borrowed(&result)).into())
        }
        IStorageProviderUriSource_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPathForContentUri: GetPathForContentUri::<Identity, Impl, OFFSET>,
            GetContentInfoForPath: GetContentInfoForPath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
