pub trait IActionOnCLREvent_Impl: ::windows_core::BaseImpl {
    fn OnEvent(this: &Self::This, event: EClrEvent, data: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActionOnCLREvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionOnCLREvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActionOnCLREvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActionOnCLREvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: EClrEvent, data: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEvent(this, ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&data)).into())
        }
        IActionOnCLREvent_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnEvent: OnEvent::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IApartmentCallback_Impl: ::windows_core::BaseImpl {
    fn DoCallback(this: &Self::This, pfunc: usize, pdata: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IApartmentCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApartmentCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApartmentCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApartmentCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunc: usize, pdata: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoCallback(this, ::core::mem::transmute_copy(&pfunc), ::core::mem::transmute_copy(&pdata)).into())
        }
        IApartmentCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DoCallback: DoCallback::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppDomainBinding_Impl: ::windows_core::BaseImpl {
    fn OnAppDomain(this: &Self::This, pappdomain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAppDomainBinding {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppDomainBinding_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppDomainBinding {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnAppDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppDomainBinding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAppDomain(this, ::windows_core::from_raw_borrowed(&pappdomain)).into())
        }
        IAppDomainBinding_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnAppDomain: OnAppDomain::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRAppDomainResourceMonitor_Impl: ::windows_core::BaseImpl {
    fn GetCurrentAllocated(this: &Self::This, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows_core::Result<()>;
    fn GetCurrentSurvived(this: &Self::This, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows_core::Result<()>;
    fn GetCurrentCpuTime(this: &Self::This, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRAppDomainResourceMonitor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRAppDomainResourceMonitor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentAllocated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pbytesallocated: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentAllocated(this, ::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pbytesallocated)).into())
        }
        unsafe extern "system" fn GetCurrentSurvived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentSurvived(this, ::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pappdomainbytessurvived), ::core::mem::transmute_copy(&ptotalbytessurvived)).into())
        }
        unsafe extern "system" fn GetCurrentCpuTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAppDomainResourceMonitor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pmilliseconds: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentCpuTime(this, ::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pmilliseconds)).into())
        }
        ICLRAppDomainResourceMonitor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentAllocated: GetCurrentAllocated::<Identity, Impl, OFFSET>,
            GetCurrentSurvived: GetCurrentSurvived::<Identity, Impl, OFFSET>,
            GetCurrentCpuTime: GetCurrentCpuTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICLRAssemblyIdentityManager_Impl: ::windows_core::BaseImpl {
    fn GetCLRAssemblyReferenceList(this: &Self::This, ppwzassemblyreferences: *const ::windows_core::PCWSTR, dwnumofreferences: u32) -> ::windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetBindingIdentityFromFile(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn GetBindingIdentityFromStream(this: &Self::This, pstream: ::core::option::Option<&super::Com::IStream>, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn GetReferencedAssembliesFromFile(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: ::core::option::Option<&ICLRAssemblyReferenceList>) -> ::windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetReferencedAssembliesFromStream(this: &Self::This, pstream: ::core::option::Option<&super::Com::IStream>, dwflags: u32, pexcludeassemblieslist: ::core::option::Option<&ICLRAssemblyReferenceList>) -> ::windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetProbingAssembliesFromReference(this: &Self::This, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<ICLRProbingAssemblyEnum>;
    fn IsStronglyNamed(this: &Self::This, pwzassemblyidentity: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ICLRAssemblyIdentityManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRAssemblyIdentityManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCLRAssemblyReferenceList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwzassemblyreferences: *const ::windows_core::PCWSTR, dwnumofreferences: u32, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCLRAssemblyReferenceList(this, ::core::mem::transmute_copy(&ppwzassemblyreferences), ::core::mem::transmute_copy(&dwnumofreferences)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBindingIdentityFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindingIdentityFromFile(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into())
        }
        unsafe extern "system" fn GetBindingIdentityFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindingIdentityFromStream(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into())
        }
        unsafe extern "system" fn GetReferencedAssembliesFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReferencedAssembliesFromFile(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pexcludeassemblieslist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferenceenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReferencedAssembliesFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pexcludeassemblieslist: *mut ::core::ffi::c_void, ppreferenceenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReferencedAssembliesFromStream(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pexcludeassemblieslist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferenceenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProbingAssembliesFromReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: ::windows_core::PCWSTR, ppprobingassemblyenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProbingAssembliesFromReference(this, ::core::mem::transmute_copy(&dwmachinetype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pwzreferenceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprobingassemblyenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsStronglyNamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyIdentityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzassemblyidentity: ::windows_core::PCWSTR, pbisstronglynamed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsStronglyNamed(this, ::core::mem::transmute(&pwzassemblyidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisstronglynamed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICLRAssemblyIdentityManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCLRAssemblyReferenceList: GetCLRAssemblyReferenceList::<Identity, Impl, OFFSET>,
            GetBindingIdentityFromFile: GetBindingIdentityFromFile::<Identity, Impl, OFFSET>,
            GetBindingIdentityFromStream: GetBindingIdentityFromStream::<Identity, Impl, OFFSET>,
            GetReferencedAssembliesFromFile: GetReferencedAssembliesFromFile::<Identity, Impl, OFFSET>,
            GetReferencedAssembliesFromStream: GetReferencedAssembliesFromStream::<Identity, Impl, OFFSET>,
            GetProbingAssembliesFromReference: GetProbingAssembliesFromReference::<Identity, Impl, OFFSET>,
            IsStronglyNamed: IsStronglyNamed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRAssemblyReferenceList_Impl: ::windows_core::BaseImpl {
    fn IsStringAssemblyReferenceInList(this: &Self::This, pwzassemblyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsAssemblyReferenceInList(this: &Self::This, pname: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRAssemblyReferenceList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRAssemblyReferenceList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsStringAssemblyReferenceInList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzassemblyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsStringAssemblyReferenceInList(this, ::core::mem::transmute(&pwzassemblyname)).into())
        }
        unsafe extern "system" fn IsAssemblyReferenceInList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRAssemblyReferenceList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsAssemblyReferenceInList(this, ::windows_core::from_raw_borrowed(&pname)).into())
        }
        ICLRAssemblyReferenceList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsStringAssemblyReferenceInList: IsStringAssemblyReferenceInList::<Identity, Impl, OFFSET>,
            IsAssemblyReferenceInList: IsAssemblyReferenceInList::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRControl_Impl: ::windows_core::BaseImpl {
    fn GetCLRManager(this: &Self::This, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetAppDomainManagerType(this: &Self::This, pwzappdomainmanagerassembly: &::windows_core::PCWSTR, pwzappdomainmanagertype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCLRManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCLRManager(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppobject)).into())
        }
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzappdomainmanagerassembly: ::windows_core::PCWSTR, pwzappdomainmanagertype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppDomainManagerType(this, ::core::mem::transmute(&pwzappdomainmanagerassembly), ::core::mem::transmute(&pwzappdomainmanagertype)).into())
        }
        ICLRControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCLRManager: GetCLRManager::<Identity, Impl, OFFSET>,
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait ICLRDebugManager_Impl: ::windows_core::BaseImpl {
    fn BeginConnection(this: &Self::This, dwconnectionid: u32, szconnectionname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetConnectionTasks(this: &Self::This, id: u32, dwcount: u32, ppclrtask: *const ::core::option::Option<ICLRTask>) -> ::windows_core::Result<()>;
    fn EndConnection(this: &Self::This, dwconnectionid: u32) -> ::windows_core::Result<()>;
    fn SetDacl(this: &Self::This, pacl: *const super::super::Security::ACL) -> ::windows_core::Result<()>;
    fn GetDacl(this: &Self::This) -> ::windows_core::Result<*mut super::super::Security::ACL>;
    fn IsDebuggerAttached(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSymbolReadingPolicy(this: &Self::This, policy: ESymbolReadingPolicy) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::Iids for ICLRDebugManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRDebugManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnectionid: u32, szconnectionname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginConnection(this, ::core::mem::transmute_copy(&dwconnectionid), ::core::mem::transmute(&szconnectionname)).into())
        }
        unsafe extern "system" fn SetConnectionTasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: u32, dwcount: u32, ppclrtask: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectionTasks(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&ppclrtask)).into())
        }
        unsafe extern "system" fn EndConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnectionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndConnection(this, ::core::mem::transmute_copy(&dwconnectionid)).into())
        }
        unsafe extern "system" fn SetDacl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacl: *const super::super::Security::ACL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDacl(this, ::core::mem::transmute_copy(&pacl)).into())
        }
        unsafe extern "system" fn GetDacl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacl: *mut *mut super::super::Security::ACL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDacl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDebuggerAttached<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDebuggerAttached(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbattached, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSymbolReadingPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: ESymbolReadingPolicy) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSymbolReadingPolicy(this, ::core::mem::transmute_copy(&policy)).into())
        }
        ICLRDebugManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginConnection: BeginConnection::<Identity, Impl, OFFSET>,
            SetConnectionTasks: SetConnectionTasks::<Identity, Impl, OFFSET>,
            EndConnection: EndConnection::<Identity, Impl, OFFSET>,
            SetDacl: SetDacl::<Identity, Impl, OFFSET>,
            GetDacl: GetDacl::<Identity, Impl, OFFSET>,
            IsDebuggerAttached: IsDebuggerAttached::<Identity, Impl, OFFSET>,
            SetSymbolReadingPolicy: SetSymbolReadingPolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRDebugging_Impl: ::windows_core::BaseImpl {
    fn OpenVirtualProcess(this: &Self::This, modulebaseaddress: u64, pdatatarget: ::core::option::Option<&::windows_core::IUnknown>, plibraryprovider: ::core::option::Option<&ICLRDebuggingLibraryProvider>, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const ::windows_core::GUID, ppprocess: *mut ::core::option::Option<::windows_core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows_core::Result<()>;
    fn CanUnloadNow(this: &Self::This, hmodule: super::super::Foundation::HMODULE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRDebugging {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRDebugging {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenVirtualProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modulebaseaddress: u64, pdatatarget: *mut ::core::ffi::c_void, plibraryprovider: *mut ::core::ffi::c_void, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const ::windows_core::GUID, ppprocess: *mut *mut ::core::ffi::c_void, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenVirtualProcess(this, ::core::mem::transmute_copy(&modulebaseaddress), ::windows_core::from_raw_borrowed(&pdatatarget), ::windows_core::from_raw_borrowed(&plibraryprovider), ::core::mem::transmute_copy(&pmaxdebuggersupportedversion), ::core::mem::transmute_copy(&riidprocess), ::core::mem::transmute_copy(&ppprocess), ::core::mem::transmute_copy(&pversion), ::core::mem::transmute_copy(&pdwflags)).into())
        }
        unsafe extern "system" fn CanUnloadNow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebugging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanUnloadNow(this, ::core::mem::transmute_copy(&hmodule)).into())
        }
        ICLRDebugging_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenVirtualProcess: OpenVirtualProcess::<Identity, Impl, OFFSET>,
            CanUnloadNow: CanUnloadNow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRDebuggingLibraryProvider_Impl: ::windows_core::BaseImpl {
    fn ProvideLibrary(this: &Self::This, pwszfilename: &::windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32) -> ::windows_core::Result<super::super::Foundation::HMODULE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRDebuggingLibraryProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebuggingLibraryProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRDebuggingLibraryProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProvideLibrary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDebuggingLibraryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProvideLibrary(this, ::core::mem::transmute(&pwszfilename), ::core::mem::transmute_copy(&dwtimestamp), ::core::mem::transmute_copy(&dwsizeofimage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmodule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICLRDebuggingLibraryProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ProvideLibrary: ProvideLibrary::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRDomainManager_Impl: ::windows_core::BaseImpl {
    fn SetAppDomainManagerType(this: &Self::This, wszappdomainmanagerassembly: &::windows_core::PCWSTR, wszappdomainmanagertype: &::windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows_core::Result<()>;
    fn SetPropertiesForDefaultAppDomain(this: &Self::This, nproperties: u32, pwszpropertynames: *const ::windows_core::PCWSTR, pwszpropertyvalues: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRDomainManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRDomainManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAppDomainManagerType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszappdomainmanagerassembly: ::windows_core::PCWSTR, wszappdomainmanagertype: ::windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppDomainManagerType(this, ::core::mem::transmute(&wszappdomainmanagerassembly), ::core::mem::transmute(&wszappdomainmanagertype), ::core::mem::transmute_copy(&dwinitializedomainflags)).into())
        }
        unsafe extern "system" fn SetPropertiesForDefaultAppDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRDomainManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nproperties: u32, pwszpropertynames: *const ::windows_core::PCWSTR, pwszpropertyvalues: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertiesForDefaultAppDomain(this, ::core::mem::transmute_copy(&nproperties), ::core::mem::transmute_copy(&pwszpropertynames), ::core::mem::transmute_copy(&pwszpropertyvalues)).into())
        }
        ICLRDomainManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, Impl, OFFSET>,
            SetPropertiesForDefaultAppDomain: SetPropertiesForDefaultAppDomain::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRErrorReportingManager_Impl: ::windows_core::BaseImpl {
    fn GetBucketParametersForCurrentException(this: &Self::This, pparams: *mut BucketParameters) -> ::windows_core::Result<()>;
    fn BeginCustomDump(this: &Self::This, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> ::windows_core::Result<()>;
    fn EndCustomDump(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRErrorReportingManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRErrorReportingManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBucketParametersForCurrentException<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparams: *mut BucketParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBucketParametersForCurrentException(this, ::core::mem::transmute_copy(&pparams)).into())
        }
        unsafe extern "system" fn BeginCustomDump<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginCustomDump(this, ::core::mem::transmute_copy(&dwflavor), ::core::mem::transmute_copy(&dwnumitems), ::core::mem::transmute_copy(&items), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn EndCustomDump<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRErrorReportingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndCustomDump(this).into())
        }
        ICLRErrorReportingManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBucketParametersForCurrentException: GetBucketParametersForCurrentException::<Identity, Impl, OFFSET>,
            BeginCustomDump: BeginCustomDump::<Identity, Impl, OFFSET>,
            EndCustomDump: EndCustomDump::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRGCManager_Impl: ::windows_core::BaseImpl {
    fn Collect(this: &Self::This, generation: i32) -> ::windows_core::Result<()>;
    fn GetStats(this: &Self::This, pstats: *mut COR_GC_STATS) -> ::windows_core::Result<()>;
    fn SetGCStartupLimits(this: &Self::This, segmentsize: u32, maxgen0size: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRGCManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRGCManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Collect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Collect(this, ::core::mem::transmute_copy(&generation)).into())
        }
        unsafe extern "system" fn GetStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStats(this, ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn SetGCStartupLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRGCManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGCStartupLimits(this, ::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into())
        }
        ICLRGCManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Collect: Collect::<Identity, Impl, OFFSET>,
            GetStats: GetStats::<Identity, Impl, OFFSET>,
            SetGCStartupLimits: SetGCStartupLimits::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRGCManager2_Impl: ::windows_core::BaseImpl + ICLRGCManager_Impl {
    fn SetGCStartupLimitsEx(this: &Self::This, segmentsize: usize, maxgen0size: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRGCManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICLRGCManager);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRGCManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRGCManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRGCManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGCStartupLimitsEx(this, ::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into())
        }
        ICLRGCManager2_Vtbl {
            base__: <ICLRGCManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRHostBindingPolicyManager_Impl: ::windows_core::BaseImpl {
    fn ModifyApplicationPolicy(this: &Self::This, pwzsourceassemblyidentity: &::windows_core::PCWSTR, pwztargetassemblyidentity: &::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows_core::Result<()>;
    fn EvaluatePolicy(this: &Self::This, pwzreferenceidentity: &::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRHostBindingPolicyManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRHostBindingPolicyManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ModifyApplicationPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzsourceassemblyidentity: ::windows_core::PCWSTR, pwztargetassemblyidentity: ::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyApplicationPolicy(this, ::core::mem::transmute(&pwzsourceassemblyidentity), ::core::mem::transmute(&pwztargetassemblyidentity), ::core::mem::transmute_copy(&pbapplicationpolicy), ::core::mem::transmute_copy(&cbapppolicysize), ::core::mem::transmute_copy(&dwpolicymodifyflags), ::core::mem::transmute_copy(&pbnewapplicationpolicy), ::core::mem::transmute_copy(&pcbnewapppolicysize)).into())
        }
        unsafe extern "system" fn EvaluatePolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRHostBindingPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzreferenceidentity: ::windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: ::windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EvaluatePolicy(this, ::core::mem::transmute(&pwzreferenceidentity), ::core::mem::transmute_copy(&pbapplicationpolicy), ::core::mem::transmute_copy(&cbapppolicysize), ::core::mem::transmute_copy(&pwzpostpolicyreferenceidentity), ::core::mem::transmute_copy(&pcchpostpolicyreferenceidentity), ::core::mem::transmute_copy(&pdwpoliciesapplied)).into())
        }
        ICLRHostBindingPolicyManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ModifyApplicationPolicy: ModifyApplicationPolicy::<Identity, Impl, OFFSET>,
            EvaluatePolicy: EvaluatePolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRHostProtectionManager_Impl: ::windows_core::BaseImpl {
    fn SetProtectedCategories(this: &Self::This, categories: EApiCategories) -> ::windows_core::Result<()>;
    fn SetEagerSerializeGrantSets(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRHostProtectionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRHostProtectionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProtectedCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, categories: EApiCategories) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtectedCategories(this, ::core::mem::transmute_copy(&categories)).into())
        }
        unsafe extern "system" fn SetEagerSerializeGrantSets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRHostProtectionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEagerSerializeGrantSets(this).into())
        }
        ICLRHostProtectionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProtectedCategories: SetProtectedCategories::<Identity, Impl, OFFSET>,
            SetEagerSerializeGrantSets: SetEagerSerializeGrantSets::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRIoCompletionManager_Impl: ::windows_core::BaseImpl {
    fn OnComplete(this: &Self::This, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRIoCompletionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRIoCompletionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRIoCompletionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnComplete(this, ::core::mem::transmute_copy(&dwerrorcode), ::core::mem::transmute_copy(&numberofbytestransferred), ::core::mem::transmute_copy(&pvoverlapped)).into())
        }
        ICLRIoCompletionManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRMemoryNotificationCallback_Impl: ::windows_core::BaseImpl {
    fn OnMemoryNotification(this: &Self::This, ememoryavailable: EMemoryAvailable) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRMemoryNotificationCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMemoryNotificationCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRMemoryNotificationCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnMemoryNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMemoryNotificationCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ememoryavailable: EMemoryAvailable) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMemoryNotification(this, ::core::mem::transmute_copy(&ememoryavailable)).into())
        }
        ICLRMemoryNotificationCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnMemoryNotification: OnMemoryNotification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICLRMetaHost_Impl: ::windows_core::BaseImpl {
    fn GetRuntime(this: &Self::This, pwzversion: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetVersionFromFile(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::Result<()>;
    fn EnumerateInstalledRuntimes(this: &Self::This) -> ::windows_core::Result<super::Com::IEnumUnknown>;
    fn EnumerateLoadedRuntimes(this: &Self::This, hndprocess: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::Com::IEnumUnknown>;
    fn RequestRuntimeLoadedNotification(this: &Self::This, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows_core::Result<()>;
    fn QueryLegacyV2RuntimeBinding(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ExitProcess(this: &Self::This, iexitcode: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ICLRMetaHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRMetaHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzversion: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRuntime(this, ::core::mem::transmute(&pwzversion), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppruntime)).into())
        }
        unsafe extern "system" fn GetVersionFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersionFromFile(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into())
        }
        unsafe extern "system" fn EnumerateInstalledRuntimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateInstalledRuntimes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateLoadedRuntimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateLoadedRuntimes(this, ::core::mem::transmute_copy(&hndprocess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestRuntimeLoadedNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestRuntimeLoadedNotification(this, ::core::mem::transmute_copy(&pcallbackfunction)).into())
        }
        unsafe extern "system" fn QueryLegacyV2RuntimeBinding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryLegacyV2RuntimeBinding(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn ExitProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iexitcode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExitProcess(this, ::core::mem::transmute_copy(&iexitcode)).into())
        }
        ICLRMetaHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRuntime: GetRuntime::<Identity, Impl, OFFSET>,
            GetVersionFromFile: GetVersionFromFile::<Identity, Impl, OFFSET>,
            EnumerateInstalledRuntimes: EnumerateInstalledRuntimes::<Identity, Impl, OFFSET>,
            EnumerateLoadedRuntimes: EnumerateLoadedRuntimes::<Identity, Impl, OFFSET>,
            RequestRuntimeLoadedNotification: RequestRuntimeLoadedNotification::<Identity, Impl, OFFSET>,
            QueryLegacyV2RuntimeBinding: QueryLegacyV2RuntimeBinding::<Identity, Impl, OFFSET>,
            ExitProcess: ExitProcess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRMetaHostPolicy_Impl: ::windows_core::BaseImpl {
    fn GetRequestedRuntime(this: &Self::This, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: &::windows_core::PCWSTR, pcfgstream: ::core::option::Option<&super::Com::IStream>, pwzversion: &::windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ICLRMetaHostPolicy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHostPolicy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRMetaHostPolicy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRequestedRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRMetaHostPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: ::windows_core::PCWSTR, pcfgstream: *mut ::core::ffi::c_void, pwzversion: ::windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: ::windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const ::windows_core::GUID, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRequestedRuntime(this, ::core::mem::transmute_copy(&dwpolicyflags), ::core::mem::transmute(&pwzbinary), ::windows_core::from_raw_borrowed(&pcfgstream), ::core::mem::transmute(&pwzversion), ::core::mem::transmute_copy(&pcchversion), ::core::mem::transmute_copy(&pwzimageversion), ::core::mem::transmute_copy(&pcchimageversion), ::core::mem::transmute_copy(&pdwconfigflags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppruntime)).into())
        }
        ICLRMetaHostPolicy_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRequestedRuntime: GetRequestedRuntime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLROnEventManager_Impl: ::windows_core::BaseImpl {
    fn RegisterActionOnEvent(this: &Self::This, event: EClrEvent, paction: ::core::option::Option<&IActionOnCLREvent>) -> ::windows_core::Result<()>;
    fn UnregisterActionOnEvent(this: &Self::This, event: EClrEvent, paction: ::core::option::Option<&IActionOnCLREvent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLROnEventManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLROnEventManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterActionOnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterActionOnEvent(this, ::core::mem::transmute_copy(&event), ::windows_core::from_raw_borrowed(&paction)).into())
        }
        unsafe extern "system" fn UnregisterActionOnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLROnEventManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: EClrEvent, paction: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterActionOnEvent(this, ::core::mem::transmute_copy(&event), ::windows_core::from_raw_borrowed(&paction)).into())
        }
        ICLROnEventManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterActionOnEvent: RegisterActionOnEvent::<Identity, Impl, OFFSET>,
            UnregisterActionOnEvent: UnregisterActionOnEvent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRPolicyManager_Impl: ::windows_core::BaseImpl {
    fn SetDefaultAction(this: &Self::This, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetTimeout(this: &Self::This, operation: EClrOperation, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn SetActionOnTimeout(this: &Self::This, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetTimeoutAndAction(this: &Self::This, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetActionOnFailure(this: &Self::This, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn SetUnhandledExceptionPolicy(this: &Self::This, policy: EClrUnhandledException) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRPolicyManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRPolicyManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultAction(this, ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn SetTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimeout(this, ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dwmilliseconds)).into())
        }
        unsafe extern "system" fn SetActionOnTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActionOnTimeout(this, ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn SetTimeoutAndAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimeoutAndAction(this, ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn SetActionOnFailure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActionOnFailure(this, ::core::mem::transmute_copy(&failure), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn SetUnhandledExceptionPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: EClrUnhandledException) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnhandledExceptionPolicy(this, ::core::mem::transmute_copy(&policy)).into())
        }
        ICLRPolicyManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDefaultAction: SetDefaultAction::<Identity, Impl, OFFSET>,
            SetTimeout: SetTimeout::<Identity, Impl, OFFSET>,
            SetActionOnTimeout: SetActionOnTimeout::<Identity, Impl, OFFSET>,
            SetTimeoutAndAction: SetTimeoutAndAction::<Identity, Impl, OFFSET>,
            SetActionOnFailure: SetActionOnFailure::<Identity, Impl, OFFSET>,
            SetUnhandledExceptionPolicy: SetUnhandledExceptionPolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRProbingAssemblyEnum_Impl: ::windows_core::BaseImpl {
    fn Get(this: &Self::This, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRProbingAssemblyEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRProbingAssemblyEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRProbingAssemblyEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRProbingAssemblyEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into())
        }
        ICLRProbingAssemblyEnum_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Get: Get::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRProfiling_Impl: ::windows_core::BaseImpl {
    fn AttachProfiler(this: &Self::This, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows_core::GUID, wszprofilerpath: &::windows_core::PCWSTR, pvclientdata: *const ::core::ffi::c_void, cbclientdata: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRProfiling {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRProfiling_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRProfiling {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AttachProfiler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRProfiling_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const ::windows_core::GUID, wszprofilerpath: ::windows_core::PCWSTR, pvclientdata: *const ::core::ffi::c_void, cbclientdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachProfiler(this, ::core::mem::transmute_copy(&dwprofileeprocessid), ::core::mem::transmute_copy(&dwmillisecondsmax), ::core::mem::transmute_copy(&pclsidprofiler), ::core::mem::transmute(&wszprofilerpath), ::core::mem::transmute_copy(&pvclientdata), ::core::mem::transmute_copy(&cbclientdata)).into())
        }
        ICLRProfiling_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AttachProfiler: AttachProfiler::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRReferenceAssemblyEnum_Impl: ::windows_core::BaseImpl {
    fn Get(this: &Self::This, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRReferenceAssemblyEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRReferenceAssemblyEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRReferenceAssemblyEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRReferenceAssemblyEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffersize)).into())
        }
        ICLRReferenceAssemblyEnum_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Get: Get::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRRuntimeHost_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetHostControl(this: &Self::This, phostcontrol: ::core::option::Option<&IHostControl>) -> ::windows_core::Result<()>;
    fn GetCLRControl(this: &Self::This) -> ::windows_core::Result<ICLRControl>;
    fn UnloadAppDomain(this: &Self::This, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ExecuteInAppDomain(this: &Self::This, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCurrentAppDomainId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ExecuteApplication(this: &Self::This, pwzappfullname: &::windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows_core::PCWSTR) -> ::windows_core::Result<i32>;
    fn ExecuteInDefaultAppDomain(this: &Self::This, pwzassemblypath: &::windows_core::PCWSTR, pwztypename: &::windows_core::PCWSTR, pwzmethodname: &::windows_core::PCWSTR, pwzargument: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRRuntimeHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRRuntimeHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn SetHostControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phostcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHostControl(this, ::windows_core::from_raw_borrowed(&phostcontrol)).into())
        }
        unsafe extern "system" fn GetCLRControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclrcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCLRControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclrcontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnloadAppDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnloadAppDomain(this, ::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&fwaituntildone)).into())
        }
        unsafe extern "system" fn ExecuteInAppDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteInAppDomain(this, ::core::mem::transmute_copy(&dwappdomainid), ::core::mem::transmute_copy(&pcallback), ::core::mem::transmute_copy(&cookie)).into())
        }
        unsafe extern "system" fn GetCurrentAppDomainId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwappdomainid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentAppDomainId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappdomainid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecuteApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzappfullname: ::windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const ::windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const ::windows_core::PCWSTR, preturnvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecuteApplication(this, ::core::mem::transmute(&pwzappfullname), ::core::mem::transmute_copy(&dwmanifestpaths), ::core::mem::transmute_copy(&ppwzmanifestpaths), ::core::mem::transmute_copy(&dwactivationdata), ::core::mem::transmute_copy(&ppwzactivationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preturnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecuteInDefaultAppDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzassemblypath: ::windows_core::PCWSTR, pwztypename: ::windows_core::PCWSTR, pwzmethodname: ::windows_core::PCWSTR, pwzargument: ::windows_core::PCWSTR, preturnvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecuteInDefaultAppDomain(this, ::core::mem::transmute(&pwzassemblypath), ::core::mem::transmute(&pwztypename), ::core::mem::transmute(&pwzmethodname), ::core::mem::transmute(&pwzargument)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preturnvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICLRRuntimeHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            SetHostControl: SetHostControl::<Identity, Impl, OFFSET>,
            GetCLRControl: GetCLRControl::<Identity, Impl, OFFSET>,
            UnloadAppDomain: UnloadAppDomain::<Identity, Impl, OFFSET>,
            ExecuteInAppDomain: ExecuteInAppDomain::<Identity, Impl, OFFSET>,
            GetCurrentAppDomainId: GetCurrentAppDomainId::<Identity, Impl, OFFSET>,
            ExecuteApplication: ExecuteApplication::<Identity, Impl, OFFSET>,
            ExecuteInDefaultAppDomain: ExecuteInDefaultAppDomain::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRRuntimeInfo_Impl: ::windows_core::BaseImpl {
    fn GetVersionString(this: &Self::This, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::Result<()>;
    fn GetRuntimeDirectory(this: &Self::This, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::Result<()>;
    fn IsLoaded(this: &Self::This, hndprocess: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn LoadErrorString(this: &Self::This, iresourceid: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows_core::Result<()>;
    fn LoadLibraryA(this: &Self::This, pwzdllname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetProcAddress(this: &Self::This, pszprocname: &::windows_core::PCSTR) -> ::windows_core::Result<*mut ::core::ffi::c_void>;
    fn GetInterface(this: &Self::This, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsLoadable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDefaultStartupFlags(this: &Self::This, dwstartupflags: u32, pwzhostconfigfile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDefaultStartupFlags(this: &Self::This, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows_core::Result<()>;
    fn BindAsLegacyV2Runtime(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsStarted(this: &Self::This, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRRuntimeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRRuntimeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVersionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersionString(this, ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into())
        }
        unsafe extern "system" fn GetRuntimeDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRuntimeDirectory(this, ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer)).into())
        }
        unsafe extern "system" fn IsLoaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, pbloaded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLoaded(this, ::core::mem::transmute_copy(&hndprocess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbloaded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadErrorString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iresourceid: u32, pwzbuffer: ::windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadErrorString(this, ::core::mem::transmute_copy(&iresourceid), ::core::mem::transmute_copy(&pwzbuffer), ::core::mem::transmute_copy(&pcchbuffer), ::core::mem::transmute_copy(&ilocaleid)).into())
        }
        unsafe extern "system" fn LoadLibraryA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzdllname: ::windows_core::PCWSTR, phndmodule: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadLibraryA(this, ::core::mem::transmute(&pwzdllname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phndmodule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProcAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszprocname: ::windows_core::PCSTR, ppproc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProcAddress(this, ::core::mem::transmute(&pszprocname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterface(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn IsLoadable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbloadable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLoadable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbloadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultStartupFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstartupflags: u32, pwzhostconfigfile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultStartupFlags(this, ::core::mem::transmute_copy(&dwstartupflags), ::core::mem::transmute(&pwzhostconfigfile)).into())
        }
        unsafe extern "system" fn GetDefaultStartupFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstartupflags: *mut u32, pwzhostconfigfile: ::windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultStartupFlags(this, ::core::mem::transmute_copy(&pdwstartupflags), ::core::mem::transmute_copy(&pwzhostconfigfile), ::core::mem::transmute_copy(&pcchhostconfigfile)).into())
        }
        unsafe extern "system" fn BindAsLegacyV2Runtime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindAsLegacyV2Runtime(this).into())
        }
        unsafe extern "system" fn IsStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRRuntimeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsStarted(this, ::core::mem::transmute_copy(&pbstarted), ::core::mem::transmute_copy(&pdwstartupflags)).into())
        }
        ICLRRuntimeInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVersionString: GetVersionString::<Identity, Impl, OFFSET>,
            GetRuntimeDirectory: GetRuntimeDirectory::<Identity, Impl, OFFSET>,
            IsLoaded: IsLoaded::<Identity, Impl, OFFSET>,
            LoadErrorString: LoadErrorString::<Identity, Impl, OFFSET>,
            LoadLibraryA: LoadLibraryA::<Identity, Impl, OFFSET>,
            GetProcAddress: GetProcAddress::<Identity, Impl, OFFSET>,
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            IsLoadable: IsLoadable::<Identity, Impl, OFFSET>,
            SetDefaultStartupFlags: SetDefaultStartupFlags::<Identity, Impl, OFFSET>,
            GetDefaultStartupFlags: GetDefaultStartupFlags::<Identity, Impl, OFFSET>,
            BindAsLegacyV2Runtime: BindAsLegacyV2Runtime::<Identity, Impl, OFFSET>,
            IsStarted: IsStarted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRStrongName_Impl: ::windows_core::BaseImpl {
    fn GetHashFromAssemblyFile(this: &Self::This, pszfilepath: &::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromAssemblyFileW(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromBlob(this: &Self::This, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromFile(this: &Self::This, pszfilepath: &::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromFileW(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn GetHashFromHandle(this: &Self::This, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameCompareAssemblies(this: &Self::This, pwzassembly1: &::windows_core::PCWSTR, pwzassembly2: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn StrongNameFreeBuffer(this: &Self::This, pbmemory: *const u8) -> ::windows_core::Result<()>;
    fn StrongNameGetBlob(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameGetBlobFromImage(this: &Self::This, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameGetPublicKey(this: &Self::This, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameHashSize(this: &Self::This, ulhashalg: u32) -> ::windows_core::Result<u32>;
    fn StrongNameKeyDelete(this: &Self::This, pwzkeycontainer: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StrongNameKeyGen(this: &Self::This, pwzkeycontainer: &::windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameKeyGenEx(this: &Self::This, pwzkeycontainer: &::windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameKeyInstall(this: &Self::This, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureGeneration(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureGenerationEx(this: &Self::This, wszfilepath: &::windows_core::PCWSTR, wszkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureSize(this: &Self::This, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureVerification(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, dwinflags: u32) -> ::windows_core::Result<u32>;
    fn StrongNameSignatureVerificationEx(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN) -> ::windows_core::Result<u8>;
    fn StrongNameSignatureVerificationFromImage(this: &Self::This, pbbase: *const u8, dwlength: u32, dwinflags: u32) -> ::windows_core::Result<u32>;
    fn StrongNameTokenFromAssembly(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameTokenFromAssemblyEx(this: &Self::This, pwzfilepath: &::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::Result<()>;
    fn StrongNameTokenFromPublicKey(this: &Self::This, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRStrongName {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRStrongName {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHashFromAssemblyFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHashFromAssemblyFile(this, ::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into())
        }
        unsafe extern "system" fn GetHashFromAssemblyFileW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHashFromAssemblyFileW(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into())
        }
        unsafe extern "system" fn GetHashFromBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHashFromBlob(this, ::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&cchblob), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into())
        }
        unsafe extern "system" fn GetHashFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHashFromFile(this, ::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into())
        }
        unsafe extern "system" fn GetHashFromFileW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHashFromFileW(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into())
        }
        unsafe extern "system" fn GetHashFromHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHashFromHandle(this, ::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&pihashalg), ::core::mem::transmute_copy(&pbhash), ::core::mem::transmute_copy(&cchhash), ::core::mem::transmute_copy(&pchhash)).into())
        }
        unsafe extern "system" fn StrongNameCompareAssemblies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzassembly1: ::windows_core::PCWSTR, pwzassembly2: ::windows_core::PCWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrongNameCompareAssemblies(this, ::core::mem::transmute(&pwzassembly1), ::core::mem::transmute(&pwzassembly2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StrongNameFreeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmemory: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameFreeBuffer(this, ::core::mem::transmute_copy(&pbmemory)).into())
        }
        unsafe extern "system" fn StrongNameGetBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameGetBlob(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&pcbblob)).into())
        }
        unsafe extern "system" fn StrongNameGetBlobFromImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameGetBlobFromImage(this, ::core::mem::transmute_copy(&pbbase), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&pbblob), ::core::mem::transmute_copy(&pcbblob)).into())
        }
        unsafe extern "system" fn StrongNameGetPublicKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameGetPublicKey(this, ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob)).into())
        }
        unsafe extern "system" fn StrongNameHashSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulhashalg: u32, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrongNameHashSize(this, ::core::mem::transmute_copy(&ulhashalg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StrongNameKeyDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameKeyDelete(this, ::core::mem::transmute(&pwzkeycontainer)).into())
        }
        unsafe extern "system" fn StrongNameKeyGen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameKeyGen(this, ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppbkeyblob), ::core::mem::transmute_copy(&pcbkeyblob)).into())
        }
        unsafe extern "system" fn StrongNameKeyGenEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameKeyGenEx(this, ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwkeysize), ::core::mem::transmute_copy(&ppbkeyblob), ::core::mem::transmute_copy(&pcbkeyblob)).into())
        }
        unsafe extern "system" fn StrongNameKeyInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameKeyInstall(this, ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob)).into())
        }
        unsafe extern "system" fn StrongNameSignatureGeneration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameSignatureGeneration(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob)).into())
        }
        unsafe extern "system" fn StrongNameSignatureGenerationEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, wszkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameSignatureGenerationEx(this, ::core::mem::transmute(&wszfilepath), ::core::mem::transmute(&wszkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn StrongNameSignatureSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameSignatureSize(this, ::core::mem::transmute_copy(&pbpublickeyblob), ::core::mem::transmute_copy(&cbpublickeyblob), ::core::mem::transmute_copy(&pcbsize)).into())
        }
        unsafe extern "system" fn StrongNameSignatureVerification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrongNameSignatureVerification(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&dwinflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoutflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pfwasverified: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrongNameSignatureVerificationEx(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&fforceverification)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfwasverified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StrongNameSignatureVerificationFromImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbase: *const u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrongNameSignatureVerificationFromImage(this, ::core::mem::transmute_copy(&pbbase), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&dwinflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoutflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StrongNameTokenFromAssembly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameTokenFromAssembly(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken)).into())
        }
        unsafe extern "system" fn StrongNameTokenFromAssemblyEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfilepath: ::windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameTokenFromAssemblyEx(this, ::core::mem::transmute(&pwzfilepath), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob)).into())
        }
        unsafe extern "system" fn StrongNameTokenFromPublicKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameTokenFromPublicKey(this, ::core::mem::transmute_copy(&pbpublickeyblob), ::core::mem::transmute_copy(&cbpublickeyblob), ::core::mem::transmute_copy(&ppbstrongnametoken), ::core::mem::transmute_copy(&pcbstrongnametoken)).into())
        }
        ICLRStrongName_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHashFromAssemblyFile: GetHashFromAssemblyFile::<Identity, Impl, OFFSET>,
            GetHashFromAssemblyFileW: GetHashFromAssemblyFileW::<Identity, Impl, OFFSET>,
            GetHashFromBlob: GetHashFromBlob::<Identity, Impl, OFFSET>,
            GetHashFromFile: GetHashFromFile::<Identity, Impl, OFFSET>,
            GetHashFromFileW: GetHashFromFileW::<Identity, Impl, OFFSET>,
            GetHashFromHandle: GetHashFromHandle::<Identity, Impl, OFFSET>,
            StrongNameCompareAssemblies: StrongNameCompareAssemblies::<Identity, Impl, OFFSET>,
            StrongNameFreeBuffer: StrongNameFreeBuffer::<Identity, Impl, OFFSET>,
            StrongNameGetBlob: StrongNameGetBlob::<Identity, Impl, OFFSET>,
            StrongNameGetBlobFromImage: StrongNameGetBlobFromImage::<Identity, Impl, OFFSET>,
            StrongNameGetPublicKey: StrongNameGetPublicKey::<Identity, Impl, OFFSET>,
            StrongNameHashSize: StrongNameHashSize::<Identity, Impl, OFFSET>,
            StrongNameKeyDelete: StrongNameKeyDelete::<Identity, Impl, OFFSET>,
            StrongNameKeyGen: StrongNameKeyGen::<Identity, Impl, OFFSET>,
            StrongNameKeyGenEx: StrongNameKeyGenEx::<Identity, Impl, OFFSET>,
            StrongNameKeyInstall: StrongNameKeyInstall::<Identity, Impl, OFFSET>,
            StrongNameSignatureGeneration: StrongNameSignatureGeneration::<Identity, Impl, OFFSET>,
            StrongNameSignatureGenerationEx: StrongNameSignatureGenerationEx::<Identity, Impl, OFFSET>,
            StrongNameSignatureSize: StrongNameSignatureSize::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerification: StrongNameSignatureVerification::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerificationEx: StrongNameSignatureVerificationEx::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerificationFromImage: StrongNameSignatureVerificationFromImage::<Identity, Impl, OFFSET>,
            StrongNameTokenFromAssembly: StrongNameTokenFromAssembly::<Identity, Impl, OFFSET>,
            StrongNameTokenFromAssemblyEx: StrongNameTokenFromAssemblyEx::<Identity, Impl, OFFSET>,
            StrongNameTokenFromPublicKey: StrongNameTokenFromPublicKey::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRStrongName2_Impl: ::windows_core::BaseImpl {
    fn StrongNameGetPublicKeyEx(this: &Self::This, pwzkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows_core::Result<()>;
    fn StrongNameSignatureVerificationEx2(this: &Self::This, wszfilepath: &::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *const u8, cbecmapublickey: u32) -> ::windows_core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRStrongName2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRStrongName2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StrongNameGetPublicKeyEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameGetPublicKeyEx(this, ::core::mem::transmute(&pwzkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&ppbpublickeyblob), ::core::mem::transmute_copy(&pcbpublickeyblob), ::core::mem::transmute_copy(&uhashalgid), ::core::mem::transmute_copy(&ureserved)).into())
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *const u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrongNameSignatureVerificationEx2(this, ::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&fforceverification), ::core::mem::transmute_copy(&pbecmapublickey), ::core::mem::transmute_copy(&cbecmapublickey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfwasverified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICLRStrongName2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StrongNameGetPublicKeyEx: StrongNameGetPublicKeyEx::<Identity, Impl, OFFSET>,
            StrongNameSignatureVerificationEx2: StrongNameSignatureVerificationEx2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRStrongName3_Impl: ::windows_core::BaseImpl {
    fn StrongNameDigestGenerate(this: &Self::This, wszfilepath: &::windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn StrongNameDigestSign(this: &Self::This, wszkeycontainer: &::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn StrongNameDigestEmbed(this: &Self::This, wszfilepath: &::windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRStrongName3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRStrongName3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StrongNameDigestGenerate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameDigestGenerate(this, ::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&ppbdigestblob), ::core::mem::transmute_copy(&pcbdigestblob), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn StrongNameDigestSign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszkeycontainer: ::windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameDigestSign(this, ::core::mem::transmute(&wszkeycontainer), ::core::mem::transmute_copy(&pbkeyblob), ::core::mem::transmute_copy(&cbkeyblob), ::core::mem::transmute_copy(&pbdigestblob), ::core::mem::transmute_copy(&cbdigestblob), ::core::mem::transmute_copy(&hashalgid), ::core::mem::transmute_copy(&ppbsignatureblob), ::core::mem::transmute_copy(&pcbsignatureblob), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn StrongNameDigestEmbed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRStrongName3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfilepath: ::windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StrongNameDigestEmbed(this, ::core::mem::transmute(&wszfilepath), ::core::mem::transmute_copy(&pbsignatureblob), ::core::mem::transmute_copy(&cbsignatureblob)).into())
        }
        ICLRStrongName3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StrongNameDigestGenerate: StrongNameDigestGenerate::<Identity, Impl, OFFSET>,
            StrongNameDigestSign: StrongNameDigestSign::<Identity, Impl, OFFSET>,
            StrongNameDigestEmbed: StrongNameDigestEmbed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRSyncManager_Impl: ::windows_core::BaseImpl {
    fn GetMonitorOwner(this: &Self::This, cookie: usize) -> ::windows_core::Result<IHostTask>;
    fn CreateRWLockOwnerIterator(this: &Self::This, cookie: usize) -> ::windows_core::Result<usize>;
    fn GetRWLockOwnerNext(this: &Self::This, iterator: usize) -> ::windows_core::Result<IHostTask>;
    fn DeleteRWLockOwnerIterator(this: &Self::This, iterator: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICLRSyncManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRSyncManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMonitorOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMonitorOwner(this, ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownerhosttask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRWLockOwnerIterator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: usize, piterator: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRWLockOwnerIterator(this, ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRWLockOwnerNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iterator: usize, ppownerhosttask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRWLockOwnerNext(this, ::core::mem::transmute_copy(&iterator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppownerhosttask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteRWLockOwnerIterator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iterator: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRWLockOwnerIterator(this, ::core::mem::transmute_copy(&iterator)).into())
        }
        ICLRSyncManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMonitorOwner: GetMonitorOwner::<Identity, Impl, OFFSET>,
            CreateRWLockOwnerIterator: CreateRWLockOwnerIterator::<Identity, Impl, OFFSET>,
            GetRWLockOwnerNext: GetRWLockOwnerNext::<Identity, Impl, OFFSET>,
            DeleteRWLockOwnerIterator: DeleteRWLockOwnerIterator::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRTask_Impl: ::windows_core::BaseImpl {
    fn SwitchIn(this: &Self::This, threadhandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SwitchOut(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMemStats(this: &Self::This) -> ::windows_core::Result<COR_GC_THREAD_STATS>;
    fn Reset(this: &Self::This, ffull: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ExitTask(this: &Self::This) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn RudeAbort(this: &Self::This) -> ::windows_core::Result<()>;
    fn NeedsPriorityScheduling(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn YieldTask(this: &Self::This) -> ::windows_core::Result<()>;
    fn LocksHeld(this: &Self::This) -> ::windows_core::Result<usize>;
    fn SetTaskIdentifier(this: &Self::This, asked: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRTask {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRTask {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SwitchIn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchIn(this, ::core::mem::transmute_copy(&threadhandle)).into())
        }
        unsafe extern "system" fn SwitchOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchOut(this).into())
        }
        unsafe extern "system" fn GetMemStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memusage: *mut COR_GC_THREAD_STATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMemStats(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(memusage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffull: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this, ::core::mem::transmute_copy(&ffull)).into())
        }
        unsafe extern "system" fn ExitTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExitTask(this).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn RudeAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RudeAbort(this).into())
        }
        unsafe extern "system" fn NeedsPriorityScheduling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NeedsPriorityScheduling(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbneedspriorityscheduling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn YieldTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::YieldTask(this).into())
        }
        unsafe extern "system" fn LocksHeld<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plockcount: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocksHeld(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plockcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTaskIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, asked: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTaskIdentifier(this, ::core::mem::transmute_copy(&asked)).into())
        }
        ICLRTask_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SwitchIn: SwitchIn::<Identity, Impl, OFFSET>,
            SwitchOut: SwitchOut::<Identity, Impl, OFFSET>,
            GetMemStats: GetMemStats::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ExitTask: ExitTask::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            RudeAbort: RudeAbort::<Identity, Impl, OFFSET>,
            NeedsPriorityScheduling: NeedsPriorityScheduling::<Identity, Impl, OFFSET>,
            YieldTask: YieldTask::<Identity, Impl, OFFSET>,
            LocksHeld: LocksHeld::<Identity, Impl, OFFSET>,
            SetTaskIdentifier: SetTaskIdentifier::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICLRTask2_Impl: ::windows_core::BaseImpl + ICLRTask_Impl {
    fn BeginPreventAsyncAbort(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndPreventAsyncAbort(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICLRTask2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICLRTask);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRTask2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginPreventAsyncAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginPreventAsyncAbort(this).into())
        }
        unsafe extern "system" fn EndPreventAsyncAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTask2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndPreventAsyncAbort(this).into())
        }
        ICLRTask2_Vtbl {
            base__: <ICLRTask as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginPreventAsyncAbort: BeginPreventAsyncAbort::<Identity, Impl, OFFSET>,
            EndPreventAsyncAbort: EndPreventAsyncAbort::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICLRTaskManager_Impl: ::windows_core::BaseImpl {
    fn CreateTask(this: &Self::This) -> ::windows_core::Result<ICLRTask>;
    fn GetCurrentTask(this: &Self::This) -> ::windows_core::Result<ICLRTask>;
    fn SetUILocale(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
    fn SetLocale(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
    fn GetCurrentTaskType(this: &Self::This) -> ::windows_core::Result<ETaskType>;
}
impl ::windows_core::Iids for ICLRTaskManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICLRTaskManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentTask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUILocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUILocale(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocale(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn GetCurrentTaskType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICLRTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptasktype: *mut ETaskType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentTaskType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptasktype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICLRTaskManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            GetCurrentTask: GetCurrentTask::<Identity, Impl, OFFSET>,
            SetUILocale: SetUILocale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            GetCurrentTaskType: GetCurrentTaskType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICatalogServices_Impl: ::windows_core::BaseImpl {
    fn Autodone(this: &Self::This) -> ::windows_core::Result<()>;
    fn NotAutodone(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICatalogServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICatalogServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Autodone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Autodone(this).into())
        }
        unsafe extern "system" fn NotAutodone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICatalogServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotAutodone(this).into())
        }
        ICatalogServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Autodone: Autodone::<Identity, Impl, OFFSET>,
            NotAutodone: NotAutodone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICorConfiguration_Impl: ::windows_core::BaseImpl {
    fn SetGCThreadControl(this: &Self::This, pgcthreadcontrol: ::core::option::Option<&IGCThreadControl>) -> ::windows_core::Result<()>;
    fn SetGCHostControl(this: &Self::This, pgchostcontrol: ::core::option::Option<&IGCHostControl>) -> ::windows_core::Result<()>;
    fn SetDebuggerThreadControl(this: &Self::This, pdebuggerthreadcontrol: ::core::option::Option<&IDebuggerThreadControl>) -> ::windows_core::Result<()>;
    fn AddDebuggerSpecialThread(this: &Self::This, dwspecialthreadid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICorConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGCThreadControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgcthreadcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGCThreadControl(this, ::windows_core::from_raw_borrowed(&pgcthreadcontrol)).into())
        }
        unsafe extern "system" fn SetGCHostControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgchostcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGCHostControl(this, ::windows_core::from_raw_borrowed(&pgchostcontrol)).into())
        }
        unsafe extern "system" fn SetDebuggerThreadControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdebuggerthreadcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebuggerThreadControl(this, ::windows_core::from_raw_borrowed(&pdebuggerthreadcontrol)).into())
        }
        unsafe extern "system" fn AddDebuggerSpecialThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwspecialthreadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDebuggerSpecialThread(this, ::core::mem::transmute_copy(&dwspecialthreadid)).into())
        }
        ICorConfiguration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGCThreadControl: SetGCThreadControl::<Identity, Impl, OFFSET>,
            SetGCHostControl: SetGCHostControl::<Identity, Impl, OFFSET>,
            SetDebuggerThreadControl: SetDebuggerThreadControl::<Identity, Impl, OFFSET>,
            AddDebuggerSpecialThread: AddDebuggerSpecialThread::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorRuntimeHost_Impl: ::windows_core::BaseImpl {
    fn CreateLogicalThreadState(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeleteLogicalThreadState(this: &Self::This) -> ::windows_core::Result<()>;
    fn SwitchInLogicalThreadState(this: &Self::This, pfibercookie: *const u32) -> ::windows_core::Result<()>;
    fn SwitchOutLogicalThreadState(this: &Self::This) -> ::windows_core::Result<*mut u32>;
    fn LocksHeldByLogicalThread(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MapFile(this: &Self::This, hfile: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetConfiguration(this: &Self::This) -> ::windows_core::Result<ICorConfiguration>;
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateDomain(this: &Self::This, pwzfriendlyname: &::windows_core::PCWSTR, pidentityarray: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetDefaultDomain(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn EnumDomains(this: &Self::This, henum: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn NextDomain(this: &Self::This, henum: *const ::core::ffi::c_void) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CloseEnum(this: &Self::This, henum: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateDomainEx(this: &Self::This, pwzfriendlyname: &::windows_core::PCWSTR, psetup: ::core::option::Option<&::windows_core::IUnknown>, pevidence: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateDomainSetup(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateEvidence(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn UnloadDomain(this: &Self::This, pappdomain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CurrentDomain(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorRuntimeHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorRuntimeHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLogicalThreadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateLogicalThreadState(this).into())
        }
        unsafe extern "system" fn DeleteLogicalThreadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteLogicalThreadState(this).into())
        }
        unsafe extern "system" fn SwitchInLogicalThreadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfibercookie: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchInLogicalThreadState(this, ::core::mem::transmute_copy(&pfibercookie)).into())
        }
        unsafe extern "system" fn SwitchOutLogicalThreadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfibercookie: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SwitchOutLogicalThreadState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfibercookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocksHeldByLogicalThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocksHeldByLogicalThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MapFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfile: super::super::Foundation::HANDLE, hmapaddress: *mut super::super::Foundation::HMODULE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapFile(this, ::core::mem::transmute_copy(&hfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hmapaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConfiguration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconfiguration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn CreateDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows_core::PCWSTR, pidentityarray: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDomain(this, ::core::mem::transmute(&pwzfriendlyname), ::windows_core::from_raw_borrowed(&pidentityarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultDomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDomains<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, henum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDomains(this, ::core::mem::transmute_copy(&henum)).into())
        }
        unsafe extern "system" fn NextDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, henum: *const ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextDomain(this, ::core::mem::transmute_copy(&henum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, henum: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseEnum(this, ::core::mem::transmute_copy(&henum)).into())
        }
        unsafe extern "system" fn CreateDomainEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzfriendlyname: ::windows_core::PCWSTR, psetup: *mut ::core::ffi::c_void, pevidence: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDomainEx(this, ::core::mem::transmute(&pwzfriendlyname), ::windows_core::from_raw_borrowed(&psetup), ::windows_core::from_raw_borrowed(&pevidence)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDomainSetup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappdomainsetup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDomainSetup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomainsetup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEvidence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevidence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEvidence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnloadDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappdomain: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnloadDomain(this, ::windows_core::from_raw_borrowed(&pappdomain)).into())
        }
        unsafe extern "system" fn CurrentDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorRuntimeHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICorRuntimeHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateLogicalThreadState: CreateLogicalThreadState::<Identity, Impl, OFFSET>,
            DeleteLogicalThreadState: DeleteLogicalThreadState::<Identity, Impl, OFFSET>,
            SwitchInLogicalThreadState: SwitchInLogicalThreadState::<Identity, Impl, OFFSET>,
            SwitchOutLogicalThreadState: SwitchOutLogicalThreadState::<Identity, Impl, OFFSET>,
            LocksHeldByLogicalThread: LocksHeldByLogicalThread::<Identity, Impl, OFFSET>,
            MapFile: MapFile::<Identity, Impl, OFFSET>,
            GetConfiguration: GetConfiguration::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            CreateDomain: CreateDomain::<Identity, Impl, OFFSET>,
            GetDefaultDomain: GetDefaultDomain::<Identity, Impl, OFFSET>,
            EnumDomains: EnumDomains::<Identity, Impl, OFFSET>,
            NextDomain: NextDomain::<Identity, Impl, OFFSET>,
            CloseEnum: CloseEnum::<Identity, Impl, OFFSET>,
            CreateDomainEx: CreateDomainEx::<Identity, Impl, OFFSET>,
            CreateDomainSetup: CreateDomainSetup::<Identity, Impl, OFFSET>,
            CreateEvidence: CreateEvidence::<Identity, Impl, OFFSET>,
            UnloadDomain: UnloadDomain::<Identity, Impl, OFFSET>,
            CurrentDomain: CurrentDomain::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`, `\"Win32_System_Threading\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
pub trait ICorThreadpool_Impl: ::windows_core::BaseImpl {
    fn CorRegisterWaitForSingleObject(this: &Self::This, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorUnregisterWait(this: &Self::This, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorQueueUserWorkItem(this: &Self::This, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorCreateTimer(this: &Self::This, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorChangeTimer(this: &Self::This, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorDeleteTimer(this: &Self::This, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorBindIoCompletionCallback(this: &Self::This, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows_core::Result<()>;
    fn CorCallOrQueueUserWorkItem(this: &Self::This, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CorSetMaxThreads(this: &Self::This, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn CorGetMaxThreads(this: &Self::This, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows_core::Result<()>;
    fn CorGetAvailableThreads(this: &Self::This, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl ::windows_core::Iids for ICorThreadpool {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorThreadpool {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CorRegisterWaitForSingleObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorRegisterWaitForSingleObject(this, ::core::mem::transmute_copy(&phnewwaitobject), ::core::mem::transmute_copy(&hwaitobject), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&executeonlyonce)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorUnregisterWait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorUnregisterWait(this, ::core::mem::transmute_copy(&hwaitobject), ::core::mem::transmute_copy(&completionevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorQueueUserWorkItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorQueueUserWorkItem(this, ::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&executeonlyonce)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorCreateTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorCreateTimer(this, ::core::mem::transmute_copy(&phnewtimer), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&parameter), ::core::mem::transmute_copy(&duetime), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorChangeTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorChangeTimer(this, ::core::mem::transmute_copy(&timer), ::core::mem::transmute_copy(&duetime), ::core::mem::transmute_copy(&period)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorDeleteTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorDeleteTimer(this, ::core::mem::transmute_copy(&timer), ::core::mem::transmute_copy(&completionevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorBindIoCompletionCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CorBindIoCompletionCallback(this, ::core::mem::transmute_copy(&filehandle), ::core::mem::transmute_copy(&callback)).into())
        }
        unsafe extern "system" fn CorCallOrQueueUserWorkItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, result: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorCallOrQueueUserWorkItem(this, ::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorSetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxworkerthreads: u32, maxiocompletionthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CorSetMaxThreads(this, ::core::mem::transmute_copy(&maxworkerthreads), ::core::mem::transmute_copy(&maxiocompletionthreads)).into())
        }
        unsafe extern "system" fn CorGetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CorGetMaxThreads(this, ::core::mem::transmute_copy(&maxworkerthreads), ::core::mem::transmute_copy(&maxiocompletionthreads)).into())
        }
        unsafe extern "system" fn CorGetAvailableThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorThreadpool_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CorGetAvailableThreads(this, ::core::mem::transmute_copy(&availableworkerthreads), ::core::mem::transmute_copy(&availableiocompletionthreads)).into())
        }
        ICorThreadpool_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CorRegisterWaitForSingleObject: CorRegisterWaitForSingleObject::<Identity, Impl, OFFSET>,
            CorUnregisterWait: CorUnregisterWait::<Identity, Impl, OFFSET>,
            CorQueueUserWorkItem: CorQueueUserWorkItem::<Identity, Impl, OFFSET>,
            CorCreateTimer: CorCreateTimer::<Identity, Impl, OFFSET>,
            CorChangeTimer: CorChangeTimer::<Identity, Impl, OFFSET>,
            CorDeleteTimer: CorDeleteTimer::<Identity, Impl, OFFSET>,
            CorBindIoCompletionCallback: CorBindIoCompletionCallback::<Identity, Impl, OFFSET>,
            CorCallOrQueueUserWorkItem: CorCallOrQueueUserWorkItem::<Identity, Impl, OFFSET>,
            CorSetMaxThreads: CorSetMaxThreads::<Identity, Impl, OFFSET>,
            CorGetMaxThreads: CorGetMaxThreads::<Identity, Impl, OFFSET>,
            CorGetAvailableThreads: CorGetAvailableThreads::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebuggerInfo_Impl: ::windows_core::BaseImpl {
    fn IsDebuggerAttached(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebuggerInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebuggerInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebuggerInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDebuggerAttached<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebuggerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDebuggerAttached(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbattached, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebuggerInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDebuggerAttached: IsDebuggerAttached::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDebuggerThreadControl_Impl: ::windows_core::BaseImpl {
    fn ThreadIsBlockingForDebugger(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReleaseAllRuntimeThreads(this: &Self::This) -> ::windows_core::Result<()>;
    fn StartBlockingForDebugger(this: &Self::This, dwunused: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebuggerThreadControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebuggerThreadControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThreadIsBlockingForDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadIsBlockingForDebugger(this).into())
        }
        unsafe extern "system" fn ReleaseAllRuntimeThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseAllRuntimeThreads(this).into())
        }
        unsafe extern "system" fn StartBlockingForDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebuggerThreadControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwunused: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartBlockingForDebugger(this, ::core::mem::transmute_copy(&dwunused)).into())
        }
        IDebuggerThreadControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ThreadIsBlockingForDebugger: ThreadIsBlockingForDebugger::<Identity, Impl, OFFSET>,
            ReleaseAllRuntimeThreads: ReleaseAllRuntimeThreads::<Identity, Impl, OFFSET>,
            StartBlockingForDebugger: StartBlockingForDebugger::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGCHost_Impl: ::windows_core::BaseImpl {
    fn SetGCStartupLimits(this: &Self::This, segmentsize: u32, maxgen0size: u32) -> ::windows_core::Result<()>;
    fn Collect(this: &Self::This, generation: i32) -> ::windows_core::Result<()>;
    fn GetStats(this: &Self::This, pstats: *mut COR_GC_STATS) -> ::windows_core::Result<()>;
    fn GetThreadStats(this: &Self::This, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows_core::Result<()>;
    fn SetVirtualMemLimit(this: &Self::This, sztmaxvirtualmemmb: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGCHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGCHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGCStartupLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGCStartupLimits(this, ::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into())
        }
        unsafe extern "system" fn Collect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generation: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Collect(this, ::core::mem::transmute_copy(&generation)).into())
        }
        unsafe extern "system" fn GetStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstats: *mut COR_GC_STATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStats(this, ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn GetThreadStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetThreadStats(this, ::core::mem::transmute_copy(&pfibercookie), ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn SetVirtualMemLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVirtualMemLimit(this, ::core::mem::transmute_copy(&sztmaxvirtualmemmb)).into())
        }
        IGCHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGCStartupLimits: SetGCStartupLimits::<Identity, Impl, OFFSET>,
            Collect: Collect::<Identity, Impl, OFFSET>,
            GetStats: GetStats::<Identity, Impl, OFFSET>,
            GetThreadStats: GetThreadStats::<Identity, Impl, OFFSET>,
            SetVirtualMemLimit: SetVirtualMemLimit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGCHost2_Impl: ::windows_core::BaseImpl + IGCHost_Impl {
    fn SetGCStartupLimitsEx(this: &Self::This, segmentsize: usize, maxgen0size: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGCHost2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IGCHost);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGCHost2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHost2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGCStartupLimitsEx(this, ::core::mem::transmute_copy(&segmentsize), ::core::mem::transmute_copy(&maxgen0size)).into())
        }
        IGCHost2_Vtbl { base__: <IGCHost as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGCHostControl_Impl: ::windows_core::BaseImpl {
    fn RequestVirtualMemLimit(this: &Self::This, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGCHostControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHostControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGCHostControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestVirtualMemLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCHostControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestVirtualMemLimit(this, ::core::mem::transmute_copy(&sztmaxvirtualmemmb), ::core::mem::transmute_copy(&psztnewmaxvirtualmemmb)).into())
        }
        IGCHostControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestVirtualMemLimit: RequestVirtualMemLimit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGCThreadControl_Impl: ::windows_core::BaseImpl {
    fn ThreadIsBlockingForSuspension(this: &Self::This) -> ::windows_core::Result<()>;
    fn SuspensionStarting(this: &Self::This) -> ::windows_core::Result<()>;
    fn SuspensionEnding(this: &Self::This, generation: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGCThreadControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGCThreadControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadIsBlockingForSuspension(this).into())
        }
        unsafe extern "system" fn SuspensionStarting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspensionStarting(this).into())
        }
        unsafe extern "system" fn SuspensionEnding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGCThreadControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspensionEnding(this, ::core::mem::transmute_copy(&generation)).into())
        }
        IGCThreadControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, Impl, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, Impl, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostAssemblyManager_Impl: ::windows_core::BaseImpl {
    fn GetNonHostStoreAssemblies(this: &Self::This) -> ::windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetAssemblyStore(this: &Self::This) -> ::windows_core::Result<IHostAssemblyStore>;
}
impl ::windows_core::Iids for IHostAssemblyManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostAssemblyManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNonHostStoreAssemblies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreferencelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNonHostStoreAssemblies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAssemblyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAssemblyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppassemblystore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAssemblyStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppassemblystore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHostAssemblyManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNonHostStoreAssemblies: GetNonHostStoreAssemblies::<Identity, Impl, OFFSET>,
            GetAssemblyStore: GetAssemblyStore::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IHostAssemblyStore_Impl: ::windows_core::BaseImpl {
    fn ProvideAssembly(this: &Self::This, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows_core::Result<()>;
    fn ProvideModule(this: &Self::This, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut ::core::option::Option<super::Com::IStream>, ppstmpdb: *mut ::core::option::Option<super::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IHostAssemblyStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostAssemblyStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProvideAssembly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProvideAssembly(this, ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&passemblyid), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&ppstmassemblyimage), ::core::mem::transmute_copy(&ppstmpdb)).into())
        }
        unsafe extern "system" fn ProvideModule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAssemblyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut *mut ::core::ffi::c_void, ppstmpdb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProvideModule(this, ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&pdwmoduleid), ::core::mem::transmute_copy(&ppstmmoduleimage), ::core::mem::transmute_copy(&ppstmpdb)).into())
        }
        IHostAssemblyStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProvideAssembly: ProvideAssembly::<Identity, Impl, OFFSET>,
            ProvideModule: ProvideModule::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostAutoEvent_Impl: ::windows_core::BaseImpl {
    fn Wait(this: &Self::This, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn Set(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostAutoEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostAutoEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostAutoEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this).into())
        }
        IHostAutoEvent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Wait: Wait::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostControl_Impl: ::windows_core::BaseImpl {
    fn GetHostManager(this: &Self::This, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetAppDomainManager(this: &Self::This, dwappdomainid: u32, punkappdomainmanager: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHostManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHostManager(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppobject)).into())
        }
        unsafe extern "system" fn SetAppDomainManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappdomainid: u32, punkappdomainmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppDomainManager(this, ::core::mem::transmute_copy(&dwappdomainid), ::windows_core::from_raw_borrowed(&punkappdomainmanager)).into())
        }
        IHostControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHostManager: GetHostManager::<Identity, Impl, OFFSET>,
            SetAppDomainManager: SetAppDomainManager::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostCrst_Impl: ::windows_core::BaseImpl {
    fn Enter(this: &Self::This, option: u32) -> ::windows_core::Result<()>;
    fn Leave(this: &Self::This) -> ::windows_core::Result<()>;
    fn TryEnter(this: &Self::This, option: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSpinCount(this: &Self::This, dwspincount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHostCrst {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostCrst {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enter(this, ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn Leave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Leave(this).into())
        }
        unsafe extern "system" fn TryEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: u32, pbsucceeded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TryEnter(this, ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsucceeded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSpinCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostCrst_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwspincount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpinCount(this, ::core::mem::transmute_copy(&dwspincount)).into())
        }
        IHostCrst_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            TryEnter: TryEnter::<Identity, Impl, OFFSET>,
            SetSpinCount: SetSpinCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostGCManager_Impl: ::windows_core::BaseImpl {
    fn ThreadIsBlockingForSuspension(this: &Self::This) -> ::windows_core::Result<()>;
    fn SuspensionStarting(this: &Self::This) -> ::windows_core::Result<()>;
    fn SuspensionEnding(this: &Self::This, generation: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostGCManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostGCManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadIsBlockingForSuspension(this).into())
        }
        unsafe extern "system" fn SuspensionStarting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspensionStarting(this).into())
        }
        unsafe extern "system" fn SuspensionEnding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostGCManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generation: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspensionEnding(this, ::core::mem::transmute_copy(&generation)).into())
        }
        IHostGCManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, Impl, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, Impl, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostIoCompletionManager_Impl: ::windows_core::BaseImpl {
    fn CreateIoCompletionPort(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn CloseIoCompletionPort(this: &Self::This, hport: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetMaxThreads(this: &Self::This, dwmaxiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn GetMaxThreads(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAvailableThreads(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetHostOverlappedSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetCLRIoCompletionManager(this: &Self::This, pmanager: ::core::option::Option<&ICLRIoCompletionManager>) -> ::windows_core::Result<()>;
    fn InitializeHostOverlapped(this: &Self::This, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Bind(this: &Self::This, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn SetMinThreads(this: &Self::This, dwminiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn GetMinThreads(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHostIoCompletionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostIoCompletionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateIoCompletionPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phport: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateIoCompletionPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseIoCompletionPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseIoCompletionPort(this, ::core::mem::transmute_copy(&hport)).into())
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxiocompletionthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxThreads(this, ::core::mem::transmute_copy(&dwmaxiocompletionthreads)).into())
        }
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwavailableiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwavailableiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHostOverlappedSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHostOverlappedSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCLRIoCompletionManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCLRIoCompletionManager(this, ::windows_core::from_raw_borrowed(&pmanager)).into())
        }
        unsafe extern "system" fn InitializeHostOverlapped<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvoverlapped: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeHostOverlapped(this, ::core::mem::transmute_copy(&pvoverlapped)).into())
        }
        unsafe extern "system" fn Bind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bind(this, ::core::mem::transmute_copy(&hport), ::core::mem::transmute_copy(&hhandle)).into())
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinThreads(this, ::core::mem::transmute_copy(&dwminiocompletionthreads)).into())
        }
        unsafe extern "system" fn GetMinThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostIoCompletionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwminiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHostIoCompletionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateIoCompletionPort: CreateIoCompletionPort::<Identity, Impl, OFFSET>,
            CloseIoCompletionPort: CloseIoCompletionPort::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, Impl, OFFSET>,
            GetHostOverlappedSize: GetHostOverlappedSize::<Identity, Impl, OFFSET>,
            SetCLRIoCompletionManager: SetCLRIoCompletionManager::<Identity, Impl, OFFSET>,
            InitializeHostOverlapped: InitializeHostOverlapped::<Identity, Impl, OFFSET>,
            Bind: Bind::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostMalloc_Impl: ::windows_core::BaseImpl {
    fn Alloc(this: &Self::This, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DebugAlloc(this: &Self::This, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Free(this: &Self::This, pmem: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostMalloc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostMalloc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Alloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Alloc(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&ppmem)).into())
        }
        unsafe extern "system" fn DebugAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DebugAlloc(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&ilineno), ::core::mem::transmute_copy(&ppmem)).into())
        }
        unsafe extern "system" fn Free<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Free(this, ::core::mem::transmute_copy(&pmem)).into())
        }
        IHostMalloc_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Alloc: Alloc::<Identity, Impl, OFFSET>,
            DebugAlloc: DebugAlloc::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostManualEvent_Impl: ::windows_core::BaseImpl {
    fn Wait(this: &Self::This, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Set(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostManualEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostManualEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostManualEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this).into())
        }
        IHostManualEvent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Wait: Wait::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostMemoryManager_Impl: ::windows_core::BaseImpl {
    fn CreateMalloc(this: &Self::This, dwmalloctype: u32) -> ::windows_core::Result<IHostMalloc>;
    fn VirtualAlloc(this: &Self::This, paddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn VirtualFree(this: &Self::This, lpaddress: *const ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows_core::Result<()>;
    fn VirtualQuery(this: &Self::This, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows_core::Result<()>;
    fn VirtualProtect(this: &Self::This, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: u32) -> ::windows_core::Result<u32>;
    fn GetMemoryLoad(this: &Self::This, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows_core::Result<()>;
    fn RegisterMemoryNotificationCallback(this: &Self::This, pcallback: ::core::option::Option<&ICLRMemoryNotificationCallback>) -> ::windows_core::Result<()>;
    fn NeedsVirtualAddressSpace(this: &Self::This, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::Result<()>;
    fn AcquiredVirtualAddressSpace(this: &Self::This, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::Result<()>;
    fn ReleasedVirtualAddressSpace(this: &Self::This, startaddress: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostMemoryManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostMemoryManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateMalloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmalloctype: u32, ppmalloc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMalloc(this, ::core::mem::transmute_copy(&dwmalloctype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmalloc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VirtualAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VirtualAlloc(this, ::core::mem::transmute_copy(&paddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&flallocationtype), ::core::mem::transmute_copy(&flprotect), ::core::mem::transmute_copy(&ecriticallevel), ::core::mem::transmute_copy(&ppmem)).into())
        }
        unsafe extern "system" fn VirtualFree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpaddress: *const ::core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VirtualFree(this, ::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwfreetype)).into())
        }
        unsafe extern "system" fn VirtualQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize, presult: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VirtualQuery(this, ::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&presult)).into())
        }
        unsafe extern "system" fn VirtualProtect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VirtualProtect(this, ::core::mem::transmute_copy(&lpaddress), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&flnewprotect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfloldprotect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMemoryLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMemoryLoad(this, ::core::mem::transmute_copy(&pmemoryload), ::core::mem::transmute_copy(&pavailablebytes)).into())
        }
        unsafe extern "system" fn RegisterMemoryNotificationCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterMemoryNotificationCallback(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn NeedsVirtualAddressSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NeedsVirtualAddressSpace(this, ::core::mem::transmute_copy(&startaddress), ::core::mem::transmute_copy(&size)).into())
        }
        unsafe extern "system" fn AcquiredVirtualAddressSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startaddress: *const ::core::ffi::c_void, size: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquiredVirtualAddressSpace(this, ::core::mem::transmute_copy(&startaddress), ::core::mem::transmute_copy(&size)).into())
        }
        unsafe extern "system" fn ReleasedVirtualAddressSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostMemoryManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startaddress: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleasedVirtualAddressSpace(this, ::core::mem::transmute_copy(&startaddress)).into())
        }
        IHostMemoryManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateMalloc: CreateMalloc::<Identity, Impl, OFFSET>,
            VirtualAlloc: VirtualAlloc::<Identity, Impl, OFFSET>,
            VirtualFree: VirtualFree::<Identity, Impl, OFFSET>,
            VirtualQuery: VirtualQuery::<Identity, Impl, OFFSET>,
            VirtualProtect: VirtualProtect::<Identity, Impl, OFFSET>,
            GetMemoryLoad: GetMemoryLoad::<Identity, Impl, OFFSET>,
            RegisterMemoryNotificationCallback: RegisterMemoryNotificationCallback::<Identity, Impl, OFFSET>,
            NeedsVirtualAddressSpace: NeedsVirtualAddressSpace::<Identity, Impl, OFFSET>,
            AcquiredVirtualAddressSpace: AcquiredVirtualAddressSpace::<Identity, Impl, OFFSET>,
            ReleasedVirtualAddressSpace: ReleasedVirtualAddressSpace::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostPolicyManager_Impl: ::windows_core::BaseImpl {
    fn OnDefaultAction(this: &Self::This, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn OnTimeout(this: &Self::This, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::Result<()>;
    fn OnFailure(this: &Self::This, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostPolicyManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostPolicyManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDefaultAction(this, ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn OnTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTimeout(this, ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn OnFailure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostPolicyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFailure(this, ::core::mem::transmute_copy(&failure), ::core::mem::transmute_copy(&action)).into())
        }
        IHostPolicyManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDefaultAction: OnDefaultAction::<Identity, Impl, OFFSET>,
            OnTimeout: OnTimeout::<Identity, Impl, OFFSET>,
            OnFailure: OnFailure::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostSecurityContext_Impl: ::windows_core::BaseImpl {
    fn Capture(this: &Self::This) -> ::windows_core::Result<IHostSecurityContext>;
}
impl ::windows_core::Iids for IHostSecurityContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostSecurityContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Capture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclonedcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Capture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclonedcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHostSecurityContext_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Capture: Capture::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostSecurityManager_Impl: ::windows_core::BaseImpl {
    fn ImpersonateLoggedOnUser(this: &Self::This, htoken: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn RevertToSelf(this: &Self::This) -> ::windows_core::Result<()>;
    fn OpenThreadToken(this: &Self::This, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn SetThreadToken(this: &Self::This, htoken: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetSecurityContext(this: &Self::This, econtexttype: EContextType) -> ::windows_core::Result<IHostSecurityContext>;
    fn SetSecurityContext(this: &Self::This, econtexttype: EContextType, psecuritycontext: ::core::option::Option<&IHostSecurityContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHostSecurityManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostSecurityManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ImpersonateLoggedOnUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImpersonateLoggedOnUser(this, ::core::mem::transmute_copy(&htoken)).into())
        }
        unsafe extern "system" fn RevertToSelf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevertToSelf(this).into())
        }
        unsafe extern "system" fn OpenThreadToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL, phthreadtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenThreadToken(this, ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&bopenasself)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phthreadtoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetThreadToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThreadToken(this, ::core::mem::transmute_copy(&htoken)).into())
        }
        unsafe extern "system" fn GetSecurityContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, econtexttype: EContextType, ppsecuritycontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityContext(this, ::core::mem::transmute_copy(&econtexttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecuritycontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSecurityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, econtexttype: EContextType, psecuritycontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityContext(this, ::core::mem::transmute_copy(&econtexttype), ::windows_core::from_raw_borrowed(&psecuritycontext)).into())
        }
        IHostSecurityManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ImpersonateLoggedOnUser: ImpersonateLoggedOnUser::<Identity, Impl, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, Impl, OFFSET>,
            OpenThreadToken: OpenThreadToken::<Identity, Impl, OFFSET>,
            SetThreadToken: SetThreadToken::<Identity, Impl, OFFSET>,
            GetSecurityContext: GetSecurityContext::<Identity, Impl, OFFSET>,
            SetSecurityContext: SetSecurityContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostSemaphore_Impl: ::windows_core::BaseImpl {
    fn Wait(this: &Self::This, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn ReleaseSemaphore(this: &Self::This, lreleasecount: i32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IHostSemaphore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostSemaphore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn ReleaseSemaphore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSemaphore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lreleasecount: i32, lppreviouscount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReleaseSemaphore(this, ::core::mem::transmute_copy(&lreleasecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppreviouscount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHostSemaphore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Wait: Wait::<Identity, Impl, OFFSET>,
            ReleaseSemaphore: ReleaseSemaphore::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHostSyncManager_Impl: ::windows_core::BaseImpl {
    fn SetCLRSyncManager(this: &Self::This, pmanager: ::core::option::Option<&ICLRSyncManager>) -> ::windows_core::Result<()>;
    fn CreateCrst(this: &Self::This) -> ::windows_core::Result<IHostCrst>;
    fn CreateCrstWithSpinCount(this: &Self::This, dwspincount: u32) -> ::windows_core::Result<IHostCrst>;
    fn CreateAutoEvent(this: &Self::This) -> ::windows_core::Result<IHostAutoEvent>;
    fn CreateManualEvent(this: &Self::This, binitialstate: super::super::Foundation::BOOL) -> ::windows_core::Result<IHostManualEvent>;
    fn CreateMonitorEvent(this: &Self::This, cookie: usize) -> ::windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockWriterEvent(this: &Self::This, cookie: usize) -> ::windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockReaderEvent(this: &Self::This, binitialstate: super::super::Foundation::BOOL, cookie: usize) -> ::windows_core::Result<IHostManualEvent>;
    fn CreateSemaphoreA(this: &Self::This, dwinitial: u32, dwmax: u32) -> ::windows_core::Result<IHostSemaphore>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHostSyncManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostSyncManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCLRSyncManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCLRSyncManager(this, ::windows_core::from_raw_borrowed(&pmanager)).into())
        }
        unsafe extern "system" fn CreateCrst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCrst(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrst, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCrstWithSpinCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwspincount: u32, ppcrst: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCrstWithSpinCount(this, ::core::mem::transmute_copy(&dwspincount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrst, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAutoEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAutoEvent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateManualEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateManualEvent(this, ::core::mem::transmute_copy(&binitialstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMonitorEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMonitorEvent(this, ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRWLockWriterEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRWLockWriterEvent(this, ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRWLockReaderEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, cookie: usize, ppevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRWLockReaderEvent(this, ::core::mem::transmute_copy(&binitialstate), ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSemaphoreA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostSyncManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinitial: u32, dwmax: u32, ppsemaphore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSemaphoreA(this, ::core::mem::transmute_copy(&dwinitial), ::core::mem::transmute_copy(&dwmax)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsemaphore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHostSyncManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCLRSyncManager: SetCLRSyncManager::<Identity, Impl, OFFSET>,
            CreateCrst: CreateCrst::<Identity, Impl, OFFSET>,
            CreateCrstWithSpinCount: CreateCrstWithSpinCount::<Identity, Impl, OFFSET>,
            CreateAutoEvent: CreateAutoEvent::<Identity, Impl, OFFSET>,
            CreateManualEvent: CreateManualEvent::<Identity, Impl, OFFSET>,
            CreateMonitorEvent: CreateMonitorEvent::<Identity, Impl, OFFSET>,
            CreateRWLockWriterEvent: CreateRWLockWriterEvent::<Identity, Impl, OFFSET>,
            CreateRWLockReaderEvent: CreateRWLockReaderEvent::<Identity, Impl, OFFSET>,
            CreateSemaphoreA: CreateSemaphoreA::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IHostTask_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Alert(this: &Self::This) -> ::windows_core::Result<()>;
    fn Join(this: &Self::This, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn SetPriority(this: &Self::This, newpriority: i32) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetCLRTask(this: &Self::This, pclrtask: ::core::option::Option<&ICLRTask>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHostTask {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostTask {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Alert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Alert(this).into())
        }
        unsafe extern "system" fn Join<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Join(this, ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&newpriority)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCLRTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTask_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclrtask: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCLRTask(this, ::windows_core::from_raw_borrowed(&pclrtask)).into())
        }
        IHostTask_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Alert: Alert::<Identity, Impl, OFFSET>,
            Join: Join::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetCLRTask: SetCLRTask::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub trait IHostTaskManager_Impl: ::windows_core::BaseImpl {
    fn GetCurrentTask(this: &Self::This) -> ::windows_core::Result<IHostTask>;
    fn CreateTask(this: &Self::This, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const ::core::ffi::c_void) -> ::windows_core::Result<IHostTask>;
    fn Sleep(this: &Self::This, dwmilliseconds: u32, option: u32) -> ::windows_core::Result<()>;
    fn SwitchToTask(this: &Self::This, option: u32) -> ::windows_core::Result<()>;
    fn SetUILocale(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
    fn SetLocale(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
    fn CallNeedsHostHook(this: &Self::This, target: usize) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn LeaveRuntime(this: &Self::This, target: usize) -> ::windows_core::Result<()>;
    fn EnterRuntime(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReverseLeaveRuntime(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReverseEnterRuntime(this: &Self::This) -> ::windows_core::Result<()>;
    fn BeginDelayAbort(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndDelayAbort(this: &Self::This) -> ::windows_core::Result<()>;
    fn BeginThreadAffinity(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndThreadAffinity(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetStackGuarantee(this: &Self::This, guarantee: u32) -> ::windows_core::Result<()>;
    fn GetStackGuarantee(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetCLRTaskManager(this: &Self::This, ppmanager: ::core::option::Option<&ICLRTaskManager>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::windows_core::Iids for IHostTaskManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostTaskManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentTask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const ::core::ffi::c_void, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTask(this, ::core::mem::transmute_copy(&dwstacksize), ::core::mem::transmute_copy(&pstartaddress), ::core::mem::transmute_copy(&pparameter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Sleep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32, option: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Sleep(this, ::core::mem::transmute_copy(&dwmilliseconds), ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn SwitchToTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchToTask(this, ::core::mem::transmute_copy(&option)).into())
        }
        unsafe extern "system" fn SetUILocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUILocale(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocale(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn CallNeedsHostHook<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: usize, pbcallneedshosthook: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallNeedsHostHook(this, ::core::mem::transmute_copy(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcallneedshosthook, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LeaveRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LeaveRuntime(this, ::core::mem::transmute_copy(&target)).into())
        }
        unsafe extern "system" fn EnterRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnterRuntime(this).into())
        }
        unsafe extern "system" fn ReverseLeaveRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReverseLeaveRuntime(this).into())
        }
        unsafe extern "system" fn ReverseEnterRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReverseEnterRuntime(this).into())
        }
        unsafe extern "system" fn BeginDelayAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDelayAbort(this).into())
        }
        unsafe extern "system" fn EndDelayAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDelayAbort(this).into())
        }
        unsafe extern "system" fn BeginThreadAffinity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginThreadAffinity(this).into())
        }
        unsafe extern "system" fn EndThreadAffinity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndThreadAffinity(this).into())
        }
        unsafe extern "system" fn SetStackGuarantee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guarantee: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStackGuarantee(this, ::core::mem::transmute_copy(&guarantee)).into())
        }
        unsafe extern "system" fn GetStackGuarantee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguarantee: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStackGuarantee(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguarantee, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCLRTaskManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostTaskManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCLRTaskManager(this, ::windows_core::from_raw_borrowed(&ppmanager)).into())
        }
        IHostTaskManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentTask: GetCurrentTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            Sleep: Sleep::<Identity, Impl, OFFSET>,
            SwitchToTask: SwitchToTask::<Identity, Impl, OFFSET>,
            SetUILocale: SetUILocale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            CallNeedsHostHook: CallNeedsHostHook::<Identity, Impl, OFFSET>,
            LeaveRuntime: LeaveRuntime::<Identity, Impl, OFFSET>,
            EnterRuntime: EnterRuntime::<Identity, Impl, OFFSET>,
            ReverseLeaveRuntime: ReverseLeaveRuntime::<Identity, Impl, OFFSET>,
            ReverseEnterRuntime: ReverseEnterRuntime::<Identity, Impl, OFFSET>,
            BeginDelayAbort: BeginDelayAbort::<Identity, Impl, OFFSET>,
            EndDelayAbort: EndDelayAbort::<Identity, Impl, OFFSET>,
            BeginThreadAffinity: BeginThreadAffinity::<Identity, Impl, OFFSET>,
            EndThreadAffinity: EndThreadAffinity::<Identity, Impl, OFFSET>,
            SetStackGuarantee: SetStackGuarantee::<Identity, Impl, OFFSET>,
            GetStackGuarantee: GetStackGuarantee::<Identity, Impl, OFFSET>,
            SetCLRTaskManager: SetCLRTaskManager::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Threading\"`"]
#[cfg(feature = "Win32_System_Threading")]
pub trait IHostThreadpoolManager_Impl: ::windows_core::BaseImpl {
    fn QueueUserWorkItem(this: &Self::This, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: u32) -> ::windows_core::Result<()>;
    fn SetMaxThreads(this: &Self::This, dwmaxworkerthreads: u32) -> ::windows_core::Result<()>;
    fn GetMaxThreads(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAvailableThreads(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMinThreads(this: &Self::This, dwminiocompletionthreads: u32) -> ::windows_core::Result<()>;
    fn GetMinThreads(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Threading")]
impl ::windows_core::Iids for IHostThreadpoolManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Threading")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHostThreadpoolManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueueUserWorkItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueueUserWorkItem(this, ::core::mem::transmute_copy(&function), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmaxworkerthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxThreads(this, ::core::mem::transmute_copy(&dwmaxworkerthreads)).into())
        }
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmaxworkerthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxworkerthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwavailableworkerthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwavailableworkerthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwminiocompletionthreads: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinThreads(this, ::core::mem::transmute_copy(&dwminiocompletionthreads)).into())
        }
        unsafe extern "system" fn GetMinThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHostThreadpoolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwminiocompletionthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHostThreadpoolManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueueUserWorkItem: QueueUserWorkItem::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IManagedObject_Impl: ::windows_core::BaseImpl {
    fn GetSerializedBuffer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetObjectIdentity(this: &Self::This, pbstrguid: *mut ::windows_core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IManagedObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IManagedObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSerializedBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSerializedBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IManagedObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, appdomainid: *mut i32, pccw: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectIdentity(this, ::core::mem::transmute_copy(&pbstrguid), ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&pccw)).into())
        }
        IManagedObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSerializedBuffer: GetSerializedBuffer::<Identity, Impl, OFFSET>,
            GetObjectIdentity: GetObjectIdentity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectHandle_Impl: ::windows_core::BaseImpl {
    fn Unwrap(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IObjectHandle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectHandle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectHandle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Unwrap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectHandle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppv: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Unwrap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IObjectHandle_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Unwrap: Unwrap::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITypeName_Impl: ::windows_core::BaseImpl {
    fn GetNameCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetNames(this: &Self::This, count: u32, rgbsznames: *mut ::windows_core::BSTR, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetTypeArgumentCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetTypeArguments(this: &Self::This, count: u32, rgparguments: *mut ::core::option::Option<ITypeName>, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetModifierLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetModifiers(this: &Self::This, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetAssemblyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITypeName {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeName {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNameCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNameCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, rgbsznames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNames(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgbsznames), ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn GetTypeArgumentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeArgumentCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, rgparguments: *mut *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTypeArguments(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgparguments), ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn GetModifierLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetModifierLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetModifiers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetModifiers(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&rgmodifiers), ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn GetAssemblyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgbszassemblynames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAssemblyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgbszassemblynames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITypeName_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNameCount: GetNameCount::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            GetTypeArgumentCount: GetTypeArgumentCount::<Identity, Impl, OFFSET>,
            GetTypeArguments: GetTypeArguments::<Identity, Impl, OFFSET>,
            GetModifierLength: GetModifierLength::<Identity, Impl, OFFSET>,
            GetModifiers: GetModifiers::<Identity, Impl, OFFSET>,
            GetAssemblyName: GetAssemblyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITypeNameBuilder_Impl: ::windows_core::BaseImpl {
    fn OpenGenericArguments(this: &Self::This) -> ::windows_core::Result<()>;
    fn CloseGenericArguments(this: &Self::This) -> ::windows_core::Result<()>;
    fn OpenGenericArgument(this: &Self::This) -> ::windows_core::Result<()>;
    fn CloseGenericArgument(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddName(this: &Self::This, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddPointer(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddByRef(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddSzArray(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddArray(this: &Self::This, rank: u32) -> ::windows_core::Result<()>;
    fn AddAssemblySpec(this: &Self::This, szassemblyspec: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ToString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITypeNameBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeNameBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenGenericArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenGenericArguments(this).into())
        }
        unsafe extern "system" fn CloseGenericArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseGenericArguments(this).into())
        }
        unsafe extern "system" fn OpenGenericArgument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenGenericArgument(this).into())
        }
        unsafe extern "system" fn CloseGenericArgument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseGenericArgument(this).into())
        }
        unsafe extern "system" fn AddName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddName(this, ::core::mem::transmute(&szname)).into())
        }
        unsafe extern "system" fn AddPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPointer(this).into())
        }
        unsafe extern "system" fn AddByRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddByRef(this).into())
        }
        unsafe extern "system" fn AddSzArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSzArray(this).into())
        }
        unsafe extern "system" fn AddArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rank: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddArray(this, ::core::mem::transmute_copy(&rank)).into())
        }
        unsafe extern "system" fn AddAssemblySpec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szassemblyspec: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAssemblySpec(this, ::core::mem::transmute(&szassemblyspec)).into())
        }
        unsafe extern "system" fn ToString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstringrepresentation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszstringrepresentation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        ITypeNameBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenGenericArguments: OpenGenericArguments::<Identity, Impl, OFFSET>,
            CloseGenericArguments: CloseGenericArguments::<Identity, Impl, OFFSET>,
            OpenGenericArgument: OpenGenericArgument::<Identity, Impl, OFFSET>,
            CloseGenericArgument: CloseGenericArgument::<Identity, Impl, OFFSET>,
            AddName: AddName::<Identity, Impl, OFFSET>,
            AddPointer: AddPointer::<Identity, Impl, OFFSET>,
            AddByRef: AddByRef::<Identity, Impl, OFFSET>,
            AddSzArray: AddSzArray::<Identity, Impl, OFFSET>,
            AddArray: AddArray::<Identity, Impl, OFFSET>,
            AddAssemblySpec: AddAssemblySpec::<Identity, Impl, OFFSET>,
            ToString: ToString::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITypeNameFactory_Impl: ::windows_core::BaseImpl {
    fn ParseTypeName(this: &Self::This, szname: &::windows_core::PCWSTR, perror: *mut u32, pptypename: *mut ::core::option::Option<ITypeName>) -> ::windows_core::Result<()>;
    fn GetTypeNameBuilder(this: &Self::This) -> ::windows_core::Result<ITypeNameBuilder>;
}
impl ::windows_core::Iids for ITypeNameFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeNameFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, perror: *mut u32, pptypename: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseTypeName(this, ::core::mem::transmute(&szname), ::core::mem::transmute_copy(&perror), ::core::mem::transmute_copy(&pptypename)).into())
        }
        unsafe extern "system" fn GetTypeNameBuilder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeNameFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptypebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeNameBuilder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypebuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITypeNameFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseTypeName: ParseTypeName::<Identity, Impl, OFFSET>,
            GetTypeNameBuilder: GetTypeNameBuilder::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
