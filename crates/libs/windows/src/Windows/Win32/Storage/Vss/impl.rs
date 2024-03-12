pub trait IVssAdmin_Impl: ::windows_core::BaseImpl {
    fn RegisterProvider(this: &Self::This, pproviderid: &::windows_core::GUID, classid: &::windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnregisterProvider(this: &Self::This, providerid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn QueryProviders(this: &Self::This) -> ::windows_core::Result<IVssEnumObject>;
    fn AbortAllSnapshotsInProgress(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssAdmin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssAdmin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows_core::GUID, classid: ::windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterProvider(this, ::core::mem::transmute(&pproviderid), ::core::mem::transmute(&classid), ::core::mem::transmute_copy(&pwszprovidername), ::core::mem::transmute_copy(&eprovidertype), ::core::mem::transmute_copy(&pwszproviderversion), ::core::mem::transmute(&providerversionid)).into())
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterProvider(this, ::core::mem::transmute(&providerid)).into())
        }
        unsafe extern "system" fn QueryProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryProviders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AbortAllSnapshotsInProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortAllSnapshotsInProgress(this).into())
        }
        IVssAdmin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterProvider: RegisterProvider::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
            QueryProviders: QueryProviders::<Identity, Impl, OFFSET>,
            AbortAllSnapshotsInProgress: AbortAllSnapshotsInProgress::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssAdminEx_Impl: ::windows_core::BaseImpl + IVssAdmin_Impl {
    fn GetProviderCapability(this: &Self::This, pproviderid: &::windows_core::GUID) -> ::windows_core::Result<u64>;
    fn GetProviderContext(this: &Self::This, providerid: &::windows_core::GUID) -> ::windows_core::Result<i32>;
    fn SetProviderContext(this: &Self::This, providerid: &::windows_core::GUID, lcontext: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssAdminEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IVssAdmin);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssAdminEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProviderCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproviderid: ::windows_core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderCapability(this, ::core::mem::transmute(&pproviderid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plloriginalcapabilitymask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, plcontext: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderContext(this, ::core::mem::transmute(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProviderContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAdminEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, lcontext: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProviderContext(this, ::core::mem::transmute(&providerid), ::core::mem::transmute_copy(&lcontext)).into())
        }
        IVssAdminEx_Vtbl {
            base__: <IVssAdmin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProviderCapability: GetProviderCapability::<Identity, Impl, OFFSET>,
            GetProviderContext: GetProviderContext::<Identity, Impl, OFFSET>,
            SetProviderContext: SetProviderContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssAsync_Impl: ::windows_core::BaseImpl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Wait(this: &Self::This, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn QueryStatus(this: &Self::This, phrresult: *mut ::windows_core::HRESULT, preserved: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&dwmilliseconds)).into())
        }
        unsafe extern "system" fn QueryStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, preserved: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryStatus(this, ::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&preserved)).into())
        }
        IVssAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponent_Impl: ::windows_core::BaseImpl {
    fn GetLogicalPath(this: &Self::This, pbstrpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetComponentType(this: &Self::This, pct: *mut VSS_COMPONENT_TYPE) -> ::windows_core::Result<()>;
    fn GetComponentName(this: &Self::This, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBackupSucceeded(this: &Self::This, pbsucceeded: *mut bool) -> ::windows_core::Result<()>;
    fn GetAlternateLocationMappingCount(this: &Self::This, pcmappings: *mut u32) -> ::windows_core::Result<()>;
    fn GetAlternateLocationMapping(this: &Self::This, imapping: u32) -> ::windows_core::Result<IVssWMFiledesc>;
    fn SetBackupMetadata(this: &Self::This, wszdata: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetBackupMetadata(this: &Self::This, pbstrdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddPartialFile(this: &Self::This, wszpath: &::windows_core::PCWSTR, wszfilename: &::windows_core::PCWSTR, wszranges: &::windows_core::PCWSTR, wszmetadata: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPartialFileCount(this: &Self::This, pcpartialfiles: *mut u32) -> ::windows_core::Result<()>;
    fn GetPartialFile(this: &Self::This, ipartialfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilename: *mut ::windows_core::BSTR, pbstrrange: *mut ::windows_core::BSTR, pbstrmetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsSelectedForRestore(this: &Self::This, pbselectedforrestore: *mut bool) -> ::windows_core::Result<()>;
    fn GetAdditionalRestores(this: &Self::This, pbadditionalrestores: *mut bool) -> ::windows_core::Result<()>;
    fn GetNewTargetCount(this: &Self::This, pcnewtarget: *mut u32) -> ::windows_core::Result<()>;
    fn GetNewTarget(this: &Self::This, inewtarget: u32) -> ::windows_core::Result<IVssWMFiledesc>;
    fn AddDirectedTarget(this: &Self::This, wszsourcepath: &::windows_core::PCWSTR, wszsourcefilename: &::windows_core::PCWSTR, wszsourcerangelist: &::windows_core::PCWSTR, wszdestinationpath: &::windows_core::PCWSTR, wszdestinationfilename: &::windows_core::PCWSTR, wszdestinationrangelist: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDirectedTargetCount(this: &Self::This, pcdirectedtarget: *mut u32) -> ::windows_core::Result<()>;
    fn GetDirectedTarget(this: &Self::This, idirectedtarget: u32, pbstrsourcepath: *mut ::windows_core::BSTR, pbstrsourcefilename: *mut ::windows_core::BSTR, pbstrsourcerangelist: *mut ::windows_core::BSTR, pbstrdestinationpath: *mut ::windows_core::BSTR, pbstrdestinationfilename: *mut ::windows_core::BSTR, pbstrdestinationrangelist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRestoreMetadata(this: &Self::This, wszrestoremetadata: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetRestoreMetadata(this: &Self::This, pbstrrestoremetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetRestoreTarget(this: &Self::This, target: VSS_RESTORE_TARGET) -> ::windows_core::Result<()>;
    fn GetRestoreTarget(this: &Self::This, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows_core::Result<()>;
    fn SetPreRestoreFailureMsg(this: &Self::This, wszprerestorefailuremsg: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPreRestoreFailureMsg(this: &Self::This, pbstrprerestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetPostRestoreFailureMsg(this: &Self::This, wszpostrestorefailuremsg: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPostRestoreFailureMsg(this: &Self::This, pbstrpostrestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetBackupStamp(this: &Self::This, wszbackupstamp: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetBackupStamp(this: &Self::This, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPreviousBackupStamp(this: &Self::This, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBackupOptions(this: &Self::This, pbstrbackupoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetRestoreOptions(this: &Self::This, pbstrrestoreoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetRestoreSubcomponentCount(this: &Self::This, pcrestoresubcomponent: *mut u32) -> ::windows_core::Result<()>;
    fn GetRestoreSubcomponent(this: &Self::This, icomponent: u32, pbstrlogicalpath: *mut ::windows_core::BSTR, pbstrcomponentname: *mut ::windows_core::BSTR, pbrepair: *mut bool) -> ::windows_core::Result<()>;
    fn GetFileRestoreStatus(this: &Self::This, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows_core::Result<()>;
    fn AddDifferencedFilesByLastModifyTime(this: &Self::This, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: &super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn AddDifferencedFilesByLastModifyLSN(this: &Self::This, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDifferencedFilesCount(this: &Self::This, pcdifferencedfiles: *mut u32) -> ::windows_core::Result<()>;
    fn GetDifferencedFile(this: &Self::This, idifferencedfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilespec: *mut ::windows_core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVssComponent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssComponent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLogicalPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLogicalPath(this, ::core::mem::transmute_copy(&pbstrpath)).into())
        }
        unsafe extern "system" fn GetComponentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetComponentType(this, ::core::mem::transmute_copy(&pct)).into())
        }
        unsafe extern "system" fn GetComponentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetComponentName(this, ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn GetBackupSucceeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsucceeded: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBackupSucceeded(this, ::core::mem::transmute_copy(&pbsucceeded)).into())
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcmappings: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAlternateLocationMappingCount(this, ::core::mem::transmute_copy(&pcmappings)).into())
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAlternateLocationMapping(this, ::core::mem::transmute_copy(&imapping)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfiledesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBackupMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszdata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackupMetadata(this, ::core::mem::transmute(&wszdata)).into())
        }
        unsafe extern "system" fn GetBackupMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBackupMetadata(this, ::core::mem::transmute_copy(&pbstrdata)).into())
        }
        unsafe extern "system" fn AddPartialFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilename: ::windows_core::PCWSTR, wszranges: ::windows_core::PCWSTR, wszmetadata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPartialFile(this, ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilename), ::core::mem::transmute(&wszranges), ::core::mem::transmute(&wszmetadata)).into())
        }
        unsafe extern "system" fn GetPartialFileCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcpartialfiles: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPartialFileCount(this, ::core::mem::transmute_copy(&pcpartialfiles)).into())
        }
        unsafe extern "system" fn GetPartialFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrrange: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrmetadata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPartialFile(this, ::core::mem::transmute_copy(&ipartialfile), ::core::mem::transmute_copy(&pbstrpath), ::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pbstrrange), ::core::mem::transmute_copy(&pbstrmetadata)).into())
        }
        unsafe extern "system" fn IsSelectedForRestore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbselectedforrestore: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSelectedForRestore(this, ::core::mem::transmute_copy(&pbselectedforrestore)).into())
        }
        unsafe extern "system" fn GetAdditionalRestores<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbadditionalrestores: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdditionalRestores(this, ::core::mem::transmute_copy(&pbadditionalrestores)).into())
        }
        unsafe extern "system" fn GetNewTargetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnewtarget: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNewTargetCount(this, ::core::mem::transmute_copy(&pcnewtarget)).into())
        }
        unsafe extern "system" fn GetNewTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNewTarget(this, ::core::mem::transmute_copy(&inewtarget)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfiledesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddDirectedTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszsourcepath: ::windows_core::PCWSTR, wszsourcefilename: ::windows_core::PCWSTR, wszsourcerangelist: ::windows_core::PCWSTR, wszdestinationpath: ::windows_core::PCWSTR, wszdestinationfilename: ::windows_core::PCWSTR, wszdestinationrangelist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDirectedTarget(this, ::core::mem::transmute(&wszsourcepath), ::core::mem::transmute(&wszsourcefilename), ::core::mem::transmute(&wszsourcerangelist), ::core::mem::transmute(&wszdestinationpath), ::core::mem::transmute(&wszdestinationfilename), ::core::mem::transmute(&wszdestinationrangelist)).into())
        }
        unsafe extern "system" fn GetDirectedTargetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdirectedtarget: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDirectedTargetCount(this, ::core::mem::transmute_copy(&pcdirectedtarget)).into())
        }
        unsafe extern "system" fn GetDirectedTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrsourcefilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrsourcerangelist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdestinationpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdestinationfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdestinationrangelist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDirectedTarget(this, ::core::mem::transmute_copy(&idirectedtarget), ::core::mem::transmute_copy(&pbstrsourcepath), ::core::mem::transmute_copy(&pbstrsourcefilename), ::core::mem::transmute_copy(&pbstrsourcerangelist), ::core::mem::transmute_copy(&pbstrdestinationpath), ::core::mem::transmute_copy(&pbstrdestinationfilename), ::core::mem::transmute_copy(&pbstrdestinationrangelist)).into())
        }
        unsafe extern "system" fn SetRestoreMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszrestoremetadata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRestoreMetadata(this, ::core::mem::transmute(&wszrestoremetadata)).into())
        }
        unsafe extern "system" fn GetRestoreMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrestoremetadata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRestoreMetadata(this, ::core::mem::transmute_copy(&pbstrrestoremetadata)).into())
        }
        unsafe extern "system" fn SetRestoreTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: VSS_RESTORE_TARGET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRestoreTarget(this, ::core::mem::transmute_copy(&target)).into())
        }
        unsafe extern "system" fn GetRestoreTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRestoreTarget(this, ::core::mem::transmute_copy(&ptarget)).into())
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszprerestorefailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreRestoreFailureMsg(this, ::core::mem::transmute(&wszprerestorefailuremsg)).into())
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprerestorefailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPreRestoreFailureMsg(this, ::core::mem::transmute_copy(&pbstrprerestorefailuremsg)).into())
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpostrestorefailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostRestoreFailureMsg(this, ::core::mem::transmute(&wszpostrestorefailuremsg)).into())
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpostrestorefailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPostRestoreFailureMsg(this, ::core::mem::transmute_copy(&pbstrpostrestorefailuremsg)).into())
        }
        unsafe extern "system" fn SetBackupStamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszbackupstamp: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackupStamp(this, ::core::mem::transmute(&wszbackupstamp)).into())
        }
        unsafe extern "system" fn GetBackupStamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBackupStamp(this, ::core::mem::transmute_copy(&pbstrbackupstamp)).into())
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPreviousBackupStamp(this, ::core::mem::transmute_copy(&pbstrbackupstamp)).into())
        }
        unsafe extern "system" fn GetBackupOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbackupoptions: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBackupOptions(this, ::core::mem::transmute_copy(&pbstrbackupoptions)).into())
        }
        unsafe extern "system" fn GetRestoreOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrestoreoptions: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRestoreOptions(this, ::core::mem::transmute_copy(&pbstrrestoreoptions)).into())
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRestoreSubcomponentCount(this, ::core::mem::transmute_copy(&pcrestoresubcomponent)).into())
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrcomponentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbrepair: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRestoreSubcomponent(this, ::core::mem::transmute_copy(&icomponent), ::core::mem::transmute_copy(&pbstrlogicalpath), ::core::mem::transmute_copy(&pbstrcomponentname), ::core::mem::transmute_copy(&pbrepair)).into())
        }
        unsafe extern "system" fn GetFileRestoreStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileRestoreStatus(this, ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDifferencedFilesByLastModifyTime(this, ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute(&ftlastmodifytime)).into())
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDifferencedFilesByLastModifyLSN(this, ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute(&bstrlsnstring)).into())
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdifferencedfiles: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDifferencedFilesCount(this, ::core::mem::transmute_copy(&pcdifferencedfiles)).into())
        }
        unsafe extern "system" fn GetDifferencedFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrfilespec: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDifferencedFile(this, ::core::mem::transmute_copy(&idifferencedfile), ::core::mem::transmute_copy(&pbstrpath), ::core::mem::transmute_copy(&pbstrfilespec), ::core::mem::transmute_copy(&pbrecursive), ::core::mem::transmute_copy(&pbstrlsnstring), ::core::mem::transmute_copy(&pftlastmodifytime)).into())
        }
        IVssComponent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLogicalPath: GetLogicalPath::<Identity, Impl, OFFSET>,
            GetComponentType: GetComponentType::<Identity, Impl, OFFSET>,
            GetComponentName: GetComponentName::<Identity, Impl, OFFSET>,
            GetBackupSucceeded: GetBackupSucceeded::<Identity, Impl, OFFSET>,
            GetAlternateLocationMappingCount: GetAlternateLocationMappingCount::<Identity, Impl, OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Identity, Impl, OFFSET>,
            SetBackupMetadata: SetBackupMetadata::<Identity, Impl, OFFSET>,
            GetBackupMetadata: GetBackupMetadata::<Identity, Impl, OFFSET>,
            AddPartialFile: AddPartialFile::<Identity, Impl, OFFSET>,
            GetPartialFileCount: GetPartialFileCount::<Identity, Impl, OFFSET>,
            GetPartialFile: GetPartialFile::<Identity, Impl, OFFSET>,
            IsSelectedForRestore: IsSelectedForRestore::<Identity, Impl, OFFSET>,
            GetAdditionalRestores: GetAdditionalRestores::<Identity, Impl, OFFSET>,
            GetNewTargetCount: GetNewTargetCount::<Identity, Impl, OFFSET>,
            GetNewTarget: GetNewTarget::<Identity, Impl, OFFSET>,
            AddDirectedTarget: AddDirectedTarget::<Identity, Impl, OFFSET>,
            GetDirectedTargetCount: GetDirectedTargetCount::<Identity, Impl, OFFSET>,
            GetDirectedTarget: GetDirectedTarget::<Identity, Impl, OFFSET>,
            SetRestoreMetadata: SetRestoreMetadata::<Identity, Impl, OFFSET>,
            GetRestoreMetadata: GetRestoreMetadata::<Identity, Impl, OFFSET>,
            SetRestoreTarget: SetRestoreTarget::<Identity, Impl, OFFSET>,
            GetRestoreTarget: GetRestoreTarget::<Identity, Impl, OFFSET>,
            SetPreRestoreFailureMsg: SetPreRestoreFailureMsg::<Identity, Impl, OFFSET>,
            GetPreRestoreFailureMsg: GetPreRestoreFailureMsg::<Identity, Impl, OFFSET>,
            SetPostRestoreFailureMsg: SetPostRestoreFailureMsg::<Identity, Impl, OFFSET>,
            GetPostRestoreFailureMsg: GetPostRestoreFailureMsg::<Identity, Impl, OFFSET>,
            SetBackupStamp: SetBackupStamp::<Identity, Impl, OFFSET>,
            GetBackupStamp: GetBackupStamp::<Identity, Impl, OFFSET>,
            GetPreviousBackupStamp: GetPreviousBackupStamp::<Identity, Impl, OFFSET>,
            GetBackupOptions: GetBackupOptions::<Identity, Impl, OFFSET>,
            GetRestoreOptions: GetRestoreOptions::<Identity, Impl, OFFSET>,
            GetRestoreSubcomponentCount: GetRestoreSubcomponentCount::<Identity, Impl, OFFSET>,
            GetRestoreSubcomponent: GetRestoreSubcomponent::<Identity, Impl, OFFSET>,
            GetFileRestoreStatus: GetFileRestoreStatus::<Identity, Impl, OFFSET>,
            AddDifferencedFilesByLastModifyTime: AddDifferencedFilesByLastModifyTime::<Identity, Impl, OFFSET>,
            AddDifferencedFilesByLastModifyLSN: AddDifferencedFilesByLastModifyLSN::<Identity, Impl, OFFSET>,
            GetDifferencedFilesCount: GetDifferencedFilesCount::<Identity, Impl, OFFSET>,
            GetDifferencedFile: GetDifferencedFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentEx_Impl: ::windows_core::BaseImpl + IVssComponent_Impl {
    fn SetPrepareForBackupFailureMsg(this: &Self::This, wszfailuremsg: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetPostSnapshotFailureMsg(this: &Self::This, wszfailuremsg: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPrepareForBackupFailureMsg(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPostSnapshotFailureMsg(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAuthoritativeRestore(this: &Self::This) -> ::windows_core::Result<bool>;
    fn GetRollForward(this: &Self::This, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetRestoreName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVssComponentEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IVssComponent);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssComponentEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrepareForBackupFailureMsg(this, ::core::mem::transmute(&wszfailuremsg)).into())
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostSnapshotFailureMsg(this, ::core::mem::transmute(&wszfailuremsg)).into())
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrepareForBackupFailureMsg(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfailuremsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPostSnapshotFailureMsg(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfailuremsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbauth: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAuthoritativeRestore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbauth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRollForward<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRollForward(this, ::core::mem::transmute_copy(&prolltype), ::core::mem::transmute_copy(&pbstrpoint)).into())
        }
        unsafe extern "system" fn GetRestoreName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestoreName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVssComponentEx_Vtbl {
            base__: <IVssComponent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPrepareForBackupFailureMsg: SetPrepareForBackupFailureMsg::<Identity, Impl, OFFSET>,
            SetPostSnapshotFailureMsg: SetPostSnapshotFailureMsg::<Identity, Impl, OFFSET>,
            GetPrepareForBackupFailureMsg: GetPrepareForBackupFailureMsg::<Identity, Impl, OFFSET>,
            GetPostSnapshotFailureMsg: GetPostSnapshotFailureMsg::<Identity, Impl, OFFSET>,
            GetAuthoritativeRestore: GetAuthoritativeRestore::<Identity, Impl, OFFSET>,
            GetRollForward: GetRollForward::<Identity, Impl, OFFSET>,
            GetRestoreName: GetRestoreName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVssComponentEx2_Impl: ::windows_core::BaseImpl + IVssComponentEx_Impl {
    fn SetFailure(this: &Self::This, hr: ::windows_core::HRESULT, hrapplication: ::windows_core::HRESULT, wszapplicationmessage: &::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::Result<()>;
    fn GetFailure(this: &Self::This, phr: *mut ::windows_core::HRESULT, phrapplication: *mut ::windows_core::HRESULT, pbstrapplicationmessage: *mut ::windows_core::BSTR, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVssComponentEx2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IVssComponentEx);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssComponentEx2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFailure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, hrapplication: ::windows_core::HRESULT, wszapplicationmessage: ::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFailure(this, ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&hrapplication), ::core::mem::transmute(&wszapplicationmessage), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn GetFailure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssComponentEx2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows_core::HRESULT, phrapplication: *mut ::windows_core::HRESULT, pbstrapplicationmessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFailure(this, ::core::mem::transmute_copy(&phr), ::core::mem::transmute_copy(&phrapplication), ::core::mem::transmute_copy(&pbstrapplicationmessage), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        IVssComponentEx2_Vtbl {
            base__: <IVssComponentEx as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFailure: SetFailure::<Identity, Impl, OFFSET>,
            GetFailure: GetFailure::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssCreateExpressWriterMetadata_Impl: ::windows_core::BaseImpl {
    fn AddExcludeFiles(this: &Self::This, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, brecursive: u8) -> ::windows_core::Result<()>;
    fn AddComponent(this: &Self::This, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &::windows_core::PCWSTR, wszcomponentname: &::windows_core::PCWSTR, wszcaption: &::windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::Result<()>;
    fn AddFilesToFileGroup(this: &Self::This, wszlogicalpath: &::windows_core::PCWSTR, wszgroupname: &::windows_core::PCWSTR, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::Result<()>;
    fn SetRestoreMethod(this: &Self::This, method: VSS_RESTOREMETHOD_ENUM, wszservice: &::windows_core::PCWSTR, wszuserprocedure: &::windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::Result<()>;
    fn AddComponentDependency(this: &Self::This, wszforlogicalpath: &::windows_core::PCWSTR, wszforcomponentname: &::windows_core::PCWSTR, onwriterid: &::windows_core::GUID, wszonlogicalpath: &::windows_core::PCWSTR, wszoncomponentname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetBackupSchema(this: &Self::This, dwschemamask: u32) -> ::windows_core::Result<()>;
    fn SaveAsXML(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IVssCreateExpressWriterMetadata {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssCreateExpressWriterMetadata {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddExcludeFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExcludeFiles(this, ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&brecursive)).into())
        }
        unsafe extern "system" fn AddComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows_core::PCWSTR, wszcomponentname: ::windows_core::PCWSTR, wszcaption: ::windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::AddComponent(
                    this,
                    ::core::mem::transmute_copy(&ct),
                    ::core::mem::transmute(&wszlogicalpath),
                    ::core::mem::transmute(&wszcomponentname),
                    ::core::mem::transmute(&wszcaption),
                    ::core::mem::transmute_copy(&pbicon),
                    ::core::mem::transmute_copy(&cbicon),
                    ::core::mem::transmute_copy(&brestoremetadata),
                    ::core::mem::transmute_copy(&bnotifyonbackupcomplete),
                    ::core::mem::transmute_copy(&bselectable),
                    ::core::mem::transmute_copy(&bselectableforrestore),
                    ::core::mem::transmute_copy(&dwcomponentflags),
                )
                .into()
            })
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszgroupname: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFilesToFileGroup(this, ::core::mem::transmute(&wszlogicalpath), ::core::mem::transmute(&wszgroupname), ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute(&wszalternatelocation), ::core::mem::transmute_copy(&dwbackuptypemask)).into())
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows_core::PCWSTR, wszuserprocedure: ::windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRestoreMethod(this, ::core::mem::transmute_copy(&method), ::core::mem::transmute(&wszservice), ::core::mem::transmute(&wszuserprocedure), ::core::mem::transmute_copy(&writerrestore), ::core::mem::transmute_copy(&brebootrequired)).into())
        }
        unsafe extern "system" fn AddComponentDependency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszforlogicalpath: ::windows_core::PCWSTR, wszforcomponentname: ::windows_core::PCWSTR, onwriterid: ::windows_core::GUID, wszonlogicalpath: ::windows_core::PCWSTR, wszoncomponentname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddComponentDependency(this, ::core::mem::transmute(&wszforlogicalpath), ::core::mem::transmute(&wszforcomponentname), ::core::mem::transmute(&onwriterid), ::core::mem::transmute(&wszonlogicalpath), ::core::mem::transmute(&wszoncomponentname)).into())
        }
        unsafe extern "system" fn SetBackupSchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackupSchema(this, ::core::mem::transmute_copy(&dwschemamask)).into())
        }
        unsafe extern "system" fn SaveAsXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssCreateExpressWriterMetadata_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveAsXML(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxml, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVssCreateExpressWriterMetadata_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddExcludeFiles: AddExcludeFiles::<Identity, Impl, OFFSET>,
            AddComponent: AddComponent::<Identity, Impl, OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity, Impl, OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Identity, Impl, OFFSET>,
            AddComponentDependency: AddComponentDependency::<Identity, Impl, OFFSET>,
            SetBackupSchema: SetBackupSchema::<Identity, Impl, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IVssCreateWriterMetadata_Impl: Sized {
    fn AddIncludeFiles(&self, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddExcludeFiles(&self, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, brecursive: u8) -> ::windows_core::Result<()>;
    fn AddComponent(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &::windows_core::PCWSTR, wszcomponentname: &::windows_core::PCWSTR, wszcaption: &::windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::Result<()>;
    fn AddDatabaseFiles(&self, wszlogicalpath: &::windows_core::PCWSTR, wszdatabasename: &::windows_core::PCWSTR, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::Result<()>;
    fn AddDatabaseLogFiles(&self, wszlogicalpath: &::windows_core::PCWSTR, wszdatabasename: &::windows_core::PCWSTR, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::Result<()>;
    fn AddFilesToFileGroup(&self, wszlogicalpath: &::windows_core::PCWSTR, wszgroupname: &::windows_core::PCWSTR, wszpath: &::windows_core::PCWSTR, wszfilespec: &::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: &::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::Result<()>;
    fn SetRestoreMethod(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: &::windows_core::PCWSTR, wszuserprocedure: &::windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::Result<()>;
    fn AddAlternateLocationMapping(&self, wszsourcepath: &::windows_core::PCWSTR, wszsourcefilespec: &::windows_core::PCWSTR, brecursive: u8, wszdestination: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddComponentDependency(&self, wszforlogicalpath: &::windows_core::PCWSTR, wszforcomponentname: &::windows_core::PCWSTR, onwriterid: &::windows_core::GUID, wszonlogicalpath: &::windows_core::PCWSTR, wszoncomponentname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows_core::Result<()>;
    fn GetDocument(&self) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument>;
    fn SaveAsXML(&self, pbstrxml: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadata_Vtbl {
    pub const fn new<Impl: IVssCreateWriterMetadata_Impl>() -> IVssCreateWriterMetadata_Vtbl {
        unsafe extern "system" fn AddIncludeFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddIncludeFiles(this, ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute(&wszalternatelocation)).into()
        }
        unsafe extern "system" fn AddExcludeFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddExcludeFiles(this, ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&brecursive)).into()
        }
        unsafe extern "system" fn AddComponent<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows_core::PCWSTR, wszcomponentname: ::windows_core::PCWSTR, wszcaption: ::windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddComponent(
                this,
                ::core::mem::transmute_copy(&ct),
                ::core::mem::transmute(&wszlogicalpath),
                ::core::mem::transmute(&wszcomponentname),
                ::core::mem::transmute(&wszcaption),
                ::core::mem::transmute_copy(&pbicon),
                ::core::mem::transmute_copy(&cbicon),
                ::core::mem::transmute_copy(&brestoremetadata),
                ::core::mem::transmute_copy(&bnotifyonbackupcomplete),
                ::core::mem::transmute_copy(&bselectable),
                ::core::mem::transmute_copy(&bselectableforrestore),
                ::core::mem::transmute_copy(&dwcomponentflags),
            )
            .into()
        }
        unsafe extern "system" fn AddDatabaseFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszdatabasename: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddDatabaseFiles(this, ::core::mem::transmute(&wszlogicalpath), ::core::mem::transmute(&wszdatabasename), ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszdatabasename: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddDatabaseLogFiles(this, ::core::mem::transmute(&wszlogicalpath), ::core::mem::transmute(&wszdatabasename), ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn AddFilesToFileGroup<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszgroupname: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddFilesToFileGroup(this, ::core::mem::transmute(&wszlogicalpath), ::core::mem::transmute(&wszgroupname), ::core::mem::transmute(&wszpath), ::core::mem::transmute(&wszfilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute(&wszalternatelocation), ::core::mem::transmute_copy(&dwbackuptypemask)).into()
        }
        unsafe extern "system" fn SetRestoreMethod<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows_core::PCWSTR, wszuserprocedure: ::windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetRestoreMethod(this, ::core::mem::transmute_copy(&method), ::core::mem::transmute(&wszservice), ::core::mem::transmute(&wszuserprocedure), ::core::mem::transmute_copy(&writerrestore), ::core::mem::transmute_copy(&brebootrequired)).into()
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, wszsourcepath: ::windows_core::PCWSTR, wszsourcefilespec: ::windows_core::PCWSTR, brecursive: u8, wszdestination: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddAlternateLocationMapping(this, ::core::mem::transmute(&wszsourcepath), ::core::mem::transmute(&wszsourcefilespec), ::core::mem::transmute_copy(&brecursive), ::core::mem::transmute(&wszdestination)).into()
        }
        unsafe extern "system" fn AddComponentDependency<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, wszforlogicalpath: ::windows_core::PCWSTR, wszforcomponentname: ::windows_core::PCWSTR, onwriterid: ::windows_core::GUID, wszonlogicalpath: ::windows_core::PCWSTR, wszoncomponentname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::AddComponentDependency(this, ::core::mem::transmute(&wszforlogicalpath), ::core::mem::transmute(&wszforcomponentname), ::core::mem::transmute(&onwriterid), ::core::mem::transmute(&wszonlogicalpath), ::core::mem::transmute(&wszoncomponentname)).into()
        }
        unsafe extern "system" fn SetBackupSchema<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SetBackupSchema(this, ::core::mem::transmute_copy(&dwschemamask)).into()
        }
        unsafe extern "system" fn GetDocument<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, pdoc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsXML<Impl: IVssCreateWriterMetadata_Impl>(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SaveAsXML(this, ::core::mem::transmute_copy(&pbstrxml)).into()
        }
        Self {
            AddIncludeFiles: AddIncludeFiles::<Impl>,
            AddExcludeFiles: AddExcludeFiles::<Impl>,
            AddComponent: AddComponent::<Impl>,
            AddDatabaseFiles: AddDatabaseFiles::<Impl>,
            AddDatabaseLogFiles: AddDatabaseLogFiles::<Impl>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Impl>,
            SetRestoreMethod: SetRestoreMethod::<Impl>,
            AddAlternateLocationMapping: AddAlternateLocationMapping::<Impl>,
            AddComponentDependency: AddComponentDependency::<Impl>,
            SetBackupSchema: SetBackupSchema::<Impl>,
            GetDocument: GetDocument::<Impl>,
            SaveAsXML: SaveAsXML::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
struct IVssCreateWriterMetadata_ImplVtbl<T: IVssCreateWriterMetadata_Impl>(::std::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl<T: IVssCreateWriterMetadata_Impl> IVssCreateWriterMetadata_ImplVtbl<T> {
    const VTABLE: IVssCreateWriterMetadata_Vtbl = IVssCreateWriterMetadata_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IVssCreateWriterMetadata {
    pub fn new<'a, T: IVssCreateWriterMetadata_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &IVssCreateWriterMetadata_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait IVssDifferentialSoftwareSnapshotMgmt_Impl: ::windows_core::BaseImpl {
    fn AddDiffArea(this: &Self::This, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()>;
    fn ChangeDiffAreaMaximumSize(this: &Self::This, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()>;
    fn QueryVolumesSupportedForDiffAreas(this: &Self::This, pwszoriginalvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForVolume(this: &Self::This, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasOnVolume(this: &Self::This, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject>;
    fn QueryDiffAreasForSnapshot(this: &Self::This, snapshotid: &::windows_core::GUID) -> ::windows_core::Result<IVssEnumMgmtObject>;
}
impl ::windows_core::Iids for IVssDifferentialSoftwareSnapshotMgmt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssDifferentialSoftwareSnapshotMgmt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddDiffArea<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDiffArea(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace)).into())
        }
        unsafe extern "system" fn ChangeDiffAreaMaximumSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeDiffAreaMaximumSize(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace)).into())
        }
        unsafe extern "system" fn QueryVolumesSupportedForDiffAreas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryVolumesSupportedForDiffAreas(this, ::core::mem::transmute_copy(&pwszoriginalvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryDiffAreasForVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDiffAreasForVolume(this, ::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryDiffAreasOnVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDiffAreasOnVolume(this, ::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryDiffAreasForSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDiffAreasForSnapshot(this, ::core::mem::transmute(&snapshotid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddDiffArea: AddDiffArea::<Identity, Impl, OFFSET>,
            ChangeDiffAreaMaximumSize: ChangeDiffAreaMaximumSize::<Identity, Impl, OFFSET>,
            QueryVolumesSupportedForDiffAreas: QueryVolumesSupportedForDiffAreas::<Identity, Impl, OFFSET>,
            QueryDiffAreasForVolume: QueryDiffAreasForVolume::<Identity, Impl, OFFSET>,
            QueryDiffAreasOnVolume: QueryDiffAreasOnVolume::<Identity, Impl, OFFSET>,
            QueryDiffAreasForSnapshot: QueryDiffAreasForSnapshot::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt2_Impl: ::windows_core::BaseImpl + IVssDifferentialSoftwareSnapshotMgmt_Impl {
    fn ChangeDiffAreaMaximumSizeEx(this: &Self::This, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn MigrateDiffAreas(this: &Self::This, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows_core::Result<()>;
    fn QueryMigrationStatus(this: &Self::This, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows_core::Result<IVssAsync>;
    fn SetSnapshotPriority(this: &Self::This, idsnapshot: &::windows_core::GUID, priority: u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVssDifferentialSoftwareSnapshotMgmt2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IVssDifferentialSoftwareSnapshotMgmt);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssDifferentialSoftwareSnapshotMgmt2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChangeDiffAreaMaximumSizeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeDiffAreaMaximumSizeEx(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&llmaximumdiffspace), ::core::mem::transmute_copy(&bvolatile)).into())
        }
        unsafe extern "system" fn MigrateDiffAreas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MigrateDiffAreas(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename), ::core::mem::transmute_copy(&pwsznewdiffareavolumename)).into())
        }
        unsafe extern "system" fn QueryMigrationStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMigrationStatus(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pwszdiffareavolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSnapshotPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idsnapshot: ::windows_core::GUID, priority: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapshotPriority(this, ::core::mem::transmute(&idsnapshot), ::core::mem::transmute_copy(&priority)).into())
        }
        IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
            base__: <IVssDifferentialSoftwareSnapshotMgmt as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ChangeDiffAreaMaximumSizeEx: ChangeDiffAreaMaximumSizeEx::<Identity, Impl, OFFSET>,
            MigrateDiffAreas: MigrateDiffAreas::<Identity, Impl, OFFSET>,
            QueryMigrationStatus: QueryMigrationStatus::<Identity, Impl, OFFSET>,
            SetSnapshotPriority: SetSnapshotPriority::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVssDifferentialSoftwareSnapshotMgmt3_Impl: ::windows_core::BaseImpl + IVssDifferentialSoftwareSnapshotMgmt2_Impl {
    fn SetVolumeProtectLevel(this: &Self::This, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows_core::Result<()>;
    fn GetVolumeProtectLevel(this: &Self::This, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows_core::Result<()>;
    fn ClearVolumeProtectFault(this: &Self::This, pwszvolumename: *const u16) -> ::windows_core::Result<()>;
    fn DeleteUnusedDiffAreas(this: &Self::This, pwszdiffareavolumename: *const u16) -> ::windows_core::Result<()>;
    fn QuerySnapshotDeltaBitmap(this: &Self::This, idsnapshotolder: &::windows_core::GUID, idsnapshotyounger: &::windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVssDifferentialSoftwareSnapshotMgmt3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IVssDifferentialSoftwareSnapshotMgmt2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssDifferentialSoftwareSnapshotMgmt3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetVolumeProtectLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolumeProtectLevel(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&protectionlevel)).into())
        }
        unsafe extern "system" fn GetVolumeProtectLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVolumeProtectLevel(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&protectionlevel)).into())
        }
        unsafe extern "system" fn ClearVolumeProtectFault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearVolumeProtectFault(this, ::core::mem::transmute_copy(&pwszvolumename)).into())
        }
        unsafe extern "system" fn DeleteUnusedDiffAreas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdiffareavolumename: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteUnusedDiffAreas(this, ::core::mem::transmute_copy(&pwszdiffareavolumename)).into())
        }
        unsafe extern "system" fn QuerySnapshotDeltaBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssDifferentialSoftwareSnapshotMgmt3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idsnapshotolder: ::windows_core::GUID, idsnapshotyounger: ::windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QuerySnapshotDeltaBitmap(this, ::core::mem::transmute(&idsnapshotolder), ::core::mem::transmute(&idsnapshotyounger), ::core::mem::transmute_copy(&pcblocksizeperbit), ::core::mem::transmute_copy(&pcbitmaplength), ::core::mem::transmute_copy(&ppbbitmap)).into())
        }
        IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
            base__: <IVssDifferentialSoftwareSnapshotMgmt2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetVolumeProtectLevel: SetVolumeProtectLevel::<Identity, Impl, OFFSET>,
            GetVolumeProtectLevel: GetVolumeProtectLevel::<Identity, Impl, OFFSET>,
            ClearVolumeProtectFault: ClearVolumeProtectFault::<Identity, Impl, OFFSET>,
            DeleteUnusedDiffAreas: DeleteUnusedDiffAreas::<Identity, Impl, OFFSET>,
            QuerySnapshotDeltaBitmap: QuerySnapshotDeltaBitmap::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssEnumMgmtObject_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This, ppenum: *mut ::core::option::Option<IVssEnumMgmtObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssEnumMgmtObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssEnumMgmtObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumMgmtObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        IVssEnumMgmtObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssEnumObject_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This, ppenum: *mut ::core::option::Option<IVssEnumObject>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssEnumObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssEnumObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssEnumObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        IVssEnumObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssExpressWriter_Impl: ::windows_core::BaseImpl {
    fn CreateMetadata(this: &Self::This, writerid: &::windows_core::GUID, writername: &::windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> ::windows_core::Result<IVssCreateExpressWriterMetadata>;
    fn LoadMetadata(this: &Self::This, metadata: &::windows_core::PCWSTR, reserved: u32) -> ::windows_core::Result<()>;
    fn Register(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unregister(this: &Self::This, writerid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssExpressWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssExpressWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, writerid: ::windows_core::GUID, writername: ::windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMetadata(this, ::core::mem::transmute(&writerid), ::core::mem::transmute(&writername), ::core::mem::transmute_copy(&usagetype), ::core::mem::transmute_copy(&versionmajor), ::core::mem::transmute_copy(&versionminor), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadata: ::windows_core::PCWSTR, reserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadMetadata(this, ::core::mem::transmute(&metadata), ::core::mem::transmute_copy(&reserved)).into())
        }
        unsafe extern "system" fn Register<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Register(this).into())
        }
        unsafe extern "system" fn Unregister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssExpressWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, writerid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unregister(this, ::core::mem::transmute(&writerid)).into())
        }
        IVssExpressWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateMetadata: CreateMetadata::<Identity, Impl, OFFSET>,
            LoadMetadata: LoadMetadata::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVssFileShareSnapshotProvider_Impl: ::windows_core::BaseImpl {
    fn SetContext(this: &Self::This, lcontext: i32) -> ::windows_core::Result<()>;
    fn GetSnapshotProperties(this: &Self::This, snapshotid: &::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::Result<()>;
    fn Query(this: &Self::This, queriedobjectid: &::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows_core::Result<IVssEnumObject>;
    fn DeleteSnapshots(this: &Self::This, sourceobjectid: &::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn BeginPrepareSnapshot(this: &Self::This, snapshotsetid: &::windows_core::GUID, snapshotid: &::windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn IsPathSupported(this: &Self::This, pwszsharepath: *const u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsPathSnapshotted(this: &Self::This, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::Result<()>;
    fn SetSnapshotProperty(this: &Self::This, snapshotid: &::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVssFileShareSnapshotProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssFileShareSnapshotProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContext(this, ::core::mem::transmute_copy(&lcontext)).into())
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSnapshotProperties(this, ::core::mem::transmute(&snapshotid), ::core::mem::transmute_copy(&pprop)).into())
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Query(this, ::core::mem::transmute(&queriedobjectid), ::core::mem::transmute_copy(&equeriedobjecttype), ::core::mem::transmute_copy(&ereturnedobjectstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteSnapshots(this, ::core::mem::transmute(&sourceobjectid), ::core::mem::transmute_copy(&esourceobjecttype), ::core::mem::transmute_copy(&bforcedelete), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)).into())
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginPrepareSnapshot(this, ::core::mem::transmute(&snapshotsetid), ::core::mem::transmute(&snapshotid), ::core::mem::transmute_copy(&pwszsharepath), ::core::mem::transmute_copy(&lnewcontext), ::core::mem::transmute(&providerid)).into())
        }
        unsafe extern "system" fn IsPathSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPathSupported(this, ::core::mem::transmute_copy(&pwszsharepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupportedbythisprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPathSnapshotted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPathSnapshotted(this, ::core::mem::transmute_copy(&pwszsharepath), ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)).into())
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssFileShareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapshotProperty(this, ::core::mem::transmute(&snapshotid), ::core::mem::transmute_copy(&esnapshotpropertyid), ::core::mem::transmute(&vproperty)).into())
        }
        IVssFileShareSnapshotProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            IsPathSupported: IsPathSupported::<Identity, Impl, OFFSET>,
            IsPathSnapshotted: IsPathSnapshotted::<Identity, Impl, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProvider_Impl: ::windows_core::BaseImpl {
    fn AreLunsSupported(this: &Self::This, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FillInLunInfo(this: &Self::This, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn BeginPrepareSnapshot(this: &Self::This, snapshotsetid: &::windows_core::GUID, snapshotid: &::windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()>;
    fn GetTargetLuns(this: &Self::This, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()>;
    fn LocateLuns(this: &Self::This, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()>;
    fn OnLunEmpty(this: &Self::This, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl ::windows_core::Iids for IVssHardwareSnapshotProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssHardwareSnapshotProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AreLunsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AreLunsSupported(this, ::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&lcontext), ::core::mem::transmute_copy(&rgwszdevices), ::core::mem::transmute_copy(&pluninformation), ::core::mem::transmute_copy(&pbissupported)).into())
        }
        unsafe extern "system" fn FillInLunInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillInLunInfo(this, ::core::mem::transmute_copy(&wszdevicename), ::core::mem::transmute_copy(&pluninfo), ::core::mem::transmute_copy(&pbissupported)).into())
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginPrepareSnapshot(this, ::core::mem::transmute(&snapshotsetid), ::core::mem::transmute(&snapshotid), ::core::mem::transmute_copy(&lcontext), ::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgdevicenames), ::core::mem::transmute_copy(&rgluninformation)).into())
        }
        unsafe extern "system" fn GetTargetLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTargetLuns(this, ::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgdevicenames), ::core::mem::transmute_copy(&rgsourceluns), ::core::mem::transmute_copy(&rgdestinationluns)).into())
        }
        unsafe extern "system" fn LocateLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LocateLuns(this, ::core::mem::transmute_copy(&lluncount), ::core::mem::transmute_copy(&rgsourceluns)).into())
        }
        unsafe extern "system" fn OnLunEmpty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLunEmpty(this, ::core::mem::transmute_copy(&wszdevicename), ::core::mem::transmute_copy(&pinformation)).into())
        }
        IVssHardwareSnapshotProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AreLunsSupported: AreLunsSupported::<Identity, Impl, OFFSET>,
            FillInLunInfo: FillInLunInfo::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            GetTargetLuns: GetTargetLuns::<Identity, Impl, OFFSET>,
            LocateLuns: LocateLuns::<Identity, Impl, OFFSET>,
            OnLunEmpty: OnLunEmpty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
pub trait IVssHardwareSnapshotProviderEx_Impl: ::windows_core::BaseImpl + IVssHardwareSnapshotProvider_Impl {
    fn GetProviderCapabilities(this: &Self::This) -> ::windows_core::Result<u64>;
    fn OnLunStateChange(this: &Self::This, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn ResyncLuns(this: &Self::This, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows_core::Result<IVssAsync>;
    fn OnReuseLuns(this: &Self::This, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl ::windows_core::Iids for IVssHardwareSnapshotProviderEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IVssHardwareSnapshotProvider);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssHardwareSnapshotProviderEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProviderCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plloriginalcapabilitymask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnLunStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLunStateChange(this, ::core::mem::transmute_copy(&psnapshotluns), ::core::mem::transmute_copy(&poriginalluns), ::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn ResyncLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResyncLuns(this, ::core::mem::transmute_copy(&psourceluns), ::core::mem::transmute_copy(&ptargetluns), ::core::mem::transmute_copy(&dwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnReuseLuns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssHardwareSnapshotProviderEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReuseLuns(this, ::core::mem::transmute_copy(&psnapshotluns), ::core::mem::transmute_copy(&poriginalluns), ::core::mem::transmute_copy(&dwcount)).into())
        }
        IVssHardwareSnapshotProviderEx_Vtbl {
            base__: <IVssHardwareSnapshotProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProviderCapabilities: GetProviderCapabilities::<Identity, Impl, OFFSET>,
            OnLunStateChange: OnLunStateChange::<Identity, Impl, OFFSET>,
            ResyncLuns: ResyncLuns::<Identity, Impl, OFFSET>,
            OnReuseLuns: OnReuseLuns::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssProviderCreateSnapshotSet_Impl: ::windows_core::BaseImpl {
    fn EndPrepareSnapshots(this: &Self::This, snapshotsetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PreCommitSnapshots(this: &Self::This, snapshotsetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn CommitSnapshots(this: &Self::This, snapshotsetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PostCommitSnapshots(this: &Self::This, snapshotsetid: &::windows_core::GUID, lsnapshotscount: i32) -> ::windows_core::Result<()>;
    fn PreFinalCommitSnapshots(this: &Self::This, snapshotsetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn PostFinalCommitSnapshots(this: &Self::This, snapshotsetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn AbortSnapshots(this: &Self::This, snapshotsetid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssProviderCreateSnapshotSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssProviderCreateSnapshotSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EndPrepareSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndPrepareSnapshots(this, ::core::mem::transmute(&snapshotsetid)).into())
        }
        unsafe extern "system" fn PreCommitSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreCommitSnapshots(this, ::core::mem::transmute(&snapshotsetid)).into())
        }
        unsafe extern "system" fn CommitSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitSnapshots(this, ::core::mem::transmute(&snapshotsetid)).into())
        }
        unsafe extern "system" fn PostCommitSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, lsnapshotscount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostCommitSnapshots(this, ::core::mem::transmute(&snapshotsetid), ::core::mem::transmute_copy(&lsnapshotscount)).into())
        }
        unsafe extern "system" fn PreFinalCommitSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreFinalCommitSnapshots(this, ::core::mem::transmute(&snapshotsetid)).into())
        }
        unsafe extern "system" fn PostFinalCommitSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostFinalCommitSnapshots(this, ::core::mem::transmute(&snapshotsetid)).into())
        }
        unsafe extern "system" fn AbortSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderCreateSnapshotSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortSnapshots(this, ::core::mem::transmute(&snapshotsetid)).into())
        }
        IVssProviderCreateSnapshotSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EndPrepareSnapshots: EndPrepareSnapshots::<Identity, Impl, OFFSET>,
            PreCommitSnapshots: PreCommitSnapshots::<Identity, Impl, OFFSET>,
            CommitSnapshots: CommitSnapshots::<Identity, Impl, OFFSET>,
            PostCommitSnapshots: PostCommitSnapshots::<Identity, Impl, OFFSET>,
            PreFinalCommitSnapshots: PreFinalCommitSnapshots::<Identity, Impl, OFFSET>,
            PostFinalCommitSnapshots: PostFinalCommitSnapshots::<Identity, Impl, OFFSET>,
            AbortSnapshots: AbortSnapshots::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IVssProviderNotifications_Impl: ::windows_core::BaseImpl {
    fn OnLoad(this: &Self::This, pcallback: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnUnload(this: &Self::This, bforceunload: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IVssProviderNotifications {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderNotifications_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssProviderNotifications {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderNotifications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLoad(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn OnUnload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssProviderNotifications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUnload(this, ::core::mem::transmute_copy(&bforceunload)).into())
        }
        IVssProviderNotifications_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssSnapshotMgmt_Impl: ::windows_core::BaseImpl {
    fn GetProviderMgmtInterface(this: &Self::This, providerid: &::windows_core::GUID, interfaceid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryVolumesSupportedForSnapshots(this: &Self::This, providerid: &::windows_core::GUID, lcontext: i32) -> ::windows_core::Result<IVssEnumMgmtObject>;
    fn QuerySnapshotsByVolume(this: &Self::This, pwszvolumename: *const u16, providerid: &::windows_core::GUID) -> ::windows_core::Result<IVssEnumObject>;
}
impl ::windows_core::Iids for IVssSnapshotMgmt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssSnapshotMgmt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProviderMgmtInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, interfaceid: *const ::windows_core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderMgmtInterface(this, ::core::mem::transmute(&providerid), ::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryVolumesSupportedForSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, lcontext: i32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryVolumesSupportedForSnapshots(this, ::core::mem::transmute(&providerid), ::core::mem::transmute_copy(&lcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QuerySnapshotsByVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSnapshotMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, providerid: ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySnapshotsByVolume(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVssSnapshotMgmt_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProviderMgmtInterface: GetProviderMgmtInterface::<Identity, Impl, OFFSET>,
            QueryVolumesSupportedForSnapshots: QueryVolumesSupportedForSnapshots::<Identity, Impl, OFFSET>,
            QuerySnapshotsByVolume: QuerySnapshotsByVolume::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssSnapshotMgmt2_Impl: ::windows_core::BaseImpl {
    fn GetMinDiffAreaSize(this: &Self::This) -> ::windows_core::Result<i64>;
}
impl ::windows_core::Iids for IVssSnapshotMgmt2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSnapshotMgmt2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssSnapshotMgmt2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMinDiffAreaSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSnapshotMgmt2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllmindiffareasize: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinDiffAreaSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllmindiffareasize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVssSnapshotMgmt2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMinDiffAreaSize: GetMinDiffAreaSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVssSoftwareSnapshotProvider_Impl: ::windows_core::BaseImpl {
    fn SetContext(this: &Self::This, lcontext: i32) -> ::windows_core::Result<()>;
    fn GetSnapshotProperties(this: &Self::This, snapshotid: &::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::Result<()>;
    fn Query(this: &Self::This, queriedobjectid: &::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows_core::Result<IVssEnumObject>;
    fn DeleteSnapshots(this: &Self::This, sourceobjectid: &::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn BeginPrepareSnapshot(this: &Self::This, snapshotsetid: &::windows_core::GUID, snapshotid: &::windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows_core::Result<()>;
    fn IsVolumeSupported(this: &Self::This, pwszvolumename: *const u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsVolumeSnapshotted(this: &Self::This, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::Result<()>;
    fn SetSnapshotProperty(this: &Self::This, snapshotid: &::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RevertToSnapshot(this: &Self::This, snapshotid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn QueryRevertStatus(this: &Self::This, pwszvolume: *const u16) -> ::windows_core::Result<IVssAsync>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVssSoftwareSnapshotProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssSoftwareSnapshotProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContext(this, ::core::mem::transmute_copy(&lcontext)).into())
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSnapshotProperties(this, ::core::mem::transmute(&snapshotid), ::core::mem::transmute_copy(&pprop)).into())
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Query(this, ::core::mem::transmute(&queriedobjectid), ::core::mem::transmute_copy(&equeriedobjecttype), ::core::mem::transmute_copy(&ereturnedobjectstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteSnapshots(this, ::core::mem::transmute(&sourceobjectid), ::core::mem::transmute_copy(&esourceobjecttype), ::core::mem::transmute_copy(&bforcedelete), ::core::mem::transmute_copy(&pldeletedsnapshots), ::core::mem::transmute_copy(&pnondeletedsnapshotid)).into())
        }
        unsafe extern "system" fn BeginPrepareSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginPrepareSnapshot(this, ::core::mem::transmute(&snapshotsetid), ::core::mem::transmute(&snapshotid), ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&lnewcontext)).into())
        }
        unsafe extern "system" fn IsVolumeSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsVolumeSupported(this, ::core::mem::transmute_copy(&pwszvolumename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupportedbythisprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsVolumeSnapshotted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsVolumeSnapshotted(this, ::core::mem::transmute_copy(&pwszvolumename), ::core::mem::transmute_copy(&pbsnapshotspresent), ::core::mem::transmute_copy(&plsnapshotcompatibility)).into())
        }
        unsafe extern "system" fn SetSnapshotProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapshotProperty(this, ::core::mem::transmute(&snapshotid), ::core::mem::transmute_copy(&esnapshotpropertyid), ::core::mem::transmute(&vproperty)).into())
        }
        unsafe extern "system" fn RevertToSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevertToSnapshot(this, ::core::mem::transmute(&snapshotid)).into())
        }
        unsafe extern "system" fn QueryRevertStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssSoftwareSnapshotProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryRevertStatus(this, ::core::mem::transmute_copy(&pwszvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVssSoftwareSnapshotProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, Impl, OFFSET>,
            BeginPrepareSnapshot: BeginPrepareSnapshot::<Identity, Impl, OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Identity, Impl, OFFSET>,
            IsVolumeSnapshotted: IsVolumeSnapshotted::<Identity, Impl, OFFSET>,
            SetSnapshotProperty: SetSnapshotProperty::<Identity, Impl, OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Identity, Impl, OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssWMDependency_Impl: ::windows_core::BaseImpl {
    fn GetWriterId(this: &Self::This, pwriterid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetLogicalPath(this: &Self::This, pbstrlogicalpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetComponentName(this: &Self::This, pbstrcomponentname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVssWMDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssWMDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWriterId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwriterid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWriterId(this, ::core::mem::transmute_copy(&pwriterid)).into())
        }
        unsafe extern "system" fn GetLogicalPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlogicalpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLogicalPath(this, ::core::mem::transmute_copy(&pbstrlogicalpath)).into())
        }
        unsafe extern "system" fn GetComponentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcomponentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetComponentName(this, ::core::mem::transmute_copy(&pbstrcomponentname)).into())
        }
        IVssWMDependency_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWriterId: GetWriterId::<Identity, Impl, OFFSET>,
            GetLogicalPath: GetLogicalPath::<Identity, Impl, OFFSET>,
            GetComponentName: GetComponentName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssWMFiledesc_Impl: ::windows_core::BaseImpl {
    fn GetPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFilespec(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRecursive(this: &Self::This) -> ::windows_core::Result<bool>;
    fn GetAlternateLocation(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetBackupTypeMask(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IVssWMFiledesc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVssWMFiledesc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilespec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfilespec: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilespec(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfilespec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecursive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrecursive: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecursive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbrecursive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAlternateLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstralternatelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAlternateLocation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstralternatelocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBackupTypeMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVssWMFiledesc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtypemask: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackupTypeMask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtypemask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVssWMFiledesc_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetFilespec: GetFilespec::<Identity, Impl, OFFSET>,
            GetRecursive: GetRecursive::<Identity, Impl, OFFSET>,
            GetAlternateLocation: GetAlternateLocation::<Identity, Impl, OFFSET>,
            GetBackupTypeMask: GetBackupTypeMask::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVssWriterComponents_Impl: Sized {
    fn GetComponentCount(&self, pccomponents: *mut u32) -> ::windows_core::Result<()>;
    fn GetWriterInfo(&self, pidinstance: *mut ::windows_core::GUID, pidwriter: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetComponent(&self, icomponent: u32) -> ::windows_core::Result<IVssComponent>;
}
impl IVssWriterComponents_Vtbl {
    pub const fn new<Impl: IVssWriterComponents_Impl>() -> IVssWriterComponents_Vtbl {
        unsafe extern "system" fn GetComponentCount<Impl: IVssWriterComponents_Impl>(this: *mut ::core::ffi::c_void, pccomponents: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetComponentCount(this, ::core::mem::transmute_copy(&pccomponents)).into()
        }
        unsafe extern "system" fn GetWriterInfo<Impl: IVssWriterComponents_Impl>(this: *mut ::core::ffi::c_void, pidinstance: *mut ::windows_core::GUID, pidwriter: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::GetWriterInfo(this, ::core::mem::transmute_copy(&pidinstance), ::core::mem::transmute_copy(&pidwriter)).into()
        }
        unsafe extern "system" fn GetComponent<Impl: IVssWriterComponents_Impl>(this: *mut ::core::ffi::c_void, icomponent: u32, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match Impl::GetComponent(this, ::core::mem::transmute_copy(&icomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomponent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { GetComponentCount: GetComponentCount::<Impl>, GetWriterInfo: GetWriterInfo::<Impl>, GetComponent: GetComponent::<Impl> }
    }
}
#[doc(hidden)]
struct IVssWriterComponents_ImplVtbl<T: IVssWriterComponents_Impl>(::std::marker::PhantomData<T>);
impl<T: IVssWriterComponents_Impl> IVssWriterComponents_ImplVtbl<T> {
    const VTABLE: IVssWriterComponents_Vtbl = IVssWriterComponents_Vtbl::new::<T>();
}
impl IVssWriterComponents {
    pub fn new<'a, T: IVssWriterComponents_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &IVssWriterComponents_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
