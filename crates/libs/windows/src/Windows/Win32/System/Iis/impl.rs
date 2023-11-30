#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpAuthenticationProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_AuthenticateUser(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_AuthenticateUser(this: &Self::This, ppszcanonicalusername: *mut ::windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for AsyncIFtpAuthenticationProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpAuthenticationProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIFtpAuthenticationProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_AuthenticateUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpAuthenticationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_AuthenticateUser(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword)).into())
        }
        unsafe extern "system" fn Finish_AuthenticateUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpAuthenticationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcanonicalusername: *mut ::windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_AuthenticateUser(this, ::core::mem::transmute_copy(&ppszcanonicalusername), ::core::mem::transmute_copy(&pfauthenticated)).into())
        }
        AsyncIFtpAuthenticationProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_AuthenticateUser: Begin_AuthenticateUser::<Identity, Impl, OFFSET>,
            Finish_AuthenticateUser: Finish_AuthenticateUser::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIFtpAuthorizationProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_GetUserAccessPermission(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszvirtualpath: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_GetUserAccessPermission(this: &Self::This) -> ::windows_core::Result<FTP_ACCESS>;
}
impl ::windows_core::Iids for AsyncIFtpAuthorizationProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpAuthorizationProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIFtpAuthorizationProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_GetUserAccessPermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpAuthorizationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszvirtualpath: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetUserAccessPermission(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszvirtualpath), ::core::mem::transmute(&pszusername)).into())
        }
        unsafe extern "system" fn Finish_GetUserAccessPermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpAuthorizationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftpaccess: *mut FTP_ACCESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_GetUserAccessPermission(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftpaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        AsyncIFtpAuthorizationProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_GetUserAccessPermission: Begin_GetUserAccessPermission::<Identity, Impl, OFFSET>,
            Finish_GetUserAccessPermission: Finish_GetUserAccessPermission::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIFtpHomeDirectoryProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_GetUserHomeDirectoryData(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_GetUserHomeDirectoryData(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for AsyncIFtpHomeDirectoryProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpHomeDirectoryProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIFtpHomeDirectoryProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_GetUserHomeDirectoryData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpHomeDirectoryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_GetUserHomeDirectoryData(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszusername)).into())
        }
        unsafe extern "system" fn Finish_GetUserHomeDirectoryData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpHomeDirectoryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszhomedirectorydata: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_GetUserHomeDirectoryData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszhomedirectorydata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        AsyncIFtpHomeDirectoryProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_GetUserHomeDirectoryData: Begin_GetUserHomeDirectoryData::<Identity, Impl, OFFSET>,
            Finish_GetUserHomeDirectoryData: Finish_GetUserHomeDirectoryData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIFtpLogProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_Log(this: &Self::This, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows_core::Result<()>;
    fn Finish_Log(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIFtpLogProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpLogProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIFtpLogProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_Log<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpLogProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_Log(this, ::core::mem::transmute_copy(&ploggingparameters)).into())
        }
        unsafe extern "system" fn Finish_Log<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpLogProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_Log(this).into())
        }
        AsyncIFtpLogProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_Log: Begin_Log::<Identity, Impl, OFFSET>,
            Finish_Log: Finish_Log::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpPostprocessProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_HandlePostprocess(this: &Self::This, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows_core::Result<()>;
    fn Finish_HandlePostprocess(this: &Self::This) -> ::windows_core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for AsyncIFtpPostprocessProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpPostprocessProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIFtpPostprocessProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_HandlePostprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpPostprocessProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_HandlePostprocess(this, ::core::mem::transmute_copy(&ppostprocessparameters)).into())
        }
        unsafe extern "system" fn Finish_HandlePostprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpPostprocessProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_HandlePostprocess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftpprocessstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        AsyncIFtpPostprocessProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_HandlePostprocess: Begin_HandlePostprocess::<Identity, Impl, OFFSET>,
            Finish_HandlePostprocess: Finish_HandlePostprocess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpPreprocessProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_HandlePreprocess(this: &Self::This, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows_core::Result<()>;
    fn Finish_HandlePreprocess(this: &Self::This) -> ::windows_core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for AsyncIFtpPreprocessProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpPreprocessProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIFtpPreprocessProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_HandlePreprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpPreprocessProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_HandlePreprocess(this, ::core::mem::transmute_copy(&ppreprocessparameters)).into())
        }
        unsafe extern "system" fn Finish_HandlePreprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpPreprocessProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_HandlePreprocess(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftpprocessstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        AsyncIFtpPreprocessProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_HandlePreprocess: Begin_HandlePreprocess::<Identity, Impl, OFFSET>,
            Finish_HandlePreprocess: Finish_HandlePreprocess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait AsyncIFtpRoleProvider_Impl: ::windows_core::BaseImpl {
    fn Begin_IsUserInRole(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszrole: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Finish_IsUserInRole(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for AsyncIFtpRoleProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpRoleProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIFtpRoleProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_IsUserInRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpRoleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszrole: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_IsUserInRole(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszrole)).into())
        }
        unsafe extern "system" fn Finish_IsUserInRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIFtpRoleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Finish_IsUserInRole(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        AsyncIFtpRoleProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_IsUserInRole: Begin_IsUserInRole::<Identity, Impl, OFFSET>,
            Finish_IsUserInRole: Finish_IsUserInRole::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait AsyncIMSAdminBaseSinkW_Impl: ::windows_core::BaseImpl {
    fn Begin_SinkNotify(this: &Self::This, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows_core::Result<()>;
    fn Finish_SinkNotify(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_ShutdownNotify(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_ShutdownNotify(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIMSAdminBaseSinkW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIMSAdminBaseSinkW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_SinkNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_SinkNotify(this, ::core::mem::transmute_copy(&dwmdnumelements), ::core::mem::transmute_copy(&pcochangelist)).into())
        }
        unsafe extern "system" fn Finish_SinkNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_SinkNotify(this).into())
        }
        unsafe extern "system" fn Begin_ShutdownNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_ShutdownNotify(this).into())
        }
        unsafe extern "system" fn Finish_ShutdownNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIMSAdminBaseSinkW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_ShutdownNotify(this).into())
        }
        AsyncIMSAdminBaseSinkW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_SinkNotify: Begin_SinkNotify::<Identity, Impl, OFFSET>,
            Finish_SinkNotify: Finish_SinkNotify::<Identity, Impl, OFFSET>,
            Begin_ShutdownNotify: Begin_ShutdownNotify::<Identity, Impl, OFFSET>,
            Finish_ShutdownNotify: Finish_ShutdownNotify::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IADMEXT_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumDcomCLSIDs(this: &Self::This, pclsiddcom: *mut ::windows_core::GUID, dwenumindex: u32) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IADMEXT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADMEXT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADMEXT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADMEXT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this).into())
        }
        unsafe extern "system" fn EnumDcomCLSIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADMEXT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsiddcom: *mut ::windows_core::GUID, dwenumindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumDcomCLSIDs(this, ::core::mem::transmute_copy(&pclsiddcom), ::core::mem::transmute_copy(&dwenumindex)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADMEXT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this).into())
        }
        IADMEXT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            EnumDcomCLSIDs: EnumDcomCLSIDs::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpAuthenticationProvider_Impl: ::windows_core::BaseImpl {
    fn AuthenticateUser(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszpassword: &::windows_core::PCWSTR, ppszcanonicalusername: *mut ::windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFtpAuthenticationProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpAuthenticationProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpAuthenticationProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AuthenticateUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpAuthenticationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR, ppszcanonicalusername: *mut ::windows_core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AuthenticateUser(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword), ::core::mem::transmute_copy(&ppszcanonicalusername), ::core::mem::transmute_copy(&pfauthenticated)).into())
        }
        IFtpAuthenticationProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AuthenticateUser: AuthenticateUser::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFtpAuthorizationProvider_Impl: ::windows_core::BaseImpl {
    fn GetUserAccessPermission(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszvirtualpath: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR) -> ::windows_core::Result<FTP_ACCESS>;
}
impl ::windows_core::Iids for IFtpAuthorizationProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpAuthorizationProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpAuthorizationProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUserAccessPermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpAuthorizationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszvirtualpath: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pftpaccess: *mut FTP_ACCESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserAccessPermission(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszvirtualpath), ::core::mem::transmute(&pszusername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftpaccess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFtpAuthorizationProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUserAccessPermission: GetUserAccessPermission::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFtpHomeDirectoryProvider_Impl: ::windows_core::BaseImpl {
    fn GetUserHomeDirectoryData(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IFtpHomeDirectoryProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpHomeDirectoryProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpHomeDirectoryProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUserHomeDirectoryData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpHomeDirectoryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, ppszhomedirectorydata: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserHomeDirectoryData(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszusername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszhomedirectorydata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFtpHomeDirectoryProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUserHomeDirectoryData: GetUserHomeDirectoryData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFtpLogProvider_Impl: ::windows_core::BaseImpl {
    fn Log(this: &Self::This, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFtpLogProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpLogProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpLogProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Log<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpLogProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Log(this, ::core::mem::transmute_copy(&ploggingparameters)).into())
        }
        IFtpLogProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Log: Log::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpPostprocessProvider_Impl: ::windows_core::BaseImpl {
    fn HandlePostprocess(this: &Self::This, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows_core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFtpPostprocessProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpPostprocessProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpPostprocessProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandlePostprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpPostprocessProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HandlePostprocess(this, ::core::mem::transmute_copy(&ppostprocessparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftpprocessstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFtpPostprocessProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandlePostprocess: HandlePostprocess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpPreprocessProvider_Impl: ::windows_core::BaseImpl {
    fn HandlePreprocess(this: &Self::This, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows_core::Result<FTP_PROCESS_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFtpPreprocessProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpPreprocessProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpPreprocessProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandlePreprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpPreprocessProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HandlePreprocess(this, ::core::mem::transmute_copy(&ppreprocessparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftpprocessstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFtpPreprocessProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandlePreprocess: HandlePreprocess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IFtpProviderConstruct_Impl: ::windows_core::BaseImpl {
    fn Construct(this: &Self::This, configurationentries: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IFtpProviderConstruct {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpProviderConstruct_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpProviderConstruct {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Construct<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpProviderConstruct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configurationentries: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Construct(this, ::core::mem::transmute_copy(&configurationentries)).into())
        }
        IFtpProviderConstruct_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Construct: Construct::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFtpRoleProvider_Impl: ::windows_core::BaseImpl {
    fn IsUserInRole(this: &Self::This, pszsessionid: &::windows_core::PCWSTR, pszsitename: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszrole: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFtpRoleProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpRoleProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFtpRoleProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsUserInRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFtpRoleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsessionid: ::windows_core::PCWSTR, pszsitename: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszrole: ::windows_core::PCWSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUserInRole(this, ::core::mem::transmute(&pszsessionid), ::core::mem::transmute(&pszsitename), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFtpRoleProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsUserInRole: IsUserInRole::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBase2W_Impl: ::windows_core::BaseImpl + IMSAdminBaseW_Impl {
    fn BackupWithPasswd(this: &Self::This, pszmdbackuplocation: &::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RestoreWithPasswd(this: &Self::This, pszmdbackuplocation: &::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Export(this: &Self::This, pszpasswd: &::windows_core::PCWSTR, pszfilename: &::windows_core::PCWSTR, pszsourcepath: &::windows_core::PCWSTR, dwmdflags: u32) -> ::windows_core::Result<()>;
    fn Import(this: &Self::This, pszpasswd: &::windows_core::PCWSTR, pszfilename: &::windows_core::PCWSTR, pszsourcepath: &::windows_core::PCWSTR, pszdestpath: &::windows_core::PCWSTR, dwmdflags: u32) -> ::windows_core::Result<()>;
    fn RestoreHistory(this: &Self::This, pszmdhistorylocation: &::windows_core::PCWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows_core::Result<()>;
    fn EnumHistory(this: &Self::This, pszmdhistorylocation: &::windows_core::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMSAdminBase2W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSAdminBaseW);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase2W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSAdminBase2W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BackupWithPasswd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackupWithPasswd(this, ::core::mem::transmute(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags), ::core::mem::transmute(&pszpasswd)).into())
        }
        unsafe extern "system" fn RestoreWithPasswd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreWithPasswd(this, ::core::mem::transmute(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags), ::core::mem::transmute(&pszpasswd)).into())
        }
        unsafe extern "system" fn Export<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpasswd: ::windows_core::PCWSTR, pszfilename: ::windows_core::PCWSTR, pszsourcepath: ::windows_core::PCWSTR, dwmdflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Export(this, ::core::mem::transmute(&pszpasswd), ::core::mem::transmute(&pszfilename), ::core::mem::transmute(&pszsourcepath), ::core::mem::transmute_copy(&dwmdflags)).into())
        }
        unsafe extern "system" fn Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpasswd: ::windows_core::PCWSTR, pszfilename: ::windows_core::PCWSTR, pszsourcepath: ::windows_core::PCWSTR, pszdestpath: ::windows_core::PCWSTR, dwmdflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Import(this, ::core::mem::transmute(&pszpasswd), ::core::mem::transmute(&pszfilename), ::core::mem::transmute(&pszsourcepath), ::core::mem::transmute(&pszdestpath), ::core::mem::transmute_copy(&dwmdflags)).into())
        }
        unsafe extern "system" fn RestoreHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: ::windows_core::PCWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreHistory(this, ::core::mem::transmute(&pszmdhistorylocation), ::core::mem::transmute_copy(&dwmdmajorversion), ::core::mem::transmute_copy(&dwmdminorversion), ::core::mem::transmute_copy(&dwmdflags)).into())
        }
        unsafe extern "system" fn EnumHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase2W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdhistorylocation: ::windows_core::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumHistory(this, ::core::mem::transmute(&pszmdhistorylocation), ::core::mem::transmute_copy(&pdwmdmajorversion), ::core::mem::transmute_copy(&pdwmdminorversion), ::core::mem::transmute_copy(&pftmdhistorytime), ::core::mem::transmute_copy(&dwmdenumindex)).into())
        }
        IMSAdminBase2W_Vtbl {
            base__: <IMSAdminBaseW as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BackupWithPasswd: BackupWithPasswd::<Identity, Impl, OFFSET>,
            RestoreWithPasswd: RestoreWithPasswd::<Identity, Impl, OFFSET>,
            Export: Export::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            RestoreHistory: RestoreHistory::<Identity, Impl, OFFSET>,
            EnumHistory: EnumHistory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBase3W_Impl: ::windows_core::BaseImpl + IMSAdminBase2W_Impl {
    fn GetChildPaths(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, cchmdbuffersize: u32, pszbuffer: &::windows_core::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMSAdminBase3W {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSAdminBase2W);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase3W_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSAdminBase3W {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChildPaths<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBase3W_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, cchmdbuffersize: u32, pszbuffer: ::windows_core::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChildPaths(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&cchmdbuffersize), ::core::mem::transmute(&pszbuffer), ::core::mem::transmute_copy(&pcchmdrequiredbuffersize)).into())
        }
        IMSAdminBase3W_Vtbl { base__: <IMSAdminBase2W as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetChildPaths: GetChildPaths::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMSAdminBaseSinkW_Impl: ::windows_core::BaseImpl {
    fn SinkNotify(this: &Self::This, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows_core::Result<()>;
    fn ShutdownNotify(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMSAdminBaseSinkW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseSinkW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSAdminBaseSinkW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SinkNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseSinkW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SinkNotify(this, ::core::mem::transmute_copy(&dwmdnumelements), ::core::mem::transmute_copy(&pcochangelist)).into())
        }
        unsafe extern "system" fn ShutdownNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseSinkW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownNotify(this).into())
        }
        IMSAdminBaseSinkW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SinkNotify: SinkNotify::<Identity, Impl, OFFSET>,
            ShutdownNotify: ShutdownNotify::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMSAdminBaseW_Impl: ::windows_core::BaseImpl {
    fn AddKey(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeleteKey(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeleteChildKeys(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumKeys(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, pszmdname: ::windows_core::PWSTR, dwmdenumobjectindex: u32) -> ::windows_core::Result<()>;
    fn CopyKey(this: &Self::This, hmdsourcehandle: u32, pszmdsourcepath: &::windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: &::windows_core::PCWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RenameKey(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, pszmdnewname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetData(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD) -> ::windows_core::Result<()>;
    fn GetData(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows_core::Result<()>;
    fn DeleteData(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows_core::Result<()>;
    fn EnumData(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows_core::Result<()>;
    fn GetAllData(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn DeleteAllData(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, dwmdusertype: u32, dwmddatatype: u32) -> ::windows_core::Result<()>;
    fn CopyData(this: &Self::This, hmdsourcehandle: u32, pszmdsourcepath: &::windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: &::windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDataPaths(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: ::windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn OpenKey(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows_core::Result<u32>;
    fn CloseKey(this: &Self::This, hmdhandle: u32) -> ::windows_core::Result<()>;
    fn ChangePermissions(this: &Self::This, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows_core::Result<()>;
    fn SaveData(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetHandleInfo(this: &Self::This, hmdhandle: u32) -> ::windows_core::Result<METADATA_HANDLE_INFO>;
    fn GetSystemChangeNumber(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDataSetNumber(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn SetLastChangeTime(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLastChangeTime(this: &Self::This, hmdhandle: u32, pszmdpath: &::windows_core::PCWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn KeyExchangePhase1(this: &Self::This) -> ::windows_core::Result<()>;
    fn KeyExchangePhase2(this: &Self::This) -> ::windows_core::Result<()>;
    fn Backup(this: &Self::This, pszmdbackuplocation: &::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This, pszmdbackuplocation: &::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows_core::Result<()>;
    fn EnumBackups(this: &Self::This, pszmdbackuplocation: &::windows_core::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows_core::Result<()>;
    fn DeleteBackup(this: &Self::This, pszmdbackuplocation: &::windows_core::PCWSTR, dwmdversion: u32) -> ::windows_core::Result<()>;
    fn UnmarshalInterface(this: &Self::This) -> ::windows_core::Result<IMSAdminBaseW>;
    fn GetServerGuid(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMSAdminBaseW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSAdminBaseW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddKey(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath)).into())
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteKey(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath)).into())
        }
        unsafe extern "system" fn DeleteChildKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteChildKeys(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath)).into())
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pszmdname: ::windows_core::PWSTR, dwmdenumobjectindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumKeys(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&pszmdname), ::core::mem::transmute_copy(&dwmdenumobjectindex)).into())
        }
        unsafe extern "system" fn CopyKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: ::windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: ::windows_core::PCWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyKey(this, ::core::mem::transmute_copy(&hmdsourcehandle), ::core::mem::transmute(&pszmdsourcepath), ::core::mem::transmute_copy(&hmddesthandle), ::core::mem::transmute(&pszmddestpath), ::core::mem::transmute_copy(&bmdoverwriteflag), ::core::mem::transmute_copy(&bmdcopyflag)).into())
        }
        unsafe extern "system" fn RenameKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pszmdnewname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameKey(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute(&pszmdnewname)).into())
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata)).into())
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata), ::core::mem::transmute_copy(&pdwmdrequireddatalen)).into())
        }
        unsafe extern "system" fn DeleteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteData(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&dwmdidentifier), ::core::mem::transmute_copy(&dwmddatatype)).into())
        }
        unsafe extern "system" fn EnumData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumData(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&pmdrmddata), ::core::mem::transmute_copy(&dwmdenumdataindex), ::core::mem::transmute_copy(&pdwmdrequireddatalen)).into())
        }
        unsafe extern "system" fn GetAllData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetAllData(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&dwmdattributes), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&pdwmdnumdataentries), ::core::mem::transmute_copy(&pdwmddatasetnumber), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pbmdbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into()
            })
        }
        unsafe extern "system" fn DeleteAllData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, dwmdusertype: u32, dwmddatatype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAllData(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype)).into())
        }
        unsafe extern "system" fn CopyData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: ::windows_core::PCWSTR, hmddesthandle: u32, pszmddestpath: ::windows_core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyData(this, ::core::mem::transmute_copy(&hmdsourcehandle), ::core::mem::transmute(&pszmdsourcepath), ::core::mem::transmute_copy(&hmddesthandle), ::core::mem::transmute(&pszmddestpath), ::core::mem::transmute_copy(&dwmdattributes), ::core::mem::transmute_copy(&dwmdusertype), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&bmdcopyflag)).into())
        }
        unsafe extern "system" fn GetDataPaths<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: ::windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataPaths(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&dwmdidentifier), ::core::mem::transmute_copy(&dwmddatatype), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into())
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32, phmdnewhandle: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenKey(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&dwmdaccessrequested), ::core::mem::transmute_copy(&dwmdtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmdnewhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseKey(this, ::core::mem::transmute_copy(&hmdhandle)).into())
        }
        unsafe extern "system" fn ChangePermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangePermissions(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute_copy(&dwmdtimeout), ::core::mem::transmute_copy(&dwmdaccessrequested)).into())
        }
        unsafe extern "system" fn SaveData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveData(this).into())
        }
        unsafe extern "system" fn GetHandleInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pmdhiinfo: *mut METADATA_HANDLE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHandleInfo(this, ::core::mem::transmute_copy(&hmdhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmdhiinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSystemChangeNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsystemchangenumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemChangeNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsystemchangenumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataSetNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pdwmddatasetnumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataSetNumber(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmddatasetnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastChangeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastChangeTime(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&pftmdlastchangetime), ::core::mem::transmute_copy(&blocaltime)).into())
        }
        unsafe extern "system" fn GetLastChangeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows_core::PCWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastChangeTime(this, ::core::mem::transmute_copy(&hmdhandle), ::core::mem::transmute(&pszmdpath), ::core::mem::transmute_copy(&pftmdlastchangetime), ::core::mem::transmute_copy(&blocaltime)).into())
        }
        unsafe extern "system" fn KeyExchangePhase1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KeyExchangePhase1(this).into())
        }
        unsafe extern "system" fn KeyExchangePhase2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KeyExchangePhase2(this).into())
        }
        unsafe extern "system" fn Backup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Backup(this, ::core::mem::transmute(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags)).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows_core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this, ::core::mem::transmute(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion), ::core::mem::transmute_copy(&dwmdflags)).into())
        }
        unsafe extern "system" fn EnumBackups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows_core::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumBackups(this, ::core::mem::transmute(&pszmdbackuplocation), ::core::mem::transmute_copy(&pdwmdversion), ::core::mem::transmute_copy(&pftmdbackuptime), ::core::mem::transmute_copy(&dwmdenumindex)).into())
        }
        unsafe extern "system" fn DeleteBackup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows_core::PCWSTR, dwmdversion: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteBackup(this, ::core::mem::transmute(&pszmdbackuplocation), ::core::mem::transmute_copy(&dwmdversion)).into())
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piadmbwinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnmarshalInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piadmbwinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetServerGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSAdminBaseW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetServerGuid(this).into())
        }
        IMSAdminBaseW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddKey: AddKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteChildKeys: DeleteChildKeys::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            CopyKey: CopyKey::<Identity, Impl, OFFSET>,
            RenameKey: RenameKey::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            DeleteData: DeleteData::<Identity, Impl, OFFSET>,
            EnumData: EnumData::<Identity, Impl, OFFSET>,
            GetAllData: GetAllData::<Identity, Impl, OFFSET>,
            DeleteAllData: DeleteAllData::<Identity, Impl, OFFSET>,
            CopyData: CopyData::<Identity, Impl, OFFSET>,
            GetDataPaths: GetDataPaths::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CloseKey: CloseKey::<Identity, Impl, OFFSET>,
            ChangePermissions: ChangePermissions::<Identity, Impl, OFFSET>,
            SaveData: SaveData::<Identity, Impl, OFFSET>,
            GetHandleInfo: GetHandleInfo::<Identity, Impl, OFFSET>,
            GetSystemChangeNumber: GetSystemChangeNumber::<Identity, Impl, OFFSET>,
            GetDataSetNumber: GetDataSetNumber::<Identity, Impl, OFFSET>,
            SetLastChangeTime: SetLastChangeTime::<Identity, Impl, OFFSET>,
            GetLastChangeTime: GetLastChangeTime::<Identity, Impl, OFFSET>,
            KeyExchangePhase1: KeyExchangePhase1::<Identity, Impl, OFFSET>,
            KeyExchangePhase2: KeyExchangePhase2::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            EnumBackups: EnumBackups::<Identity, Impl, OFFSET>,
            DeleteBackup: DeleteBackup::<Identity, Impl, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, Impl, OFFSET>,
            GetServerGuid: GetServerGuid::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMSImpExpHelpW_Impl: ::windows_core::BaseImpl {
    fn EnumeratePathsInFile(this: &Self::This, pszfilename: &::windows_core::PCWSTR, pszkeytype: &::windows_core::PCWSTR, dwmdbuffersize: u32, pszbuffer: &::windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMSImpExpHelpW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSImpExpHelpW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSImpExpHelpW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumeratePathsInFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSImpExpHelpW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, pszkeytype: ::windows_core::PCWSTR, dwmdbuffersize: u32, pszbuffer: ::windows_core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumeratePathsInFile(this, ::core::mem::transmute(&pszfilename), ::core::mem::transmute(&pszkeytype), ::core::mem::transmute_copy(&dwmdbuffersize), ::core::mem::transmute(&pszbuffer), ::core::mem::transmute_copy(&pdwmdrequiredbuffersize)).into())
        }
        IMSImpExpHelpW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumeratePathsInFile: EnumeratePathsInFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
