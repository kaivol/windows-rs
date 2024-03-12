pub trait IEnumOfflineFilesItems_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IOfflineFilesItem>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumOfflineFilesItems>;
}
impl ::windows_core::Iids for IEnumOfflineFilesItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumOfflineFilesItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumOfflineFilesItems_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumOfflineFilesSettings_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IOfflineFilesSetting>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumOfflineFilesSettings>;
}
impl ::windows_core::Iids for IEnumOfflineFilesSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumOfflineFilesSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumOfflineFilesSettings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesCache_Impl: ::windows_core::BaseImpl {
    fn Synchronize(this: &Self::This, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::core::option::Option<&IOfflineFilesSyncConflictHandler>, piprogress: ::core::option::Option<&IOfflineFilesSyncProgress>, psyncid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DeleteItems(this: &Self::This, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::core::option::Option<&IOfflineFilesSimpleProgress>) -> ::windows_core::Result<()>;
    fn DeleteItemsForUser(this: &Self::This, pszuser: &::windows_core::PCWSTR, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::core::option::Option<&IOfflineFilesSimpleProgress>) -> ::windows_core::Result<()>;
    fn Pin(this: &Self::This, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::core::option::Option<&IOfflineFilesSyncProgress>) -> ::windows_core::Result<()>;
    fn Unpin(this: &Self::This, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::core::option::Option<&IOfflineFilesSyncProgress>) -> ::windows_core::Result<()>;
    fn GetEncryptionStatus(this: &Self::This, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Encrypt(this: &Self::This, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::core::option::Option<&IOfflineFilesSyncProgress>) -> ::windows_core::Result<()>;
    fn FindItem(this: &Self::This, pszpath: &::windows_core::PCWSTR, dwqueryflags: u32) -> ::windows_core::Result<IOfflineFilesItem>;
    fn FindItemEx(this: &Self::This, pszpath: &::windows_core::PCWSTR, pincludefilefilter: ::core::option::Option<&IOfflineFilesItemFilter>, pincludedirfilter: ::core::option::Option<&IOfflineFilesItemFilter>, pexcludefilefilter: ::core::option::Option<&IOfflineFilesItemFilter>, pexcludedirfilter: ::core::option::Option<&IOfflineFilesItemFilter>, dwqueryflags: u32) -> ::windows_core::Result<IOfflineFilesItem>;
    fn RenameItem(this: &Self::This, pszpathoriginal: &::windows_core::PCWSTR, pszpathnew: &::windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLocation(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDiskSpaceInformation(this: &Self::This, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows_core::Result<()>;
    fn SetDiskSpaceLimits(this: &Self::This, cblimit: u64, cbunpinnedlimit: u64) -> ::windows_core::Result<()>;
    fn ProcessAdminPinPolicy(this: &Self::This, ppinprogress: ::core::option::Option<&IOfflineFilesSyncProgress>, punpinprogress: ::core::option::Option<&IOfflineFilesSyncProgress>) -> ::windows_core::Result<()>;
    fn GetSettingObject(this: &Self::This, pszsettingname: &::windows_core::PCWSTR) -> ::windows_core::Result<IOfflineFilesSetting>;
    fn EnumSettingObjects(this: &Self::This) -> ::windows_core::Result<IEnumOfflineFilesSettings>;
    fn IsPathCacheable(this: &Self::This, pszpath: &::windows_core::PCWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesCache {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesCache {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Synchronize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: *mut ::core::ffi::c_void, piprogress: *mut ::core::ffi::c_void, psyncid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Synchronize(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwsynccontrol), ::windows_core::from_raw_borrowed(&pisyncconflicthandler), ::windows_core::from_raw_borrowed(&piprogress), ::core::mem::transmute_copy(&psyncid)).into())
        }
        unsafe extern "system" fn DeleteItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItems(this, ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&basync), ::windows_core::from_raw_borrowed(&piprogress)).into())
        }
        unsafe extern "system" fn DeleteItemsForUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszuser: ::windows_core::PCWSTR, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItemsForUser(this, ::core::mem::transmute(&pszuser), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&basync), ::windows_core::from_raw_borrowed(&piprogress)).into())
        }
        unsafe extern "system" fn Pin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pin(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&bdeep), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwpincontrolflags), ::windows_core::from_raw_borrowed(&piprogress)).into())
        }
        unsafe extern "system" fn Unpin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unpin(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&bdeep), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwpincontrolflags), ::windows_core::from_raw_borrowed(&piprogress)).into())
        }
        unsafe extern "system" fn GetEncryptionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEncryptionStatus(this, ::core::mem::transmute_copy(&pbencrypted), ::core::mem::transmute_copy(&pbpartial)).into())
        }
        unsafe extern "system" fn Encrypt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Encrypt(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bencrypt), ::core::mem::transmute_copy(&dwencryptioncontrolflags), ::core::mem::transmute_copy(&basync), ::windows_core::from_raw_borrowed(&piprogress)).into())
        }
        unsafe extern "system" fn FindItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, dwqueryflags: u32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItem(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindItemEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pincludefilefilter: *mut ::core::ffi::c_void, pincludedirfilter: *mut ::core::ffi::c_void, pexcludefilefilter: *mut ::core::ffi::c_void, pexcludedirfilter: *mut ::core::ffi::c_void, dwqueryflags: u32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItemEx(this, ::core::mem::transmute(&pszpath), ::windows_core::from_raw_borrowed(&pincludefilefilter), ::windows_core::from_raw_borrowed(&pincludedirfilter), ::windows_core::from_raw_borrowed(&pexcludefilefilter), ::windows_core::from_raw_borrowed(&pexcludedirfilter), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RenameItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpathoriginal: ::windows_core::PCWSTR, pszpathnew: ::windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameItem(this, ::core::mem::transmute(&pszpathoriginal), ::core::mem::transmute(&pszpathnew), ::core::mem::transmute_copy(&breplaceifexists)).into())
        }
        unsafe extern "system" fn GetLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDiskSpaceInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDiskSpaceInformation(this, ::core::mem::transmute_copy(&pcbvolumetotal), ::core::mem::transmute_copy(&pcblimit), ::core::mem::transmute_copy(&pcbused), ::core::mem::transmute_copy(&pcbunpinnedlimit), ::core::mem::transmute_copy(&pcbunpinnedused)).into())
        }
        unsafe extern "system" fn SetDiskSpaceLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiskSpaceLimits(this, ::core::mem::transmute_copy(&cblimit), ::core::mem::transmute_copy(&cbunpinnedlimit)).into())
        }
        unsafe extern "system" fn ProcessAdminPinPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinprogress: *mut ::core::ffi::c_void, punpinprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessAdminPinPolicy(this, ::windows_core::from_raw_borrowed(&ppinprogress), ::windows_core::from_raw_borrowed(&punpinprogress)).into())
        }
        unsafe extern "system" fn GetSettingObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsettingname: ::windows_core::PCWSTR, ppsetting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSettingObject(this, ::core::mem::transmute(&pszsettingname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsetting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumSettingObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumSettingObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPathCacheable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPathCacheable(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&pbcacheable), ::core::mem::transmute_copy(&psharecachingmode)).into())
        }
        IOfflineFilesCache_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Synchronize: Synchronize::<Identity, Impl, OFFSET>,
            DeleteItems: DeleteItems::<Identity, Impl, OFFSET>,
            DeleteItemsForUser: DeleteItemsForUser::<Identity, Impl, OFFSET>,
            Pin: Pin::<Identity, Impl, OFFSET>,
            Unpin: Unpin::<Identity, Impl, OFFSET>,
            GetEncryptionStatus: GetEncryptionStatus::<Identity, Impl, OFFSET>,
            Encrypt: Encrypt::<Identity, Impl, OFFSET>,
            FindItem: FindItem::<Identity, Impl, OFFSET>,
            FindItemEx: FindItemEx::<Identity, Impl, OFFSET>,
            RenameItem: RenameItem::<Identity, Impl, OFFSET>,
            GetLocation: GetLocation::<Identity, Impl, OFFSET>,
            GetDiskSpaceInformation: GetDiskSpaceInformation::<Identity, Impl, OFFSET>,
            SetDiskSpaceLimits: SetDiskSpaceLimits::<Identity, Impl, OFFSET>,
            ProcessAdminPinPolicy: ProcessAdminPinPolicy::<Identity, Impl, OFFSET>,
            GetSettingObject: GetSettingObject::<Identity, Impl, OFFSET>,
            EnumSettingObjects: EnumSettingObjects::<Identity, Impl, OFFSET>,
            IsPathCacheable: IsPathCacheable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesCache2_Impl: ::windows_core::BaseImpl + IOfflineFilesCache_Impl {
    fn RenameItemEx(this: &Self::This, pszpathoriginal: &::windows_core::PCWSTR, pszpathnew: &::windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesCache2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesCache);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesCache2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RenameItemEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesCache2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpathoriginal: ::windows_core::PCWSTR, pszpathnew: ::windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameItemEx(this, ::core::mem::transmute(&pszpathoriginal), ::core::mem::transmute(&pszpathnew), ::core::mem::transmute_copy(&breplaceifexists)).into())
        }
        IOfflineFilesCache2_Vtbl { base__: <IOfflineFilesCache as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RenameItemEx: RenameItemEx::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesChangeInfo_Impl: ::windows_core::BaseImpl {
    fn IsDirty(this: &Self::This, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
    fn IsDeletedOffline(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsCreatedOffline(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedData(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedAttributes(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesChangeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesChangeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this, ::core::mem::transmute_copy(&pbdirty)))
        }
        unsafe extern "system" fn IsDeletedOffline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDeletedOffline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdeletedoffline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCreatedOffline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCreatedOffline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcreatedoffline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLocallyModifiedData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLocallyModifiedData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblocallymodifieddata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLocallyModifiedAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLocallyModifiedAttributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblocallymodifiedattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLocallyModifiedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLocallyModifiedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblocallymodifiedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesChangeInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            IsDeletedOffline: IsDeletedOffline::<Identity, Impl, OFFSET>,
            IsCreatedOffline: IsCreatedOffline::<Identity, Impl, OFFSET>,
            IsLocallyModifiedData: IsLocallyModifiedData::<Identity, Impl, OFFSET>,
            IsLocallyModifiedAttributes: IsLocallyModifiedAttributes::<Identity, Impl, OFFSET>,
            IsLocallyModifiedTime: IsLocallyModifiedTime::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesConnectionInfo_Impl: ::windows_core::BaseImpl {
    fn GetConnectState(this: &Self::This, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows_core::Result<()>;
    fn SetConnectState(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows_core::Result<()>;
    fn TransitionOnline(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::Result<()>;
    fn TransitionOffline(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesConnectionInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesConnectionInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnectState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectState(this, ::core::mem::transmute_copy(&pconnectstate), ::core::mem::transmute_copy(&pofflinereason)).into())
        }
        unsafe extern "system" fn SetConnectState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectState(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&connectstate)).into())
        }
        unsafe extern "system" fn TransitionOnline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransitionOnline(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn TransitionOffline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransitionOffline(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&bforceopenfilesclosed)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbopenfilespreventedtransition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesConnectionInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnectState: GetConnectState::<Identity, Impl, OFFSET>,
            SetConnectState: SetConnectState::<Identity, Impl, OFFSET>,
            TransitionOnline: TransitionOnline::<Identity, Impl, OFFSET>,
            TransitionOffline: TransitionOffline::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesDirectoryItem_Impl: ::windows_core::BaseImpl + IOfflineFilesItem_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesDirectoryItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesItem);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesDirectoryItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesDirectoryItem {
    const VTABLE: Self::Vtable = { IOfflineFilesDirectoryItem_Vtbl { base__: <IOfflineFilesItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IOfflineFilesDirtyInfo_Impl: ::windows_core::BaseImpl {
    fn LocalDirtyByteCount(this: &Self::This) -> ::windows_core::Result<i64>;
    fn RemoteDirtyByteCount(this: &Self::This) -> ::windows_core::Result<i64>;
}
impl ::windows_core::Iids for IOfflineFilesDirtyInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesDirtyInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LocalDirtyByteCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalDirtyByteCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdirtybytecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoteDirtyByteCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteDirtyByteCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdirtybytecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesDirtyInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LocalDirtyByteCount: LocalDirtyByteCount::<Identity, Impl, OFFSET>,
            RemoteDirtyByteCount: RemoteDirtyByteCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOfflineFilesErrorInfo_Impl: ::windows_core::BaseImpl {
    fn GetRawData(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::BYTE_BLOB>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOfflineFilesErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRawData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRawData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppblob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesErrorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRawData: GetRawData::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents_Impl: ::windows_core::BaseImpl {
    fn CacheMoved(this: &Self::This, pszoldpath: &::windows_core::PCWSTR, psznewpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CacheIsFull(this: &Self::This) -> ::windows_core::Result<()>;
    fn CacheIsCorrupted(this: &Self::This) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This, benabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EncryptionChanged(this: &Self::This, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SyncBegin(this: &Self::This, rsyncid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SyncFileResult(this: &Self::This, rsyncid: *const ::windows_core::GUID, pszfile: &::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn SyncConflictRecAdded(this: &Self::This, pszconflictpath: &::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>;
    fn SyncConflictRecUpdated(this: &Self::This, pszconflictpath: &::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>;
    fn SyncConflictRecRemoved(this: &Self::This, pszconflictpath: &::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>;
    fn SyncEnd(this: &Self::This, rsyncid: *const ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn NetTransportArrived(this: &Self::This) -> ::windows_core::Result<()>;
    fn NoNetTransports(this: &Self::This) -> ::windows_core::Result<()>;
    fn ItemDisconnected(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemReconnected(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemAvailableOffline(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemNotAvailableOffline(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemPinned(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemNotPinned(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemModified(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ItemAddedToCache(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemDeletedFromCache(this: &Self::This, pszpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn ItemRenamed(this: &Self::This, pszoldpath: &::windows_core::PCWSTR, psznewpath: &::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>;
    fn DataLost(this: &Self::This) -> ::windows_core::Result<()>;
    fn Ping(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CacheMoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszoldpath: ::windows_core::PCWSTR, psznewpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CacheMoved(this, ::core::mem::transmute(&pszoldpath), ::core::mem::transmute(&psznewpath)).into())
        }
        unsafe extern "system" fn CacheIsFull<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CacheIsFull(this).into())
        }
        unsafe extern "system" fn CacheIsCorrupted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CacheIsCorrupted(this).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enabled(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn EncryptionChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncryptionChanged(this, ::core::mem::transmute_copy(&bwasencrypted), ::core::mem::transmute_copy(&bwaspartial), ::core::mem::transmute_copy(&bisencrypted), ::core::mem::transmute_copy(&bispartial)).into())
        }
        unsafe extern "system" fn SyncBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncBegin(this, ::core::mem::transmute_copy(&rsyncid)).into())
        }
        unsafe extern "system" fn SyncFileResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows_core::GUID, pszfile: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncFileResult(this, ::core::mem::transmute_copy(&rsyncid), ::core::mem::transmute(&pszfile), ::core::mem::transmute_copy(&hrresult)).into())
        }
        unsafe extern "system" fn SyncConflictRecAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszconflictpath: ::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncConflictRecAdded(this, ::core::mem::transmute(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into())
        }
        unsafe extern "system" fn SyncConflictRecUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszconflictpath: ::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncConflictRecUpdated(this, ::core::mem::transmute(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into())
        }
        unsafe extern "system" fn SyncConflictRecRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszconflictpath: ::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncConflictRecRemoved(this, ::core::mem::transmute(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into())
        }
        unsafe extern "system" fn SyncEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncEnd(this, ::core::mem::transmute_copy(&rsyncid), ::core::mem::transmute_copy(&hrresult)).into())
        }
        unsafe extern "system" fn NetTransportArrived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NetTransportArrived(this).into())
        }
        unsafe extern "system" fn NoNetTransports<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NoNetTransports(this).into())
        }
        unsafe extern "system" fn ItemDisconnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemDisconnected(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemReconnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemReconnected(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemAvailableOffline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemAvailableOffline(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemNotAvailableOffline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemNotAvailableOffline(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemPinned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemPinned(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemNotPinned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemNotPinned(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemModified(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype), ::core::mem::transmute_copy(&bmodifieddata), ::core::mem::transmute_copy(&bmodifiedattributes)).into())
        }
        unsafe extern "system" fn ItemAddedToCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemAddedToCache(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemDeletedFromCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemDeletedFromCache(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn ItemRenamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszoldpath: ::windows_core::PCWSTR, psznewpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemRenamed(this, ::core::mem::transmute(&pszoldpath), ::core::mem::transmute(&psznewpath), ::core::mem::transmute_copy(&itemtype)).into())
        }
        unsafe extern "system" fn DataLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DataLost(this).into())
        }
        unsafe extern "system" fn Ping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Ping(this).into())
        }
        IOfflineFilesEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CacheMoved: CacheMoved::<Identity, Impl, OFFSET>,
            CacheIsFull: CacheIsFull::<Identity, Impl, OFFSET>,
            CacheIsCorrupted: CacheIsCorrupted::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            EncryptionChanged: EncryptionChanged::<Identity, Impl, OFFSET>,
            SyncBegin: SyncBegin::<Identity, Impl, OFFSET>,
            SyncFileResult: SyncFileResult::<Identity, Impl, OFFSET>,
            SyncConflictRecAdded: SyncConflictRecAdded::<Identity, Impl, OFFSET>,
            SyncConflictRecUpdated: SyncConflictRecUpdated::<Identity, Impl, OFFSET>,
            SyncConflictRecRemoved: SyncConflictRecRemoved::<Identity, Impl, OFFSET>,
            SyncEnd: SyncEnd::<Identity, Impl, OFFSET>,
            NetTransportArrived: NetTransportArrived::<Identity, Impl, OFFSET>,
            NoNetTransports: NoNetTransports::<Identity, Impl, OFFSET>,
            ItemDisconnected: ItemDisconnected::<Identity, Impl, OFFSET>,
            ItemReconnected: ItemReconnected::<Identity, Impl, OFFSET>,
            ItemAvailableOffline: ItemAvailableOffline::<Identity, Impl, OFFSET>,
            ItemNotAvailableOffline: ItemNotAvailableOffline::<Identity, Impl, OFFSET>,
            ItemPinned: ItemPinned::<Identity, Impl, OFFSET>,
            ItemNotPinned: ItemNotPinned::<Identity, Impl, OFFSET>,
            ItemModified: ItemModified::<Identity, Impl, OFFSET>,
            ItemAddedToCache: ItemAddedToCache::<Identity, Impl, OFFSET>,
            ItemDeletedFromCache: ItemDeletedFromCache::<Identity, Impl, OFFSET>,
            ItemRenamed: ItemRenamed::<Identity, Impl, OFFSET>,
            DataLost: DataLost::<Identity, Impl, OFFSET>,
            Ping: Ping::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents2_Impl: ::windows_core::BaseImpl + IOfflineFilesEvents_Impl {
    fn ItemReconnectBegin(this: &Self::This) -> ::windows_core::Result<()>;
    fn ItemReconnectEnd(this: &Self::This) -> ::windows_core::Result<()>;
    fn CacheEvictBegin(this: &Self::This) -> ::windows_core::Result<()>;
    fn CacheEvictEnd(this: &Self::This) -> ::windows_core::Result<()>;
    fn BackgroundSyncBegin(this: &Self::This, dwsynccontrolflags: u32) -> ::windows_core::Result<()>;
    fn BackgroundSyncEnd(this: &Self::This, dwsynccontrolflags: u32) -> ::windows_core::Result<()>;
    fn PolicyChangeDetected(this: &Self::This) -> ::windows_core::Result<()>;
    fn PreferenceChangeDetected(this: &Self::This) -> ::windows_core::Result<()>;
    fn SettingsChangesApplied(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesEvents2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesEvents);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesEvents2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ItemReconnectBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemReconnectBegin(this).into())
        }
        unsafe extern "system" fn ItemReconnectEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ItemReconnectEnd(this).into())
        }
        unsafe extern "system" fn CacheEvictBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CacheEvictBegin(this).into())
        }
        unsafe extern "system" fn CacheEvictEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CacheEvictEnd(this).into())
        }
        unsafe extern "system" fn BackgroundSyncBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackgroundSyncBegin(this, ::core::mem::transmute_copy(&dwsynccontrolflags)).into())
        }
        unsafe extern "system" fn BackgroundSyncEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackgroundSyncEnd(this, ::core::mem::transmute_copy(&dwsynccontrolflags)).into())
        }
        unsafe extern "system" fn PolicyChangeDetected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PolicyChangeDetected(this).into())
        }
        unsafe extern "system" fn PreferenceChangeDetected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreferenceChangeDetected(this).into())
        }
        unsafe extern "system" fn SettingsChangesApplied<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettingsChangesApplied(this).into())
        }
        IOfflineFilesEvents2_Vtbl {
            base__: <IOfflineFilesEvents as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ItemReconnectBegin: ItemReconnectBegin::<Identity, Impl, OFFSET>,
            ItemReconnectEnd: ItemReconnectEnd::<Identity, Impl, OFFSET>,
            CacheEvictBegin: CacheEvictBegin::<Identity, Impl, OFFSET>,
            CacheEvictEnd: CacheEvictEnd::<Identity, Impl, OFFSET>,
            BackgroundSyncBegin: BackgroundSyncBegin::<Identity, Impl, OFFSET>,
            BackgroundSyncEnd: BackgroundSyncEnd::<Identity, Impl, OFFSET>,
            PolicyChangeDetected: PolicyChangeDetected::<Identity, Impl, OFFSET>,
            PreferenceChangeDetected: PreferenceChangeDetected::<Identity, Impl, OFFSET>,
            SettingsChangesApplied: SettingsChangesApplied::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents3_Impl: ::windows_core::BaseImpl + IOfflineFilesEvents2_Impl {
    fn TransparentCacheItemNotify(this: &Self::This, pszpath: &::windows_core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn PrefetchFileBegin(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn PrefetchFileEnd(this: &Self::This, pszpath: &::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesEvents3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesEvents2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesEvents3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransparentCacheItemNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransparentCacheItemNotify(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&eventtype), ::core::mem::transmute_copy(&itemtype), ::core::mem::transmute_copy(&bmodifieddata), ::core::mem::transmute_copy(&bmodifiedattributes), ::core::mem::transmute(&pzsoldpath)).into())
        }
        unsafe extern "system" fn PrefetchFileBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrefetchFileBegin(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn PrefetchFileEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrefetchFileEnd(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&hrresult)).into())
        }
        IOfflineFilesEvents3_Vtbl {
            base__: <IOfflineFilesEvents2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransparentCacheItemNotify: TransparentCacheItemNotify::<Identity, Impl, OFFSET>,
            PrefetchFileBegin: PrefetchFileBegin::<Identity, Impl, OFFSET>,
            PrefetchFileEnd: PrefetchFileEnd::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents4_Impl: ::windows_core::BaseImpl + IOfflineFilesEvents3_Impl {
    fn PrefetchCloseHandleBegin(this: &Self::This) -> ::windows_core::Result<()>;
    fn PrefetchCloseHandleEnd(this: &Self::This, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesEvents4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesEvents3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesEvents4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrefetchCloseHandleBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrefetchCloseHandleBegin(this).into())
        }
        unsafe extern "system" fn PrefetchCloseHandleEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEvents4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrefetchCloseHandleEnd(this, ::core::mem::transmute_copy(&dwclosedhandlecount), ::core::mem::transmute_copy(&dwopenhandlecount), ::core::mem::transmute_copy(&hrresult)).into())
        }
        IOfflineFilesEvents4_Vtbl {
            base__: <IOfflineFilesEvents3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrefetchCloseHandleBegin: PrefetchCloseHandleBegin::<Identity, Impl, OFFSET>,
            PrefetchCloseHandleEnd: PrefetchCloseHandleEnd::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOfflineFilesEventsFilter_Impl: ::windows_core::BaseImpl {
    fn GetPathFilter(this: &Self::This, ppszfilter: *mut ::windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows_core::Result<()>;
    fn GetIncludedEvents(this: &Self::This, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_core::Result<()>;
    fn GetExcludedEvents(this: &Self::This, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IOfflineFilesEventsFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesEventsFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPathFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszfilter: *mut ::windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPathFilter(this, ::core::mem::transmute_copy(&ppszfilter), ::core::mem::transmute_copy(&pmatch)).into())
        }
        unsafe extern "system" fn GetIncludedEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIncludedEvents(this, ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)).into())
        }
        unsafe extern "system" fn GetExcludedEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExcludedEvents(this, ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)).into())
        }
        IOfflineFilesEventsFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPathFilter: GetPathFilter::<Identity, Impl, OFFSET>,
            GetIncludedEvents: GetIncludedEvents::<Identity, Impl, OFFSET>,
            GetExcludedEvents: GetExcludedEvents::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileItem_Impl: ::windows_core::BaseImpl + IOfflineFilesItem_Impl {
    fn IsSparse(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEncrypted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesFileItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesItem);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesFileItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesFileItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsSparse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesFileItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSparse(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbissparse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEncrypted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesFileItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEncrypted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisencrypted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesFileItem_Vtbl {
            base__: <IOfflineFilesItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsSparse: IsSparse::<Identity, Impl, OFFSET>,
            IsEncrypted: IsEncrypted::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileSysInfo_Impl: ::windows_core::BaseImpl {
    fn GetAttributes(this: &Self::This, copy: OFFLINEFILES_ITEM_COPY) -> ::windows_core::Result<u32>;
    fn GetTimes(this: &Self::This, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn GetFileSize(this: &Self::This, copy: OFFLINEFILES_ITEM_COPY) -> ::windows_core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesFileSysInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesFileSysInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributes(this, ::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTimes(this, ::core::mem::transmute_copy(&copy), ::core::mem::transmute_copy(&pftcreationtime), ::core::mem::transmute_copy(&pftlastwritetime), ::core::mem::transmute_copy(&pftchangetime), ::core::mem::transmute_copy(&pftlastaccesstime)).into())
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSize(this, ::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesFileSysInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            GetTimes: GetTimes::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesGhostInfo_Impl: ::windows_core::BaseImpl {
    fn IsGhosted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesGhostInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesGhostInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesGhostInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsGhosted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesGhostInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsGhosted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbghosted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesGhostInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsGhosted: IsGhosted::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItem_Impl: ::windows_core::BaseImpl {
    fn GetItemType(this: &Self::This) -> ::windows_core::Result<OFFLINEFILES_ITEM_TYPE>;
    fn GetPath(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetParentItem(this: &Self::This) -> ::windows_core::Result<IOfflineFilesItem>;
    fn Refresh(this: &Self::This, dwqueryflags: u32) -> ::windows_core::Result<()>;
    fn IsMarkedForDeletion(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this, ::core::mem::transmute_copy(&dwqueryflags)).into())
        }
        unsafe extern "system" fn IsMarkedForDeletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMarkedForDeletion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmarkedfordeletion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemType: GetItemType::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            IsMarkedForDeletion: IsMarkedForDeletion::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOfflineFilesItemContainer_Impl: ::windows_core::BaseImpl {
    fn EnumItems(this: &Self::This, dwqueryflags: u32) -> ::windows_core::Result<IEnumOfflineFilesItems>;
    fn EnumItemsEx(this: &Self::This, pincludefilefilter: ::core::option::Option<&IOfflineFilesItemFilter>, pincludedirfilter: ::core::option::Option<&IOfflineFilesItemFilter>, pexcludefilefilter: ::core::option::Option<&IOfflineFilesItemFilter>, pexcludedirfilter: ::core::option::Option<&IOfflineFilesItemFilter>, dwenumflags: u32, dwqueryflags: u32) -> ::windows_core::Result<IEnumOfflineFilesItems>;
}
impl ::windows_core::Iids for IOfflineFilesItemContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesItemContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumItems(this, ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumItemsEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pincludefilefilter: *mut ::core::ffi::c_void, pincludedirfilter: *mut ::core::ffi::c_void, pexcludefilefilter: *mut ::core::ffi::c_void, pexcludedirfilter: *mut ::core::ffi::c_void, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumItemsEx(this, ::windows_core::from_raw_borrowed(&pincludefilefilter), ::windows_core::from_raw_borrowed(&pincludedirfilter), ::windows_core::from_raw_borrowed(&pexcludefilefilter), ::windows_core::from_raw_borrowed(&pexcludedirfilter), ::core::mem::transmute_copy(&dwenumflags), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesItemContainer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumItems: EnumItems::<Identity, Impl, OFFSET>,
            EnumItemsEx: EnumItemsEx::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItemFilter_Impl: ::windows_core::BaseImpl {
    fn GetFilterFlags(this: &Self::This, pullflags: *mut u64, pullmask: *mut u64) -> ::windows_core::Result<()>;
    fn GetTimeFilter(this: &Self::This, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows_core::Result<()>;
    fn GetPatternFilter(this: &Self::This, pszpattern: ::windows_core::PWSTR, cchpattern: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesItemFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesItemFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilterFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilterFlags(this, ::core::mem::transmute_copy(&pullflags), ::core::mem::transmute_copy(&pullmask)).into())
        }
        unsafe extern "system" fn GetTimeFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTimeFilter(this, ::core::mem::transmute_copy(&pfttime), ::core::mem::transmute_copy(&pbevaltimeofday), ::core::mem::transmute_copy(&ptimetype), ::core::mem::transmute_copy(&pcompare)).into())
        }
        unsafe extern "system" fn GetPatternFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpattern: ::windows_core::PWSTR, cchpattern: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatternFilter(this, ::core::mem::transmute_copy(&pszpattern), ::core::mem::transmute_copy(&cchpattern)).into())
        }
        IOfflineFilesItemFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFilterFlags: GetFilterFlags::<Identity, Impl, OFFSET>,
            GetTimeFilter: GetTimeFilter::<Identity, Impl, OFFSET>,
            GetPatternFilter: GetPatternFilter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfo_Impl: ::windows_core::BaseImpl {
    fn IsPinned(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsPinnedForUser(this: &Self::This, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsPinnedForUserByPolicy(this: &Self::This, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsPinnedForComputer(this: &Self::This, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsPinnedForFolderRedirection(this: &Self::This, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesPinInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesPinInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsPinned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPinned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpinned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPinnedForUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPinnedForUser(this, ::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)).into())
        }
        unsafe extern "system" fn IsPinnedForUserByPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPinnedForUserByPolicy(this, ::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)).into())
        }
        unsafe extern "system" fn IsPinnedForComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPinnedForComputer(this, ::core::mem::transmute_copy(&pbpinnedforcomputer), ::core::mem::transmute_copy(&pbinherit)).into())
        }
        unsafe extern "system" fn IsPinnedForFolderRedirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPinnedForFolderRedirection(this, ::core::mem::transmute_copy(&pbpinnedforfolderredirection), ::core::mem::transmute_copy(&pbinherit)).into())
        }
        IOfflineFilesPinInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsPinned: IsPinned::<Identity, Impl, OFFSET>,
            IsPinnedForUser: IsPinnedForUser::<Identity, Impl, OFFSET>,
            IsPinnedForUserByPolicy: IsPinnedForUserByPolicy::<Identity, Impl, OFFSET>,
            IsPinnedForComputer: IsPinnedForComputer::<Identity, Impl, OFFSET>,
            IsPinnedForFolderRedirection: IsPinnedForFolderRedirection::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfo2_Impl: ::windows_core::BaseImpl + IOfflineFilesPinInfo_Impl {
    fn IsPartlyPinned(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesPinInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesPinInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesPinInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsPartlyPinned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesPinInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPartlyPinned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpartlypinned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesPinInfo2_Vtbl { base__: <IOfflineFilesPinInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsPartlyPinned: IsPartlyPinned::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesProgress_Impl: ::windows_core::BaseImpl {
    fn Begin(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn QueryAbort(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn End(this: &Self::This, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Begin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbabort, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryAbort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbabort, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this, ::core::mem::transmute_copy(&hrresult)).into())
        }
        IOfflineFilesProgress_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin: Begin::<Identity, Impl, OFFSET>,
            QueryAbort: QueryAbort::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesServerItem_Impl: ::windows_core::BaseImpl + IOfflineFilesItem_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesServerItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesItem);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesServerItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesServerItem {
    const VTABLE: Self::Vtable = { IOfflineFilesServerItem_Vtbl { base__: <IOfflineFilesItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOfflineFilesSetting_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetValueType(this: &Self::This) -> ::windows_core::Result<OFFLINEFILES_SETTING_VALUE_TYPE>;
    fn GetPreference(this: &Self::This, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::Result<()>;
    fn GetPreferenceScope(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetPreference(this: &Self::This, pvarvalue: *const super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::Result<()>;
    fn DeletePreference(this: &Self::This, dwscope: u32) -> ::windows_core::Result<()>;
    fn GetPolicy(this: &Self::This, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::Result<()>;
    fn GetPolicyScope(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetValue(this: &Self::This, pvarvalue: *mut super::super::System::Variant::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IOfflineFilesSetting {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSetting {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValueType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValueType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPreference(this, ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into())
        }
        unsafe extern "system" fn GetPreferenceScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreferenceScope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreference(this, ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into())
        }
        unsafe extern "system" fn DeletePreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwscope: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePreference(this, ::core::mem::transmute_copy(&dwscope)).into())
        }
        unsafe extern "system" fn GetPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPolicy(this, ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into())
        }
        unsafe extern "system" fn GetPolicyScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPolicyScope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValue(this, ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pbsetbypolicy)).into())
        }
        IOfflineFilesSetting_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetValueType: GetValueType::<Identity, Impl, OFFSET>,
            GetPreference: GetPreference::<Identity, Impl, OFFSET>,
            GetPreferenceScope: GetPreferenceScope::<Identity, Impl, OFFSET>,
            SetPreference: SetPreference::<Identity, Impl, OFFSET>,
            DeletePreference: DeletePreference::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            GetPolicyScope: GetPolicyScope::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesShareInfo_Impl: ::windows_core::BaseImpl {
    fn GetShareItem(this: &Self::This) -> ::windows_core::Result<IOfflineFilesShareItem>;
    fn GetShareCachingMode(this: &Self::This) -> ::windows_core::Result<OFFLINEFILES_CACHING_MODE>;
    fn IsShareDfsJunction(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesShareInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesShareInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetShareItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppshareitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetShareItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshareitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetShareCachingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetShareCachingMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcachingmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsShareDfsJunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsShareDfsJunction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisdfsjunction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesShareInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetShareItem: GetShareItem::<Identity, Impl, OFFSET>,
            GetShareCachingMode: GetShareCachingMode::<Identity, Impl, OFFSET>,
            IsShareDfsJunction: IsShareDfsJunction::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesShareItem_Impl: ::windows_core::BaseImpl + IOfflineFilesItem_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesShareItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesItem);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesShareItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesShareItem {
    const VTABLE: Self::Vtable = { IOfflineFilesShareItem_Vtbl { base__: <IOfflineFilesItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSimpleProgress_Impl: ::windows_core::BaseImpl + IOfflineFilesProgress_Impl {
    fn ItemBegin(this: &Self::This, pszfile: &::windows_core::PCWSTR) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn ItemResult(this: &Self::This, pszfile: &::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesSimpleProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesProgress);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSimpleProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ItemBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemBegin(this, ::core::mem::transmute(&pszfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemResult(this, ::core::mem::transmute(&pszfile), ::core::mem::transmute_copy(&hrresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesSimpleProgress_Vtbl {
            base__: <IOfflineFilesProgress as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ItemBegin: ItemBegin::<Identity, Impl, OFFSET>,
            ItemResult: ItemResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspend_Impl: ::windows_core::BaseImpl {
    fn SuspendRoot(this: &Self::This, bsuspend: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesSuspend {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSuspend_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSuspend {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SuspendRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSuspend_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsuspend: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspendRoot(this, ::core::mem::transmute_copy(&bsuspend)).into())
        }
        IOfflineFilesSuspend_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SuspendRoot: SuspendRoot::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspendInfo_Impl: ::windows_core::BaseImpl {
    fn IsSuspended(this: &Self::This, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesSuspendInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSuspendInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSuspendInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsSuspended<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSuspendInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSuspended(this, ::core::mem::transmute_copy(&pbsuspended), ::core::mem::transmute_copy(&pbsuspendedroot)).into())
        }
        IOfflineFilesSuspendInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsSuspended: IsSuspended::<Identity, Impl, OFFSET> }
    };
}
pub trait IOfflineFilesSyncConflictHandler_Impl: ::windows_core::BaseImpl {
    fn ResolveConflict(this: &Self::This, pszpath: &::windows_core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IOfflineFilesSyncConflictHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSyncConflictHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResolveConflict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveConflict(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&fstateknown), ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&fchangedetails), ::core::mem::transmute_copy(&pconflictresolution), ::core::mem::transmute_copy(&ppsznewname)).into())
        }
        IOfflineFilesSyncConflictHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResolveConflict: ResolveConflict::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOfflineFilesSyncErrorInfo_Impl: ::windows_core::BaseImpl + IOfflineFilesErrorInfo_Impl {
    fn GetSyncOperation(this: &Self::This) -> ::windows_core::Result<OFFLINEFILES_SYNC_OPERATION>;
    fn GetItemChangeFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn InfoEnumerated(this: &Self::This, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn InfoAvailable(this: &Self::This, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLocalInfo(this: &Self::This) -> ::windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetRemoteInfo(this: &Self::This) -> ::windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetOriginalInfo(this: &Self::This) -> ::windows_core::Result<IOfflineFilesSyncErrorItemInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOfflineFilesSyncErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesErrorInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSyncErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psyncop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemChangeFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwitemchangeflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemChangeFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwitemchangeflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InfoEnumerated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InfoEnumerated(this, ::core::mem::transmute_copy(&pblocalenumerated), ::core::mem::transmute_copy(&pbremoteenumerated), ::core::mem::transmute_copy(&pboriginalenumerated)).into())
        }
        unsafe extern "system" fn InfoAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InfoAvailable(this, ::core::mem::transmute_copy(&pblocalinfo), ::core::mem::transmute_copy(&pbremoteinfo), ::core::mem::transmute_copy(&pboriginalinfo)).into())
        }
        unsafe extern "system" fn GetLocalInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRemoteInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRemoteInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOriginalInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOriginalInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesSyncErrorInfo_Vtbl {
            base__: <IOfflineFilesErrorInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSyncOperation: GetSyncOperation::<Identity, Impl, OFFSET>,
            GetItemChangeFlags: GetItemChangeFlags::<Identity, Impl, OFFSET>,
            InfoEnumerated: InfoEnumerated::<Identity, Impl, OFFSET>,
            InfoAvailable: InfoAvailable::<Identity, Impl, OFFSET>,
            GetLocalInfo: GetLocalInfo::<Identity, Impl, OFFSET>,
            GetRemoteInfo: GetRemoteInfo::<Identity, Impl, OFFSET>,
            GetOriginalInfo: GetOriginalInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncErrorItemInfo_Impl: ::windows_core::BaseImpl {
    fn GetFileAttributes(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFileTimes(this: &Self::This, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn GetFileSize(this: &Self::This) -> ::windows_core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesSyncErrorItemInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSyncErrorItemInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFileAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileAttributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileTimes(this, ::core::mem::transmute_copy(&pftlastwrite), ::core::mem::transmute_copy(&pftchange)).into())
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psize: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesSyncErrorItemInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFileAttributes: GetFileAttributes::<Identity, Impl, OFFSET>,
            GetFileTimes: GetFileTimes::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncProgress_Impl: ::windows_core::BaseImpl + IOfflineFilesProgress_Impl {
    fn SyncItemBegin(this: &Self::This, pszfile: &::windows_core::PCWSTR) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn SyncItemResult(this: &Self::This, pszfile: &::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT, perrorinfo: ::core::option::Option<&IOfflineFilesSyncErrorInfo>) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesSyncProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOfflineFilesProgress);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesSyncProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SyncItemBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SyncItemBegin(this, ::core::mem::transmute(&pszfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SyncItemResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT, perrorinfo: *mut ::core::ffi::c_void, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SyncItemResult(this, ::core::mem::transmute(&pszfile), ::core::mem::transmute_copy(&hrresult), ::windows_core::from_raw_borrowed(&perrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesSyncProgress_Vtbl {
            base__: <IOfflineFilesProgress as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SyncItemBegin: SyncItemBegin::<Identity, Impl, OFFSET>,
            SyncItemResult: SyncItemResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesTransparentCacheInfo_Impl: ::windows_core::BaseImpl {
    fn IsTransparentlyCached(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOfflineFilesTransparentCacheInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOfflineFilesTransparentCacheInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsTransparentlyCached<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTransparentlyCached(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbtransparentlycached, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOfflineFilesTransparentCacheInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsTransparentlyCached: IsTransparentlyCached::<Identity, Impl, OFFSET>,
        }
    };
}
