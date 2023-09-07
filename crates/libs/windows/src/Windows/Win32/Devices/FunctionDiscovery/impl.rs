#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFunctionDiscovery_Impl: ::windows_core::BaseImpl {
    fn GetInstanceCollection(this: &Self::This, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL) -> ::windows_core::Result<IFunctionInstanceCollection>;
    fn GetInstance(this: &Self::This, pszfunctioninstanceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(this: &Self::This, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::core::option::Option<IFunctionInstanceCollectionQuery>) -> ::windows_core::Result<()>;
    fn CreateInstanceQuery(this: &Self::This, pszfunctioninstanceidentity: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::core::option::Option<IFunctionInstanceQuery>) -> ::windows_core::Result<()>;
    fn AddInstance(this: &Self::This, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, pszcategoryidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(this: &Self::This, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &::windows_core::PCWSTR, pszsubcategory: &::windows_core::PCWSTR, pszcategoryidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IFunctionDiscovery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionDiscovery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInstanceCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstanceCollection(this, ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstance(this, ::core::mem::transmute(&pszfunctioninstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstanceCollectionQuery(this, ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancecollectionquery)).into())
        }
        unsafe extern "system" fn CreateInstanceQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstanceQuery(this, ::core::mem::transmute(&pszfunctioninstanceidentity), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancequery)).into())
        }
        unsafe extern "system" fn AddInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, pszcategoryidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddInstance(this, ::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszcategoryidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscovery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows_core::PCWSTR, pszsubcategory: ::windows_core::PCWSTR, pszcategoryidentity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveInstance(this, ::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszcategoryidentity)).into())
        }
        IFunctionDiscovery_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInstanceCollection: GetInstanceCollection::<Identity, Impl, OFFSET>,
            GetInstance: GetInstance::<Identity, Impl, OFFSET>,
            CreateInstanceCollectionQuery: CreateInstanceCollectionQuery::<Identity, Impl, OFFSET>,
            CreateInstanceQuery: CreateInstanceQuery::<Identity, Impl, OFFSET>,
            AddInstance: AddInstance::<Identity, Impl, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryNotification_Impl: ::windows_core::BaseImpl {
    fn OnUpdate(this: &Self::This, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::core::option::Option<&IFunctionInstance>) -> ::windows_core::Result<()>;
    fn OnError(this: &Self::This, hr: ::windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnEvent(this: &Self::This, dweventid: u32, fdqcquerycontext: u64, pszprovider: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IFunctionDiscoveryNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionDiscoveryNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUpdate(this, ::core::mem::transmute_copy(&enumqueryupdateaction), ::core::mem::transmute_copy(&fdqcquerycontext), ::windows_core::from_raw_borrowed(&pifunctioninstance)).into())
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, fdqcquerycontext: u64, pszprovider: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnError(this, ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pszprovider)).into())
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEvent(this, ::core::mem::transmute_copy(&dweventid), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pszprovider)).into())
        }
        IFunctionDiscoveryNotification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnUpdate: OnUpdate::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProvider_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pifunctiondiscoveryproviderfactory: ::core::option::Option<&IFunctionDiscoveryProviderFactory>, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>, lciduserdefault: u32) -> ::windows_core::Result<u32>;
    fn Query(this: &Self::This, pifunctiondiscoveryproviderquery: ::core::option::Option<&IFunctionDiscoveryProviderQuery>) -> ::windows_core::Result<IFunctionInstanceCollection>;
    fn EndQuery(this: &Self::This) -> ::windows_core::Result<()>;
    fn InstancePropertyStoreValidateAccess(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::Result<()>;
    fn InstancePropertyStoreOpen(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn InstancePropertyStoreFlush(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows_core::Result<()>;
    fn InstanceQueryService(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InstanceReleased(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IFunctionDiscoveryProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionDiscoveryProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: *mut ::core::ffi::c_void, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pifunctiondiscoveryproviderfactory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&lciduserdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstgaccesscapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Query(this, ::windows_core::from_raw_borrowed(&pifunctiondiscoveryproviderquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndQuery(this).into())
        }
        unsafe extern "system" fn InstancePropertyStoreValidateAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstancePropertyStoreValidateAccess(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)).into())
        }
        unsafe extern "system" fn InstancePropertyStoreOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstancePropertyStoreOpen(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstancePropertyStoreFlush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstancePropertyStoreFlush(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into())
        }
        unsafe extern "system" fn InstanceQueryService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, guidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstanceQueryService(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&guidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstanceReleased<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstanceReleased(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into())
        }
        IFunctionDiscoveryProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            EndQuery: EndQuery::<Identity, Impl, OFFSET>,
            InstancePropertyStoreValidateAccess: InstancePropertyStoreValidateAccess::<Identity, Impl, OFFSET>,
            InstancePropertyStoreOpen: InstancePropertyStoreOpen::<Identity, Impl, OFFSET>,
            InstancePropertyStoreFlush: InstancePropertyStoreFlush::<Identity, Impl, OFFSET>,
            InstanceQueryService: InstanceQueryService::<Identity, Impl, OFFSET>,
            InstanceReleased: InstanceReleased::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProviderFactory_Impl: ::windows_core::BaseImpl {
    fn CreatePropertyStore(this: &Self::This) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateInstance(this: &Self::This, pszsubcategory: &::windows_core::PCWSTR, pszproviderinstanceidentity: &::windows_core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pifunctiondiscoveryprovider: ::core::option::Option<&IFunctionDiscoveryProvider>) -> ::windows_core::Result<IFunctionInstance>;
    fn CreateFunctionInstanceCollection(this: &Self::This) -> ::windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IFunctionDiscoveryProviderFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionDiscoveryProviderFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: *mut ::core::ffi::c_void, pifunctiondiscoveryprovider: *mut ::core::ffi::c_void, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this, ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity), ::core::mem::transmute_copy(&iproviderinstancecontext), ::windows_core::from_raw_borrowed(&pipropertystore), ::windows_core::from_raw_borrowed(&pifunctiondiscoveryprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFunctionInstanceCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFunctionInstanceCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFunctionDiscoveryProviderFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePropertyStore: CreatePropertyStore::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            CreateFunctionInstanceCollection: CreateFunctionInstanceCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFunctionDiscoveryProviderQuery_Impl: ::windows_core::BaseImpl {
    fn IsInstanceQuery(this: &Self::This, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn IsSubcategoryQuery(this: &Self::This, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn GetQueryConstraints(this: &Self::This) -> ::windows_core::Result<IProviderQueryConstraintCollection>;
    fn GetPropertyConstraints(this: &Self::This) -> ::windows_core::Result<IProviderPropertyConstraintCollection>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFunctionDiscoveryProviderQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionDiscoveryProviderQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsInstanceQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsInstanceQuery(this, ::core::mem::transmute_copy(&pisinstancequery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into())
        }
        unsafe extern "system" fn IsSubcategoryQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSubcategoryQuery(this, ::core::mem::transmute_copy(&pissubcategoryquery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into())
        }
        unsafe extern "system" fn GetQueryConstraints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQueryConstraints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiproviderqueryconstraints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyConstraints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyConstraints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiproviderpropertyconstraints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFunctionDiscoveryProviderQuery_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsInstanceQuery: IsInstanceQuery::<Identity, Impl, OFFSET>,
            IsSubcategoryQuery: IsSubcategoryQuery::<Identity, Impl, OFFSET>,
            GetQueryConstraints: GetQueryConstraints::<Identity, Impl, OFFSET>,
            GetPropertyConstraints: GetPropertyConstraints::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryServiceProvider_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IFunctionDiscoveryServiceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryServiceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionDiscoveryServiceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionDiscoveryServiceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IFunctionDiscoveryServiceProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstance_Impl: ::windows_core::BaseImpl + super::super::System::Com::IServiceProvider_Impl {
    fn GetID(this: &Self::This) -> ::windows_core::Result<*mut u16>;
    fn GetProviderInstanceID(this: &Self::This) -> ::windows_core::Result<*mut u16>;
    fn OpenPropertyStore(this: &Self::This, dwstgaccess: super::super::System::Com::STGM) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetCategory(this: &Self::This, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IFunctionInstance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IServiceProvider);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionInstance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemidentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderInstanceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderInstanceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemproviderinstanceidentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstgaccess: super::super::System::Com::STGM, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenPropertyStore(this, ::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCategory(this, ::core::mem::transmute_copy(&ppszcomemcategory), ::core::mem::transmute_copy(&ppszcomemsubcategory)).into())
        }
        IFunctionInstance_Vtbl {
            base__: <super::super::System::Com::IServiceProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Identity, Impl, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Get(this: &Self::This, pszinstanceidentity: &::windows_core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::core::option::Option<IFunctionInstance>) -> ::windows_core::Result<()>;
    fn Item(this: &Self::This, dwindex: u32) -> ::windows_core::Result<IFunctionInstance>;
    fn Add(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, dwindex: u32) -> ::windows_core::Result<IFunctionInstance>;
    fn Delete(this: &Self::This, dwindex: u32) -> ::windows_core::Result<()>;
    fn DeleteAll(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IFunctionInstanceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionInstanceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows_core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppifunctioninstance)).into())
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&pifunctioninstance)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Remove(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&dwindex)).into())
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAll(this).into())
        }
        IFunctionInstanceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstanceCollectionQuery_Impl: ::windows_core::BaseImpl {
    fn AddQueryConstraint(this: &Self::This, pszconstraintname: &::windows_core::PCWSTR, pszconstraintvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddPropertyConstraint(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows_core::Result<()>;
    fn Execute(this: &Self::This) -> ::windows_core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IFunctionInstanceCollectionQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionInstanceCollectionQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddQueryConstraint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows_core::PCWSTR, pszconstraintvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddQueryConstraint(this, ::core::mem::transmute(&pszconstraintname), ::core::mem::transmute(&pszconstraintvalue)).into())
        }
        unsafe extern "system" fn AddPropertyConstraint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyConstraint(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&enumpropertyconstraint)).into())
        }
        unsafe extern "system" fn Execute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Execute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstancecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFunctionInstanceCollectionQuery_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddQueryConstraint: AddQueryConstraint::<Identity, Impl, OFFSET>,
            AddPropertyConstraint: AddPropertyConstraint::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceQuery_Impl: ::windows_core::BaseImpl {
    fn Execute(this: &Self::This) -> ::windows_core::Result<IFunctionInstance>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IFunctionInstanceQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFunctionInstanceQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Execute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFunctionInstanceQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Execute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFunctionInstanceQuery_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Execute: Execute::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPNPXAssociation_Impl: ::windows_core::BaseImpl {
    fn Associate(this: &Self::This, pszsubcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Unassociate(this: &Self::This, pszsubcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, pszsubcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPNPXAssociation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPNPXAssociation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Associate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Associate(this, ::core::mem::transmute(&pszsubcategory)).into())
        }
        unsafe extern "system" fn Unassociate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unassociate(this, ::core::mem::transmute(&pszsubcategory)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXAssociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&pszsubcategory)).into())
        }
        IPNPXAssociation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Associate: Associate::<Identity, Impl, OFFSET>,
            Unassociate: Unassociate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPNPXDeviceAssociation_Impl: ::windows_core::BaseImpl {
    fn Associate(this: &Self::This, pszsubcategory: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>) -> ::windows_core::Result<()>;
    fn Unassociate(this: &Self::This, pszsubcategory: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, pszsubcategory: &::windows_core::PCWSTR, pifunctiondiscoverynotification: ::core::option::Option<&IFunctionDiscoveryNotification>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPNPXDeviceAssociation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPNPXDeviceAssociation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Associate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Associate(this, ::core::mem::transmute(&pszsubcategory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into())
        }
        unsafe extern "system" fn Unassociate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unassociate(this, ::core::mem::transmute(&pszsubcategory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows_core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&pszsubcategory), ::windows_core::from_raw_borrowed(&pifunctiondiscoverynotification)).into())
        }
        IPNPXDeviceAssociation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Associate: Associate::<Identity, Impl, OFFSET>,
            Unassociate: Unassociate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPropertyStoreCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Get(this: &Self::This, pszinstanceidentity: &::windows_core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Item(this: &Self::This, dwindex: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Add(this: &Self::This, pipropertystore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, dwindex: u32) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Delete(this: &Self::This, dwindex: u32) -> ::windows_core::Result<()>;
    fn DeleteAll(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IPropertyStoreCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyStoreCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows_core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppipropertystore)).into())
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&pipropertystore)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Remove(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pipropertystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&dwindex)).into())
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAll(this).into())
        }
        IPropertyStoreCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderProperties_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(this: &Self::This, pifunctioninstance: ::core::option::Option<&IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IProviderProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProviderProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::windows_core::from_raw_borrowed(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar)).into())
        }
        IProviderProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderPropertyConstraintCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Get(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()>;
    fn Item(this: &Self::This, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IProviderPropertyConstraintCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProviderPropertyConstraintCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into())
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Item(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IProviderPropertyConstraintCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProviderPublishing_Impl: ::windows_core::BaseImpl {
    fn CreateInstance(this: &Self::This, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &::windows_core::PCWSTR, pszproviderinstanceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<IFunctionInstance>;
    fn RemoveInstance(this: &Self::This, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &::windows_core::PCWSTR, pszproviderinstanceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IProviderPublishing {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPublishing_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProviderPublishing {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPublishing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this, ::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifunctioninstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderPublishing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows_core::PCWSTR, pszproviderinstanceidentity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveInstance(this, ::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity)).into())
        }
        IProviderPublishing_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IProviderQueryConstraintCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Get(this: &Self::This, pszconstraintname: &::windows_core::PCWSTR) -> ::windows_core::Result<*mut u16>;
    fn Item(this: &Self::This, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IProviderQueryConstraintCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProviderQueryConstraintCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows_core::PCWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Get(this, ::core::mem::transmute(&pszconstraintname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszconstraintvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Item(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IProviderQueryConstraintCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
