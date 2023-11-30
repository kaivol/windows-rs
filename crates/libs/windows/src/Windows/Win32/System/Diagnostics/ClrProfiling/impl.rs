#[doc = "Required features: `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerAssemblyReferenceProvider_Impl: ::windows_core::BaseImpl {
    fn AddAssemblyReference(this: &Self::This, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ::windows_core::Iids for ICorProfilerAssemblyReferenceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerAssemblyReferenceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerAssemblyReferenceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAssemblyReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerAssemblyReferenceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAssemblyReference(this, ::core::mem::transmute_copy(&passemblyrefinfo)).into())
        }
        ICorProfilerAssemblyReferenceProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAssemblyReference: AddAssemblyReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, picorprofilerinfounk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
    fn AppDomainCreationStarted(this: &Self::This, appdomainid: usize) -> ::windows_core::Result<()>;
    fn AppDomainCreationFinished(this: &Self::This, appdomainid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn AppDomainShutdownStarted(this: &Self::This, appdomainid: usize) -> ::windows_core::Result<()>;
    fn AppDomainShutdownFinished(this: &Self::This, appdomainid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn AssemblyLoadStarted(this: &Self::This, assemblyid: usize) -> ::windows_core::Result<()>;
    fn AssemblyLoadFinished(this: &Self::This, assemblyid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn AssemblyUnloadStarted(this: &Self::This, assemblyid: usize) -> ::windows_core::Result<()>;
    fn AssemblyUnloadFinished(this: &Self::This, assemblyid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ModuleLoadStarted(this: &Self::This, moduleid: usize) -> ::windows_core::Result<()>;
    fn ModuleLoadFinished(this: &Self::This, moduleid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ModuleUnloadStarted(this: &Self::This, moduleid: usize) -> ::windows_core::Result<()>;
    fn ModuleUnloadFinished(this: &Self::This, moduleid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ModuleAttachedToAssembly(this: &Self::This, moduleid: usize, assemblyid: usize) -> ::windows_core::Result<()>;
    fn ClassLoadStarted(this: &Self::This, classid: usize) -> ::windows_core::Result<()>;
    fn ClassLoadFinished(this: &Self::This, classid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ClassUnloadStarted(this: &Self::This, classid: usize) -> ::windows_core::Result<()>;
    fn ClassUnloadFinished(this: &Self::This, classid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn FunctionUnloadStarted(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn JITCompilationStarted(this: &Self::This, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn JITCompilationFinished(this: &Self::This, functionid: usize, hrstatus: ::windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn JITCachedFunctionSearchStarted(this: &Self::This, functionid: usize) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn JITCachedFunctionSearchFinished(this: &Self::This, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows_core::Result<()>;
    fn JITFunctionPitched(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn JITInlining(this: &Self::This, callerid: usize, calleeid: usize) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn ThreadCreated(this: &Self::This, threadid: usize) -> ::windows_core::Result<()>;
    fn ThreadDestroyed(this: &Self::This, threadid: usize) -> ::windows_core::Result<()>;
    fn ThreadAssignedToOSThread(this: &Self::This, managedthreadid: usize, osthreadid: u32) -> ::windows_core::Result<()>;
    fn RemotingClientInvocationStarted(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemotingClientSendingMessage(this: &Self::This, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RemotingClientReceivingReply(this: &Self::This, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RemotingClientInvocationFinished(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemotingServerReceivingMessage(this: &Self::This, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RemotingServerInvocationStarted(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemotingServerInvocationReturned(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemotingServerSendingReply(this: &Self::This, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UnmanagedToManagedTransition(this: &Self::This, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows_core::Result<()>;
    fn ManagedToUnmanagedTransition(this: &Self::This, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows_core::Result<()>;
    fn RuntimeSuspendStarted(this: &Self::This, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows_core::Result<()>;
    fn RuntimeSuspendFinished(this: &Self::This) -> ::windows_core::Result<()>;
    fn RuntimeSuspendAborted(this: &Self::This) -> ::windows_core::Result<()>;
    fn RuntimeResumeStarted(this: &Self::This) -> ::windows_core::Result<()>;
    fn RuntimeResumeFinished(this: &Self::This) -> ::windows_core::Result<()>;
    fn RuntimeThreadSuspended(this: &Self::This, threadid: usize) -> ::windows_core::Result<()>;
    fn RuntimeThreadResumed(this: &Self::This, threadid: usize) -> ::windows_core::Result<()>;
    fn MovedReferences(this: &Self::This, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> ::windows_core::Result<()>;
    fn ObjectAllocated(this: &Self::This, objectid: usize, classid: usize) -> ::windows_core::Result<()>;
    fn ObjectsAllocatedByClass(this: &Self::This, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> ::windows_core::Result<()>;
    fn ObjectReferences(this: &Self::This, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> ::windows_core::Result<()>;
    fn RootReferences(this: &Self::This, crootrefs: u32, rootrefids: *const usize) -> ::windows_core::Result<()>;
    fn ExceptionThrown(this: &Self::This, thrownobjectid: usize) -> ::windows_core::Result<()>;
    fn ExceptionSearchFunctionEnter(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn ExceptionSearchFunctionLeave(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExceptionSearchFilterEnter(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn ExceptionSearchFilterLeave(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExceptionSearchCatcherFound(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn ExceptionOSHandlerEnter(this: &Self::This, __unused: usize) -> ::windows_core::Result<()>;
    fn ExceptionOSHandlerLeave(this: &Self::This, __unused: usize) -> ::windows_core::Result<()>;
    fn ExceptionUnwindFunctionEnter(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn ExceptionUnwindFunctionLeave(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExceptionUnwindFinallyEnter(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn ExceptionUnwindFinallyLeave(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExceptionCatcherEnter(this: &Self::This, functionid: usize, objectid: usize) -> ::windows_core::Result<()>;
    fn ExceptionCatcherLeave(this: &Self::This) -> ::windows_core::Result<()>;
    fn COMClassicVTableCreated(this: &Self::This, wrappedclassid: usize, implementediid: *const ::windows_core::GUID, pvtable: *const ::core::ffi::c_void, cslots: u32) -> ::windows_core::Result<()>;
    fn COMClassicVTableDestroyed(this: &Self::This, wrappedclassid: usize, implementediid: *const ::windows_core::GUID, pvtable: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ExceptionCLRCatcherFound(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExceptionCLRCatcherExecute(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, picorprofilerinfounk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&picorprofilerinfounk)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        unsafe extern "system" fn AppDomainCreationStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appdomainid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppDomainCreationStarted(this, ::core::mem::transmute_copy(&appdomainid)).into())
        }
        unsafe extern "system" fn AppDomainCreationFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appdomainid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppDomainCreationFinished(this, ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn AppDomainShutdownStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appdomainid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppDomainShutdownStarted(this, ::core::mem::transmute_copy(&appdomainid)).into())
        }
        unsafe extern "system" fn AppDomainShutdownFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appdomainid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppDomainShutdownFinished(this, ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn AssemblyLoadStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assemblyid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssemblyLoadStarted(this, ::core::mem::transmute_copy(&assemblyid)).into())
        }
        unsafe extern "system" fn AssemblyLoadFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assemblyid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssemblyLoadFinished(this, ::core::mem::transmute_copy(&assemblyid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn AssemblyUnloadStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assemblyid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssemblyUnloadStarted(this, ::core::mem::transmute_copy(&assemblyid)).into())
        }
        unsafe extern "system" fn AssemblyUnloadFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assemblyid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssemblyUnloadFinished(this, ::core::mem::transmute_copy(&assemblyid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn ModuleLoadStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModuleLoadStarted(this, ::core::mem::transmute_copy(&moduleid)).into())
        }
        unsafe extern "system" fn ModuleLoadFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModuleLoadFinished(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn ModuleUnloadStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModuleUnloadStarted(this, ::core::mem::transmute_copy(&moduleid)).into())
        }
        unsafe extern "system" fn ModuleUnloadFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModuleUnloadFinished(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn ModuleAttachedToAssembly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, assemblyid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModuleAttachedToAssembly(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&assemblyid)).into())
        }
        unsafe extern "system" fn ClassLoadStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClassLoadStarted(this, ::core::mem::transmute_copy(&classid)).into())
        }
        unsafe extern "system" fn ClassLoadFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClassLoadFinished(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn ClassUnloadStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClassUnloadStarted(this, ::core::mem::transmute_copy(&classid)).into())
        }
        unsafe extern "system" fn ClassUnloadFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClassUnloadFinished(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn FunctionUnloadStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FunctionUnloadStarted(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn JITCompilationStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JITCompilationStarted(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&fissafetoblock)).into())
        }
        unsafe extern "system" fn JITCompilationFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, hrstatus: ::windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JITCompilationFinished(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&fissafetoblock)).into())
        }
        unsafe extern "system" fn JITCachedFunctionSearchStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JITCachedFunctionSearchStarted(this, ::core::mem::transmute_copy(&functionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusecachedfunction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JITCachedFunctionSearchFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, result: COR_PRF_JIT_CACHE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JITCachedFunctionSearchFinished(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&result)).into())
        }
        unsafe extern "system" fn JITFunctionPitched<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JITFunctionPitched(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn JITInlining<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JITInlining(this, ::core::mem::transmute_copy(&callerid), ::core::mem::transmute_copy(&calleeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfshouldinline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ThreadCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadCreated(this, ::core::mem::transmute_copy(&threadid)).into())
        }
        unsafe extern "system" fn ThreadDestroyed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadDestroyed(this, ::core::mem::transmute_copy(&threadid)).into())
        }
        unsafe extern "system" fn ThreadAssignedToOSThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, managedthreadid: usize, osthreadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadAssignedToOSThread(this, ::core::mem::transmute_copy(&managedthreadid), ::core::mem::transmute_copy(&osthreadid)).into())
        }
        unsafe extern "system" fn RemotingClientInvocationStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingClientInvocationStarted(this).into())
        }
        unsafe extern "system" fn RemotingClientSendingMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingClientSendingMessage(this, ::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into())
        }
        unsafe extern "system" fn RemotingClientReceivingReply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingClientReceivingReply(this, ::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into())
        }
        unsafe extern "system" fn RemotingClientInvocationFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingClientInvocationFinished(this).into())
        }
        unsafe extern "system" fn RemotingServerReceivingMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingServerReceivingMessage(this, ::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into())
        }
        unsafe extern "system" fn RemotingServerInvocationStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingServerInvocationStarted(this).into())
        }
        unsafe extern "system" fn RemotingServerInvocationReturned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingServerInvocationReturned(this).into())
        }
        unsafe extern "system" fn RemotingServerSendingReply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *const ::windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemotingServerSendingReply(this, ::core::mem::transmute_copy(&pcookie), ::core::mem::transmute_copy(&fisasync)).into())
        }
        unsafe extern "system" fn UnmanagedToManagedTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnmanagedToManagedTransition(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&reason)).into())
        }
        unsafe extern "system" fn ManagedToUnmanagedTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ManagedToUnmanagedTransition(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&reason)).into())
        }
        unsafe extern "system" fn RuntimeSuspendStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, suspendreason: COR_PRF_SUSPEND_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RuntimeSuspendStarted(this, ::core::mem::transmute_copy(&suspendreason)).into())
        }
        unsafe extern "system" fn RuntimeSuspendFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RuntimeSuspendFinished(this).into())
        }
        unsafe extern "system" fn RuntimeSuspendAborted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RuntimeSuspendAborted(this).into())
        }
        unsafe extern "system" fn RuntimeResumeStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RuntimeResumeStarted(this).into())
        }
        unsafe extern "system" fn RuntimeResumeFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RuntimeResumeFinished(this).into())
        }
        unsafe extern "system" fn RuntimeThreadSuspended<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RuntimeThreadSuspended(this, ::core::mem::transmute_copy(&threadid)).into())
        }
        unsafe extern "system" fn RuntimeThreadResumed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RuntimeThreadResumed(this, ::core::mem::transmute_copy(&threadid)).into())
        }
        unsafe extern "system" fn MovedReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MovedReferences(this, ::core::mem::transmute_copy(&cmovedobjectidranges), ::core::mem::transmute_copy(&oldobjectidrangestart), ::core::mem::transmute_copy(&newobjectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into())
        }
        unsafe extern "system" fn ObjectAllocated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, classid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ObjectAllocated(this, ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&classid)).into())
        }
        unsafe extern "system" fn ObjectsAllocatedByClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ObjectsAllocatedByClass(this, ::core::mem::transmute_copy(&cclasscount), ::core::mem::transmute_copy(&classids), ::core::mem::transmute_copy(&cobjects)).into())
        }
        unsafe extern "system" fn ObjectReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ObjectReferences(this, ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&cobjectrefs), ::core::mem::transmute_copy(&objectrefids)).into())
        }
        unsafe extern "system" fn RootReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crootrefs: u32, rootrefids: *const usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RootReferences(this, ::core::mem::transmute_copy(&crootrefs), ::core::mem::transmute_copy(&rootrefids)).into())
        }
        unsafe extern "system" fn ExceptionThrown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thrownobjectid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionThrown(this, ::core::mem::transmute_copy(&thrownobjectid)).into())
        }
        unsafe extern "system" fn ExceptionSearchFunctionEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionSearchFunctionEnter(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn ExceptionSearchFunctionLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionSearchFunctionLeave(this).into())
        }
        unsafe extern "system" fn ExceptionSearchFilterEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionSearchFilterEnter(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn ExceptionSearchFilterLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionSearchFilterLeave(this).into())
        }
        unsafe extern "system" fn ExceptionSearchCatcherFound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionSearchCatcherFound(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn ExceptionOSHandlerEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __unused: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionOSHandlerEnter(this, ::core::mem::transmute_copy(&__unused)).into())
        }
        unsafe extern "system" fn ExceptionOSHandlerLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __unused: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionOSHandlerLeave(this, ::core::mem::transmute_copy(&__unused)).into())
        }
        unsafe extern "system" fn ExceptionUnwindFunctionEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionUnwindFunctionEnter(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn ExceptionUnwindFunctionLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionUnwindFunctionLeave(this).into())
        }
        unsafe extern "system" fn ExceptionUnwindFinallyEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionUnwindFinallyEnter(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn ExceptionUnwindFinallyLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionUnwindFinallyLeave(this).into())
        }
        unsafe extern "system" fn ExceptionCatcherEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, objectid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionCatcherEnter(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&objectid)).into())
        }
        unsafe extern "system" fn ExceptionCatcherLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionCatcherLeave(this).into())
        }
        unsafe extern "system" fn COMClassicVTableCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrappedclassid: usize, implementediid: *const ::windows_core::GUID, pvtable: *const ::core::ffi::c_void, cslots: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::COMClassicVTableCreated(this, ::core::mem::transmute_copy(&wrappedclassid), ::core::mem::transmute_copy(&implementediid), ::core::mem::transmute_copy(&pvtable), ::core::mem::transmute_copy(&cslots)).into())
        }
        unsafe extern "system" fn COMClassicVTableDestroyed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrappedclassid: usize, implementediid: *const ::windows_core::GUID, pvtable: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::COMClassicVTableDestroyed(this, ::core::mem::transmute_copy(&wrappedclassid), ::core::mem::transmute_copy(&implementediid), ::core::mem::transmute_copy(&pvtable)).into())
        }
        unsafe extern "system" fn ExceptionCLRCatcherFound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionCLRCatcherFound(this).into())
        }
        unsafe extern "system" fn ExceptionCLRCatcherExecute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExceptionCLRCatcherExecute(this).into())
        }
        ICorProfilerCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            AppDomainCreationStarted: AppDomainCreationStarted::<Identity, Impl, OFFSET>,
            AppDomainCreationFinished: AppDomainCreationFinished::<Identity, Impl, OFFSET>,
            AppDomainShutdownStarted: AppDomainShutdownStarted::<Identity, Impl, OFFSET>,
            AppDomainShutdownFinished: AppDomainShutdownFinished::<Identity, Impl, OFFSET>,
            AssemblyLoadStarted: AssemblyLoadStarted::<Identity, Impl, OFFSET>,
            AssemblyLoadFinished: AssemblyLoadFinished::<Identity, Impl, OFFSET>,
            AssemblyUnloadStarted: AssemblyUnloadStarted::<Identity, Impl, OFFSET>,
            AssemblyUnloadFinished: AssemblyUnloadFinished::<Identity, Impl, OFFSET>,
            ModuleLoadStarted: ModuleLoadStarted::<Identity, Impl, OFFSET>,
            ModuleLoadFinished: ModuleLoadFinished::<Identity, Impl, OFFSET>,
            ModuleUnloadStarted: ModuleUnloadStarted::<Identity, Impl, OFFSET>,
            ModuleUnloadFinished: ModuleUnloadFinished::<Identity, Impl, OFFSET>,
            ModuleAttachedToAssembly: ModuleAttachedToAssembly::<Identity, Impl, OFFSET>,
            ClassLoadStarted: ClassLoadStarted::<Identity, Impl, OFFSET>,
            ClassLoadFinished: ClassLoadFinished::<Identity, Impl, OFFSET>,
            ClassUnloadStarted: ClassUnloadStarted::<Identity, Impl, OFFSET>,
            ClassUnloadFinished: ClassUnloadFinished::<Identity, Impl, OFFSET>,
            FunctionUnloadStarted: FunctionUnloadStarted::<Identity, Impl, OFFSET>,
            JITCompilationStarted: JITCompilationStarted::<Identity, Impl, OFFSET>,
            JITCompilationFinished: JITCompilationFinished::<Identity, Impl, OFFSET>,
            JITCachedFunctionSearchStarted: JITCachedFunctionSearchStarted::<Identity, Impl, OFFSET>,
            JITCachedFunctionSearchFinished: JITCachedFunctionSearchFinished::<Identity, Impl, OFFSET>,
            JITFunctionPitched: JITFunctionPitched::<Identity, Impl, OFFSET>,
            JITInlining: JITInlining::<Identity, Impl, OFFSET>,
            ThreadCreated: ThreadCreated::<Identity, Impl, OFFSET>,
            ThreadDestroyed: ThreadDestroyed::<Identity, Impl, OFFSET>,
            ThreadAssignedToOSThread: ThreadAssignedToOSThread::<Identity, Impl, OFFSET>,
            RemotingClientInvocationStarted: RemotingClientInvocationStarted::<Identity, Impl, OFFSET>,
            RemotingClientSendingMessage: RemotingClientSendingMessage::<Identity, Impl, OFFSET>,
            RemotingClientReceivingReply: RemotingClientReceivingReply::<Identity, Impl, OFFSET>,
            RemotingClientInvocationFinished: RemotingClientInvocationFinished::<Identity, Impl, OFFSET>,
            RemotingServerReceivingMessage: RemotingServerReceivingMessage::<Identity, Impl, OFFSET>,
            RemotingServerInvocationStarted: RemotingServerInvocationStarted::<Identity, Impl, OFFSET>,
            RemotingServerInvocationReturned: RemotingServerInvocationReturned::<Identity, Impl, OFFSET>,
            RemotingServerSendingReply: RemotingServerSendingReply::<Identity, Impl, OFFSET>,
            UnmanagedToManagedTransition: UnmanagedToManagedTransition::<Identity, Impl, OFFSET>,
            ManagedToUnmanagedTransition: ManagedToUnmanagedTransition::<Identity, Impl, OFFSET>,
            RuntimeSuspendStarted: RuntimeSuspendStarted::<Identity, Impl, OFFSET>,
            RuntimeSuspendFinished: RuntimeSuspendFinished::<Identity, Impl, OFFSET>,
            RuntimeSuspendAborted: RuntimeSuspendAborted::<Identity, Impl, OFFSET>,
            RuntimeResumeStarted: RuntimeResumeStarted::<Identity, Impl, OFFSET>,
            RuntimeResumeFinished: RuntimeResumeFinished::<Identity, Impl, OFFSET>,
            RuntimeThreadSuspended: RuntimeThreadSuspended::<Identity, Impl, OFFSET>,
            RuntimeThreadResumed: RuntimeThreadResumed::<Identity, Impl, OFFSET>,
            MovedReferences: MovedReferences::<Identity, Impl, OFFSET>,
            ObjectAllocated: ObjectAllocated::<Identity, Impl, OFFSET>,
            ObjectsAllocatedByClass: ObjectsAllocatedByClass::<Identity, Impl, OFFSET>,
            ObjectReferences: ObjectReferences::<Identity, Impl, OFFSET>,
            RootReferences: RootReferences::<Identity, Impl, OFFSET>,
            ExceptionThrown: ExceptionThrown::<Identity, Impl, OFFSET>,
            ExceptionSearchFunctionEnter: ExceptionSearchFunctionEnter::<Identity, Impl, OFFSET>,
            ExceptionSearchFunctionLeave: ExceptionSearchFunctionLeave::<Identity, Impl, OFFSET>,
            ExceptionSearchFilterEnter: ExceptionSearchFilterEnter::<Identity, Impl, OFFSET>,
            ExceptionSearchFilterLeave: ExceptionSearchFilterLeave::<Identity, Impl, OFFSET>,
            ExceptionSearchCatcherFound: ExceptionSearchCatcherFound::<Identity, Impl, OFFSET>,
            ExceptionOSHandlerEnter: ExceptionOSHandlerEnter::<Identity, Impl, OFFSET>,
            ExceptionOSHandlerLeave: ExceptionOSHandlerLeave::<Identity, Impl, OFFSET>,
            ExceptionUnwindFunctionEnter: ExceptionUnwindFunctionEnter::<Identity, Impl, OFFSET>,
            ExceptionUnwindFunctionLeave: ExceptionUnwindFunctionLeave::<Identity, Impl, OFFSET>,
            ExceptionUnwindFinallyEnter: ExceptionUnwindFinallyEnter::<Identity, Impl, OFFSET>,
            ExceptionUnwindFinallyLeave: ExceptionUnwindFinallyLeave::<Identity, Impl, OFFSET>,
            ExceptionCatcherEnter: ExceptionCatcherEnter::<Identity, Impl, OFFSET>,
            ExceptionCatcherLeave: ExceptionCatcherLeave::<Identity, Impl, OFFSET>,
            COMClassicVTableCreated: COMClassicVTableCreated::<Identity, Impl, OFFSET>,
            COMClassicVTableDestroyed: COMClassicVTableDestroyed::<Identity, Impl, OFFSET>,
            ExceptionCLRCatcherFound: ExceptionCLRCatcherFound::<Identity, Impl, OFFSET>,
            ExceptionCLRCatcherExecute: ExceptionCLRCatcherExecute::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback10_Impl: ::windows_core::BaseImpl + ICorProfilerCallback9_Impl {
    fn EventPipeEventDelivered(this: &Self::This, provider: usize, eventid: u32, eventversion: u32, cbmetadatablob: u32, metadatablob: *const u8, cbeventdata: u32, eventdata: *const u8, pactivityid: *const ::windows_core::GUID, prelatedactivityid: *const ::windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> ::windows_core::Result<()>;
    fn EventPipeProviderCreated(this: &Self::This, provider: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback10 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback9);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback10_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback10 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventPipeEventDelivered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: usize, eventid: u32, eventversion: u32, cbmetadatablob: u32, metadatablob: *const u8, cbeventdata: u32, eventdata: *const u8, pactivityid: *const ::windows_core::GUID, prelatedactivityid: *const ::windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::EventPipeEventDelivered(
                    this,
                    ::core::mem::transmute_copy(&provider),
                    ::core::mem::transmute_copy(&eventid),
                    ::core::mem::transmute_copy(&eventversion),
                    ::core::mem::transmute_copy(&cbmetadatablob),
                    ::core::mem::transmute_copy(&metadatablob),
                    ::core::mem::transmute_copy(&cbeventdata),
                    ::core::mem::transmute_copy(&eventdata),
                    ::core::mem::transmute_copy(&pactivityid),
                    ::core::mem::transmute_copy(&prelatedactivityid),
                    ::core::mem::transmute_copy(&eventthread),
                    ::core::mem::transmute_copy(&numstackframes),
                    ::core::mem::transmute_copy(&stackframes),
                )
                .into()
            })
        }
        unsafe extern "system" fn EventPipeProviderCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EventPipeProviderCreated(this, ::core::mem::transmute_copy(&provider)).into())
        }
        ICorProfilerCallback10_Vtbl {
            base__: <ICorProfilerCallback9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventPipeEventDelivered: EventPipeEventDelivered::<Identity, Impl, OFFSET>,
            EventPipeProviderCreated: EventPipeProviderCreated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback11_Impl: ::windows_core::BaseImpl + ICorProfilerCallback10_Impl {
    fn LoadAsNotificationOnly(this: &Self::This, pbnotificationonly: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback11 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback10);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback11_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback11 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadAsNotificationOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback11_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbnotificationonly: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadAsNotificationOnly(this, ::core::mem::transmute_copy(&pbnotificationonly)).into())
        }
        ICorProfilerCallback11_Vtbl {
            base__: <ICorProfilerCallback10 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadAsNotificationOnly: LoadAsNotificationOnly::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback2_Impl: ::windows_core::BaseImpl + ICorProfilerCallback_Impl {
    fn ThreadNameChanged(this: &Self::This, threadid: usize, cchname: u32, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GarbageCollectionStarted(this: &Self::This, cgenerations: i32, generationcollected: *const super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows_core::Result<()>;
    fn SurvivingReferences(this: &Self::This, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> ::windows_core::Result<()>;
    fn GarbageCollectionFinished(this: &Self::This) -> ::windows_core::Result<()>;
    fn FinalizeableObjectQueued(this: &Self::This, finalizerflags: u32, objectid: usize) -> ::windows_core::Result<()>;
    fn RootReferences2(this: &Self::This, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> ::windows_core::Result<()>;
    fn HandleCreated(this: &Self::This, handleid: usize, initialobjectid: usize) -> ::windows_core::Result<()>;
    fn HandleDestroyed(this: &Self::This, handleid: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThreadNameChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize, cchname: u32, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadNameChanged(this, ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn GarbageCollectionStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cgenerations: i32, generationcollected: *const super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GarbageCollectionStarted(this, ::core::mem::transmute_copy(&cgenerations), ::core::mem::transmute_copy(&generationcollected), ::core::mem::transmute_copy(&reason)).into())
        }
        unsafe extern "system" fn SurvivingReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SurvivingReferences(this, ::core::mem::transmute_copy(&csurvivingobjectidranges), ::core::mem::transmute_copy(&objectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into())
        }
        unsafe extern "system" fn GarbageCollectionFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GarbageCollectionFinished(this).into())
        }
        unsafe extern "system" fn FinalizeableObjectQueued<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalizerflags: u32, objectid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinalizeableObjectQueued(this, ::core::mem::transmute_copy(&finalizerflags), ::core::mem::transmute_copy(&objectid)).into())
        }
        unsafe extern "system" fn RootReferences2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RootReferences2(this, ::core::mem::transmute_copy(&crootrefs), ::core::mem::transmute_copy(&rootrefids), ::core::mem::transmute_copy(&rootkinds), ::core::mem::transmute_copy(&rootflags), ::core::mem::transmute_copy(&rootids)).into())
        }
        unsafe extern "system" fn HandleCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handleid: usize, initialobjectid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleCreated(this, ::core::mem::transmute_copy(&handleid), ::core::mem::transmute_copy(&initialobjectid)).into())
        }
        unsafe extern "system" fn HandleDestroyed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handleid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleDestroyed(this, ::core::mem::transmute_copy(&handleid)).into())
        }
        ICorProfilerCallback2_Vtbl {
            base__: <ICorProfilerCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ThreadNameChanged: ThreadNameChanged::<Identity, Impl, OFFSET>,
            GarbageCollectionStarted: GarbageCollectionStarted::<Identity, Impl, OFFSET>,
            SurvivingReferences: SurvivingReferences::<Identity, Impl, OFFSET>,
            GarbageCollectionFinished: GarbageCollectionFinished::<Identity, Impl, OFFSET>,
            FinalizeableObjectQueued: FinalizeableObjectQueued::<Identity, Impl, OFFSET>,
            RootReferences2: RootReferences2::<Identity, Impl, OFFSET>,
            HandleCreated: HandleCreated::<Identity, Impl, OFFSET>,
            HandleDestroyed: HandleDestroyed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback3_Impl: ::windows_core::BaseImpl + ICorProfilerCallback2_Impl {
    fn InitializeForAttach(this: &Self::This, pcorprofilerinfounk: ::core::option::Option<&::windows_core::IUnknown>, pvclientdata: *const ::core::ffi::c_void, cbclientdata: u32) -> ::windows_core::Result<()>;
    fn ProfilerAttachComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn ProfilerDetachSucceeded(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeForAttach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcorprofilerinfounk: *mut ::core::ffi::c_void, pvclientdata: *const ::core::ffi::c_void, cbclientdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeForAttach(this, ::windows_core::from_raw_borrowed(&pcorprofilerinfounk), ::core::mem::transmute_copy(&pvclientdata), ::core::mem::transmute_copy(&cbclientdata)).into())
        }
        unsafe extern "system" fn ProfilerAttachComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProfilerAttachComplete(this).into())
        }
        unsafe extern "system" fn ProfilerDetachSucceeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProfilerDetachSucceeded(this).into())
        }
        ICorProfilerCallback3_Vtbl {
            base__: <ICorProfilerCallback2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeForAttach: InitializeForAttach::<Identity, Impl, OFFSET>,
            ProfilerAttachComplete: ProfilerAttachComplete::<Identity, Impl, OFFSET>,
            ProfilerDetachSucceeded: ProfilerDetachSucceeded::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback4_Impl: ::windows_core::BaseImpl + ICorProfilerCallback3_Impl {
    fn ReJITCompilationStarted(this: &Self::This, functionid: usize, rejitid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetReJITParameters(this: &Self::This, moduleid: usize, methodid: u32, pfunctioncontrol: ::core::option::Option<&ICorProfilerFunctionControl>) -> ::windows_core::Result<()>;
    fn ReJITCompilationFinished(this: &Self::This, functionid: usize, rejitid: usize, hrstatus: ::windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ReJITError(this: &Self::This, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn MovedReferences2(this: &Self::This, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> ::windows_core::Result<()>;
    fn SurvivingReferences2(this: &Self::This, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReJITCompilationStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReJITCompilationStarted(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&fissafetoblock)).into())
        }
        unsafe extern "system" fn GetReJITParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, pfunctioncontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReJITParameters(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::windows_core::from_raw_borrowed(&pfunctioncontrol)).into())
        }
        unsafe extern "system" fn ReJITCompilationFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, hrstatus: ::windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReJITCompilationFinished(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&fissafetoblock)).into())
        }
        unsafe extern "system" fn ReJITError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, functionid: usize, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReJITError(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn MovedReferences2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MovedReferences2(this, ::core::mem::transmute_copy(&cmovedobjectidranges), ::core::mem::transmute_copy(&oldobjectidrangestart), ::core::mem::transmute_copy(&newobjectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into())
        }
        unsafe extern "system" fn SurvivingReferences2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SurvivingReferences2(this, ::core::mem::transmute_copy(&csurvivingobjectidranges), ::core::mem::transmute_copy(&objectidrangestart), ::core::mem::transmute_copy(&cobjectidrangelength)).into())
        }
        ICorProfilerCallback4_Vtbl {
            base__: <ICorProfilerCallback3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReJITCompilationStarted: ReJITCompilationStarted::<Identity, Impl, OFFSET>,
            GetReJITParameters: GetReJITParameters::<Identity, Impl, OFFSET>,
            ReJITCompilationFinished: ReJITCompilationFinished::<Identity, Impl, OFFSET>,
            ReJITError: ReJITError::<Identity, Impl, OFFSET>,
            MovedReferences2: MovedReferences2::<Identity, Impl, OFFSET>,
            SurvivingReferences2: SurvivingReferences2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback5_Impl: ::windows_core::BaseImpl + ICorProfilerCallback4_Impl {
    fn ConditionalWeakTableElementReferences(this: &Self::This, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback4);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConditionalWeakTableElementReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConditionalWeakTableElementReferences(this, ::core::mem::transmute_copy(&crootrefs), ::core::mem::transmute_copy(&keyrefids), ::core::mem::transmute_copy(&valuerefids), ::core::mem::transmute_copy(&rootids)).into())
        }
        ICorProfilerCallback5_Vtbl {
            base__: <ICorProfilerCallback4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConditionalWeakTableElementReferences: ConditionalWeakTableElementReferences::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback6_Impl: ::windows_core::BaseImpl + ICorProfilerCallback5_Impl {
    fn GetAssemblyReferences(this: &Self::This, wszassemblypath: &::windows_core::PCWSTR, pasmrefprovider: ::core::option::Option<&ICorProfilerAssemblyReferenceProvider>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback5);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAssemblyReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszassemblypath: ::windows_core::PCWSTR, pasmrefprovider: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAssemblyReferences(this, ::core::mem::transmute(&wszassemblypath), ::windows_core::from_raw_borrowed(&pasmrefprovider)).into())
        }
        ICorProfilerCallback6_Vtbl {
            base__: <ICorProfilerCallback5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAssemblyReferences: GetAssemblyReferences::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback7_Impl: ::windows_core::BaseImpl + ICorProfilerCallback6_Impl {
    fn ModuleInMemorySymbolsUpdated(this: &Self::This, moduleid: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback6);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ModuleInMemorySymbolsUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModuleInMemorySymbolsUpdated(this, ::core::mem::transmute_copy(&moduleid)).into())
        }
        ICorProfilerCallback7_Vtbl {
            base__: <ICorProfilerCallback6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ModuleInMemorySymbolsUpdated: ModuleInMemorySymbolsUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback8_Impl: ::windows_core::BaseImpl + ICorProfilerCallback7_Impl {
    fn DynamicMethodJITCompilationStarted(this: &Self::This, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL, pilheader: *const u8, cbilheader: u32) -> ::windows_core::Result<()>;
    fn DynamicMethodJITCompilationFinished(this: &Self::This, functionid: usize, hrstatus: ::windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback7);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DynamicMethodJITCompilationStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL, pilheader: *const u8, cbilheader: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DynamicMethodJITCompilationStarted(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&fissafetoblock), ::core::mem::transmute_copy(&pilheader), ::core::mem::transmute_copy(&cbilheader)).into())
        }
        unsafe extern "system" fn DynamicMethodJITCompilationFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, hrstatus: ::windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DynamicMethodJITCompilationFinished(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&fissafetoblock)).into())
        }
        ICorProfilerCallback8_Vtbl {
            base__: <ICorProfilerCallback7 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DynamicMethodJITCompilationStarted: DynamicMethodJITCompilationStarted::<Identity, Impl, OFFSET>,
            DynamicMethodJITCompilationFinished: DynamicMethodJITCompilationFinished::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerCallback9_Impl: ::windows_core::BaseImpl + ICorProfilerCallback8_Impl {
    fn DynamicMethodUnloaded(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerCallback9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerCallback8);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerCallback9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DynamicMethodUnloaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerCallback9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DynamicMethodUnloaded(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        ICorProfilerCallback9_Vtbl {
            base__: <ICorProfilerCallback8 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DynamicMethodUnloaded: DynamicMethodUnloaded::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICorProfilerFunctionControl_Impl: ::windows_core::BaseImpl {
    fn SetCodegenFlags(this: &Self::This, flags: u32) -> ::windows_core::Result<()>;
    fn SetILFunctionBody(this: &Self::This, cbnewilmethodheader: u32, pbnewilmethodheader: *const u8) -> ::windows_core::Result<()>;
    fn SetILInstrumentedCodeMap(this: &Self::This, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICorProfilerFunctionControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerFunctionControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCodegenFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCodegenFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbnewilmethodheader: u32, pbnewilmethodheader: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetILFunctionBody(this, ::core::mem::transmute_copy(&cbnewilmethodheader), ::core::mem::transmute_copy(&pbnewilmethodheader)).into())
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetILInstrumentedCodeMap(this, ::core::mem::transmute_copy(&cilmapentries), ::core::mem::transmute_copy(&rgilmapentries)).into())
        }
        ICorProfilerFunctionControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCodegenFlags: SetCodegenFlags::<Identity, Impl, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, Impl, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICorProfilerFunctionEnum_Impl: ::windows_core::BaseImpl {
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ICorProfilerFunctionEnum>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Next(this: &Self::This, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICorProfilerFunctionEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerFunctionEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerFunctionEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ids), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        ICorProfilerFunctionEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo_Impl: ::windows_core::BaseImpl {
    fn GetClassFromObject(this: &Self::This, objectid: usize) -> ::windows_core::Result<usize>;
    fn GetClassFromToken(this: &Self::This, moduleid: usize, typedef: u32) -> ::windows_core::Result<usize>;
    fn GetCodeInfo(this: &Self::This, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetEventMask(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFunctionFromIP(this: &Self::This, ip: *const u8) -> ::windows_core::Result<usize>;
    fn GetFunctionFromToken(this: &Self::This, moduleid: usize, token: u32) -> ::windows_core::Result<usize>;
    fn GetHandleFromThread(this: &Self::This, threadid: usize) -> ::windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn GetObjectSize(this: &Self::This, objectid: usize) -> ::windows_core::Result<u32>;
    fn IsArrayClass(this: &Self::This, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows_core::Result<()>;
    fn GetThreadInfo(this: &Self::This, threadid: usize) -> ::windows_core::Result<u32>;
    fn GetCurrentThreadID(this: &Self::This) -> ::windows_core::Result<usize>;
    fn GetClassIDInfo(this: &Self::This, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows_core::Result<()>;
    fn GetFunctionInfo(this: &Self::This, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows_core::Result<()>;
    fn SetEventMask(this: &Self::This, dwevents: u32) -> ::windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks(this: &Self::This, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> ::windows_core::Result<()>;
    fn SetFunctionIDMapper(this: &Self::This, pfunc: *const FunctionIDMapper) -> ::windows_core::Result<()>;
    fn GetTokenAndMetaDataFromFunction(this: &Self::This, functionid: usize, riid: *const ::windows_core::GUID, ppimport: *mut ::core::option::Option<::windows_core::IUnknown>, ptoken: *mut u32) -> ::windows_core::Result<()>;
    fn GetModuleInfo(this: &Self::This, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, passemblyid: *mut usize) -> ::windows_core::Result<()>;
    fn GetModuleMetaData(this: &Self::This, moduleid: usize, dwopenflags: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetILFunctionBody(this: &Self::This, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetILFunctionBodyAllocator(this: &Self::This, moduleid: usize) -> ::windows_core::Result<IMethodMalloc>;
    fn SetILFunctionBody(this: &Self::This, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> ::windows_core::Result<()>;
    fn GetAppDomainInfo(this: &Self::This, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, pprocessid: *mut usize) -> ::windows_core::Result<()>;
    fn GetAssemblyInfo(this: &Self::This, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows_core::Result<()>;
    fn SetFunctionReJIT(this: &Self::This, functionid: usize) -> ::windows_core::Result<()>;
    fn ForceGC(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetILInstrumentedCodeMap(this: &Self::This, functionid: usize, fstartjit: super::super::super::Foundation::BOOL, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> ::windows_core::Result<()>;
    fn GetInprocInspectionInterface(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetInprocInspectionIThisThread(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetThreadContext(this: &Self::This, threadid: usize) -> ::windows_core::Result<usize>;
    fn BeginInprocDebugging(this: &Self::This, fthisthreadonly: super::super::super::Foundation::BOOL) -> ::windows_core::Result<u32>;
    fn EndInprocDebugging(this: &Self::This, dwprofilercontext: u32) -> ::windows_core::Result<()>;
    fn GetILToNativeMapping(this: &Self::This, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassFromObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, pclassid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassFromObject(this, ::core::mem::transmute_copy(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClassFromToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, typedef: u32, pclassid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassFromToken(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&typedef)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCodeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodeInfo(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pcsize)).into())
        }
        unsafe extern "system" fn GetEventMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwevents: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventMask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionFromIP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ip: *const u8, pfunctionid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionFromIP(this, ::core::mem::transmute_copy(&ip)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfunctionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionFromToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, token: u32, pfunctionid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionFromToken(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&token)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfunctionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHandleFromThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHandleFromThread(this, ::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, pcsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectSize(this, ::core::mem::transmute_copy(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsArrayClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsArrayClass(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pbaseelemtype), ::core::mem::transmute_copy(&pbaseclassid), ::core::mem::transmute_copy(&pcrank)).into())
        }
        unsafe extern "system" fn GetThreadInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize, pdwwin32threadid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThreadInfo(this, ::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwwin32threadid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentThreadID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pthreadid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentThreadID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pthreadid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClassIDInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassIDInfo(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptypedeftoken)).into())
        }
        unsafe extern "system" fn GetFunctionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionInfo(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&pclassid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptoken)).into())
        }
        unsafe extern "system" fn SetEventMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwevents: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventMask(this, ::core::mem::transmute_copy(&dwevents)).into())
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnterLeaveFunctionHooks(this, ::core::mem::transmute_copy(&pfuncenter), ::core::mem::transmute_copy(&pfuncleave), ::core::mem::transmute_copy(&pfunctailcall)).into())
        }
        unsafe extern "system" fn SetFunctionIDMapper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunc: *const FunctionIDMapper) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFunctionIDMapper(this, ::core::mem::transmute_copy(&pfunc)).into())
        }
        unsafe extern "system" fn GetTokenAndMetaDataFromFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, riid: *const ::windows_core::GUID, ppimport: *mut *mut ::core::ffi::c_void, ptoken: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTokenAndMetaDataFromFunction(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppimport), ::core::mem::transmute_copy(&ptoken)).into())
        }
        unsafe extern "system" fn GetModuleInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, passemblyid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetModuleInfo(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&ppbaseloadaddress), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&passemblyid)).into())
        }
        unsafe extern "system" fn GetModuleMetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, dwopenflags: u32, riid: *const ::windows_core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetModuleMetaData(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&dwopenflags), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetILFunctionBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetILFunctionBody(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::core::mem::transmute_copy(&ppmethodheader), ::core::mem::transmute_copy(&pcbmethodsize)).into())
        }
        unsafe extern "system" fn GetILFunctionBodyAllocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppmalloc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetILFunctionBodyAllocator(this, ::core::mem::transmute_copy(&moduleid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmalloc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetILFunctionBody(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&methodid), ::core::mem::transmute_copy(&pbnewilmethodheader)).into())
        }
        unsafe extern "system" fn GetAppDomainInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, pprocessid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAppDomainInfo(this, ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pprocessid)).into())
        }
        unsafe extern "system" fn GetAssemblyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAssemblyInfo(this, ::core::mem::transmute_copy(&assemblyid), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&pappdomainid), ::core::mem::transmute_copy(&pmoduleid)).into())
        }
        unsafe extern "system" fn SetFunctionReJIT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFunctionReJIT(this, ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn ForceGC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForceGC(this).into())
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, fstartjit: super::super::super::Foundation::BOOL, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetILInstrumentedCodeMap(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&fstartjit), ::core::mem::transmute_copy(&cilmapentries), ::core::mem::transmute_copy(&rgilmapentries)).into())
        }
        unsafe extern "system" fn GetInprocInspectionInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppicd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInprocInspectionInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppicd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInprocInspectionIThisThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppicd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInprocInspectionIThisThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppicd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThreadContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize, pcontextid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThreadContext(this, ::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontextid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginInprocDebugging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fthisthreadonly: super::super::super::Foundation::BOOL, pdwprofilercontext: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginInprocDebugging(this, ::core::mem::transmute_copy(&fthisthreadonly)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwprofilercontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndInprocDebugging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofilercontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndInprocDebugging(this, ::core::mem::transmute_copy(&dwprofilercontext)).into())
        }
        unsafe extern "system" fn GetILToNativeMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetILToNativeMapping(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&cmap), ::core::mem::transmute_copy(&pcmap), ::core::mem::transmute_copy(&map)).into())
        }
        ICorProfilerInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClassFromObject: GetClassFromObject::<Identity, Impl, OFFSET>,
            GetClassFromToken: GetClassFromToken::<Identity, Impl, OFFSET>,
            GetCodeInfo: GetCodeInfo::<Identity, Impl, OFFSET>,
            GetEventMask: GetEventMask::<Identity, Impl, OFFSET>,
            GetFunctionFromIP: GetFunctionFromIP::<Identity, Impl, OFFSET>,
            GetFunctionFromToken: GetFunctionFromToken::<Identity, Impl, OFFSET>,
            GetHandleFromThread: GetHandleFromThread::<Identity, Impl, OFFSET>,
            GetObjectSize: GetObjectSize::<Identity, Impl, OFFSET>,
            IsArrayClass: IsArrayClass::<Identity, Impl, OFFSET>,
            GetThreadInfo: GetThreadInfo::<Identity, Impl, OFFSET>,
            GetCurrentThreadID: GetCurrentThreadID::<Identity, Impl, OFFSET>,
            GetClassIDInfo: GetClassIDInfo::<Identity, Impl, OFFSET>,
            GetFunctionInfo: GetFunctionInfo::<Identity, Impl, OFFSET>,
            SetEventMask: SetEventMask::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks: SetEnterLeaveFunctionHooks::<Identity, Impl, OFFSET>,
            SetFunctionIDMapper: SetFunctionIDMapper::<Identity, Impl, OFFSET>,
            GetTokenAndMetaDataFromFunction: GetTokenAndMetaDataFromFunction::<Identity, Impl, OFFSET>,
            GetModuleInfo: GetModuleInfo::<Identity, Impl, OFFSET>,
            GetModuleMetaData: GetModuleMetaData::<Identity, Impl, OFFSET>,
            GetILFunctionBody: GetILFunctionBody::<Identity, Impl, OFFSET>,
            GetILFunctionBodyAllocator: GetILFunctionBodyAllocator::<Identity, Impl, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, Impl, OFFSET>,
            GetAppDomainInfo: GetAppDomainInfo::<Identity, Impl, OFFSET>,
            GetAssemblyInfo: GetAssemblyInfo::<Identity, Impl, OFFSET>,
            SetFunctionReJIT: SetFunctionReJIT::<Identity, Impl, OFFSET>,
            ForceGC: ForceGC::<Identity, Impl, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, Impl, OFFSET>,
            GetInprocInspectionInterface: GetInprocInspectionInterface::<Identity, Impl, OFFSET>,
            GetInprocInspectionIThisThread: GetInprocInspectionIThisThread::<Identity, Impl, OFFSET>,
            GetThreadContext: GetThreadContext::<Identity, Impl, OFFSET>,
            BeginInprocDebugging: BeginInprocDebugging::<Identity, Impl, OFFSET>,
            EndInprocDebugging: EndInprocDebugging::<Identity, Impl, OFFSET>,
            GetILToNativeMapping: GetILToNativeMapping::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo10_Impl: ::windows_core::BaseImpl + ICorProfilerInfo9_Impl {
    fn EnumerateObjectReferences(this: &Self::This, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsFrozenObject(this: &Self::This, objectid: usize, pbfrozen: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLOHObjectSizeThreshold(this: &Self::This, pthreshold: *mut u32) -> ::windows_core::Result<()>;
    fn RequestReJITWithInliners(this: &Self::This, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> ::windows_core::Result<()>;
    fn SuspendRuntime(this: &Self::This) -> ::windows_core::Result<()>;
    fn ResumeRuntime(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo10 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo9);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo10_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo10 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumerateObjectReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateObjectReferences(this, ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&clientdata)).into())
        }
        unsafe extern "system" fn IsFrozenObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, pbfrozen: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsFrozenObject(this, ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&pbfrozen)).into())
        }
        unsafe extern "system" fn GetLOHObjectSizeThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pthreshold: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLOHObjectSizeThreshold(this, ::core::mem::transmute_copy(&pthreshold)).into())
        }
        unsafe extern "system" fn RequestReJITWithInliners<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestReJITWithInliners(this, ::core::mem::transmute_copy(&dwrejitflags), ::core::mem::transmute_copy(&cfunctions), ::core::mem::transmute_copy(&moduleids), ::core::mem::transmute_copy(&methodids)).into())
        }
        unsafe extern "system" fn SuspendRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspendRuntime(this).into())
        }
        unsafe extern "system" fn ResumeRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo10_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeRuntime(this).into())
        }
        ICorProfilerInfo10_Vtbl {
            base__: <ICorProfilerInfo9 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumerateObjectReferences: EnumerateObjectReferences::<Identity, Impl, OFFSET>,
            IsFrozenObject: IsFrozenObject::<Identity, Impl, OFFSET>,
            GetLOHObjectSizeThreshold: GetLOHObjectSizeThreshold::<Identity, Impl, OFFSET>,
            RequestReJITWithInliners: RequestReJITWithInliners::<Identity, Impl, OFFSET>,
            SuspendRuntime: SuspendRuntime::<Identity, Impl, OFFSET>,
            ResumeRuntime: ResumeRuntime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo11_Impl: ::windows_core::BaseImpl + ICorProfilerInfo10_Impl {
    fn GetEnvironmentVariableA(this: &Self::This, szname: &::windows_core::PCWSTR, cchvalue: u32, pcchvalue: *mut u32, szvalue: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetEnvironmentVariable(this: &Self::This, szname: &::windows_core::PCWSTR, szvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo11 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo10);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo11_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo11 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEnvironmentVariableA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo11_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, cchvalue: u32, pcchvalue: *mut u32, szvalue: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEnvironmentVariableA(this, ::core::mem::transmute(&szname), ::core::mem::transmute_copy(&cchvalue), ::core::mem::transmute_copy(&pcchvalue), ::core::mem::transmute_copy(&szvalue)).into())
        }
        unsafe extern "system" fn SetEnvironmentVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo11_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, szvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnvironmentVariable(this, ::core::mem::transmute(&szname), ::core::mem::transmute(&szvalue)).into())
        }
        ICorProfilerInfo11_Vtbl {
            base__: <ICorProfilerInfo10 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEnvironmentVariableA: GetEnvironmentVariableA::<Identity, Impl, OFFSET>,
            SetEnvironmentVariable: SetEnvironmentVariable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo12_Impl: ::windows_core::BaseImpl + ICorProfilerInfo11_Impl {
    fn EventPipeStartSession(this: &Self::This, cproviderconfigs: u32, pproviderconfigs: *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, requestrundown: super::super::super::Foundation::BOOL) -> ::windows_core::Result<u64>;
    fn EventPipeAddProviderToSession(this: &Self::This, session: u64, providerconfig: &COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> ::windows_core::Result<()>;
    fn EventPipeStopSession(this: &Self::This, session: u64) -> ::windows_core::Result<()>;
    fn EventPipeCreateProvider(this: &Self::This, providername: &::windows_core::PCWSTR) -> ::windows_core::Result<usize>;
    fn EventPipeGetProviderInfo(this: &Self::This, provider: usize, cchname: u32, pcchname: *mut u32, providername: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn EventPipeDefineEvent(this: &Self::This, provider: usize, eventname: &::windows_core::PCWSTR, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: super::super::super::Foundation::BOOL, cparamdescs: u32, pparamdescs: *const COR_PRF_EVENTPIPE_PARAM_DESC) -> ::windows_core::Result<usize>;
    fn EventPipeWriteEvent(this: &Self::This, event: usize, cdata: u32, data: *const COR_PRF_EVENT_DATA, pactivityid: *const ::windows_core::GUID, prelatedactivityid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo12 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo11);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo12 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventPipeStartSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproviderconfigs: u32, pproviderconfigs: *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, requestrundown: super::super::super::Foundation::BOOL, psession: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventPipeStartSession(this, ::core::mem::transmute_copy(&cproviderconfigs), ::core::mem::transmute_copy(&pproviderconfigs), ::core::mem::transmute_copy(&requestrundown)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventPipeAddProviderToSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, session: u64, providerconfig: COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EventPipeAddProviderToSession(this, ::core::mem::transmute_copy(&session), ::core::mem::transmute(&providerconfig)).into())
        }
        unsafe extern "system" fn EventPipeStopSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, session: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EventPipeStopSession(this, ::core::mem::transmute_copy(&session)).into())
        }
        unsafe extern "system" fn EventPipeCreateProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::windows_core::PCWSTR, pprovider: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventPipeCreateProvider(this, ::core::mem::transmute(&providername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventPipeGetProviderInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: usize, cchname: u32, pcchname: *mut u32, providername: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EventPipeGetProviderInfo(this, ::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&providername)).into())
        }
        unsafe extern "system" fn EventPipeDefineEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: usize, eventname: ::windows_core::PCWSTR, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: super::super::super::Foundation::BOOL, cparamdescs: u32, pparamdescs: *const COR_PRF_EVENTPIPE_PARAM_DESC, pevent: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventPipeDefineEvent(this, ::core::mem::transmute_copy(&provider), ::core::mem::transmute(&eventname), ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&keywords), ::core::mem::transmute_copy(&eventversion), ::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&opcode), ::core::mem::transmute_copy(&needstack), ::core::mem::transmute_copy(&cparamdescs), ::core::mem::transmute_copy(&pparamdescs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventPipeWriteEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo12_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: usize, cdata: u32, data: *const COR_PRF_EVENT_DATA, pactivityid: *const ::windows_core::GUID, prelatedactivityid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EventPipeWriteEvent(this, ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&cdata), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&pactivityid), ::core::mem::transmute_copy(&prelatedactivityid)).into())
        }
        ICorProfilerInfo12_Vtbl {
            base__: <ICorProfilerInfo11 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventPipeStartSession: EventPipeStartSession::<Identity, Impl, OFFSET>,
            EventPipeAddProviderToSession: EventPipeAddProviderToSession::<Identity, Impl, OFFSET>,
            EventPipeStopSession: EventPipeStopSession::<Identity, Impl, OFFSET>,
            EventPipeCreateProvider: EventPipeCreateProvider::<Identity, Impl, OFFSET>,
            EventPipeGetProviderInfo: EventPipeGetProviderInfo::<Identity, Impl, OFFSET>,
            EventPipeDefineEvent: EventPipeDefineEvent::<Identity, Impl, OFFSET>,
            EventPipeWriteEvent: EventPipeWriteEvent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo13_Impl: ::windows_core::BaseImpl + ICorProfilerInfo12_Impl {
    fn CreateHandle(this: &Self::This, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DestroyHandle(this: &Self::This, handle: *const *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetObjectIDFromHandle(this: &Self::This, handle: *const *const ::core::ffi::c_void) -> ::windows_core::Result<usize>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo13 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo12);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo13_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo13 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo13_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateHandle(this, ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&phandle)).into())
        }
        unsafe extern "system" fn DestroyHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo13_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: *const *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyHandle(this, ::core::mem::transmute_copy(&handle)).into())
        }
        unsafe extern "system" fn GetObjectIDFromHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo13_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: *const *const ::core::ffi::c_void, pobject: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectIDFromHandle(this, ::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICorProfilerInfo13_Vtbl {
            base__: <ICorProfilerInfo12 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateHandle: CreateHandle::<Identity, Impl, OFFSET>,
            DestroyHandle: DestroyHandle::<Identity, Impl, OFFSET>,
            GetObjectIDFromHandle: GetObjectIDFromHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo14_Impl: ::windows_core::BaseImpl + ICorProfilerInfo13_Impl {
    fn EnumerateNonGCObjects(this: &Self::This) -> ::windows_core::Result<ICorProfilerObjectEnum>;
    fn GetNonGCHeapBounds(this: &Self::This, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_NONGC_HEAP_RANGE) -> ::windows_core::Result<()>;
    fn EventPipeCreateProvider2(this: &Self::This, providername: &::windows_core::PCWSTR, pcallback: *const EventPipeProviderCallback) -> ::windows_core::Result<usize>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo14 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo13);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo14_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo14 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumerateNonGCObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo14_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateNonGCObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNonGCHeapBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo14_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_NONGC_HEAP_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNonGCHeapBounds(this, ::core::mem::transmute_copy(&cobjectranges), ::core::mem::transmute_copy(&pcobjectranges), ::core::mem::transmute_copy(&ranges)).into())
        }
        unsafe extern "system" fn EventPipeCreateProvider2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo14_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::windows_core::PCWSTR, pcallback: *const EventPipeProviderCallback, pprovider: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventPipeCreateProvider2(this, ::core::mem::transmute(&providername), ::core::mem::transmute_copy(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICorProfilerInfo14_Vtbl {
            base__: <ICorProfilerInfo13 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumerateNonGCObjects: EnumerateNonGCObjects::<Identity, Impl, OFFSET>,
            GetNonGCHeapBounds: GetNonGCHeapBounds::<Identity, Impl, OFFSET>,
            EventPipeCreateProvider2: EventPipeCreateProvider2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo2_Impl: ::windows_core::BaseImpl + ICorProfilerInfo_Impl {
    fn DoStackSnapshot(this: &Self::This, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const ::core::ffi::c_void, context: *const u8, contextsize: u32) -> ::windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks2(this: &Self::This, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> ::windows_core::Result<()>;
    fn GetFunctionInfo2(this: &Self::This, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows_core::Result<()>;
    fn GetStringLayout(this: &Self::This, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows_core::Result<()>;
    fn GetClassLayout(this: &Self::This, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows_core::Result<()>;
    fn GetClassIDInfo2(this: &Self::This, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows_core::Result<()>;
    fn GetCodeInfo2(this: &Self::This, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows_core::Result<()>;
    fn GetClassFromTokenAndTypeArgs(this: &Self::This, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *const usize) -> ::windows_core::Result<usize>;
    fn GetFunctionFromTokenAndTypeArgs(this: &Self::This, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *const usize) -> ::windows_core::Result<usize>;
    fn EnumModuleFrozenObjects(this: &Self::This, moduleid: usize) -> ::windows_core::Result<ICorProfilerObjectEnum>;
    fn GetArrayObjectInfo(this: &Self::This, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn GetBoxClassLayout(this: &Self::This, classid: usize) -> ::windows_core::Result<u32>;
    fn GetThreadAppDomain(this: &Self::This, threadid: usize) -> ::windows_core::Result<usize>;
    fn GetRVAStaticAddress(this: &Self::This, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAppDomainStaticAddress(this: &Self::This, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetThreadStaticAddress(this: &Self::This, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetContextStaticAddress(this: &Self::This, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetStaticFieldInfo(this: &Self::This, classid: usize, fieldtoken: u32) -> ::windows_core::Result<COR_PRF_STATIC_TYPE>;
    fn GetGenerationBounds(this: &Self::This, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows_core::Result<()>;
    fn GetObjectGeneration(this: &Self::This, objectid: usize) -> ::windows_core::Result<COR_PRF_GC_GENERATION_RANGE>;
    fn GetNotifiedExceptionClauseInfo(this: &Self::This) -> ::windows_core::Result<COR_PRF_EX_CLAUSE_INFO>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoStackSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const ::core::ffi::c_void, context: *const u8, contextsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoStackSnapshot(this, ::core::mem::transmute_copy(&thread), ::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&infoflags), ::core::mem::transmute_copy(&clientdata), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&contextsize)).into())
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnterLeaveFunctionHooks2(this, ::core::mem::transmute_copy(&pfuncenter), ::core::mem::transmute_copy(&pfuncleave), ::core::mem::transmute_copy(&pfunctailcall)).into())
        }
        unsafe extern "system" fn GetFunctionInfo2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionInfo2(this, ::core::mem::transmute_copy(&funcid), ::core::mem::transmute_copy(&frameinfo), ::core::mem::transmute_copy(&pclassid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptoken), ::core::mem::transmute_copy(&ctypeargs), ::core::mem::transmute_copy(&pctypeargs), ::core::mem::transmute_copy(&typeargs)).into())
        }
        unsafe extern "system" fn GetStringLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringLayout(this, ::core::mem::transmute_copy(&pbufferlengthoffset), ::core::mem::transmute_copy(&pstringlengthoffset), ::core::mem::transmute_copy(&pbufferoffset)).into())
        }
        unsafe extern "system" fn GetClassLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassLayout(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&rfieldoffset), ::core::mem::transmute_copy(&cfieldoffset), ::core::mem::transmute_copy(&pcfieldoffset), ::core::mem::transmute_copy(&pulclasssize)).into())
        }
        unsafe extern "system" fn GetClassIDInfo2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassIDInfo2(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&pmoduleid), ::core::mem::transmute_copy(&ptypedeftoken), ::core::mem::transmute_copy(&pparentclassid), ::core::mem::transmute_copy(&cnumtypeargs), ::core::mem::transmute_copy(&pcnumtypeargs), ::core::mem::transmute_copy(&typeargs)).into())
        }
        unsafe extern "system" fn GetCodeInfo2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodeInfo2(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&ccodeinfos), ::core::mem::transmute_copy(&pccodeinfos), ::core::mem::transmute_copy(&codeinfos)).into())
        }
        unsafe extern "system" fn GetClassFromTokenAndTypeArgs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *const usize, pclassid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassFromTokenAndTypeArgs(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&typedef), ::core::mem::transmute_copy(&ctypeargs), ::core::mem::transmute_copy(&typeargs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclassid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionFromTokenAndTypeArgs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *const usize, pfunctionid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionFromTokenAndTypeArgs(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&funcdef), ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&ctypeargs), ::core::mem::transmute_copy(&typeargs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfunctionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumModuleFrozenObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumModuleFrozenObjects(this, ::core::mem::transmute_copy(&moduleid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetArrayObjectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetArrayObjectInfo(this, ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&cdimensions), ::core::mem::transmute_copy(&pdimensionsizes), ::core::mem::transmute_copy(&pdimensionlowerbounds), ::core::mem::transmute_copy(&ppdata)).into())
        }
        unsafe extern "system" fn GetBoxClassLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, pbufferoffset: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoxClassLayout(this, ::core::mem::transmute_copy(&classid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbufferoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThreadAppDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: usize, pappdomainid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThreadAppDomain(this, ::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappdomainid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRVAStaticAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRVAStaticAddress(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&ppaddress)).into())
        }
        unsafe extern "system" fn GetAppDomainStaticAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAppDomainStaticAddress(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&ppaddress)).into())
        }
        unsafe extern "system" fn GetThreadStaticAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetThreadStaticAddress(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&ppaddress)).into())
        }
        unsafe extern "system" fn GetContextStaticAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContextStaticAddress(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&contextid), ::core::mem::transmute_copy(&ppaddress)).into())
        }
        unsafe extern "system" fn GetStaticFieldInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStaticFieldInfo(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfieldinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGenerationBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGenerationBounds(this, ::core::mem::transmute_copy(&cobjectranges), ::core::mem::transmute_copy(&pcobjectranges), ::core::mem::transmute_copy(&ranges)).into())
        }
        unsafe extern "system" fn GetObjectGeneration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectGeneration(this, ::core::mem::transmute_copy(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNotifiedExceptionClauseInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNotifiedExceptionClauseInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICorProfilerInfo2_Vtbl {
            base__: <ICorProfilerInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DoStackSnapshot: DoStackSnapshot::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks2: SetEnterLeaveFunctionHooks2::<Identity, Impl, OFFSET>,
            GetFunctionInfo2: GetFunctionInfo2::<Identity, Impl, OFFSET>,
            GetStringLayout: GetStringLayout::<Identity, Impl, OFFSET>,
            GetClassLayout: GetClassLayout::<Identity, Impl, OFFSET>,
            GetClassIDInfo2: GetClassIDInfo2::<Identity, Impl, OFFSET>,
            GetCodeInfo2: GetCodeInfo2::<Identity, Impl, OFFSET>,
            GetClassFromTokenAndTypeArgs: GetClassFromTokenAndTypeArgs::<Identity, Impl, OFFSET>,
            GetFunctionFromTokenAndTypeArgs: GetFunctionFromTokenAndTypeArgs::<Identity, Impl, OFFSET>,
            EnumModuleFrozenObjects: EnumModuleFrozenObjects::<Identity, Impl, OFFSET>,
            GetArrayObjectInfo: GetArrayObjectInfo::<Identity, Impl, OFFSET>,
            GetBoxClassLayout: GetBoxClassLayout::<Identity, Impl, OFFSET>,
            GetThreadAppDomain: GetThreadAppDomain::<Identity, Impl, OFFSET>,
            GetRVAStaticAddress: GetRVAStaticAddress::<Identity, Impl, OFFSET>,
            GetAppDomainStaticAddress: GetAppDomainStaticAddress::<Identity, Impl, OFFSET>,
            GetThreadStaticAddress: GetThreadStaticAddress::<Identity, Impl, OFFSET>,
            GetContextStaticAddress: GetContextStaticAddress::<Identity, Impl, OFFSET>,
            GetStaticFieldInfo: GetStaticFieldInfo::<Identity, Impl, OFFSET>,
            GetGenerationBounds: GetGenerationBounds::<Identity, Impl, OFFSET>,
            GetObjectGeneration: GetObjectGeneration::<Identity, Impl, OFFSET>,
            GetNotifiedExceptionClauseInfo: GetNotifiedExceptionClauseInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo3_Impl: ::windows_core::BaseImpl + ICorProfilerInfo2_Impl {
    fn EnumJITedFunctions(this: &Self::This) -> ::windows_core::Result<ICorProfilerFunctionEnum>;
    fn RequestProfilerDetach(this: &Self::This, dwexpectedcompletionmilliseconds: u32) -> ::windows_core::Result<()>;
    fn SetFunctionIDMapper2(this: &Self::This, pfunc: *const FunctionIDMapper2, clientdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetStringLayout2(this: &Self::This, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks3(this: &Self::This, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> ::windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks3WithInfo(this: &Self::This, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> ::windows_core::Result<()>;
    fn GetFunctionEnter3Info(this: &Self::This, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows_core::Result<()>;
    fn GetFunctionLeave3Info(this: &Self::This, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows_core::Result<()>;
    fn GetFunctionTailcall3Info(this: &Self::This, functionid: usize, eltinfo: usize) -> ::windows_core::Result<usize>;
    fn EnumModules(this: &Self::This) -> ::windows_core::Result<ICorProfilerModuleEnum>;
    fn GetRuntimeInformation(this: &Self::This, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetThreadStaticAddress2(this: &Self::This, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAppDomainsContainingModule(this: &Self::This, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows_core::Result<()>;
    fn GetModuleInfo2(this: &Self::This, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumJITedFunctions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumJITedFunctions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestProfilerDetach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwexpectedcompletionmilliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestProfilerDetach(this, ::core::mem::transmute_copy(&dwexpectedcompletionmilliseconds)).into())
        }
        unsafe extern "system" fn SetFunctionIDMapper2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfunc: *const FunctionIDMapper2, clientdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFunctionIDMapper2(this, ::core::mem::transmute_copy(&pfunc), ::core::mem::transmute_copy(&clientdata)).into())
        }
        unsafe extern "system" fn GetStringLayout2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringLayout2(this, ::core::mem::transmute_copy(&pstringlengthoffset), ::core::mem::transmute_copy(&pbufferoffset)).into())
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnterLeaveFunctionHooks3(this, ::core::mem::transmute_copy(&pfuncenter3), ::core::mem::transmute_copy(&pfuncleave3), ::core::mem::transmute_copy(&pfunctailcall3)).into())
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3WithInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnterLeaveFunctionHooks3WithInfo(this, ::core::mem::transmute_copy(&pfuncenter3withinfo), ::core::mem::transmute_copy(&pfuncleave3withinfo), ::core::mem::transmute_copy(&pfunctailcall3withinfo)).into())
        }
        unsafe extern "system" fn GetFunctionEnter3Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionEnter3Info(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&eltinfo), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&pcbargumentinfo), ::core::mem::transmute_copy(&pargumentinfo)).into())
        }
        unsafe extern "system" fn GetFunctionLeave3Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionLeave3Info(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&eltinfo), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&pretvalrange)).into())
        }
        unsafe extern "system" fn GetFunctionTailcall3Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionTailcall3Info(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&eltinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pframeinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumModules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumModules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRuntimeInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRuntimeInformation(this, ::core::mem::transmute_copy(&pclrinstanceid), ::core::mem::transmute_copy(&pruntimetype), ::core::mem::transmute_copy(&pmajorversion), ::core::mem::transmute_copy(&pminorversion), ::core::mem::transmute_copy(&pbuildnumber), ::core::mem::transmute_copy(&pqfeversion), ::core::mem::transmute_copy(&cchversionstring), ::core::mem::transmute_copy(&pcchversionstring), ::core::mem::transmute_copy(&szversionstring)).into())
        }
        unsafe extern "system" fn GetThreadStaticAddress2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetThreadStaticAddress2(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute_copy(&fieldtoken), ::core::mem::transmute_copy(&appdomainid), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&ppaddress)).into())
        }
        unsafe extern "system" fn GetAppDomainsContainingModule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAppDomainsContainingModule(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&cappdomainids), ::core::mem::transmute_copy(&pcappdomainids), ::core::mem::transmute_copy(&appdomainids)).into())
        }
        unsafe extern "system" fn GetModuleInfo2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: ::windows_core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetModuleInfo2(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&ppbaseloadaddress), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&szname), ::core::mem::transmute_copy(&passemblyid), ::core::mem::transmute_copy(&pdwmoduleflags)).into())
        }
        ICorProfilerInfo3_Vtbl {
            base__: <ICorProfilerInfo2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumJITedFunctions: EnumJITedFunctions::<Identity, Impl, OFFSET>,
            RequestProfilerDetach: RequestProfilerDetach::<Identity, Impl, OFFSET>,
            SetFunctionIDMapper2: SetFunctionIDMapper2::<Identity, Impl, OFFSET>,
            GetStringLayout2: GetStringLayout2::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks3: SetEnterLeaveFunctionHooks3::<Identity, Impl, OFFSET>,
            SetEnterLeaveFunctionHooks3WithInfo: SetEnterLeaveFunctionHooks3WithInfo::<Identity, Impl, OFFSET>,
            GetFunctionEnter3Info: GetFunctionEnter3Info::<Identity, Impl, OFFSET>,
            GetFunctionLeave3Info: GetFunctionLeave3Info::<Identity, Impl, OFFSET>,
            GetFunctionTailcall3Info: GetFunctionTailcall3Info::<Identity, Impl, OFFSET>,
            EnumModules: EnumModules::<Identity, Impl, OFFSET>,
            GetRuntimeInformation: GetRuntimeInformation::<Identity, Impl, OFFSET>,
            GetThreadStaticAddress2: GetThreadStaticAddress2::<Identity, Impl, OFFSET>,
            GetAppDomainsContainingModule: GetAppDomainsContainingModule::<Identity, Impl, OFFSET>,
            GetModuleInfo2: GetModuleInfo2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo4_Impl: ::windows_core::BaseImpl + ICorProfilerInfo3_Impl {
    fn EnumThreads(this: &Self::This) -> ::windows_core::Result<ICorProfilerThreadEnum>;
    fn InitializeCurrentThread(this: &Self::This) -> ::windows_core::Result<()>;
    fn RequestReJIT(this: &Self::This, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> ::windows_core::Result<()>;
    fn RequestRevert(this: &Self::This, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetCodeInfo3(this: &Self::This, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows_core::Result<()>;
    fn GetFunctionFromIP2(this: &Self::This, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows_core::Result<()>;
    fn GetReJITIDs(this: &Self::This, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows_core::Result<()>;
    fn GetILToNativeMapping2(this: &Self::This, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows_core::Result<()>;
    fn EnumJITedFunctions2(this: &Self::This) -> ::windows_core::Result<ICorProfilerFunctionEnum>;
    fn GetObjectSize2(this: &Self::This, objectid: usize) -> ::windows_core::Result<usize>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeCurrentThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeCurrentThread(this).into())
        }
        unsafe extern "system" fn RequestReJIT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestReJIT(this, ::core::mem::transmute_copy(&cfunctions), ::core::mem::transmute_copy(&moduleids), ::core::mem::transmute_copy(&methodids)).into())
        }
        unsafe extern "system" fn RequestRevert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestRevert(this, ::core::mem::transmute_copy(&cfunctions), ::core::mem::transmute_copy(&moduleids), ::core::mem::transmute_copy(&methodids), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn GetCodeInfo3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodeInfo3(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&ccodeinfos), ::core::mem::transmute_copy(&pccodeinfos), ::core::mem::transmute_copy(&codeinfos)).into())
        }
        unsafe extern "system" fn GetFunctionFromIP2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionFromIP2(this, ::core::mem::transmute_copy(&ip), ::core::mem::transmute_copy(&pfunctionid), ::core::mem::transmute_copy(&prejitid)).into())
        }
        unsafe extern "system" fn GetReJITIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReJITIDs(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&crejitids), ::core::mem::transmute_copy(&pcrejitids), ::core::mem::transmute_copy(&rejitids)).into())
        }
        unsafe extern "system" fn GetILToNativeMapping2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetILToNativeMapping2(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&cmap), ::core::mem::transmute_copy(&pcmap), ::core::mem::transmute_copy(&map)).into())
        }
        unsafe extern "system" fn EnumJITedFunctions2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumJITedFunctions2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectSize2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objectid: usize, pcsize: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectSize2(this, ::core::mem::transmute_copy(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICorProfilerInfo4_Vtbl {
            base__: <ICorProfilerInfo3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumThreads: EnumThreads::<Identity, Impl, OFFSET>,
            InitializeCurrentThread: InitializeCurrentThread::<Identity, Impl, OFFSET>,
            RequestReJIT: RequestReJIT::<Identity, Impl, OFFSET>,
            RequestRevert: RequestRevert::<Identity, Impl, OFFSET>,
            GetCodeInfo3: GetCodeInfo3::<Identity, Impl, OFFSET>,
            GetFunctionFromIP2: GetFunctionFromIP2::<Identity, Impl, OFFSET>,
            GetReJITIDs: GetReJITIDs::<Identity, Impl, OFFSET>,
            GetILToNativeMapping2: GetILToNativeMapping2::<Identity, Impl, OFFSET>,
            EnumJITedFunctions2: EnumJITedFunctions2::<Identity, Impl, OFFSET>,
            GetObjectSize2: GetObjectSize2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo5_Impl: ::windows_core::BaseImpl + ICorProfilerInfo4_Impl {
    fn GetEventMask2(this: &Self::This, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows_core::Result<()>;
    fn SetEventMask2(this: &Self::This, dweventslow: u32, dweventshigh: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEventMask2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEventMask2(this, ::core::mem::transmute_copy(&pdweventslow), ::core::mem::transmute_copy(&pdweventshigh)).into())
        }
        unsafe extern "system" fn SetEventMask2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweventslow: u32, dweventshigh: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventMask2(this, ::core::mem::transmute_copy(&dweventslow), ::core::mem::transmute_copy(&dweventshigh)).into())
        }
        ICorProfilerInfo5_Vtbl {
            base__: <ICorProfilerInfo4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEventMask2: GetEventMask2::<Identity, Impl, OFFSET>,
            SetEventMask2: SetEventMask2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo6_Impl: ::windows_core::BaseImpl + ICorProfilerInfo5_Impl {
    fn EnumNgenModuleMethodsInliningThisMethod(this: &Self::This, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut ::core::option::Option<ICorProfilerMethodEnum>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumNgenModuleMethodsInliningThisMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumNgenModuleMethodsInliningThisMethod(this, ::core::mem::transmute_copy(&inlinersmoduleid), ::core::mem::transmute_copy(&inlineemoduleid), ::core::mem::transmute_copy(&inlineemethodid), ::core::mem::transmute_copy(&incompletedata), ::core::mem::transmute_copy(&ppenum)).into())
        }
        ICorProfilerInfo6_Vtbl {
            base__: <ICorProfilerInfo5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumNgenModuleMethodsInliningThisMethod: EnumNgenModuleMethodsInliningThisMethod::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo7_Impl: ::windows_core::BaseImpl + ICorProfilerInfo6_Impl {
    fn ApplyMetaData(this: &Self::This, moduleid: usize) -> ::windows_core::Result<()>;
    fn GetInMemorySymbolsLength(this: &Self::This, moduleid: usize) -> ::windows_core::Result<u32>;
    fn ReadInMemorySymbols(this: &Self::This, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo6);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApplyMetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyMetaData(this, ::core::mem::transmute_copy(&moduleid)).into())
        }
        unsafe extern "system" fn GetInMemorySymbolsLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, pcountsymbolbytes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInMemorySymbolsLength(this, ::core::mem::transmute_copy(&moduleid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcountsymbolbytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadInMemorySymbols<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadInMemorySymbols(this, ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&symbolsreadoffset), ::core::mem::transmute_copy(&psymbolbytes), ::core::mem::transmute_copy(&countsymbolbytes), ::core::mem::transmute_copy(&pcountsymbolbytesread)).into())
        }
        ICorProfilerInfo7_Vtbl {
            base__: <ICorProfilerInfo6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ApplyMetaData: ApplyMetaData::<Identity, Impl, OFFSET>,
            GetInMemorySymbolsLength: GetInMemorySymbolsLength::<Identity, Impl, OFFSET>,
            ReadInMemorySymbols: ReadInMemorySymbols::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo8_Impl: ::windows_core::BaseImpl + ICorProfilerInfo7_Impl {
    fn IsFunctionDynamic(this: &Self::This, functionid: usize) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetFunctionFromIP3(this: &Self::This, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> ::windows_core::Result<()>;
    fn GetDynamicFunctionInfo(this: &Self::This, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo7);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsFunctionDynamic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, isdynamic: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFunctionDynamic(this, ::core::mem::transmute_copy(&functionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdynamic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionFromIP3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFunctionFromIP3(this, ::core::mem::transmute_copy(&ip), ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&prejitid)).into())
        }
        unsafe extern "system" fn GetDynamicFunctionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDynamicFunctionInfo(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&moduleid), ::core::mem::transmute_copy(&ppvsig), ::core::mem::transmute_copy(&pbsig), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&wszname)).into())
        }
        ICorProfilerInfo8_Vtbl {
            base__: <ICorProfilerInfo7 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsFunctionDynamic: IsFunctionDynamic::<Identity, Impl, OFFSET>,
            GetFunctionFromIP3: GetFunctionFromIP3::<Identity, Impl, OFFSET>,
            GetDynamicFunctionInfo: GetDynamicFunctionInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_WinRT_Metadata\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
pub trait ICorProfilerInfo9_Impl: ::windows_core::BaseImpl + ICorProfilerInfo8_Impl {
    fn GetNativeCodeStartAddresses(this: &Self::This, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> ::windows_core::Result<()>;
    fn GetILToNativeMapping3(this: &Self::This, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows_core::Result<()>;
    fn GetCodeInfo4(this: &Self::This, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl ::windows_core::Iids for ICorProfilerInfo9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICorProfilerInfo8);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WinRT_Metadata"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerInfo9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNativeCodeStartAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNativeCodeStartAddresses(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&rejitid), ::core::mem::transmute_copy(&ccodestartaddresses), ::core::mem::transmute_copy(&pccodestartaddresses), ::core::mem::transmute_copy(&codestartaddresses)).into())
        }
        unsafe extern "system" fn GetILToNativeMapping3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetILToNativeMapping3(this, ::core::mem::transmute_copy(&pnativecodestartaddress), ::core::mem::transmute_copy(&cmap), ::core::mem::transmute_copy(&pcmap), ::core::mem::transmute_copy(&map)).into())
        }
        unsafe extern "system" fn GetCodeInfo4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerInfo9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodeInfo4(this, ::core::mem::transmute_copy(&pnativecodestartaddress), ::core::mem::transmute_copy(&ccodeinfos), ::core::mem::transmute_copy(&pccodeinfos), ::core::mem::transmute_copy(&codeinfos)).into())
        }
        ICorProfilerInfo9_Vtbl {
            base__: <ICorProfilerInfo8 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNativeCodeStartAddresses: GetNativeCodeStartAddresses::<Identity, Impl, OFFSET>,
            GetILToNativeMapping3: GetILToNativeMapping3::<Identity, Impl, OFFSET>,
            GetCodeInfo4: GetCodeInfo4::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICorProfilerMethodEnum_Impl: ::windows_core::BaseImpl {
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ICorProfilerMethodEnum>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Next(this: &Self::This, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICorProfilerMethodEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerMethodEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerMethodEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        ICorProfilerMethodEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICorProfilerModuleEnum_Impl: ::windows_core::BaseImpl {
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ICorProfilerModuleEnum>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Next(this: &Self::This, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICorProfilerModuleEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerModuleEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerModuleEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ids), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        ICorProfilerModuleEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICorProfilerObjectEnum_Impl: ::windows_core::BaseImpl {
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ICorProfilerObjectEnum>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Next(this: &Self::This, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICorProfilerObjectEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerObjectEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerObjectEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&objects), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        ICorProfilerObjectEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICorProfilerThreadEnum_Impl: ::windows_core::BaseImpl {
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ICorProfilerThreadEnum>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Next(this: &Self::This, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICorProfilerThreadEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorProfilerThreadEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorProfilerThreadEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ids), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        ICorProfilerThreadEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMethodMalloc_Impl: ::windows_core::BaseImpl {
    fn Alloc(this: &Self::This, cb: u32) -> *mut ::core::ffi::c_void;
}
impl ::windows_core::Iids for IMethodMalloc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMethodMalloc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMethodMalloc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Alloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMethodMalloc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cb: u32) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Alloc(this, ::core::mem::transmute_copy(&cb)))
        }
        IMethodMalloc_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Alloc: Alloc::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
