#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAccountingProviderConfig_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pszmachinename: &::windows_core::PCWSTR) -> ::windows_core::Result<usize>;
    fn Uninitialize(this: &Self::This, uconnectionparam: usize) -> ::windows_core::Result<()>;
    fn Configure(this: &Self::This, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows_core::Result<()>;
    fn Activate(this: &Self::This, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAccountingProviderConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccountingProviderConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows_core::PCWSTR, puconnectionparam: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Initialize(this, ::core::mem::transmute(&pszmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puconnectionparam, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this, ::core::mem::transmute_copy(&uconnectionparam)).into())
        }
        unsafe extern "system" fn Configure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Configure(this, ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into())
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this, ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into())
        }
        IAccountingProviderConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticationProviderConfig_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pszmachinename: &::windows_core::PCWSTR) -> ::windows_core::Result<usize>;
    fn Uninitialize(this: &Self::This, uconnectionparam: usize) -> ::windows_core::Result<()>;
    fn Configure(this: &Self::This, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows_core::Result<()>;
    fn Activate(this: &Self::This, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAuthenticationProviderConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAuthenticationProviderConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows_core::PCWSTR, puconnectionparam: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Initialize(this, ::core::mem::transmute(&pszmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puconnectionparam, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this, ::core::mem::transmute_copy(&uconnectionparam)).into())
        }
        unsafe extern "system" fn Configure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Configure(this, ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into())
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this, ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into())
        }
        IAuthenticationProviderConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pszmachinename: &::windows_core::PCWSTR, dweaptypeid: u32) -> ::windows_core::Result<usize>;
    fn Uninitialize(this: &Self::This, dweaptypeid: u32, uconnectionparam: usize) -> ::windows_core::Result<()>;
    fn ServerInvokeConfigUI(this: &Self::This, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows_core::Result<()>;
    fn RouterInvokeConfigUI(this: &Self::This, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows_core::Result<()>;
    fn RouterInvokeCredentialsUI(this: &Self::This, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEAPProviderConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEAPProviderConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows_core::PCWSTR, dweaptypeid: u32, puconnectionparam: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Initialize(this, ::core::mem::transmute(&pszmachinename), ::core::mem::transmute_copy(&dweaptypeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puconnectionparam, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this, ::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam)).into())
        }
        unsafe extern "system" fn ServerInvokeConfigUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ServerInvokeConfigUI(this, ::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into())
        }
        unsafe extern "system" fn RouterInvokeConfigUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RouterInvokeConfigUI(this, ::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pconnectiondatain), ::core::mem::transmute_copy(&dwsizeofconnectiondatain), ::core::mem::transmute_copy(&ppconnectiondataout), ::core::mem::transmute_copy(&pdwsizeofconnectiondataout)).into())
        }
        unsafe extern "system" fn RouterInvokeCredentialsUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::RouterInvokeCredentialsUI(this, ::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pconnectiondatain), ::core::mem::transmute_copy(&dwsizeofconnectiondatain), ::core::mem::transmute_copy(&puserdatain), ::core::mem::transmute_copy(&dwsizeofuserdatain), ::core::mem::transmute_copy(&ppuserdataout), ::core::mem::transmute_copy(&pdwsizeofuserdataout)).into()
            })
        }
        IEAPProviderConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            ServerInvokeConfigUI: ServerInvokeConfigUI::<Identity, Impl, OFFSET>,
            RouterInvokeConfigUI: RouterInvokeConfigUI::<Identity, Impl, OFFSET>,
            RouterInvokeCredentialsUI: RouterInvokeCredentialsUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig2_Impl: ::windows_core::BaseImpl + IEAPProviderConfig_Impl {
    fn ServerInvokeConfigUI2(this: &Self::This, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows_core::Result<()>;
    fn GetGlobalConfig(this: &Self::This, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEAPProviderConfig2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IEAPProviderConfig);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEAPProviderConfig2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServerInvokeConfigUI2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ServerInvokeConfigUI2(this, ::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pconfigdatain), ::core::mem::transmute_copy(&dwsizeofconfigdatain), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout)).into())
        }
        unsafe extern "system" fn GetGlobalConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlobalConfig(this, ::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout)).into())
        }
        IEAPProviderConfig2_Vtbl {
            base__: <IEAPProviderConfig as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServerInvokeConfigUI2: ServerInvokeConfigUI2::<Identity, Impl, OFFSET>,
            GetGlobalConfig: GetGlobalConfig::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig3_Impl: ::windows_core::BaseImpl + IEAPProviderConfig2_Impl {
    fn ServerInvokeCertificateConfigUI(this: &Self::This, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEAPProviderConfig3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IEAPProviderConfig2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEAPProviderConfig3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServerInvokeCertificateConfigUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEAPProviderConfig3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ServerInvokeCertificateConfigUI(this, ::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pconfigdatain), ::core::mem::transmute_copy(&dwsizeofconfigdatain), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout), ::core::mem::transmute_copy(&ureserved)).into())
        }
        IEAPProviderConfig3_Vtbl {
            base__: <IEAPProviderConfig2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServerInvokeCertificateConfigUI: ServerInvokeCertificateConfigUI::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRouterProtocolConfig_Impl: ::windows_core::BaseImpl {
    fn AddProtocol(this: &Self::This, pszmachinename: &::windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: ::core::option::Option<&::windows_core::IUnknown>, ureserved1: usize) -> ::windows_core::Result<()>;
    fn RemoveProtocol(this: &Self::This, pszmachinename: &::windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: ::core::option::Option<&::windows_core::IUnknown>, ureserved1: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRouterProtocolConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRouterProtocolConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRouterProtocolConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRouterProtocolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddProtocol(this, ::core::mem::transmute(&pszmachinename), ::core::mem::transmute_copy(&dwtransportid), ::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&prouter), ::core::mem::transmute_copy(&ureserved1)).into())
        }
        unsafe extern "system" fn RemoveProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRouterProtocolConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows_core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveProtocol(this, ::core::mem::transmute(&pszmachinename), ::core::mem::transmute_copy(&dwtransportid), ::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&prouter), ::core::mem::transmute_copy(&ureserved1)).into())
        }
        IRouterProtocolConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddProtocol: AddProtocol::<Identity, Impl, OFFSET>,
            RemoveProtocol: RemoveProtocol::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
