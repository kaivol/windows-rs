#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetPropertyInfo(this: &Self::This, id: i32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetProperty(this: &Self::This, id: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PutProperty(this: &Self::This, id: i32, pvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ResetProperty(this: &Self::This, id: i32) -> ::windows_core::Result<()>;
    fn Apply(this: &Self::This) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISdo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISdo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, pppropertyinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyInfo(this, ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutProperty(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn ResetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetProperty(this, ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Apply(this).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISdo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            PutProperty: PutProperty::<Identity, Impl, OFFSET>,
            ResetProperty: ResetProperty::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, bstrname: &::windows_core::BSTR, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, pitem: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn RemoveAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reload(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsNameUnique(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Item(this: &Self::This, name: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISdoCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISdoCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&ppitem)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::windows_core::from_raw_borrowed(&pitem)).into())
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAll(this).into())
        }
        unsafe extern "system" fn Reload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reload(this).into())
        }
        unsafe extern "system" fn IsNameUnique<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsNameUnique(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *const super::super::System::Variant::VARIANT, pitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISdoCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Reload: Reload::<Identity, Impl, OFFSET>,
            IsNameUnique: IsNameUnique::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoDictionaryOld_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn EnumAttributes(this: &Self::This, id: *mut super::super::System::Variant::VARIANT, pvalues: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetAttributeInfo(this: &Self::This, id: ATTRIBUTEID, pinfoids: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumAttributeValues(this: &Self::This, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Variant::VARIANT, pvaluesdesc: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CreateAttribute(this: &Self::This, id: ATTRIBUTEID) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetAttributeID(this: &Self::This, bstrattributename: &::windows_core::BSTR) -> ::windows_core::Result<ATTRIBUTEID>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISdoDictionaryOld {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISdoDictionaryOld {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Variant::VARIANT, pvalues: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttributes(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalues)).into())
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pinfoids: *const super::super::System::Variant::VARIANT, pinfovalues: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeInfo(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pinfoids)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfovalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumAttributeValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Variant::VARIANT, pvaluesdesc: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttributeValues(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalueids), ::core::mem::transmute_copy(&pvaluesdesc)).into())
        }
        unsafe extern "system" fn CreateAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, ppattributeobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAttribute(this, ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributeobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattributename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pid: *mut ATTRIBUTEID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeID(this, ::core::mem::transmute(&bstrattributename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISdoDictionaryOld_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumAttributes: EnumAttributes::<Identity, Impl, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, Impl, OFFSET>,
            EnumAttributeValues: EnumAttributeValues::<Identity, Impl, OFFSET>,
            CreateAttribute: CreateAttribute::<Identity, Impl, OFFSET>,
            GetAttributeID: GetAttributeID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoMachine_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Attach(this: &Self::This, bstrcomputername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDictionarySDO(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetServiceSDO(this: &Self::This, edatastore: IASDATASTORE, bstrservicename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetUserSDO(this: &Self::This, edatastore: IASDATASTORE, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetOSType(this: &Self::This) -> ::windows_core::Result<IASOSTYPE>;
    fn GetDomainType(this: &Self::This) -> ::windows_core::Result<IASDOMAINTYPE>;
    fn IsDirectoryAvailable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAttachedComputer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSDOSchema(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISdoMachine {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISdoMachine {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Attach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcomputername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Attach(this, ::core::mem::transmute(&bstrcomputername)).into())
        }
        unsafe extern "system" fn GetDictionarySDO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdictionarysdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionarySDO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdictionarysdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetServiceSDO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppservicesdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceSDO(this, ::core::mem::transmute_copy(&edatastore), ::core::mem::transmute(&bstrservicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservicesdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserSDO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppusersdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserSDO(this, ::core::mem::transmute_copy(&edatastore), ::core::mem::transmute(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppusersdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOSType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eostype: *mut IASOSTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOSType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eostype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, edomaintype: *mut IASDOMAINTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDomainType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edomaintype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDirectoryAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, booldirectoryavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDirectoryAvailable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(booldirectoryavailable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttachedComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcomputername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttachedComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrcomputername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSDOSchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsdoschema: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSDOSchema(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsdoschema, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISdoMachine_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Attach: Attach::<Identity, Impl, OFFSET>,
            GetDictionarySDO: GetDictionarySDO::<Identity, Impl, OFFSET>,
            GetServiceSDO: GetServiceSDO::<Identity, Impl, OFFSET>,
            GetUserSDO: GetUserSDO::<Identity, Impl, OFFSET>,
            GetOSType: GetOSType::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
            IsDirectoryAvailable: IsDirectoryAvailable::<Identity, Impl, OFFSET>,
            GetAttachedComputer: GetAttachedComputer::<Identity, Impl, OFFSET>,
            GetSDOSchema: GetSDOSchema::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoMachine2_Impl: ::windows_core::BaseImpl + ISdoMachine_Impl {
    fn GetTemplatesSDO(this: &Self::This, bstrservicename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn EnableTemplates(this: &Self::This) -> ::windows_core::Result<()>;
    fn SyncConfigAgainstTemplates(this: &Self::This, bstrservicename: &::windows_core::BSTR, ppconfigroot: *mut ::core::option::Option<::windows_core::IUnknown>, pptemplatesroot: *mut ::core::option::Option<::windows_core::IUnknown>, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ImportRemoteTemplates(this: &Self::This, plocaltemplatesroot: ::core::option::Option<&::windows_core::IUnknown>, bstrremotemachinename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Reload(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISdoMachine2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISdoMachine);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISdoMachine2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTemplatesSDO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptemplatessdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTemplatesSDO(this, ::core::mem::transmute(&bstrservicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplatessdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableTemplates(this).into())
        }
        unsafe extern "system" fn SyncConfigAgainstTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppconfigroot: *mut *mut ::core::ffi::c_void, pptemplatesroot: *mut *mut ::core::ffi::c_void, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncConfigAgainstTemplates(this, ::core::mem::transmute(&bstrservicename), ::core::mem::transmute_copy(&ppconfigroot), ::core::mem::transmute_copy(&pptemplatesroot), ::core::mem::transmute_copy(&bforcedsync)).into())
        }
        unsafe extern "system" fn ImportRemoteTemplates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocaltemplatesroot: *mut ::core::ffi::c_void, bstrremotemachinename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImportRemoteTemplates(this, ::windows_core::from_raw_borrowed(&plocaltemplatesroot), ::core::mem::transmute(&bstrremotemachinename)).into())
        }
        unsafe extern "system" fn Reload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reload(this).into())
        }
        ISdoMachine2_Vtbl {
            base__: <ISdoMachine as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTemplatesSDO: GetTemplatesSDO::<Identity, Impl, OFFSET>,
            EnableTemplates: EnableTemplates::<Identity, Impl, OFFSET>,
            SyncConfigAgainstTemplates: SyncConfigAgainstTemplates::<Identity, Impl, OFFSET>,
            ImportRemoteTemplates: ImportRemoteTemplates::<Identity, Impl, OFFSET>,
            Reload: Reload::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoServiceControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StartService(this: &Self::This) -> ::windows_core::Result<()>;
    fn StopService(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetServiceStatus(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ResetService(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISdoServiceControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISdoServiceControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartService(this).into())
        }
        unsafe extern "system" fn StopService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopService(this).into())
        }
        unsafe extern "system" fn GetServiceStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResetService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetService(this).into())
        }
        ISdoServiceControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartService: StartService::<Identity, Impl, OFFSET>,
            StopService: StopService::<Identity, Impl, OFFSET>,
            GetServiceStatus: GetServiceStatus::<Identity, Impl, OFFSET>,
            ResetService: ResetService::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITemplateSdo_Impl: ::windows_core::BaseImpl + ISdo_Impl {
    fn AddToCollection(this: &Self::This, bstrname: &::windows_core::BSTR, pcollection: ::core::option::Option<&super::super::System::Com::IDispatch>, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn AddToSdo(this: &Self::This, bstrname: &::windows_core::BSTR, psdotarget: ::core::option::Option<&super::super::System::Com::IDispatch>, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn AddToSdoAsProperty(this: &Self::This, psdotarget: ::core::option::Option<&super::super::System::Com::IDispatch>, id: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITemplateSdo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISdo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITemplateSdo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddToCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcollection: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToCollection(this, ::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&pcollection), ::core::mem::transmute_copy(&ppitem)).into())
        }
        unsafe extern "system" fn AddToSdo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, psdotarget: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToSdo(this, ::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&psdotarget), ::core::mem::transmute_copy(&ppitem)).into())
        }
        unsafe extern "system" fn AddToSdoAsProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psdotarget: *mut ::core::ffi::c_void, id: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToSdoAsProperty(this, ::windows_core::from_raw_borrowed(&psdotarget), ::core::mem::transmute_copy(&id)).into())
        }
        ITemplateSdo_Vtbl {
            base__: <ISdo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddToCollection: AddToCollection::<Identity, Impl, OFFSET>,
            AddToSdo: AddToSdo::<Identity, Impl, OFFSET>,
            AddToSdoAsProperty: AddToSdoAsProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
