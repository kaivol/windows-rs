#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDiskQuotaControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IConnectionPointContainer_Impl {
    fn Initialize(this: &Self::This, pszpath: &::windows_core::PCWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetQuotaState(this: &Self::This, dwstate: u32) -> ::windows_core::Result<()>;
    fn GetQuotaState(this: &Self::This, pdwstate: *mut u32) -> ::windows_core::Result<()>;
    fn SetQuotaLogFlags(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetQuotaLogFlags(this: &Self::This, pdwflags: *mut u32) -> ::windows_core::Result<()>;
    fn SetDefaultQuotaThreshold(this: &Self::This, llthreshold: i64) -> ::windows_core::Result<()>;
    fn GetDefaultQuotaThreshold(this: &Self::This, pllthreshold: *mut i64) -> ::windows_core::Result<()>;
    fn GetDefaultQuotaThresholdText(this: &Self::This, psztext: &::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn SetDefaultQuotaLimit(this: &Self::This, lllimit: i64) -> ::windows_core::Result<()>;
    fn GetDefaultQuotaLimit(this: &Self::This, plllimit: *mut i64) -> ::windows_core::Result<()>;
    fn GetDefaultQuotaLimitText(this: &Self::This, psztext: &::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn AddUserSid(this: &Self::This, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows_core::Result<IDiskQuotaUser>;
    fn AddUserName(this: &Self::This, pszlogonname: &::windows_core::PCWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows_core::Result<IDiskQuotaUser>;
    fn DeleteUser(this: &Self::This, puser: ::core::option::Option<&IDiskQuotaUser>) -> ::windows_core::Result<()>;
    fn FindUserSid(this: &Self::This, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows_core::Result<IDiskQuotaUser>;
    fn FindUserName(this: &Self::This, pszlogonname: &::windows_core::PCWSTR) -> ::windows_core::Result<IDiskQuotaUser>;
    fn CreateEnumUsers(this: &Self::This, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut ::core::option::Option<IEnumDiskQuotaUsers>) -> ::windows_core::Result<()>;
    fn CreateUserBatch(this: &Self::This) -> ::windows_core::Result<IDiskQuotaUserBatch>;
    fn InvalidateSidNameCache(this: &Self::This) -> ::windows_core::Result<()>;
    fn GiveUserNameResolutionPriority(this: &Self::This, puser: ::core::option::Option<&IDiskQuotaUser>) -> ::windows_core::Result<()>;
    fn ShutdownNameResolution(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IDiskQuotaControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IConnectionPointContainer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiskQuotaControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&breadwrite)).into())
        }
        unsafe extern "system" fn SetQuotaState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuotaState(this, ::core::mem::transmute_copy(&dwstate)).into())
        }
        unsafe extern "system" fn GetQuotaState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaState(this, ::core::mem::transmute_copy(&pdwstate)).into())
        }
        unsafe extern "system" fn SetQuotaLogFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuotaLogFlags(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetQuotaLogFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaLogFlags(this, ::core::mem::transmute_copy(&pdwflags)).into())
        }
        unsafe extern "system" fn SetDefaultQuotaThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llthreshold: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultQuotaThreshold(this, ::core::mem::transmute_copy(&llthreshold)).into())
        }
        unsafe extern "system" fn GetDefaultQuotaThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultQuotaThreshold(this, ::core::mem::transmute_copy(&pllthreshold)).into())
        }
        unsafe extern "system" fn GetDefaultQuotaThresholdText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultQuotaThresholdText(this, ::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn SetDefaultQuotaLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lllimit: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultQuotaLimit(this, ::core::mem::transmute_copy(&lllimit)).into())
        }
        unsafe extern "system" fn GetDefaultQuotaLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultQuotaLimit(this, ::core::mem::transmute_copy(&plllimit)).into())
        }
        unsafe extern "system" fn GetDefaultQuotaLimitText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultQuotaLimitText(this, ::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn AddUserSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddUserSid(this, ::core::mem::transmute_copy(&pusersid), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlogonname: ::windows_core::PCWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddUserName(this, ::core::mem::transmute(&pszlogonname), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteUser(this, ::windows_core::from_raw_borrowed(&puser)).into())
        }
        unsafe extern "system" fn FindUserSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindUserSid(this, ::core::mem::transmute_copy(&pusersid), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlogonname: ::windows_core::PCWSTR, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindUserName(this, ::core::mem::transmute(&pszlogonname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEnumUsers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateEnumUsers(this, ::core::mem::transmute_copy(&rgpusersids), ::core::mem::transmute_copy(&cpsids), ::core::mem::transmute_copy(&fnameresolution), ::core::mem::transmute_copy(&ppenum)).into())
        }
        unsafe extern "system" fn CreateUserBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateUserBatch(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InvalidateSidNameCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateSidNameCache(this).into())
        }
        unsafe extern "system" fn GiveUserNameResolutionPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GiveUserNameResolutionPriority(this, ::windows_core::from_raw_borrowed(&puser)).into())
        }
        unsafe extern "system" fn ShutdownNameResolution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownNameResolution(this).into())
        }
        IDiskQuotaControl_Vtbl {
            base__: <super::super::System::Com::IConnectionPointContainer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetQuotaState: SetQuotaState::<Identity, Impl, OFFSET>,
            GetQuotaState: GetQuotaState::<Identity, Impl, OFFSET>,
            SetQuotaLogFlags: SetQuotaLogFlags::<Identity, Impl, OFFSET>,
            GetQuotaLogFlags: GetQuotaLogFlags::<Identity, Impl, OFFSET>,
            SetDefaultQuotaThreshold: SetDefaultQuotaThreshold::<Identity, Impl, OFFSET>,
            GetDefaultQuotaThreshold: GetDefaultQuotaThreshold::<Identity, Impl, OFFSET>,
            GetDefaultQuotaThresholdText: GetDefaultQuotaThresholdText::<Identity, Impl, OFFSET>,
            SetDefaultQuotaLimit: SetDefaultQuotaLimit::<Identity, Impl, OFFSET>,
            GetDefaultQuotaLimit: GetDefaultQuotaLimit::<Identity, Impl, OFFSET>,
            GetDefaultQuotaLimitText: GetDefaultQuotaLimitText::<Identity, Impl, OFFSET>,
            AddUserSid: AddUserSid::<Identity, Impl, OFFSET>,
            AddUserName: AddUserName::<Identity, Impl, OFFSET>,
            DeleteUser: DeleteUser::<Identity, Impl, OFFSET>,
            FindUserSid: FindUserSid::<Identity, Impl, OFFSET>,
            FindUserName: FindUserName::<Identity, Impl, OFFSET>,
            CreateEnumUsers: CreateEnumUsers::<Identity, Impl, OFFSET>,
            CreateUserBatch: CreateUserBatch::<Identity, Impl, OFFSET>,
            InvalidateSidNameCache: InvalidateSidNameCache::<Identity, Impl, OFFSET>,
            GiveUserNameResolutionPriority: GiveUserNameResolutionPriority::<Identity, Impl, OFFSET>,
            ShutdownNameResolution: ShutdownNameResolution::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDiskQuotaEvents_Impl: ::windows_core::BaseImpl {
    fn OnUserNameChanged(this: &Self::This, puser: ::core::option::Option<&IDiskQuotaUser>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDiskQuotaEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiskQuotaEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUserNameChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUserNameChanged(this, ::windows_core::from_raw_borrowed(&puser)).into())
        }
        IDiskQuotaEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnUserNameChanged: OnUserNameChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDiskQuotaUser_Impl: ::windows_core::BaseImpl {
    fn GetID(this: &Self::This, pulid: *mut u32) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This, pszaccountcontainer: &::windows_core::PCWSTR, cchaccountcontainer: u32, pszlogonname: &::windows_core::PCWSTR, cchlogonname: u32, pszdisplayname: &::windows_core::PCWSTR, cchdisplayname: u32) -> ::windows_core::Result<()>;
    fn GetSidLength(this: &Self::This, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetSid(this: &Self::This, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows_core::Result<()>;
    fn GetQuotaThreshold(this: &Self::This, pllthreshold: *mut i64) -> ::windows_core::Result<()>;
    fn GetQuotaThresholdText(this: &Self::This, psztext: &::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetQuotaLimit(this: &Self::This, plllimit: *mut i64) -> ::windows_core::Result<()>;
    fn GetQuotaLimitText(this: &Self::This, psztext: &::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetQuotaUsed(this: &Self::This, pllused: *mut i64) -> ::windows_core::Result<()>;
    fn GetQuotaUsedText(this: &Self::This, psztext: &::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetQuotaInformation(this: &Self::This, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows_core::Result<()>;
    fn SetQuotaThreshold(this: &Self::This, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetQuotaLimit(this: &Self::This, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Invalidate(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAccountStatus(this: &Self::This, pdwstatus: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDiskQuotaUser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiskQuotaUser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetID(this, ::core::mem::transmute_copy(&pulid)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszaccountcontainer: ::windows_core::PCWSTR, cchaccountcontainer: u32, pszlogonname: ::windows_core::PCWSTR, cchlogonname: u32, pszdisplayname: ::windows_core::PCWSTR, cchdisplayname: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute(&pszaccountcontainer), ::core::mem::transmute_copy(&cchaccountcontainer), ::core::mem::transmute(&pszlogonname), ::core::mem::transmute_copy(&cchlogonname), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&cchdisplayname)).into())
        }
        unsafe extern "system" fn GetSidLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSidLength(this, ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn GetSid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSid(this, ::core::mem::transmute_copy(&pbsidbuffer), ::core::mem::transmute_copy(&cbsidbuffer)).into())
        }
        unsafe extern "system" fn GetQuotaThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaThreshold(this, ::core::mem::transmute_copy(&pllthreshold)).into())
        }
        unsafe extern "system" fn GetQuotaThresholdText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaThresholdText(this, ::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn GetQuotaLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaLimit(this, ::core::mem::transmute_copy(&plllimit)).into())
        }
        unsafe extern "system" fn GetQuotaLimitText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaLimitText(this, ::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn GetQuotaUsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllused: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaUsed(this, ::core::mem::transmute_copy(&pllused)).into())
        }
        unsafe extern "system" fn GetQuotaUsedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaUsedText(this, ::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn GetQuotaInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuotaInformation(this, ::core::mem::transmute_copy(&pbquotainfo), ::core::mem::transmute_copy(&cbquotainfo)).into())
        }
        unsafe extern "system" fn SetQuotaThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuotaThreshold(this, ::core::mem::transmute_copy(&llthreshold), ::core::mem::transmute_copy(&fwritethrough)).into())
        }
        unsafe extern "system" fn SetQuotaLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuotaLimit(this, ::core::mem::transmute_copy(&lllimit), ::core::mem::transmute_copy(&fwritethrough)).into())
        }
        unsafe extern "system" fn Invalidate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invalidate(this).into())
        }
        unsafe extern "system" fn GetAccountStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAccountStatus(this, ::core::mem::transmute_copy(&pdwstatus)).into())
        }
        IDiskQuotaUser_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSidLength: GetSidLength::<Identity, Impl, OFFSET>,
            GetSid: GetSid::<Identity, Impl, OFFSET>,
            GetQuotaThreshold: GetQuotaThreshold::<Identity, Impl, OFFSET>,
            GetQuotaThresholdText: GetQuotaThresholdText::<Identity, Impl, OFFSET>,
            GetQuotaLimit: GetQuotaLimit::<Identity, Impl, OFFSET>,
            GetQuotaLimitText: GetQuotaLimitText::<Identity, Impl, OFFSET>,
            GetQuotaUsed: GetQuotaUsed::<Identity, Impl, OFFSET>,
            GetQuotaUsedText: GetQuotaUsedText::<Identity, Impl, OFFSET>,
            GetQuotaInformation: GetQuotaInformation::<Identity, Impl, OFFSET>,
            SetQuotaThreshold: SetQuotaThreshold::<Identity, Impl, OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetAccountStatus: GetAccountStatus::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDiskQuotaUserBatch_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, puser: ::core::option::Option<&IDiskQuotaUser>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, puser: ::core::option::Option<&IDiskQuotaUser>) -> ::windows_core::Result<()>;
    fn RemoveAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn FlushToDisk(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDiskQuotaUserBatch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDiskQuotaUserBatch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&puser)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::windows_core::from_raw_borrowed(&puser)).into())
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAll(this).into())
        }
        unsafe extern "system" fn FlushToDisk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushToDisk(this).into())
        }
        IDiskQuotaUserBatch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            FlushToDisk: FlushToDisk::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumDiskQuotaUsers_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cusers: u32, rgusers: *mut ::core::option::Option<IDiskQuotaUser>, pcusersfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cusers: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDiskQuotaUsers>;
}
impl ::windows_core::Iids for IEnumDiskQuotaUsers {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDiskQuotaUsers {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cusers: u32, rgusers: *mut *mut ::core::ffi::c_void, pcusersfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cusers), ::core::mem::transmute_copy(&rgusers), ::core::mem::transmute_copy(&pcusersfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cusers: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cusers)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDiskQuotaUsers_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
