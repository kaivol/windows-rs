#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IGPEInformation_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::Result<()>;
    fn GetDisplayName(this: &Self::This, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::Result<()>;
    fn GetRegistryKey(this: &Self::This, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::Result<()>;
    fn GetDSPath(this: &Self::This, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::Result<()>;
    fn GetFileSysPath(this: &Self::This, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::Result<()>;
    fn GetOptions(this: &Self::This, dwoptions: *mut u32) -> ::windows_core::Result<()>;
    fn GetType(this: &Self::This, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::Result<()>;
    fn GetHint(this: &Self::This, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows_core::Result<()>;
    fn PolicyChanged(this: &Self::This, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguidsnapin: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for IGPEInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPEInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into())
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayName(this, ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into())
        }
        unsafe extern "system" fn GetRegistryKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegistryKey(this, ::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into())
        }
        unsafe extern "system" fn GetDSPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDSPath(this, ::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into())
        }
        unsafe extern "system" fn GetFileSysPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileSysPath(this, ::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into())
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptions(this, ::core::mem::transmute_copy(&dwoptions)).into())
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this, ::core::mem::transmute_copy(&gpotype)).into())
        }
        unsafe extern "system" fn GetHint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHint(this, ::core::mem::transmute_copy(&gphint)).into())
        }
        unsafe extern "system" fn PolicyChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguidsnapin: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PolicyChanged(this, ::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguidsnapin)).into())
        }
        IGPEInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, Impl, OFFSET>,
            GetDSPath: GetDSPath::<Identity, Impl, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetHint: GetHint::<Identity, Impl, OFFSET>,
            PolicyChanged: PolicyChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPM_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetDomain(this: &Self::This, bstrdomain: &::windows_core::BSTR, bstrdomaincontroller: &::windows_core::BSTR, ldcflags: i32) -> ::windows_core::Result<IGPMDomain>;
    fn GetBackupDir(this: &Self::This, bstrbackupdir: &::windows_core::BSTR) -> ::windows_core::Result<IGPMBackupDir>;
    fn GetSitesContainer(this: &Self::This, bstrforest: &::windows_core::BSTR, bstrdomain: &::windows_core::BSTR, bstrdomaincontroller: &::windows_core::BSTR, ldcflags: i32) -> ::windows_core::Result<IGPMSitesContainer>;
    fn GetRSOP(this: &Self::This, gpmrsopmode: GPMRSOPMode, bstrnamespace: &::windows_core::BSTR, lflags: i32) -> ::windows_core::Result<IGPMRSOP>;
    fn CreatePermission(this: &Self::This, bstrtrustee: &::windows_core::BSTR, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IGPMPermission>;
    fn CreateSearchCriteria(this: &Self::This) -> ::windows_core::Result<IGPMSearchCriteria>;
    fn CreateTrustee(this: &Self::This, bstrtrustee: &::windows_core::BSTR) -> ::windows_core::Result<IGPMTrustee>;
    fn GetClientSideExtensions(this: &Self::This) -> ::windows_core::Result<IGPMCSECollection>;
    fn GetConstants(this: &Self::This) -> ::windows_core::Result<IGPMConstants>;
    fn GetMigrationTable(this: &Self::This, bstrmigrationtablepath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMMigrationTable>;
    fn CreateMigrationTable(this: &Self::This) -> ::windows_core::Result<IGPMMigrationTable>;
    fn InitializeReporting(this: &Self::This, bstradmpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPM {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPM {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomaincontroller: ::std::mem::MaybeUninit<::windows_core::BSTR>, ldcflags: i32, pigpmdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDomain(this, ::core::mem::transmute(&bstrdomain), ::core::mem::transmute(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pigpmdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBackupDir<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, pigpmbackupdir: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackupDir(this, ::core::mem::transmute(&bstrbackupdir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pigpmbackupdir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSitesContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrforest: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomaincontroller: ::std::mem::MaybeUninit<::windows_core::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSitesContainer(this, ::core::mem::transmute(&bstrforest), ::core::mem::transmute(&bstrdomain), ::core::mem::transmute(&bstrdomaincontroller), ::core::mem::transmute_copy(&ldcflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsitescontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRSOP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: i32, ppigpmrsop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRSOP(this, ::core::mem::transmute_copy(&gpmrsopmode), ::core::mem::transmute(&bstrnamespace), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmrsop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL, ppperm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePermission(this, ::core::mem::transmute(&bstrtrustee), ::core::mem::transmute_copy(&perm), ::core::mem::transmute_copy(&binheritable)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppperm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSearchCriteria<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSearchCriteria(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsearchcriteria, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTrustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTrustee(this, ::core::mem::transmute(&bstrtrustee)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtrustee, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClientSideExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClientSideExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmcsecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConstants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConstants(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmconstants, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMigrationTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMigrationTable(this, ::core::mem::transmute(&bstrmigrationtablepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmigrationtable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMigrationTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMigrationTable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmigrationtable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeReporting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeReporting(this, ::core::mem::transmute(&bstradmpath)).into())
        }
        IGPM_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDomain: GetDomain::<Identity, Impl, OFFSET>,
            GetBackupDir: GetBackupDir::<Identity, Impl, OFFSET>,
            GetSitesContainer: GetSitesContainer::<Identity, Impl, OFFSET>,
            GetRSOP: GetRSOP::<Identity, Impl, OFFSET>,
            CreatePermission: CreatePermission::<Identity, Impl, OFFSET>,
            CreateSearchCriteria: CreateSearchCriteria::<Identity, Impl, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, Impl, OFFSET>,
            GetClientSideExtensions: GetClientSideExtensions::<Identity, Impl, OFFSET>,
            GetConstants: GetConstants::<Identity, Impl, OFFSET>,
            GetMigrationTable: GetMigrationTable::<Identity, Impl, OFFSET>,
            CreateMigrationTable: CreateMigrationTable::<Identity, Impl, OFFSET>,
            InitializeReporting: InitializeReporting::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPM2_Impl: ::windows_core::BaseImpl + IGPM_Impl {
    fn GetBackupDirEx(this: &Self::This, bstrbackupdir: &::windows_core::BSTR, backupdirtype: GPMBackupType) -> ::windows_core::Result<IGPMBackupDirEx>;
    fn InitializeReportingEx(this: &Self::This, bstradmpath: &::windows_core::BSTR, reportingoptions: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPM2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IGPM);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPM2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBackupDirEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackupDirEx(this, ::core::mem::transmute(&bstrbackupdir), ::core::mem::transmute_copy(&backupdirtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmbackupdirex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeReportingEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradmpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, reportingoptions: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeReportingEx(this, ::core::mem::transmute(&bstradmpath), ::core::mem::transmute_copy(&reportingoptions)).into())
        }
        IGPM2_Vtbl {
            base__: <IGPM as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBackupDirEx: GetBackupDirEx::<Identity, Impl, OFFSET>,
            InitializeReportingEx: InitializeReportingEx::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMAsyncCancel_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMAsyncCancel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMAsyncCancel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMAsyncCancel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMAsyncCancel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IGPMAsyncCancel_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Cancel: Cancel::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMAsyncProgress_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Status(this: &Self::This, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows_core::HRESULT, presult: *const super::Variant::VARIANT, ppigpmstatusmsgcollection: ::core::option::Option<&IGPMStatusMsgCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMAsyncProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMAsyncProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMAsyncProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMAsyncProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows_core::HRESULT, presult: *const super::Variant::VARIANT, ppigpmstatusmsgcollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Status(this, ::core::mem::transmute_copy(&lprogressnumerator), ::core::mem::transmute_copy(&lprogressdenominator), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&presult), ::windows_core::from_raw_borrowed(&ppigpmstatusmsgcollection)).into())
        }
        IGPMAsyncProgress_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Status: Status::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackup_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GPOID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GPODomain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GPODisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Timestamp(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Comment(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BackupDir(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GenerateReport(this: &Self::This, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn GenerateReportToFile(this: &Self::This, gpmreporttype: GPMReportType, bstrtargetfilepath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMBackup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMBackup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GPOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GPOID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GPODomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GPODomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GPODisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GPODisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Comment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Comment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BackupDir<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackupDir(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateReport(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateReportToFile(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMBackup_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ID: ID::<Identity, Impl, OFFSET>,
            GPOID: GPOID::<Identity, Impl, OFFSET>,
            GPODomain: GPODomain::<Identity, Impl, OFFSET>,
            GPODisplayName: GPODisplayName::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackupCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMBackupCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMBackupCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmbackup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMBackupCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackupDir_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BackupDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetBackup(this: &Self::This, bstrid: &::windows_core::BSTR) -> ::windows_core::Result<IGPMBackup>;
    fn SearchBackups(this: &Self::This, pigpmsearchcriteria: ::core::option::Option<&IGPMSearchCriteria>) -> ::windows_core::Result<IGPMBackupCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMBackupDir {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMBackupDir {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BackupDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackupDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBackup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackup(this, ::core::mem::transmute(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchBackups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmbackupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchBackups(this, ::windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmbackupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMBackupDir_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BackupDirectory: BackupDirectory::<Identity, Impl, OFFSET>,
            GetBackup: GetBackup::<Identity, Impl, OFFSET>,
            SearchBackups: SearchBackups::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMBackupDirEx_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BackupDir(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BackupType(this: &Self::This) -> ::windows_core::Result<GPMBackupType>;
    fn GetBackup(this: &Self::This, bstrid: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SearchBackups(this: &Self::This, pigpmsearchcriteria: ::core::option::Option<&IGPMSearchCriteria>) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMBackupDirEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMBackupDirEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BackupDir<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackupDir(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbackupdir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BackupType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackupType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgpmbackuptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBackup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarbackup: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackup(this, ::core::mem::transmute(&bstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbackup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchBackups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, pvarbackupcollection: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchBackups(this, ::windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbackupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMBackupDirEx_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            BackupType: BackupType::<Identity, Impl, OFFSET>,
            GetBackup: GetBackup::<Identity, Impl, OFFSET>,
            SearchBackups: SearchBackups::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMCSECollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMCSECollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMCSECollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmcses: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmcses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMCSECollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMClientSideExtension_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsUserEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsComputerEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMClientSideExtension {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMClientSideExtension {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsUserEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUserEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsComputerEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMClientSideExtension_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ID: ID::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, Impl, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMConstants_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn PermGPOApply(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermGPORead(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermGPOEdit(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermGPOEditSecurityAndDelete(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermGPOCustom(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterEdit(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterFullControl(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterCustom(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermSOMLink(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermSOMLogging(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermSOMPlanning(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermSOMGPOCreate(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermSOMWMICreate(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermSOMWMIFullControl(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn SearchPropertyGPOPermissions(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOEffectivePermissions(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODisplayName(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOWMIFilter(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOID(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOComputerExtensions(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOUserExtensions(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertySOMLinks(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODomain(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyBackupMostRecent(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchOpEquals(this: &Self::This) -> ::windows_core::Result<GPMSearchOperation>;
    fn SearchOpContains(this: &Self::This) -> ::windows_core::Result<GPMSearchOperation>;
    fn SearchOpNotContains(this: &Self::This) -> ::windows_core::Result<GPMSearchOperation>;
    fn SearchOpNotEquals(this: &Self::This) -> ::windows_core::Result<GPMSearchOperation>;
    fn UsePDC(this: &Self::This) -> ::windows_core::Result<i32>;
    fn UseAnyDC(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DoNotUseW2KDC(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SOMSite(this: &Self::This) -> ::windows_core::Result<GPMSOMType>;
    fn SOMDomain(this: &Self::This) -> ::windows_core::Result<GPMSOMType>;
    fn SOMOU(this: &Self::This) -> ::windows_core::Result<GPMSOMType>;
    fn get_SecurityFlags(this: &Self::This, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<i32>;
    fn DoNotValidateDC(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ReportHTML(this: &Self::This) -> ::windows_core::Result<GPMReportType>;
    fn ReportXML(this: &Self::This) -> ::windows_core::Result<GPMReportType>;
    fn RSOPModeUnknown(this: &Self::This) -> ::windows_core::Result<GPMRSOPMode>;
    fn RSOPModePlanning(this: &Self::This) -> ::windows_core::Result<GPMRSOPMode>;
    fn RSOPModeLogging(this: &Self::This) -> ::windows_core::Result<GPMRSOPMode>;
    fn EntryTypeUser(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
    fn EntryTypeComputer(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
    fn EntryTypeLocalGroup(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
    fn EntryTypeGlobalGroup(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
    fn EntryTypeUniversalGroup(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
    fn EntryTypeUNCPath(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
    fn EntryTypeUnknown(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
    fn DestinationOptionSameAsSource(this: &Self::This) -> ::windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionNone(this: &Self::This) -> ::windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionByRelativeName(this: &Self::This) -> ::windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionSet(this: &Self::This) -> ::windows_core::Result<GPMDestinationOption>;
    fn MigrationTableOnly(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ProcessSecurity(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RsopLoggingNoComputer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RsopLoggingNoUser(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RsopPlanningAssumeSlowLink(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_RsopPlanningLoopbackOption(this: &Self::This, vbmerge: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<i32>;
    fn RsopPlanningAssumeUserWQLFilterTrue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RsopPlanningAssumeCompWQLFilterTrue(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMConstants {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMConstants {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PermGPOApply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermGPOApply(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermGPORead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermGPORead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermGPOEdit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermGPOEdit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermGPOEditSecurityAndDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermGPOEditSecurityAndDelete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermGPOCustom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermGPOCustom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermWMIFilterEdit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermWMIFilterEdit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermWMIFilterFullControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermWMIFilterFullControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermWMIFilterCustom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermWMIFilterCustom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermSOMLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermSOMLink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermSOMLogging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermSOMLogging(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermSOMPlanning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermSOMPlanning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermSOMGPOCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermSOMGPOCreate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermSOMWMICreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermSOMWMICreate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermSOMWMIFullControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermSOMWMIFullControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPOPermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPOPermissions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPOEffectivePermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPOEffectivePermissions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPODisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPODisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPOWMIFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPOWMIFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPOID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPOComputerExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPOComputerExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPOUserExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPOUserExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertySOMLinks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertySOMLinks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyGPODomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyGPODomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyBackupMostRecent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyBackupMostRecent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchOpEquals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchOpEquals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchOpContains<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchOpContains(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchOpNotContains<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchOpNotContains(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchOpNotEquals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchOpNotEquals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UsePDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UsePDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UseAnyDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseAnyDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoNotUseW2KDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoNotUseW2KDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SOMSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SOMSite(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SOMDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SOMDomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SOMOU<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SOMOU(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_SecurityFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_SecurityFlags(this, ::core::mem::transmute_copy(&vbowner), ::core::mem::transmute_copy(&vbgroup), ::core::mem::transmute_copy(&vbdacl), ::core::mem::transmute_copy(&vbsacl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoNotValidateDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoNotValidateDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReportHTML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReportHTML(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReportXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReportXML(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RSOPModeUnknown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RSOPModeUnknown(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RSOPModePlanning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RSOPModePlanning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RSOPModeLogging<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RSOPModeLogging(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryTypeUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryTypeUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryTypeComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryTypeComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryTypeLocalGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryTypeLocalGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryTypeGlobalGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryTypeGlobalGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryTypeUniversalGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryTypeUniversalGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryTypeUNCPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryTypeUNCPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryTypeUnknown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryTypeUnknown(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationOptionSameAsSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationOptionSameAsSource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationOptionNone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationOptionNone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationOptionByRelativeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationOptionByRelativeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationOptionSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationOptionSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MigrationTableOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MigrationTableOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProcessSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProcessSecurity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RsopLoggingNoComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RsopLoggingNoComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RsopLoggingNoUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RsopLoggingNoUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RsopPlanningAssumeSlowLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RsopPlanningAssumeSlowLink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_RsopPlanningLoopbackOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbmerge: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_RsopPlanningLoopbackOption(this, ::core::mem::transmute_copy(&vbmerge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RsopPlanningAssumeUserWQLFilterTrue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RsopPlanningAssumeUserWQLFilterTrue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RsopPlanningAssumeCompWQLFilterTrue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RsopPlanningAssumeCompWQLFilterTrue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMConstants_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PermGPOApply: PermGPOApply::<Identity, Impl, OFFSET>,
            PermGPORead: PermGPORead::<Identity, Impl, OFFSET>,
            PermGPOEdit: PermGPOEdit::<Identity, Impl, OFFSET>,
            PermGPOEditSecurityAndDelete: PermGPOEditSecurityAndDelete::<Identity, Impl, OFFSET>,
            PermGPOCustom: PermGPOCustom::<Identity, Impl, OFFSET>,
            PermWMIFilterEdit: PermWMIFilterEdit::<Identity, Impl, OFFSET>,
            PermWMIFilterFullControl: PermWMIFilterFullControl::<Identity, Impl, OFFSET>,
            PermWMIFilterCustom: PermWMIFilterCustom::<Identity, Impl, OFFSET>,
            PermSOMLink: PermSOMLink::<Identity, Impl, OFFSET>,
            PermSOMLogging: PermSOMLogging::<Identity, Impl, OFFSET>,
            PermSOMPlanning: PermSOMPlanning::<Identity, Impl, OFFSET>,
            PermSOMGPOCreate: PermSOMGPOCreate::<Identity, Impl, OFFSET>,
            PermSOMWMICreate: PermSOMWMICreate::<Identity, Impl, OFFSET>,
            PermSOMWMIFullControl: PermSOMWMIFullControl::<Identity, Impl, OFFSET>,
            SearchPropertyGPOPermissions: SearchPropertyGPOPermissions::<Identity, Impl, OFFSET>,
            SearchPropertyGPOEffectivePermissions: SearchPropertyGPOEffectivePermissions::<Identity, Impl, OFFSET>,
            SearchPropertyGPODisplayName: SearchPropertyGPODisplayName::<Identity, Impl, OFFSET>,
            SearchPropertyGPOWMIFilter: SearchPropertyGPOWMIFilter::<Identity, Impl, OFFSET>,
            SearchPropertyGPOID: SearchPropertyGPOID::<Identity, Impl, OFFSET>,
            SearchPropertyGPOComputerExtensions: SearchPropertyGPOComputerExtensions::<Identity, Impl, OFFSET>,
            SearchPropertyGPOUserExtensions: SearchPropertyGPOUserExtensions::<Identity, Impl, OFFSET>,
            SearchPropertySOMLinks: SearchPropertySOMLinks::<Identity, Impl, OFFSET>,
            SearchPropertyGPODomain: SearchPropertyGPODomain::<Identity, Impl, OFFSET>,
            SearchPropertyBackupMostRecent: SearchPropertyBackupMostRecent::<Identity, Impl, OFFSET>,
            SearchOpEquals: SearchOpEquals::<Identity, Impl, OFFSET>,
            SearchOpContains: SearchOpContains::<Identity, Impl, OFFSET>,
            SearchOpNotContains: SearchOpNotContains::<Identity, Impl, OFFSET>,
            SearchOpNotEquals: SearchOpNotEquals::<Identity, Impl, OFFSET>,
            UsePDC: UsePDC::<Identity, Impl, OFFSET>,
            UseAnyDC: UseAnyDC::<Identity, Impl, OFFSET>,
            DoNotUseW2KDC: DoNotUseW2KDC::<Identity, Impl, OFFSET>,
            SOMSite: SOMSite::<Identity, Impl, OFFSET>,
            SOMDomain: SOMDomain::<Identity, Impl, OFFSET>,
            SOMOU: SOMOU::<Identity, Impl, OFFSET>,
            get_SecurityFlags: get_SecurityFlags::<Identity, Impl, OFFSET>,
            DoNotValidateDC: DoNotValidateDC::<Identity, Impl, OFFSET>,
            ReportHTML: ReportHTML::<Identity, Impl, OFFSET>,
            ReportXML: ReportXML::<Identity, Impl, OFFSET>,
            RSOPModeUnknown: RSOPModeUnknown::<Identity, Impl, OFFSET>,
            RSOPModePlanning: RSOPModePlanning::<Identity, Impl, OFFSET>,
            RSOPModeLogging: RSOPModeLogging::<Identity, Impl, OFFSET>,
            EntryTypeUser: EntryTypeUser::<Identity, Impl, OFFSET>,
            EntryTypeComputer: EntryTypeComputer::<Identity, Impl, OFFSET>,
            EntryTypeLocalGroup: EntryTypeLocalGroup::<Identity, Impl, OFFSET>,
            EntryTypeGlobalGroup: EntryTypeGlobalGroup::<Identity, Impl, OFFSET>,
            EntryTypeUniversalGroup: EntryTypeUniversalGroup::<Identity, Impl, OFFSET>,
            EntryTypeUNCPath: EntryTypeUNCPath::<Identity, Impl, OFFSET>,
            EntryTypeUnknown: EntryTypeUnknown::<Identity, Impl, OFFSET>,
            DestinationOptionSameAsSource: DestinationOptionSameAsSource::<Identity, Impl, OFFSET>,
            DestinationOptionNone: DestinationOptionNone::<Identity, Impl, OFFSET>,
            DestinationOptionByRelativeName: DestinationOptionByRelativeName::<Identity, Impl, OFFSET>,
            DestinationOptionSet: DestinationOptionSet::<Identity, Impl, OFFSET>,
            MigrationTableOnly: MigrationTableOnly::<Identity, Impl, OFFSET>,
            ProcessSecurity: ProcessSecurity::<Identity, Impl, OFFSET>,
            RsopLoggingNoComputer: RsopLoggingNoComputer::<Identity, Impl, OFFSET>,
            RsopLoggingNoUser: RsopLoggingNoUser::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeSlowLink: RsopPlanningAssumeSlowLink::<Identity, Impl, OFFSET>,
            get_RsopPlanningLoopbackOption: get_RsopPlanningLoopbackOption::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeUserWQLFilterTrue: RsopPlanningAssumeUserWQLFilterTrue::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeCompWQLFilterTrue: RsopPlanningAssumeCompWQLFilterTrue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMConstants2_Impl: ::windows_core::BaseImpl + IGPMConstants_Impl {
    fn BackupTypeGPO(this: &Self::This) -> ::windows_core::Result<GPMBackupType>;
    fn BackupTypeStarterGPO(this: &Self::This) -> ::windows_core::Result<GPMBackupType>;
    fn StarterGPOTypeSystem(this: &Self::This) -> ::windows_core::Result<GPMStarterGPOType>;
    fn StarterGPOTypeCustom(this: &Self::This) -> ::windows_core::Result<GPMStarterGPOType>;
    fn SearchPropertyStarterGPOPermissions(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOEffectivePermissions(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODisplayName(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOID(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODomain(this: &Self::This) -> ::windows_core::Result<GPMSearchProperty>;
    fn PermStarterGPORead(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOEdit(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOFullControl(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOCustom(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn ReportLegacy(this: &Self::This) -> ::windows_core::Result<GPMReportingOptions>;
    fn ReportComments(this: &Self::This) -> ::windows_core::Result<GPMReportingOptions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMConstants2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IGPMConstants);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMConstants2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BackupTypeGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackupTypeGPO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BackupTypeStarterGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackupTypeStarterGPO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StarterGPOTypeSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StarterGPOTypeSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StarterGPOTypeCustom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StarterGPOTypeCustom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyStarterGPOPermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyStarterGPOPermissions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyStarterGPOEffectivePermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyStarterGPOEffectivePermissions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyStarterGPODisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyStarterGPODisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyStarterGPOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyStarterGPOID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchPropertyStarterGPODomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchPropertyStarterGPODomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermStarterGPORead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermStarterGPORead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermStarterGPOEdit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermStarterGPOEdit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermStarterGPOFullControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermStarterGPOFullControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PermStarterGPOCustom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermStarterGPOCustom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReportLegacy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReportLegacy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReportComments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReportComments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMConstants2_Vtbl {
            base__: <IGPMConstants as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BackupTypeGPO: BackupTypeGPO::<Identity, Impl, OFFSET>,
            BackupTypeStarterGPO: BackupTypeStarterGPO::<Identity, Impl, OFFSET>,
            StarterGPOTypeSystem: StarterGPOTypeSystem::<Identity, Impl, OFFSET>,
            StarterGPOTypeCustom: StarterGPOTypeCustom::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOPermissions: SearchPropertyStarterGPOPermissions::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOEffectivePermissions: SearchPropertyStarterGPOEffectivePermissions::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPODisplayName: SearchPropertyStarterGPODisplayName::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOID: SearchPropertyStarterGPOID::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPODomain: SearchPropertyStarterGPODomain::<Identity, Impl, OFFSET>,
            PermStarterGPORead: PermStarterGPORead::<Identity, Impl, OFFSET>,
            PermStarterGPOEdit: PermStarterGPOEdit::<Identity, Impl, OFFSET>,
            PermStarterGPOFullControl: PermStarterGPOFullControl::<Identity, Impl, OFFSET>,
            PermStarterGPOCustom: PermStarterGPOCustom::<Identity, Impl, OFFSET>,
            ReportLegacy: ReportLegacy::<Identity, Impl, OFFSET>,
            ReportComments: ReportComments::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMDomain_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DomainController(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Domain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateGPO(this: &Self::This) -> ::windows_core::Result<IGPMGPO>;
    fn GetGPO(this: &Self::This, bstrguid: &::windows_core::BSTR) -> ::windows_core::Result<IGPMGPO>;
    fn SearchGPOs(this: &Self::This, pigpmsearchcriteria: ::core::option::Option<&IGPMSearchCriteria>) -> ::windows_core::Result<IGPMGPOCollection>;
    fn RestoreGPO(this: &Self::This, pigpmbackup: ::core::option::Option<&IGPMBackup>, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn GetSOM(this: &Self::This, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMSOM>;
    fn SearchSOMs(this: &Self::This, pigpmsearchcriteria: ::core::option::Option<&IGPMSearchCriteria>) -> ::windows_core::Result<IGPMSOMCollection>;
    fn GetWMIFilter(this: &Self::This, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMWMIFilter>;
    fn SearchWMIFilters(this: &Self::This, pigpmsearchcriteria: ::core::option::Option<&IGPMSearchCriteria>) -> ::windows_core::Result<IGPMWMIFilterCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMDomain {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMDomain {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DomainController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainController(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Domain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Domain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGPO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewgpo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppgpo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGPO(this, ::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgpo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchGPOs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmgpocollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchGPOs(this, ::windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmgpocollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RestoreGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmbackup: *mut ::core::ffi::c_void, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreGPO(this, ::windows_core::from_raw_borrowed(&pigpmbackup), ::core::mem::transmute_copy(&ldcflags), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn GetSOM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSOM(this, ::core::mem::transmute(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchSOMs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchSOMs(this, ::windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsomcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWMIFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWMIFilter(this, ::core::mem::transmute(&bstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwmifilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchWMIFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmwmifiltercollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchWMIFilters(this, ::windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmwmifiltercollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMDomain_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DomainController: DomainController::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            CreateGPO: CreateGPO::<Identity, Impl, OFFSET>,
            GetGPO: GetGPO::<Identity, Impl, OFFSET>,
            SearchGPOs: SearchGPOs::<Identity, Impl, OFFSET>,
            RestoreGPO: RestoreGPO::<Identity, Impl, OFFSET>,
            GetSOM: GetSOM::<Identity, Impl, OFFSET>,
            SearchSOMs: SearchSOMs::<Identity, Impl, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, Impl, OFFSET>,
            SearchWMIFilters: SearchWMIFilters::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMDomain2_Impl: ::windows_core::BaseImpl + IGPMDomain_Impl {
    fn CreateStarterGPO(this: &Self::This) -> ::windows_core::Result<IGPMStarterGPO>;
    fn CreateGPOFromStarterGPO(this: &Self::This, pgpotemplate: ::core::option::Option<&IGPMStarterGPO>) -> ::windows_core::Result<IGPMGPO>;
    fn GetStarterGPO(this: &Self::This, bstrguid: &::windows_core::BSTR) -> ::windows_core::Result<IGPMStarterGPO>;
    fn SearchStarterGPOs(this: &Self::This, pigpmsearchcriteria: ::core::option::Option<&IGPMSearchCriteria>) -> ::windows_core::Result<IGPMStarterGPOCollection>;
    fn LoadStarterGPO(this: &Self::This, bstrloadfile: &::windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn RestoreStarterGPO(this: &Self::This, pigpmtmplbackup: ::core::option::Option<&IGPMStarterGPOBackup>, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMDomain2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IGPMDomain);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMDomain2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateStarterGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStarterGPO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewtemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGPOFromStarterGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgpotemplate: *mut ::core::ffi::c_void, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGPOFromStarterGPO(this, ::windows_core::from_raw_borrowed(&pgpotemplate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewgpo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStarterGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStarterGPO(this, ::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchStarterGPOs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmtemplatecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchStarterGPOs(this, ::windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtemplatecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadStarterGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrloadfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadStarterGPO(this, ::core::mem::transmute(&bstrloadfile), ::core::mem::transmute_copy(&boverwrite), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn RestoreStarterGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmtmplbackup: *mut ::core::ffi::c_void, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreStarterGPO(this, ::windows_core::from_raw_borrowed(&pigpmtmplbackup), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        IGPMDomain2_Vtbl {
            base__: <IGPMDomain as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateStarterGPO: CreateStarterGPO::<Identity, Impl, OFFSET>,
            CreateGPOFromStarterGPO: CreateGPOFromStarterGPO::<Identity, Impl, OFFSET>,
            GetStarterGPO: GetStarterGPO::<Identity, Impl, OFFSET>,
            SearchStarterGPOs: SearchStarterGPOs::<Identity, Impl, OFFSET>,
            LoadStarterGPO: LoadStarterGPO::<Identity, Impl, OFFSET>,
            RestoreStarterGPO: RestoreStarterGPO::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMDomain3_Impl: ::windows_core::BaseImpl + IGPMDomain2_Impl {
    fn GenerateReport(this: &Self::This, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn InfrastructureDC(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetInfrastructureDC(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetInfrastructureFlags(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMDomain3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IGPMDomain2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMDomain3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GenerateReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateReport(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn InfrastructureDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InfrastructureDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfrastructureDC(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfrastructureFlags(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        IGPMDomain3_Vtbl {
            base__: <IGPMDomain2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            InfrastructureDC: InfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPO_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplayName(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreationTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ModificationTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn UserDSVersionNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ComputerDSVersionNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn UserSysvolVersionNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ComputerSysvolVersionNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetWMIFilter(this: &Self::This) -> ::windows_core::Result<IGPMWMIFilter>;
    fn SetWMIFilter(this: &Self::This, pigpmwmifilter: ::core::option::Option<&IGPMWMIFilter>) -> ::windows_core::Result<()>;
    fn SetUserEnabled(this: &Self::This, vbenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetComputerEnabled(this: &Self::This, vbenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsUserEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsComputerEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSecurityInfo(this: &Self::This) -> ::windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(this: &Self::This, psecurityinfo: ::core::option::Option<&IGPMSecurityInfo>) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Backup(this: &Self::This, bstrbackupdir: &::windows_core::BSTR, bstrcomment: &::windows_core::BSTR, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn Import(this: &Self::This, lflags: i32, pigpmbackup: ::core::option::Option<&IGPMBackup>, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn GenerateReport(this: &Self::This, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn GenerateReportToFile(this: &Self::This, gpmreporttype: GPMReportType, bstrtargetfilepath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMResult>;
    fn CopyTo(this: &Self::This, lflags: i32, pigpmdomain: ::core::option::Option<&IGPMDomain>, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn SetSecurityDescriptor(this: &Self::This, lflags: i32, psd: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn GetSecurityDescriptor(this: &Self::This, lflags: i32) -> ::windows_core::Result<super::Com::IDispatch>;
    fn IsACLConsistent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MakeACLConsistent(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMGPO {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMGPO {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModificationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModificationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserDSVersionNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserDSVersionNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputerDSVersionNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputerDSVersionNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserSysvolVersionNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserSysvolVersionNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputerSysvolVersionNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputerSysvolVersionNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWMIFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWMIFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmwmifilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWMIFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmwmifilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWMIFilter(this, ::windows_core::from_raw_borrowed(&pigpmwmifilter)).into())
        }
        unsafe extern "system" fn SetUserEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserEnabled(this, ::core::mem::transmute_copy(&vbenabled)).into())
        }
        unsafe extern "system" fn SetComputerEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputerEnabled(this, ::core::mem::transmute_copy(&vbenabled)).into())
        }
        unsafe extern "system" fn IsUserEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUserEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsComputerEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityInfo(this, ::windows_core::from_raw_borrowed(&psecurityinfo)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Backup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcomment: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Backup(this, ::core::mem::transmute(&bstrbackupdir), ::core::mem::transmute(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: *mut ::core::ffi::c_void, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Import(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pigpmbackup), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateReport(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateReportToFile(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTo(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pigpmdomain), ::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvarmigrationtable), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, psd: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityDescriptor(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&psd)).into())
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityDescriptor(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsACLConsistent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbconsistent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsACLConsistent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbconsistent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MakeACLConsistent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeACLConsistent(this).into())
        }
        IGPMGPO_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            ModificationTime: ModificationTime::<Identity, Impl, OFFSET>,
            UserDSVersionNumber: UserDSVersionNumber::<Identity, Impl, OFFSET>,
            ComputerDSVersionNumber: ComputerDSVersionNumber::<Identity, Impl, OFFSET>,
            UserSysvolVersionNumber: UserSysvolVersionNumber::<Identity, Impl, OFFSET>,
            ComputerSysvolVersionNumber: ComputerSysvolVersionNumber::<Identity, Impl, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, Impl, OFFSET>,
            SetWMIFilter: SetWMIFilter::<Identity, Impl, OFFSET>,
            SetUserEnabled: SetUserEnabled::<Identity, Impl, OFFSET>,
            SetComputerEnabled: SetComputerEnabled::<Identity, Impl, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, Impl, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            IsACLConsistent: IsACLConsistent::<Identity, Impl, OFFSET>,
            MakeACLConsistent: MakeACLConsistent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPO2_Impl: ::windows_core::BaseImpl + IGPMGPO_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMGPO2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IGPMGPO);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMGPO2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&newval)).into())
        }
        IGPMGPO2_Vtbl {
            base__: <IGPMGPO as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPO3_Impl: ::windows_core::BaseImpl + IGPMGPO2_Impl {
    fn InfrastructureDC(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetInfrastructureDC(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetInfrastructureFlags(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMGPO3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IGPMGPO2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMGPO3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InfrastructureDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InfrastructureDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfrastructureDC(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfrastructureFlags(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        IGPMGPO3_Vtbl {
            base__: <IGPMGPO2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InfrastructureDC: InfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPOCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMGPOCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMGPOCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmgpos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMGPOCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPOLink_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GPOID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GPODomain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Enforced(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnforced(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SOMLinkOrder(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SOM(this: &Self::This) -> ::windows_core::Result<IGPMSOM>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMGPOLink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMGPOLink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GPOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GPOID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GPODomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GPODomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Enforced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enforced(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnforced<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnforced(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn SOMLinkOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SOMLinkOrder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SOM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SOM(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IGPMGPOLink_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GPOID: GPOID::<Identity, Impl, OFFSET>,
            GPODomain: GPODomain::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Enforced: Enforced::<Identity, Impl, OFFSET>,
            SetEnforced: SetEnforced::<Identity, Impl, OFFSET>,
            SOMLinkOrder: SOMLinkOrder::<Identity, Impl, OFFSET>,
            SOM: SOM::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMGPOLinksCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMGPOLinksCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMGPOLinksCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmlinks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMGPOLinksCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMMapEntry_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Source(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Destination(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DestinationOption(this: &Self::This) -> ::windows_core::Result<GPMDestinationOption>;
    fn EntryType(this: &Self::This) -> ::windows_core::Result<GPMEntryType>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMMapEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMMapEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Source<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Source(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Destination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdestination: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Destination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdestination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationOption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgpmdestoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EntryType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EntryType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgpmentrytype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMMapEntry_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Source: Source::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            DestinationOption: DestinationOption::<Identity, Impl, OFFSET>,
            EntryType: EntryType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMMapEntryCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMMapEntryCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMMapEntryCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMMapEntryCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMMigrationTable_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Save(this: &Self::This, bstrmigrationtablepath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, lflags: i32, var: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddEntry(this: &Self::This, bstrsource: &::windows_core::BSTR, gpmentrytype: GPMEntryType, pvardestination: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMMapEntry>;
    fn GetEntry(this: &Self::This, bstrsource: &::windows_core::BSTR) -> ::windows_core::Result<IGPMMapEntry>;
    fn DeleteEntry(this: &Self::This, bstrsource: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UpdateDestination(this: &Self::This, bstrsource: &::windows_core::BSTR, pvardestination: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMMapEntry>;
    fn Validate(this: &Self::This) -> ::windows_core::Result<IGPMResult>;
    fn GetEntries(this: &Self::This) -> ::windows_core::Result<IGPMMapEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMMigrationTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMMigrationTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::core::mem::transmute(&bstrmigrationtablepath)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, var: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn AddEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Variant::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddEntry(this, ::core::mem::transmute(&bstrsource), ::core::mem::transmute_copy(&gpmentrytype), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEntry(this, ::core::mem::transmute(&bstrsource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteEntry(this, ::core::mem::transmute(&bstrsource)).into())
        }
        unsafe extern "system" fn UpdateDestination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvardestination: *const super::Variant::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UpdateDestination(this, ::core::mem::transmute(&bstrsource), ::core::mem::transmute_copy(&pvardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Validate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppentries: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEntries(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppentries, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMMigrationTable_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Save: Save::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddEntry: AddEntry::<Identity, Impl, OFFSET>,
            GetEntry: GetEntry::<Identity, Impl, OFFSET>,
            DeleteEntry: DeleteEntry::<Identity, Impl, OFFSET>,
            UpdateDestination: UpdateDestination::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
            GetEntries: GetEntries::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMPermission_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Inherited(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Inheritable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Denied(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Permission(this: &Self::This) -> ::windows_core::Result<GPMPermissionType>;
    fn Trustee(this: &Self::This) -> ::windows_core::Result<IGPMTrustee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMPermission {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMPermission {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Inherited<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Inherited(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Inheritable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Inheritable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Denied<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Denied(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Permission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Permission(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Trustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Trustee(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtrustee, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMPermission_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Inherited: Inherited::<Identity, Impl, OFFSET>,
            Inheritable: Inheritable::<Identity, Impl, OFFSET>,
            Denied: Denied::<Identity, Impl, OFFSET>,
            Permission: Permission::<Identity, Impl, OFFSET>,
            Trustee: Trustee::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMRSOP_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Mode(this: &Self::This) -> ::windows_core::Result<GPMRSOPMode>;
    fn Namespace(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLoggingComputer(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LoggingComputer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLoggingUser(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LoggingUser(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLoggingFlags(this: &Self::This, lval: i32) -> ::windows_core::Result<()>;
    fn LoggingFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPlanningFlags(this: &Self::This, lval: i32) -> ::windows_core::Result<()>;
    fn PlanningFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPlanningDomainController(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlanningDomainController(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPlanningSiteName(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlanningSiteName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPlanningUser(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlanningUser(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPlanningUserSOM(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlanningUserSOM(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPlanningUserWMIFilters(this: &Self::This, varval: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PlanningUserWMIFilters(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetPlanningUserSecurityGroups(this: &Self::This, varval: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PlanningUserSecurityGroups(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetPlanningComputer(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlanningComputer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPlanningComputerSOM(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlanningComputerSOM(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPlanningComputerWMIFilters(this: &Self::This, varval: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PlanningComputerWMIFilters(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetPlanningComputerSecurityGroups(this: &Self::This, varval: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PlanningComputerSecurityGroups(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn LoggingEnumerateUsers(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn CreateQueryResults(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReleaseQueryResults(this: &Self::This) -> ::windows_core::Result<()>;
    fn GenerateReport(this: &Self::This, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn GenerateReportToFile(this: &Self::This, gpmreporttype: GPMReportType, bstrtargetfilepath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMRSOP {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMRSOP {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Mode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Mode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Namespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Namespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLoggingComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoggingComputer(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn LoggingComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoggingComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLoggingUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoggingUser(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn LoggingUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoggingUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLoggingFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoggingFlags(this, ::core::mem::transmute_copy(&lval)).into())
        }
        unsafe extern "system" fn LoggingFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoggingFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningFlags(this, ::core::mem::transmute_copy(&lval)).into())
        }
        unsafe extern "system" fn PlanningFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningDomainController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningDomainController(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn PlanningDomainController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningDomainController(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningSiteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningSiteName(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn PlanningSiteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningSiteName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningUser(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn PlanningUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningUserSOM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningUserSOM(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn PlanningUserSOM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningUserSOM(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningUserWMIFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningUserWMIFilters(this, ::core::mem::transmute(&varval)).into())
        }
        unsafe extern "system" fn PlanningUserWMIFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningUserWMIFilters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningUserSecurityGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningUserSecurityGroups(this, ::core::mem::transmute(&varval)).into())
        }
        unsafe extern "system" fn PlanningUserSecurityGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningUserSecurityGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningComputer(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn PlanningComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningComputerSOM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningComputerSOM(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn PlanningComputerSOM<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningComputerSOM(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningComputerWMIFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningComputerWMIFilters(this, ::core::mem::transmute(&varval)).into())
        }
        unsafe extern "system" fn PlanningComputerWMIFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningComputerWMIFilters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPlanningComputerSecurityGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlanningComputerSecurityGroups(this, ::core::mem::transmute(&varval)).into())
        }
        unsafe extern "system" fn PlanningComputerSecurityGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlanningComputerSecurityGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoggingEnumerateUsers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoggingEnumerateUsers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateQueryResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateQueryResults(this).into())
        }
        unsafe extern "system" fn ReleaseQueryResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseQueryResults(this).into())
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateReport(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateReportToFile(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMRSOP_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Mode: Mode::<Identity, Impl, OFFSET>,
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            SetLoggingComputer: SetLoggingComputer::<Identity, Impl, OFFSET>,
            LoggingComputer: LoggingComputer::<Identity, Impl, OFFSET>,
            SetLoggingUser: SetLoggingUser::<Identity, Impl, OFFSET>,
            LoggingUser: LoggingUser::<Identity, Impl, OFFSET>,
            SetLoggingFlags: SetLoggingFlags::<Identity, Impl, OFFSET>,
            LoggingFlags: LoggingFlags::<Identity, Impl, OFFSET>,
            SetPlanningFlags: SetPlanningFlags::<Identity, Impl, OFFSET>,
            PlanningFlags: PlanningFlags::<Identity, Impl, OFFSET>,
            SetPlanningDomainController: SetPlanningDomainController::<Identity, Impl, OFFSET>,
            PlanningDomainController: PlanningDomainController::<Identity, Impl, OFFSET>,
            SetPlanningSiteName: SetPlanningSiteName::<Identity, Impl, OFFSET>,
            PlanningSiteName: PlanningSiteName::<Identity, Impl, OFFSET>,
            SetPlanningUser: SetPlanningUser::<Identity, Impl, OFFSET>,
            PlanningUser: PlanningUser::<Identity, Impl, OFFSET>,
            SetPlanningUserSOM: SetPlanningUserSOM::<Identity, Impl, OFFSET>,
            PlanningUserSOM: PlanningUserSOM::<Identity, Impl, OFFSET>,
            SetPlanningUserWMIFilters: SetPlanningUserWMIFilters::<Identity, Impl, OFFSET>,
            PlanningUserWMIFilters: PlanningUserWMIFilters::<Identity, Impl, OFFSET>,
            SetPlanningUserSecurityGroups: SetPlanningUserSecurityGroups::<Identity, Impl, OFFSET>,
            PlanningUserSecurityGroups: PlanningUserSecurityGroups::<Identity, Impl, OFFSET>,
            SetPlanningComputer: SetPlanningComputer::<Identity, Impl, OFFSET>,
            PlanningComputer: PlanningComputer::<Identity, Impl, OFFSET>,
            SetPlanningComputerSOM: SetPlanningComputerSOM::<Identity, Impl, OFFSET>,
            PlanningComputerSOM: PlanningComputerSOM::<Identity, Impl, OFFSET>,
            SetPlanningComputerWMIFilters: SetPlanningComputerWMIFilters::<Identity, Impl, OFFSET>,
            PlanningComputerWMIFilters: PlanningComputerWMIFilters::<Identity, Impl, OFFSET>,
            SetPlanningComputerSecurityGroups: SetPlanningComputerSecurityGroups::<Identity, Impl, OFFSET>,
            PlanningComputerSecurityGroups: PlanningComputerSecurityGroups::<Identity, Impl, OFFSET>,
            LoggingEnumerateUsers: LoggingEnumerateUsers::<Identity, Impl, OFFSET>,
            CreateQueryResults: CreateQueryResults::<Identity, Impl, OFFSET>,
            ReleaseQueryResults: ReleaseQueryResults::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMResult_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<IGPMStatusMsgCollection>;
    fn Result(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn OverallStatus(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmstatusmsgcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Result<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Result(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OverallStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OverallStatus(this).into())
        }
        IGPMResult_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            Result: Result::<Identity, Impl, OFFSET>,
            OverallStatus: OverallStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSOM_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GPOInheritanceBlocked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGPOInheritanceBlocked(this: &Self::This, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateGPOLink(this: &Self::This, llinkpos: i32, pgpo: ::core::option::Option<&IGPMGPO>) -> ::windows_core::Result<IGPMGPOLink>;
    fn Type(this: &Self::This) -> ::windows_core::Result<GPMSOMType>;
    fn GetGPOLinks(this: &Self::This) -> ::windows_core::Result<IGPMGPOLinksCollection>;
    fn GetInheritedGPOLinks(this: &Self::This) -> ::windows_core::Result<IGPMGPOLinksCollection>;
    fn GetSecurityInfo(this: &Self::This) -> ::windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(this: &Self::This, psecurityinfo: ::core::option::Option<&IGPMSecurityInfo>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMSOM {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMSOM {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GPOInheritanceBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GPOInheritanceBlocked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGPOInheritanceBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGPOInheritanceBlocked(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGPOLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: *mut ::core::ffi::c_void, ppnewgpolink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGPOLink(this, ::core::mem::transmute_copy(&llinkpos), ::windows_core::from_raw_borrowed(&pgpo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewgpolink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGPOLinks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGPOLinks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgpolinks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInheritedGPOLinks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInheritedGPOLinks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgpolinks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityInfo(this, ::windows_core::from_raw_borrowed(&psecurityinfo)).into())
        }
        IGPMSOM_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GPOInheritanceBlocked: GPOInheritanceBlocked::<Identity, Impl, OFFSET>,
            SetGPOInheritanceBlocked: SetGPOInheritanceBlocked::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            CreateGPOLink: CreateGPOLink::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            GetGPOLinks: GetGPOLinks::<Identity, Impl, OFFSET>,
            GetInheritedGPOLinks: GetInheritedGPOLinks::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSOMCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMSOMCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMSOMCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMSOMCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSearchCriteria_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Add(this: &Self::This, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMSearchCriteria {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSearchCriteria_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMSearchCriteria {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSearchCriteria_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&searchproperty), ::core::mem::transmute_copy(&searchoperation), ::core::mem::transmute(&varvalue)).into())
        }
        IGPMSearchCriteria_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Add: Add::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSecurityInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
    fn Add(this: &Self::This, pperm: ::core::option::Option<&IGPMPermission>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, pperm: ::core::option::Option<&IGPMPermission>) -> ::windows_core::Result<()>;
    fn RemoveTrustee(this: &Self::This, bstrtrustee: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMSecurityInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMSecurityInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&pperm)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::windows_core::from_raw_borrowed(&pperm)).into())
        }
        unsafe extern "system" fn RemoveTrustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveTrustee(this, ::core::mem::transmute(&bstrtrustee)).into())
        }
        IGPMSecurityInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveTrustee: RemoveTrustee::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMSitesContainer_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DomainController(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Domain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Forest(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSite(this: &Self::This, bstrsitename: &::windows_core::BSTR) -> ::windows_core::Result<IGPMSOM>;
    fn SearchSites(this: &Self::This, pigpmsearchcriteria: ::core::option::Option<&IGPMSearchCriteria>) -> ::windows_core::Result<IGPMSOMCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMSitesContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMSitesContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DomainController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainController(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Domain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Domain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Forest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Forest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsitename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSite(this, ::core::mem::transmute(&bstrsitename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchSites<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchSites(this, ::windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmsomcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMSitesContainer_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DomainController: DomainController::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            Forest: Forest::<Identity, Impl, OFFSET>,
            GetSite: GetSite::<Identity, Impl, OFFSET>,
            SearchSites: SearchSites::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPO_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplayName(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Author(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Product(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreationTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModifiedTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Type(this: &Self::This) -> ::windows_core::Result<GPMStarterGPOType>;
    fn ComputerVersion(this: &Self::This) -> ::windows_core::Result<u16>;
    fn UserVersion(this: &Self::This) -> ::windows_core::Result<u16>;
    fn StarterGPOVersion(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, bstrsavefile: &::windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, bsaveassystem: super::super::Foundation::VARIANT_BOOL, bstrlanguage: *const super::Variant::VARIANT, bstrauthor: *const super::Variant::VARIANT, bstrproduct: *const super::Variant::VARIANT, bstruniqueid: *const super::Variant::VARIANT, bstrversion: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn Backup(this: &Self::This, bstrbackupdir: &::windows_core::BSTR, bstrcomment: &::windows_core::BSTR, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn CopyTo(this: &Self::This, pvarnewdisplayname: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMResult>;
    fn GenerateReport(this: &Self::This, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(this: &Self::This, gpmreporttype: GPMReportType, bstrtargetfilepath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMResult>;
    fn GetSecurityInfo(this: &Self::This) -> ::windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(this: &Self::This, psecurityinfo: ::core::option::Option<&IGPMSecurityInfo>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMStarterGPO {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMStarterGPO {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn Author<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Author(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Product<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Product(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifiedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModifiedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputerVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputerVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StarterGPOVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StarterGPOVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsavefile: ::std::mem::MaybeUninit<::windows_core::BSTR>, boverwrite: super::super::Foundation::VARIANT_BOOL, bsaveassystem: super::super::Foundation::VARIANT_BOOL, bstrlanguage: *const super::Variant::VARIANT, bstrauthor: *const super::Variant::VARIANT, bstrproduct: *const super::Variant::VARIANT, bstruniqueid: *const super::Variant::VARIANT, bstrversion: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::Save(
                    this,
                    ::core::mem::transmute(&bstrsavefile),
                    ::core::mem::transmute_copy(&boverwrite),
                    ::core::mem::transmute_copy(&bsaveassystem),
                    ::core::mem::transmute_copy(&bstrlanguage),
                    ::core::mem::transmute_copy(&bstrauthor),
                    ::core::mem::transmute_copy(&bstrproduct),
                    ::core::mem::transmute_copy(&bstruniqueid),
                    ::core::mem::transmute_copy(&bstrversion),
                    ::core::mem::transmute_copy(&pvargpmprogress),
                    ::core::mem::transmute_copy(&pvargpmcancel),
                    ::core::mem::transmute_copy(&ppigpmresult),
                )
                .into()
            })
        }
        unsafe extern "system" fn Backup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcomment: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Backup(this, ::core::mem::transmute(&bstrbackupdir), ::core::mem::transmute(&bstrcomment), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyTo(this, ::core::mem::transmute_copy(&pvarnewdisplayname), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateReport(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateReportToFile(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityInfo(this, ::windows_core::from_raw_borrowed(&psecurityinfo)).into())
        }
        IGPMStarterGPO_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Product: Product::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            ModifiedTime: ModifiedTime::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            ComputerVersion: ComputerVersion::<Identity, Impl, OFFSET>,
            UserVersion: UserVersion::<Identity, Impl, OFFSET>,
            StarterGPOVersion: StarterGPOVersion::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPOBackup_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BackupDir(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Comment(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Domain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn StarterGPOID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Timestamp(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Type(this: &Self::This) -> ::windows_core::Result<GPMStarterGPOType>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GenerateReport(this: &Self::This, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>;
    fn GenerateReportToFile(this: &Self::This, gpmreporttype: GPMReportType, bstrtargetfilepath: &::windows_core::BSTR) -> ::windows_core::Result<IGPMResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMStarterGPOBackup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMStarterGPOBackup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BackupDir<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackupDir(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbackupdir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Comment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcomment: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Comment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcomment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Domain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Domain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtemplatedomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StarterGPOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StarterGPOID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtemplateid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptimestamp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn GenerateReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateReport(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute_copy(&pvargpmprogress), ::core::mem::transmute_copy(&pvargpmcancel), ::core::mem::transmute_copy(&ppigpmresult)).into())
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateReportToFile(this, ::core::mem::transmute_copy(&gpmreporttype), ::core::mem::transmute(&bstrtargetfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMStarterGPOBackup_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            StarterGPOID: StarterGPOID::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPOBackupCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMStarterGPOBackupCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMStarterGPOBackupCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtmplbackup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMStarterGPOBackupCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStarterGPOCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMStarterGPOCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMStarterGPOCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppigpmtemplates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMStarterGPOCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStatusMessage_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ObjectPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ErrorCode(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExtensionName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettingsName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn OperationCode(this: &Self::This) -> ::windows_core::Result<()>;
    fn Message(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMStatusMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMStatusMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ObjectPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ErrorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ErrorCode(this).into())
        }
        unsafe extern "system" fn ExtensionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtensionName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SettingsName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SettingsName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OperationCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OperationCode(this).into())
        }
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMStatusMessage_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ObjectPath: ObjectPath::<Identity, Impl, OFFSET>,
            ErrorCode: ErrorCode::<Identity, Impl, OFFSET>,
            ExtensionName: ExtensionName::<Identity, Impl, OFFSET>,
            SettingsName: SettingsName::<Identity, Impl, OFFSET>,
            OperationCode: OperationCode::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMStatusMsgCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMStatusMsgCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMStatusMsgCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMStatusMsgCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMTrustee_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn TrusteeSid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TrusteeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TrusteeDomain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TrusteeDSPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TrusteeType(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMTrustee {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMTrustee {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TrusteeSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrusteeSid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrusteeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrusteeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrusteeDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrusteeDomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrusteeDSPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrusteeDSPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrusteeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrusteeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMTrustee_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TrusteeSid: TrusteeSid::<Identity, Impl, OFFSET>,
            TrusteeName: TrusteeName::<Identity, Impl, OFFSET>,
            TrusteeDomain: TrusteeDomain::<Identity, Impl, OFFSET>,
            TrusteeDSPath: TrusteeDSPath::<Identity, Impl, OFFSET>,
            TrusteeType: TrusteeType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMWMIFilter_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetQueryList(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetSecurityInfo(this: &Self::This) -> ::windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(this: &Self::This, psecurityinfo: ::core::option::Option<&IGPMSecurityInfo>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMWMIFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMWMIFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&newval)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetQueryList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQueryList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pqrylist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityInfo(this, ::windows_core::from_raw_borrowed(&psecurityinfo)).into())
        }
        IGPMWMIFilter_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Path: Path::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            GetQueryList: GetQueryList::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGPMWMIFilterCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IGPMWMIFilterCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGPMWMIFilterCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGPMWMIFilterCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"Win32_UI_Controls\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
pub trait IGroupPolicyObject_Impl: ::windows_core::BaseImpl {
    fn New(this: &Self::This, pszdomainname: &::windows_core::PCWSTR, pszdisplayname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn OpenDSGPO(this: &Self::This, pszpath: &::windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::Result<()>;
    fn OpenLocalMachineGPO(this: &Self::This, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::Result<()>;
    fn OpenRemoteMachineGPO(this: &Self::This, pszcomputername: &::windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::Result<()>;
    fn GetDisplayName(this: &Self::This, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::Result<()>;
    fn SetDisplayName(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPath(this: &Self::This, pszpath: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::Result<()>;
    fn GetDSPath(this: &Self::This, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::Result<()>;
    fn GetFileSysPath(this: &Self::This, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::Result<()>;
    fn GetRegistryKey(this: &Self::This, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> ::windows_core::Result<()>;
    fn GetOptions(this: &Self::This, dwoptions: *mut GPO_OPTIONS) -> ::windows_core::Result<()>;
    fn SetOptions(this: &Self::This, dwoptions: GPO_OPTIONS, dwmask: u32) -> ::windows_core::Result<()>;
    fn GetType(this: &Self::This, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::Result<()>;
    fn GetMachineName(this: &Self::This, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::Result<()>;
    fn GetPropertySheetPages(this: &Self::This, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl ::windows_core::Iids for IGroupPolicyObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGroupPolicyObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn New<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdomainname: ::windows_core::PCWSTR, pszdisplayname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::New(this, ::core::mem::transmute(&pszdomainname), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn OpenDSGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenDSGPO(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenLocalMachineGPO(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcomputername: ::windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenRemoteMachineGPO(this, ::core::mem::transmute(&pszcomputername), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::core::mem::transmute_copy(&bmachine), ::core::mem::transmute_copy(&badd), ::core::mem::transmute_copy(&pguidextension), ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into())
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayName(this, ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into())
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&pszname)).into())
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPath(this, ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxlength)).into())
        }
        unsafe extern "system" fn GetDSPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDSPath(this, ::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into())
        }
        unsafe extern "system" fn GetFileSysPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileSysPath(this, ::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&cchmaxpath)).into())
        }
        unsafe extern "system" fn GetRegistryKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegistryKey(this, ::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&hkey)).into())
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoptions: *mut GPO_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptions(this, ::core::mem::transmute_copy(&dwoptions)).into())
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoptions: GPO_OPTIONS, dwmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOptions(this, ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute_copy(&dwmask)).into())
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this, ::core::mem::transmute_copy(&gpotype)).into())
        }
        unsafe extern "system" fn GetMachineName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMachineName(this, ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into())
        }
        unsafe extern "system" fn GetPropertySheetPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertySheetPages(this, ::core::mem::transmute_copy(&hpages), ::core::mem::transmute_copy(&upagecount)).into())
        }
        IGroupPolicyObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            New: New::<Identity, Impl, OFFSET>,
            OpenDSGPO: OpenDSGPO::<Identity, Impl, OFFSET>,
            OpenLocalMachineGPO: OpenLocalMachineGPO::<Identity, Impl, OFFSET>,
            OpenRemoteMachineGPO: OpenRemoteMachineGPO::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetDSPath: GetDSPath::<Identity, Impl, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, Impl, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetMachineName: GetMachineName::<Identity, Impl, OFFSET>,
            GetPropertySheetPages: GetPropertySheetPages::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRSOPInformation_Impl: ::windows_core::BaseImpl {
    fn GetNamespace(this: &Self::This, dwsection: u32, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This, pdwflags: *mut u32) -> ::windows_core::Result<()>;
    fn GetEventLogEntryText(this: &Self::This, pszeventsource: &::windows_core::PCWSTR, pszeventlogname: &::windows_core::PCWSTR, pszeventtime: &::windows_core::PCWSTR, dweventid: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IRSOPInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRSOPInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNamespace(this, ::core::mem::transmute_copy(&dwsection), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&cchmaxlength)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlags(this, ::core::mem::transmute_copy(&pdwflags)).into())
        }
        unsafe extern "system" fn GetEventLogEntryText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszeventsource: ::windows_core::PCWSTR, pszeventlogname: ::windows_core::PCWSTR, pszeventtime: ::windows_core::PCWSTR, dweventid: u32, ppsztext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventLogEntryText(this, ::core::mem::transmute(&pszeventsource), ::core::mem::transmute(&pszeventlogname), ::core::mem::transmute(&pszeventtime), ::core::mem::transmute_copy(&dweventid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRSOPInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNamespace: GetNamespace::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Identity, Impl, OFFSET>,
        }
    };
}
