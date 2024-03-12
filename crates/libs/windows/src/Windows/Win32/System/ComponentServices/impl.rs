#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn IsInTransaction(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransaction(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetTransactionId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetActivityId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetContextId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ContextInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ContextInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisintx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransactionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtxid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransactionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtxid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActivityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstractivityid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivityId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstractivityid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContextId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrctxid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContextId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrctxid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ContextInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, Impl, OFFSET>,
            GetActivityId: GetActivityId::<Identity, Impl, OFFSET>,
            GetContextId: GetContextId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextInfo2_Impl: ::windows_core::BaseImpl + ContextInfo_Impl {
    fn GetPartitionId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetApplicationId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetApplicationInstanceId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ContextInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ContextInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ContextInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPartitionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartitionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20000, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetApplicationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplicationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20001, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplicationInstanceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20002, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ContextInfo2_Vtbl {
            base__: <ContextInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPartitionId: GetPartitionId::<Identity, Impl, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, Impl, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAppDomainHelper_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Initialize(this: &Self::This, punkad: ::core::option::Option<&::windows_core::IUnknown>, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DoCallback(this: &Self::This, punkad: ::core::option::Option<&::windows_core::IUnknown>, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAppDomainHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppDomainHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0000), ::core::mem::transmute_copy(&ppool)).into())
        }
        unsafe extern "system" fn DoCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoCallback(this, ::windows_core::from_raw_borrowed(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0001), ::core::mem::transmute_copy(&ppool)).into())
        }
        IAppDomainHelper_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            DoCallback: DoCallback::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAssemblyLocator_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetModules(this: &Self::This, applicationdir: &::windows_core::BSTR, applicationname: &::windows_core::BSTR, assemblyname: &::windows_core::BSTR) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAssemblyLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAssemblyLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetModules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, applicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, assemblyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetModules(this, ::core::mem::transmute(&applicationdir), ::core::mem::transmute(&applicationname), ::core::mem::transmute(&assemblyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmodules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAssemblyLocator_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetModules: GetModules::<Identity, Impl, OFFSET> }
    };
}
pub trait IAsyncErrorNotify_Impl: ::windows_core::BaseImpl {
    fn OnError(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAsyncErrorNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsyncErrorNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnError(this, ::core::mem::transmute_copy(&hr)).into())
        }
        IAsyncErrorNotify_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnError: OnError::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICOMAdminCatalog_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetCollection(this: &Self::This, bstrcollname: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Connect(this: &Self::This, bstrcatalogservername: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetCollectionByQuery(this: &Self::This, bstrcollname: &::windows_core::BSTR, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<super::Com::IDispatch>;
    fn ImportComponent(this: &Self::This, bstrapplidorname: &::windows_core::BSTR, bstrclsidorprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InstallComponent(this: &Self::This, bstrapplidorname: &::windows_core::BSTR, bstrdll: &::windows_core::BSTR, bstrtlb: &::windows_core::BSTR, bstrpsdll: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ShutdownApplication(this: &Self::This, bstrapplidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExportApplication(this: &Self::This, bstrapplidorname: &::windows_core::BSTR, bstrapplicationfile: &::windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows_core::Result<()>;
    fn InstallApplication(this: &Self::This, bstrapplicationfile: &::windows_core::BSTR, bstrdestinationdirectory: &::windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bstrrsn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StopRouter(this: &Self::This) -> ::windows_core::Result<()>;
    fn RefreshRouter(this: &Self::This) -> ::windows_core::Result<()>;
    fn StartRouter(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reserved1(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reserved2(this: &Self::This) -> ::windows_core::Result<()>;
    fn InstallMultipleComponents(this: &Self::This, bstrapplidorname: &::windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn GetMultipleComponentsInfo(this: &Self::This, bstrapplidorname: &::windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn RefreshComponents(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackupREGDB(this: &Self::This, bstrbackupfilepath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RestoreREGDB(this: &Self::This, bstrbackupfilepath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryApplicationFile(this: &Self::This, bstrapplicationfile: &::windows_core::BSTR, pbstrapplicationname: *mut ::windows_core::BSTR, pbstrapplicationdescription: *mut ::windows_core::BSTR, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn StartApplication(this: &Self::This, bstrapplidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceCheck(this: &Self::This, lservice: i32) -> ::windows_core::Result<i32>;
    fn InstallMultipleEventClasses(this: &Self::This, bstrapplidorname: &::windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn InstallEventClass(this: &Self::This, bstrapplidorname: &::windows_core::BSTR, bstrdll: &::windows_core::BSTR, bstrtlb: &::windows_core::BSTR, bstrpsdll: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetEventClassesForIID(this: &Self::This, bstriid: &::windows_core::BSTR, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICOMAdminCatalog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICOMAdminCatalog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCollection(this, ::core::mem::transmute(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Connect(this, ::core::mem::transmute(&bstrcatalogservername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MajorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCollectionByQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCollectionByQuery(this, ::core::mem::transmute(&bstrcollname), ::core::mem::transmute_copy(&ppsavarquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImportComponent(this, ::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrclsidorprogid)).into())
        }
        unsafe extern "system" fn InstallComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdll: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtlb: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpsdll: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallComponent(this, ::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrdll), ::core::mem::transmute(&bstrtlb), ::core::mem::transmute(&bstrpsdll)).into())
        }
        unsafe extern "system" fn ShutdownApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownApplication(this, ::core::mem::transmute(&bstrapplidorname)).into())
        }
        unsafe extern "system" fn ExportApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExportApplication(this, ::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute_copy(&loptions)).into())
        }
        unsafe extern "system" fn InstallApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestinationdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrsn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallApplication(this, ::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute(&bstrdestinationdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrrsn)).into())
        }
        unsafe extern "system" fn StopRouter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopRouter(this).into())
        }
        unsafe extern "system" fn RefreshRouter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshRouter(this).into())
        }
        unsafe extern "system" fn StartRouter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartRouter(this).into())
        }
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved1(this).into())
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved2(this).into())
        }
        unsafe extern "system" fn InstallMultipleComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallMultipleComponents(this, ::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into())
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMultipleComponentsInfo(this, ::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarclassnames), ::core::mem::transmute_copy(&ppsavarfileflags), ::core::mem::transmute_copy(&ppsavarcomponentflags)).into())
        }
        unsafe extern "system" fn RefreshComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshComponents(this).into())
        }
        unsafe extern "system" fn BackupREGDB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackupREGDB(this, ::core::mem::transmute(&bstrbackupfilepath)).into())
        }
        unsafe extern "system" fn RestoreREGDB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreREGDB(this, ::core::mem::transmute(&bstrbackupfilepath)).into())
        }
        unsafe extern "system" fn QueryApplicationFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrapplicationname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrapplicationdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryApplicationFile(this, ::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute_copy(&pbstrapplicationname), ::core::mem::transmute_copy(&pbstrapplicationdescription), ::core::mem::transmute_copy(&pbhasusers), ::core::mem::transmute_copy(&pbisproxy), ::core::mem::transmute_copy(&ppsavarfilenames)).into())
        }
        unsafe extern "system" fn StartApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartApplication(this, ::core::mem::transmute(&bstrapplidorname)).into())
        }
        unsafe extern "system" fn ServiceCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceCheck(this, ::core::mem::transmute_copy(&lservice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallMultipleEventClasses(this, ::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into())
        }
        unsafe extern "system" fn InstallEventClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdll: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtlb: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpsdll: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallEventClass(this, ::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrdll), ::core::mem::transmute(&bstrtlb), ::core::mem::transmute(&bstrpsdll)).into())
        }
        unsafe extern "system" fn GetEventClassesForIID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstriid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEventClassesForIID(this, ::core::mem::transmute(&bstriid), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarprogids), ::core::mem::transmute_copy(&ppsavardescriptions)).into())
        }
        ICOMAdminCatalog_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            GetCollectionByQuery: GetCollectionByQuery::<Identity, Impl, OFFSET>,
            ImportComponent: ImportComponent::<Identity, Impl, OFFSET>,
            InstallComponent: InstallComponent::<Identity, Impl, OFFSET>,
            ShutdownApplication: ShutdownApplication::<Identity, Impl, OFFSET>,
            ExportApplication: ExportApplication::<Identity, Impl, OFFSET>,
            InstallApplication: InstallApplication::<Identity, Impl, OFFSET>,
            StopRouter: StopRouter::<Identity, Impl, OFFSET>,
            RefreshRouter: RefreshRouter::<Identity, Impl, OFFSET>,
            StartRouter: StartRouter::<Identity, Impl, OFFSET>,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            InstallMultipleComponents: InstallMultipleComponents::<Identity, Impl, OFFSET>,
            GetMultipleComponentsInfo: GetMultipleComponentsInfo::<Identity, Impl, OFFSET>,
            RefreshComponents: RefreshComponents::<Identity, Impl, OFFSET>,
            BackupREGDB: BackupREGDB::<Identity, Impl, OFFSET>,
            RestoreREGDB: RestoreREGDB::<Identity, Impl, OFFSET>,
            QueryApplicationFile: QueryApplicationFile::<Identity, Impl, OFFSET>,
            StartApplication: StartApplication::<Identity, Impl, OFFSET>,
            ServiceCheck: ServiceCheck::<Identity, Impl, OFFSET>,
            InstallMultipleEventClasses: InstallMultipleEventClasses::<Identity, Impl, OFFSET>,
            InstallEventClass: InstallEventClass::<Identity, Impl, OFFSET>,
            GetEventClassesForIID: GetEventClassesForIID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICOMAdminCatalog2_Impl: ::windows_core::BaseImpl + ICOMAdminCatalog_Impl {
    fn GetCollectionByQuery2(this: &Self::This, bstrcollectionname: &::windows_core::BSTR, pvarquerystrings: *const super::Variant::VARIANT) -> ::windows_core::Result<super::Com::IDispatch>;
    fn GetApplicationInstanceIDFromProcessID(this: &Self::This, lprocessid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ShutdownApplicationInstances(this: &Self::This, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PauseApplicationInstances(this: &Self::This, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ResumeApplicationInstances(this: &Self::This, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RecycleApplicationInstances(this: &Self::This, pvarapplicationinstanceid: *const super::Variant::VARIANT, lreasoncode: i32) -> ::windows_core::Result<()>;
    fn AreApplicationInstancesPaused(this: &Self::This, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DumpApplicationInstance(this: &Self::This, bstrapplicationinstanceid: &::windows_core::BSTR, bstrdirectory: &::windows_core::BSTR, lmaximages: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsApplicationInstanceDumpSupported(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CreateServiceForApplication(this: &Self::This, bstrapplicationidorname: &::windows_core::BSTR, bstrservicename: &::windows_core::BSTR, bstrstarttype: &::windows_core::BSTR, bstrerrorcontrol: &::windows_core::BSTR, bstrdependencies: &::windows_core::BSTR, bstrrunas: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DeleteServiceForApplication(this: &Self::This, bstrapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPartitionID(this: &Self::This, bstrapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPartitionName(this: &Self::This, bstrapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCurrentPartition(this: &Self::This, bstrpartitionidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CurrentPartitionID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentPartitionName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GlobalPartitionID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FlushPartitionCache(this: &Self::This) -> ::windows_core::Result<()>;
    fn CopyApplications(this: &Self::This, bstrsourcepartitionidorname: &::windows_core::BSTR, pvarapplicationid: *const super::Variant::VARIANT, bstrdestinationpartitionidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CopyComponents(this: &Self::This, bstrsourceapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MoveComponents(this: &Self::This, bstrsourceapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AliasComponent(this: &Self::This, bstrsrcapplicationidorname: &::windows_core::BSTR, bstrclsidorprogid: &::windows_core::BSTR, bstrdestapplicationidorname: &::windows_core::BSTR, bstrnewprogid: &::windows_core::BSTR, bstrnewclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsSafeToDelete(this: &Self::This, bstrdllname: &::windows_core::BSTR) -> ::windows_core::Result<COMAdminInUse>;
    fn ImportUnconfiguredComponents(this: &Self::This, bstrapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PromoteUnconfiguredComponents(this: &Self::This, bstrapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ImportComponents(this: &Self::This, bstrapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Is64BitCatalogServer(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExportPartition(this: &Self::This, bstrpartitionidorname: &::windows_core::BSTR, bstrpartitionfilename: &::windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows_core::Result<()>;
    fn InstallPartition(this: &Self::This, bstrfilename: &::windows_core::BSTR, bstrdestdirectory: &::windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bstrrsn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryApplicationFile2(this: &Self::This, bstrapplicationfile: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn GetComponentVersionCount(this: &Self::This, bstrclsidorprogid: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICOMAdminCatalog2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICOMAdminCatalog);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICOMAdminCatalog2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCollectionByQuery2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcollectionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarquerystrings: *const super::Variant::VARIANT, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCollectionByQuery2(this, ::core::mem::transmute(&bstrcollectionname), ::core::mem::transmute_copy(&pvarquerystrings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplicationInstanceIDFromProcessID(this, ::core::mem::transmute_copy(&lprocessid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownApplicationInstances(this, ::core::mem::transmute_copy(&pvarapplicationinstanceid)).into())
        }
        unsafe extern "system" fn PauseApplicationInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PauseApplicationInstances(this, ::core::mem::transmute_copy(&pvarapplicationinstanceid)).into())
        }
        unsafe extern "system" fn ResumeApplicationInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeApplicationInstances(this, ::core::mem::transmute_copy(&pvarapplicationinstanceid)).into())
        }
        unsafe extern "system" fn RecycleApplicationInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT, lreasoncode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecycleApplicationInstances(this, ::core::mem::transmute_copy(&pvarapplicationinstanceid), ::core::mem::transmute_copy(&lreasoncode)).into())
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT, pvarboolpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AreApplicationInstancesPaused(this, ::core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarboolpaused, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DumpApplicationInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, lmaximages: i32, pbstrdumpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DumpApplicationInstance(this, ::core::mem::transmute(&bstrapplicationinstanceid), ::core::mem::transmute(&bstrdirectory), ::core::mem::transmute_copy(&lmaximages)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdumpfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsApplicationInstanceDumpSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbooldumpsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateServiceForApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrstarttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrerrorcontrol: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdependencies: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrunas: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateServiceForApplication(this, ::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute(&bstrservicename), ::core::mem::transmute(&bstrstarttype), ::core::mem::transmute(&bstrerrorcontrol), ::core::mem::transmute(&bstrdependencies), ::core::mem::transmute(&bstrrunas), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&bdesktopok)).into())
        }
        unsafe extern "system" fn DeleteServiceForApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteServiceForApplication(this, ::core::mem::transmute(&bstrapplicationidorname)).into())
        }
        unsafe extern "system" fn GetPartitionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrpartitionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartitionID(this, ::core::mem::transmute(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartitionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrpartitionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartitionName(this, ::core::mem::transmute(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCurrentPartition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentPartition(this, ::core::mem::transmute(&bstrpartitionidorname)).into())
        }
        unsafe extern "system" fn CurrentPartitionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPartitionID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentPartitionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPartitionName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GlobalPartitionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GlobalPartitionID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrglobalpartitionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FlushPartitionCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushPartitionCache(this).into())
        }
        unsafe extern "system" fn CopyApplications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarapplicationid: *const super::Variant::VARIANT, bstrdestinationpartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyApplications(this, ::core::mem::transmute(&bstrsourcepartitionidorname), ::core::mem::transmute_copy(&pvarapplicationid), ::core::mem::transmute(&bstrdestinationpartitionidorname)).into())
        }
        unsafe extern "system" fn CopyComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyComponents(this, ::core::mem::transmute(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute(&bstrdestinationapplicationidorname)).into())
        }
        unsafe extern "system" fn MoveComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveComponents(this, ::core::mem::transmute(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute(&bstrdestinationapplicationidorname)).into())
        }
        unsafe extern "system" fn AliasComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AliasComponent(this, ::core::mem::transmute(&bstrsrcapplicationidorname), ::core::mem::transmute(&bstrclsidorprogid), ::core::mem::transmute(&bstrdestapplicationidorname), ::core::mem::transmute(&bstrnewprogid), ::core::mem::transmute(&bstrnewclsid)).into())
        }
        unsafe extern "system" fn IsSafeToDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdllname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSafeToDelete(this, ::core::mem::transmute(&bstrdllname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomadmininuse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImportUnconfiguredComponents(this, ::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into())
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PromoteUnconfiguredComponents(this, ::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into())
        }
        unsafe extern "system" fn ImportComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImportComponents(this, ::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into())
        }
        unsafe extern "system" fn Is64BitCatalogServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbis64bit: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Is64BitCatalogServer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbis64bit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExportPartition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpartitionfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExportPartition(this, ::core::mem::transmute(&bstrpartitionidorname), ::core::mem::transmute(&bstrpartitionfilename), ::core::mem::transmute_copy(&loptions)).into())
        }
        unsafe extern "system" fn InstallPartition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrsn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallPartition(this, ::core::mem::transmute(&bstrfilename), ::core::mem::transmute(&bstrdestdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrrsn)).into())
        }
        unsafe extern "system" fn QueryApplicationFile2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfilesforimport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryApplicationFile2(this, ::core::mem::transmute(&bstrapplicationfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfilesforimport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetComponentVersionCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, plversioncount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetComponentVersionCount(this, ::core::mem::transmute(&bstrclsidorprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plversioncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICOMAdminCatalog2_Vtbl {
            base__: <ICOMAdminCatalog as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCollectionByQuery2: GetCollectionByQuery2::<Identity, Impl, OFFSET>,
            GetApplicationInstanceIDFromProcessID: GetApplicationInstanceIDFromProcessID::<Identity, Impl, OFFSET>,
            ShutdownApplicationInstances: ShutdownApplicationInstances::<Identity, Impl, OFFSET>,
            PauseApplicationInstances: PauseApplicationInstances::<Identity, Impl, OFFSET>,
            ResumeApplicationInstances: ResumeApplicationInstances::<Identity, Impl, OFFSET>,
            RecycleApplicationInstances: RecycleApplicationInstances::<Identity, Impl, OFFSET>,
            AreApplicationInstancesPaused: AreApplicationInstancesPaused::<Identity, Impl, OFFSET>,
            DumpApplicationInstance: DumpApplicationInstance::<Identity, Impl, OFFSET>,
            IsApplicationInstanceDumpSupported: IsApplicationInstanceDumpSupported::<Identity, Impl, OFFSET>,
            CreateServiceForApplication: CreateServiceForApplication::<Identity, Impl, OFFSET>,
            DeleteServiceForApplication: DeleteServiceForApplication::<Identity, Impl, OFFSET>,
            GetPartitionID: GetPartitionID::<Identity, Impl, OFFSET>,
            GetPartitionName: GetPartitionName::<Identity, Impl, OFFSET>,
            SetCurrentPartition: SetCurrentPartition::<Identity, Impl, OFFSET>,
            CurrentPartitionID: CurrentPartitionID::<Identity, Impl, OFFSET>,
            CurrentPartitionName: CurrentPartitionName::<Identity, Impl, OFFSET>,
            GlobalPartitionID: GlobalPartitionID::<Identity, Impl, OFFSET>,
            FlushPartitionCache: FlushPartitionCache::<Identity, Impl, OFFSET>,
            CopyApplications: CopyApplications::<Identity, Impl, OFFSET>,
            CopyComponents: CopyComponents::<Identity, Impl, OFFSET>,
            MoveComponents: MoveComponents::<Identity, Impl, OFFSET>,
            AliasComponent: AliasComponent::<Identity, Impl, OFFSET>,
            IsSafeToDelete: IsSafeToDelete::<Identity, Impl, OFFSET>,
            ImportUnconfiguredComponents: ImportUnconfiguredComponents::<Identity, Impl, OFFSET>,
            PromoteUnconfiguredComponents: PromoteUnconfiguredComponents::<Identity, Impl, OFFSET>,
            ImportComponents: ImportComponents::<Identity, Impl, OFFSET>,
            Is64BitCatalogServer: Is64BitCatalogServer::<Identity, Impl, OFFSET>,
            ExportPartition: ExportPartition::<Identity, Impl, OFFSET>,
            InstallPartition: InstallPartition::<Identity, Impl, OFFSET>,
            QueryApplicationFile2: QueryApplicationFile2::<Identity, Impl, OFFSET>,
            GetComponentVersionCount: GetComponentVersionCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICOMLBArguments_Impl: ::windows_core::BaseImpl {
    fn GetCLSID(this: &Self::This, pclsid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetCLSID(this: &Self::This, pclsid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetMachineName(this: &Self::This, cchsvr: u32, szservername: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetMachineName(this: &Self::This, cchsvr: u32, szservername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICOMLBArguments {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICOMLBArguments {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCLSID(this, ::core::mem::transmute_copy(&pclsid)).into())
        }
        unsafe extern "system" fn SetCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCLSID(this, ::core::mem::transmute_copy(&pclsid)).into())
        }
        unsafe extern "system" fn GetMachineName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMachineName(this, ::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute_copy(&szservername)).into())
        }
        unsafe extern "system" fn SetMachineName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMachineName(this, ::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute(&szservername)).into())
        }
        ICOMLBArguments_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            GetMachineName: GetMachineName::<Identity, Impl, OFFSET>,
            SetMachineName: SetMachineName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalogCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Remove(this: &Self::This, lindex: i32) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Populate(this: &Self::This) -> ::windows_core::Result<()>;
    fn SaveChanges(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetCollection(this: &Self::This, bstrcollname: &::windows_core::BSTR, varobjectkey: &super::Variant::VARIANT) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Name(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn AddEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RemoveEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetUtilInterface(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn DataStoreMajorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DataStoreMinorVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PopulateByKey(this: &Self::This, psakeys: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn PopulateByQuery(this: &Self::This, bstrquerystring: &::windows_core::BSTR, lquerytype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICatalogCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICatalogCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plobjectcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&lindex)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Populate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Populate(this).into())
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveChanges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchanges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varobjectkey: super::Variant::VARIANT, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCollection(this, ::core::mem::transmute(&bstrcollname), ::core::mem::transmute(&varobjectkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarnamel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoveEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUtilInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppidispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUtilInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidispatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DataStoreMajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataStoreMajorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DataStoreMinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataStoreMinorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversionl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PopulateByKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopulateByKey(this, ::core::mem::transmute_copy(&psakeys)).into())
        }
        unsafe extern "system" fn PopulateByQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrquerystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, lquerytype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopulateByQuery(this, ::core::mem::transmute(&bstrquerystring), ::core::mem::transmute_copy(&lquerytype)).into())
        }
        ICatalogCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Populate: Populate::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            AddEnabled: AddEnabled::<Identity, Impl, OFFSET>,
            RemoveEnabled: RemoveEnabled::<Identity, Impl, OFFSET>,
            GetUtilInterface: GetUtilInterface::<Identity, Impl, OFFSET>,
            DataStoreMajorVersion: DataStoreMajorVersion::<Identity, Impl, OFFSET>,
            DataStoreMinorVersion: DataStoreMinorVersion::<Identity, Impl, OFFSET>,
            PopulateByKey: PopulateByKey::<Identity, Impl, OFFSET>,
            PopulateByQuery: PopulateByQuery::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalogObject_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Value(this: &Self::This, bstrpropname: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn put_Value(this: &Self::This, bstrpropname: &::windows_core::BSTR, val: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Key(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Name(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsPropertyReadOnly(this: &Self::This, bstrpropname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Valid(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPropertyWriteOnly(this: &Self::This, bstrpropname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICatalogObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICatalogObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarretval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Value(this, ::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, val: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_Value(this, ::core::mem::transmute(&bstrpropname), ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn Key<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Key(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPropertyReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPropertyReadOnly(this, ::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Valid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Valid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPropertyWriteOnly(this, ::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICatalogObject_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Value: get_Value::<Identity, Impl, OFFSET>,
            put_Value: put_Value::<Identity, Impl, OFFSET>,
            Key: Key::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsPropertyReadOnly: IsPropertyReadOnly::<Identity, Impl, OFFSET>,
            Valid: Valid::<Identity, Impl, OFFSET>,
            IsPropertyWriteOnly: IsPropertyWriteOnly::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICheckSxsConfig_Impl: ::windows_core::BaseImpl {
    fn IsSameSxsConfig(this: &Self::This, wszsxsname: &::windows_core::PCWSTR, wszsxsdirectory: &::windows_core::PCWSTR, wszsxsappname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICheckSxsConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICheckSxsConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsSameSxsConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszsxsname: ::windows_core::PCWSTR, wszsxsdirectory: ::windows_core::PCWSTR, wszsxsappname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSameSxsConfig(this, ::core::mem::transmute(&wszsxsname), ::core::mem::transmute(&wszsxsdirectory), ::core::mem::transmute(&wszsxsappname)).into())
        }
        ICheckSxsConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsSameSxsConfig: IsSameSxsConfig::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComActivityEvents_Impl: ::windows_core::BaseImpl {
    fn OnActivityCreate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnActivityDestroy(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnActivityEnter(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32) -> ::windows_core::Result<()>;
    fn OnActivityTimeout(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows_core::Result<()>;
    fn OnActivityReenter(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows_core::Result<()>;
    fn OnActivityLeave(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidleft: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnActivityLeaveSame(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwcalldepth: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComActivityEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComActivityEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnActivityCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityCreate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into())
        }
        unsafe extern "system" fn OnActivityDestroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityDestroy(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into())
        }
        unsafe extern "system" fn OnActivityEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityEnter(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread)).into())
        }
        unsafe extern "system" fn OnActivityTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityTimeout(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtimeout)).into())
        }
        unsafe extern "system" fn OnActivityReenter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityReenter(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwcalldepth)).into())
        }
        unsafe extern "system" fn OnActivityLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidleft: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityLeave(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidleft)).into())
        }
        unsafe extern "system" fn OnActivityLeaveSame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwcalldepth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivityLeaveSame(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwcalldepth)).into())
        }
        IComActivityEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnActivityCreate: OnActivityCreate::<Identity, Impl, OFFSET>,
            OnActivityDestroy: OnActivityDestroy::<Identity, Impl, OFFSET>,
            OnActivityEnter: OnActivityEnter::<Identity, Impl, OFFSET>,
            OnActivityTimeout: OnActivityTimeout::<Identity, Impl, OFFSET>,
            OnActivityReenter: OnActivityReenter::<Identity, Impl, OFFSET>,
            OnActivityLeave: OnActivityLeave::<Identity, Impl, OFFSET>,
            OnActivityLeaveSame: OnActivityLeaveSame::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComApp2Events_Impl: ::windows_core::BaseImpl {
    fn OnAppActivation2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID, guidprocess: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppShutdown2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppForceShutdown2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppPaused2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnAppRecycle2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID, guidprocess: &::windows_core::GUID, lreason: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComApp2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComApp2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnAppActivation2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, guidprocess: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppActivation2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute(&guidprocess)).into())
        }
        unsafe extern "system" fn OnAppShutdown2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppShutdown2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        unsafe extern "system" fn OnAppForceShutdown2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppForceShutdown2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        unsafe extern "system" fn OnAppPaused2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppPaused2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute_copy(&bpaused)).into())
        }
        unsafe extern "system" fn OnAppRecycle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, guidprocess: ::windows_core::GUID, lreason: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppRecycle2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute(&guidprocess), ::core::mem::transmute_copy(&lreason)).into())
        }
        IComApp2Events_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnAppActivation2: OnAppActivation2::<Identity, Impl, OFFSET>,
            OnAppShutdown2: OnAppShutdown2::<Identity, Impl, OFFSET>,
            OnAppForceShutdown2: OnAppForceShutdown2::<Identity, Impl, OFFSET>,
            OnAppPaused2: OnAppPaused2::<Identity, Impl, OFFSET>,
            OnAppRecycle2: OnAppRecycle2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComAppEvents_Impl: ::windows_core::BaseImpl {
    fn OnAppActivation(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppShutdown(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppForceShutdown(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComAppEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComAppEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnAppActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppActivation(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        unsafe extern "system" fn OnAppShutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppShutdown(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        unsafe extern "system" fn OnAppForceShutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppForceShutdown(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        IComAppEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnAppActivation: OnAppActivation::<Identity, Impl, OFFSET>,
            OnAppShutdown: OnAppShutdown::<Identity, Impl, OFFSET>,
            OnAppForceShutdown: OnAppForceShutdown::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComCRMEvents_Impl: ::windows_core::BaseImpl {
    fn OnCRMRecoveryStart(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMRecoveryDone(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMCheckpoint(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMBegin(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, guidactivity: &::windows_core::GUID, guidtx: &::windows_core::GUID, szprogidcompensator: &::windows_core::PCWSTR, szdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnCRMPrepare(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMCommit(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMAbort(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMIndoubt(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMDone(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMRelease(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMAnalyze(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows_core::Result<()>;
    fn OnCRMWrite(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::Result<()>;
    fn OnCRMForget(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMForce(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMDeliver(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComCRMEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComCRMEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCRMRecoveryStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMRecoveryStart(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMRecoveryDone(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        unsafe extern "system" fn OnCRMCheckpoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMCheckpoint(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into())
        }
        unsafe extern "system" fn OnCRMBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, guidactivity: ::windows_core::GUID, guidtx: ::windows_core::GUID, szprogidcompensator: ::windows_core::PCWSTR, szdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMBegin(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute(&guidactivity), ::core::mem::transmute(&guidtx), ::core::mem::transmute(&szprogidcompensator), ::core::mem::transmute(&szdescription)).into())
        }
        unsafe extern "system" fn OnCRMPrepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMPrepare(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMCommit(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMAbort(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMIndoubt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMIndoubt(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMDone(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMRelease<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMRelease(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMAnalyze<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMAnalyze(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&dwcrmrecordtype), ::core::mem::transmute_copy(&dwrecordsize)).into())
        }
        unsafe extern "system" fn OnCRMWrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMWrite(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into())
        }
        unsafe extern "system" fn OnCRMForget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMForget(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMForce<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMForce(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into())
        }
        unsafe extern "system" fn OnCRMDeliver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCRMDeliver(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into())
        }
        IComCRMEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCRMRecoveryStart: OnCRMRecoveryStart::<Identity, Impl, OFFSET>,
            OnCRMRecoveryDone: OnCRMRecoveryDone::<Identity, Impl, OFFSET>,
            OnCRMCheckpoint: OnCRMCheckpoint::<Identity, Impl, OFFSET>,
            OnCRMBegin: OnCRMBegin::<Identity, Impl, OFFSET>,
            OnCRMPrepare: OnCRMPrepare::<Identity, Impl, OFFSET>,
            OnCRMCommit: OnCRMCommit::<Identity, Impl, OFFSET>,
            OnCRMAbort: OnCRMAbort::<Identity, Impl, OFFSET>,
            OnCRMIndoubt: OnCRMIndoubt::<Identity, Impl, OFFSET>,
            OnCRMDone: OnCRMDone::<Identity, Impl, OFFSET>,
            OnCRMRelease: OnCRMRelease::<Identity, Impl, OFFSET>,
            OnCRMAnalyze: OnCRMAnalyze::<Identity, Impl, OFFSET>,
            OnCRMWrite: OnCRMWrite::<Identity, Impl, OFFSET>,
            OnCRMForget: OnCRMForget::<Identity, Impl, OFFSET>,
            OnCRMForce: OnCRMForce::<Identity, Impl, OFFSET>,
            OnCRMDeliver: OnCRMDeliver::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComExceptionEvents_Impl: ::windows_core::BaseImpl {
    fn OnExceptionUser(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComExceptionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComExceptionEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnExceptionUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnExceptionUser(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&code), ::core::mem::transmute_copy(&address), ::core::mem::transmute(&pszstacktrace)).into())
        }
        IComExceptionEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnExceptionUser: OnExceptionUser::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComIdentityEvents_Impl: ::windows_core::BaseImpl {
    fn OnIISRequestInfo(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: &::windows_core::PCWSTR, pszserverip: &::windows_core::PCWSTR, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComIdentityEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComIdentityEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnIISRequestInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: ::windows_core::PCWSTR, pszserverip: ::windows_core::PCWSTR, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnIISRequestInfo(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute(&pszclientip), ::core::mem::transmute(&pszserverip), ::core::mem::transmute(&pszurl)).into())
        }
        IComIdentityEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnIISRequestInfo: OnIISRequestInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComInstance2Events_Impl: ::windows_core::BaseImpl {
    fn OnObjectCreate2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnObjectDestroy2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComInstance2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComInstance2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjectCreate2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectCreate2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidpartition)).into())
        }
        unsafe extern "system" fn OnObjectDestroy2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectDestroy2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into())
        }
        IComInstance2Events_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjectCreate2: OnObjectCreate2::<Identity, Impl, OFFSET>,
            OnObjectDestroy2: OnObjectDestroy2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComInstanceEvents_Impl: ::windows_core::BaseImpl {
    fn OnObjectCreate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()>;
    fn OnObjectDestroy(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComInstanceEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComInstanceEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjectCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectCreate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into())
        }
        unsafe extern "system" fn OnObjectDestroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectDestroy(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into())
        }
        IComInstanceEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjectCreate: OnObjectCreate::<Identity, Impl, OFFSET>,
            OnObjectDestroy: OnObjectDestroy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComLTxEvents_Impl: ::windows_core::BaseImpl {
    fn OnLtxTransactionStart(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID, tsid: &::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::Result<()>;
    fn OnLtxTransactionPrepare(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnLtxTransactionAbort(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLtxTransactionCommit(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLtxTransactionPromote(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID, txnid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComLTxEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComLTxEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLtxTransactionStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, tsid: ::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLtxTransactionStart(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into())
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLtxTransactionPrepare(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute_copy(&fvote)).into())
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLtxTransactionAbort(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx)).into())
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLtxTransactionCommit(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx)).into())
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, txnid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLtxTransactionPromote(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute(&txnid)).into())
        }
        IComLTxEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLtxTransactionStart: OnLtxTransactionStart::<Identity, Impl, OFFSET>,
            OnLtxTransactionPrepare: OnLtxTransactionPrepare::<Identity, Impl, OFFSET>,
            OnLtxTransactionAbort: OnLtxTransactionAbort::<Identity, Impl, OFFSET>,
            OnLtxTransactionCommit: OnLtxTransactionCommit::<Identity, Impl, OFFSET>,
            OnLtxTransactionPromote: OnLtxTransactionPromote::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComMethod2Events_Impl: ::windows_core::BaseImpl {
    fn OnMethodCall2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::Result<()>;
    fn OnMethodReturn2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnMethodException2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComMethod2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComMethod2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnMethodCall2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMethodCall2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into())
        }
        unsafe extern "system" fn OnMethodReturn2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMethodReturn2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into())
        }
        unsafe extern "system" fn OnMethodException2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMethodException2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into())
        }
        IComMethod2Events_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnMethodCall2: OnMethodCall2::<Identity, Impl, OFFSET>,
            OnMethodReturn2: OnMethodReturn2::<Identity, Impl, OFFSET>,
            OnMethodException2: OnMethodException2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComMethodEvents_Impl: ::windows_core::BaseImpl {
    fn OnMethodCall(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::Result<()>;
    fn OnMethodReturn(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnMethodException(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComMethodEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComMethodEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnMethodCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMethodCall(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into())
        }
        unsafe extern "system" fn OnMethodReturn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMethodReturn(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into())
        }
        unsafe extern "system" fn OnMethodException<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMethodException(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into())
        }
        IComMethodEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnMethodCall: OnMethodCall::<Identity, Impl, OFFSET>,
            OnMethodReturn: OnMethodReturn::<Identity, Impl, OFFSET>,
            OnMethodException: OnMethodException::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComMtaThreadPoolKnobs_Impl: ::windows_core::BaseImpl {
    fn MTASetMaxThreadCount(this: &Self::This, dwmaxthreads: u32) -> ::windows_core::Result<()>;
    fn MTAGetMaxThreadCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MTASetThrottleValue(this: &Self::This, dwthrottle: u32) -> ::windows_core::Result<()>;
    fn MTAGetThrottleValue(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IComMtaThreadPoolKnobs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComMtaThreadPoolKnobs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MTASetMaxThreadCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MTASetMaxThreadCount(this, ::core::mem::transmute_copy(&dwmaxthreads)).into())
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MTAGetMaxThreadCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MTASetThrottleValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MTASetThrottleValue(this, ::core::mem::transmute_copy(&dwthrottle)).into())
        }
        unsafe extern "system" fn MTAGetThrottleValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MTAGetThrottleValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthrottle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IComMtaThreadPoolKnobs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MTASetMaxThreadCount: MTASetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTAGetMaxThreadCount: MTAGetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTASetThrottleValue: MTASetThrottleValue::<Identity, Impl, OFFSET>,
            MTAGetThrottleValue: MTAGetThrottleValue::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComObjectConstruction2Events_Impl: ::windows_core::BaseImpl {
    fn OnObjectConstruct2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: &::windows_core::PCWSTR, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComObjectConstruction2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComObjectConstruction2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjectConstruct2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: ::windows_core::PCWSTR, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectConstruct2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute(&sconstructstring), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into())
        }
        IComObjectConstruction2Events_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjectConstruct2: OnObjectConstruct2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComObjectConstructionEvents_Impl: ::windows_core::BaseImpl {
    fn OnObjectConstruct(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: &::windows_core::PCWSTR, oid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComObjectConstructionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComObjectConstructionEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjectConstruct<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: ::windows_core::PCWSTR, oid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectConstruct(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute(&sconstructstring), ::core::mem::transmute_copy(&oid)).into())
        }
        IComObjectConstructionEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjectConstruct: OnObjectConstruct::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComObjectEvents_Impl: ::windows_core::BaseImpl {
    fn OnObjectActivate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()>;
    fn OnObjectDeactivate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()>;
    fn OnDisableCommit(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
    fn OnEnableCommit(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
    fn OnSetComplete(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
    fn OnSetAbort(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComObjectEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComObjectEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjectActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectActivate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into())
        }
        unsafe extern "system" fn OnObjectDeactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjectDeactivate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into())
        }
        unsafe extern "system" fn OnDisableCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDisableCommit(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into())
        }
        unsafe extern "system" fn OnEnableCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEnableCommit(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into())
        }
        unsafe extern "system" fn OnSetComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetComplete(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into())
        }
        unsafe extern "system" fn OnSetAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetAbort(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into())
        }
        IComObjectEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjectActivate: OnObjectActivate::<Identity, Impl, OFFSET>,
            OnObjectDeactivate: OnObjectDeactivate::<Identity, Impl, OFFSET>,
            OnDisableCommit: OnDisableCommit::<Identity, Impl, OFFSET>,
            OnEnableCommit: OnEnableCommit::<Identity, Impl, OFFSET>,
            OnSetComplete: OnSetComplete::<Identity, Impl, OFFSET>,
            OnSetAbort: OnSetAbort::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComObjectPool2Events_Impl: ::windows_core::BaseImpl {
    fn OnObjPoolPutObject2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetObject2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnObjPoolRecycleToTx2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetFromTx2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComObjectPool2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComObjectPool2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjPoolPutObject2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolPutObject2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into())
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolGetObject2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into())
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolRecycleToTx2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into())
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolGetFromTx2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidpartition)).into())
        }
        IComObjectPool2Events_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjPoolPutObject2: OnObjPoolPutObject2::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject2: OnObjPoolGetObject2::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx2: OnObjPoolRecycleToTx2::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx2: OnObjPoolGetFromTx2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComObjectPoolEvents_Impl: ::windows_core::BaseImpl {
    fn OnObjPoolPutObject(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetObject(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolRecycleToTx(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetFromTx(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComObjectPoolEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComObjectPoolEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjPoolPutObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolPutObject(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into())
        }
        unsafe extern "system" fn OnObjPoolGetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolGetObject(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into())
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolRecycleToTx(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into())
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolGetFromTx(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into())
        }
        IComObjectPoolEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjPoolPutObject: OnObjPoolPutObject::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject: OnObjPoolGetObject::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx: OnObjPoolRecycleToTx::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx: OnObjPoolGetFromTx::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComObjectPoolEvents2_Impl: ::windows_core::BaseImpl {
    fn OnObjPoolCreateObject(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolDestroyObject(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolCreateDecision(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows_core::Result<()>;
    fn OnObjPoolTimeout(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, guidactivity: *const ::windows_core::GUID, dwtimeout: u32) -> ::windows_core::Result<()>;
    fn OnObjPoolCreatePool(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComObjectPoolEvents2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComObjectPoolEvents2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnObjPoolCreateObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolCreateObject(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into())
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolDestroyObject(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into())
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolCreateDecision(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&dwthreadswaiting), ::core::mem::transmute_copy(&dwavail), ::core::mem::transmute_copy(&dwcreated), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax)).into())
        }
        unsafe extern "system" fn OnObjPoolTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, guidactivity: *const ::windows_core::GUID, dwtimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolTimeout(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&dwtimeout)).into())
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnObjPoolCreatePool(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax), ::core::mem::transmute_copy(&dwtimeout)).into())
        }
        IComObjectPoolEvents2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnObjPoolCreateObject: OnObjPoolCreateObject::<Identity, Impl, OFFSET>,
            OnObjPoolDestroyObject: OnObjPoolDestroyObject::<Identity, Impl, OFFSET>,
            OnObjPoolCreateDecision: OnObjPoolCreateDecision::<Identity, Impl, OFFSET>,
            OnObjPoolTimeout: OnObjPoolTimeout::<Identity, Impl, OFFSET>,
            OnObjPoolCreatePool: OnObjPoolCreatePool::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComQCEvents_Impl: ::windows_core::BaseImpl {
    fn OnQCRecord(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &::windows_core::PCWSTR, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, msmqhr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCQueueOpen(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, szqueue: &::windows_core::PCWSTR, queueid: u64, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCReceive(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCReceiveFail(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCMoveToReTryQueue(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, retryindex: u32) -> ::windows_core::Result<()>;
    fn OnQCMoveToDeadQueue(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnQCPlayback(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComQCEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComQCEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnQCRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: ::windows_core::PCWSTR, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, msmqhr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQCRecord(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute(&szqueue), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&msmqhr)).into())
        }
        unsafe extern "system" fn OnQCQueueOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: ::windows_core::PCWSTR, queueid: u64, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQCQueueOpen(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&szqueue), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn OnQCReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQCReceive(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn OnQCReceiveFail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQCReceiveFail(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&msmqhr)).into())
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, retryindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQCMoveToReTryQueue(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&retryindex)).into())
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQCMoveToDeadQueue(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid)).into())
        }
        unsafe extern "system" fn OnQCPlayback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQCPlayback(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into())
        }
        IComQCEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnQCRecord: OnQCRecord::<Identity, Impl, OFFSET>,
            OnQCQueueOpen: OnQCQueueOpen::<Identity, Impl, OFFSET>,
            OnQCReceive: OnQCReceive::<Identity, Impl, OFFSET>,
            OnQCReceiveFail: OnQCReceiveFail::<Identity, Impl, OFFSET>,
            OnQCMoveToReTryQueue: OnQCMoveToReTryQueue::<Identity, Impl, OFFSET>,
            OnQCMoveToDeadQueue: OnQCMoveToDeadQueue::<Identity, Impl, OFFSET>,
            OnQCPlayback: OnQCPlayback::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComResourceEvents_Impl: ::windows_core::BaseImpl {
    fn OnResourceCreate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnResourceAllocate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows_core::Result<()>;
    fn OnResourceRecycle(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64) -> ::windows_core::Result<()>;
    fn OnResourceDestroy(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows_core::HRESULT, psztype: &::windows_core::PCWSTR, resid: u64) -> ::windows_core::Result<()>;
    fn OnResourceTrack(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComResourceEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComResourceEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnResourceCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResourceCreate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into())
        }
        unsafe extern "system" fn OnResourceAllocate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResourceAllocate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted), ::core::mem::transmute_copy(&numrated), ::core::mem::transmute_copy(&rating)).into())
        }
        unsafe extern "system" fn OnResourceRecycle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResourceRecycle(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid)).into())
        }
        unsafe extern "system" fn OnResourceDestroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows_core::HRESULT, psztype: ::windows_core::PCWSTR, resid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResourceDestroy(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&hr), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid)).into())
        }
        unsafe extern "system" fn OnResourceTrack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResourceTrack(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into())
        }
        IComResourceEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnResourceCreate: OnResourceCreate::<Identity, Impl, OFFSET>,
            OnResourceAllocate: OnResourceAllocate::<Identity, Impl, OFFSET>,
            OnResourceRecycle: OnResourceRecycle::<Identity, Impl, OFFSET>,
            OnResourceDestroy: OnResourceDestroy::<Identity, Impl, OFFSET>,
            OnResourceTrack: OnResourceTrack::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComSecurityEvents_Impl: ::windows_core::BaseImpl {
    fn OnAuthenticate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnAuthenticateFail(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComSecurityEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComSecurityEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnAuthenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::OnAuthenticate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
            })
        }
        unsafe extern "system" fn OnAuthenticateFail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::OnAuthenticateFail(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
            })
        }
        IComSecurityEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnAuthenticate: OnAuthenticate::<Identity, Impl, OFFSET>,
            OnAuthenticateFail: OnAuthenticateFail::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComStaThreadPoolKnobs_Impl: ::windows_core::BaseImpl {
    fn SetMinThreadCount(this: &Self::This, minthreads: u32) -> ::windows_core::Result<()>;
    fn GetMinThreadCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaxThreadCount(this: &Self::This, maxthreads: u32) -> ::windows_core::Result<()>;
    fn GetMaxThreadCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetActivityPerThread(this: &Self::This, activitiesperthread: u32) -> ::windows_core::Result<()>;
    fn GetActivityPerThread(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetActivityRatio(this: &Self::This, activityratio: f64) -> ::windows_core::Result<()>;
    fn GetActivityRatio(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetThreadCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetQueueDepth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetQueueDepth(this: &Self::This, dwqdepth: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComStaThreadPoolKnobs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComStaThreadPoolKnobs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMinThreadCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinThreadCount(this, ::core::mem::transmute_copy(&minthreads)).into())
        }
        unsafe extern "system" fn GetMinThreadCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinThreadCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxThreadCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxThreadCount(this, ::core::mem::transmute_copy(&maxthreads)).into())
        }
        unsafe extern "system" fn GetMaxThreadCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxThreadCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetActivityPerThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActivityPerThread(this, ::core::mem::transmute_copy(&activitiesperthread)).into())
        }
        unsafe extern "system" fn GetActivityPerThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivityPerThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activitiesperthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetActivityRatio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActivityRatio(this, ::core::mem::transmute_copy(&activityratio)).into())
        }
        unsafe extern "system" fn GetActivityRatio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivityRatio(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activityratio, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThreadCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThreadCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetQueueDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQueueDepth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwqdepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQueueDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueueDepth(this, ::core::mem::transmute_copy(&dwqdepth)).into())
        }
        IComStaThreadPoolKnobs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMinThreadCount: SetMinThreadCount::<Identity, Impl, OFFSET>,
            GetMinThreadCount: GetMinThreadCount::<Identity, Impl, OFFSET>,
            SetMaxThreadCount: SetMaxThreadCount::<Identity, Impl, OFFSET>,
            GetMaxThreadCount: GetMaxThreadCount::<Identity, Impl, OFFSET>,
            SetActivityPerThread: SetActivityPerThread::<Identity, Impl, OFFSET>,
            GetActivityPerThread: GetActivityPerThread::<Identity, Impl, OFFSET>,
            SetActivityRatio: SetActivityRatio::<Identity, Impl, OFFSET>,
            GetActivityRatio: GetActivityRatio::<Identity, Impl, OFFSET>,
            GetThreadCount: GetThreadCount::<Identity, Impl, OFFSET>,
            GetQueueDepth: GetQueueDepth::<Identity, Impl, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComStaThreadPoolKnobs2_Impl: ::windows_core::BaseImpl + IComStaThreadPoolKnobs_Impl {
    fn GetMaxCPULoad(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaxCPULoad(this: &Self::This, pdwload: i32) -> ::windows_core::Result<()>;
    fn GetCPUMetricEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCPUMetricEnabled(this: &Self::This, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetCreateThreadsAggressively(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCreateThreadsAggressively(this: &Self::This, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMaxCSR(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaxCSR(this: &Self::This, dwcsr: i32) -> ::windows_core::Result<()>;
    fn GetWaitTimeForThreadCleanup(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetWaitTimeForThreadCleanup(this: &Self::This, dwthreadcleanupwaittime: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComStaThreadPoolKnobs2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IComStaThreadPoolKnobs);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComStaThreadPoolKnobs2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaxCPULoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxCPULoad(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxCPULoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxCPULoad(this, ::core::mem::transmute_copy(&pdwload)).into())
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCPUMetricEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmetricenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCPUMetricEnabled(this, ::core::mem::transmute_copy(&bmetricenabled)).into())
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCreateThreadsAggressively(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmetricenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreateThreadsAggressively(this, ::core::mem::transmute_copy(&bmetricenabled)).into())
        }
        unsafe extern "system" fn GetMaxCSR<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxCSR(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcsr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxCSR<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxCSR(this, ::core::mem::transmute_copy(&dwcsr)).into())
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWaitTimeForThreadCleanup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthreadcleanupwaittime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWaitTimeForThreadCleanup(this, ::core::mem::transmute_copy(&dwthreadcleanupwaittime)).into())
        }
        IComStaThreadPoolKnobs2_Vtbl {
            base__: <IComStaThreadPoolKnobs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaxCPULoad: GetMaxCPULoad::<Identity, Impl, OFFSET>,
            SetMaxCPULoad: SetMaxCPULoad::<Identity, Impl, OFFSET>,
            GetCPUMetricEnabled: GetCPUMetricEnabled::<Identity, Impl, OFFSET>,
            SetCPUMetricEnabled: SetCPUMetricEnabled::<Identity, Impl, OFFSET>,
            GetCreateThreadsAggressively: GetCreateThreadsAggressively::<Identity, Impl, OFFSET>,
            SetCreateThreadsAggressively: SetCreateThreadsAggressively::<Identity, Impl, OFFSET>,
            GetMaxCSR: GetMaxCSR::<Identity, Impl, OFFSET>,
            SetMaxCSR: SetMaxCSR::<Identity, Impl, OFFSET>,
            GetWaitTimeForThreadCleanup: GetWaitTimeForThreadCleanup::<Identity, Impl, OFFSET>,
            SetWaitTimeForThreadCleanup: SetWaitTimeForThreadCleanup::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComThreadEvents_Impl: ::windows_core::BaseImpl {
    fn OnThreadStart(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadTerminate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadBindToApartment(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadUnBind(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadWorkEnque(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()>;
    fn OnThreadWorkPrivate(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows_core::Result<()>;
    fn OnThreadWorkPublic(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()>;
    fn OnThreadWorkRedirect(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows_core::Result<()>;
    fn OnThreadWorkReject(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()>;
    fn OnThreadAssignApartment(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, aptid: u64) -> ::windows_core::Result<()>;
    fn OnThreadUnassignApartment(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComThreadEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComThreadEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnThreadStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadStart(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into())
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadTerminate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into())
        }
        unsafe extern "system" fn OnThreadBindToApartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadBindToApartment(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt), ::core::mem::transmute_copy(&dwlowcnt)).into())
        }
        unsafe extern "system" fn OnThreadUnBind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadUnBind(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt)).into())
        }
        unsafe extern "system" fn OnThreadWorkEnque<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadWorkEnque(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into())
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadWorkPrivate(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid)).into())
        }
        unsafe extern "system" fn OnThreadWorkPublic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadWorkPublic(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into())
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadWorkRedirect(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen), ::core::mem::transmute_copy(&threadnum)).into())
        }
        unsafe extern "system" fn OnThreadWorkReject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadWorkReject(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into())
        }
        unsafe extern "system" fn OnThreadAssignApartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, aptid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadAssignApartment(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&aptid)).into())
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadUnassignApartment(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&aptid)).into())
        }
        IComThreadEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnThreadStart: OnThreadStart::<Identity, Impl, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, Impl, OFFSET>,
            OnThreadBindToApartment: OnThreadBindToApartment::<Identity, Impl, OFFSET>,
            OnThreadUnBind: OnThreadUnBind::<Identity, Impl, OFFSET>,
            OnThreadWorkEnque: OnThreadWorkEnque::<Identity, Impl, OFFSET>,
            OnThreadWorkPrivate: OnThreadWorkPrivate::<Identity, Impl, OFFSET>,
            OnThreadWorkPublic: OnThreadWorkPublic::<Identity, Impl, OFFSET>,
            OnThreadWorkRedirect: OnThreadWorkRedirect::<Identity, Impl, OFFSET>,
            OnThreadWorkReject: OnThreadWorkReject::<Identity, Impl, OFFSET>,
            OnThreadAssignApartment: OnThreadAssignApartment::<Identity, Impl, OFFSET>,
            OnThreadUnassignApartment: OnThreadUnassignApartment::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComTrackingInfoCollection_Impl: ::windows_core::BaseImpl {
    fn Type(this: &Self::This) -> ::windows_core::Result<TRACKING_COLL_TYPE>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Item(this: &Self::This, ulindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComTrackingInfoCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComTrackingInfoCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Item(this, ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IComTrackingInfoCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IComTrackingInfoEvents_Impl: ::windows_core::BaseImpl {
    fn OnNewTrackingInfo(this: &Self::This, ptoplevelcollection: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IComTrackingInfoEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComTrackingInfoEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnNewTrackingInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNewTrackingInfo(this, ::windows_core::from_raw_borrowed(&ptoplevelcollection)).into())
        }
        IComTrackingInfoEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnNewTrackingInfo: OnNewTrackingInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComTrackingInfoObject_Impl: ::windows_core::BaseImpl {
    fn GetValue(this: &Self::This, szpropertyname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IComTrackingInfoObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComTrackingInfoObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szpropertyname: ::windows_core::PCWSTR, pvarout: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::core::mem::transmute(&szpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IComTrackingInfoObject_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetValue: GetValue::<Identity, Impl, OFFSET> }
    };
}
pub trait IComTrackingInfoProperties_Impl: ::windows_core::BaseImpl {
    fn PropCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPropName(this: &Self::This, ulindex: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IComTrackingInfoProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComTrackingInfoProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropName(this, ::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpropname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IComTrackingInfoProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropCount: PropCount::<Identity, Impl, OFFSET>,
            GetPropName: GetPropName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransaction2Events_Impl: ::windows_core::BaseImpl {
    fn OnTransactionStart2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::Result<()>;
    fn OnTransactionPrepare2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTransactionAbort2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnTransactionCommit2(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComTransaction2Events {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComTransaction2Events {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTransactionStart2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionStart2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into())
        }
        unsafe extern "system" fn OnTransactionPrepare2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionPrepare2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into())
        }
        unsafe extern "system" fn OnTransactionAbort2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionAbort2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into())
        }
        unsafe extern "system" fn OnTransactionCommit2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionCommit2(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into())
        }
        IComTransaction2Events_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTransactionStart2: OnTransactionStart2::<Identity, Impl, OFFSET>,
            OnTransactionPrepare2: OnTransactionPrepare2::<Identity, Impl, OFFSET>,
            OnTransactionAbort2: OnTransactionAbort2::<Identity, Impl, OFFSET>,
            OnTransactionCommit2: OnTransactionCommit2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransactionEvents_Impl: ::windows_core::BaseImpl {
    fn OnTransactionStart(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTransactionPrepare(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTransactionAbort(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnTransactionCommit(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IComTransactionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComTransactionEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTransactionStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionStart(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot)).into())
        }
        unsafe extern "system" fn OnTransactionPrepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionPrepare(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into())
        }
        unsafe extern "system" fn OnTransactionAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionAbort(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into())
        }
        unsafe extern "system" fn OnTransactionCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTransactionCommit(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into())
        }
        IComTransactionEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTransactionStart: OnTransactionStart::<Identity, Impl, OFFSET>,
            OnTransactionPrepare: OnTransactionPrepare::<Identity, Impl, OFFSET>,
            OnTransactionAbort: OnTransactionAbort::<Identity, Impl, OFFSET>,
            OnTransactionCommit: OnTransactionCommit::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComUserEvent_Impl: ::windows_core::BaseImpl {
    fn OnUserEvent(this: &Self::This, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IComUserEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComUserEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUserEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUserEvent(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pvarevent)).into())
        }
        IComUserEvent_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnUserEvent: OnUserEvent::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IContextProperties_Impl: ::windows_core::BaseImpl {
    fn Count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, name: &::windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EnumNames(this: &Self::This) -> ::windows_core::Result<IEnumNames>;
    fn SetProperty(this: &Self::This, name: &::windows_core::BSTR, property: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RemoveProperty(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IContextProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContextProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&pproperty)).into())
        }
        unsafe extern "system" fn EnumNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, property: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute(&name), ::core::mem::transmute(&property)).into())
        }
        unsafe extern "system" fn RemoveProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveProperty(this, ::core::mem::transmute(&name)).into())
        }
        IContextProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContextSecurityPerimeter_Impl: ::windows_core::BaseImpl {
    fn GetPerimeterFlag(this: &Self::This, pflag: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetPerimeterFlag(this: &Self::This, fflag: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContextSecurityPerimeter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContextSecurityPerimeter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPerimeterFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPerimeterFlag(this, ::core::mem::transmute_copy(&pflag)).into())
        }
        unsafe extern "system" fn SetPerimeterFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPerimeterFlag(this, ::core::mem::transmute_copy(&fflag)).into())
        }
        IContextSecurityPerimeter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPerimeterFlag: GetPerimeterFlag::<Identity, Impl, OFFSET>,
            SetPerimeterFlag: SetPerimeterFlag::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContextState_Impl: ::windows_core::BaseImpl {
    fn SetDeactivateOnReturn(this: &Self::This, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetDeactivateOnReturn(this: &Self::This, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetMyTransactionVote(this: &Self::This, txvote: TransactionVote) -> ::windows_core::Result<()>;
    fn GetMyTransactionVote(this: &Self::This, ptxvote: *mut TransactionVote) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContextState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContextState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDeactivateOnReturn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeactivateOnReturn(this, ::core::mem::transmute_copy(&bdeactivate)).into())
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeactivateOnReturn(this, ::core::mem::transmute_copy(&pbdeactivate)).into())
        }
        unsafe extern "system" fn SetMyTransactionVote<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMyTransactionVote(this, ::core::mem::transmute_copy(&txvote)).into())
        }
        unsafe extern "system" fn GetMyTransactionVote<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMyTransactionVote(this, ::core::mem::transmute_copy(&ptxvote)).into())
        }
        IContextState_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDeactivateOnReturn: SetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            GetDeactivateOnReturn: GetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            SetMyTransactionVote: SetMyTransactionVote::<Identity, Impl, OFFSET>,
            GetMyTransactionVote: GetMyTransactionVote::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICreateWithLocalTransaction_Impl: ::windows_core::BaseImpl {
    fn CreateInstanceWithSysTx(this: &Self::This, ptransaction: ::core::option::Option<&::windows_core::IUnknown>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICreateWithLocalTransaction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateWithLocalTransaction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstanceWithSysTx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstanceWithSysTx(this, ::windows_core::from_raw_borrowed(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into())
        }
        ICreateWithLocalTransaction_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstanceWithSysTx: CreateInstanceWithSysTx::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICreateWithTipTransactionEx_Impl: ::windows_core::BaseImpl {
    fn CreateInstance(this: &Self::This, bstrtipurl: &::windows_core::BSTR, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICreateWithTipTransactionEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateWithTipTransactionEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtipurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstance(this, ::core::mem::transmute(&bstrtipurl), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into())
        }
        ICreateWithTipTransactionEx_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_DistributedTransactionCoordinator\"`"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ICreateWithTransactionEx_Impl: ::windows_core::BaseImpl {
    fn CreateInstance(this: &Self::This, ptransaction: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransaction>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::Iids for ICreateWithTransactionEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateWithTransactionEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstance(this, ::windows_core::from_raw_borrowed(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into())
        }
        ICreateWithTransactionEx_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICrmCompensator_Impl: ::windows_core::BaseImpl {
    fn SetLogControl(this: &Self::This, plogcontrol: ::core::option::Option<&ICrmLogControl>) -> ::windows_core::Result<()>;
    fn BeginPrepare(this: &Self::This) -> ::windows_core::Result<()>;
    fn PrepareRecord(this: &Self::This, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EndPrepare(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn BeginCommit(this: &Self::This, frecovery: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CommitRecord(this: &Self::This, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EndCommit(this: &Self::This) -> ::windows_core::Result<()>;
    fn BeginAbort(this: &Self::This, frecovery: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AbortRecord(this: &Self::This, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EndAbort(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ICrmCompensator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICrmCompensator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLogControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogControl(this, ::windows_core::from_raw_borrowed(&plogcontrol)).into())
        }
        unsafe extern "system" fn BeginPrepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginPrepare(this).into())
        }
        unsafe extern "system" fn PrepareRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrepareRecord(this, ::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndPrepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndPrepare(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfoktoprepare, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginCommit(this, ::core::mem::transmute_copy(&frecovery)).into())
        }
        unsafe extern "system" fn CommitRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitRecord(this, ::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndCommit(this).into())
        }
        unsafe extern "system" fn BeginAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginAbort(this, ::core::mem::transmute_copy(&frecovery)).into())
        }
        unsafe extern "system" fn AbortRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AbortRecord(this, ::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndAbort(this).into())
        }
        ICrmCompensator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLogControl: SetLogControl::<Identity, Impl, OFFSET>,
            BeginPrepare: BeginPrepare::<Identity, Impl, OFFSET>,
            PrepareRecord: PrepareRecord::<Identity, Impl, OFFSET>,
            EndPrepare: EndPrepare::<Identity, Impl, OFFSET>,
            BeginCommit: BeginCommit::<Identity, Impl, OFFSET>,
            CommitRecord: CommitRecord::<Identity, Impl, OFFSET>,
            EndCommit: EndCommit::<Identity, Impl, OFFSET>,
            BeginAbort: BeginAbort::<Identity, Impl, OFFSET>,
            AbortRecord: AbortRecord::<Identity, Impl, OFFSET>,
            EndAbort: EndAbort::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmCompensatorVariants_Impl: ::windows_core::BaseImpl {
    fn SetLogControlVariants(this: &Self::This, plogcontrol: ::core::option::Option<&ICrmLogControl>) -> ::windows_core::Result<()>;
    fn BeginPrepareVariants(this: &Self::This) -> ::windows_core::Result<()>;
    fn PrepareRecordVariants(this: &Self::This, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndPrepareVariants(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BeginCommitVariants(this: &Self::This, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn CommitRecordVariants(this: &Self::This, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndCommitVariants(this: &Self::This) -> ::windows_core::Result<()>;
    fn BeginAbortVariants(this: &Self::This, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AbortRecordVariants(this: &Self::This, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndAbortVariants(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICrmCompensatorVariants {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICrmCompensatorVariants {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLogControlVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLogControlVariants(this, ::windows_core::from_raw_borrowed(&plogcontrol)).into())
        }
        unsafe extern "system" fn BeginPrepareVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginPrepareVariants(this).into())
        }
        unsafe extern "system" fn PrepareRecordVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrepareRecordVariants(this, ::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndPrepareVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pboktoprepare: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndPrepareVariants(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboktoprepare, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginCommitVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginCommitVariants(this, ::core::mem::transmute_copy(&brecovery)).into())
        }
        unsafe extern "system" fn CommitRecordVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitRecordVariants(this, ::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndCommitVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndCommitVariants(this).into())
        }
        unsafe extern "system" fn BeginAbortVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginAbortVariants(this, ::core::mem::transmute_copy(&brecovery)).into())
        }
        unsafe extern "system" fn AbortRecordVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AbortRecordVariants(this, ::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndAbortVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndAbortVariants(this).into())
        }
        ICrmCompensatorVariants_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLogControlVariants: SetLogControlVariants::<Identity, Impl, OFFSET>,
            BeginPrepareVariants: BeginPrepareVariants::<Identity, Impl, OFFSET>,
            PrepareRecordVariants: PrepareRecordVariants::<Identity, Impl, OFFSET>,
            EndPrepareVariants: EndPrepareVariants::<Identity, Impl, OFFSET>,
            BeginCommitVariants: BeginCommitVariants::<Identity, Impl, OFFSET>,
            CommitRecordVariants: CommitRecordVariants::<Identity, Impl, OFFSET>,
            EndCommitVariants: EndCommitVariants::<Identity, Impl, OFFSET>,
            BeginAbortVariants: BeginAbortVariants::<Identity, Impl, OFFSET>,
            AbortRecordVariants: AbortRecordVariants::<Identity, Impl, OFFSET>,
            EndAbortVariants: EndAbortVariants::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmFormatLogRecords_Impl: ::windows_core::BaseImpl {
    fn GetColumnCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetColumnHeaders(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetColumn(this: &Self::This, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetColumnVariants(this: &Self::This, logrecord: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICrmFormatLogRecords {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICrmFormatLogRecords {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColumnCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcolumncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheaders: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumn(this, ::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformattedlogrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumnVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logrecord: super::Variant::VARIANT, pformattedlogrecord: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnVariants(this, ::core::mem::transmute(&logrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformattedlogrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICrmFormatLogRecords_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetColumnVariants: GetColumnVariants::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmLogControl_Impl: ::windows_core::BaseImpl {
    fn TransactionUOW(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterCompensator(this: &Self::This, lpcwstrprogidcompensator: &::windows_core::PCWSTR, lpcwstrdescription: &::windows_core::PCWSTR, lcrmregflags: i32) -> ::windows_core::Result<()>;
    fn WriteLogRecordVariants(this: &Self::This, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ForceLog(this: &Self::This) -> ::windows_core::Result<()>;
    fn ForgetLogRecord(this: &Self::This) -> ::windows_core::Result<()>;
    fn ForceTransactionToAbort(this: &Self::This) -> ::windows_core::Result<()>;
    fn WriteLogRecord(this: &Self::This, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICrmLogControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICrmLogControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransactionUOW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionUOW(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterCompensator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: ::windows_core::PCWSTR, lpcwstrdescription: ::windows_core::PCWSTR, lcrmregflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterCompensator(this, ::core::mem::transmute(&lpcwstrprogidcompensator), ::core::mem::transmute(&lpcwstrdescription), ::core::mem::transmute_copy(&lcrmregflags)).into())
        }
        unsafe extern "system" fn WriteLogRecordVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteLogRecordVariants(this, ::core::mem::transmute_copy(&plogrecord)).into())
        }
        unsafe extern "system" fn ForceLog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForceLog(this).into())
        }
        unsafe extern "system" fn ForgetLogRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForgetLogRecord(this).into())
        }
        unsafe extern "system" fn ForceTransactionToAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForceTransactionToAbort(this).into())
        }
        unsafe extern "system" fn WriteLogRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteLogRecord(this, ::core::mem::transmute_copy(&rgblob), ::core::mem::transmute_copy(&cblob)).into())
        }
        ICrmLogControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransactionUOW: TransactionUOW::<Identity, Impl, OFFSET>,
            RegisterCompensator: RegisterCompensator::<Identity, Impl, OFFSET>,
            WriteLogRecordVariants: WriteLogRecordVariants::<Identity, Impl, OFFSET>,
            ForceLog: ForceLog::<Identity, Impl, OFFSET>,
            ForgetLogRecord: ForgetLogRecord::<Identity, Impl, OFFSET>,
            ForceTransactionToAbort: ForceTransactionToAbort::<Identity, Impl, OFFSET>,
            WriteLogRecord: WriteLogRecord::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitor_Impl: ::windows_core::BaseImpl {
    fn GetClerks(this: &Self::This) -> ::windows_core::Result<ICrmMonitorClerks>;
    fn HoldClerk(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICrmMonitor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICrmMonitor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClerks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclerks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClerks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclerks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HoldClerk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HoldClerk(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICrmMonitor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClerks: GetClerks::<Identity, Impl, OFFSET>,
            HoldClerk: HoldClerk::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitorClerks_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Item(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProgIdCompensator(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Description(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn TransactionUOW(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ActivityId(this: &Self::This, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICrmMonitorClerks {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICrmMonitorClerks {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProgIdCompensator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProgIdCompensator(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransactionUOW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionUOW(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivityId(this, ::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICrmMonitorClerks_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ProgIdCompensator: ProgIdCompensator::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            TransactionUOW: TransactionUOW::<Identity, Impl, OFFSET>,
            ActivityId: ActivityId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitorLogRecords_Impl: ::windows_core::BaseImpl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TransactionState(this: &Self::This) -> ::windows_core::Result<CrmTransactionState>;
    fn StructuredRecords(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetLogRecord(this: &Self::This, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows_core::Result<()>;
    fn GetLogRecordVariants(this: &Self::This, indexnumber: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICrmMonitorLogRecords {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICrmMonitorLogRecords {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransactionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StructuredRecords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StructuredRecords(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLogRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLogRecord(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcrmlogrec)).into())
        }
        unsafe extern "system" fn GetLogRecordVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexnumber: super::Variant::VARIANT, plogrecord: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLogRecordVariants(this, ::core::mem::transmute(&indexnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plogrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICrmMonitorLogRecords_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            TransactionState: TransactionState::<Identity, Impl, OFFSET>,
            StructuredRecords: StructuredRecords::<Identity, Impl, OFFSET>,
            GetLogRecord: GetLogRecord::<Identity, Impl, OFFSET>,
            GetLogRecordVariants: GetLogRecordVariants::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDispenserDriver_Impl: ::windows_core::BaseImpl {
    fn CreateResource(this: &Self::This, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows_core::Result<()>;
    fn RateResource(this: &Self::This, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows_core::Result<()>;
    fn EnlistResource(this: &Self::This, resid: usize, transid: usize) -> ::windows_core::Result<()>;
    fn ResetResource(this: &Self::This, resid: usize) -> ::windows_core::Result<()>;
    fn DestroyResource(this: &Self::This, resid: usize) -> ::windows_core::Result<()>;
    fn DestroyResourceS(this: &Self::This, resid: *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDispenserDriver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDispenserDriver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateResource(this, ::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&presid), ::core::mem::transmute_copy(&psecsfreebeforedestroy)).into())
        }
        unsafe extern "system" fn RateResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RateResource(this, ::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&frequirestransactionenlistment), ::core::mem::transmute_copy(&prating)).into())
        }
        unsafe extern "system" fn EnlistResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnlistResource(this, ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&transid)).into())
        }
        unsafe extern "system" fn ResetResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetResource(this, ::core::mem::transmute_copy(&resid)).into())
        }
        unsafe extern "system" fn DestroyResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyResource(this, ::core::mem::transmute_copy(&resid)).into())
        }
        unsafe extern "system" fn DestroyResourceS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyResourceS(this, ::core::mem::transmute_copy(&resid)).into())
        }
        IDispenserDriver_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateResource: CreateResource::<Identity, Impl, OFFSET>,
            RateResource: RateResource::<Identity, Impl, OFFSET>,
            EnlistResource: EnlistResource::<Identity, Impl, OFFSET>,
            ResetResource: ResetResource::<Identity, Impl, OFFSET>,
            DestroyResource: DestroyResource::<Identity, Impl, OFFSET>,
            DestroyResourceS: DestroyResourceS::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDispenserManager_Impl: ::windows_core::BaseImpl {
    fn RegisterDispenser(this: &Self::This, __midl__idispensermanager0000: ::core::option::Option<&IDispenserDriver>, szdispensername: &::windows_core::PCWSTR) -> ::windows_core::Result<IHolder>;
    fn GetContext(this: &Self::This, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDispenserManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDispenserManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterDispenser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: *mut ::core::ffi::c_void, szdispensername: ::windows_core::PCWSTR, __midl__idispensermanager0001: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterDispenser(this, ::windows_core::from_raw_borrowed(&__midl__idispensermanager0000), ::core::mem::transmute(&szdispensername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__idispensermanager0001, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContext(this, ::core::mem::transmute_copy(&__midl__idispensermanager0002), ::core::mem::transmute_copy(&__midl__idispensermanager0003)).into())
        }
        IDispenserManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterDispenser: RegisterDispenser::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumNames_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgname: *mut ::windows_core::BSTR, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNames>;
}
impl ::windows_core::Iids for IEnumNames {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNames {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgname), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNames_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventServerTrace_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn StartTraceGuid(this: &Self::This, bstrguidevent: &::windows_core::BSTR, bstrguidfilter: &::windows_core::BSTR, lpidfilter: i32) -> ::windows_core::Result<()>;
    fn StopTraceGuid(this: &Self::This, bstrguidevent: &::windows_core::BSTR, bstrguidfilter: &::windows_core::BSTR, lpidfilter: i32) -> ::windows_core::Result<()>;
    fn EnumTraceGuid(this: &Self::This, plcntguids: *mut i32, pbstrguidlist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEventServerTrace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEventServerTrace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartTraceGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrguidfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpidfilter: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartTraceGuid(this, ::core::mem::transmute(&bstrguidevent), ::core::mem::transmute(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into())
        }
        unsafe extern "system" fn StopTraceGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrguidfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpidfilter: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopTraceGuid(this, ::core::mem::transmute(&bstrguidevent), ::core::mem::transmute(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into())
        }
        unsafe extern "system" fn EnumTraceGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumTraceGuid(this, ::core::mem::transmute_copy(&plcntguids), ::core::mem::transmute_copy(&pbstrguidlist)).into())
        }
        IEventServerTrace_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartTraceGuid: StartTraceGuid::<Identity, Impl, OFFSET>,
            StopTraceGuid: StopTraceGuid::<Identity, Impl, OFFSET>,
            EnumTraceGuid: EnumTraceGuid::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGetAppTrackerData_Impl: ::windows_core::BaseImpl {
    fn GetApplicationProcesses(this: &Self::This, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows_core::Result<()>;
    fn GetApplicationProcessDetails(this: &Self::This, applicationinstanceid: *const ::windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetApplicationsInProcess(this: &Self::This, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows_core::Result<()>;
    fn GetComponentsInProcess(this: &Self::This, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows_core::Result<()>;
    fn GetComponentDetails(this: &Self::This, applicationinstanceid: *const ::windows_core::GUID, processid: u32, clsid: *const ::windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows_core::Result<()>;
    fn GetTrackerDataAsCollectionObject(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetSuggestedPollingInterval(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IGetAppTrackerData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetAppTrackerData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetApplicationProcesses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApplicationProcesses(this, ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationprocesses), ::core::mem::transmute_copy(&applicationprocesses)).into())
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApplicationProcessDetails(this, ::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&recycleinfo), ::core::mem::transmute_copy(&anycomponentshangmonitored)).into())
        }
        unsafe extern "system" fn GetApplicationsInProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApplicationsInProcess(this, ::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationsinprocess), ::core::mem::transmute_copy(&applications)).into())
        }
        unsafe extern "system" fn GetComponentsInProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetComponentsInProcess(this, ::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numcomponentsinprocess), ::core::mem::transmute_copy(&components)).into())
        }
        unsafe extern "system" fn GetComponentDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, clsid: *const ::windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetComponentDetails(this, ::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&hangmonitorinfo)).into())
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTrackerDataAsCollectionObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(toplevelcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSuggestedPollingInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pollingintervalinseconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetAppTrackerData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetApplicationProcesses: GetApplicationProcesses::<Identity, Impl, OFFSET>,
            GetApplicationProcessDetails: GetApplicationProcessDetails::<Identity, Impl, OFFSET>,
            GetApplicationsInProcess: GetApplicationsInProcess::<Identity, Impl, OFFSET>,
            GetComponentsInProcess: GetComponentsInProcess::<Identity, Impl, OFFSET>,
            GetComponentDetails: GetComponentDetails::<Identity, Impl, OFFSET>,
            GetTrackerDataAsCollectionObject: GetTrackerDataAsCollectionObject::<Identity, Impl, OFFSET>,
            GetSuggestedPollingInterval: GetSuggestedPollingInterval::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGetContextProperties_Impl: ::windows_core::BaseImpl {
    fn Count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, name: &::windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EnumNames(this: &Self::This) -> ::windows_core::Result<IEnumNames>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGetContextProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetContextProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&pproperty)).into())
        }
        unsafe extern "system" fn EnumNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetContextProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGetSecurityCallContext_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetSecurityCallContext(this: &Self::This) -> ::windows_core::Result<ISecurityCallContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGetSecurityCallContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetSecurityCallContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSecurityCallContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityCallContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetSecurityCallContext_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSecurityCallContext: GetSecurityCallContext::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHolder_Impl: ::windows_core::BaseImpl {
    fn AllocResource(this: &Self::This, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows_core::Result<()>;
    fn FreeResource(this: &Self::This, __midl__iholder0002: usize) -> ::windows_core::Result<()>;
    fn TrackResource(this: &Self::This, __midl__iholder0003: usize) -> ::windows_core::Result<()>;
    fn TrackResourceS(this: &Self::This, __midl__iholder0004: *mut u16) -> ::windows_core::Result<()>;
    fn UntrackResource(this: &Self::This, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UntrackResourceS(this: &Self::This, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn RequestDestroyResource(this: &Self::This, __midl__iholder0009: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllocResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocResource(this, ::core::mem::transmute_copy(&__midl__iholder0000), ::core::mem::transmute_copy(&__midl__iholder0001)).into())
        }
        unsafe extern "system" fn FreeResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeResource(this, ::core::mem::transmute_copy(&__midl__iholder0002)).into())
        }
        unsafe extern "system" fn TrackResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TrackResource(this, ::core::mem::transmute_copy(&__midl__iholder0003)).into())
        }
        unsafe extern "system" fn TrackResourceS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TrackResourceS(this, ::core::mem::transmute_copy(&__midl__iholder0004)).into())
        }
        unsafe extern "system" fn UntrackResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UntrackResource(this, ::core::mem::transmute_copy(&__midl__iholder0005), ::core::mem::transmute_copy(&__midl__iholder0006)).into())
        }
        unsafe extern "system" fn UntrackResourceS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UntrackResourceS(this, ::core::mem::transmute_copy(&__midl__iholder0007), ::core::mem::transmute_copy(&__midl__iholder0008)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn RequestDestroyResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestDestroyResource(this, ::core::mem::transmute_copy(&__midl__iholder0009)).into())
        }
        IHolder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllocResource: AllocResource::<Identity, Impl, OFFSET>,
            FreeResource: FreeResource::<Identity, Impl, OFFSET>,
            TrackResource: TrackResource::<Identity, Impl, OFFSET>,
            TrackResourceS: TrackResourceS::<Identity, Impl, OFFSET>,
            UntrackResource: UntrackResource::<Identity, Impl, OFFSET>,
            UntrackResourceS: UntrackResourceS::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            RequestDestroyResource: RequestDestroyResource::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILBEvents_Impl: ::windows_core::BaseImpl {
    fn TargetUp(this: &Self::This, bstrservername: &::windows_core::BSTR, bstrclsideng: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TargetDown(this: &Self::This, bstrservername: &::windows_core::BSTR, bstrclsideng: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EngineDefined(this: &Self::This, bstrpropname: &::windows_core::BSTR, varpropvalue: *const super::Variant::VARIANT, bstrclsideng: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ILBEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILBEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TargetUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsideng: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TargetUp(this, ::core::mem::transmute(&bstrservername), ::core::mem::transmute(&bstrclsideng)).into())
        }
        unsafe extern "system" fn TargetDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsideng: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TargetDown(this, ::core::mem::transmute(&bstrservername), ::core::mem::transmute(&bstrclsideng)).into())
        }
        unsafe extern "system" fn EngineDefined<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varpropvalue: *const super::Variant::VARIANT, bstrclsideng: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EngineDefined(this, ::core::mem::transmute(&bstrpropname), ::core::mem::transmute_copy(&varpropvalue), ::core::mem::transmute(&bstrclsideng)).into())
        }
        ILBEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TargetUp: TargetUp::<Identity, Impl, OFFSET>,
            TargetDown: TargetDown::<Identity, Impl, OFFSET>,
            EngineDefined: EngineDefined::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMTSActivity_Impl: ::windows_core::BaseImpl {
    fn SynchronousCall(this: &Self::This, pcall: ::core::option::Option<&IMTSCall>) -> ::windows_core::Result<()>;
    fn AsyncCall(this: &Self::This, pcall: ::core::option::Option<&IMTSCall>) -> ::windows_core::Result<()>;
    fn Reserved1(this: &Self::This);
    fn BindToCurrentThread(this: &Self::This) -> ::windows_core::Result<()>;
    fn UnbindFromThread(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMTSActivity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMTSActivity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SynchronousCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCall(this, ::windows_core::from_raw_borrowed(&pcall)).into())
        }
        unsafe extern "system" fn AsyncCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncCall(this, ::windows_core::from_raw_borrowed(&pcall)).into())
        }
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved1(this))
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToCurrentThread(this).into())
        }
        unsafe extern "system" fn UnbindFromThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnbindFromThread(this).into())
        }
        IMTSActivity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsyncCall: AsyncCall::<Identity, Impl, OFFSET>,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMTSCall_Impl: ::windows_core::BaseImpl {
    fn OnCall(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMTSCall {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMTSCall {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCall(this).into())
        }
        IMTSCall_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnCall: OnCall::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMTSLocator_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetEventDispatcher(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMTSLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMTSLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEventDispatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventDispatcher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMTSLocator_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEventDispatcher: GetEventDispatcher::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedActivationEvents_Impl: ::windows_core::BaseImpl {
    fn CreateManagedStub(this: &Self::This, pinfo: ::core::option::Option<&IManagedObjectInfo>, fdist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DestroyManagedStub(this: &Self::This, pinfo: ::core::option::Option<&IManagedObjectInfo>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IManagedActivationEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IManagedActivationEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateManagedStub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, fdist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateManagedStub(this, ::windows_core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&fdist)).into())
        }
        unsafe extern "system" fn DestroyManagedStub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyManagedStub(this, ::windows_core::from_raw_borrowed(&pinfo)).into())
        }
        IManagedActivationEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateManagedStub: CreateManagedStub::<Identity, Impl, OFFSET>,
            DestroyManagedStub: DestroyManagedStub::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedObjectInfo_Impl: ::windows_core::BaseImpl {
    fn GetIUnknown(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetIObjectControl(this: &Self::This) -> ::windows_core::Result<IObjectControl>;
    fn SetInPool(this: &Self::This, binpool: super::super::Foundation::BOOL, ppooledobj: ::core::option::Option<&IManagedPooledObj>) -> ::windows_core::Result<()>;
    fn SetWrapperStrength(this: &Self::This, bstrong: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IManagedObjectInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IManagedObjectInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIUnknown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIUnknown(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIObjectControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctrl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIObjectControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctrl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInPool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInPool(this, ::core::mem::transmute_copy(&binpool), ::windows_core::from_raw_borrowed(&ppooledobj)).into())
        }
        unsafe extern "system" fn SetWrapperStrength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWrapperStrength(this, ::core::mem::transmute_copy(&bstrong)).into())
        }
        IManagedObjectInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIUnknown: GetIUnknown::<Identity, Impl, OFFSET>,
            GetIObjectControl: GetIObjectControl::<Identity, Impl, OFFSET>,
            SetInPool: SetInPool::<Identity, Impl, OFFSET>,
            SetWrapperStrength: SetWrapperStrength::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IManagedPoolAction_Impl: ::windows_core::BaseImpl {
    fn LastRelease(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IManagedPoolAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IManagedPoolAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LastRelease<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LastRelease(this).into())
        }
        IManagedPoolAction_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LastRelease: LastRelease::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedPooledObj_Impl: ::windows_core::BaseImpl {
    fn SetHeld(this: &Self::This, m_bheld: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IManagedPooledObj {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IManagedPooledObj {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHeld<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHeld(this, ::core::mem::transmute_copy(&m_bheld)).into())
        }
        IManagedPooledObj_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetHeld: SetHeld::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMessageMover_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn SourcePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSourcePath(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DestPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDestPath(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CommitBatchSize(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetCommitBatchSize(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn MoveMessages(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMessageMover {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMessageMover {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SourcePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourcePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSourcePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSourcePath(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn DestPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDestPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestPath(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn CommitBatchSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitBatchSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCommitBatchSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCommitBatchSize(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn MoveMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveMessages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmessagesmoved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMessageMover_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SourcePath: SourcePath::<Identity, Impl, OFFSET>,
            SetSourcePath: SetSourcePath::<Identity, Impl, OFFSET>,
            DestPath: DestPath::<Identity, Impl, OFFSET>,
            SetDestPath: SetDestPath::<Identity, Impl, OFFSET>,
            CommitBatchSize: CommitBatchSize::<Identity, Impl, OFFSET>,
            SetCommitBatchSize: SetCommitBatchSize::<Identity, Impl, OFFSET>,
            MoveMessages: MoveMessages::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsEventInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Names(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EventID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Value(this: &Self::This, skey: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMtsEventInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMtsEventInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Names<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Names(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sguideventid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sguideventid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, skey: ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Value(this, ::core::mem::transmute(&skey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMtsEventInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Names: Names::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            EventID: EventID::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Value: get_Value::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn PackageName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PackageGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PostEvent(this: &Self::This, vevent: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn FireEvents(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetProcessID(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMtsEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMtsEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PackageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PackageName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PackageGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PackageGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PostEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vevent: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostEvent(this, ::core::mem::transmute_copy(&vevent)).into())
        }
        unsafe extern "system" fn FireEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FireEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProcessID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProcessID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMtsEvents_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PackageName: PackageName::<Identity, Impl, OFFSET>,
            PackageGuid: PackageGuid::<Identity, Impl, OFFSET>,
            PostEvent: PostEvent::<Identity, Impl, OFFSET>,
            FireEvents: FireEvents::<Identity, Impl, OFFSET>,
            GetProcessID: GetProcessID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsGrp_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMtsGrp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMtsGrp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdispatcher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        IMtsGrp_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IObjPool_Impl: ::windows_core::BaseImpl {
    fn Reserved1(this: &Self::This);
    fn Reserved2(this: &Self::This);
    fn Reserved3(this: &Self::This);
    fn Reserved4(this: &Self::This);
    fn PutEndTx(this: &Self::This, pobj: ::core::option::Option<&::windows_core::IUnknown>);
    fn Reserved5(this: &Self::This);
    fn Reserved6(this: &Self::This);
}
impl ::windows_core::Iids for IObjPool {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjPool {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved1(this))
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved2(this))
        }
        unsafe extern "system" fn Reserved3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved3(this))
        }
        unsafe extern "system" fn Reserved4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved4(this))
        }
        unsafe extern "system" fn PutEndTx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutEndTx(this, ::windows_core::from_raw_borrowed(&pobj)))
        }
        unsafe extern "system" fn Reserved5<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved5(this))
        }
        unsafe extern "system" fn Reserved6<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved6(this))
        }
        IObjPool_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            Reserved3: Reserved3::<Identity, Impl, OFFSET>,
            Reserved4: Reserved4::<Identity, Impl, OFFSET>,
            PutEndTx: PutEndTx::<Identity, Impl, OFFSET>,
            Reserved5: Reserved5::<Identity, Impl, OFFSET>,
            Reserved6: Reserved6::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstruct_Impl: ::windows_core::BaseImpl {
    fn Construct(this: &Self::This, pctorobj: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IObjectConstruct {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectConstruct {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Construct<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctorobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Construct(this, ::windows_core::from_raw_borrowed(&pctorobj)).into())
        }
        IObjectConstruct_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Construct: Construct::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectConstructString_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ConstructString(this: &Self::This, pval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IObjectConstructString {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectConstructString {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConstructString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConstructString(this, ::core::mem::transmute_copy(&pval)).into())
        }
        IObjectConstructString_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ConstructString: ConstructString::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContext_Impl: ::windows_core::BaseImpl {
    fn CreateInstance(this: &Self::This, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetAbort(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnableCommit(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableCommit(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsInTransaction(this: &Self::This) -> super::super::Foundation::BOOL;
    fn IsSecurityEnabled(this: &Self::This) -> super::super::Foundation::BOOL;
    fn IsCallerInRole(this: &Self::This, bstrrole: &::windows_core::BSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IObjectContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstance(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn SetComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComplete(this).into())
        }
        unsafe extern "system" fn SetAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAbort(this).into())
        }
        unsafe extern "system" fn EnableCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableCommit(this).into())
        }
        unsafe extern "system" fn DisableCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableCommit(this).into())
        }
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsInTransaction(this))
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSecurityEnabled(this))
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsCallerInRole(this, ::core::mem::transmute(&bstrrole), ::core::mem::transmute_copy(&pfisinrole)).into())
        }
        IObjectContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            SetComplete: SetComplete::<Identity, Impl, OFFSET>,
            SetAbort: SetAbort::<Identity, Impl, OFFSET>,
            EnableCommit: EnableCommit::<Identity, Impl, OFFSET>,
            DisableCommit: DisableCommit::<Identity, Impl, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IObjectContextActivity_Impl: ::windows_core::BaseImpl {
    fn GetActivityId(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectContextActivity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectContextActivity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActivityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActivityId(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        IObjectContextActivity_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetActivityId: GetActivityId::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo_Impl: ::windows_core::BaseImpl {
    fn IsInTransaction(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetTransaction(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetTransactionId(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetActivityId(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetContextId(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IObjectContextInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectContextInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsInTransaction(this))
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrans, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransactionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransactionId(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn GetActivityId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActivityId(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn GetContextId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContextId(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        IObjectContextInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, Impl, OFFSET>,
            GetActivityId: GetActivityId::<Identity, Impl, OFFSET>,
            GetContextId: GetContextId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo2_Impl: ::windows_core::BaseImpl + IObjectContextInfo_Impl {
    fn GetPartitionId(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetApplicationId(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetApplicationInstanceId(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IObjectContextInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IObjectContextInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectContextInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPartitionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPartitionId(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn GetApplicationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApplicationId(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApplicationInstanceId(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        IObjectContextInfo2_Vtbl {
            base__: <IObjectContextInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPartitionId: GetPartitionId::<Identity, Impl, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, Impl, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IObjectContextTip_Impl: ::windows_core::BaseImpl {
    fn GetTipUrl(this: &Self::This, ptipurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectContextTip {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectContextTip {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTipUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptipurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTipUrl(this, ::core::mem::transmute_copy(&ptipurl)).into())
        }
        IObjectContextTip_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetTipUrl: GetTipUrl::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectControl_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This);
    fn CanBePooled(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IObjectControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this))
        }
        unsafe extern "system" fn CanBePooled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanBePooled(this))
        }
        IObjectControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPlaybackControl_Impl: ::windows_core::BaseImpl {
    fn FinalClientRetry(this: &Self::This) -> ::windows_core::Result<()>;
    fn FinalServerRetry(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPlaybackControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlaybackControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FinalClientRetry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinalClientRetry(this).into())
        }
        unsafe extern "system" fn FinalServerRetry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinalServerRetry(this).into())
        }
        IPlaybackControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FinalClientRetry: FinalClientRetry::<Identity, Impl, OFFSET>,
            FinalServerRetry: FinalServerRetry::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPoolManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ShutdownPool(this: &Self::This, clsidorprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPoolManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPoolManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShutdownPool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownPool(this, ::core::mem::transmute(&clsidorprogid)).into())
        }
        IPoolManager_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ShutdownPool: ShutdownPool::<Identity, Impl, OFFSET> }
    };
}
pub trait IProcessInitializer_Impl: ::windows_core::BaseImpl {
    fn Startup(this: &Self::This, punkprocesscontrol: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IProcessInitializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProcessInitializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Startup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Startup(this, ::windows_core::from_raw_borrowed(&punkprocesscontrol)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        IProcessInitializer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Startup: Startup::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityCallContext_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn IsCallerInRole(this: &Self::This, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsUserInRole(this: &Self::This, puser: *const super::Variant::VARIANT, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISecurityCallContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityCallContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCallerInRole(this, ::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSecurityEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsUserInRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *const super::Variant::VARIANT, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUserInRole(this, ::core::mem::transmute_copy(&puser), ::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISecurityCallContext_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsUserInRole: IsUserInRole::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityCallersColl_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<ISecurityIdentityColl>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISecurityCallersColl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityCallersColl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISecurityCallersColl_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityIdentityColl_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISecurityIdentityColl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityIdentityColl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISecurityIdentityColl_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityProperty_Impl: ::windows_core::BaseImpl {
    fn GetDirectCreatorSID(this: &Self::This, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn GetOriginalCreatorSID(this: &Self::This, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn GetDirectCallerSID(this: &Self::This, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn GetOriginalCallerSID(this: &Self::This, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn ReleaseSID(this: &Self::This, psid: super::super::Foundation::PSID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISecurityProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDirectCreatorSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDirectCreatorSID(this, ::core::mem::transmute_copy(&psid)).into())
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOriginalCreatorSID(this, ::core::mem::transmute_copy(&psid)).into())
        }
        unsafe extern "system" fn GetDirectCallerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDirectCallerSID(this, ::core::mem::transmute_copy(&psid)).into())
        }
        unsafe extern "system" fn GetOriginalCallerSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOriginalCallerSID(this, ::core::mem::transmute_copy(&psid)).into())
        }
        unsafe extern "system" fn ReleaseSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseSID(this, ::core::mem::transmute_copy(&psid)).into())
        }
        ISecurityProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDirectCreatorSID: GetDirectCreatorSID::<Identity, Impl, OFFSET>,
            GetOriginalCreatorSID: GetOriginalCreatorSID::<Identity, Impl, OFFSET>,
            GetDirectCallerSID: GetDirectCallerSID::<Identity, Impl, OFFSET>,
            GetOriginalCallerSID: GetOriginalCallerSID::<Identity, Impl, OFFSET>,
            ReleaseSID: ReleaseSID::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISelectCOMLBServer_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetLBServer(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISelectCOMLBServer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISelectCOMLBServer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this).into())
        }
        unsafe extern "system" fn GetLBServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLBServer(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        ISelectCOMLBServer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            GetLBServer: GetLBServer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISendMethodEvents_Impl: ::windows_core::BaseImpl {
    fn SendMethodCall(this: &Self::This, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32) -> ::windows_core::Result<()>;
    fn SendMethodReturn(this: &Self::This, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32, hrcall: ::windows_core::HRESULT, hrserver: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISendMethodEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISendMethodEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendMethodCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMethodCall(this, ::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth)).into())
        }
        unsafe extern "system" fn SendMethodReturn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32, hrcall: ::windows_core::HRESULT, hrserver: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMethodReturn(this, ::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth), ::core::mem::transmute_copy(&hrcall), ::core::mem::transmute_copy(&hrserver)).into())
        }
        ISendMethodEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendMethodCall: SendMethodCall::<Identity, Impl, OFFSET>,
            SendMethodReturn: SendMethodReturn::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceActivity_Impl: ::windows_core::BaseImpl {
    fn SynchronousCall(this: &Self::This, piservicecall: ::core::option::Option<&IServiceCall>) -> ::windows_core::Result<()>;
    fn AsynchronousCall(this: &Self::This, piservicecall: ::core::option::Option<&IServiceCall>) -> ::windows_core::Result<()>;
    fn BindToCurrentThread(this: &Self::This) -> ::windows_core::Result<()>;
    fn UnbindFromThread(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceActivity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceActivity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SynchronousCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCall(this, ::windows_core::from_raw_borrowed(&piservicecall)).into())
        }
        unsafe extern "system" fn AsynchronousCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsynchronousCall(this, ::windows_core::from_raw_borrowed(&piservicecall)).into())
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToCurrentThread(this).into())
        }
        unsafe extern "system" fn UnbindFromThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnbindFromThread(this).into())
        }
        IServiceActivity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsynchronousCall: AsynchronousCall::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceCall_Impl: ::windows_core::BaseImpl {
    fn OnCall(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceCall {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceCall {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCall(this).into())
        }
        IServiceCall_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnCall: OnCall::<Identity, Impl, OFFSET> }
    };
}
pub trait IServiceComTIIntrinsicsConfig_Impl: ::windows_core::BaseImpl {
    fn ComTIIntrinsicsConfig(this: &Self::This, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceComTIIntrinsicsConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceComTIIntrinsicsConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ComTIIntrinsicsConfig(this, ::core::mem::transmute_copy(&comtiintrinsicsconfig)).into())
        }
        IServiceComTIIntrinsicsConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ComTIIntrinsicsConfig: ComTIIntrinsicsConfig::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceIISIntrinsicsConfig_Impl: ::windows_core::BaseImpl {
    fn IISIntrinsicsConfig(this: &Self::This, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceIISIntrinsicsConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceIISIntrinsicsConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IISIntrinsicsConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IISIntrinsicsConfig(this, ::core::mem::transmute_copy(&iisintrinsicsconfig)).into())
        }
        IServiceIISIntrinsicsConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IISIntrinsicsConfig: IISIntrinsicsConfig::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceInheritanceConfig_Impl: ::windows_core::BaseImpl {
    fn ContainingContextTreatment(this: &Self::This, inheritanceconfig: CSC_InheritanceConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceInheritanceConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceInheritanceConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContainingContextTreatment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContainingContextTreatment(this, ::core::mem::transmute_copy(&inheritanceconfig)).into())
        }
        IServiceInheritanceConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContainingContextTreatment: ContainingContextTreatment::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServicePartitionConfig_Impl: ::windows_core::BaseImpl {
    fn PartitionConfig(this: &Self::This, partitionconfig: CSC_PartitionConfig) -> ::windows_core::Result<()>;
    fn PartitionID(this: &Self::This, guidpartitionid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServicePartitionConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServicePartitionConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PartitionConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PartitionConfig(this, ::core::mem::transmute_copy(&partitionconfig)).into())
        }
        unsafe extern "system" fn PartitionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PartitionID(this, ::core::mem::transmute_copy(&guidpartitionid)).into())
        }
        IServicePartitionConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PartitionConfig: PartitionConfig::<Identity, Impl, OFFSET>,
            PartitionID: PartitionID::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServicePool_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, ppoolconfig: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetObject(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServicePool {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServicePool {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&ppoolconfig)).into())
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObject(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        IServicePool_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IServicePoolConfig_Impl: ::windows_core::BaseImpl {
    fn SetMaxPoolSize(this: &Self::This, dwmaxpool: u32) -> ::windows_core::Result<()>;
    fn MaxPoolSize(this: &Self::This, pdwmaxpool: *mut u32) -> ::windows_core::Result<()>;
    fn SetMinPoolSize(this: &Self::This, dwminpool: u32) -> ::windows_core::Result<()>;
    fn MinPoolSize(this: &Self::This, pdwminpool: *mut u32) -> ::windows_core::Result<()>;
    fn SetCreationTimeout(this: &Self::This, dwcreationtimeout: u32) -> ::windows_core::Result<()>;
    fn CreationTimeout(this: &Self::This, pdwcreationtimeout: *mut u32) -> ::windows_core::Result<()>;
    fn SetTransactionAffinity(this: &Self::This, ftxaffinity: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn TransactionAffinity(this: &Self::This, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetClassFactory(this: &Self::This, pfactory: ::core::option::Option<&super::Com::IClassFactory>) -> ::windows_core::Result<()>;
    fn ClassFactory(this: &Self::This) -> ::windows_core::Result<super::Com::IClassFactory>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IServicePoolConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServicePoolConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMaxPoolSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxPoolSize(this, ::core::mem::transmute_copy(&dwmaxpool)).into())
        }
        unsafe extern "system" fn MaxPoolSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MaxPoolSize(this, ::core::mem::transmute_copy(&pdwmaxpool)).into())
        }
        unsafe extern "system" fn SetMinPoolSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinPoolSize(this, ::core::mem::transmute_copy(&dwminpool)).into())
        }
        unsafe extern "system" fn MinPoolSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MinPoolSize(this, ::core::mem::transmute_copy(&pdwminpool)).into())
        }
        unsafe extern "system" fn SetCreationTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreationTimeout(this, ::core::mem::transmute_copy(&dwcreationtimeout)).into())
        }
        unsafe extern "system" fn CreationTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreationTimeout(this, ::core::mem::transmute_copy(&pdwcreationtimeout)).into())
        }
        unsafe extern "system" fn SetTransactionAffinity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransactionAffinity(this, ::core::mem::transmute_copy(&ftxaffinity)).into())
        }
        unsafe extern "system" fn TransactionAffinity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransactionAffinity(this, ::core::mem::transmute_copy(&pftxaffinity)).into())
        }
        unsafe extern "system" fn SetClassFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfactory: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClassFactory(this, ::windows_core::from_raw_borrowed(&pfactory)).into())
        }
        unsafe extern "system" fn ClassFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassFactory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IServicePoolConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMaxPoolSize: SetMaxPoolSize::<Identity, Impl, OFFSET>,
            MaxPoolSize: MaxPoolSize::<Identity, Impl, OFFSET>,
            SetMinPoolSize: SetMinPoolSize::<Identity, Impl, OFFSET>,
            MinPoolSize: MinPoolSize::<Identity, Impl, OFFSET>,
            SetCreationTimeout: SetCreationTimeout::<Identity, Impl, OFFSET>,
            CreationTimeout: CreationTimeout::<Identity, Impl, OFFSET>,
            SetTransactionAffinity: SetTransactionAffinity::<Identity, Impl, OFFSET>,
            TransactionAffinity: TransactionAffinity::<Identity, Impl, OFFSET>,
            SetClassFactory: SetClassFactory::<Identity, Impl, OFFSET>,
            ClassFactory: ClassFactory::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceSxsConfig_Impl: ::windows_core::BaseImpl {
    fn SxsConfig(this: &Self::This, scsconfig: CSC_SxsConfig) -> ::windows_core::Result<()>;
    fn SxsName(this: &Self::This, szsxsname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SxsDirectory(this: &Self::This, szsxsdirectory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceSxsConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceSxsConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SxsConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SxsConfig(this, ::core::mem::transmute_copy(&scsconfig)).into())
        }
        unsafe extern "system" fn SxsName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szsxsname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SxsName(this, ::core::mem::transmute(&szsxsname)).into())
        }
        unsafe extern "system" fn SxsDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szsxsdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SxsDirectory(this, ::core::mem::transmute(&szsxsdirectory)).into())
        }
        IServiceSxsConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SxsConfig: SxsConfig::<Identity, Impl, OFFSET>,
            SxsName: SxsName::<Identity, Impl, OFFSET>,
            SxsDirectory: SxsDirectory::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceSynchronizationConfig_Impl: ::windows_core::BaseImpl {
    fn ConfigureSynchronization(this: &Self::This, synchconfig: CSC_SynchronizationConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceSynchronizationConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceSynchronizationConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConfigureSynchronization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureSynchronization(this, ::core::mem::transmute_copy(&synchconfig)).into())
        }
        IServiceSynchronizationConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConfigureSynchronization: ConfigureSynchronization::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_DistributedTransactionCoordinator\"`"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceSysTxnConfig_Impl: ::windows_core::BaseImpl + IServiceTransactionConfig_Impl {
    fn ConfigureBYOTSysTxn(this: &Self::This, ptxproxy: ::core::option::Option<&ITransactionProxy>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::Iids for IServiceSysTxnConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IServiceTransactionConfig);
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceSysTxnConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptxproxy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureBYOTSysTxn(this, ::windows_core::from_raw_borrowed(&ptxproxy)).into())
        }
        IServiceSysTxnConfig_Vtbl {
            base__: <IServiceTransactionConfig as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConfigureBYOTSysTxn: ConfigureBYOTSysTxn::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceThreadPoolConfig_Impl: ::windows_core::BaseImpl {
    fn SelectThreadPool(this: &Self::This, threadpool: CSC_ThreadPool) -> ::windows_core::Result<()>;
    fn SetBindingInfo(this: &Self::This, binding: CSC_Binding) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceThreadPoolConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceThreadPoolConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SelectThreadPool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectThreadPool(this, ::core::mem::transmute_copy(&threadpool)).into())
        }
        unsafe extern "system" fn SetBindingInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBindingInfo(this, ::core::mem::transmute_copy(&binding)).into())
        }
        IServiceThreadPoolConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SelectThreadPool: SelectThreadPool::<Identity, Impl, OFFSET>,
            SetBindingInfo: SetBindingInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceTrackerConfig_Impl: ::windows_core::BaseImpl {
    fn TrackerConfig(this: &Self::This, trackerconfig: CSC_TrackerConfig, sztrackerappname: &::windows_core::PCWSTR, sztrackerctxname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceTrackerConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceTrackerConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TrackerConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: ::windows_core::PCWSTR, sztrackerctxname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TrackerConfig(this, ::core::mem::transmute_copy(&trackerconfig), ::core::mem::transmute(&sztrackerappname), ::core::mem::transmute(&sztrackerctxname)).into())
        }
        IServiceTrackerConfig_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TrackerConfig: TrackerConfig::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_DistributedTransactionCoordinator\"`"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceTransactionConfig_Impl: ::windows_core::BaseImpl + IServiceTransactionConfigBase_Impl {
    fn ConfigureBYOT(this: &Self::This, pitxbyot: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransaction>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::Iids for IServiceTransactionConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IServiceTransactionConfigBase);
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceTransactionConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConfigureBYOT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitxbyot: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureBYOT(this, ::windows_core::from_raw_borrowed(&pitxbyot)).into())
        }
        IServiceTransactionConfig_Vtbl {
            base__: <IServiceTransactionConfigBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConfigureBYOT: ConfigureBYOT::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IServiceTransactionConfigBase_Impl: ::windows_core::BaseImpl {
    fn ConfigureTransaction(this: &Self::This, transactionconfig: CSC_TransactionConfig) -> ::windows_core::Result<()>;
    fn IsolationLevel(this: &Self::This, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::Result<()>;
    fn TransactionTimeout(this: &Self::This, ultimeoutsec: u32) -> ::windows_core::Result<()>;
    fn BringYourOwnTransaction(this: &Self::This, sztipurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn NewTransactionDescription(this: &Self::This, sztxdesc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IServiceTransactionConfigBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServiceTransactionConfigBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConfigureTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureTransaction(this, ::core::mem::transmute_copy(&transactionconfig)).into())
        }
        unsafe extern "system" fn IsolationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsolationLevel(this, ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn TransactionTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransactionTimeout(this, ::core::mem::transmute_copy(&ultimeoutsec)).into())
        }
        unsafe extern "system" fn BringYourOwnTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sztipurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BringYourOwnTransaction(this, ::core::mem::transmute(&sztipurl)).into())
        }
        unsafe extern "system" fn NewTransactionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sztxdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewTransactionDescription(this, ::core::mem::transmute(&sztxdesc)).into())
        }
        IServiceTransactionConfigBase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConfigureTransaction: ConfigureTransaction::<Identity, Impl, OFFSET>,
            IsolationLevel: IsolationLevel::<Identity, Impl, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, Impl, OFFSET>,
            BringYourOwnTransaction: BringYourOwnTransaction::<Identity, Impl, OFFSET>,
            NewTransactionDescription: NewTransactionDescription::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedProperty_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(this: &Self::This, val: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISharedProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISharedProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&val)).into())
        }
        ISharedProperty_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedPropertyGroup_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CreatePropertyByPosition(this: &Self::This, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows_core::Result<()>;
    fn get_PropertyByPosition(this: &Self::This, index: i32) -> ::windows_core::Result<ISharedProperty>;
    fn CreateProperty(this: &Self::This, name: &::windows_core::BSTR, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows_core::Result<()>;
    fn get_Property(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<ISharedProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISharedPropertyGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISharedPropertyGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePropertyByPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePropertyByPosition(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into())
        }
        unsafe extern "system" fn get_PropertyByPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PropertyByPosition(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateProperty(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into())
        }
        unsafe extern "system" fn get_Property<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Property(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISharedPropertyGroup_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePropertyByPosition: CreatePropertyByPosition::<Identity, Impl, OFFSET>,
            get_PropertyByPosition: get_PropertyByPosition::<Identity, Impl, OFFSET>,
            CreateProperty: CreateProperty::<Identity, Impl, OFFSET>,
            get_Property: get_Property::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedPropertyGroupManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CreatePropertyGroup(this: &Self::This, name: &::windows_core::BSTR, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut ::core::option::Option<ISharedPropertyGroup>) -> ::windows_core::Result<()>;
    fn get_Group(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<ISharedPropertyGroup>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISharedPropertyGroupManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISharedPropertyGroupManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePropertyGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePropertyGroup(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&dwisomode), ::core::mem::transmute_copy(&dwrelmode), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppgroup)).into())
        }
        unsafe extern "system" fn get_Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Group(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISharedPropertyGroupManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePropertyGroup: CreatePropertyGroup::<Identity, Impl, OFFSET>,
            get_Group: get_Group::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISystemAppEventData_Impl: ::windows_core::BaseImpl {
    fn Startup(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnDataChanged(this: &Self::This, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: &::windows_core::BSTR, dwreason: u32, u64tracehandle: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISystemAppEventData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISystemAppEventData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Startup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Startup(this).into())
        }
        unsafe extern "system" fn OnDataChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataChanged(this, ::core::mem::transmute_copy(&dwpid), ::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&dwnumbersinks), ::core::mem::transmute(&bstrdwmethodmask), ::core::mem::transmute_copy(&dwreason), ::core::mem::transmute_copy(&u64tracehandle)).into())
        }
        ISystemAppEventData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Startup: Startup::<Identity, Impl, OFFSET>,
            OnDataChanged: OnDataChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IThreadPoolKnobs_Impl: ::windows_core::BaseImpl {
    fn GetMaxThreads(this: &Self::This, plcmaxthreads: *mut i32) -> ::windows_core::Result<()>;
    fn GetCurrentThreads(this: &Self::This, plccurrentthreads: *mut i32) -> ::windows_core::Result<()>;
    fn SetMaxThreads(this: &Self::This, lcmaxthreads: i32) -> ::windows_core::Result<()>;
    fn GetDeleteDelay(this: &Self::This, pmsecdeletedelay: *mut i32) -> ::windows_core::Result<()>;
    fn SetDeleteDelay(this: &Self::This, msecdeletedelay: i32) -> ::windows_core::Result<()>;
    fn GetMaxQueuedRequests(this: &Self::This, plcmaxqueuedrequests: *mut i32) -> ::windows_core::Result<()>;
    fn GetCurrentQueuedRequests(this: &Self::This, plccurrentqueuedrequests: *mut i32) -> ::windows_core::Result<()>;
    fn SetMaxQueuedRequests(this: &Self::This, lcmaxqueuedrequests: i32) -> ::windows_core::Result<()>;
    fn SetMinThreads(this: &Self::This, lcminthreads: i32) -> ::windows_core::Result<()>;
    fn SetQueueDepth(this: &Self::This, lcqueuedepth: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IThreadPoolKnobs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IThreadPoolKnobs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaxThreads(this, ::core::mem::transmute_copy(&plcmaxthreads)).into())
        }
        unsafe extern "system" fn GetCurrentThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentThreads(this, ::core::mem::transmute_copy(&plccurrentthreads)).into())
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxThreads(this, ::core::mem::transmute_copy(&lcmaxthreads)).into())
        }
        unsafe extern "system" fn GetDeleteDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeleteDelay(this, ::core::mem::transmute_copy(&pmsecdeletedelay)).into())
        }
        unsafe extern "system" fn SetDeleteDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeleteDelay(this, ::core::mem::transmute_copy(&msecdeletedelay)).into())
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaxQueuedRequests(this, ::core::mem::transmute_copy(&plcmaxqueuedrequests)).into())
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentQueuedRequests(this, ::core::mem::transmute_copy(&plccurrentqueuedrequests)).into())
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxQueuedRequests(this, ::core::mem::transmute_copy(&lcmaxqueuedrequests)).into())
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinThreads(this, ::core::mem::transmute_copy(&lcminthreads)).into())
        }
        unsafe extern "system" fn SetQueueDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueueDepth(this, ::core::mem::transmute_copy(&lcqueuedepth)).into())
        }
        IThreadPoolKnobs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetCurrentThreads: GetCurrentThreads::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetDeleteDelay: GetDeleteDelay::<Identity, Impl, OFFSET>,
            SetDeleteDelay: SetDeleteDelay::<Identity, Impl, OFFSET>,
            GetMaxQueuedRequests: GetMaxQueuedRequests::<Identity, Impl, OFFSET>,
            GetCurrentQueuedRequests: GetCurrentQueuedRequests::<Identity, Impl, OFFSET>,
            SetMaxQueuedRequests: SetMaxQueuedRequests::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITransactionContext_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CreateInstance(this: &Self::This, pszprogid: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITransactionContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pobject: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this, ::core::mem::transmute(&pszprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        ITransactionContext_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionContextEx_Impl: ::windows_core::BaseImpl {
    fn CreateInstance(this: &Self::This, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionContextEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionContextEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstance(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        ITransactionContextEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionProperty_Impl: ::windows_core::BaseImpl {
    fn Reserved1(this: &Self::This);
    fn Reserved2(this: &Self::This);
    fn Reserved3(this: &Self::This);
    fn Reserved4(this: &Self::This);
    fn Reserved5(this: &Self::This);
    fn Reserved6(this: &Self::This);
    fn Reserved7(this: &Self::This);
    fn Reserved8(this: &Self::This);
    fn Reserved9(this: &Self::This);
    fn GetTransactionResourcePool(this: &Self::This) -> ::windows_core::Result<ITransactionResourcePool>;
    fn Reserved10(this: &Self::This);
    fn Reserved11(this: &Self::This);
    fn Reserved12(this: &Self::This);
    fn Reserved13(this: &Self::This);
    fn Reserved14(this: &Self::This);
    fn Reserved15(this: &Self::This);
    fn Reserved16(this: &Self::This);
    fn Reserved17(this: &Self::This);
}
impl ::windows_core::Iids for ITransactionProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved1(this))
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved2(this))
        }
        unsafe extern "system" fn Reserved3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved3(this))
        }
        unsafe extern "system" fn Reserved4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved4(this))
        }
        unsafe extern "system" fn Reserved5<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved5(this))
        }
        unsafe extern "system" fn Reserved6<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved6(this))
        }
        unsafe extern "system" fn Reserved7<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved7(this))
        }
        unsafe extern "system" fn Reserved8<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved8(this))
        }
        unsafe extern "system" fn Reserved9<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved9(this))
        }
        unsafe extern "system" fn GetTransactionResourcePool<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptxpool: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransactionResourcePool(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptxpool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reserved10<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved10(this))
        }
        unsafe extern "system" fn Reserved11<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved11(this))
        }
        unsafe extern "system" fn Reserved12<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved12(this))
        }
        unsafe extern "system" fn Reserved13<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved13(this))
        }
        unsafe extern "system" fn Reserved14<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved14(this))
        }
        unsafe extern "system" fn Reserved15<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved15(this))
        }
        unsafe extern "system" fn Reserved16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved16(this))
        }
        unsafe extern "system" fn Reserved17<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved17(this))
        }
        ITransactionProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            Reserved3: Reserved3::<Identity, Impl, OFFSET>,
            Reserved4: Reserved4::<Identity, Impl, OFFSET>,
            Reserved5: Reserved5::<Identity, Impl, OFFSET>,
            Reserved6: Reserved6::<Identity, Impl, OFFSET>,
            Reserved7: Reserved7::<Identity, Impl, OFFSET>,
            Reserved8: Reserved8::<Identity, Impl, OFFSET>,
            Reserved9: Reserved9::<Identity, Impl, OFFSET>,
            GetTransactionResourcePool: GetTransactionResourcePool::<Identity, Impl, OFFSET>,
            Reserved10: Reserved10::<Identity, Impl, OFFSET>,
            Reserved11: Reserved11::<Identity, Impl, OFFSET>,
            Reserved12: Reserved12::<Identity, Impl, OFFSET>,
            Reserved13: Reserved13::<Identity, Impl, OFFSET>,
            Reserved14: Reserved14::<Identity, Impl, OFFSET>,
            Reserved15: Reserved15::<Identity, Impl, OFFSET>,
            Reserved16: Reserved16::<Identity, Impl, OFFSET>,
            Reserved17: Reserved17::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_DistributedTransactionCoordinator\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionProxy_Impl: ::windows_core::BaseImpl {
    fn Commit(this: &Self::This, guid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn Promote(this: &Self::This) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransaction>;
    fn CreateVoter(this: &Self::This, ptxasync: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>;
    fn GetIsolationLevel(this: &Self::This, __midl__itransactionproxy0000: *mut i32) -> ::windows_core::Result<()>;
    fn GetIdentifier(this: &Self::This, pbstridentifier: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn IsReusable(this: &Self::This, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ::windows_core::Iids for ITransactionProxy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionProxy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute(&guid)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn Promote<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Promote(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVoter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptxasync: *mut ::core::ffi::c_void, ppballot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVoter(this, ::windows_core::from_raw_borrowed(&ptxasync)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppballot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIsolationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIsolationLevel(this, ::core::mem::transmute_copy(&__midl__itransactionproxy0000)).into())
        }
        unsafe extern "system" fn GetIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdentifier(this, ::core::mem::transmute_copy(&pbstridentifier)).into())
        }
        unsafe extern "system" fn IsReusable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsReusable(this, ::core::mem::transmute_copy(&pfisreusable)).into())
        }
        ITransactionProxy_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            Promote: Promote::<Identity, Impl, OFFSET>,
            CreateVoter: CreateVoter::<Identity, Impl, OFFSET>,
            GetIsolationLevel: GetIsolationLevel::<Identity, Impl, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET>,
            IsReusable: IsReusable::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionResourcePool_Impl: ::windows_core::BaseImpl {
    fn PutResource(this: &Self::This, ppool: ::core::option::Option<&IObjPool>, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetResource(this: &Self::This, ppool: ::core::option::Option<&IObjPool>) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for ITransactionResourcePool {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionResourcePool {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PutResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutResource(this, ::windows_core::from_raw_borrowed(&ppool), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn GetResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResource(this, ::windows_core::from_raw_borrowed(&ppool)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionResourcePool_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PutResource: PutResource::<Identity, Impl, OFFSET>,
            GetResource: GetResource::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITransactionStatus_Impl: ::windows_core::BaseImpl {
    fn SetTransactionStatus(this: &Self::This, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetTransactionStatus(this: &Self::This, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITransactionStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTransactionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransactionStatus(this, ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn GetTransactionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransactionStatus(this, ::core::mem::transmute_copy(&phrstatus)).into())
        }
        ITransactionStatus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTransactionStatus: SetTransactionStatus::<Identity, Impl, OFFSET>,
            GetTransactionStatus: GetTransactionStatus::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITxProxyHolder_Impl: ::windows_core::BaseImpl {
    fn GetIdentifier(this: &Self::This, pguidltx: *mut ::windows_core::GUID);
}
impl ::windows_core::Iids for ITxProxyHolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITxProxyHolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows_core::GUID) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdentifier(this, ::core::mem::transmute_copy(&pguidltx)))
        }
        ITxProxyHolder_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ObjectContext_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CreateInstance(this: &Self::This, bstrprogid: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetAbort(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnableCommit(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableCommit(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsInTransaction(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCallerInRole(this: &Self::This, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Security(this: &Self::This) -> ::windows_core::Result<SecurityProperty>;
    fn ContextInfo(this: &Self::This) -> ::windows_core::Result<ContextInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ObjectContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ObjectContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pobject: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this, ::core::mem::transmute(&bstrprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComplete(this).into())
        }
        unsafe extern "system" fn SetAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAbort(this).into())
        }
        unsafe extern "system" fn EnableCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableCommit(this).into())
        }
        unsafe extern "system" fn DisableCommit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableCommit(this).into())
        }
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisintx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSecurityEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCallerInRole(this, ::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContextInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContextInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontextinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ObjectContext_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            SetComplete: SetComplete::<Identity, Impl, OFFSET>,
            SetAbort: SetAbort::<Identity, Impl, OFFSET>,
            EnableCommit: EnableCommit::<Identity, Impl, OFFSET>,
            DisableCommit: DisableCommit::<Identity, Impl, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            ContextInfo: ContextInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ObjectControl_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn CanBePooled(this: &Self::This, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ObjectControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ObjectControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this).into())
        }
        unsafe extern "system" fn CanBePooled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanBePooled(this, ::core::mem::transmute_copy(&pbpoolable)).into())
        }
        ObjectControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SecurityProperty_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetDirectCallerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDirectCreatorName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetOriginalCallerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetOriginalCreatorName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for SecurityProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for SecurityProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDirectCallerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDirectCallerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDirectCreatorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDirectCreatorName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOriginalCallerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOriginalCallerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOriginalCreatorName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOriginalCreatorName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        SecurityProperty_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDirectCallerName: GetDirectCallerName::<Identity, Impl, OFFSET>,
            GetDirectCreatorName: GetDirectCreatorName::<Identity, Impl, OFFSET>,
            GetOriginalCallerName: GetOriginalCallerName::<Identity, Impl, OFFSET>,
            GetOriginalCreatorName: GetOriginalCreatorName::<Identity, Impl, OFFSET>,
        }
    };
}
