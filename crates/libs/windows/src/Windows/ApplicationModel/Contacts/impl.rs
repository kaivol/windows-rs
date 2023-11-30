pub trait IContactField_Impl: ::windows_core::BaseImpl {
    fn Type(this: &Self::This) -> ::windows_core::Result<ContactFieldType>;
    fn Category(this: &Self::This) -> ::windows_core::Result<ContactFieldCategory>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IContactField {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactField {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Category<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldCategory) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Category(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactField_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Category: Category::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactFieldFactory_Impl: ::windows_core::BaseImpl {
    fn CreateField_Default(this: &Self::This, value: &::windows_core::HSTRING, r#type: ContactFieldType) -> ::windows_core::Result<ContactField>;
    fn CreateField_Category(this: &Self::This, value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField>;
    fn CreateField_Custom(this: &Self::This, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows_core::Result<ContactField>;
}
impl ::windows_core::Iids for IContactFieldFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactFieldFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateField_Default<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: ContactFieldType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateField_Default(this, ::core::mem::transmute(&value), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateField_Category<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateField_Category(this, ::core::mem::transmute(&value), r#type, category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateField_Custom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateField_Custom(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value), r#type, category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactFieldFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateField_Default: CreateField_Default::<Identity, Impl, OFFSET>,
            CreateField_Category: CreateField_Category::<Identity, Impl, OFFSET>,
            CreateField_Custom: CreateField_Custom::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IContactInstantMessageFieldFactory_Impl: ::windows_core::BaseImpl {
    fn CreateInstantMessage_Default(this: &Self::This, username: &::windows_core::HSTRING) -> ::windows_core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_Category(this: &Self::This, username: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_All(this: &Self::This, username: &::windows_core::HSTRING, category: ContactFieldCategory, service: &::windows_core::HSTRING, displaytext: &::windows_core::HSTRING, verb: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<ContactInstantMessageField>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IContactInstantMessageFieldFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactInstantMessageFieldFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstantMessage_Default<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstantMessage_Default(this, ::core::mem::transmute(&username)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstantMessage_Category<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstantMessage_Category(this, ::core::mem::transmute(&username), category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstantMessage_All<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, service: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displaytext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, verb: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstantMessage_All(this, ::core::mem::transmute(&username), category, ::core::mem::transmute(&service), ::core::mem::transmute(&displaytext), ::windows_core::from_raw_borrowed(&verb)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactInstantMessageFieldFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstantMessage_Default: CreateInstantMessage_Default::<Identity, Impl, OFFSET>,
            CreateInstantMessage_Category: CreateInstantMessage_Category::<Identity, Impl, OFFSET>,
            CreateInstantMessage_All: CreateInstantMessage_All::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactLocationFieldFactory_Impl: ::windows_core::BaseImpl {
    fn CreateLocation_Default(this: &Self::This, unstructuredaddress: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField>;
    fn CreateLocation_Category(this: &Self::This, unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory) -> ::windows_core::Result<ContactLocationField>;
    fn CreateLocation_All(this: &Self::This, unstructuredaddress: &::windows_core::HSTRING, category: ContactFieldCategory, street: &::windows_core::HSTRING, city: &::windows_core::HSTRING, region: &::windows_core::HSTRING, country: &::windows_core::HSTRING, postalcode: &::windows_core::HSTRING) -> ::windows_core::Result<ContactLocationField>;
}
impl ::windows_core::Iids for IContactLocationFieldFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactLocationFieldFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLocation_Default<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLocation_Default(this, ::core::mem::transmute(&unstructuredaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLocation_Category<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLocation_Category(this, ::core::mem::transmute(&unstructuredaddress), category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLocation_All<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, category: ContactFieldCategory, street: ::std::mem::MaybeUninit<::windows_core::HSTRING>, city: ::std::mem::MaybeUninit<::windows_core::HSTRING>, region: ::std::mem::MaybeUninit<::windows_core::HSTRING>, country: ::std::mem::MaybeUninit<::windows_core::HSTRING>, postalcode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLocation_All(this, ::core::mem::transmute(&unstructuredaddress), category, ::core::mem::transmute(&street), ::core::mem::transmute(&city), ::core::mem::transmute(&region), ::core::mem::transmute(&country), ::core::mem::transmute(&postalcode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactLocationFieldFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateLocation_Default: CreateLocation_Default::<Identity, Impl, OFFSET>,
            CreateLocation_Category: CreateLocation_Category::<Identity, Impl, OFFSET>,
            CreateLocation_All: CreateLocation_All::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
