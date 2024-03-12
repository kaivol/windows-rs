#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IABContainer_Impl: ::windows_core::BaseImpl + IMAPIContainer_Impl {
    fn CreateEntry(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows_core::Result<IMAPIProp>;
    fn CopyEntries(this: &Self::This, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn DeleteEntries(this: &Self::This, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows_core::Result<()>;
    fn ResolveNames(this: &Self::This, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows_core::Result<FlagList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IABContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIContainer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IABContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IABContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IABContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEntry(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulcreateflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppmapipropentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IABContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyEntries(this, ::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn DeleteEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IABContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteEntries(this, ::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn ResolveNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IABContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut FlagList) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResolveNames(this, ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpadrlist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpflaglist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IABContainer_Vtbl {
            base__: <IMAPIContainer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEntry: CreateEntry::<Identity, Impl, OFFSET>,
            CopyEntries: CopyEntries::<Identity, Impl, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, Impl, OFFSET>,
            ResolveNames: ResolveNames::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAddrBook_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {
    fn OpenEntry(this: &Self::This, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CompareEntryIDs(this: &Self::This, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows_core::Result<()>;
    fn Advise(this: &Self::This, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: ::core::option::Option<&IMAPIAdviseSink>, lpulconnection: *mut u32) -> ::windows_core::Result<()>;
    fn Unadvise(this: &Self::This, ulconnection: u32) -> ::windows_core::Result<()>;
    fn CreateOneOff(this: &Self::This, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_core::Result<()>;
    fn NewEntry(this: &Self::This, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows_core::Result<()>;
    fn ResolveName(this: &Self::This, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows_core::Result<()>;
    fn Address(this: &Self::This, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows_core::Result<()>;
    fn Details(this: &Self::This, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows_core::Result<()>;
    fn RecipOptions(this: &Self::This, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows_core::Result<()>;
    fn QueryDefaultRecipOpt(this: &Self::This, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows_core::Result<()>;
    fn GetPAB(this: &Self::This, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_core::Result<()>;
    fn SetPAB(this: &Self::This, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows_core::Result<()>;
    fn GetDefaultDir(this: &Self::This, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_core::Result<()>;
    fn SetDefaultDir(this: &Self::This, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows_core::Result<()>;
    fn GetSearchPath(this: &Self::This, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows_core::Result<()>;
    fn SetSearchPath(this: &Self::This, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows_core::Result<()>;
    fn PrepareRecips(this: &Self::This, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAddrBook {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAddrBook {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenEntry(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&lppunk)).into())
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompareEntryIDs(this, ::core::mem::transmute_copy(&cbentryid1), ::core::mem::transmute_copy(&lpentryid1), ::core::mem::transmute_copy(&cbentryid2), ::core::mem::transmute_copy(&lpentryid2), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulresult)).into())
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Advise(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&uleventmask), ::windows_core::from_raw_borrowed(&lpadvisesink), ::core::mem::transmute_copy(&lpulconnection)).into())
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&ulconnection)).into())
        }
        unsafe extern "system" fn CreateOneOff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateOneOff(this, ::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&lpszadrtype), ::core::mem::transmute_copy(&lpszaddress), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid)).into())
        }
        unsafe extern "system" fn NewEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewEntry(this, ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbeidcontainer), ::core::mem::transmute_copy(&lpeidcontainer), ::core::mem::transmute_copy(&cbeidnewentrytpl), ::core::mem::transmute_copy(&lpeidnewentrytpl), ::core::mem::transmute_copy(&lpcbeidnewentry), ::core::mem::transmute_copy(&lppeidnewentry)).into())
        }
        unsafe extern "system" fn ResolveName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveName(this, ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpsznewentrytitle), ::core::mem::transmute_copy(&lpadrlist)).into())
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Address(this, ::core::mem::transmute_copy(&lpuluiparam), ::core::mem::transmute_copy(&lpadrparms), ::core::mem::transmute_copy(&lppadrlist)).into())
        }
        unsafe extern "system" fn Details<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpuluiparam: *mut usize, lpfndismiss: LPFNDISMISS, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: LPFNBUTTON, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Details(this, ::core::mem::transmute_copy(&lpuluiparam), ::core::mem::transmute_copy(&lpfndismiss), ::core::mem::transmute_copy(&lpvdismisscontext), ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpfbuttoncallback), ::core::mem::transmute_copy(&lpvbuttoncontext), ::core::mem::transmute_copy(&lpszbuttontext), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn RecipOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecipOptions(this, ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lprecip)).into())
        }
        unsafe extern "system" fn QueryDefaultRecipOpt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryDefaultRecipOpt(this, ::core::mem::transmute_copy(&lpszadrtype), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcvalues), ::core::mem::transmute_copy(&lppoptions)).into())
        }
        unsafe extern "system" fn GetPAB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPAB(this, ::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid)).into())
        }
        unsafe extern "system" fn SetPAB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPAB(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into())
        }
        unsafe extern "system" fn GetDefaultDir<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultDir(this, ::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid)).into())
        }
        unsafe extern "system" fn SetDefaultDir<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultDir(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into())
        }
        unsafe extern "system" fn GetSearchPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSearchPath(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppsearchpath)).into())
        }
        unsafe extern "system" fn SetSearchPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSearchPath(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpsearchpath)).into())
        }
        unsafe extern "system" fn PrepareRecips<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAddrBook_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareRecips(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&lpreciplist)).into())
        }
        IAddrBook_Vtbl {
            base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenEntry: OpenEntry::<Identity, Impl, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            CreateOneOff: CreateOneOff::<Identity, Impl, OFFSET>,
            NewEntry: NewEntry::<Identity, Impl, OFFSET>,
            ResolveName: ResolveName::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            Details: Details::<Identity, Impl, OFFSET>,
            RecipOptions: RecipOptions::<Identity, Impl, OFFSET>,
            QueryDefaultRecipOpt: QueryDefaultRecipOpt::<Identity, Impl, OFFSET>,
            GetPAB: GetPAB::<Identity, Impl, OFFSET>,
            SetPAB: SetPAB::<Identity, Impl, OFFSET>,
            GetDefaultDir: GetDefaultDir::<Identity, Impl, OFFSET>,
            SetDefaultDir: SetDefaultDir::<Identity, Impl, OFFSET>,
            GetSearchPath: GetSearchPath::<Identity, Impl, OFFSET>,
            SetSearchPath: SetSearchPath::<Identity, Impl, OFFSET>,
            PrepareRecips: PrepareRecips::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAttach_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAttach {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAttach_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAttach {
    const VTABLE: Self::Vtable = { IAttach_Vtbl { base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDistList_Impl: ::windows_core::BaseImpl + IMAPIContainer_Impl {
    fn CreateEntry(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows_core::Result<IMAPIProp>;
    fn CopyEntries(this: &Self::This, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn DeleteEntries(this: &Self::This, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows_core::Result<()>;
    fn ResolveNames(this: &Self::This, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows_core::Result<FlagList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IDistList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIContainer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDistList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDistList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDistList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEntry(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulcreateflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppmapipropentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDistList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyEntries(this, ::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn DeleteEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDistList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteEntries(this, ::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn ResolveNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDistList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut FlagList) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResolveNames(this, ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpadrlist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpflaglist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDistList_Vtbl {
            base__: <IMAPIContainer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEntry: CreateEntry::<Identity, Impl, OFFSET>,
            CopyEntries: CopyEntries::<Identity, Impl, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, Impl, OFFSET>,
            ResolveNames: ResolveNames::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIAdviseSink_Impl: ::windows_core::BaseImpl {
    fn OnNotify(this: &Self::This, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMAPIAdviseSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIAdviseSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPIAdviseSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIAdviseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNotify(this, ::core::mem::transmute_copy(&cnotif), ::core::mem::transmute_copy(&lpnotifications)))
        }
        IMAPIAdviseSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIContainer_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {
    fn GetContentsTable(this: &Self::This, ulflags: u32) -> ::windows_core::Result<IMAPITable>;
    fn GetHierarchyTable(this: &Self::This, ulflags: u32) -> ::windows_core::Result<IMAPITable>;
    fn OpenEntry(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetSearchCriteria(this: &Self::This, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows_core::Result<()>;
    fn GetSearchCriteria(this: &Self::This, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMAPIContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPIContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContentsTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentsTable(this, ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHierarchyTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHierarchyTable(this, ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenEntry(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&lppunk)).into())
        }
        unsafe extern "system" fn SetSearchCriteria<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSearchCriteria(this, ::core::mem::transmute_copy(&lprestriction), ::core::mem::transmute_copy(&lpcontainerlist), ::core::mem::transmute_copy(&ulsearchflags)).into())
        }
        unsafe extern "system" fn GetSearchCriteria<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSearchCriteria(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpprestriction), ::core::mem::transmute_copy(&lppcontainerlist), ::core::mem::transmute_copy(&lpulsearchstate)).into())
        }
        IMAPIContainer_Vtbl {
            base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContentsTable: GetContentsTable::<Identity, Impl, OFFSET>,
            GetHierarchyTable: GetHierarchyTable::<Identity, Impl, OFFSET>,
            OpenEntry: OpenEntry::<Identity, Impl, OFFSET>,
            SetSearchCriteria: SetSearchCriteria::<Identity, Impl, OFFSET>,
            GetSearchCriteria: GetSearchCriteria::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMAPIControl_Impl: ::windows_core::BaseImpl {
    fn GetLastError(this: &Self::This, hresult: ::windows_core::HRESULT, ulflags: u32) -> ::windows_core::Result<*mut MAPIERROR>;
    fn Activate(this: &Self::This, ulflags: u32, uluiparam: usize) -> ::windows_core::Result<()>;
    fn GetState(this: &Self::This, ulflags: u32, lpulstate: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMAPIControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPIControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastError(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppmapierror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, uluiparam: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&uluiparam)).into())
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpulstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetState(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulstate)).into())
        }
        IMAPIControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIFolder_Impl: ::windows_core::BaseImpl + IMAPIContainer_Impl {
    fn CreateMessage(this: &Self::This, lpinterface: *mut ::windows_core::GUID, ulflags: u32, lppmessage: *mut ::core::option::Option<IMessage>) -> ::windows_core::Result<()>;
    fn CopyMessages(this: &Self::This, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows_core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn DeleteMessages(this: &Self::This, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn CreateFolder(this: &Self::This, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows_core::GUID, ulflags: u32) -> ::windows_core::Result<IMAPIFolder>;
    fn CopyFolder(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows_core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn DeleteFolder(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn SetReadFlags(this: &Self::This, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn GetMessageStatus(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows_core::Result<u32>;
    fn SetMessageStatus(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> ::windows_core::Result<u32>;
    fn SaveContentsSort(this: &Self::This, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows_core::Result<()>;
    fn EmptyFolder(this: &Self::This, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMAPIFolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIContainer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPIFolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpinterface: *mut ::windows_core::GUID, ulflags: u32, lppmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateMessage(this, ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmessage)).into())
        }
        unsafe extern "system" fn CopyMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows_core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyMessages(this, ::core::mem::transmute_copy(&lpmsglist), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestfolder), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn DeleteMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMessages(this, ::core::mem::transmute_copy(&lpmsglist), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn CreateFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows_core::GUID, ulflags: u32, lppfolder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFolder(this, ::core::mem::transmute_copy(&ulfoldertype), ::core::mem::transmute_copy(&lpszfoldername), ::core::mem::transmute_copy(&lpszfoldercomment), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppfolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows_core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyFolder(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestfolder), ::core::mem::transmute_copy(&lpsznewfoldername), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFolder(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn SetReadFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReadFlags(this, ::core::mem::transmute_copy(&lpmsglist), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn GetMessageStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessageStatus(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpulmessagestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMessageStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetMessageStatus(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulnewstatus), ::core::mem::transmute_copy(&ulnewstatusmask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpuloldstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveContentsSort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveContentsSort(this, ::core::mem::transmute_copy(&lpsortcriteria), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn EmptyFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EmptyFolder(this, ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        IMAPIFolder_Vtbl {
            base__: <IMAPIContainer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateMessage: CreateMessage::<Identity, Impl, OFFSET>,
            CopyMessages: CopyMessages::<Identity, Impl, OFFSET>,
            DeleteMessages: DeleteMessages::<Identity, Impl, OFFSET>,
            CreateFolder: CreateFolder::<Identity, Impl, OFFSET>,
            CopyFolder: CopyFolder::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            SetReadFlags: SetReadFlags::<Identity, Impl, OFFSET>,
            GetMessageStatus: GetMessageStatus::<Identity, Impl, OFFSET>,
            SetMessageStatus: SetMessageStatus::<Identity, Impl, OFFSET>,
            SaveContentsSort: SaveContentsSort::<Identity, Impl, OFFSET>,
            EmptyFolder: EmptyFolder::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMAPIProgress_Impl: ::windows_core::BaseImpl {
    fn Progress(this: &Self::This, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This, lpulflags: *mut u32) -> ::windows_core::Result<()>;
    fn GetMax(this: &Self::This, lpulmax: *mut u32) -> ::windows_core::Result<()>;
    fn GetMin(this: &Self::This, lpulmin: *mut u32) -> ::windows_core::Result<()>;
    fn SetLimits(this: &Self::This, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMAPIProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPIProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Progress(this, ::core::mem::transmute_copy(&ulvalue), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ultotal)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlags(this, ::core::mem::transmute_copy(&lpulflags)).into())
        }
        unsafe extern "system" fn GetMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpulmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMax(this, ::core::mem::transmute_copy(&lpulmax)).into())
        }
        unsafe extern "system" fn GetMin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpulmin: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMin(this, ::core::mem::transmute_copy(&lpulmin)).into())
        }
        unsafe extern "system" fn SetLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLimits(this, ::core::mem::transmute_copy(&lpulmin), ::core::mem::transmute_copy(&lpulmax), ::core::mem::transmute_copy(&lpulflags)).into())
        }
        IMAPIProgress_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Progress: Progress::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetMax: GetMax::<Identity, Impl, OFFSET>,
            GetMin: GetMin::<Identity, Impl, OFFSET>,
            SetLimits: SetLimits::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIProp_Impl: ::windows_core::BaseImpl {
    fn GetLastError(this: &Self::This, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::Result<()>;
    fn SaveChanges(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
    fn GetProps(this: &Self::This, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows_core::Result<()>;
    fn GetPropList(this: &Self::This, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows_core::Result<()>;
    fn OpenProperty(this: &Self::This, ulproptag: u32, lpiid: *mut ::windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetProps(this: &Self::This, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::Result<()>;
    fn DeleteProps(this: &Self::This, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::Result<()>;
    fn CopyTo(this: &Self::This, ciidexclude: u32, rgiidexclude: *mut ::windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, lpinterface: *mut ::windows_core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::Result<()>;
    fn CopyProps(this: &Self::This, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, lpinterface: *mut ::windows_core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::Result<()>;
    fn GetNamesFromIDs(this: &Self::This, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows_core::Result<()>;
    fn GetIDsFromNames(this: &Self::This, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMAPIProp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPIProp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastError(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmapierror)).into())
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveChanges(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn GetProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProps(this, ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcvalues), ::core::mem::transmute_copy(&lppproparray)).into())
        }
        unsafe extern "system" fn GetPropList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropList(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproptagarray)).into())
        }
        unsafe extern "system" fn OpenProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulproptag: u32, lpiid: *mut ::windows_core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenProperty(this, ::core::mem::transmute_copy(&ulproptag), ::core::mem::transmute_copy(&lpiid), ::core::mem::transmute_copy(&ulinterfaceoptions), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppunk)).into())
        }
        unsafe extern "system" fn SetProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProps(this, ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpproparray), ::core::mem::transmute_copy(&lppproblems)).into())
        }
        unsafe extern "system" fn DeleteProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteProps(this, ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&lppproblems)).into())
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *mut ::windows_core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, lpinterface: *mut ::windows_core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTo(this, ::core::mem::transmute_copy(&ciidexclude), ::core::mem::transmute_copy(&rgiidexclude), ::core::mem::transmute_copy(&lpexcludeprops), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestobj), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproblems)).into())
        }
        unsafe extern "system" fn CopyProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, lpinterface: *mut ::windows_core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyProps(this, ::core::mem::transmute_copy(&lpincludeprops), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestobj), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproblems)).into())
        }
        unsafe extern "system" fn GetNamesFromIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows_core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNamesFromIDs(this, ::core::mem::transmute_copy(&lppproptags), ::core::mem::transmute_copy(&lppropsetguid), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcpropnames), ::core::mem::transmute_copy(&lppppropnames)).into())
        }
        unsafe extern "system" fn GetIDsFromNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIProp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIDsFromNames(this, ::core::mem::transmute_copy(&cpropnames), ::core::mem::transmute_copy(&lpppropnames), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproptags)).into())
        }
        IMAPIProp_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            GetProps: GetProps::<Identity, Impl, OFFSET>,
            GetPropList: GetPropList::<Identity, Impl, OFFSET>,
            OpenProperty: OpenProperty::<Identity, Impl, OFFSET>,
            SetProps: SetProps::<Identity, Impl, OFFSET>,
            DeleteProps: DeleteProps::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            CopyProps: CopyProps::<Identity, Impl, OFFSET>,
            GetNamesFromIDs: GetNamesFromIDs::<Identity, Impl, OFFSET>,
            GetIDsFromNames: GetIDsFromNames::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIStatus_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {
    fn ValidateState(this: &Self::This, uluiparam: usize, ulflags: u32) -> ::windows_core::Result<()>;
    fn SettingsDialog(this: &Self::This, uluiparam: usize, ulflags: u32) -> ::windows_core::Result<()>;
    fn ChangePassword(this: &Self::This, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows_core::Result<()>;
    fn FlushQueues(this: &Self::This, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMAPIStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPIStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ValidateState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateState(this, ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn SettingsDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettingsDialog(this, ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn ChangePassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangePassword(this, ::core::mem::transmute_copy(&lpoldpass), ::core::mem::transmute_copy(&lpnewpass), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn FlushQueues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPIStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushQueues(this, ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&cbtargettransport), ::core::mem::transmute_copy(&lptargettransport), ::core::mem::transmute_copy(&ulflags)).into())
        }
        IMAPIStatus_Vtbl {
            base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ValidateState: ValidateState::<Identity, Impl, OFFSET>,
            SettingsDialog: SettingsDialog::<Identity, Impl, OFFSET>,
            ChangePassword: ChangePassword::<Identity, Impl, OFFSET>,
            FlushQueues: FlushQueues::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPITable_Impl: ::windows_core::BaseImpl {
    fn GetLastError(this: &Self::This, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::Result<()>;
    fn Advise(this: &Self::This, uleventmask: u32, lpadvisesink: ::core::option::Option<&IMAPIAdviseSink>, lpulconnection: *mut u32) -> ::windows_core::Result<()>;
    fn Unadvise(this: &Self::This, ulconnection: u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows_core::Result<()>;
    fn SetColumns(this: &Self::This, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows_core::Result<()>;
    fn QueryColumns(this: &Self::This, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows_core::Result<()>;
    fn GetRowCount(this: &Self::This, ulflags: u32, lpulcount: *mut u32) -> ::windows_core::Result<()>;
    fn SeekRow(this: &Self::This, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows_core::Result<()>;
    fn SeekRowApprox(this: &Self::This, ulnumerator: u32, uldenominator: u32) -> ::windows_core::Result<()>;
    fn QueryPosition(this: &Self::This, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows_core::Result<()>;
    fn FindRow(this: &Self::This, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows_core::Result<()>;
    fn Restrict(this: &Self::This, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows_core::Result<()>;
    fn CreateBookmark(this: &Self::This, lpbkposition: *mut u32) -> ::windows_core::Result<()>;
    fn FreeBookmark(this: &Self::This, bkposition: u32) -> ::windows_core::Result<()>;
    fn SortTable(this: &Self::This, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows_core::Result<()>;
    fn QuerySortOrder(this: &Self::This, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows_core::Result<()>;
    fn QueryRows(this: &Self::This, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExpandRow(this: &Self::This, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows_core::Result<()>;
    fn CollapseRow(this: &Self::This, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows_core::Result<()>;
    fn WaitForCompletion(this: &Self::This, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows_core::Result<()>;
    fn GetCollapseState(this: &Self::This, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows_core::Result<()>;
    fn SetCollapseState(this: &Self::This, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMAPITable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMAPITable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastError(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmapierror)).into())
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Advise(this, ::core::mem::transmute_copy(&uleventmask), ::windows_core::from_raw_borrowed(&lpadvisesink), ::core::mem::transmute_copy(&lpulconnection)).into())
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&ulconnection)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&lpultablestatus), ::core::mem::transmute_copy(&lpultabletype)).into())
        }
        unsafe extern "system" fn SetColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumns(this, ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn QueryColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryColumns(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproptagarray)).into())
        }
        unsafe extern "system" fn GetRowCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowCount(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulcount)).into())
        }
        unsafe extern "system" fn SeekRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SeekRow(this, ::core::mem::transmute_copy(&bkorigin), ::core::mem::transmute_copy(&lrowcount), ::core::mem::transmute_copy(&lplrowssought)).into())
        }
        unsafe extern "system" fn SeekRowApprox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulnumerator: u32, uldenominator: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SeekRowApprox(this, ::core::mem::transmute_copy(&ulnumerator), ::core::mem::transmute_copy(&uldenominator)).into())
        }
        unsafe extern "system" fn QueryPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryPosition(this, ::core::mem::transmute_copy(&lpulrow), ::core::mem::transmute_copy(&lpulnumerator), ::core::mem::transmute_copy(&lpuldenominator)).into())
        }
        unsafe extern "system" fn FindRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindRow(this, ::core::mem::transmute_copy(&lprestriction), ::core::mem::transmute_copy(&bkorigin), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn Restrict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restrict(this, ::core::mem::transmute_copy(&lprestriction), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn CreateBookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbkposition: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBookmark(this, ::core::mem::transmute_copy(&lpbkposition)).into())
        }
        unsafe extern "system" fn FreeBookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bkposition: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeBookmark(this, ::core::mem::transmute_copy(&bkposition)).into())
        }
        unsafe extern "system" fn SortTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SortTable(this, ::core::mem::transmute_copy(&lpsortcriteria), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn QuerySortOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QuerySortOrder(this, ::core::mem::transmute_copy(&lppsortcriteria)).into())
        }
        unsafe extern "system" fn QueryRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryRows(this, ::core::mem::transmute_copy(&lrowcount), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpprows)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn ExpandRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExpandRow(this, ::core::mem::transmute_copy(&cbinstancekey), ::core::mem::transmute_copy(&pbinstancekey), ::core::mem::transmute_copy(&ulrowcount), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpprows), ::core::mem::transmute_copy(&lpulmorerows)).into())
        }
        unsafe extern "system" fn CollapseRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CollapseRow(this, ::core::mem::transmute_copy(&cbinstancekey), ::core::mem::transmute_copy(&pbinstancekey), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulrowcount)).into())
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForCompletion(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ultimeout), ::core::mem::transmute_copy(&lpultablestatus)).into())
        }
        unsafe extern "system" fn GetCollapseState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCollapseState(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbinstancekey), ::core::mem::transmute_copy(&lpbinstancekey), ::core::mem::transmute_copy(&lpcbcollapsestate), ::core::mem::transmute_copy(&lppbcollapsestate)).into())
        }
        unsafe extern "system" fn SetCollapseState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMAPITable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCollapseState(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbcollapsestate), ::core::mem::transmute_copy(&pbcollapsestate), ::core::mem::transmute_copy(&lpbklocation)).into())
        }
        IMAPITable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetColumns: SetColumns::<Identity, Impl, OFFSET>,
            QueryColumns: QueryColumns::<Identity, Impl, OFFSET>,
            GetRowCount: GetRowCount::<Identity, Impl, OFFSET>,
            SeekRow: SeekRow::<Identity, Impl, OFFSET>,
            SeekRowApprox: SeekRowApprox::<Identity, Impl, OFFSET>,
            QueryPosition: QueryPosition::<Identity, Impl, OFFSET>,
            FindRow: FindRow::<Identity, Impl, OFFSET>,
            Restrict: Restrict::<Identity, Impl, OFFSET>,
            CreateBookmark: CreateBookmark::<Identity, Impl, OFFSET>,
            FreeBookmark: FreeBookmark::<Identity, Impl, OFFSET>,
            SortTable: SortTable::<Identity, Impl, OFFSET>,
            QuerySortOrder: QuerySortOrder::<Identity, Impl, OFFSET>,
            QueryRows: QueryRows::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            ExpandRow: ExpandRow::<Identity, Impl, OFFSET>,
            CollapseRow: CollapseRow::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            GetCollapseState: GetCollapseState::<Identity, Impl, OFFSET>,
            SetCollapseState: SetCollapseState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMailUser_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMailUser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMailUser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMailUser {
    const VTABLE: Self::Vtable = { IMailUser_Vtbl { base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMessage_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {
    fn GetAttachmentTable(this: &Self::This, ulflags: u32) -> ::windows_core::Result<IMAPITable>;
    fn OpenAttach(this: &Self::This, ulattachmentnum: u32, lpinterface: *const ::windows_core::GUID, ulflags: u32) -> ::windows_core::Result<IAttach>;
    fn CreateAttach(this: &Self::This, lpinterface: *const ::windows_core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut ::core::option::Option<IAttach>) -> ::windows_core::Result<()>;
    fn DeleteAttach(this: &Self::This, ulattachmentnum: u32, uluiparam: usize, lpprogress: ::core::option::Option<&IMAPIProgress>, ulflags: u32) -> ::windows_core::Result<()>;
    fn GetRecipientTable(this: &Self::This, ulflags: u32) -> ::windows_core::Result<IMAPITable>;
    fn ModifyRecipients(this: &Self::This, ulflags: u32, lpmods: *const ADRLIST) -> ::windows_core::Result<()>;
    fn SubmitMessage(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
    fn SetReadFlag(this: &Self::This, ulflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAttachmentTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttachmentTable(this, ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenAttach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, lpinterface: *const ::windows_core::GUID, ulflags: u32, lppattach: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenAttach(this, ::core::mem::transmute_copy(&ulattachmentnum), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppattach, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAttach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpinterface: *const ::windows_core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAttach(this, ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulattachmentnum), ::core::mem::transmute_copy(&lppattach)).into())
        }
        unsafe extern "system" fn DeleteAttach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, uluiparam: usize, lpprogress: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttach(this, ::core::mem::transmute_copy(&ulattachmentnum), ::core::mem::transmute_copy(&uluiparam), ::windows_core::from_raw_borrowed(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn GetRecipientTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecipientTable(this, ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifyRecipients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpmods: *const ADRLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyRecipients(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpmods)).into())
        }
        unsafe extern "system" fn SubmitMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubmitMessage(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn SetReadFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReadFlag(this, ::core::mem::transmute_copy(&ulflags)).into())
        }
        IMessage_Vtbl {
            base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAttachmentTable: GetAttachmentTable::<Identity, Impl, OFFSET>,
            OpenAttach: OpenAttach::<Identity, Impl, OFFSET>,
            CreateAttach: CreateAttach::<Identity, Impl, OFFSET>,
            DeleteAttach: DeleteAttach::<Identity, Impl, OFFSET>,
            GetRecipientTable: GetRecipientTable::<Identity, Impl, OFFSET>,
            ModifyRecipients: ModifyRecipients::<Identity, Impl, OFFSET>,
            SubmitMessage: SubmitMessage::<Identity, Impl, OFFSET>,
            SetReadFlag: SetReadFlag::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMsgStore_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {
    fn Advise(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: ::core::option::Option<&IMAPIAdviseSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, ulconnection: u32) -> ::windows_core::Result<()>;
    fn CompareEntryIDs(this: &Self::This, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> ::windows_core::Result<u32>;
    fn OpenEntry(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetReceiveFolder(this: &Self::This, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows_core::Result<()>;
    fn GetReceiveFolder(this: &Self::This, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows_core::Result<()>;
    fn GetReceiveFolderTable(this: &Self::This, ulflags: u32) -> ::windows_core::Result<IMAPITable>;
    fn StoreLogoff(this: &Self::This, lpulflags: *mut u32) -> ::windows_core::Result<()>;
    fn AbortSubmit(this: &Self::This, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows_core::Result<()>;
    fn GetOutgoingQueue(this: &Self::This, ulflags: u32) -> ::windows_core::Result<IMAPITable>;
    fn SetLockState(this: &Self::This, lpmessage: ::core::option::Option<&IMessage>, ullockstate: u32) -> ::windows_core::Result<()>;
    fn FinishedMsg(this: &Self::This, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows_core::Result<()>;
    fn NotifyNewMail(this: &Self::This, lpnotification: *const NOTIFICATION) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMsgStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsgStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: *mut ::core::ffi::c_void, lpulconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&uleventmask), ::windows_core::from_raw_borrowed(&lpadvisesink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpulconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&ulconnection)).into())
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareEntryIDs(this, ::core::mem::transmute_copy(&cbentryid1), ::core::mem::transmute_copy(&lpentryid1), ::core::mem::transmute_copy(&cbentryid2), ::core::mem::transmute_copy(&lpentryid2), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpulresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows_core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenEntry(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn SetReceiveFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReceiveFolder(this, ::core::mem::transmute_copy(&lpszmessageclass), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into())
        }
        unsafe extern "system" fn GetReceiveFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReceiveFolder(this, ::core::mem::transmute_copy(&lpszmessageclass), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid), ::core::mem::transmute_copy(&lppszexplicitclass)).into())
        }
        unsafe extern "system" fn GetReceiveFolderTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReceiveFolderTable(this, ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StoreLogoff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StoreLogoff(this, ::core::mem::transmute_copy(&lpulflags)).into())
        }
        unsafe extern "system" fn AbortSubmit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortSubmit(this, ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulflags)).into())
        }
        unsafe extern "system" fn GetOutgoingQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutgoingQueue(this, ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLockState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmessage: *mut ::core::ffi::c_void, ullockstate: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLockState(this, ::windows_core::from_raw_borrowed(&lpmessage), ::core::mem::transmute_copy(&ullockstate)).into())
        }
        unsafe extern "system" fn FinishedMsg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishedMsg(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into())
        }
        unsafe extern "system" fn NotifyNewMail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsgStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpnotification: *const NOTIFICATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyNewMail(this, ::core::mem::transmute_copy(&lpnotification)).into())
        }
        IMsgStore_Vtbl {
            base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, Impl, OFFSET>,
            OpenEntry: OpenEntry::<Identity, Impl, OFFSET>,
            SetReceiveFolder: SetReceiveFolder::<Identity, Impl, OFFSET>,
            GetReceiveFolder: GetReceiveFolder::<Identity, Impl, OFFSET>,
            GetReceiveFolderTable: GetReceiveFolderTable::<Identity, Impl, OFFSET>,
            StoreLogoff: StoreLogoff::<Identity, Impl, OFFSET>,
            AbortSubmit: AbortSubmit::<Identity, Impl, OFFSET>,
            GetOutgoingQueue: GetOutgoingQueue::<Identity, Impl, OFFSET>,
            SetLockState: SetLockState::<Identity, Impl, OFFSET>,
            FinishedMsg: FinishedMsg::<Identity, Impl, OFFSET>,
            NotifyNewMail: NotifyNewMail::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IProfSect_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IProfSect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProfSect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProfSect {
    const VTABLE: Self::Vtable = { IProfSect_Vtbl { base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPropData_Impl: ::windows_core::BaseImpl + IMAPIProp_Impl {
    fn HrSetObjAccess(this: &Self::This, ulaccess: u32) -> ::windows_core::Result<()>;
    fn HrSetPropAccess(this: &Self::This, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows_core::Result<()>;
    fn HrGetPropAccess(this: &Self::This, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows_core::Result<()>;
    fn HrAddObjProps(this: &Self::This, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPropData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMAPIProp);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HrSetObjAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulaccess: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrSetObjAccess(this, ::core::mem::transmute_copy(&ulaccess)).into())
        }
        unsafe extern "system" fn HrSetPropAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrSetPropAccess(this, ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&rgulaccess)).into())
        }
        unsafe extern "system" fn HrGetPropAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrGetPropAccess(this, ::core::mem::transmute_copy(&lppproptagarray), ::core::mem::transmute_copy(&lprgulaccess)).into())
        }
        unsafe extern "system" fn HrAddObjProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrAddObjProps(this, ::core::mem::transmute_copy(&lppproptagarray), ::core::mem::transmute_copy(&lprgulaccess)).into())
        }
        IPropData_Vtbl {
            base__: <IMAPIProp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HrSetObjAccess: HrSetObjAccess::<Identity, Impl, OFFSET>,
            HrSetPropAccess: HrSetPropAccess::<Identity, Impl, OFFSET>,
            HrGetPropAccess: HrGetPropAccess::<Identity, Impl, OFFSET>,
            HrAddObjProps: HrAddObjProps::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IProviderAdmin_Impl: ::windows_core::BaseImpl {
    fn GetLastError(this: &Self::This, hresult: ::windows_core::HRESULT, ulflags: u32) -> ::windows_core::Result<*mut MAPIERROR>;
    fn GetProviderTable(this: &Self::This, ulflags: u32) -> ::windows_core::Result<IMAPITable>;
    fn CreateProvider(this: &Self::This, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32) -> ::windows_core::Result<MAPIUID>;
    fn DeleteProvider(this: &Self::This, lpuid: *const MAPIUID) -> ::windows_core::Result<()>;
    fn OpenProfileSection(this: &Self::This, lpuid: *const MAPIUID, lpinterface: *const ::windows_core::GUID, ulflags: u32) -> ::windows_core::Result<IProfSect>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IProviderAdmin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderAdmin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProviderAdmin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastError(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppmapierror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderTable(this, ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateProvider(this, ::core::mem::transmute_copy(&lpszprovider), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpprops), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpuid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteProvider(this, ::core::mem::transmute_copy(&lpuid)).into())
        }
        unsafe extern "system" fn OpenProfileSection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID, lpinterface: *const ::windows_core::GUID, ulflags: u32, lppprofsect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenProfileSection(this, ::core::mem::transmute_copy(&lpuid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppprofsect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProviderAdmin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GetProviderTable: GetProviderTable::<Identity, Impl, OFFSET>,
            CreateProvider: CreateProvider::<Identity, Impl, OFFSET>,
            DeleteProvider: DeleteProvider::<Identity, Impl, OFFSET>,
            OpenProfileSection: OpenProfileSection::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITableData_Impl: ::windows_core::BaseImpl {
    fn HrGetView(this: &Self::This, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut ::core::option::Option<IMAPITable>) -> ::windows_core::Result<()>;
    fn HrModifyRow(this: &Self::This, param0: *mut SRow) -> ::windows_core::Result<()>;
    fn HrDeleteRow(this: &Self::This, lpspropvalue: *mut SPropValue) -> ::windows_core::Result<()>;
    fn HrQueryRow(this: &Self::This, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows_core::Result<()>;
    fn HrEnumRow(this: &Self::This, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows_core::Result<()>;
    fn HrNotify(this: &Self::This, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows_core::Result<()>;
    fn HrInsertRow(this: &Self::This, ulirow: u32, lpsrow: *mut SRow) -> ::windows_core::Result<()>;
    fn HrModifyRows(this: &Self::This, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows_core::Result<()>;
    fn HrDeleteRows(this: &Self::This, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITableData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITableData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HrGetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrGetView(this, ::core::mem::transmute_copy(&lpssortorderset), ::core::mem::transmute_copy(&lpfcallerrelease), ::core::mem::transmute_copy(&ulcallerdata), ::core::mem::transmute_copy(&lppmapitable)).into())
        }
        unsafe extern "system" fn HrModifyRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, param0: *mut SRow) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrModifyRow(this, ::core::mem::transmute_copy(&param0)).into())
        }
        unsafe extern "system" fn HrDeleteRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrDeleteRow(this, ::core::mem::transmute_copy(&lpspropvalue)).into())
        }
        unsafe extern "system" fn HrQueryRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrQueryRow(this, ::core::mem::transmute_copy(&lpspropvalue), ::core::mem::transmute_copy(&lppsrow), ::core::mem::transmute_copy(&lpulirow)).into())
        }
        unsafe extern "system" fn HrEnumRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrEnumRow(this, ::core::mem::transmute_copy(&ulrownumber), ::core::mem::transmute_copy(&lppsrow)).into())
        }
        unsafe extern "system" fn HrNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrNotify(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpspropvalue)).into())
        }
        unsafe extern "system" fn HrInsertRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulirow: u32, lpsrow: *mut SRow) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrInsertRow(this, ::core::mem::transmute_copy(&ulirow), ::core::mem::transmute_copy(&lpsrow)).into())
        }
        unsafe extern "system" fn HrModifyRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrModifyRows(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpsrowset)).into())
        }
        unsafe extern "system" fn HrDeleteRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HrDeleteRows(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lprowsettodelete), ::core::mem::transmute_copy(&crowsdeleted)).into())
        }
        ITableData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HrGetView: HrGetView::<Identity, Impl, OFFSET>,
            HrModifyRow: HrModifyRow::<Identity, Impl, OFFSET>,
            HrDeleteRow: HrDeleteRow::<Identity, Impl, OFFSET>,
            HrQueryRow: HrQueryRow::<Identity, Impl, OFFSET>,
            HrEnumRow: HrEnumRow::<Identity, Impl, OFFSET>,
            HrNotify: HrNotify::<Identity, Impl, OFFSET>,
            HrInsertRow: HrInsertRow::<Identity, Impl, OFFSET>,
            HrModifyRows: HrModifyRows::<Identity, Impl, OFFSET>,
            HrDeleteRows: HrDeleteRows::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWABExtInit_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWABExtInit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABExtInit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWABExtInit {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABExtInit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&lpwabextdisplay)).into())
        }
        IWABExtInit_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWABObject_Impl: ::windows_core::BaseImpl {
    fn GetLastError(this: &Self::This, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::Result<()>;
    fn AllocateBuffer(this: &Self::This, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn AllocateMore(this: &Self::This, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FreeBuffer(this: &Self::This, lpbuffer: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Backup(this: &Self::This, lpfilename: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn Import(this: &Self::This, lpwip: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn Find(this: &Self::This, lpiab: ::core::option::Option<&IAddrBook>, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn VCardDisplay(this: &Self::This, lpiab: ::core::option::Option<&IAddrBook>, hwnd: super::super::Foundation::HWND, lpszfilename: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn LDAPUrl(this: &Self::This, lpiab: ::core::option::Option<&IAddrBook>, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: &::windows_core::PCSTR) -> ::windows_core::Result<IMailUser>;
    fn VCardCreate(this: &Self::This, lpiab: ::core::option::Option<&IAddrBook>, ulflags: u32, lpszvcard: &::windows_core::PCSTR, lpmailuser: ::core::option::Option<&IMailUser>) -> ::windows_core::Result<()>;
    fn VCardRetrieve(this: &Self::This, lpiab: ::core::option::Option<&IAddrBook>, ulflags: u32, lpszvcard: &::windows_core::PCSTR) -> ::windows_core::Result<IMailUser>;
    fn GetMe(this: &Self::This, lpiab: ::core::option::Option<&IAddrBook>, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn SetMe(this: &Self::This, lpiab: ::core::option::Option<&IAddrBook>, ulflags: u32, sbeid: &SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWABObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWABObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastError(this, ::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmapierror)).into())
        }
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateBuffer(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&lppbuffer)).into())
        }
        unsafe extern "system" fn AllocateMore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AllocateMore(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&lpobject), ::core::mem::transmute_copy(&lppbuffer)).into())
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeBuffer(this, ::core::mem::transmute_copy(&lpbuffer)).into())
        }
        unsafe extern "system" fn Backup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpfilename: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Backup(this, ::core::mem::transmute(&lpfilename)).into())
        }
        unsafe extern "system" fn Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpwip: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Import(this, ::core::mem::transmute(&lpwip)).into())
        }
        unsafe extern "system" fn Find<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Find(this, ::windows_core::from_raw_borrowed(&lpiab), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn VCardDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, lpszfilename: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VCardDisplay(this, ::windows_core::from_raw_borrowed(&lpiab), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&lpszfilename)).into())
        }
        unsafe extern "system" fn LDAPUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: ::windows_core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LDAPUrl(this, ::windows_core::from_raw_borrowed(&lpiab), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute(&lpszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppmailuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VCardCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows_core::PCSTR, lpmailuser: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VCardCreate(this, ::windows_core::from_raw_borrowed(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute(&lpszvcard), ::windows_core::from_raw_borrowed(&lpmailuser)).into())
        }
        unsafe extern "system" fn VCardRetrieve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpszvcard: ::windows_core::PCSTR, lppmailuser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VCardRetrieve(this, ::windows_core::from_raw_borrowed(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute(&lpszvcard)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppmailuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMe(this, ::windows_core::from_raw_borrowed(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpdwaction), ::core::mem::transmute_copy(&lpsbeid), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn SetMe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWABObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiab: *mut ::core::ffi::c_void, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMe(this, ::windows_core::from_raw_borrowed(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute(&sbeid), ::core::mem::transmute_copy(&hwnd)).into())
        }
        IWABObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            AllocateMore: AllocateMore::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            Find: Find::<Identity, Impl, OFFSET>,
            VCardDisplay: VCardDisplay::<Identity, Impl, OFFSET>,
            LDAPUrl: LDAPUrl::<Identity, Impl, OFFSET>,
            VCardCreate: VCardCreate::<Identity, Impl, OFFSET>,
            VCardRetrieve: VCardRetrieve::<Identity, Impl, OFFSET>,
            GetMe: GetMe::<Identity, Impl, OFFSET>,
            SetMe: SetMe::<Identity, Impl, OFFSET>,
        }
    };
}
