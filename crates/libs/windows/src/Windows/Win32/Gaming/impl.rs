#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer_Impl: ::windows_core::BaseImpl {
    fn AddGame(this: &Self::This, bstrgdfbinarypath: &::windows_core::BSTR, bstrgameinstalldirectory: &::windows_core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RemoveGame(this: &Self::This, guidinstanceid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UpdateGame(this: &Self::This, guidinstanceid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn VerifyAccess(this: &Self::This, bstrgdfbinarypath: &::windows_core::BSTR) -> ::windows_core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IGameExplorer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameExplorer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddGame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrgameinstalldirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddGame(this, ::core::mem::transmute(&bstrgdfbinarypath), ::core::mem::transmute(&bstrgameinstalldirectory), ::core::mem::transmute_copy(&installscope), ::core::mem::transmute_copy(&pguidinstanceid)).into())
        }
        unsafe extern "system" fn RemoveGame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveGame(this, ::core::mem::transmute(&guidinstanceid)).into())
        }
        unsafe extern "system" fn UpdateGame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateGame(this, ::core::mem::transmute(&guidinstanceid)).into())
        }
        unsafe extern "system" fn VerifyAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VerifyAccess(this, ::core::mem::transmute(&bstrgdfbinarypath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGameExplorer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddGame: AddGame::<Identity, Impl, OFFSET>,
            RemoveGame: RemoveGame::<Identity, Impl, OFFSET>,
            UpdateGame: UpdateGame::<Identity, Impl, OFFSET>,
            VerifyAccess: VerifyAccess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGameExplorer2_Impl: ::windows_core::BaseImpl {
    fn InstallGame(this: &Self::This, binarygdfpath: &::windows_core::PCWSTR, installdirectory: &::windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows_core::Result<()>;
    fn UninstallGame(this: &Self::This, binarygdfpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CheckAccess(this: &Self::This, binarygdfpath: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IGameExplorer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameExplorer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InstallGame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR, installdirectory: ::windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallGame(this, ::core::mem::transmute(&binarygdfpath), ::core::mem::transmute(&installdirectory), ::core::mem::transmute_copy(&installscope)).into())
        }
        unsafe extern "system" fn UninstallGame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UninstallGame(this, ::core::mem::transmute(&binarygdfpath)).into())
        }
        unsafe extern "system" fn CheckAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameExplorer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckAccess(this, ::core::mem::transmute(&binarygdfpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phasaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGameExplorer2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InstallGame: InstallGame::<Identity, Impl, OFFSET>,
            UninstallGame: UninstallGame::<Identity, Impl, OFFSET>,
            CheckAccess: CheckAccess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGameStatistics_Impl: ::windows_core::BaseImpl {
    fn GetMaxCategoryLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMaxNameLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMaxValueLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetMaxCategories(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetMaxStatsPerCategory(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SetCategoryTitle(this: &Self::This, categoryindex: u16, title: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCategoryTitle(this: &Self::This, categoryindex: u16) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetStatistic(this: &Self::This, categoryindex: u16, statindex: u16, pname: *mut ::windows_core::PWSTR, pvalue: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetStatistic(this: &Self::This, categoryindex: u16, statindex: u16, name: &::windows_core::PCWSTR, value: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, trackchanges: super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetLastPlayedCategory(this: &Self::This, categoryindex: u32) -> ::windows_core::Result<()>;
    fn GetLastPlayedCategory(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IGameStatistics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameStatistics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaxCategoryLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxCategoryLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxNameLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxValueLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxValueLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxCategories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxStatsPerCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCategoryTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, categoryindex: u16, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCategoryTitle(this, ::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute(&title)).into())
        }
        unsafe extern "system" fn GetCategoryTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategoryTitle(this, ::core::mem::transmute_copy(&categoryindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatistic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut ::windows_core::PWSTR, pvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatistic(this, ::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn SetStatistic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatistic(this, ::core::mem::transmute_copy(&categoryindex), ::core::mem::transmute_copy(&statindex), ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::core::mem::transmute_copy(&trackchanges)).into())
        }
        unsafe extern "system" fn SetLastPlayedCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastPlayedCategory(this, ::core::mem::transmute_copy(&categoryindex)).into())
        }
        unsafe extern "system" fn GetLastPlayedCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatistics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastPlayedCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcategoryindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGameStatistics_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaxCategoryLength: GetMaxCategoryLength::<Identity, Impl, OFFSET>,
            GetMaxNameLength: GetMaxNameLength::<Identity, Impl, OFFSET>,
            GetMaxValueLength: GetMaxValueLength::<Identity, Impl, OFFSET>,
            GetMaxCategories: GetMaxCategories::<Identity, Impl, OFFSET>,
            GetMaxStatsPerCategory: GetMaxStatsPerCategory::<Identity, Impl, OFFSET>,
            SetCategoryTitle: SetCategoryTitle::<Identity, Impl, OFFSET>,
            GetCategoryTitle: GetCategoryTitle::<Identity, Impl, OFFSET>,
            GetStatistic: GetStatistic::<Identity, Impl, OFFSET>,
            SetStatistic: SetStatistic::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SetLastPlayedCategory: SetLastPlayedCategory::<Identity, Impl, OFFSET>,
            GetLastPlayedCategory: GetLastPlayedCategory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGameStatisticsMgr_Impl: ::windows_core::BaseImpl {
    fn GetGameStatistics(this: &Self::This, gdfbinarypath: &::windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows_core::Result<()>;
    fn RemoveGameStatistics(this: &Self::This, gdfbinarypath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGameStatisticsMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameStatisticsMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGameStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGameStatistics(this, ::core::mem::transmute(&gdfbinarypath), ::core::mem::transmute_copy(&opentype), ::core::mem::transmute_copy(&popenresult), ::core::mem::transmute_copy(&ppistats)).into())
        }
        unsafe extern "system" fn RemoveGameStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameStatisticsMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveGameStatistics(this, ::core::mem::transmute(&gdfbinarypath)).into())
        }
        IGameStatisticsMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGameStatistics: GetGameStatistics::<Identity, Impl, OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthManager_Impl: ::windows_core::BaseImpl {
    fn SetGamerAccount(this: &Self::This, msaaccountid: &::windows_core::PCWSTR, xuid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetGamerAccount(this: &Self::This, msaaccountid: *mut ::windows_core::PWSTR, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetAppViewInitialized(this: &Self::This, appsid: &::windows_core::PCWSTR, msaaccountid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetEnvironment(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSandbox(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetTokenAndSignatureWithTokenResult(this: &Self::This, msaaccountid: &::windows_core::PCWSTR, appsid: &::windows_core::PCWSTR, msatarget: &::windows_core::PCWSTR, msapolicy: &::windows_core::PCWSTR, httpmethod: &::windows_core::PCWSTR, uri: &::windows_core::PCWSTR, headers: &::windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL) -> ::windows_core::Result<IXblIdpAuthTokenResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXblIdpAuthManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXblIdpAuthManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGamerAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msaaccountid: ::windows_core::PCWSTR, xuid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGamerAccount(this, ::core::mem::transmute(&msaaccountid), ::core::mem::transmute(&xuid)).into())
        }
        unsafe extern "system" fn GetGamerAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows_core::PWSTR, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGamerAccount(this, ::core::mem::transmute_copy(&msaaccountid), ::core::mem::transmute_copy(&xuid)).into())
        }
        unsafe extern "system" fn SetAppViewInitialized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appsid: ::windows_core::PCWSTR, msaaccountid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppViewInitialized(this, ::core::mem::transmute(&appsid), ::core::mem::transmute(&msaaccountid)).into())
        }
        unsafe extern "system" fn GetEnvironment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnvironment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(environment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSandbox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSandbox(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sandbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msaaccountid: ::windows_core::PCWSTR, appsid: ::windows_core::PCWSTR, msatarget: ::windows_core::PCWSTR, msapolicy: ::windows_core::PCWSTR, httpmethod: ::windows_core::PCWSTR, uri: ::windows_core::PCWSTR, headers: ::windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTokenAndSignatureWithTokenResult(this, ::core::mem::transmute(&msaaccountid), ::core::mem::transmute(&appsid), ::core::mem::transmute(&msatarget), ::core::mem::transmute(&msapolicy), ::core::mem::transmute(&httpmethod), ::core::mem::transmute(&uri), ::core::mem::transmute(&headers), ::core::mem::transmute_copy(&body), ::core::mem::transmute_copy(&bodysize), ::core::mem::transmute_copy(&forcerefresh)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXblIdpAuthManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGamerAccount: SetGamerAccount::<Identity, Impl, OFFSET>,
            GetGamerAccount: GetGamerAccount::<Identity, Impl, OFFSET>,
            SetAppViewInitialized: SetAppViewInitialized::<Identity, Impl, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, Impl, OFFSET>,
            GetSandbox: GetSandbox::<Identity, Impl, OFFSET>,
            GetTokenAndSignatureWithTokenResult: GetTokenAndSignatureWithTokenResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXblIdpAuthManager2_Impl: ::windows_core::BaseImpl {
    fn GetUserlessTokenAndSignatureWithTokenResult(this: &Self::This, appsid: &::windows_core::PCWSTR, msatarget: &::windows_core::PCWSTR, msapolicy: &::windows_core::PCWSTR, httpmethod: &::windows_core::PCWSTR, uri: &::windows_core::PCWSTR, headers: &::windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL) -> ::windows_core::Result<IXblIdpAuthTokenResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXblIdpAuthManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXblIdpAuthManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUserlessTokenAndSignatureWithTokenResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appsid: ::windows_core::PCWSTR, msatarget: ::windows_core::PCWSTR, msapolicy: ::windows_core::PCWSTR, httpmethod: ::windows_core::PCWSTR, uri: ::windows_core::PCWSTR, headers: ::windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserlessTokenAndSignatureWithTokenResult(this, ::core::mem::transmute(&appsid), ::core::mem::transmute(&msatarget), ::core::mem::transmute(&msapolicy), ::core::mem::transmute(&httpmethod), ::core::mem::transmute(&uri), ::core::mem::transmute(&headers), ::core::mem::transmute_copy(&body), ::core::mem::transmute_copy(&bodysize), ::core::mem::transmute_copy(&forcerefresh)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXblIdpAuthManager2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUserlessTokenAndSignatureWithTokenResult: GetUserlessTokenAndSignatureWithTokenResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXblIdpAuthTokenResult_Impl: ::windows_core::BaseImpl {
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<XBL_IDP_AUTH_TOKEN_STATUS>;
    fn GetErrorCode(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetToken(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSignature(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSandbox(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetEnvironment(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMsaAccountId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetXuid(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetGamertag(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetAgeGroup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPrivileges(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMsaTarget(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMsaPolicy(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMsaAppId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetRedirect(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMessage(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetHelpId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetEnforcementBans(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetRestrictions(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetTitleRestrictions(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IXblIdpAuthTokenResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXblIdpAuthTokenResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetToken(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signature: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSandbox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSandbox(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sandbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnvironment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnvironment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(environment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMsaAccountId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMsaAccountId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msaaccountid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xuid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGamertag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gamertag: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGamertag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gamertag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAgeGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, agegroup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAgeGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(agegroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrivileges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, privileges: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrivileges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(privileges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMsaTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msatarget: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMsaTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msatarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMsaPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msapolicy: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMsaPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msapolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMsaAppId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msaappid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMsaAppId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(msaappid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRedirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, redirect: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRedirect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(redirect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHelpId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, helpid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHelpId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(helpid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnforcementBans<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enforcementbans: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnforcementBans(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enforcementbans, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRestrictions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restrictions: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTitleRestrictions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, titlerestrictions: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTitleRestrictions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(titlerestrictions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXblIdpAuthTokenResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, Impl, OFFSET>,
            GetToken: GetToken::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetSandbox: GetSandbox::<Identity, Impl, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, Impl, OFFSET>,
            GetMsaAccountId: GetMsaAccountId::<Identity, Impl, OFFSET>,
            GetXuid: GetXuid::<Identity, Impl, OFFSET>,
            GetGamertag: GetGamertag::<Identity, Impl, OFFSET>,
            GetAgeGroup: GetAgeGroup::<Identity, Impl, OFFSET>,
            GetPrivileges: GetPrivileges::<Identity, Impl, OFFSET>,
            GetMsaTarget: GetMsaTarget::<Identity, Impl, OFFSET>,
            GetMsaPolicy: GetMsaPolicy::<Identity, Impl, OFFSET>,
            GetMsaAppId: GetMsaAppId::<Identity, Impl, OFFSET>,
            GetRedirect: GetRedirect::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetHelpId: GetHelpId::<Identity, Impl, OFFSET>,
            GetEnforcementBans: GetEnforcementBans::<Identity, Impl, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, Impl, OFFSET>,
            GetTitleRestrictions: GetTitleRestrictions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXblIdpAuthTokenResult2_Impl: ::windows_core::BaseImpl {
    fn GetModernGamertag(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetModernGamertagSuffix(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetUniqueModernGamertag(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IXblIdpAuthTokenResult2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXblIdpAuthTokenResult2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetModernGamertag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetModernGamertag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetModernGamertagSuffix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXblIdpAuthTokenResult2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUniqueModernGamertag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXblIdpAuthTokenResult2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetModernGamertag: GetModernGamertag::<Identity, Impl, OFFSET>,
            GetModernGamertagSuffix: GetModernGamertagSuffix::<Identity, Impl, OFFSET>,
            GetUniqueModernGamertag: GetUniqueModernGamertag::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
