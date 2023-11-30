#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCGamesSettings_Impl: ::windows_core::BaseImpl + IWPCSettings_Impl {
    fn IsBlocked(this: &Self::This, guidappid: &::windows_core::GUID) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWPCGamesSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWPCSettings);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCGamesSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWPCGamesSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsBlocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCGamesSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidappid: ::windows_core::GUID, pdwreasons: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBlocked(this, ::core::mem::transmute(&guidappid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwreasons, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWPCGamesSettings_Vtbl { base__: <IWPCSettings as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsBlocked: IsBlocked::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCProviderConfig_Impl: ::windows_core::BaseImpl {
    fn GetUserSummary(this: &Self::This, bstrsid: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Configure(this: &Self::This, hwnd: super::super::Foundation::HWND, bstrsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RequestOverride(this: &Self::This, hwnd: super::super::Foundation::HWND, bstrpath: &::windows_core::BSTR, dwflags: &WPCFLAG_RESTRICTION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWPCProviderConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWPCProviderConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUserSummary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrusersummary: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserSummary(this, ::core::mem::transmute(&bstrsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusersummary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Configure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Configure(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&bstrsid)).into())
        }
        unsafe extern "system" fn RequestOverride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestOverride(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&bstrpath), ::core::mem::transmute(&dwflags)).into())
        }
        IWPCProviderConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUserSummary: GetUserSummary::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            RequestOverride: RequestOverride::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWPCProviderState_Impl: ::windows_core::BaseImpl {
    fn Enable(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disable(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWPCProviderState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWPCProviderState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this).into())
        }
        unsafe extern "system" fn Disable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disable(this).into())
        }
        IWPCProviderState_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWPCProviderSupport_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IWPCProviderSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWPCProviderSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCProviderSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidprovider: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWPCProviderSupport_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetCurrent: GetCurrent::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCSettings_Impl: ::windows_core::BaseImpl {
    fn IsLoggingRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetLastSettingsChangeTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetRestrictions(this: &Self::This) -> ::windows_core::Result<WPCFLAG_RESTRICTION>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWPCSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWPCSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsLoggingRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrequired: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLoggingRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrequired, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastSettingsChangeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastSettingsChangeTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRestrictions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrestrictions: *mut WPCFLAG_RESTRICTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrestrictions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWPCSettings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsLoggingRequired: IsLoggingRequired::<Identity, Impl, OFFSET>,
            GetLastSettingsChangeTime: GetLastSettingsChangeTime::<Identity, Impl, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWPCWebSettings_Impl: ::windows_core::BaseImpl + IWPCSettings_Impl {
    fn GetSettings(this: &Self::This) -> ::windows_core::Result<WPCFLAG_WEB_SETTING>;
    fn RequestURLOverride(this: &Self::This, hwnd: super::super::Foundation::HWND, pcszurl: &::windows_core::PCWSTR, curls: u32, ppcszsuburls: *const ::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWPCWebSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWPCSettings);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCWebSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWPCWebSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCWebSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsettings: *mut WPCFLAG_WEB_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestURLOverride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWPCWebSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pcszurl: ::windows_core::PCWSTR, curls: u32, ppcszsuburls: *const ::windows_core::PCWSTR, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestURLOverride(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pcszurl), ::core::mem::transmute_copy(&curls), ::core::mem::transmute_copy(&ppcszsuburls)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfchanged, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWPCWebSettings_Vtbl {
            base__: <IWPCSettings as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSettings: GetSettings::<Identity, Impl, OFFSET>,
            RequestURLOverride: RequestURLOverride::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWindowsParentalControls_Impl: ::windows_core::BaseImpl + IWindowsParentalControlsCore_Impl {
    fn GetGamesSettings(this: &Self::This, pcszsid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWPCGamesSettings>;
}
impl ::windows_core::Iids for IWindowsParentalControls {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWindowsParentalControlsCore);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsParentalControls_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsParentalControls {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGamesSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsParentalControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszsid: ::windows_core::PCWSTR, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGamesSettings(this, ::core::mem::transmute(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsParentalControls_Vtbl {
            base__: <IWindowsParentalControlsCore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGamesSettings: GetGamesSettings::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWindowsParentalControlsCore_Impl: ::windows_core::BaseImpl {
    fn GetVisibility(this: &Self::This) -> ::windows_core::Result<WPCFLAG_VISIBILITY>;
    fn GetUserSettings(this: &Self::This, pcszsid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWPCSettings>;
    fn GetWebSettings(this: &Self::This, pcszsid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWPCWebSettings>;
    fn GetWebFilterInfo(this: &Self::This, pguidid: *mut ::windows_core::GUID, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWindowsParentalControlsCore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsParentalControlsCore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVisibility<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevisibility: *mut WPCFLAG_VISIBILITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisibility(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevisibility, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszsid: ::windows_core::PCWSTR, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserSettings(this, ::core::mem::transmute(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWebSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszsid: ::windows_core::PCWSTR, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWebSettings(this, ::core::mem::transmute(&pcszsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWebFilterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsParentalControlsCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows_core::GUID, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWebFilterInfo(this, ::core::mem::transmute_copy(&pguidid), ::core::mem::transmute_copy(&ppszname)).into())
        }
        IWindowsParentalControlsCore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVisibility: GetVisibility::<Identity, Impl, OFFSET>,
            GetUserSettings: GetUserSettings::<Identity, Impl, OFFSET>,
            GetWebSettings: GetWebSettings::<Identity, Impl, OFFSET>,
            GetWebFilterInfo: GetWebFilterInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
