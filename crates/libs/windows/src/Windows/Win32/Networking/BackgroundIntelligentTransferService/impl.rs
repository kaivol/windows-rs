pub trait AsyncIBackgroundCopyCallback_Impl: ::windows_core::BaseImpl {
    fn Begin_JobTransferred(this: &Self::This, pjob: ::core::option::Option<&IBackgroundCopyJob>) -> ::windows_core::Result<()>;
    fn Finish_JobTransferred(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_JobError(this: &Self::This, pjob: ::core::option::Option<&IBackgroundCopyJob>, perror: ::core::option::Option<&IBackgroundCopyError>) -> ::windows_core::Result<()>;
    fn Finish_JobError(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_JobModification(this: &Self::This, pjob: ::core::option::Option<&IBackgroundCopyJob>, dwreserved: u32) -> ::windows_core::Result<()>;
    fn Finish_JobModification(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIBackgroundCopyCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIBackgroundCopyCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_JobTransferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_JobTransferred(this, ::windows_core::from_raw_borrowed(&pjob)).into())
        }
        unsafe extern "system" fn Finish_JobTransferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_JobTransferred(this).into())
        }
        unsafe extern "system" fn Begin_JobError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, perror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_JobError(this, ::windows_core::from_raw_borrowed(&pjob), ::windows_core::from_raw_borrowed(&perror)).into())
        }
        unsafe extern "system" fn Finish_JobError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_JobError(this).into())
        }
        unsafe extern "system" fn Begin_JobModification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_JobModification(this, ::windows_core::from_raw_borrowed(&pjob), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn Finish_JobModification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_JobModification(this).into())
        }
        AsyncIBackgroundCopyCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_JobTransferred: Begin_JobTransferred::<Identity, Impl, OFFSET>,
            Finish_JobTransferred: Finish_JobTransferred::<Identity, Impl, OFFSET>,
            Begin_JobError: Begin_JobError::<Identity, Impl, OFFSET>,
            Finish_JobError: Finish_JobError::<Identity, Impl, OFFSET>,
            Begin_JobModification: Begin_JobModification::<Identity, Impl, OFFSET>,
            Finish_JobModification: Finish_JobModification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBITSExtensionSetup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn EnableBITSUploads(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableBITSUploads(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCleanupTaskName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCleanupTask(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IBITSExtensionSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBITSExtensionSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableBITSUploads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableBITSUploads(this).into())
        }
        unsafe extern "system" fn DisableBITSUploads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableBITSUploads(this).into())
        }
        unsafe extern "system" fn GetCleanupTaskName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCleanupTaskName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptaskname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCleanupTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBITSExtensionSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCleanupTask(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBITSExtensionSetup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableBITSUploads: EnableBITSUploads::<Identity, Impl, OFFSET>,
            DisableBITSUploads: DisableBITSUploads::<Identity, Impl, OFFSET>,
            GetCleanupTaskName: GetCleanupTaskName::<Identity, Impl, OFFSET>,
            GetCleanupTask: GetCleanupTask::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBITSExtensionSetupFactory_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetObject(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<IBITSExtensionSetup>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IBITSExtensionSetupFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBITSExtensionSetupFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBITSExtensionSetupFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBITSExtensionSetupFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppextensionsetup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this, ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppextensionsetup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBITSExtensionSetupFactory_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyCallback_Impl: ::windows_core::BaseImpl {
    fn JobTransferred(this: &Self::This, pjob: ::core::option::Option<&IBackgroundCopyJob>) -> ::windows_core::Result<()>;
    fn JobError(this: &Self::This, pjob: ::core::option::Option<&IBackgroundCopyJob>, perror: ::core::option::Option<&IBackgroundCopyError>) -> ::windows_core::Result<()>;
    fn JobModification(this: &Self::This, pjob: ::core::option::Option<&IBackgroundCopyJob>, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundCopyCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn JobTransferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JobTransferred(this, ::windows_core::from_raw_borrowed(&pjob)).into())
        }
        unsafe extern "system" fn JobError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, perror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JobError(this, ::windows_core::from_raw_borrowed(&pjob), ::windows_core::from_raw_borrowed(&perror)).into())
        }
        unsafe extern "system" fn JobModification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JobModification(this, ::windows_core::from_raw_borrowed(&pjob), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IBackgroundCopyCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            JobTransferred: JobTransferred::<Identity, Impl, OFFSET>,
            JobError: JobError::<Identity, Impl, OFFSET>,
            JobModification: JobModification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyCallback1_Impl: ::windows_core::BaseImpl {
    fn OnStatus(this: &Self::This, pgroup: ::core::option::Option<&IBackgroundCopyGroup>, pjob: ::core::option::Option<&IBackgroundCopyJob1>, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows_core::Result<()>;
    fn OnProgress(this: &Self::This, progresstype: u32, pgroup: ::core::option::Option<&IBackgroundCopyGroup>, pjob: ::core::option::Option<&IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32) -> ::windows_core::Result<()>;
    fn OnProgressEx(this: &Self::This, progresstype: u32, pgroup: ::core::option::Option<&IBackgroundCopyGroup>, pjob: ::core::option::Option<&IBackgroundCopyJob1>, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundCopyCallback1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyCallback1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroup: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatus(this, ::windows_core::from_raw_borrowed(&pgroup), ::windows_core::from_raw_borrowed(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute_copy(&dwnumofretries), ::core::mem::transmute_copy(&dwwin32result), ::core::mem::transmute_copy(&dwtransportresult)).into())
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&progresstype), ::windows_core::from_raw_borrowed(&pgroup), ::windows_core::from_raw_borrowed(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwprogressvalue)).into())
        }
        unsafe extern "system" fn OnProgressEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgressEx(this, ::core::mem::transmute_copy(&progresstype), ::windows_core::from_raw_borrowed(&pgroup), ::windows_core::from_raw_borrowed(&pjob), ::core::mem::transmute_copy(&dwfileindex), ::core::mem::transmute_copy(&dwprogressvalue), ::core::mem::transmute_copy(&dwbytearraysize), ::core::mem::transmute_copy(&pbyte)).into())
        }
        IBackgroundCopyCallback1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStatus: OnStatus::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnProgressEx: OnProgressEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyCallback2_Impl: ::windows_core::BaseImpl + IBackgroundCopyCallback_Impl {
    fn FileTransferred(this: &Self::This, pjob: ::core::option::Option<&IBackgroundCopyJob>, pfile: ::core::option::Option<&IBackgroundCopyFile>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundCopyCallback2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyCallback);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyCallback2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FileTransferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, pfile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FileTransferred(this, ::windows_core::from_raw_borrowed(&pjob), ::windows_core::from_raw_borrowed(&pfile)).into())
        }
        IBackgroundCopyCallback2_Vtbl {
            base__: <IBackgroundCopyCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FileTransferred: FileTransferred::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyCallback3_Impl: ::windows_core::BaseImpl + IBackgroundCopyCallback2_Impl {
    fn FileRangesTransferred(this: &Self::This, job: ::core::option::Option<&IBackgroundCopyJob>, file: ::core::option::Option<&IBackgroundCopyFile>, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundCopyCallback3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyCallback2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyCallback3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FileRangesTransferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyCallback3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, job: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FileRangesTransferred(this, ::windows_core::from_raw_borrowed(&job), ::windows_core::from_raw_borrowed(&file), ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into())
        }
        IBackgroundCopyCallback3_Vtbl {
            base__: <IBackgroundCopyCallback2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FileRangesTransferred: FileRangesTransferred::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyError_Impl: ::windows_core::BaseImpl {
    fn GetError(this: &Self::This, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetFile(this: &Self::This) -> ::windows_core::Result<IBackgroundCopyFile>;
    fn GetErrorDescription(this: &Self::This, languageid: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetErrorContextDescription(this: &Self::This, languageid: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetProtocol(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IBackgroundCopyError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetError(this, ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&pcode)).into())
        }
        unsafe extern "system" fn GetFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: u32, perrordescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorDescription(this, ::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrordescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorContextDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: u32, pcontextdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorContextDescription(this, ::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontextdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocol: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProtocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprotocol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyError_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetError: GetError::<Identity, Impl, OFFSET>,
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            GetErrorContextDescription: GetErrorContextDescription::<Identity, Impl, OFFSET>,
            GetProtocol: GetProtocol::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile_Impl: ::windows_core::BaseImpl {
    fn GetRemoteName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetLocalName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetProgress(this: &Self::This, pval: *mut BG_FILE_PROGRESS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRemoteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRemoteName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProgress(this, ::core::mem::transmute_copy(&pval)).into())
        }
        IBackgroundCopyFile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRemoteName: GetRemoteName::<Identity, Impl, OFFSET>,
            GetLocalName: GetLocalName::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile2_Impl: ::windows_core::BaseImpl + IBackgroundCopyFile_Impl {
    fn GetFileRanges(this: &Self::This, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows_core::Result<()>;
    fn SetRemoteName(this: &Self::This, val: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyFile2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyFile);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyFile2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFileRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileRanges(this, ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into())
        }
        unsafe extern "system" fn SetRemoteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteName(this, ::core::mem::transmute(&val)).into())
        }
        IBackgroundCopyFile2_Vtbl {
            base__: <IBackgroundCopyFile as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFileRanges: GetFileRanges::<Identity, Impl, OFFSET>,
            SetRemoteName: SetRemoteName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile3_Impl: ::windows_core::BaseImpl + IBackgroundCopyFile2_Impl {
    fn GetTemporaryName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetValidationState(this: &Self::This, state: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetValidationState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDownloadedFromPeer(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyFile3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyFile2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyFile3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTemporaryName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTemporaryName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValidationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValidationState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn GetValidationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValidationState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDownloadedFromPeer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyFile3_Vtbl {
            base__: <IBackgroundCopyFile2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTemporaryName: GetTemporaryName::<Identity, Impl, OFFSET>,
            SetValidationState: SetValidationState::<Identity, Impl, OFFSET>,
            GetValidationState: GetValidationState::<Identity, Impl, OFFSET>,
            IsDownloadedFromPeer: IsDownloadedFromPeer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile4_Impl: ::windows_core::BaseImpl + IBackgroundCopyFile3_Impl {
    fn GetPeerDownloadStats(this: &Self::This, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyFile4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyFile3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyFile4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPeerDownloadStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPeerDownloadStats(this, ::core::mem::transmute_copy(&pfromorigin), ::core::mem::transmute_copy(&pfrompeers)).into())
        }
        IBackgroundCopyFile4_Vtbl {
            base__: <IBackgroundCopyFile3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPeerDownloadStats: GetPeerDownloadStats::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile5_Impl: ::windows_core::BaseImpl + IBackgroundCopyFile4_Impl {
    fn SetProperty(this: &Self::This, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: &BITS_FILE_PROPERTY_VALUE) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, propertyid: BITS_FILE_PROPERTY_ID) -> ::windows_core::Result<BITS_FILE_PROPERTY_VALUE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyFile5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyFile4);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyFile5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&propertyvalue)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyFile5_Vtbl {
            base__: <IBackgroundCopyFile4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyFile6_Impl: ::windows_core::BaseImpl + IBackgroundCopyFile5_Impl {
    fn UpdateDownloadPosition(this: &Self::This, offset: u64) -> ::windows_core::Result<()>;
    fn RequestFileRanges(this: &Self::This, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows_core::Result<()>;
    fn GetFilledFileRanges(this: &Self::This, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyFile6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyFile5);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyFile6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateDownloadPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateDownloadPosition(this, ::core::mem::transmute_copy(&offset)).into())
        }
        unsafe extern "system" fn RequestFileRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestFileRanges(this, ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into())
        }
        unsafe extern "system" fn GetFilledFileRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyFile6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilledFileRanges(this, ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into())
        }
        IBackgroundCopyFile6_Vtbl {
            base__: <IBackgroundCopyFile5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateDownloadPosition: UpdateDownloadPosition::<Identity, Impl, OFFSET>,
            RequestFileRanges: RequestFileRanges::<Identity, Impl, OFFSET>,
            GetFilledFileRanges: GetFilledFileRanges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBackgroundCopyGroup_Impl: ::windows_core::BaseImpl {
    fn GetProp(this: &Self::This, propid: GROUPPROP) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProp(this: &Self::This, propid: GROUPPROP, pvarval: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetProgress(this: &Self::This, dwflags: u32) -> ::windows_core::Result<u32>;
    fn GetStatus(this: &Self::This, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows_core::Result<()>;
    fn GetJob(this: &Self::This, jobid: &::windows_core::GUID) -> ::windows_core::Result<IBackgroundCopyJob1>;
    fn SuspendGroup(this: &Self::This) -> ::windows_core::Result<()>;
    fn ResumeGroup(this: &Self::This) -> ::windows_core::Result<()>;
    fn CancelGroup(this: &Self::This) -> ::windows_core::Result<()>;
    fn Size(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GroupID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn CreateJob(this: &Self::This, guidjobid: &::windows_core::GUID) -> ::windows_core::Result<IBackgroundCopyJob1>;
    fn EnumJobs(this: &Self::This, dwflags: u32) -> ::windows_core::Result<IEnumBackgroundCopyJobs1>;
    fn SwitchToForeground(this: &Self::This) -> ::windows_core::Result<()>;
    fn QueryNewJobInterface(this: &Self::This, iid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetNotificationPointer(this: &Self::This, iid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IBackgroundCopyGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProp(this, ::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProp(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&pvarval)).into())
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProgress(this, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwprogress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwjobindex)).into())
        }
        unsafe extern "system" fn GetJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobid: ::windows_core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJob(this, ::core::mem::transmute(&jobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SuspendGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspendGroup(this).into())
        }
        unsafe extern "system" fn ResumeGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeGroup(this).into())
        }
        unsafe extern "system" fn CancelGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelGroup(this).into())
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GroupID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidgroupid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GroupID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidgroupid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidjobid: ::windows_core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateJob(this, ::core::mem::transmute(&guidjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumjobs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumJobs(this, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumjobs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SwitchToForeground<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchToForeground(this).into())
        }
        unsafe extern "system" fn QueryNewJobInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryNewJobInterface(this, ::core::mem::transmute_copy(&iid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotificationPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotificationPointer(this, ::core::mem::transmute_copy(&iid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        IBackgroundCopyGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProp: GetProp::<Identity, Impl, OFFSET>,
            SetProp: SetProp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
            SuspendGroup: SuspendGroup::<Identity, Impl, OFFSET>,
            ResumeGroup: ResumeGroup::<Identity, Impl, OFFSET>,
            CancelGroup: CancelGroup::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            GroupID: GroupID::<Identity, Impl, OFFSET>,
            CreateJob: CreateJob::<Identity, Impl, OFFSET>,
            EnumJobs: EnumJobs::<Identity, Impl, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, Impl, OFFSET>,
            QueryNewJobInterface: QueryNewJobInterface::<Identity, Impl, OFFSET>,
            SetNotificationPointer: SetNotificationPointer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob_Impl: ::windows_core::BaseImpl {
    fn AddFileSet(this: &Self::This, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows_core::Result<()>;
    fn AddFile(this: &Self::This, remoteurl: &::windows_core::PCWSTR, localname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumFiles(this: &Self::This) -> ::windows_core::Result<IEnumBackgroundCopyFiles>;
    fn Suspend(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Complete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<BG_JOB_TYPE>;
    fn GetProgress(this: &Self::This, pval: *mut BG_JOB_PROGRESS) -> ::windows_core::Result<()>;
    fn GetTimes(this: &Self::This, pval: *mut BG_JOB_TIMES) -> ::windows_core::Result<()>;
    fn GetState(this: &Self::This) -> ::windows_core::Result<BG_JOB_STATE>;
    fn GetError(this: &Self::This) -> ::windows_core::Result<IBackgroundCopyError>;
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDisplayName(this: &Self::This, val: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDescription(this: &Self::This, val: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetPriority(this: &Self::This, val: BG_JOB_PRIORITY) -> ::windows_core::Result<()>;
    fn GetPriority(this: &Self::This) -> ::windows_core::Result<BG_JOB_PRIORITY>;
    fn SetNotifyFlags(this: &Self::This, val: u32) -> ::windows_core::Result<()>;
    fn GetNotifyFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetNotifyInterface(this: &Self::This, val: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetNotifyInterface(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetMinimumRetryDelay(this: &Self::This, seconds: u32) -> ::windows_core::Result<()>;
    fn GetMinimumRetryDelay(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetNoProgressTimeout(this: &Self::This, seconds: u32) -> ::windows_core::Result<()>;
    fn GetNoProgressTimeout(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetErrorCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetProxySettings(this: &Self::This, proxyusage: BG_JOB_PROXY_USAGE, proxylist: &::windows_core::PCWSTR, proxybypasslist: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetProxySettings(this: &Self::This, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows_core::PWSTR, pproxybypasslist: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn TakeOwnership(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddFileSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFileSet(this, ::core::mem::transmute_copy(&cfilecount), ::core::mem::transmute_copy(&pfileset)).into())
        }
        unsafe extern "system" fn AddFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteurl: ::windows_core::PCWSTR, localname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFile(this, ::core::mem::transmute(&remoteurl), ::core::mem::transmute(&localname)).into())
        }
        unsafe extern "system" fn EnumFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Suspend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Suspend(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Complete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Complete(this).into())
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProgress(this, ::core::mem::transmute_copy(&pval)).into())
        }
        unsafe extern "system" fn GetTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTimes(this, ::core::mem::transmute_copy(&pval)).into())
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: BG_JOB_PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&val)).into())
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotifyFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyFlags(this, ::core::mem::transmute_copy(&val)).into())
        }
        unsafe extern "system" fn GetNotifyFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNotifyFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotifyInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyInterface(this, ::windows_core::from_raw_borrowed(&val)).into())
        }
        unsafe extern "system" fn GetNotifyInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNotifyInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinimumRetryDelay(this, ::core::mem::transmute_copy(&seconds)).into())
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinimumRetryDelay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(seconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNoProgressTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNoProgressTimeout(this, ::core::mem::transmute_copy(&seconds)).into())
        }
        unsafe extern "system" fn GetNoProgressTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNoProgressTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(seconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errors: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: ::windows_core::PCWSTR, proxybypasslist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxySettings(this, ::core::mem::transmute_copy(&proxyusage), ::core::mem::transmute(&proxylist), ::core::mem::transmute(&proxybypasslist)).into())
        }
        unsafe extern "system" fn GetProxySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows_core::PWSTR, pproxybypasslist: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProxySettings(this, ::core::mem::transmute_copy(&pproxyusage), ::core::mem::transmute_copy(&pproxylist), ::core::mem::transmute_copy(&pproxybypasslist)).into())
        }
        unsafe extern "system" fn TakeOwnership<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TakeOwnership(this).into())
        }
        IBackgroundCopyJob_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddFileSet: AddFileSet::<Identity, Impl, OFFSET>,
            AddFile: AddFile::<Identity, Impl, OFFSET>,
            EnumFiles: EnumFiles::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Complete: Complete::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetTimes: GetTimes::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetError: GetError::<Identity, Impl, OFFSET>,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Identity, Impl, OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Identity, Impl, OFFSET>,
            SetNotifyInterface: SetNotifyInterface::<Identity, Impl, OFFSET>,
            GetNotifyInterface: GetNotifyInterface::<Identity, Impl, OFFSET>,
            SetMinimumRetryDelay: SetMinimumRetryDelay::<Identity, Impl, OFFSET>,
            GetMinimumRetryDelay: GetMinimumRetryDelay::<Identity, Impl, OFFSET>,
            SetNoProgressTimeout: SetNoProgressTimeout::<Identity, Impl, OFFSET>,
            GetNoProgressTimeout: GetNoProgressTimeout::<Identity, Impl, OFFSET>,
            GetErrorCount: GetErrorCount::<Identity, Impl, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, Impl, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyJob1_Impl: ::windows_core::BaseImpl {
    fn CancelJob(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProgress(this: &Self::This, dwflags: u32) -> ::windows_core::Result<u32>;
    fn GetStatus(this: &Self::This, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows_core::Result<()>;
    fn AddFiles(this: &Self::This, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows_core::Result<()>;
    fn GetFile(this: &Self::This, cfileindex: u32) -> ::windows_core::Result<FILESETINFO>;
    fn GetFileCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SwitchToForeground(this: &Self::This) -> ::windows_core::Result<()>;
    fn JobID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IBackgroundCopyJob1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJob1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CancelJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelJob(this).into())
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProgress(this, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwprogress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pdwwin32result), ::core::mem::transmute_copy(&pdwtransportresult), ::core::mem::transmute_copy(&pdwnumofretries)).into())
        }
        unsafe extern "system" fn AddFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFiles(this, ::core::mem::transmute_copy(&cfilecount), ::core::mem::transmute_copy(&ppfileset)).into())
        }
        unsafe extern "system" fn GetFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFile(this, ::core::mem::transmute_copy(&cfileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfileinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfilecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwfilecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SwitchToForeground<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchToForeground(this).into())
        }
        unsafe extern "system" fn JobID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidjobid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JobID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidjobid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyJob1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CancelJob: CancelJob::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            AddFiles: AddFiles::<Identity, Impl, OFFSET>,
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetFileCount: GetFileCount::<Identity, Impl, OFFSET>,
            SwitchToForeground: SwitchToForeground::<Identity, Impl, OFFSET>,
            JobID: JobID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob2_Impl: ::windows_core::BaseImpl + IBackgroundCopyJob_Impl {
    fn SetNotifyCmdLine(this: &Self::This, program: &::windows_core::PCWSTR, parameters: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetNotifyCmdLine(this: &Self::This, pprogram: *mut ::windows_core::PWSTR, pparameters: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetReplyProgress(this: &Self::This, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows_core::Result<()>;
    fn GetReplyData(this: &Self::This, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows_core::Result<()>;
    fn SetReplyFileName(this: &Self::This, replyfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetReplyFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetCredentials(this: &Self::This, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows_core::Result<()>;
    fn RemoveCredentials(this: &Self::This, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyJob2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyJob);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJob2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNotifyCmdLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, program: ::windows_core::PCWSTR, parameters: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyCmdLine(this, ::core::mem::transmute(&program), ::core::mem::transmute(&parameters)).into())
        }
        unsafe extern "system" fn GetNotifyCmdLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprogram: *mut ::windows_core::PWSTR, pparameters: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNotifyCmdLine(this, ::core::mem::transmute_copy(&pprogram), ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn GetReplyProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReplyProgress(this, ::core::mem::transmute_copy(&pprogress)).into())
        }
        unsafe extern "system" fn GetReplyData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReplyData(this, ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&plength)).into())
        }
        unsafe extern "system" fn SetReplyFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, replyfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReplyFileName(this, ::core::mem::transmute(&replyfilename)).into())
        }
        unsafe extern "system" fn GetReplyFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preplyfilename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReplyFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preplyfilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentials(this, ::core::mem::transmute_copy(&credentials)).into())
        }
        unsafe extern "system" fn RemoveCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveCredentials(this, ::core::mem::transmute_copy(&target), ::core::mem::transmute_copy(&scheme)).into())
        }
        IBackgroundCopyJob2_Vtbl {
            base__: <IBackgroundCopyJob as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNotifyCmdLine: SetNotifyCmdLine::<Identity, Impl, OFFSET>,
            GetNotifyCmdLine: GetNotifyCmdLine::<Identity, Impl, OFFSET>,
            GetReplyProgress: GetReplyProgress::<Identity, Impl, OFFSET>,
            GetReplyData: GetReplyData::<Identity, Impl, OFFSET>,
            SetReplyFileName: SetReplyFileName::<Identity, Impl, OFFSET>,
            GetReplyFileName: GetReplyFileName::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            RemoveCredentials: RemoveCredentials::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob3_Impl: ::windows_core::BaseImpl + IBackgroundCopyJob2_Impl {
    fn ReplaceRemotePrefix(this: &Self::This, oldprefix: &::windows_core::PCWSTR, newprefix: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddFileWithRanges(this: &Self::This, remoteurl: &::windows_core::PCWSTR, localname: &::windows_core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows_core::Result<()>;
    fn SetFileACLFlags(this: &Self::This, flags: u32) -> ::windows_core::Result<()>;
    fn GetFileACLFlags(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyJob3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyJob2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJob3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReplaceRemotePrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldprefix: ::windows_core::PCWSTR, newprefix: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplaceRemotePrefix(this, ::core::mem::transmute(&oldprefix), ::core::mem::transmute(&newprefix)).into())
        }
        unsafe extern "system" fn AddFileWithRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteurl: ::windows_core::PCWSTR, localname: ::windows_core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFileWithRanges(this, ::core::mem::transmute(&remoteurl), ::core::mem::transmute(&localname), ::core::mem::transmute_copy(&rangecount), ::core::mem::transmute_copy(&ranges)).into())
        }
        unsafe extern "system" fn SetFileACLFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileACLFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetFileACLFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileACLFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyJob3_Vtbl {
            base__: <IBackgroundCopyJob2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReplaceRemotePrefix: ReplaceRemotePrefix::<Identity, Impl, OFFSET>,
            AddFileWithRanges: AddFileWithRanges::<Identity, Impl, OFFSET>,
            SetFileACLFlags: SetFileACLFlags::<Identity, Impl, OFFSET>,
            GetFileACLFlags: GetFileACLFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob4_Impl: ::windows_core::BaseImpl + IBackgroundCopyJob3_Impl {
    fn SetPeerCachingFlags(this: &Self::This, flags: u32) -> ::windows_core::Result<()>;
    fn GetPeerCachingFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetOwnerIntegrityLevel(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetOwnerElevationState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetMaximumDownloadTime(this: &Self::This, timeout: u32) -> ::windows_core::Result<()>;
    fn GetMaximumDownloadTime(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyJob4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyJob3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJob4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPeerCachingFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPeerCachingFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetPeerCachingFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPeerCachingFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwnerIntegrityLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOwnerElevationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwnerElevationState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pelevated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumDownloadTime(this, ::core::mem::transmute_copy(&timeout)).into())
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumDownloadTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyJob4_Vtbl {
            base__: <IBackgroundCopyJob3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPeerCachingFlags: SetPeerCachingFlags::<Identity, Impl, OFFSET>,
            GetPeerCachingFlags: GetPeerCachingFlags::<Identity, Impl, OFFSET>,
            GetOwnerIntegrityLevel: GetOwnerIntegrityLevel::<Identity, Impl, OFFSET>,
            GetOwnerElevationState: GetOwnerElevationState::<Identity, Impl, OFFSET>,
            SetMaximumDownloadTime: SetMaximumDownloadTime::<Identity, Impl, OFFSET>,
            GetMaximumDownloadTime: GetMaximumDownloadTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBackgroundCopyJob5_Impl: ::windows_core::BaseImpl + IBackgroundCopyJob4_Impl {
    fn SetProperty(this: &Self::This, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: &BITS_JOB_PROPERTY_VALUE) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, propertyid: BITS_JOB_PROPERTY_ID) -> ::windows_core::Result<BITS_JOB_PROPERTY_VALUE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBackgroundCopyJob5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyJob4);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJob5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&propertyvalue)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJob5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyJob5_Vtbl {
            base__: <IBackgroundCopyJob4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyJobHttpOptions_Impl: ::windows_core::BaseImpl {
    fn SetClientCertificateByID(this: &Self::This, storelocation: BG_CERT_STORE_LOCATION, storename: &::windows_core::PCWSTR, pcerthashblob: *const u8) -> ::windows_core::Result<()>;
    fn SetClientCertificateByName(this: &Self::This, storelocation: BG_CERT_STORE_LOCATION, storename: &::windows_core::PCWSTR, subjectname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoveClientCertificate(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetClientCertificate(this: &Self::This, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut ::windows_core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetCustomHeaders(this: &Self::This, requestheaders: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCustomHeaders(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSecurityFlags(this: &Self::This, flags: u32) -> ::windows_core::Result<()>;
    fn GetSecurityFlags(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IBackgroundCopyJobHttpOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJobHttpOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetClientCertificateByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: ::windows_core::PCWSTR, pcerthashblob: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCertificateByID(this, ::core::mem::transmute_copy(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute_copy(&pcerthashblob)).into())
        }
        unsafe extern "system" fn SetClientCertificateByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: ::windows_core::PCWSTR, subjectname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCertificateByName(this, ::core::mem::transmute_copy(&storelocation), ::core::mem::transmute(&storename), ::core::mem::transmute(&subjectname)).into())
        }
        unsafe extern "system" fn RemoveClientCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveClientCertificate(this).into())
        }
        unsafe extern "system" fn GetClientCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut ::windows_core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClientCertificate(this, ::core::mem::transmute_copy(&pstorelocation), ::core::mem::transmute_copy(&pstorename), ::core::mem::transmute_copy(&ppcerthashblob), ::core::mem::transmute_copy(&psubjectname)).into())
        }
        unsafe extern "system" fn SetCustomHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCustomHeaders(this, ::core::mem::transmute(&requestheaders)).into())
        }
        unsafe extern "system" fn GetCustomHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequestheaders: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn GetSecurityFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyJobHttpOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetClientCertificateByID: SetClientCertificateByID::<Identity, Impl, OFFSET>,
            SetClientCertificateByName: SetClientCertificateByName::<Identity, Impl, OFFSET>,
            RemoveClientCertificate: RemoveClientCertificate::<Identity, Impl, OFFSET>,
            GetClientCertificate: GetClientCertificate::<Identity, Impl, OFFSET>,
            SetCustomHeaders: SetCustomHeaders::<Identity, Impl, OFFSET>,
            GetCustomHeaders: GetCustomHeaders::<Identity, Impl, OFFSET>,
            SetSecurityFlags: SetSecurityFlags::<Identity, Impl, OFFSET>,
            GetSecurityFlags: GetSecurityFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyJobHttpOptions2_Impl: ::windows_core::BaseImpl + IBackgroundCopyJobHttpOptions_Impl {
    fn SetHttpMethod(this: &Self::This, method: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetHttpMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IBackgroundCopyJobHttpOptions2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyJobHttpOptions);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJobHttpOptions2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHttpMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHttpMethod(this, ::core::mem::transmute(&method)).into())
        }
        unsafe extern "system" fn GetHttpMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHttpMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(method, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyJobHttpOptions2_Vtbl {
            base__: <IBackgroundCopyJobHttpOptions as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetHttpMethod: SetHttpMethod::<Identity, Impl, OFFSET>,
            GetHttpMethod: GetHttpMethod::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyJobHttpOptions3_Impl: ::windows_core::BaseImpl + IBackgroundCopyJobHttpOptions2_Impl {
    fn SetServerCertificateValidationInterface(this: &Self::This, certvalidationcallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn MakeCustomHeadersWriteOnly(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundCopyJobHttpOptions3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBackgroundCopyJobHttpOptions2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyJobHttpOptions3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certvalidationcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerCertificateValidationInterface(this, ::windows_core::from_raw_borrowed(&certvalidationcallback)).into())
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeCustomHeadersWriteOnly(this).into())
        }
        IBackgroundCopyJobHttpOptions3_Vtbl {
            base__: <IBackgroundCopyJobHttpOptions2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetServerCertificateValidationInterface: SetServerCertificateValidationInterface::<Identity, Impl, OFFSET>,
            MakeCustomHeadersWriteOnly: MakeCustomHeadersWriteOnly::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyManager_Impl: ::windows_core::BaseImpl {
    fn CreateJob(this: &Self::This, displayname: &::windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows_core::GUID, ppjob: *mut ::core::option::Option<IBackgroundCopyJob>) -> ::windows_core::Result<()>;
    fn GetJob(this: &Self::This, jobid: *const ::windows_core::GUID) -> ::windows_core::Result<IBackgroundCopyJob>;
    fn EnumJobs(this: &Self::This, dwflags: u32) -> ::windows_core::Result<IEnumBackgroundCopyJobs>;
    fn GetErrorDescription(this: &Self::This, hresult: ::windows_core::HRESULT, languageid: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IBackgroundCopyManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayname: ::windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows_core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateJob(this, ::core::mem::transmute(&displayname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pjobid), ::core::mem::transmute_copy(&ppjob)).into())
        }
        unsafe extern "system" fn GetJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobid: *const ::windows_core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetJob(this, ::core::mem::transmute_copy(&jobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppjob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumJobs(this, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, languageid: u32, perrordescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorDescription(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrordescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateJob: CreateJob::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
            EnumJobs: EnumJobs::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyQMgr_Impl: ::windows_core::BaseImpl {
    fn CreateGroup(this: &Self::This, guidgroupid: &::windows_core::GUID) -> ::windows_core::Result<IBackgroundCopyGroup>;
    fn GetGroup(this: &Self::This, groupid: &::windows_core::GUID) -> ::windows_core::Result<IBackgroundCopyGroup>;
    fn EnumGroups(this: &Self::This, dwflags: u32) -> ::windows_core::Result<IEnumBackgroundCopyGroups>;
}
impl ::windows_core::Iids for IBackgroundCopyQMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyQMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidgroupid: ::windows_core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGroup(this, ::core::mem::transmute(&guidgroupid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, groupid: ::windows_core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGroup(this, ::core::mem::transmute(&groupid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyQMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumGroups(this, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBackgroundCopyQMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateGroup: CreateGroup::<Identity, Impl, OFFSET>,
            GetGroup: GetGroup::<Identity, Impl, OFFSET>,
            EnumGroups: EnumGroups::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBackgroundCopyServerCertificateValidationCallback_Impl: ::windows_core::BaseImpl {
    fn ValidateServerCertificate(this: &Self::This, job: ::core::option::Option<&IBackgroundCopyJob>, file: ::core::option::Option<&IBackgroundCopyFile>, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBackgroundCopyServerCertificateValidationCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBackgroundCopyServerCertificateValidationCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ValidateServerCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, job: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateServerCertificate(this, ::windows_core::from_raw_borrowed(&job), ::windows_core::from_raw_borrowed(&file), ::core::mem::transmute_copy(&certlength), ::core::mem::transmute_copy(&certdata), ::core::mem::transmute_copy(&certencodingtype), ::core::mem::transmute_copy(&certstorelength), ::core::mem::transmute_copy(&certstoredata)).into())
        }
        IBackgroundCopyServerCertificateValidationCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ValidateServerCertificate: ValidateServerCertificate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeer_Impl: ::windows_core::BaseImpl {
    fn GetPeerName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsAuthenticated(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsAvailable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBitsPeer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBitsPeer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPeerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPeerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAuthenticated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pauth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAvailable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ponline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBitsPeer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPeerName: GetPeerName::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBitsPeerCacheAdministration_Impl: ::windows_core::BaseImpl {
    fn GetMaximumCacheSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaximumCacheSize(this: &Self::This, bytes: u32) -> ::windows_core::Result<()>;
    fn GetMaximumContentAge(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaximumContentAge(this: &Self::This, seconds: u32) -> ::windows_core::Result<()>;
    fn GetConfigurationFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetConfigurationFlags(this: &Self::This, flags: u32) -> ::windows_core::Result<()>;
    fn EnumRecords(this: &Self::This) -> ::windows_core::Result<IEnumBitsPeerCacheRecords>;
    fn GetRecord(this: &Self::This, id: *const ::windows_core::GUID) -> ::windows_core::Result<IBitsPeerCacheRecord>;
    fn ClearRecords(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeleteRecord(this: &Self::This, id: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DeleteUrl(this: &Self::This, url: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumPeers(this: &Self::This) -> ::windows_core::Result<IEnumBitsPeers>;
    fn ClearPeers(this: &Self::This) -> ::windows_core::Result<()>;
    fn DiscoverPeers(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBitsPeerCacheAdministration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBitsPeerCacheAdministration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaximumCacheSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumCacheSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaximumCacheSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bytes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumCacheSize(this, ::core::mem::transmute_copy(&bytes)).into())
        }
        unsafe extern "system" fn GetMaximumContentAge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pseconds: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumContentAge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pseconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaximumContentAge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumContentAge(this, ::core::mem::transmute_copy(&seconds)).into())
        }
        unsafe extern "system" fn GetConfigurationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConfigurationFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConfigurationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConfigurationFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn EnumRecords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRecords(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *const ::windows_core::GUID, pprecord: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecord(this, ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClearRecords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRecords(this).into())
        }
        unsafe extern "system" fn DeleteRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRecord(this, ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn DeleteUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, url: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteUrl(this, ::core::mem::transmute(&url)).into())
        }
        unsafe extern "system" fn EnumPeers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumPeers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClearPeers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearPeers(this).into())
        }
        unsafe extern "system" fn DiscoverPeers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheAdministration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscoverPeers(this).into())
        }
        IBitsPeerCacheAdministration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaximumCacheSize: GetMaximumCacheSize::<Identity, Impl, OFFSET>,
            SetMaximumCacheSize: SetMaximumCacheSize::<Identity, Impl, OFFSET>,
            GetMaximumContentAge: GetMaximumContentAge::<Identity, Impl, OFFSET>,
            SetMaximumContentAge: SetMaximumContentAge::<Identity, Impl, OFFSET>,
            GetConfigurationFlags: GetConfigurationFlags::<Identity, Impl, OFFSET>,
            SetConfigurationFlags: SetConfigurationFlags::<Identity, Impl, OFFSET>,
            EnumRecords: EnumRecords::<Identity, Impl, OFFSET>,
            GetRecord: GetRecord::<Identity, Impl, OFFSET>,
            ClearRecords: ClearRecords::<Identity, Impl, OFFSET>,
            DeleteRecord: DeleteRecord::<Identity, Impl, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, Impl, OFFSET>,
            EnumPeers: EnumPeers::<Identity, Impl, OFFSET>,
            ClearPeers: ClearPeers::<Identity, Impl, OFFSET>,
            DiscoverPeers: DiscoverPeers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IBitsPeerCacheRecord_Impl: ::windows_core::BaseImpl {
    fn GetId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetOriginUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFileSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetFileModificationTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetLastAccessTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsFileValidated(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetFileRanges(this: &Self::This, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IBitsPeerCacheRecord {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBitsPeerCacheRecord {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOriginUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOriginUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileModificationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileModificationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastAccessTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastAccessTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFileValidated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsFileValidated(this).into())
        }
        unsafe extern "system" fn GetFileRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsPeerCacheRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileRanges(this, ::core::mem::transmute_copy(&prangecount), ::core::mem::transmute_copy(&ppranges)).into())
        }
        IBitsPeerCacheRecord_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetOriginUrl: GetOriginUrl::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            GetFileModificationTime: GetFileModificationTime::<Identity, Impl, OFFSET>,
            GetLastAccessTime: GetLastAccessTime::<Identity, Impl, OFFSET>,
            IsFileValidated: IsFileValidated::<Identity, Impl, OFFSET>,
            GetFileRanges: GetFileRanges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IBitsTokenOptions_Impl: ::windows_core::BaseImpl {
    fn SetHelperTokenFlags(this: &Self::This, usageflags: BG_TOKEN) -> ::windows_core::Result<()>;
    fn GetHelperTokenFlags(this: &Self::This) -> ::windows_core::Result<BG_TOKEN>;
    fn SetHelperToken(this: &Self::This) -> ::windows_core::Result<()>;
    fn ClearHelperToken(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetHelperTokenSid(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IBitsTokenOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBitsTokenOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHelperTokenFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usageflags: BG_TOKEN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelperTokenFlags(this, ::core::mem::transmute_copy(&usageflags)).into())
        }
        unsafe extern "system" fn GetHelperTokenFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut BG_TOKEN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHelperTokenFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHelperToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelperToken(this).into())
        }
        unsafe extern "system" fn ClearHelperToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearHelperToken(this).into())
        }
        unsafe extern "system" fn GetHelperTokenSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitsTokenOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHelperTokenSid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBitsTokenOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetHelperTokenFlags: SetHelperTokenFlags::<Identity, Impl, OFFSET>,
            GetHelperTokenFlags: GetHelperTokenFlags::<Identity, Impl, OFFSET>,
            SetHelperToken: SetHelperToken::<Identity, Impl, OFFSET>,
            ClearHelperToken: ClearHelperToken::<Identity, Impl, OFFSET>,
            GetHelperTokenSid: GetHelperTokenSid::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumBackgroundCopyFiles_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyFile>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumBackgroundCopyFiles>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumBackgroundCopyFiles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumBackgroundCopyFiles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyFiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumBackgroundCopyFiles_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumBackgroundCopyGroups_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumBackgroundCopyGroups>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumBackgroundCopyGroups {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumBackgroundCopyGroups {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumBackgroundCopyGroups_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumBackgroundCopyJobs_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyJob>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumBackgroundCopyJobs>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumBackgroundCopyJobs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumBackgroundCopyJobs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumBackgroundCopyJobs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumBackgroundCopyJobs1_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumBackgroundCopyJobs1>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumBackgroundCopyJobs1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumBackgroundCopyJobs1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBackgroundCopyJobs1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumBackgroundCopyJobs1_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumBitsPeerCacheRecords_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumBitsPeerCacheRecords>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumBitsPeerCacheRecords {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumBitsPeerCacheRecords {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeerCacheRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumBitsPeerCacheRecords_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumBitsPeers_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeer>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumBitsPeers>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumBitsPeers {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumBitsPeers {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBitsPeers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumBitsPeers_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
