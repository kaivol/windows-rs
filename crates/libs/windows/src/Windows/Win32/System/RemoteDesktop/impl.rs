#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsTSUserEx_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn TerminalServicesProfilePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTerminalServicesProfilePath(this: &Self::This, pnewval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TerminalServicesHomeDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTerminalServicesHomeDirectory(this: &Self::This, pnewval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TerminalServicesHomeDrive(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTerminalServicesHomeDrive(this: &Self::This, pnewval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AllowLogon(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAllowLogon(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn EnableRemoteControl(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEnableRemoteControl(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn MaxDisconnectionTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxDisconnectionTime(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn MaxConnectionTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxConnectionTime(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn MaxIdleTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxIdleTime(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn ReconnectionAction(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetReconnectionAction(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn BrokenConnectionAction(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBrokenConnectionAction(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn ConnectClientDrivesAtLogon(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetConnectClientDrivesAtLogon(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn ConnectClientPrintersAtLogon(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetConnectClientPrintersAtLogon(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn DefaultToMainPrinter(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDefaultToMainPrinter(this: &Self::This, newval: i32) -> ::windows_core::Result<()>;
    fn TerminalServicesWorkDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTerminalServicesWorkDirectory(this: &Self::This, pnewval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TerminalServicesInitialProgram(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTerminalServicesInitialProgram(this: &Self::This, pnewval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsTSUserEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsTSUserEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TerminalServicesProfilePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalServicesProfilePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTerminalServicesProfilePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTerminalServicesProfilePath(this, ::core::mem::transmute(&pnewval)).into())
        }
        unsafe extern "system" fn TerminalServicesHomeDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalServicesHomeDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTerminalServicesHomeDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTerminalServicesHomeDirectory(this, ::core::mem::transmute(&pnewval)).into())
        }
        unsafe extern "system" fn TerminalServicesHomeDrive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalServicesHomeDrive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTerminalServicesHomeDrive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTerminalServicesHomeDrive(this, ::core::mem::transmute(&pnewval)).into())
        }
        unsafe extern "system" fn AllowLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowLogon(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowLogon(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn EnableRemoteControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnableRemoteControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableRemoteControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableRemoteControl(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn MaxDisconnectionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxDisconnectionTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxDisconnectionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxDisconnectionTime(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn MaxConnectionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxConnectionTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxConnectionTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxConnectionTime(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn MaxIdleTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxIdleTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxIdleTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxIdleTime(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn ReconnectionAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReconnectionAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnewval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReconnectionAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReconnectionAction(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn BrokenConnectionAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BrokenConnectionAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnewval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBrokenConnectionAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBrokenConnectionAction(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn ConnectClientDrivesAtLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectClientDrivesAtLogon(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnewval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectClientDrivesAtLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectClientDrivesAtLogon(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn ConnectClientPrintersAtLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectClientPrintersAtLogon(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectClientPrintersAtLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectClientPrintersAtLogon(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn DefaultToMainPrinter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultToMainPrinter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultToMainPrinter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultToMainPrinter(this, ::core::mem::transmute_copy(&newval)).into())
        }
        unsafe extern "system" fn TerminalServicesWorkDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalServicesWorkDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTerminalServicesWorkDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTerminalServicesWorkDirectory(this, ::core::mem::transmute(&pnewval)).into())
        }
        unsafe extern "system" fn TerminalServicesInitialProgram<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalServicesInitialProgram(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTerminalServicesInitialProgram<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTSUserEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTerminalServicesInitialProgram(this, ::core::mem::transmute(&pnewval)).into())
        }
        IADsTSUserEx_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TerminalServicesProfilePath: TerminalServicesProfilePath::<Identity, Impl, OFFSET>,
            SetTerminalServicesProfilePath: SetTerminalServicesProfilePath::<Identity, Impl, OFFSET>,
            TerminalServicesHomeDirectory: TerminalServicesHomeDirectory::<Identity, Impl, OFFSET>,
            SetTerminalServicesHomeDirectory: SetTerminalServicesHomeDirectory::<Identity, Impl, OFFSET>,
            TerminalServicesHomeDrive: TerminalServicesHomeDrive::<Identity, Impl, OFFSET>,
            SetTerminalServicesHomeDrive: SetTerminalServicesHomeDrive::<Identity, Impl, OFFSET>,
            AllowLogon: AllowLogon::<Identity, Impl, OFFSET>,
            SetAllowLogon: SetAllowLogon::<Identity, Impl, OFFSET>,
            EnableRemoteControl: EnableRemoteControl::<Identity, Impl, OFFSET>,
            SetEnableRemoteControl: SetEnableRemoteControl::<Identity, Impl, OFFSET>,
            MaxDisconnectionTime: MaxDisconnectionTime::<Identity, Impl, OFFSET>,
            SetMaxDisconnectionTime: SetMaxDisconnectionTime::<Identity, Impl, OFFSET>,
            MaxConnectionTime: MaxConnectionTime::<Identity, Impl, OFFSET>,
            SetMaxConnectionTime: SetMaxConnectionTime::<Identity, Impl, OFFSET>,
            MaxIdleTime: MaxIdleTime::<Identity, Impl, OFFSET>,
            SetMaxIdleTime: SetMaxIdleTime::<Identity, Impl, OFFSET>,
            ReconnectionAction: ReconnectionAction::<Identity, Impl, OFFSET>,
            SetReconnectionAction: SetReconnectionAction::<Identity, Impl, OFFSET>,
            BrokenConnectionAction: BrokenConnectionAction::<Identity, Impl, OFFSET>,
            SetBrokenConnectionAction: SetBrokenConnectionAction::<Identity, Impl, OFFSET>,
            ConnectClientDrivesAtLogon: ConnectClientDrivesAtLogon::<Identity, Impl, OFFSET>,
            SetConnectClientDrivesAtLogon: SetConnectClientDrivesAtLogon::<Identity, Impl, OFFSET>,
            ConnectClientPrintersAtLogon: ConnectClientPrintersAtLogon::<Identity, Impl, OFFSET>,
            SetConnectClientPrintersAtLogon: SetConnectClientPrintersAtLogon::<Identity, Impl, OFFSET>,
            DefaultToMainPrinter: DefaultToMainPrinter::<Identity, Impl, OFFSET>,
            SetDefaultToMainPrinter: SetDefaultToMainPrinter::<Identity, Impl, OFFSET>,
            TerminalServicesWorkDirectory: TerminalServicesWorkDirectory::<Identity, Impl, OFFSET>,
            SetTerminalServicesWorkDirectory: SetTerminalServicesWorkDirectory::<Identity, Impl, OFFSET>,
            TerminalServicesInitialProgram: TerminalServicesInitialProgram::<Identity, Impl, OFFSET>,
            SetTerminalServicesInitialProgram: SetTerminalServicesInitialProgram::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioDeviceEndpoint_Impl: ::windows_core::BaseImpl {
    fn SetBuffer(this: &Self::This, maxperiod: i64, u32latencycoefficient: u32) -> ::windows_core::Result<()>;
    fn GetRTCaps(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetEventDrivenCapable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn WriteExclusiveModeParametersToSharedMemory(this: &Self::This, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAudioDeviceEndpoint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioDeviceEndpoint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioDeviceEndpoint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioDeviceEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBuffer(this, ::core::mem::transmute_copy(&maxperiod), ::core::mem::transmute_copy(&u32latencycoefficient)).into())
        }
        unsafe extern "system" fn GetRTCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioDeviceEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRTCaps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisrtcapable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventDrivenCapable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioDeviceEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventDrivenCapable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbiseventcapable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteExclusiveModeParametersToSharedMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioDeviceEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteExclusiveModeParametersToSharedMemory(this, ::core::mem::transmute_copy(&htargetprocess), ::core::mem::transmute_copy(&hnsperiod), ::core::mem::transmute_copy(&hnsbufferduration), ::core::mem::transmute_copy(&u32latencycoefficient), ::core::mem::transmute_copy(&pu32sharedmemorysize), ::core::mem::transmute_copy(&phsharedmemory)).into())
        }
        IAudioDeviceEndpoint_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBuffer: SetBuffer::<Identity, Impl, OFFSET>,
            GetRTCaps: GetRTCaps::<Identity, Impl, OFFSET>,
            GetEventDrivenCapable: GetEventDrivenCapable::<Identity, Impl, OFFSET>,
            WriteExclusiveModeParametersToSharedMemory: WriteExclusiveModeParametersToSharedMemory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub trait IAudioEndpoint_Impl: ::windows_core::BaseImpl {
    fn GetFrameFormat(this: &Self::This) -> ::windows_core::Result<*mut super::super::Media::Audio::WAVEFORMATEX>;
    fn GetFramesPerPacket(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLatency(this: &Self::This) -> ::windows_core::Result<i64>;
    fn SetStreamFlags(this: &Self::This, streamflags: u32) -> ::windows_core::Result<()>;
    fn SetEventHandle(this: &Self::This, eventhandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl ::windows_core::Iids for IAudioEndpoint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpoint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpoint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFrameFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFramesPerPacket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pframesperpacket: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFramesPerPacket(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pframesperpacket, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, platency: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLatency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(platency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStreamFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStreamFlags(this, ::core::mem::transmute_copy(&streamflags)).into())
        }
        unsafe extern "system" fn SetEventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventHandle(this, ::core::mem::transmute_copy(&eventhandle)).into())
        }
        IAudioEndpoint_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFrameFormat: GetFrameFormat::<Identity, Impl, OFFSET>,
            GetFramesPerPacket: GetFramesPerPacket::<Identity, Impl, OFFSET>,
            GetLatency: GetLatency::<Identity, Impl, OFFSET>,
            SetStreamFlags: SetStreamFlags::<Identity, Impl, OFFSET>,
            SetEventHandle: SetEventHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioEndpointControl_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioEndpointControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        IAudioEndpointControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAudioEndpointRT_Impl: ::windows_core::BaseImpl {
    fn GetCurrentPadding(this: &Self::This, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION);
    fn ProcessingComplete(this: &Self::This);
    fn SetPinInactive(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPinActive(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioEndpointRT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointRT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioEndpointRT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentPadding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentPadding(this, ::core::mem::transmute_copy(&ppadding), ::core::mem::transmute_copy(&paecurrentposition)))
        }
        unsafe extern "system" fn ProcessingComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessingComplete(this))
        }
        unsafe extern "system" fn SetPinInactive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPinInactive(this).into())
        }
        unsafe extern "system" fn SetPinActive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPinActive(this).into())
        }
        IAudioEndpointRT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentPadding: GetCurrentPadding::<Identity, Impl, OFFSET>,
            ProcessingComplete: ProcessingComplete::<Identity, Impl, OFFSET>,
            SetPinInactive: SetPinInactive::<Identity, Impl, OFFSET>,
            SetPinActive: SetPinActive::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio_Apo\"`"]
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioInputEndpointRT_Impl: ::windows_core::BaseImpl {
    fn GetInputDataPointer(this: &Self::This, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION);
    fn ReleaseInputDataPointer(this: &Self::This, u32framecount: u32, pdatapointer: usize);
    fn PulseEndpoint(this: &Self::This);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl ::windows_core::Iids for IAudioInputEndpointRT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputEndpointRT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioInputEndpointRT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputDataPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputDataPointer(this, ::core::mem::transmute_copy(&pconnectionproperty), ::core::mem::transmute_copy(&paetimestamp)))
        }
        unsafe extern "system" fn ReleaseInputDataPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32framecount: u32, pdatapointer: usize) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseInputDataPointer(this, ::core::mem::transmute_copy(&u32framecount), ::core::mem::transmute_copy(&pdatapointer)))
        }
        unsafe extern "system" fn PulseEndpoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioInputEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PulseEndpoint(this))
        }
        IAudioInputEndpointRT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputDataPointer: GetInputDataPointer::<Identity, Impl, OFFSET>,
            ReleaseInputDataPointer: ReleaseInputDataPointer::<Identity, Impl, OFFSET>,
            PulseEndpoint: PulseEndpoint::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Media_Audio_Apo\"`"]
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioOutputEndpointRT_Impl: ::windows_core::BaseImpl {
    fn GetOutputDataPointer(this: &Self::This, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize;
    fn ReleaseOutputDataPointer(this: &Self::This, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY);
    fn PulseEndpoint(this: &Self::This);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl ::windows_core::Iids for IAudioOutputEndpointRT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioOutputEndpointRT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioOutputEndpointRT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOutputDataPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioOutputEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputDataPointer(this, ::core::mem::transmute_copy(&u32framecount), ::core::mem::transmute_copy(&paetimestamp)))
        }
        unsafe extern "system" fn ReleaseOutputDataPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioOutputEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseOutputDataPointer(this, ::core::mem::transmute_copy(&pconnectionproperty)))
        }
        unsafe extern "system" fn PulseEndpoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioOutputEndpointRT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PulseEndpoint(this))
        }
        IAudioOutputEndpointRT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOutputDataPointer: GetOutputDataPointer::<Identity, Impl, OFFSET>,
            ReleaseOutputDataPointer: ReleaseOutputDataPointer::<Identity, Impl, OFFSET>,
            PulseEndpoint: PulseEndpoint::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClient_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Connect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reconnect(this: &Self::This, width: u32, height: u32) -> ::windows_core::Result<()>;
    fn Settings(this: &Self::This) -> ::windows_core::Result<IRemoteDesktopClientSettings>;
    fn Actions(this: &Self::This) -> ::windows_core::Result<IRemoteDesktopClientActions>;
    fn TouchPointer(this: &Self::This) -> ::windows_core::Result<IRemoteDesktopClientTouchPointer>;
    fn DeleteSavedCredentials(this: &Self::This, servername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UpdateSessionDisplaySettings(this: &Self::This, width: u32, height: u32) -> ::windows_core::Result<()>;
    fn attachEvent(this: &Self::This, eventname: &::windows_core::BSTR, callback: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn detachEvent(this: &Self::This, eventname: &::windows_core::BSTR, callback: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRemoteDesktopClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDesktopClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn Reconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reconnect(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn Settings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Settings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(settings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Actions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Actions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TouchPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, touchpointer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TouchPointer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(touchpointer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteSavedCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteSavedCredentials(this, ::core::mem::transmute(&servername)).into())
        }
        unsafe extern "system" fn UpdateSessionDisplaySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateSessionDisplaySettings(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn attachEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::BSTR>, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::attachEvent(this, ::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&callback)).into())
        }
        unsafe extern "system" fn detachEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::BSTR>, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::detachEvent(this, ::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&callback)).into())
        }
        IRemoteDesktopClient_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Reconnect: Reconnect::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            Actions: Actions::<Identity, Impl, OFFSET>,
            TouchPointer: TouchPointer::<Identity, Impl, OFFSET>,
            DeleteSavedCredentials: DeleteSavedCredentials::<Identity, Impl, OFFSET>,
            UpdateSessionDisplaySettings: UpdateSessionDisplaySettings::<Identity, Impl, OFFSET>,
            attachEvent: attachEvent::<Identity, Impl, OFFSET>,
            detachEvent: detachEvent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClientActions_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn SuspendScreenUpdates(this: &Self::This) -> ::windows_core::Result<()>;
    fn ResumeScreenUpdates(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExecuteRemoteAction(this: &Self::This, remoteaction: RemoteActionType) -> ::windows_core::Result<()>;
    fn GetSnapshot(this: &Self::This, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRemoteDesktopClientActions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientActions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDesktopClientActions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SuspendScreenUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientActions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspendScreenUpdates(this).into())
        }
        unsafe extern "system" fn ResumeScreenUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientActions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeScreenUpdates(this).into())
        }
        unsafe extern "system" fn ExecuteRemoteAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientActions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaction: RemoteActionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteRemoteAction(this, ::core::mem::transmute_copy(&remoteaction)).into())
        }
        unsafe extern "system" fn GetSnapshot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientActions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSnapshot(this, ::core::mem::transmute_copy(&snapshotencoding), ::core::mem::transmute_copy(&snapshotformat), ::core::mem::transmute_copy(&snapshotwidth), ::core::mem::transmute_copy(&snapshotheight)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapshotdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRemoteDesktopClientActions_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SuspendScreenUpdates: SuspendScreenUpdates::<Identity, Impl, OFFSET>,
            ResumeScreenUpdates: ResumeScreenUpdates::<Identity, Impl, OFFSET>,
            ExecuteRemoteAction: ExecuteRemoteAction::<Identity, Impl, OFFSET>,
            GetSnapshot: GetSnapshot::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClientSettings_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ApplySettings(this: &Self::This, rdpfilecontents: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RetrieveSettings(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRdpProperty(this: &Self::This, propertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetRdpProperty(this: &Self::This, propertyname: &::windows_core::BSTR, value: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRemoteDesktopClientSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDesktopClientSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApplySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rdpfilecontents: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplySettings(this, ::core::mem::transmute(&rdpfilecontents)).into())
        }
        unsafe extern "system" fn RetrieveSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rdpfilecontents: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rdpfilecontents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRdpProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRdpProperty(this, ::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRdpProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRdpProperty(this, ::core::mem::transmute(&propertyname), ::core::mem::transmute(&value)).into())
        }
        IRemoteDesktopClientSettings_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ApplySettings: ApplySettings::<Identity, Impl, OFFSET>,
            RetrieveSettings: RetrieveSettings::<Identity, Impl, OFFSET>,
            GetRdpProperty: GetRdpProperty::<Identity, Impl, OFFSET>,
            SetRdpProperty: SetRdpProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClientTouchPointer_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventsEnabled(this: &Self::This, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EventsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPointerSpeed(this: &Self::This, pointerspeed: u32) -> ::windows_core::Result<()>;
    fn PointerSpeed(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRemoteDesktopClientTouchPointer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDesktopClientTouchPointer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventsEnabled(this, ::core::mem::transmute_copy(&eventsenabled)).into())
        }
        unsafe extern "system" fn EventsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventsenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPointerSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerspeed: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPointerSpeed(this, ::core::mem::transmute_copy(&pointerspeed)).into())
        }
        unsafe extern "system" fn PointerSpeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerspeed: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerSpeed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pointerspeed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRemoteDesktopClientTouchPointer_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEventsEnabled: SetEventsEnabled::<Identity, Impl, OFFSET>,
            EventsEnabled: EventsEnabled::<Identity, Impl, OFFSET>,
            SetPointerSpeed: SetPointerSpeed::<Identity, Impl, OFFSET>,
            PointerSpeed: PointerSpeed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRemoteSystemAdditionalInfoProvider_Impl: ::windows_core::BaseImpl {
    fn GetAdditionalInfo(this: &Self::This, deduplicationid: *mut ::windows_core::HSTRING, riid: *const ::windows_core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRemoteSystemAdditionalInfoProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteSystemAdditionalInfoProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteSystemAdditionalInfoProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAdditionalInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteSystemAdditionalInfoProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deduplicationid: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdditionalInfo(this, ::core::mem::transmute_copy(&deduplicationid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&mapview)).into())
        }
        IRemoteSystemAdditionalInfoProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAdditionalInfo: GetAdditionalInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITSGAccountingEngine_Impl: ::windows_core::BaseImpl {
    fn DoAccounting(this: &Self::This, accountingdatatype: AAAccountingDataType, accountingdata: &AAAccountingData) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITSGAccountingEngine {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAccountingEngine_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSGAccountingEngine {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoAccounting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAccountingEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: AAAccountingData) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoAccounting(this, ::core::mem::transmute_copy(&accountingdatatype), ::core::mem::transmute(&accountingdata)).into())
        }
        ITSGAccountingEngine_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DoAccounting: DoAccounting::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGAuthenticateUserSink_Impl: ::windows_core::BaseImpl {
    fn OnUserAuthenticated(this: &Self::This, username: &::windows_core::BSTR, userdomain: &::windows_core::BSTR, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()>;
    fn OnUserAuthenticationFailed(this: &Self::This, context: usize, genericerrorcode: ::windows_core::HRESULT, specificerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ReauthenticateUser(this: &Self::This, context: usize) -> ::windows_core::Result<()>;
    fn DisconnectUser(this: &Self::This, context: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITSGAuthenticateUserSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSGAuthenticateUserSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUserAuthenticated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, userdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUserAuthenticated(this, ::core::mem::transmute(&username), ::core::mem::transmute(&userdomain), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&usertoken)).into())
        }
        unsafe extern "system" fn OnUserAuthenticationFailed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: usize, genericerrorcode: ::windows_core::HRESULT, specificerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUserAuthenticationFailed(this, ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&genericerrorcode), ::core::mem::transmute_copy(&specificerrorcode)).into())
        }
        unsafe extern "system" fn ReauthenticateUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReauthenticateUser(this, ::core::mem::transmute_copy(&context)).into())
        }
        unsafe extern "system" fn DisconnectUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectUser(this, ::core::mem::transmute_copy(&context)).into())
        }
        ITSGAuthenticateUserSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnUserAuthenticated: OnUserAuthenticated::<Identity, Impl, OFFSET>,
            OnUserAuthenticationFailed: OnUserAuthenticationFailed::<Identity, Impl, OFFSET>,
            ReauthenticateUser: ReauthenticateUser::<Identity, Impl, OFFSET>,
            DisconnectUser: DisconnectUser::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITSGAuthenticationEngine_Impl: ::windows_core::BaseImpl {
    fn AuthenticateUser(this: &Self::This, mainsessionid: &::windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: ::core::option::Option<&ITSGAuthenticateUserSink>) -> ::windows_core::Result<()>;
    fn CancelAuthentication(this: &Self::This, mainsessionid: &::windows_core::GUID, context: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITSGAuthenticationEngine {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticationEngine_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSGAuthenticationEngine {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AuthenticateUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticationEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AuthenticateUser(this, ::core::mem::transmute(&mainsessionid), ::core::mem::transmute_copy(&cookiedata), ::core::mem::transmute_copy(&numcookiebytes), ::core::mem::transmute_copy(&context), ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn CancelAuthentication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthenticationEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, context: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAuthentication(this, ::core::mem::transmute(&mainsessionid), ::core::mem::transmute_copy(&context)).into())
        }
        ITSGAuthenticationEngine_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AuthenticateUser: AuthenticateUser::<Identity, Impl, OFFSET>,
            CancelAuthentication: CancelAuthentication::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITSGAuthorizeConnectionSink_Impl: ::windows_core::BaseImpl {
    fn OnConnectionAuthorized(this: &Self::This, hrin: ::windows_core::HRESULT, mainsessionid: &::windows_core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITSGAuthorizeConnectionSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthorizeConnectionSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSGAuthorizeConnectionSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConnectionAuthorized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthorizeConnectionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrin: ::windows_core::HRESULT, mainsessionid: ::windows_core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectionAuthorized(this, ::core::mem::transmute_copy(&hrin), ::core::mem::transmute(&mainsessionid), ::core::mem::transmute_copy(&cbsohresponse), ::core::mem::transmute_copy(&pbsohresponse), ::core::mem::transmute_copy(&idletimeout), ::core::mem::transmute_copy(&sessiontimeout), ::core::mem::transmute_copy(&sessiontimeoutaction), ::core::mem::transmute_copy(&trustclass), ::core::mem::transmute_copy(&policyattributes)).into())
        }
        ITSGAuthorizeConnectionSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnConnectionAuthorized: OnConnectionAuthorized::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITSGAuthorizeResourceSink_Impl: ::windows_core::BaseImpl {
    fn OnChannelAuthorized(this: &Self::This, hrin: ::windows_core::HRESULT, mainsessionid: &::windows_core::GUID, subsessionid: i32, allowedresourcenames: *const ::windows_core::BSTR, numallowedresourcenames: u32, failedresourcenames: *const ::windows_core::BSTR, numfailedresourcenames: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITSGAuthorizeResourceSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthorizeResourceSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSGAuthorizeResourceSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChannelAuthorized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGAuthorizeResourceSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrin: ::windows_core::HRESULT, mainsessionid: ::windows_core::GUID, subsessionid: i32, allowedresourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numallowedresourcenames: u32, failedresourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numfailedresourcenames: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChannelAuthorized(this, ::core::mem::transmute_copy(&hrin), ::core::mem::transmute(&mainsessionid), ::core::mem::transmute_copy(&subsessionid), ::core::mem::transmute_copy(&allowedresourcenames), ::core::mem::transmute_copy(&numallowedresourcenames), ::core::mem::transmute_copy(&failedresourcenames), ::core::mem::transmute_copy(&numfailedresourcenames)).into())
        }
        ITSGAuthorizeResourceSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnChannelAuthorized: OnChannelAuthorized::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGPolicyEngine_Impl: ::windows_core::BaseImpl {
    fn AuthorizeConnection(this: &Self::This, mainsessionid: &::windows_core::GUID, username: &::windows_core::BSTR, authtype: AAAuthSchemes, clientmachineip: &::windows_core::BSTR, clientmachinename: &::windows_core::BSTR, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: ::core::option::Option<&ITSGAuthorizeConnectionSink>) -> ::windows_core::Result<()>;
    fn AuthorizeResource(this: &Self::This, mainsessionid: &::windows_core::GUID, subsessionid: i32, username: &::windows_core::BSTR, resourcenames: *const ::windows_core::BSTR, numresources: u32, alternateresourcenames: *const ::windows_core::BSTR, numalternateresourcename: u32, portnumber: u32, operation: &::windows_core::BSTR, cookie: *const u8, numbytesincookie: u32, psink: ::core::option::Option<&ITSGAuthorizeResourceSink>) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsQuarantineEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITSGPolicyEngine {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGPolicyEngine_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSGPolicyEngine {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AuthorizeConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGPolicyEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, authtype: AAAuthSchemes, clientmachineip: ::std::mem::MaybeUninit<::windows_core::BSTR>, clientmachinename: ::std::mem::MaybeUninit<::windows_core::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::AuthorizeConnection(this, ::core::mem::transmute(&mainsessionid), ::core::mem::transmute(&username), ::core::mem::transmute_copy(&authtype), ::core::mem::transmute(&clientmachineip), ::core::mem::transmute(&clientmachinename), ::core::mem::transmute_copy(&sohdata), ::core::mem::transmute_copy(&numsohbytes), ::core::mem::transmute_copy(&cookiedata), ::core::mem::transmute_copy(&numcookiebytes), ::core::mem::transmute_copy(&usertoken), ::windows_core::from_raw_borrowed(&psink)).into()
            })
        }
        unsafe extern "system" fn AuthorizeResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGPolicyEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows_core::GUID, subsessionid: i32, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, resourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numresources: u32, alternateresourcenames: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, numalternateresourcename: u32, portnumber: u32, operation: ::std::mem::MaybeUninit<::windows_core::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::AuthorizeResource(
                    this,
                    ::core::mem::transmute(&mainsessionid),
                    ::core::mem::transmute_copy(&subsessionid),
                    ::core::mem::transmute(&username),
                    ::core::mem::transmute_copy(&resourcenames),
                    ::core::mem::transmute_copy(&numresources),
                    ::core::mem::transmute_copy(&alternateresourcenames),
                    ::core::mem::transmute_copy(&numalternateresourcename),
                    ::core::mem::transmute_copy(&portnumber),
                    ::core::mem::transmute(&operation),
                    ::core::mem::transmute_copy(&cookie),
                    ::core::mem::transmute_copy(&numbytesincookie),
                    ::windows_core::from_raw_borrowed(&psink),
                )
                .into()
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGPolicyEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn IsQuarantineEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSGPolicyEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsQuarantineEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quarantineenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITSGPolicyEngine_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AuthorizeConnection: AuthorizeConnection::<Identity, Impl, OFFSET>,
            AuthorizeResource: AuthorizeResource::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            IsQuarantineEnabled: IsQuarantineEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbBaseNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnError(this: &Self::This, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnReportStatus(this: &Self::This, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbBaseNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbBaseNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbBaseNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbBaseNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnError(this, ::core::mem::transmute_copy(&hrerror)).into())
        }
        unsafe extern "system" fn OnReportStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbBaseNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReportStatus(this, ::core::mem::transmute_copy(&messagetype), ::core::mem::transmute_copy(&messageid)).into())
        }
        ITsSbBaseNotifySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnError: OnError::<Identity, Impl, OFFSET>,
            OnReportStatus: OnReportStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbClientConnection_Impl: ::windows_core::BaseImpl {
    fn UserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Domain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InitialProgram(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LoadBalanceResult(this: &Self::This) -> ::windows_core::Result<ITsSbLoadBalanceResult>;
    fn FarmName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PutContext(this: &Self::This, contextid: &::windows_core::BSTR, context: &super::Variant::VARIANT, existingcontext: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetContext(this: &Self::This, contextid: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Environment(this: &Self::This) -> ::windows_core::Result<ITsSbEnvironment>;
    fn get_ConnectionError(this: &Self::This) -> ::windows_core::Result<()>;
    fn SamUserAccount(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClientConnectionPropertySet(this: &Self::This) -> ::windows_core::Result<ITsSbClientConnectionPropertySet>;
    fn IsFirstAssignment(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn RdFarmType(this: &Self::This) -> ::windows_core::Result<RD_FARM_TYPE>;
    fn UserSidString(this: &Self::This) -> ::windows_core::Result<*mut i8>;
    fn GetDisconnectedSession(this: &Self::This) -> ::windows_core::Result<ITsSbSession>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbClientConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbClientConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Domain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Domain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitialProgram<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialProgram(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadBalanceResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadBalanceResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FarmName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FarmName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextid: ::std::mem::MaybeUninit<::windows_core::BSTR>, context: super::Variant::VARIANT, existingcontext: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutContext(this, ::core::mem::transmute(&contextid), ::core::mem::transmute(&context), ::core::mem::transmute_copy(&existingcontext)).into())
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextid: ::std::mem::MaybeUninit<::windows_core::BSTR>, context: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this, ::core::mem::transmute(&contextid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Environment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Environment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenvironment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ConnectionError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_ConnectionError(this).into())
        }
        unsafe extern "system" fn SamUserAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SamUserAccount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientConnectionPropertySet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientConnectionPropertySet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFirstAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFirstAssignment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RdFarmType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RdFarmType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prdfarmtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserSidString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszusersidstring: *mut *mut i8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserSidString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszusersidstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisconnectedSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisconnectedSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITsSbClientConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UserName: UserName::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            InitialProgram: InitialProgram::<Identity, Impl, OFFSET>,
            LoadBalanceResult: LoadBalanceResult::<Identity, Impl, OFFSET>,
            FarmName: FarmName::<Identity, Impl, OFFSET>,
            PutContext: PutContext::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            Environment: Environment::<Identity, Impl, OFFSET>,
            get_ConnectionError: get_ConnectionError::<Identity, Impl, OFFSET>,
            SamUserAccount: SamUserAccount::<Identity, Impl, OFFSET>,
            ClientConnectionPropertySet: ClientConnectionPropertySet::<Identity, Impl, OFFSET>,
            IsFirstAssignment: IsFirstAssignment::<Identity, Impl, OFFSET>,
            RdFarmType: RdFarmType::<Identity, Impl, OFFSET>,
            UserSidString: UserSidString::<Identity, Impl, OFFSET>,
            GetDisconnectedSession: GetDisconnectedSession::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbClientConnectionPropertySet_Impl: ::windows_core::BaseImpl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbClientConnectionPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPropertySet);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbClientConnectionPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbClientConnectionPropertySet {
    const VTABLE: Self::Vtable = { ITsSbClientConnectionPropertySet_Vtbl { base__: <ITsSbPropertySet as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbEnvironment_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServerWeight(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnvironmentPropertySet(this: &Self::This) -> ::windows_core::Result<ITsSbEnvironmentPropertySet>;
    fn SetEnvironmentPropertySet(this: &Self::This, pval: ::core::option::Option<&ITsSbEnvironmentPropertySet>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbEnvironment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbEnvironment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbEnvironment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbEnvironment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbEnvironment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerWeight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnvironmentPropertySet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbEnvironment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnvironmentPropertySet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnvironmentPropertySet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbEnvironment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnvironmentPropertySet(this, ::windows_core::from_raw_borrowed(&pval)).into())
        }
        ITsSbEnvironment_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            ServerWeight: ServerWeight::<Identity, Impl, OFFSET>,
            EnvironmentPropertySet: EnvironmentPropertySet::<Identity, Impl, OFFSET>,
            SetEnvironmentPropertySet: SetEnvironmentPropertySet::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbEnvironmentPropertySet_Impl: ::windows_core::BaseImpl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbEnvironmentPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPropertySet);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbEnvironmentPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbEnvironmentPropertySet {
    const VTABLE: Self::Vtable = { ITsSbEnvironmentPropertySet_Vtbl { base__: <ITsSbPropertySet as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbFilterPluginStore_Impl: ::windows_core::BaseImpl {
    fn SaveProperties(this: &Self::This, ppropertyset: ::core::option::Option<&ITsSbPropertySet>) -> ::windows_core::Result<()>;
    fn EnumerateProperties(this: &Self::This) -> ::windows_core::Result<ITsSbPropertySet>;
    fn DeleteProperties(this: &Self::This, propertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbFilterPluginStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbFilterPluginStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbFilterPluginStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SaveProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbFilterPluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropertyset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveProperties(this, ::windows_core::from_raw_borrowed(&ppropertyset)).into())
        }
        unsafe extern "system" fn EnumerateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbFilterPluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbFilterPluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteProperties(this, ::core::mem::transmute(&propertyname)).into())
        }
        ITsSbFilterPluginStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SaveProperties: SaveProperties::<Identity, Impl, OFFSET>,
            EnumerateProperties: EnumerateProperties::<Identity, Impl, OFFSET>,
            DeleteProperties: DeleteProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbGenericNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnCompleted(this: &Self::This, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetWaitTimeout(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITsSbGenericNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGenericNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbGenericNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGenericNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCompleted(this, ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn GetWaitTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGenericNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWaitTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfttimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITsSbGenericNotifySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCompleted: OnCompleted::<Identity, Impl, OFFSET>,
            GetWaitTimeout: GetWaitTimeout::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbGlobalStore_Impl: ::windows_core::BaseImpl {
    fn QueryTarget(this: &Self::This, providername: &::windows_core::BSTR, targetname: &::windows_core::BSTR, farmname: &::windows_core::BSTR) -> ::windows_core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(this: &Self::This, providername: &::windows_core::BSTR, dwsessionid: u32, targetname: &::windows_core::BSTR) -> ::windows_core::Result<ITsSbSession>;
    fn EnumerateFarms(this: &Self::This, providername: &::windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn EnumerateTargets(this: &Self::This, providername: &::windows_core::BSTR, farmname: &::windows_core::BSTR, envname: &::windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::Result<()>;
    fn EnumerateEnvironmentsByProvider(this: &Self::This, providername: &::windows_core::BSTR, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::Result<()>;
    fn EnumerateSessions(this: &Self::This, providername: &::windows_core::BSTR, targetname: &::windows_core::BSTR, username: &::windows_core::BSTR, userdomain: &::windows_core::BSTR, poolname: &::windows_core::BSTR, initialprogram: &::windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::Result<()>;
    fn GetFarmProperty(this: &Self::This, farmname: &::windows_core::BSTR, propertyname: &::windows_core::BSTR, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbGlobalStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbGlobalStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryTarget(this, ::core::mem::transmute(&providername), ::core::mem::transmute(&targetname), ::core::mem::transmute(&farmname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QuerySessionBySessionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsessionid: u32, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySessionBySessionId(this, ::core::mem::transmute(&providername), ::core::mem::transmute_copy(&dwsessionid), ::core::mem::transmute(&targetname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateFarms<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateFarms(this, ::core::mem::transmute(&providername), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into())
        }
        unsafe extern "system" fn EnumerateTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, envname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateTargets(this, ::core::mem::transmute(&providername), ::core::mem::transmute(&farmname), ::core::mem::transmute(&envname), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into())
        }
        unsafe extern "system" fn EnumerateEnvironmentsByProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateEnvironmentsByProvider(this, ::core::mem::transmute(&providername), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&ppval)).into())
        }
        unsafe extern "system" fn EnumerateSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, userdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, initialprogram: ::std::mem::MaybeUninit<::windows_core::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateSessions(this, ::core::mem::transmute(&providername), ::core::mem::transmute(&targetname), ::core::mem::transmute(&username), ::core::mem::transmute(&userdomain), ::core::mem::transmute(&poolname), ::core::mem::transmute(&initialprogram), ::core::mem::transmute_copy(&psessionstate), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&ppval)).into())
        }
        unsafe extern "system" fn GetFarmProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbGlobalStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFarmProperty(this, ::core::mem::transmute(&farmname), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        ITsSbGlobalStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryTarget: QueryTarget::<Identity, Impl, OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Identity, Impl, OFFSET>,
            EnumerateFarms: EnumerateFarms::<Identity, Impl, OFFSET>,
            EnumerateTargets: EnumerateTargets::<Identity, Impl, OFFSET>,
            EnumerateEnvironmentsByProvider: EnumerateEnvironmentsByProvider::<Identity, Impl, OFFSET>,
            EnumerateSessions: EnumerateSessions::<Identity, Impl, OFFSET>,
            GetFarmProperty: GetFarmProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbLoadBalanceResult_Impl: ::windows_core::BaseImpl {
    fn TargetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITsSbLoadBalanceResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbLoadBalanceResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbLoadBalanceResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TargetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbLoadBalanceResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITsSbLoadBalanceResult_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TargetName: TargetName::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbLoadBalancing_Impl: ::windows_core::BaseImpl + ITsSbPlugin_Impl {
    fn GetMostSuitableTarget(this: &Self::This, pconnection: ::core::option::Option<&ITsSbClientConnection>, plbsink: ::core::option::Option<&ITsSbLoadBalancingNotifySink>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbLoadBalancing {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPlugin);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbLoadBalancing_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbLoadBalancing {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMostSuitableTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbLoadBalancing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, plbsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMostSuitableTarget(this, ::windows_core::from_raw_borrowed(&pconnection), ::windows_core::from_raw_borrowed(&plbsink)).into())
        }
        ITsSbLoadBalancing_Vtbl {
            base__: <ITsSbPlugin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMostSuitableTarget: GetMostSuitableTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbLoadBalancingNotifySink_Impl: ::windows_core::BaseImpl + ITsSbBaseNotifySink_Impl {
    fn OnGetMostSuitableTarget(this: &Self::This, plbresult: ::core::option::Option<&ITsSbLoadBalanceResult>, fisnewconnection: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITsSbLoadBalancingNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbBaseNotifySink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbLoadBalancingNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbLoadBalancingNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnGetMostSuitableTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbLoadBalancingNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbresult: *mut ::core::ffi::c_void, fisnewconnection: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnGetMostSuitableTarget(this, ::windows_core::from_raw_borrowed(&plbresult), ::core::mem::transmute_copy(&fisnewconnection)).into())
        }
        ITsSbLoadBalancingNotifySink_Vtbl {
            base__: <ITsSbBaseNotifySink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnGetMostSuitableTarget: OnGetMostSuitableTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbOrchestration_Impl: ::windows_core::BaseImpl + ITsSbPlugin_Impl {
    fn PrepareTargetForConnect(this: &Self::This, pconnection: ::core::option::Option<&ITsSbClientConnection>, porchestrationnotifysink: ::core::option::Option<&ITsSbOrchestrationNotifySink>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbOrchestration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPlugin);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbOrchestration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbOrchestration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrepareTargetForConnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbOrchestration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, porchestrationnotifysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareTargetForConnect(this, ::windows_core::from_raw_borrowed(&pconnection), ::windows_core::from_raw_borrowed(&porchestrationnotifysink)).into())
        }
        ITsSbOrchestration_Vtbl {
            base__: <ITsSbPlugin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrepareTargetForConnect: PrepareTargetForConnect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbOrchestrationNotifySink_Impl: ::windows_core::BaseImpl + ITsSbBaseNotifySink_Impl {
    fn OnReadyToConnect(this: &Self::This, ptarget: ::core::option::Option<&ITsSbTarget>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbOrchestrationNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbBaseNotifySink);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbOrchestrationNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbOrchestrationNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnReadyToConnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbOrchestrationNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReadyToConnect(this, ::windows_core::from_raw_borrowed(&ptarget)).into())
        }
        ITsSbOrchestrationNotifySink_Vtbl { base__: <ITsSbBaseNotifySink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnReadyToConnect: OnReadyToConnect::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlacement_Impl: ::windows_core::BaseImpl + ITsSbPlugin_Impl {
    fn QueryEnvironmentForTarget(this: &Self::This, pconnection: ::core::option::Option<&ITsSbClientConnection>, pplacementsink: ::core::option::Option<&ITsSbPlacementNotifySink>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbPlacement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPlugin);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPlacement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbPlacement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryEnvironmentForTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPlacement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, pplacementsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryEnvironmentForTarget(this, ::windows_core::from_raw_borrowed(&pconnection), ::windows_core::from_raw_borrowed(&pplacementsink)).into())
        }
        ITsSbPlacement_Vtbl {
            base__: <ITsSbPlugin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryEnvironmentForTarget: QueryEnvironmentForTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbPlacementNotifySink_Impl: ::windows_core::BaseImpl + ITsSbBaseNotifySink_Impl {
    fn OnQueryEnvironmentCompleted(this: &Self::This, penvironment: ::core::option::Option<&ITsSbEnvironment>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbPlacementNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbBaseNotifySink);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPlacementNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbPlacementNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnQueryEnvironmentCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPlacementNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQueryEnvironmentCompleted(this, ::windows_core::from_raw_borrowed(&penvironment)).into())
        }
        ITsSbPlacementNotifySink_Vtbl {
            base__: <ITsSbBaseNotifySink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnQueryEnvironmentCompleted: OnQueryEnvironmentCompleted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlugin_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pprovider: ::core::option::Option<&ITsSbProvider>, pnotifysink: ::core::option::Option<&ITsSbPluginNotifySink>, ppropertyset: ::core::option::Option<&ITsSbPluginPropertySet>) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbPlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbPlugin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, pnotifysink: *mut ::core::ffi::c_void, ppropertyset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pprovider), ::windows_core::from_raw_borrowed(&pnotifysink), ::windows_core::from_raw_borrowed(&ppropertyset)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this, ::core::mem::transmute_copy(&hr)).into())
        }
        ITsSbPlugin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbPluginNotifySink_Impl: ::windows_core::BaseImpl + ITsSbBaseNotifySink_Impl {
    fn OnInitialized(this: &Self::This, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnTerminated(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbPluginNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbBaseNotifySink);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPluginNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbPluginNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInitialized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInitialized(this, ::core::mem::transmute_copy(&hr)).into())
        }
        unsafe extern "system" fn OnTerminated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTerminated(this).into())
        }
        ITsSbPluginNotifySink_Vtbl {
            base__: <ITsSbBaseNotifySink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInitialized: OnInitialized::<Identity, Impl, OFFSET>,
            OnTerminated: OnTerminated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbPluginPropertySet_Impl: ::windows_core::BaseImpl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbPluginPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPropertySet);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPluginPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbPluginPropertySet {
    const VTABLE: Self::Vtable = { ITsSbPluginPropertySet_Vtbl { base__: <ITsSbPropertySet as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbPropertySet_Impl: ::windows_core::BaseImpl + super::Com::StructuredStorage::IPropertyBag_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::StructuredStorage::IPropertyBag);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbPropertySet {
    const VTABLE: Self::Vtable = { ITsSbPropertySet_Vtbl { base__: <super::Com::StructuredStorage::IPropertyBag as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbProvider_Impl: ::windows_core::BaseImpl {
    fn CreateTargetObject(this: &Self::This, targetname: &::windows_core::BSTR, environmentname: &::windows_core::BSTR) -> ::windows_core::Result<ITsSbTarget>;
    fn CreateLoadBalanceResultObject(this: &Self::This, targetname: &::windows_core::BSTR) -> ::windows_core::Result<ITsSbLoadBalanceResult>;
    fn CreateSessionObject(this: &Self::This, targetname: &::windows_core::BSTR, username: &::windows_core::BSTR, domain: &::windows_core::BSTR, sessionid: u32) -> ::windows_core::Result<ITsSbSession>;
    fn CreatePluginPropertySet(this: &Self::This) -> ::windows_core::Result<ITsSbPluginPropertySet>;
    fn CreateTargetPropertySetObject(this: &Self::This) -> ::windows_core::Result<ITsSbTargetPropertySet>;
    fn CreateEnvironmentObject(this: &Self::This, name: &::windows_core::BSTR, serverweight: u32) -> ::windows_core::Result<ITsSbEnvironment>;
    fn GetResourcePluginStore(this: &Self::This) -> ::windows_core::Result<ITsSbResourcePluginStore>;
    fn GetFilterPluginStore(this: &Self::This) -> ::windows_core::Result<ITsSbFilterPluginStore>;
    fn RegisterForNotification(this: &Self::This, notificationtype: u32, resourcetomonitor: &::windows_core::BSTR, ppluginnotification: ::core::option::Option<&ITsSbResourceNotification>) -> ::windows_core::Result<()>;
    fn UnRegisterForNotification(this: &Self::This, notificationtype: u32, resourcetomonitor: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetInstanceOfGlobalStore(this: &Self::This) -> ::windows_core::Result<ITsSbGlobalStore>;
    fn CreateEnvironmentPropertySetObject(this: &Self::This) -> ::windows_core::Result<ITsSbEnvironmentPropertySet>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTargetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTargetObject(this, ::core::mem::transmute(&targetname), ::core::mem::transmute(&environmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLoadBalanceResultObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pplbresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLoadBalanceResultObject(this, ::core::mem::transmute(&targetname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplbresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSessionObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, sessionid: u32, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSessionObject(this, ::core::mem::transmute(&targetname), ::core::mem::transmute(&username), ::core::mem::transmute(&domain), ::core::mem::transmute_copy(&sessionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePluginPropertySet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePluginPropertySet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTargetPropertySetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTargetPropertySetObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEnvironmentObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, serverweight: u32, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEnvironmentObject(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&serverweight)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenvironment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResourcePluginStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResourcePluginStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilterPluginStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilterPluginStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterForNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppluginnotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterForNotification(this, ::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute(&resourcetomonitor), ::windows_core::from_raw_borrowed(&ppluginnotification)).into())
        }
        unsafe extern "system" fn UnRegisterForNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnRegisterForNotification(this, ::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute(&resourcetomonitor)).into())
        }
        unsafe extern "system" fn GetInstanceOfGlobalStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppglobalstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstanceOfGlobalStore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppglobalstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEnvironmentPropertySetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEnvironmentPropertySetObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITsSbProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTargetObject: CreateTargetObject::<Identity, Impl, OFFSET>,
            CreateLoadBalanceResultObject: CreateLoadBalanceResultObject::<Identity, Impl, OFFSET>,
            CreateSessionObject: CreateSessionObject::<Identity, Impl, OFFSET>,
            CreatePluginPropertySet: CreatePluginPropertySet::<Identity, Impl, OFFSET>,
            CreateTargetPropertySetObject: CreateTargetPropertySetObject::<Identity, Impl, OFFSET>,
            CreateEnvironmentObject: CreateEnvironmentObject::<Identity, Impl, OFFSET>,
            GetResourcePluginStore: GetResourcePluginStore::<Identity, Impl, OFFSET>,
            GetFilterPluginStore: GetFilterPluginStore::<Identity, Impl, OFFSET>,
            RegisterForNotification: RegisterForNotification::<Identity, Impl, OFFSET>,
            UnRegisterForNotification: UnRegisterForNotification::<Identity, Impl, OFFSET>,
            GetInstanceOfGlobalStore: GetInstanceOfGlobalStore::<Identity, Impl, OFFSET>,
            CreateEnvironmentPropertySetObject: CreateEnvironmentPropertySetObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbProvisioning_Impl: ::windows_core::BaseImpl + ITsSbPlugin_Impl {
    fn CreateVirtualMachines(this: &Self::This, jobxmlstring: &::windows_core::BSTR, jobguid: &::windows_core::BSTR, psink: ::core::option::Option<&ITsSbProvisioningPluginNotifySink>) -> ::windows_core::Result<()>;
    fn PatchVirtualMachines(this: &Self::This, jobxmlstring: &::windows_core::BSTR, jobguid: &::windows_core::BSTR, psink: ::core::option::Option<&ITsSbProvisioningPluginNotifySink>, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows_core::Result<()>;
    fn DeleteVirtualMachines(this: &Self::This, jobxmlstring: &::windows_core::BSTR, jobguid: &::windows_core::BSTR, psink: ::core::option::Option<&ITsSbProvisioningPluginNotifySink>) -> ::windows_core::Result<()>;
    fn CancelJob(this: &Self::This, jobguid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbProvisioning {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPlugin);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioning_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbProvisioning {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVirtualMachines<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVirtualMachines(this, ::core::mem::transmute(&jobxmlstring), ::core::mem::transmute(&jobguid), ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn PatchVirtualMachines<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psink: *mut ::core::ffi::c_void, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PatchVirtualMachines(this, ::core::mem::transmute(&jobxmlstring), ::core::mem::transmute(&jobguid), ::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&pvmpatchinfo)).into())
        }
        unsafe extern "system" fn DeleteVirtualMachines<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteVirtualMachines(this, ::core::mem::transmute(&jobxmlstring), ::core::mem::transmute(&jobguid), ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn CancelJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobguid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelJob(this, ::core::mem::transmute(&jobguid)).into())
        }
        ITsSbProvisioning_Vtbl {
            base__: <ITsSbPlugin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVirtualMachines: CreateVirtualMachines::<Identity, Impl, OFFSET>,
            PatchVirtualMachines: PatchVirtualMachines::<Identity, Impl, OFFSET>,
            DeleteVirtualMachines: DeleteVirtualMachines::<Identity, Impl, OFFSET>,
            CancelJob: CancelJob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbProvisioningPluginNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnJobCreated(this: &Self::This, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows_core::Result<()>;
    fn OnVirtualMachineStatusChanged(this: &Self::This, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnJobCompleted(this: &Self::This, resultcode: ::windows_core::HRESULT, resultdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnJobCancelled(this: &Self::This) -> ::windows_core::Result<()>;
    fn LockVirtualMachine(this: &Self::This, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows_core::Result<()>;
    fn OnVirtualMachineHostStatusChanged(this: &Self::This, vmhost: &::windows_core::BSTR, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbProvisioningPluginNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbProvisioningPluginNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnJobCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnJobCreated(this, ::core::mem::transmute_copy(&pvmnotifyinfo)).into())
        }
        unsafe extern "system" fn OnVirtualMachineStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnVirtualMachineStatusChanged(this, ::core::mem::transmute_copy(&pvmnotifyentry), ::core::mem::transmute_copy(&vmnotifystatus), ::core::mem::transmute_copy(&errorcode), ::core::mem::transmute(&errordescr)).into())
        }
        unsafe extern "system" fn OnJobCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resultcode: ::windows_core::HRESULT, resultdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnJobCompleted(this, ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute(&resultdescription)).into())
        }
        unsafe extern "system" fn OnJobCancelled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnJobCancelled(this).into())
        }
        unsafe extern "system" fn LockVirtualMachine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockVirtualMachine(this, ::core::mem::transmute_copy(&pvmnotifyentry)).into())
        }
        unsafe extern "system" fn OnVirtualMachineHostStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vmhost: ::std::mem::MaybeUninit<::windows_core::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows_core::HRESULT, errordescr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnVirtualMachineHostStatusChanged(this, ::core::mem::transmute(&vmhost), ::core::mem::transmute_copy(&vmhostnotifystatus), ::core::mem::transmute_copy(&errorcode), ::core::mem::transmute(&errordescr)).into())
        }
        ITsSbProvisioningPluginNotifySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnJobCreated: OnJobCreated::<Identity, Impl, OFFSET>,
            OnVirtualMachineStatusChanged: OnVirtualMachineStatusChanged::<Identity, Impl, OFFSET>,
            OnJobCompleted: OnJobCompleted::<Identity, Impl, OFFSET>,
            OnJobCancelled: OnJobCancelled::<Identity, Impl, OFFSET>,
            LockVirtualMachine: LockVirtualMachine::<Identity, Impl, OFFSET>,
            OnVirtualMachineHostStatusChanged: OnVirtualMachineHostStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbResourceNotification_Impl: ::windows_core::BaseImpl {
    fn NotifySessionChange(this: &Self::This, changetype: TSSESSION_STATE, psession: ::core::option::Option<&ITsSbSession>) -> ::windows_core::Result<()>;
    fn NotifyTargetChange(this: &Self::This, targetchangetype: u32, ptarget: ::core::option::Option<&ITsSbTarget>) -> ::windows_core::Result<()>;
    fn NotifyClientConnectionStateChange(this: &Self::This, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: ::core::option::Option<&ITsSbClientConnection>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbResourceNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbResourceNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifySessionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: TSSESSION_STATE, psession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionChange(this, ::core::mem::transmute_copy(&changetype), ::windows_core::from_raw_borrowed(&psession)).into())
        }
        unsafe extern "system" fn NotifyTargetChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetchangetype: u32, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyTargetChange(this, ::core::mem::transmute_copy(&targetchangetype), ::windows_core::from_raw_borrowed(&ptarget)).into())
        }
        unsafe extern "system" fn NotifyClientConnectionStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyClientConnectionStateChange(this, ::core::mem::transmute_copy(&changetype), ::windows_core::from_raw_borrowed(&pconnection)).into())
        }
        ITsSbResourceNotification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotifySessionChange: NotifySessionChange::<Identity, Impl, OFFSET>,
            NotifyTargetChange: NotifyTargetChange::<Identity, Impl, OFFSET>,
            NotifyClientConnectionStateChange: NotifyClientConnectionStateChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbResourceNotificationEx_Impl: ::windows_core::BaseImpl {
    fn NotifySessionChangeEx(this: &Self::This, targetname: &::windows_core::BSTR, username: &::windows_core::BSTR, domain: &::windows_core::BSTR, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows_core::Result<()>;
    fn NotifyTargetChangeEx(this: &Self::This, targetname: &::windows_core::BSTR, targetchangetype: u32) -> ::windows_core::Result<()>;
    fn NotifyClientConnectionStateChangeEx(this: &Self::This, username: &::windows_core::BSTR, domain: &::windows_core::BSTR, initialprogram: &::windows_core::BSTR, poolname: &::windows_core::BSTR, targetname: &::windows_core::BSTR, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbResourceNotificationEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotificationEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbResourceNotificationEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifySessionChangeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotificationEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionChangeEx(this, ::core::mem::transmute(&targetname), ::core::mem::transmute(&username), ::core::mem::transmute(&domain), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&sessionstate)).into())
        }
        unsafe extern "system" fn NotifyTargetChangeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotificationEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetchangetype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyTargetChangeEx(this, ::core::mem::transmute(&targetname), ::core::mem::transmute_copy(&targetchangetype)).into())
        }
        unsafe extern "system" fn NotifyClientConnectionStateChangeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourceNotificationEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, initialprogram: ::std::mem::MaybeUninit<::windows_core::BSTR>, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyClientConnectionStateChangeEx(this, ::core::mem::transmute(&username), ::core::mem::transmute(&domain), ::core::mem::transmute(&initialprogram), ::core::mem::transmute(&poolname), ::core::mem::transmute(&targetname), ::core::mem::transmute_copy(&connectionchangetype)).into())
        }
        ITsSbResourceNotificationEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotifySessionChangeEx: NotifySessionChangeEx::<Identity, Impl, OFFSET>,
            NotifyTargetChangeEx: NotifyTargetChangeEx::<Identity, Impl, OFFSET>,
            NotifyClientConnectionStateChangeEx: NotifyClientConnectionStateChangeEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbResourcePlugin_Impl: ::windows_core::BaseImpl + ITsSbPlugin_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbResourcePlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPlugin);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbResourcePlugin {
    const VTABLE: Self::Vtable = { ITsSbResourcePlugin_Vtbl { base__: <ITsSbPlugin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbResourcePluginStore_Impl: ::windows_core::BaseImpl {
    fn QueryTarget(this: &Self::This, targetname: &::windows_core::BSTR, farmname: &::windows_core::BSTR) -> ::windows_core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(this: &Self::This, dwsessionid: u32, targetname: &::windows_core::BSTR) -> ::windows_core::Result<ITsSbSession>;
    fn AddTargetToStore(this: &Self::This, ptarget: ::core::option::Option<&ITsSbTarget>) -> ::windows_core::Result<()>;
    fn AddSessionToStore(this: &Self::This, psession: ::core::option::Option<&ITsSbSession>) -> ::windows_core::Result<()>;
    fn AddEnvironmentToStore(this: &Self::This, penvironment: ::core::option::Option<&ITsSbEnvironment>) -> ::windows_core::Result<()>;
    fn RemoveEnvironmentFromStore(this: &Self::This, environmentname: &::windows_core::BSTR, bignoreowner: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnumerateFarms(this: &Self::This, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn QueryEnvironment(this: &Self::This, environmentname: &::windows_core::BSTR) -> ::windows_core::Result<ITsSbEnvironment>;
    fn EnumerateEnvironments(this: &Self::This, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::Result<()>;
    fn SaveTarget(this: &Self::This, ptarget: ::core::option::Option<&ITsSbTarget>, bforcewrite: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SaveEnvironment(this: &Self::This, penvironment: ::core::option::Option<&ITsSbEnvironment>, bforcewrite: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SaveSession(this: &Self::This, psession: ::core::option::Option<&ITsSbSession>) -> ::windows_core::Result<()>;
    fn SetTargetProperty(this: &Self::This, targetname: &::windows_core::BSTR, propertyname: &::windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetEnvironmentProperty(this: &Self::This, environmentname: &::windows_core::BSTR, propertyname: &::windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetTargetState(this: &Self::This, targetname: &::windows_core::BSTR, newstate: TARGET_STATE) -> ::windows_core::Result<TARGET_STATE>;
    fn SetSessionState(this: &Self::This, sbsession: ::core::option::Option<&ITsSbSession>) -> ::windows_core::Result<()>;
    fn EnumerateTargets(this: &Self::This, farmname: &::windows_core::BSTR, envname: &::windows_core::BSTR, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: &::windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::Result<()>;
    fn EnumerateSessions(this: &Self::This, targetname: &::windows_core::BSTR, username: &::windows_core::BSTR, userdomain: &::windows_core::BSTR, poolname: &::windows_core::BSTR, initialprogram: &::windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::Result<()>;
    fn GetFarmProperty(this: &Self::This, farmname: &::windows_core::BSTR, propertyname: &::windows_core::BSTR, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteTarget(this: &Self::This, targetname: &::windows_core::BSTR, hostname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetTargetPropertyWithVersionCheck(this: &Self::This, ptarget: ::core::option::Option<&ITsSbTarget>, propertyname: &::windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetEnvironmentPropertyWithVersionCheck(this: &Self::This, penvironment: ::core::option::Option<&ITsSbEnvironment>, propertyname: &::windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AcquireTargetLock(this: &Self::This, targetname: &::windows_core::BSTR, dwtimeout: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ReleaseTargetLock(this: &Self::This, pcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn TestAndSetServerState(this: &Self::This, poolname: &::windows_core::BSTR, serverfqdn: &::windows_core::BSTR, newstate: TARGET_STATE, teststate: TARGET_STATE) -> ::windows_core::Result<TARGET_STATE>;
    fn SetServerWaitingToStart(this: &Self::This, poolname: &::windows_core::BSTR, servername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetServerState(this: &Self::This, poolname: &::windows_core::BSTR, serverfqdn: &::windows_core::BSTR) -> ::windows_core::Result<TARGET_STATE>;
    fn SetServerDrainMode(this: &Self::This, serverfqdn: &::windows_core::BSTR, drainmode: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbResourcePluginStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbResourcePluginStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryTarget(this, ::core::mem::transmute(&targetname), ::core::mem::transmute(&farmname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QuerySessionBySessionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsessionid: u32, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySessionBySessionId(this, ::core::mem::transmute_copy(&dwsessionid), ::core::mem::transmute(&targetname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddTargetToStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTargetToStore(this, ::windows_core::from_raw_borrowed(&ptarget)).into())
        }
        unsafe extern "system" fn AddSessionToStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSessionToStore(this, ::windows_core::from_raw_borrowed(&psession)).into())
        }
        unsafe extern "system" fn AddEnvironmentToStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddEnvironmentToStore(this, ::windows_core::from_raw_borrowed(&penvironment)).into())
        }
        unsafe extern "system" fn RemoveEnvironmentFromStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveEnvironmentFromStore(this, ::core::mem::transmute(&environmentname), ::core::mem::transmute_copy(&bignoreowner)).into())
        }
        unsafe extern "system" fn EnumerateFarms<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateFarms(this, ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into())
        }
        unsafe extern "system" fn QueryEnvironment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppenvironment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryEnvironment(this, ::core::mem::transmute(&environmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenvironment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateEnvironments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateEnvironments(this, ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into())
        }
        unsafe extern "system" fn SaveTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveTarget(this, ::windows_core::from_raw_borrowed(&ptarget), ::core::mem::transmute_copy(&bforcewrite)).into())
        }
        unsafe extern "system" fn SaveEnvironment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void, bforcewrite: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveEnvironment(this, ::windows_core::from_raw_borrowed(&penvironment), ::core::mem::transmute_copy(&bforcewrite)).into())
        }
        unsafe extern "system" fn SaveSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveSession(this, ::windows_core::from_raw_borrowed(&psession)).into())
        }
        unsafe extern "system" fn SetTargetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetProperty(this, ::core::mem::transmute(&targetname), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&pproperty)).into())
        }
        unsafe extern "system" fn SetEnvironmentProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnvironmentProperty(this, ::core::mem::transmute(&environmentname), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&pproperty)).into())
        }
        unsafe extern "system" fn SetTargetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetTargetState(this, ::core::mem::transmute(&targetname), ::core::mem::transmute_copy(&newstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poldstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSessionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sbsession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSessionState(this, ::windows_core::from_raw_borrowed(&sbsession)).into())
        }
        unsafe extern "system" fn EnumerateTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, envname: ::std::mem::MaybeUninit<::windows_core::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateTargets(this, ::core::mem::transmute(&farmname), ::core::mem::transmute(&envname), ::core::mem::transmute_copy(&sortbyfieldid), ::core::mem::transmute(&sortybypropname), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into())
        }
        unsafe extern "system" fn EnumerateSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, userdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, initialprogram: ::std::mem::MaybeUninit<::windows_core::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumerateSessions(this, ::core::mem::transmute(&targetname), ::core::mem::transmute(&username), ::core::mem::transmute(&userdomain), ::core::mem::transmute(&poolname), ::core::mem::transmute(&initialprogram), ::core::mem::transmute_copy(&psessionstate), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&ppval)).into())
        }
        unsafe extern "system" fn GetFarmProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, farmname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFarmProperty(this, ::core::mem::transmute(&farmname), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn DeleteTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, hostname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTarget(this, ::core::mem::transmute(&targetname), ::core::mem::transmute(&hostname)).into())
        }
        unsafe extern "system" fn SetTargetPropertyWithVersionCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetPropertyWithVersionCheck(this, ::windows_core::from_raw_borrowed(&ptarget), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&pproperty)).into())
        }
        unsafe extern "system" fn SetEnvironmentPropertyWithVersionCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penvironment: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnvironmentPropertyWithVersionCheck(this, ::windows_core::from_raw_borrowed(&penvironment), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&pproperty)).into())
        }
        unsafe extern "system" fn AcquireTargetLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwtimeout: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AcquireTargetLock(this, ::core::mem::transmute(&targetname), ::core::mem::transmute_copy(&dwtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseTargetLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseTargetLock(this, ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        unsafe extern "system" fn TestAndSetServerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, serverfqdn: ::std::mem::MaybeUninit<::windows_core::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TestAndSetServerState(this, ::core::mem::transmute(&poolname), ::core::mem::transmute(&serverfqdn), ::core::mem::transmute_copy(&newstate), ::core::mem::transmute_copy(&teststate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinitstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerWaitingToStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, servername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerWaitingToStart(this, ::core::mem::transmute(&poolname), ::core::mem::transmute(&servername)).into())
        }
        unsafe extern "system" fn GetServerState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poolname: ::std::mem::MaybeUninit<::windows_core::BSTR>, serverfqdn: ::std::mem::MaybeUninit<::windows_core::BSTR>, pstate: *mut TARGET_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServerState(this, ::core::mem::transmute(&poolname), ::core::mem::transmute(&serverfqdn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerDrainMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbResourcePluginStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serverfqdn: ::std::mem::MaybeUninit<::windows_core::BSTR>, drainmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerDrainMode(this, ::core::mem::transmute(&serverfqdn), ::core::mem::transmute_copy(&drainmode)).into())
        }
        ITsSbResourcePluginStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryTarget: QueryTarget::<Identity, Impl, OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Identity, Impl, OFFSET>,
            AddTargetToStore: AddTargetToStore::<Identity, Impl, OFFSET>,
            AddSessionToStore: AddSessionToStore::<Identity, Impl, OFFSET>,
            AddEnvironmentToStore: AddEnvironmentToStore::<Identity, Impl, OFFSET>,
            RemoveEnvironmentFromStore: RemoveEnvironmentFromStore::<Identity, Impl, OFFSET>,
            EnumerateFarms: EnumerateFarms::<Identity, Impl, OFFSET>,
            QueryEnvironment: QueryEnvironment::<Identity, Impl, OFFSET>,
            EnumerateEnvironments: EnumerateEnvironments::<Identity, Impl, OFFSET>,
            SaveTarget: SaveTarget::<Identity, Impl, OFFSET>,
            SaveEnvironment: SaveEnvironment::<Identity, Impl, OFFSET>,
            SaveSession: SaveSession::<Identity, Impl, OFFSET>,
            SetTargetProperty: SetTargetProperty::<Identity, Impl, OFFSET>,
            SetEnvironmentProperty: SetEnvironmentProperty::<Identity, Impl, OFFSET>,
            SetTargetState: SetTargetState::<Identity, Impl, OFFSET>,
            SetSessionState: SetSessionState::<Identity, Impl, OFFSET>,
            EnumerateTargets: EnumerateTargets::<Identity, Impl, OFFSET>,
            EnumerateSessions: EnumerateSessions::<Identity, Impl, OFFSET>,
            GetFarmProperty: GetFarmProperty::<Identity, Impl, OFFSET>,
            DeleteTarget: DeleteTarget::<Identity, Impl, OFFSET>,
            SetTargetPropertyWithVersionCheck: SetTargetPropertyWithVersionCheck::<Identity, Impl, OFFSET>,
            SetEnvironmentPropertyWithVersionCheck: SetEnvironmentPropertyWithVersionCheck::<Identity, Impl, OFFSET>,
            AcquireTargetLock: AcquireTargetLock::<Identity, Impl, OFFSET>,
            ReleaseTargetLock: ReleaseTargetLock::<Identity, Impl, OFFSET>,
            TestAndSetServerState: TestAndSetServerState::<Identity, Impl, OFFSET>,
            SetServerWaitingToStart: SetServerWaitingToStart::<Identity, Impl, OFFSET>,
            GetServerState: GetServerState::<Identity, Impl, OFFSET>,
            SetServerDrainMode: SetServerDrainMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITsSbServiceNotification_Impl: ::windows_core::BaseImpl {
    fn NotifyServiceFailure(this: &Self::This) -> ::windows_core::Result<()>;
    fn NotifyServiceSuccess(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITsSbServiceNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbServiceNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbServiceNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifyServiceFailure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbServiceNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyServiceFailure(this).into())
        }
        unsafe extern "system" fn NotifyServiceSuccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbServiceNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyServiceSuccess(this).into())
        }
        ITsSbServiceNotification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotifyServiceFailure: NotifyServiceFailure::<Identity, Impl, OFFSET>,
            NotifyServiceSuccess: NotifyServiceSuccess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbSession_Impl: ::windows_core::BaseImpl {
    fn SessionId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn TargetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTargetName(this: &Self::This, targetname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Username(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Domain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn State(this: &Self::This) -> ::windows_core::Result<TSSESSION_STATE>;
    fn SetState(this: &Self::This, state: TSSESSION_STATE) -> ::windows_core::Result<()>;
    fn CreateTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn SetCreateTime(this: &Self::This, time: &super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn DisconnectTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn SetDisconnectTime(this: &Self::This, time: &super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn InitialProgram(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetInitialProgram(this: &Self::This, application: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientDisplay(this: &Self::This) -> ::windows_core::Result<CLIENT_DISPLAY>;
    fn SetClientDisplay(this: &Self::This, pclientdisplay: &CLIENT_DISPLAY) -> ::windows_core::Result<()>;
    fn ProtocolType(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetProtocolType(this: &Self::This, val: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITsSbSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SessionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TargetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetName(this, ::core::mem::transmute(&targetname)).into())
        }
        unsafe extern "system" fn Username<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Username(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(username, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Domain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Domain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(domain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: TSSESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCreateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreateTime(this, ::core::mem::transmute(&time)).into())
        }
        unsafe extern "system" fn DisconnectTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisconnectTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisconnectTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisconnectTime(this, ::core::mem::transmute(&time)).into())
        }
        unsafe extern "system" fn InitialProgram<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, app: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitialProgram(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(app, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInitialProgram<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, application: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialProgram(this, ::core::mem::transmute(&application)).into())
        }
        unsafe extern "system" fn ClientDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientDisplay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclientdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientDisplay(this, ::core::mem::transmute(&pclientdisplay)).into())
        }
        unsafe extern "system" fn ProtocolType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProtocolType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProtocolType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtocolType(this, ::core::mem::transmute_copy(&val)).into())
        }
        ITsSbSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SessionId: SessionId::<Identity, Impl, OFFSET>,
            TargetName: TargetName::<Identity, Impl, OFFSET>,
            SetTargetName: SetTargetName::<Identity, Impl, OFFSET>,
            Username: Username::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            SetCreateTime: SetCreateTime::<Identity, Impl, OFFSET>,
            DisconnectTime: DisconnectTime::<Identity, Impl, OFFSET>,
            SetDisconnectTime: SetDisconnectTime::<Identity, Impl, OFFSET>,
            InitialProgram: InitialProgram::<Identity, Impl, OFFSET>,
            SetInitialProgram: SetInitialProgram::<Identity, Impl, OFFSET>,
            ClientDisplay: ClientDisplay::<Identity, Impl, OFFSET>,
            SetClientDisplay: SetClientDisplay::<Identity, Impl, OFFSET>,
            ProtocolType: ProtocolType::<Identity, Impl, OFFSET>,
            SetProtocolType: SetProtocolType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTarget_Impl: ::windows_core::BaseImpl {
    fn TargetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTargetName(this: &Self::This, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FarmName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFarmName(this: &Self::This, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TargetFQDN(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTargetFQDN(this: &Self::This, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TargetNetbios(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTargetNetbios(this: &Self::This, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_IpAddresses(this: &Self::This, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows_core::Result<()>;
    fn put_IpAddresses(this: &Self::This, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows_core::Result<()>;
    fn TargetState(this: &Self::This) -> ::windows_core::Result<TARGET_STATE>;
    fn SetTargetState(this: &Self::This, state: TARGET_STATE) -> ::windows_core::Result<()>;
    fn TargetPropertySet(this: &Self::This) -> ::windows_core::Result<ITsSbTargetPropertySet>;
    fn SetTargetPropertySet(this: &Self::This, pval: ::core::option::Option<&ITsSbTargetPropertySet>) -> ::windows_core::Result<()>;
    fn EnvironmentName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEnvironmentName(this: &Self::This, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NumSessions(this: &Self::This) -> ::windows_core::Result<u32>;
    fn NumPendingConnections(this: &Self::This) -> ::windows_core::Result<u32>;
    fn TargetLoad(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TargetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetName(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn FarmName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FarmName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFarmName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFarmName(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn TargetFQDN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetfqdnname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetFQDN(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetfqdnname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetFQDN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetFQDN(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn TargetNetbios<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetnetbiosname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetNetbios(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetnetbiosname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetNetbios<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetNetbios(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn get_IpAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_IpAddresses(this, ::core::mem::transmute_copy(&sockaddr), ::core::mem::transmute_copy(&numaddresses)).into())
        }
        unsafe extern "system" fn put_IpAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_IpAddresses(this, ::core::mem::transmute_copy(&sockaddr), ::core::mem::transmute_copy(&numaddresses)).into())
        }
        unsafe extern "system" fn TargetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut TARGET_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: TARGET_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn TargetPropertySet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetPropertySet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTargetPropertySet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTargetPropertySet(this, ::windows_core::from_raw_borrowed(&pval)).into())
        }
        unsafe extern "system" fn EnvironmentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnvironmentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnvironmentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnvironmentName(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn NumSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumsessions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumSessions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumsessions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumPendingConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumpendingconnections: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumPendingConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumpendingconnections, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TargetLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetload: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetLoad(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptargetload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITsSbTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TargetName: TargetName::<Identity, Impl, OFFSET>,
            SetTargetName: SetTargetName::<Identity, Impl, OFFSET>,
            FarmName: FarmName::<Identity, Impl, OFFSET>,
            SetFarmName: SetFarmName::<Identity, Impl, OFFSET>,
            TargetFQDN: TargetFQDN::<Identity, Impl, OFFSET>,
            SetTargetFQDN: SetTargetFQDN::<Identity, Impl, OFFSET>,
            TargetNetbios: TargetNetbios::<Identity, Impl, OFFSET>,
            SetTargetNetbios: SetTargetNetbios::<Identity, Impl, OFFSET>,
            get_IpAddresses: get_IpAddresses::<Identity, Impl, OFFSET>,
            put_IpAddresses: put_IpAddresses::<Identity, Impl, OFFSET>,
            TargetState: TargetState::<Identity, Impl, OFFSET>,
            SetTargetState: SetTargetState::<Identity, Impl, OFFSET>,
            TargetPropertySet: TargetPropertySet::<Identity, Impl, OFFSET>,
            SetTargetPropertySet: SetTargetPropertySet::<Identity, Impl, OFFSET>,
            EnvironmentName: EnvironmentName::<Identity, Impl, OFFSET>,
            SetEnvironmentName: SetEnvironmentName::<Identity, Impl, OFFSET>,
            NumSessions: NumSessions::<Identity, Impl, OFFSET>,
            NumPendingConnections: NumPendingConnections::<Identity, Impl, OFFSET>,
            TargetLoad: TargetLoad::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbTargetPropertySet_Impl: ::windows_core::BaseImpl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITsSbTargetPropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPropertySet);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTargetPropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbTargetPropertySet {
    const VTABLE: Self::Vtable = { ITsSbTargetPropertySet_Vtbl { base__: <ITsSbPropertySet as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITsSbTaskInfo_Impl: ::windows_core::BaseImpl {
    fn TargetId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn EndTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn Deadline(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn Identifier(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Context(this: &Self::This) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn Plugin(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Status(this: &Self::This) -> ::windows_core::Result<RDV_TASK_STATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITsSbTaskInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbTaskInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TargetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstarttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pendtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Deadline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Deadline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdeadline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Identifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Identifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Context<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Context(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Plugin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplugin: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Plugin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplugin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITsSbTaskInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TargetId: TargetId::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            EndTime: EndTime::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
            Identifier: Identifier::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
            Plugin: Plugin::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTaskPlugin_Impl: ::windows_core::BaseImpl + ITsSbPlugin_Impl {
    fn InitializeTaskPlugin(this: &Self::This, pitssbtaskpluginnotifysink: ::core::option::Option<&ITsSbTaskPluginNotifySink>) -> ::windows_core::Result<()>;
    fn SetTaskQueue(this: &Self::This, pszhostname: &::windows_core::BSTR, sbtaskinfosize: u32, pitssbtaskinfo: *const ::core::option::Option<ITsSbTaskInfo>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for ITsSbTaskPlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbPlugin);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbTaskPlugin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeTaskPlugin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitssbtaskpluginnotifysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeTaskPlugin(this, ::windows_core::from_raw_borrowed(&pitssbtaskpluginnotifysink)).into())
        }
        unsafe extern "system" fn SetTaskQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszhostname: ::std::mem::MaybeUninit<::windows_core::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTaskQueue(this, ::core::mem::transmute(&pszhostname), ::core::mem::transmute_copy(&sbtaskinfosize), ::core::mem::transmute_copy(&pitssbtaskinfo)).into())
        }
        ITsSbTaskPlugin_Vtbl {
            base__: <ITsSbPlugin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeTaskPlugin: InitializeTaskPlugin::<Identity, Impl, OFFSET>,
            SetTaskQueue: SetTaskQueue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITsSbTaskPluginNotifySink_Impl: ::windows_core::BaseImpl + ITsSbBaseNotifySink_Impl {
    fn OnSetTaskTime(this: &Self::This, sztargetname: &::windows_core::BSTR, taskstarttime: &super::super::Foundation::FILETIME, taskendtime: &super::super::Foundation::FILETIME, taskdeadline: &super::super::Foundation::FILETIME, sztasklabel: &::windows_core::BSTR, sztaskidentifier: &::windows_core::BSTR, sztaskplugin: &::windows_core::BSTR, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn OnDeleteTaskTime(this: &Self::This, sztargetname: &::windows_core::BSTR, sztaskidentifier: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnUpdateTaskStatus(this: &Self::This, sztargetname: &::windows_core::BSTR, taskidentifier: &::windows_core::BSTR, taskstatus: RDV_TASK_STATUS) -> ::windows_core::Result<()>;
    fn OnReportTasks(this: &Self::This, szhostname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITsSbTaskPluginNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITsSbBaseNotifySink);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITsSbTaskPluginNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSetTaskTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sztargetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: ::std::mem::MaybeUninit<::windows_core::BSTR>, sztaskidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, sztaskplugin: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetTaskTime(this, ::core::mem::transmute(&sztargetname), ::core::mem::transmute(&taskstarttime), ::core::mem::transmute(&taskendtime), ::core::mem::transmute(&taskdeadline), ::core::mem::transmute(&sztasklabel), ::core::mem::transmute(&sztaskidentifier), ::core::mem::transmute(&sztaskplugin), ::core::mem::transmute_copy(&dwtaskstatus), ::core::mem::transmute_copy(&sacontext)).into())
        }
        unsafe extern "system" fn OnDeleteTaskTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sztargetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, sztaskidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDeleteTaskTime(this, ::core::mem::transmute(&sztargetname), ::core::mem::transmute(&sztaskidentifier)).into())
        }
        unsafe extern "system" fn OnUpdateTaskStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sztargetname: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskidentifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUpdateTaskStatus(this, ::core::mem::transmute(&sztargetname), ::core::mem::transmute(&taskidentifier), ::core::mem::transmute_copy(&taskstatus)).into())
        }
        unsafe extern "system" fn OnReportTasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szhostname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReportTasks(this, ::core::mem::transmute(&szhostname)).into())
        }
        ITsSbTaskPluginNotifySink_Vtbl {
            base__: <ITsSbBaseNotifySink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSetTaskTime: OnSetTaskTime::<Identity, Impl, OFFSET>,
            OnDeleteTaskTime: OnDeleteTaskTime::<Identity, Impl, OFFSET>,
            OnUpdateTaskStatus: OnUpdateTaskStatus::<Identity, Impl, OFFSET>,
            OnReportTasks: OnReportTasks::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsEnhancedFastReconnectArbitrator_Impl: ::windows_core::BaseImpl {
    fn GetSessionForEnhancedFastReconnect(this: &Self::This, psessionidarray: *const i32, dwsessioncount: u32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IWRdsEnhancedFastReconnectArbitrator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsEnhancedFastReconnectArbitrator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsEnhancedFastReconnectArbitrator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSessionForEnhancedFastReconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsEnhancedFastReconnectArbitrator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSessionForEnhancedFastReconnect(this, ::core::mem::transmute_copy(&psessionidarray), ::core::mem::transmute_copy(&dwsessioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presultsessionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWRdsEnhancedFastReconnectArbitrator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSessionForEnhancedFastReconnect: GetSessionForEnhancedFastReconnect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsGraphicsChannel_Impl: ::windows_core::BaseImpl {
    fn Write(this: &Self::This, cbsize: u32, pbuffer: *const u8, pcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, pchannelevents: ::core::option::Option<&IWRdsGraphicsChannelEvents>, popencontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWRdsGraphicsChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsGraphicsChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer), ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannelevents: *mut ::core::ffi::c_void, popencontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&pchannelevents), ::windows_core::from_raw_borrowed(&popencontext)).into())
        }
        IWRdsGraphicsChannel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Write: Write::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsGraphicsChannelEvents_Impl: ::windows_core::BaseImpl {
    fn OnDataReceived(this: &Self::This, cbsize: u32, pbuffer: *const u8) -> ::windows_core::Result<()>;
    fn OnClose(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnChannelOpened(this: &Self::This, openresult: ::windows_core::HRESULT, popencontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnDataSent(this: &Self::This, pwritecontext: ::core::option::Option<&::windows_core::IUnknown>, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows_core::Result<()>;
    fn OnMetricsUpdate(this: &Self::This, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWRdsGraphicsChannelEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsGraphicsChannelEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDataReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataReceived(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer)).into())
        }
        unsafe extern "system" fn OnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClose(this).into())
        }
        unsafe extern "system" fn OnChannelOpened<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, openresult: ::windows_core::HRESULT, popencontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChannelOpened(this, ::core::mem::transmute_copy(&openresult), ::windows_core::from_raw_borrowed(&popencontext)).into())
        }
        unsafe extern "system" fn OnDataSent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwritecontext: *mut ::core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataSent(this, ::windows_core::from_raw_borrowed(&pwritecontext), ::core::mem::transmute_copy(&bcancelled), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer)).into())
        }
        unsafe extern "system" fn OnMetricsUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMetricsUpdate(this, ::core::mem::transmute_copy(&bandwidth), ::core::mem::transmute_copy(&rtt), ::core::mem::transmute_copy(&lastsentbyteindex)).into())
        }
        IWRdsGraphicsChannelEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDataReceived: OnDataReceived::<Identity, Impl, OFFSET>,
            OnClose: OnClose::<Identity, Impl, OFFSET>,
            OnChannelOpened: OnChannelOpened::<Identity, Impl, OFFSET>,
            OnDataSent: OnDataSent::<Identity, Impl, OFFSET>,
            OnMetricsUpdate: OnMetricsUpdate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsGraphicsChannelManager_Impl: ::windows_core::BaseImpl {
    fn CreateChannel(this: &Self::This, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> ::windows_core::Result<IWRdsGraphicsChannel>;
}
impl ::windows_core::Iids for IWRdsGraphicsChannelManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsGraphicsChannelManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsGraphicsChannelManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateChannel(this, ::core::mem::transmute_copy(&pszchannelname), ::core::mem::transmute_copy(&channeltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvirtualchannel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWRdsGraphicsChannelManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateChannel: CreateChannel::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolConnection_Impl: ::windows_core::BaseImpl {
    fn GetLogonErrorRedirector(this: &Self::This) -> ::windows_core::Result<IWRdsProtocolLogonErrorRedirector>;
    fn AcceptConnection(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetClientData(this: &Self::This, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::Result<()>;
    fn GetClientMonitorData(this: &Self::This, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows_core::Result<()>;
    fn GetUserCredentials(this: &Self::This, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::Result<()>;
    fn GetLicenseConnection(this: &Self::This) -> ::windows_core::Result<IWRdsProtocolLicenseConnection>;
    fn AuthenticateClientToSession(this: &Self::This, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn NotifySessionId(this: &Self::This, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()>;
    fn GetInputHandles(this: &Self::This, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()>;
    fn GetVideoHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE_PTR>;
    fn ConnectNotify(this: &Self::This, sessionid: u32) -> ::windows_core::Result<()>;
    fn IsUserAllowedToLogon(this: &Self::This, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: &::windows_core::PCWSTR, pusername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SessionArbitrationEnumeration(this: &Self::This, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::Result<()>;
    fn LogonNotify(this: &Self::This, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: &::windows_core::PCWSTR, wszdomainname: &::windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::Result<()>;
    fn PreDisconnect(this: &Self::This, disconnectreason: u32) -> ::windows_core::Result<()>;
    fn DisconnectNotify(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProtocolStatus(this: &Self::This, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::Result<()>;
    fn GetLastInputTime(this: &Self::This) -> ::windows_core::Result<u64>;
    fn SetErrorInfo(this: &Self::This, ulerror: u32) -> ::windows_core::Result<()>;
    fn CreateVirtualChannel(this: &Self::This, szendpointname: &::windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32) -> ::windows_core::Result<usize>;
    fn QueryProperty(this: &Self::This, querytype: &::windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::Result<()>;
    fn GetShadowConnection(this: &Self::This) -> ::windows_core::Result<IWRdsProtocolShadowConnection>;
    fn NotifyCommandProcessCreated(this: &Self::This, sessionid: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWRdsProtocolConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLogonErrorRedirector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLogonErrorRedirector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplogonerrorredir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AcceptConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcceptConnection(this).into())
        }
        unsafe extern "system" fn GetClientData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClientData(this, ::core::mem::transmute_copy(&pclientdata)).into())
        }
        unsafe extern "system" fn GetClientMonitorData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClientMonitorData(this, ::core::mem::transmute_copy(&pnummonitors), ::core::mem::transmute_copy(&pprimarymonitor)).into())
        }
        unsafe extern "system" fn GetUserCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserCredentials(this, ::core::mem::transmute_copy(&pusercreds)).into())
        }
        unsafe extern "system" fn GetLicenseConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLicenseConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplicenseconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AuthenticateClientToSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AuthenticateClientToSession(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn NotifySessionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionId(this, ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&sessionhandle)).into())
        }
        unsafe extern "system" fn GetInputHandles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputHandles(this, ::core::mem::transmute_copy(&pkeyboardhandle), ::core::mem::transmute_copy(&pmousehandle), ::core::mem::transmute_copy(&pbeephandle)).into())
        }
        unsafe extern "system" fn GetVideoHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVideoHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvideohandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectNotify(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows_core::PCWSTR, pusername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUserAllowedToLogon(this, ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&usertoken), ::core::mem::transmute(&pdomainname), ::core::mem::transmute(&pusername)).into())
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SessionArbitrationEnumeration(this, ::core::mem::transmute_copy(&husertoken), ::core::mem::transmute_copy(&bsinglesessionperuserenabled), ::core::mem::transmute_copy(&psessionidarray), ::core::mem::transmute_copy(&pdwsessionidentifiercount)).into())
        }
        unsafe extern "system" fn LogonNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows_core::PCWSTR, wszdomainname: ::windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogonNotify(this, ::core::mem::transmute_copy(&hclienttoken), ::core::mem::transmute(&wszusername), ::core::mem::transmute(&wszdomainname), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&pwrdsconnectionsettings)).into())
        }
        unsafe extern "system" fn PreDisconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disconnectreason: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreDisconnect(this, ::core::mem::transmute_copy(&disconnectreason)).into())
        }
        unsafe extern "system" fn DisconnectNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectNotify(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetProtocolStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProtocolStatus(this, ::core::mem::transmute_copy(&pprotocolstatus)).into())
        }
        unsafe extern "system" fn GetLastInputTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastInputTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plastinputtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorInfo(this, ::core::mem::transmute_copy(&ulerror)).into())
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szendpointname: ::windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVirtualChannel(this, ::core::mem::transmute(&szendpointname), ::core::mem::transmute_copy(&bstatic), ::core::mem::transmute_copy(&requestedpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phchannel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querytype: ::windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryProperty(this, ::core::mem::transmute(&querytype), ::core::mem::transmute_copy(&ulnumentriesin), ::core::mem::transmute_copy(&ulnumentriesout), ::core::mem::transmute_copy(&ppropertyentriesin), ::core::mem::transmute_copy(&ppropertyentriesout)).into())
        }
        unsafe extern "system" fn GetShadowConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetShadowConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshadowconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyCommandProcessCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyCommandProcessCreated(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        IWRdsProtocolConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Identity, Impl, OFFSET>,
            AcceptConnection: AcceptConnection::<Identity, Impl, OFFSET>,
            GetClientData: GetClientData::<Identity, Impl, OFFSET>,
            GetClientMonitorData: GetClientMonitorData::<Identity, Impl, OFFSET>,
            GetUserCredentials: GetUserCredentials::<Identity, Impl, OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Identity, Impl, OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Identity, Impl, OFFSET>,
            NotifySessionId: NotifySessionId::<Identity, Impl, OFFSET>,
            GetInputHandles: GetInputHandles::<Identity, Impl, OFFSET>,
            GetVideoHandle: GetVideoHandle::<Identity, Impl, OFFSET>,
            ConnectNotify: ConnectNotify::<Identity, Impl, OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Identity, Impl, OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Identity, Impl, OFFSET>,
            LogonNotify: LogonNotify::<Identity, Impl, OFFSET>,
            PreDisconnect: PreDisconnect::<Identity, Impl, OFFSET>,
            DisconnectNotify: DisconnectNotify::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Identity, Impl, OFFSET>,
            GetLastInputTime: GetLastInputTime::<Identity, Impl, OFFSET>,
            SetErrorInfo: SetErrorInfo::<Identity, Impl, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, Impl, OFFSET>,
            QueryProperty: QueryProperty::<Identity, Impl, OFFSET>,
            GetShadowConnection: GetShadowConnection::<Identity, Impl, OFFSET>,
            NotifyCommandProcessCreated: NotifyCommandProcessCreated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsProtocolConnectionCallback_Impl: ::windows_core::BaseImpl {
    fn OnReady(this: &Self::This) -> ::windows_core::Result<()>;
    fn BrokenConnection(this: &Self::This, reason: u32, source: u32) -> ::windows_core::Result<()>;
    fn StopScreenUpdates(this: &Self::This) -> ::windows_core::Result<()>;
    fn RedrawWindow(this: &Self::This, rect: *const WTS_SMALL_RECT) -> ::windows_core::Result<()>;
    fn GetConnectionId(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IWRdsProtocolConnectionCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolConnectionCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnReady<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReady(this).into())
        }
        unsafe extern "system" fn BrokenConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BrokenConnection(this, ::core::mem::transmute_copy(&reason), ::core::mem::transmute_copy(&source)).into())
        }
        unsafe extern "system" fn StopScreenUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopScreenUpdates(this).into())
        }
        unsafe extern "system" fn RedrawWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RedrawWindow(this, ::core::mem::transmute_copy(&rect)).into())
        }
        unsafe extern "system" fn GetConnectionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectionid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWRdsProtocolConnectionCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnReady: OnReady::<Identity, Impl, OFFSET>,
            BrokenConnection: BrokenConnection::<Identity, Impl, OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Identity, Impl, OFFSET>,
            RedrawWindow: RedrawWindow::<Identity, Impl, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsProtocolConnectionSettings_Impl: ::windows_core::BaseImpl {
    fn SetConnectionSetting(this: &Self::This, propertyid: &::windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows_core::Result<()>;
    fn GetConnectionSetting(this: &Self::This, propertyid: &::windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWRdsProtocolConnectionSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolConnectionSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetConnectionSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectionSetting(this, ::core::mem::transmute(&propertyid), ::core::mem::transmute_copy(&ppropertyentriesin)).into())
        }
        unsafe extern "system" fn GetConnectionSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolConnectionSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectionSetting(this, ::core::mem::transmute(&propertyid), ::core::mem::transmute_copy(&ppropertyentriesout)).into())
        }
        IWRdsProtocolConnectionSettings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetConnectionSetting: SetConnectionSetting::<Identity, Impl, OFFSET>,
            GetConnectionSetting: GetConnectionSetting::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolLicenseConnection_Impl: ::windows_core::BaseImpl {
    fn RequestLicensingCapabilities(this: &Self::This, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::Result<()>;
    fn SendClientLicense(this: &Self::This, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_core::Result<()>;
    fn RequestClientLicense(this: &Self::This, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::Result<()>;
    fn ProtocolComplete(this: &Self::This, ulcomplete: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWRdsProtocolLicenseConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolLicenseConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestLicensingCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestLicensingCapabilities(this, ::core::mem::transmute_copy(&pplicensecapabilities), ::core::mem::transmute_copy(&pcblicensecapabilities)).into())
        }
        unsafe extern "system" fn SendClientLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendClientLicense(this, ::core::mem::transmute_copy(&pclientlicense), ::core::mem::transmute_copy(&cbclientlicense)).into())
        }
        unsafe extern "system" fn RequestClientLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestClientLicense(this, ::core::mem::transmute_copy(&reserve1), ::core::mem::transmute_copy(&reserve2), ::core::mem::transmute_copy(&ppclientlicense), ::core::mem::transmute_copy(&pcbclientlicense)).into())
        }
        unsafe extern "system" fn ProtocolComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProtocolComplete(this, ::core::mem::transmute_copy(&ulcomplete)).into())
        }
        IWRdsProtocolLicenseConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Identity, Impl, OFFSET>,
            SendClientLicense: SendClientLicense::<Identity, Impl, OFFSET>,
            RequestClientLicense: RequestClientLicense::<Identity, Impl, OFFSET>,
            ProtocolComplete: ProtocolComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsProtocolListener_Impl: ::windows_core::BaseImpl {
    fn GetSettings(this: &Self::This, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> ::windows_core::Result<WRDS_LISTENER_SETTINGS>;
    fn StartListen(this: &Self::This, pcallback: ::core::option::Option<&IWRdsProtocolListenerCallback>) -> ::windows_core::Result<()>;
    fn StopListen(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWRdsProtocolListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSettings(this, ::core::mem::transmute_copy(&wrdslistenersettinglevel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwrdslistenersettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartListen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartListen(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn StopListen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopListen(this).into())
        }
        IWRdsProtocolListener_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSettings: GetSettings::<Identity, Impl, OFFSET>,
            StartListen: StartListen::<Identity, Impl, OFFSET>,
            StopListen: StopListen::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolListenerCallback_Impl: ::windows_core::BaseImpl {
    fn OnConnected(this: &Self::This, pconnection: ::core::option::Option<&IWRdsProtocolConnection>, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> ::windows_core::Result<IWRdsProtocolConnectionCallback>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWRdsProtocolListenerCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolListenerCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolListenerCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolListenerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnConnected(this, ::windows_core::from_raw_borrowed(&pconnection), ::core::mem::transmute_copy(&pwrdsconnectionsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWRdsProtocolListenerCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnConnected: OnConnected::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsProtocolLogonErrorRedirector_Impl: ::windows_core::BaseImpl {
    fn OnBeginPainting(this: &Self::This) -> ::windows_core::Result<()>;
    fn RedirectStatus(this: &Self::This, pszmessage: &::windows_core::PCWSTR) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(this: &Self::This, pszcaption: &::windows_core::PCWSTR, pszmessage: &::windows_core::PCWSTR, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(this: &Self::This, ntsstatus: i32, ntssubstatus: i32, pszcaption: &::windows_core::PCWSTR, pszmessage: &::windows_core::PCWSTR, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
impl ::windows_core::Iids for IWRdsProtocolLogonErrorRedirector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolLogonErrorRedirector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnBeginPainting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBeginPainting(this).into())
        }
        unsafe extern "system" fn RedirectStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmessage: ::windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectStatus(this, ::core::mem::transmute(&pszmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RedirectMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectMessage(this, ::core::mem::transmute(&pszcaption), ::core::mem::transmute(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RedirectLogonError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectLogonError(this, ::core::mem::transmute_copy(&ntsstatus), ::core::mem::transmute_copy(&ntssubstatus), ::core::mem::transmute(&pszcaption), ::core::mem::transmute(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWRdsProtocolLogonErrorRedirector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnBeginPainting: OnBeginPainting::<Identity, Impl, OFFSET>,
            RedirectStatus: RedirectStatus::<Identity, Impl, OFFSET>,
            RedirectMessage: RedirectMessage::<Identity, Impl, OFFSET>,
            RedirectLogonError: RedirectLogonError::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolManager_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, piwrdssettings: ::core::option::Option<&IWRdsProtocolSettings>, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::Result<()>;
    fn CreateListener(this: &Self::This, wszlistenername: &::windows_core::PCWSTR) -> ::windows_core::Result<IWRdsProtocolListener>;
    fn NotifyServiceStateChange(this: &Self::This, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::Result<()>;
    fn NotifySessionOfServiceStart(this: &Self::This, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn NotifySessionOfServiceStop(this: &Self::This, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn NotifySessionStateChange(this: &Self::This, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::Result<()>;
    fn NotifySettingsChange(this: &Self::This, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::Result<()>;
    fn Uninitialize(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWRdsProtocolManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwrdssettings: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&piwrdssettings), ::core::mem::transmute_copy(&pwrdssettings)).into())
        }
        unsafe extern "system" fn CreateListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszlistenername: ::windows_core::PCWSTR, pprotocollistener: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateListener(this, ::core::mem::transmute(&wszlistenername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprotocollistener, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyServiceStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyServiceStateChange(this, ::core::mem::transmute_copy(&ptsservicestatechange)).into())
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionOfServiceStart(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionOfServiceStop(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn NotifySessionStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionStateChange(this, ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&eventid)).into())
        }
        unsafe extern "system" fn NotifySettingsChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySettingsChange(this, ::core::mem::transmute_copy(&pwrdssettings)).into())
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this).into())
        }
        IWRdsProtocolManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateListener: CreateListener::<Identity, Impl, OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Identity, Impl, OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Identity, Impl, OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Identity, Impl, OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Identity, Impl, OFFSET>,
            NotifySettingsChange: NotifySettingsChange::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolSettings_Impl: ::windows_core::BaseImpl {
    fn GetSettings(this: &Self::This, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows_core::Result<()>;
    fn MergeSettings(this: &Self::This, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWRdsProtocolSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSettings(this, ::core::mem::transmute_copy(&wrdssettingtype), ::core::mem::transmute_copy(&wrdssettinglevel), ::core::mem::transmute_copy(&pwrdssettings)).into())
        }
        unsafe extern "system" fn MergeSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MergeSettings(this, ::core::mem::transmute_copy(&pwrdssettings), ::core::mem::transmute_copy(&wrdsconnectionsettinglevel), ::core::mem::transmute_copy(&pwrdsconnectionsettings)).into())
        }
        IWRdsProtocolSettings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSettings: GetSettings::<Identity, Impl, OFFSET>,
            MergeSettings: MergeSettings::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsProtocolShadowCallback_Impl: ::windows_core::BaseImpl {
    fn StopShadow(this: &Self::This) -> ::windows_core::Result<()>;
    fn InvokeTargetShadow(this: &Self::This, ptargetservername: &::windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWRdsProtocolShadowCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolShadowCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolShadowCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StopShadow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolShadowCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopShadow(this).into())
        }
        unsafe extern "system" fn InvokeTargetShadow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolShadowCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::InvokeTargetShadow(this, ::core::mem::transmute(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute(&pclientname)).into()
            })
        }
        IWRdsProtocolShadowCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StopShadow: StopShadow::<Identity, Impl, OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWRdsProtocolShadowConnection_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This, ptargetservername: &::windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::core::option::Option<&IWRdsProtocolShadowCallback>) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn DoTarget(this: &Self::This, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWRdsProtocolShadowConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolShadowConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsProtocolShadowConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolShadowConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&hotkeyvk), ::core::mem::transmute_copy(&hotkeymodifiers), ::windows_core::from_raw_borrowed(&pshadowcallback)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolShadowConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn DoTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsProtocolShadowConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoTarget(this, ::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute(&pclientname)).into())
        }
        IWRdsProtocolShadowConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            DoTarget: DoTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsWddmIddProps_Impl: ::windows_core::BaseImpl {
    fn GetHardwareId(this: &Self::This, pdisplaydriverhardwareid: &::windows_core::PCWSTR, count: u32) -> ::windows_core::Result<()>;
    fn OnDriverLoad(this: &Self::This, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()>;
    fn OnDriverUnload(this: &Self::This, sessionid: u32) -> ::windows_core::Result<()>;
    fn EnableWddmIdd(this: &Self::This, enabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWRdsWddmIddProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsWddmIddProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWRdsWddmIddProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHardwareId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsWddmIddProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplaydriverhardwareid: ::windows_core::PCWSTR, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHardwareId(this, ::core::mem::transmute(&pdisplaydriverhardwareid), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn OnDriverLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsWddmIddProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDriverLoad(this, ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&driverhandle)).into())
        }
        unsafe extern "system" fn OnDriverUnload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsWddmIddProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDriverUnload(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn EnableWddmIdd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWRdsWddmIddProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableWddmIdd(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        IWRdsWddmIddProps_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHardwareId: GetHardwareId::<Identity, Impl, OFFSET>,
            OnDriverLoad: OnDriverLoad::<Identity, Impl, OFFSET>,
            OnDriverUnload: OnDriverUnload::<Identity, Impl, OFFSET>,
            EnableWddmIdd: EnableWddmIdd::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSBitmapRenderService_Impl: ::windows_core::BaseImpl {
    fn GetMappedRenderer(this: &Self::This, mappingid: u64, pmappedrenderercallback: ::core::option::Option<&IWTSBitmapRendererCallback>) -> ::windows_core::Result<IWTSBitmapRenderer>;
}
impl ::windows_core::Iids for IWTSBitmapRenderService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRenderService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSBitmapRenderService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMappedRenderer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRenderService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mappingid: u64, pmappedrenderercallback: *mut ::core::ffi::c_void, ppmappedrenderer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMappedRenderer(this, ::core::mem::transmute_copy(&mappingid), ::windows_core::from_raw_borrowed(&pmappedrenderercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmappedrenderer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWTSBitmapRenderService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMappedRenderer: GetMappedRenderer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSBitmapRenderer_Impl: ::windows_core::BaseImpl {
    fn Render(this: &Self::This, imageformat: &::windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows_core::Result<()>;
    fn GetRendererStatistics(this: &Self::This) -> ::windows_core::Result<BITMAP_RENDERER_STATISTICS>;
    fn RemoveMapping(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSBitmapRenderer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRenderer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSBitmapRenderer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Render<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageformat: ::windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Render(this, ::core::mem::transmute(&imageformat), ::core::mem::transmute_copy(&dwwidth), ::core::mem::transmute_copy(&dwheight), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbimagebuffer), ::core::mem::transmute_copy(&pimagebuffer)).into())
        }
        unsafe extern "system" fn GetRendererStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRendererStatistics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatistics, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveMapping(this).into())
        }
        IWTSBitmapRenderer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Render: Render::<Identity, Impl, OFFSET>,
            GetRendererStatistics: GetRendererStatistics::<Identity, Impl, OFFSET>,
            RemoveMapping: RemoveMapping::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSBitmapRendererCallback_Impl: ::windows_core::BaseImpl {
    fn OnTargetSizeChanged(this: &Self::This, rcnewsize: &super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWTSBitmapRendererCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRendererCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSBitmapRendererCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTargetSizeChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSBitmapRendererCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTargetSizeChanged(this, ::core::mem::transmute(&rcnewsize)).into())
        }
        IWTSBitmapRendererCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTargetSizeChanged: OnTargetSizeChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWTSListener_Impl: ::windows_core::BaseImpl {
    fn GetConfiguration(this: &Self::This) -> ::windows_core::Result<super::Com::StructuredStorage::IPropertyBag>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for IWTSListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConfiguration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWTSListener_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConfiguration: GetConfiguration::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSListenerCallback_Impl: ::windows_core::BaseImpl {
    fn OnNewChannelConnection(this: &Self::This, pchannel: ::core::option::Option<&IWTSVirtualChannel>, data: &::windows_core::BSTR, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::core::option::Option<IWTSVirtualChannelCallback>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWTSListenerCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSListenerCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSListenerCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnNewChannelConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSListenerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNewChannelConnection(this, ::windows_core::from_raw_borrowed(&pchannel), ::core::mem::transmute(&data), ::core::mem::transmute_copy(&pbaccept), ::core::mem::transmute_copy(&ppcallback)).into())
        }
        IWTSListenerCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnNewChannelConnection: OnNewChannelConnection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSPlugin_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pchannelmgr: ::core::option::Option<&IWTSVirtualChannelManager>) -> ::windows_core::Result<()>;
    fn Connected(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disconnected(this: &Self::This, dwdisconnectcode: u32) -> ::windows_core::Result<()>;
    fn Terminated(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSPlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSPlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSPlugin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchannelmgr: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pchannelmgr)).into())
        }
        unsafe extern "system" fn Connected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connected(this).into())
        }
        unsafe extern "system" fn Disconnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdisconnectcode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnected(this, ::core::mem::transmute_copy(&dwdisconnectcode)).into())
        }
        unsafe extern "system" fn Terminated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminated(this).into())
        }
        IWTSPlugin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Connected: Connected::<Identity, Impl, OFFSET>,
            Disconnected: Disconnected::<Identity, Impl, OFFSET>,
            Terminated: Terminated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSPluginServiceProvider_Impl: ::windows_core::BaseImpl {
    fn GetService(this: &Self::This, serviceid: &::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IWTSPluginServiceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSPluginServiceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSPluginServiceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSPluginServiceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::windows_core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetService(this, ::core::mem::transmute(&serviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWTSPluginServiceProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetService: GetService::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolConnection_Impl: ::windows_core::BaseImpl {
    fn GetLogonErrorRedirector(this: &Self::This) -> ::windows_core::Result<IWTSProtocolLogonErrorRedirector>;
    fn SendPolicyData(this: &Self::This, ppolicydata: *const WTS_POLICY_DATA) -> ::windows_core::Result<()>;
    fn AcceptConnection(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetClientData(this: &Self::This, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::Result<()>;
    fn GetUserCredentials(this: &Self::This, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::Result<()>;
    fn GetLicenseConnection(this: &Self::This) -> ::windows_core::Result<IWTSProtocolLicenseConnection>;
    fn AuthenticateClientToSession(this: &Self::This, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn NotifySessionId(this: &Self::This, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn GetProtocolHandles(this: &Self::This, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::Result<()>;
    fn ConnectNotify(this: &Self::This, sessionid: u32) -> ::windows_core::Result<()>;
    fn IsUserAllowedToLogon(this: &Self::This, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: &::windows_core::PCWSTR, pusername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SessionArbitrationEnumeration(this: &Self::This, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::Result<()>;
    fn LogonNotify(this: &Self::This, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: &::windows_core::PCWSTR, wszdomainname: &::windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn GetUserData(this: &Self::This, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows_core::Result<()>;
    fn DisconnectNotify(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProtocolStatus(this: &Self::This, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::Result<()>;
    fn GetLastInputTime(this: &Self::This) -> ::windows_core::Result<u64>;
    fn SetErrorInfo(this: &Self::This, ulerror: u32) -> ::windows_core::Result<()>;
    fn SendBeep(this: &Self::This, frequency: u32, duration: u32) -> ::windows_core::Result<()>;
    fn CreateVirtualChannel(this: &Self::This, szendpointname: &::windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32) -> ::windows_core::Result<usize>;
    fn QueryProperty(this: &Self::This, querytype: &::windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::Result<()>;
    fn GetShadowConnection(this: &Self::This) -> ::windows_core::Result<IWTSProtocolShadowConnection>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWTSProtocolConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLogonErrorRedirector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLogonErrorRedirector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplogonerrorredir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendPolicyData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendPolicyData(this, ::core::mem::transmute_copy(&ppolicydata)).into())
        }
        unsafe extern "system" fn AcceptConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcceptConnection(this).into())
        }
        unsafe extern "system" fn GetClientData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClientData(this, ::core::mem::transmute_copy(&pclientdata)).into())
        }
        unsafe extern "system" fn GetUserCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserCredentials(this, ::core::mem::transmute_copy(&pusercreds)).into())
        }
        unsafe extern "system" fn GetLicenseConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLicenseConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplicenseconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AuthenticateClientToSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AuthenticateClientToSession(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn NotifySessionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionId(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn GetProtocolHandles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProtocolHandles(this, ::core::mem::transmute_copy(&pkeyboardhandle), ::core::mem::transmute_copy(&pmousehandle), ::core::mem::transmute_copy(&pbeephandle), ::core::mem::transmute_copy(&pvideohandle)).into())
        }
        unsafe extern "system" fn ConnectNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectNotify(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: ::windows_core::PCWSTR, pusername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUserAllowedToLogon(this, ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&usertoken), ::core::mem::transmute(&pdomainname), ::core::mem::transmute(&pusername)).into())
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SessionArbitrationEnumeration(this, ::core::mem::transmute_copy(&husertoken), ::core::mem::transmute_copy(&bsinglesessionperuserenabled), ::core::mem::transmute_copy(&psessionidarray), ::core::mem::transmute_copy(&pdwsessionidentifiercount)).into())
        }
        unsafe extern "system" fn LogonNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: ::windows_core::PCWSTR, wszdomainname: ::windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogonNotify(this, ::core::mem::transmute_copy(&hclienttoken), ::core::mem::transmute(&wszusername), ::core::mem::transmute(&wszdomainname), ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn GetUserData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUserData(this, ::core::mem::transmute_copy(&ppolicydata), ::core::mem::transmute_copy(&pclientdata)).into())
        }
        unsafe extern "system" fn DisconnectNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectNotify(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetProtocolStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProtocolStatus(this, ::core::mem::transmute_copy(&pprotocolstatus)).into())
        }
        unsafe extern "system" fn GetLastInputTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastInputTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plastinputtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorInfo(this, ::core::mem::transmute_copy(&ulerror)).into())
        }
        unsafe extern "system" fn SendBeep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frequency: u32, duration: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendBeep(this, ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&duration)).into())
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szendpointname: ::windows_core::PCSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVirtualChannel(this, ::core::mem::transmute(&szendpointname), ::core::mem::transmute_copy(&bstatic), ::core::mem::transmute_copy(&requestedpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phchannel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querytype: ::windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryProperty(this, ::core::mem::transmute(&querytype), ::core::mem::transmute_copy(&ulnumentriesin), ::core::mem::transmute_copy(&ulnumentriesout), ::core::mem::transmute_copy(&ppropertyentriesin), ::core::mem::transmute_copy(&ppropertyentriesout)).into())
        }
        unsafe extern "system" fn GetShadowConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetShadowConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppshadowconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWTSProtocolConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Identity, Impl, OFFSET>,
            SendPolicyData: SendPolicyData::<Identity, Impl, OFFSET>,
            AcceptConnection: AcceptConnection::<Identity, Impl, OFFSET>,
            GetClientData: GetClientData::<Identity, Impl, OFFSET>,
            GetUserCredentials: GetUserCredentials::<Identity, Impl, OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Identity, Impl, OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Identity, Impl, OFFSET>,
            NotifySessionId: NotifySessionId::<Identity, Impl, OFFSET>,
            GetProtocolHandles: GetProtocolHandles::<Identity, Impl, OFFSET>,
            ConnectNotify: ConnectNotify::<Identity, Impl, OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Identity, Impl, OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Identity, Impl, OFFSET>,
            LogonNotify: LogonNotify::<Identity, Impl, OFFSET>,
            GetUserData: GetUserData::<Identity, Impl, OFFSET>,
            DisconnectNotify: DisconnectNotify::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Identity, Impl, OFFSET>,
            GetLastInputTime: GetLastInputTime::<Identity, Impl, OFFSET>,
            SetErrorInfo: SetErrorInfo::<Identity, Impl, OFFSET>,
            SendBeep: SendBeep::<Identity, Impl, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, Impl, OFFSET>,
            QueryProperty: QueryProperty::<Identity, Impl, OFFSET>,
            GetShadowConnection: GetShadowConnection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSProtocolConnectionCallback_Impl: ::windows_core::BaseImpl {
    fn OnReady(this: &Self::This) -> ::windows_core::Result<()>;
    fn BrokenConnection(this: &Self::This, reason: u32, source: u32) -> ::windows_core::Result<()>;
    fn StopScreenUpdates(this: &Self::This) -> ::windows_core::Result<()>;
    fn RedrawWindow(this: &Self::This, rect: *const WTS_SMALL_RECT) -> ::windows_core::Result<()>;
    fn DisplayIOCtl(this: &Self::This, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSProtocolConnectionCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolConnectionCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnReady<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReady(this).into())
        }
        unsafe extern "system" fn BrokenConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BrokenConnection(this, ::core::mem::transmute_copy(&reason), ::core::mem::transmute_copy(&source)).into())
        }
        unsafe extern "system" fn StopScreenUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopScreenUpdates(this).into())
        }
        unsafe extern "system" fn RedrawWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RedrawWindow(this, ::core::mem::transmute_copy(&rect)).into())
        }
        unsafe extern "system" fn DisplayIOCtl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayIOCtl(this, ::core::mem::transmute_copy(&displayioctl)).into())
        }
        IWTSProtocolConnectionCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnReady: OnReady::<Identity, Impl, OFFSET>,
            BrokenConnection: BrokenConnection::<Identity, Impl, OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Identity, Impl, OFFSET>,
            RedrawWindow: RedrawWindow::<Identity, Impl, OFFSET>,
            DisplayIOCtl: DisplayIOCtl::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolLicenseConnection_Impl: ::windows_core::BaseImpl {
    fn RequestLicensingCapabilities(this: &Self::This, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::Result<()>;
    fn SendClientLicense(this: &Self::This, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_core::Result<()>;
    fn RequestClientLicense(this: &Self::This, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::Result<()>;
    fn ProtocolComplete(this: &Self::This, ulcomplete: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWTSProtocolLicenseConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolLicenseConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestLicensingCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestLicensingCapabilities(this, ::core::mem::transmute_copy(&pplicensecapabilities), ::core::mem::transmute_copy(&pcblicensecapabilities)).into())
        }
        unsafe extern "system" fn SendClientLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendClientLicense(this, ::core::mem::transmute_copy(&pclientlicense), ::core::mem::transmute_copy(&cbclientlicense)).into())
        }
        unsafe extern "system" fn RequestClientLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestClientLicense(this, ::core::mem::transmute_copy(&reserve1), ::core::mem::transmute_copy(&reserve2), ::core::mem::transmute_copy(&ppclientlicense), ::core::mem::transmute_copy(&pcbclientlicense)).into())
        }
        unsafe extern "system" fn ProtocolComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProtocolComplete(this, ::core::mem::transmute_copy(&ulcomplete)).into())
        }
        IWTSProtocolLicenseConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Identity, Impl, OFFSET>,
            SendClientLicense: SendClientLicense::<Identity, Impl, OFFSET>,
            RequestClientLicense: RequestClientLicense::<Identity, Impl, OFFSET>,
            ProtocolComplete: ProtocolComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSProtocolListener_Impl: ::windows_core::BaseImpl {
    fn StartListen(this: &Self::This, pcallback: ::core::option::Option<&IWTSProtocolListenerCallback>) -> ::windows_core::Result<()>;
    fn StopListen(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSProtocolListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartListen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartListen(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn StopListen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopListen(this).into())
        }
        IWTSProtocolListener_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartListen: StartListen::<Identity, Impl, OFFSET>,
            StopListen: StopListen::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSProtocolListenerCallback_Impl: ::windows_core::BaseImpl {
    fn OnConnected(this: &Self::This, pconnection: ::core::option::Option<&IWTSProtocolConnection>) -> ::windows_core::Result<IWTSProtocolConnectionCallback>;
}
impl ::windows_core::Iids for IWTSProtocolListenerCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolListenerCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolListenerCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolListenerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void, pcallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnConnected(this, ::windows_core::from_raw_borrowed(&pconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWTSProtocolListenerCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnConnected: OnConnected::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSProtocolLogonErrorRedirector_Impl: ::windows_core::BaseImpl {
    fn OnBeginPainting(this: &Self::This) -> ::windows_core::Result<()>;
    fn RedirectStatus(this: &Self::This, pszmessage: &::windows_core::PCWSTR) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(this: &Self::This, pszcaption: &::windows_core::PCWSTR, pszmessage: &::windows_core::PCWSTR, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(this: &Self::This, ntsstatus: i32, ntssubstatus: i32, pszcaption: &::windows_core::PCWSTR, pszmessage: &::windows_core::PCWSTR, utype: u32) -> ::windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
impl ::windows_core::Iids for IWTSProtocolLogonErrorRedirector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolLogonErrorRedirector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnBeginPainting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBeginPainting(this).into())
        }
        unsafe extern "system" fn RedirectStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmessage: ::windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectStatus(this, ::core::mem::transmute(&pszmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RedirectMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectMessage(this, ::core::mem::transmute(&pszcaption), ::core::mem::transmute(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RedirectLogonError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: ::windows_core::PCWSTR, pszmessage: ::windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectLogonError(this, ::core::mem::transmute_copy(&ntsstatus), ::core::mem::transmute_copy(&ntssubstatus), ::core::mem::transmute(&pszcaption), ::core::mem::transmute(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWTSProtocolLogonErrorRedirector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnBeginPainting: OnBeginPainting::<Identity, Impl, OFFSET>,
            RedirectStatus: RedirectStatus::<Identity, Impl, OFFSET>,
            RedirectMessage: RedirectMessage::<Identity, Impl, OFFSET>,
            RedirectLogonError: RedirectLogonError::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSProtocolManager_Impl: ::windows_core::BaseImpl {
    fn CreateListener(this: &Self::This, wszlistenername: &::windows_core::PCWSTR) -> ::windows_core::Result<IWTSProtocolListener>;
    fn NotifyServiceStateChange(this: &Self::This, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::Result<()>;
    fn NotifySessionOfServiceStart(this: &Self::This, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn NotifySessionOfServiceStop(this: &Self::This, sessionid: *const WTS_SESSION_ID) -> ::windows_core::Result<()>;
    fn NotifySessionStateChange(this: &Self::This, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSProtocolManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszlistenername: ::windows_core::PCWSTR, pprotocollistener: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateListener(this, ::core::mem::transmute(&wszlistenername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprotocollistener, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyServiceStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyServiceStateChange(this, ::core::mem::transmute_copy(&ptsservicestatechange)).into())
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionOfServiceStart(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionOfServiceStop(this, ::core::mem::transmute_copy(&sessionid)).into())
        }
        unsafe extern "system" fn NotifySessionStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifySessionStateChange(this, ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&eventid)).into())
        }
        IWTSProtocolManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateListener: CreateListener::<Identity, Impl, OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Identity, Impl, OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Identity, Impl, OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Identity, Impl, OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSProtocolShadowCallback_Impl: ::windows_core::BaseImpl {
    fn StopShadow(this: &Self::This) -> ::windows_core::Result<()>;
    fn InvokeTargetShadow(this: &Self::This, ptargetservername: &::windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSProtocolShadowCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolShadowCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolShadowCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StopShadow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolShadowCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopShadow(this).into())
        }
        unsafe extern "system" fn InvokeTargetShadow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolShadowCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::InvokeTargetShadow(this, ::core::mem::transmute(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute(&pclientname)).into()
            })
        }
        IWTSProtocolShadowCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StopShadow: StopShadow::<Identity, Impl, OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSProtocolShadowConnection_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This, ptargetservername: &::windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::core::option::Option<&IWTSProtocolShadowCallback>) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn DoTarget(this: &Self::This, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSProtocolShadowConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolShadowConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSProtocolShadowConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolShadowConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetservername: ::windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&hotkeyvk), ::core::mem::transmute_copy(&hotkeymodifiers), ::windows_core::from_raw_borrowed(&pshadowcallback)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolShadowConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn DoTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSProtocolShadowConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoTarget(this, ::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute(&pclientname)).into())
        }
        IWTSProtocolShadowConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            DoTarget: DoTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSSBPlugin_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn WTSSBX_MachineChangeNotification(this: &Self::This, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows_core::Result<()>;
    fn WTSSBX_SessionChangeNotification(this: &Self::This, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows_core::Result<()>;
    fn WTSSBX_GetMostSuitableServer(this: &Self::This, username: &::windows_core::PCWSTR, domainname: &::windows_core::PCWSTR, applicationtype: &::windows_core::PCWSTR, farmname: &::windows_core::PCWSTR, pmachineid: *mut i32) -> ::windows_core::Result<()>;
    fn Terminated(this: &Self::This) -> ::windows_core::Result<()>;
    fn WTSSBX_GetUserExternalSession(this: &Self::This, username: &::windows_core::PCWSTR, domainname: &::windows_core::PCWSTR, applicationtype: &::windows_core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWTSSBPlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSSBPlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSSBPlugin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSSBPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plugincapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Initialize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plugincapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WTSSBX_MachineChangeNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSSBPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WTSSBX_MachineChangeNotification(this, ::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&machineid), ::core::mem::transmute_copy(&pmachineinfo)).into())
        }
        unsafe extern "system" fn WTSSBX_SessionChangeNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSSBPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WTSSBX_SessionChangeNotification(this, ::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&machineid), ::core::mem::transmute_copy(&numofsessions), ::core::mem::transmute_copy(&sessioninfo)).into())
        }
        unsafe extern "system" fn WTSSBX_GetMostSuitableServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSSBPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::windows_core::PCWSTR, domainname: ::windows_core::PCWSTR, applicationtype: ::windows_core::PCWSTR, farmname: ::windows_core::PCWSTR, pmachineid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WTSSBX_GetMostSuitableServer(this, ::core::mem::transmute(&username), ::core::mem::transmute(&domainname), ::core::mem::transmute(&applicationtype), ::core::mem::transmute(&farmname), ::core::mem::transmute_copy(&pmachineid)).into())
        }
        unsafe extern "system" fn Terminated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSSBPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminated(this).into())
        }
        unsafe extern "system" fn WTSSBX_GetUserExternalSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSSBPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::windows_core::PCWSTR, domainname: ::windows_core::PCWSTR, applicationtype: ::windows_core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WTSSBX_GetUserExternalSession(this, ::core::mem::transmute(&username), ::core::mem::transmute(&domainname), ::core::mem::transmute(&applicationtype), ::core::mem::transmute_copy(&redirectorinternalip), ::core::mem::transmute_copy(&psessionid), ::core::mem::transmute_copy(&pmachineconnectinfo)).into())
        }
        IWTSSBPlugin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            WTSSBX_MachineChangeNotification: WTSSBX_MachineChangeNotification::<Identity, Impl, OFFSET>,
            WTSSBX_SessionChangeNotification: WTSSBX_SessionChangeNotification::<Identity, Impl, OFFSET>,
            WTSSBX_GetMostSuitableServer: WTSSBX_GetMostSuitableServer::<Identity, Impl, OFFSET>,
            Terminated: Terminated::<Identity, Impl, OFFSET>,
            WTSSBX_GetUserExternalSession: WTSSBX_GetUserExternalSession::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSVirtualChannel_Impl: ::windows_core::BaseImpl {
    fn Write(this: &Self::This, cbsize: u32, pbuffer: *const u8, preserved: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSVirtualChannel {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannel_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSVirtualChannel {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer), ::windows_core::from_raw_borrowed(&preserved)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannel_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IWTSVirtualChannel_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Write: Write::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSVirtualChannelCallback_Impl: ::windows_core::BaseImpl {
    fn OnDataReceived(this: &Self::This, cbsize: u32, pbuffer: *const u8) -> ::windows_core::Result<()>;
    fn OnClose(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWTSVirtualChannelCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannelCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSVirtualChannelCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDataReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannelCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataReceived(this, ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer)).into())
        }
        unsafe extern "system" fn OnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannelCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClose(this).into())
        }
        IWTSVirtualChannelCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDataReceived: OnDataReceived::<Identity, Impl, OFFSET>,
            OnClose: OnClose::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWTSVirtualChannelManager_Impl: ::windows_core::BaseImpl {
    fn CreateListener(this: &Self::This, pszchannelname: &::windows_core::PCSTR, uflags: u32, plistenercallback: ::core::option::Option<&IWTSListenerCallback>) -> ::windows_core::Result<IWTSListener>;
}
impl ::windows_core::Iids for IWTSVirtualChannelManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannelManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWTSVirtualChannelManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWTSVirtualChannelManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszchannelname: ::windows_core::PCSTR, uflags: u32, plistenercallback: *mut ::core::ffi::c_void, pplistener: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateListener(this, ::core::mem::transmute(&pszchannelname), ::core::mem::transmute_copy(&uflags), ::windows_core::from_raw_borrowed(&plistenercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplistener, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWTSVirtualChannelManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateListener: CreateListener::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspace_Impl: ::windows_core::BaseImpl {
    fn GetWorkspaceNames(this: &Self::This) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn StartRemoteApplication(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn GetProcessId(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWorkspace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWorkspaceNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWorkspaceNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psawkspnames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartRemoteApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartRemoteApplication(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute_copy(&psaparams)).into())
        }
        unsafe extern "system" fn GetProcessId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulprocessid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProcessId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprocessid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWorkspace_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWorkspaceNames: GetWorkspaceNames::<Identity, Impl, OFFSET>,
            StartRemoteApplication: StartRemoteApplication::<Identity, Impl, OFFSET>,
            GetProcessId: GetProcessId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspace2_Impl: ::windows_core::BaseImpl + IWorkspace_Impl {
    fn StartRemoteApplicationEx(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, bstrrequestingappid: &::windows_core::BSTR, bstrrequestingappfamilyname: &::windows_core::BSTR, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: &::windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWorkspace2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWorkspace);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspace2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartRemoteApplicationEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrequestingappid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrequestingappfamilyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: ::std::mem::MaybeUninit<::windows_core::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartRemoteApplicationEx(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute(&bstrrequestingappid), ::core::mem::transmute(&bstrrequestingappfamilyname), ::core::mem::transmute_copy(&blaunchintoimmersiveclient), ::core::mem::transmute(&bstrimmersiveclientactivationcontext), ::core::mem::transmute_copy(&psaparams)).into())
        }
        IWorkspace2_Vtbl {
            base__: <IWorkspace as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartRemoteApplicationEx: StartRemoteApplicationEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspace3_Impl: ::windows_core::BaseImpl + IWorkspace2_Impl {
    fn GetClaimsToken2(this: &Self::This, bstrclaimshint: &::windows_core::BSTR, bstruserhint: &::windows_core::BSTR, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: &super::super::Foundation::RECT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClaimsToken(this: &Self::This, bstraccesstoken: &::windows_core::BSTR, ullaccesstokenexpiration: u64, bstrrefreshtoken: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWorkspace3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWorkspace2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspace3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClaimsToken2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclaimshint: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruserhint: ::std::mem::MaybeUninit<::windows_core::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClaimsToken2(this, ::core::mem::transmute(&bstrclaimshint), ::core::mem::transmute(&bstruserhint), ::core::mem::transmute_copy(&claimcookie), ::core::mem::transmute_copy(&hwndcreduiparent), ::core::mem::transmute(&rectcreduiparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstraccesstoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClaimsToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstraccesstoken: ::std::mem::MaybeUninit<::windows_core::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClaimsToken(this, ::core::mem::transmute(&bstraccesstoken), ::core::mem::transmute_copy(&ullaccesstokenexpiration), ::core::mem::transmute(&bstrrefreshtoken)).into())
        }
        IWorkspace3_Vtbl {
            base__: <IWorkspace2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClaimsToken2: GetClaimsToken2::<Identity, Impl, OFFSET>,
            SetClaimsToken: SetClaimsToken::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWorkspaceClientExt_Impl: ::windows_core::BaseImpl {
    fn GetResourceId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetResourceDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IssueDisconnect(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWorkspaceClientExt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceClientExt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceClientExt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResourceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceClientExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResourceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrworkspaceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResourceDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceClientExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspacedisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResourceDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrworkspacedisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IssueDisconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceClientExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IssueDisconnect(this).into())
        }
        IWorkspaceClientExt_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResourceId: GetResourceId::<Identity, Impl, OFFSET>,
            GetResourceDisplayName: GetResourceDisplayName::<Identity, Impl, OFFSET>,
            IssueDisconnect: IssueDisconnect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWorkspaceRegistration_Impl: ::windows_core::BaseImpl {
    fn AddResource(this: &Self::This, punk: ::core::option::Option<&IWorkspaceClientExt>) -> ::windows_core::Result<u32>;
    fn RemoveResource(this: &Self::This, dwcookieconnection: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWorkspaceRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddResource(this, ::windows_core::from_raw_borrowed(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveResource(this, ::core::mem::transmute_copy(&dwcookieconnection)).into())
        }
        IWorkspaceRegistration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            RemoveResource: RemoveResource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWorkspaceRegistration2_Impl: ::windows_core::BaseImpl + IWorkspaceRegistration_Impl {
    fn AddResourceEx(this: &Self::This, punk: ::core::option::Option<&IWorkspaceClientExt>, bstreventloguploadaddress: &::windows_core::BSTR, pdwcookie: *mut u32, correlationid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RemoveResourceEx(this: &Self::This, dwcookieconnection: u32, correlationid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWorkspaceRegistration2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWorkspaceRegistration);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceRegistration2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceRegistration2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddResourceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceRegistration2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, bstreventloguploadaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcookie: *mut u32, correlationid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddResourceEx(this, ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute(&bstreventloguploadaddress), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute(&correlationid)).into())
        }
        unsafe extern "system" fn RemoveResourceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceRegistration2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32, correlationid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveResourceEx(this, ::core::mem::transmute_copy(&dwcookieconnection), ::core::mem::transmute(&correlationid)).into())
        }
        IWorkspaceRegistration2_Vtbl {
            base__: <IWorkspaceRegistration as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddResourceEx: AddResourceEx::<Identity, Impl, OFFSET>,
            RemoveResourceEx: RemoveResourceEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWorkspaceReportMessage_Impl: ::windows_core::BaseImpl {
    fn RegisterErrorLogMessage(this: &Self::This, bstrmessage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsErrorMessageRegistered(this: &Self::This, bstrwkspid: &::windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &::windows_core::BSTR, dwerrorcode: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RegisterErrorEvent(this: &Self::This, bstrwkspid: &::windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &::windows_core::BSTR, dwerrorcode: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWorkspaceReportMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceReportMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceReportMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterErrorLogMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceReportMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterErrorLogMessage(this, ::core::mem::transmute(&bstrmessage)).into())
        }
        unsafe extern "system" fn IsErrorMessageRegistered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceReportMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrorcode: u32, pferrorexist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsErrorMessageRegistered(this, ::core::mem::transmute(&bstrwkspid), ::core::mem::transmute_copy(&dwerrortype), ::core::mem::transmute(&bstrerrormessagetype), ::core::mem::transmute_copy(&dwerrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pferrorexist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterErrorEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceReportMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwerrorcode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterErrorEvent(this, ::core::mem::transmute(&bstrwkspid), ::core::mem::transmute_copy(&dwerrortype), ::core::mem::transmute(&bstrerrormessagetype), ::core::mem::transmute_copy(&dwerrorcode)).into())
        }
        IWorkspaceReportMessage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterErrorLogMessage: RegisterErrorLogMessage::<Identity, Impl, OFFSET>,
            IsErrorMessageRegistered: IsErrorMessageRegistered::<Identity, Impl, OFFSET>,
            RegisterErrorEvent: RegisterErrorEvent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceResTypeRegistry_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn AddResourceType(this: &Self::This, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &::windows_core::BSTR, bstrlauncher: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteResourceType(this: &Self::This, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetRegisteredFileExtensions(this: &Self::This, fmachinewide: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn GetResourceTypeInfo(this: &Self::This, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ModifyResourceType(this: &Self::This, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &::windows_core::BSTR, bstrlauncher: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWorkspaceResTypeRegistry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceResTypeRegistry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddResourceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlauncher: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddResourceType(this, ::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute(&bstrfileextension), ::core::mem::transmute(&bstrlauncher)).into())
        }
        unsafe extern "system" fn DeleteResourceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteResourceType(this, ::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute(&bstrfileextension)).into())
        }
        unsafe extern "system" fn GetRegisteredFileExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegisteredFileExtensions(this, ::core::mem::transmute_copy(&fmachinewide)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psafileextensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResourceTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrlauncher: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResourceTypeInfo(this, ::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute(&bstrfileextension)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlauncher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifyResourceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlauncher: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyResourceType(this, ::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute(&bstrfileextension), ::core::mem::transmute(&bstrlauncher)).into())
        }
        IWorkspaceResTypeRegistry_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddResourceType: AddResourceType::<Identity, Impl, OFFSET>,
            DeleteResourceType: DeleteResourceType::<Identity, Impl, OFFSET>,
            GetRegisteredFileExtensions: GetRegisteredFileExtensions::<Identity, Impl, OFFSET>,
            GetResourceTypeInfo: GetResourceTypeInfo::<Identity, Impl, OFFSET>,
            ModifyResourceType: ModifyResourceType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceScriptable_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DisconnectWorkspace(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartWorkspace(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bstrworkspaceparams: &::windows_core::BSTR, ltimeout: i32, lflags: i32) -> ::windows_core::Result<()>;
    fn IsWorkspaceCredentialSpecified(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorkspaceSSOEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ClearWorkspaceCredential(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnAuthenticated(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisconnectWorkspaceByFriendlyName(this: &Self::This, bstrworkspacefriendlyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWorkspaceScriptable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceScriptable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisconnectWorkspace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectWorkspace(this, ::core::mem::transmute(&bstrworkspaceid)).into())
        }
        unsafe extern "system" fn StartWorkspace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspaceparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, ltimeout: i32, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartWorkspace(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrworkspaceparams), ::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn IsWorkspaceCredentialSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL, pbcredexist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorkspaceCredentialSpecified(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute_copy(&bcountunauthenticatedcredentials)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcredexist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsWorkspaceSSOEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbssoenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorkspaceSSOEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbssoenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClearWorkspaceCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearWorkspaceCredential(this, ::core::mem::transmute(&bstrworkspaceid)).into())
        }
        unsafe extern "system" fn OnAuthenticated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAuthenticated(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn DisconnectWorkspaceByFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectWorkspaceByFriendlyName(this, ::core::mem::transmute(&bstrworkspacefriendlyname)).into())
        }
        IWorkspaceScriptable_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisconnectWorkspace: DisconnectWorkspace::<Identity, Impl, OFFSET>,
            StartWorkspace: StartWorkspace::<Identity, Impl, OFFSET>,
            IsWorkspaceCredentialSpecified: IsWorkspaceCredentialSpecified::<Identity, Impl, OFFSET>,
            IsWorkspaceSSOEnabled: IsWorkspaceSSOEnabled::<Identity, Impl, OFFSET>,
            ClearWorkspaceCredential: ClearWorkspaceCredential::<Identity, Impl, OFFSET>,
            OnAuthenticated: OnAuthenticated::<Identity, Impl, OFFSET>,
            DisconnectWorkspaceByFriendlyName: DisconnectWorkspaceByFriendlyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceScriptable2_Impl: ::windows_core::BaseImpl + IWorkspaceScriptable_Impl {
    fn StartWorkspaceEx(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, bstrworkspacefriendlyname: &::windows_core::BSTR, bstrredirectorname: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bstrappcontainer: &::windows_core::BSTR, bstrworkspaceparams: &::windows_core::BSTR, ltimeout: i32, lflags: i32) -> ::windows_core::Result<()>;
    fn ResourceDismissed(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, bstrworkspacefriendlyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWorkspaceScriptable2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWorkspaceScriptable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceScriptable2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartWorkspaceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrredirectorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrappcontainer: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspaceparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, ltimeout: i32, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartWorkspaceEx(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute(&bstrworkspacefriendlyname), ::core::mem::transmute(&bstrredirectorname), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrappcontainer), ::core::mem::transmute(&bstrworkspaceparams), ::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn ResourceDismissed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResourceDismissed(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute(&bstrworkspacefriendlyname)).into())
        }
        IWorkspaceScriptable2_Vtbl {
            base__: <IWorkspaceScriptable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartWorkspaceEx: StartWorkspaceEx::<Identity, Impl, OFFSET>,
            ResourceDismissed: ResourceDismissed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceScriptable3_Impl: ::windows_core::BaseImpl + IWorkspaceScriptable2_Impl {
    fn StartWorkspaceEx2(this: &Self::This, bstrworkspaceid: &::windows_core::BSTR, bstrworkspacefriendlyname: &::windows_core::BSTR, bstrredirectorname: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bstrappcontainer: &::windows_core::BSTR, bstrworkspaceparams: &::windows_core::BSTR, ltimeout: i32, lflags: i32, bstreventloguploadaddress: &::windows_core::BSTR, correlationid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWorkspaceScriptable3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWorkspaceScriptable2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWorkspaceScriptable3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartWorkspaceEx2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWorkspaceScriptable3_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            bstrworkspaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bstrworkspacefriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bstrredirectorname: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bstrappcontainer: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            bstrworkspaceparams: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            ltimeout: i32,
            lflags: i32,
            bstreventloguploadaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            correlationid: ::windows_core::GUID,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::StartWorkspaceEx2(this, ::core::mem::transmute(&bstrworkspaceid), ::core::mem::transmute(&bstrworkspacefriendlyname), ::core::mem::transmute(&bstrredirectorname), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrappcontainer), ::core::mem::transmute(&bstrworkspaceparams), ::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstreventloguploadaddress), ::core::mem::transmute(&correlationid)).into()
            })
        }
        IWorkspaceScriptable3_Vtbl {
            base__: <IWorkspaceScriptable2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartWorkspaceEx2: StartWorkspaceEx2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ItsPubPlugin_Impl: ::windows_core::BaseImpl {
    fn GetResourceList(this: &Self::This, userid: &::windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows_core::Result<()>;
    fn GetResource(this: &Self::This, alias: &::windows_core::PCWSTR, flags: i32, resource: *mut pluginResource) -> ::windows_core::Result<()>;
    fn GetCacheLastUpdateTime(this: &Self::This) -> ::windows_core::Result<u64>;
    fn pluginName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn pluginVersion(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ResolveResource(this: &Self::This, resourcetype: *mut u32, resourcelocation: ::windows_core::PWSTR, endpointname: ::windows_core::PWSTR, userid: &::windows_core::PCWSTR, alias: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ItsPubPlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ItsPubPlugin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResourceList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResourceList(this, ::core::mem::transmute(&userid), ::core::mem::transmute_copy(&pceapplistsize), ::core::mem::transmute_copy(&resourcelist)).into())
        }
        unsafe extern "system" fn GetResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alias: ::windows_core::PCWSTR, flags: i32, resource: *mut pluginResource) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResource(this, ::core::mem::transmute(&alias), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&resource)).into())
        }
        unsafe extern "system" fn GetCacheLastUpdateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastupdatetime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCacheLastUpdateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastupdatetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn pluginName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::pluginName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn pluginVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::pluginVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResolveResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcetype: *mut u32, resourcelocation: ::windows_core::PWSTR, endpointname: ::windows_core::PWSTR, userid: ::windows_core::PCWSTR, alias: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveResource(this, ::core::mem::transmute_copy(&resourcetype), ::core::mem::transmute_copy(&resourcelocation), ::core::mem::transmute_copy(&endpointname), ::core::mem::transmute(&userid), ::core::mem::transmute(&alias)).into())
        }
        ItsPubPlugin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResourceList: GetResourceList::<Identity, Impl, OFFSET>,
            GetResource: GetResource::<Identity, Impl, OFFSET>,
            GetCacheLastUpdateTime: GetCacheLastUpdateTime::<Identity, Impl, OFFSET>,
            pluginName: pluginName::<Identity, Impl, OFFSET>,
            pluginVersion: pluginVersion::<Identity, Impl, OFFSET>,
            ResolveResource: ResolveResource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ItsPubPlugin2_Impl: ::windows_core::BaseImpl + ItsPubPlugin_Impl {
    fn GetResource2List(this: &Self::This, userid: &::windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows_core::Result<()>;
    fn GetResource2(this: &Self::This, alias: &::windows_core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows_core::Result<()>;
    fn ResolvePersonalDesktop(this: &Self::This, userid: &::windows_core::PCWSTR, poolid: &::windows_core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn DeletePersonalDesktopAssignment(this: &Self::This, userid: &::windows_core::PCWSTR, poolid: &::windows_core::PCWSTR, endpointname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ItsPubPlugin2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ItsPubPlugin);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ItsPubPlugin2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResource2List<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResource2List(this, ::core::mem::transmute(&userid), ::core::mem::transmute_copy(&pceapplistsize), ::core::mem::transmute_copy(&resourcelist)).into())
        }
        unsafe extern "system" fn GetResource2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alias: ::windows_core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResource2(this, ::core::mem::transmute(&alias), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&resource)).into())
        }
        unsafe extern "system" fn ResolvePersonalDesktop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, poolid: ::windows_core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolvePersonalDesktop(this, ::core::mem::transmute(&userid), ::core::mem::transmute(&poolid), ::core::mem::transmute_copy(&epdresolutiontype), ::core::mem::transmute_copy(&ppdassignmenttype), ::core::mem::transmute_copy(&endpointname)).into())
        }
        unsafe extern "system" fn DeletePersonalDesktopAssignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ItsPubPlugin2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, userid: ::windows_core::PCWSTR, poolid: ::windows_core::PCWSTR, endpointname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePersonalDesktopAssignment(this, ::core::mem::transmute(&userid), ::core::mem::transmute(&poolid), ::core::mem::transmute(&endpointname)).into())
        }
        ItsPubPlugin2_Vtbl {
            base__: <ItsPubPlugin as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetResource2List: GetResource2List::<Identity, Impl, OFFSET>,
            GetResource2: GetResource2::<Identity, Impl, OFFSET>,
            ResolvePersonalDesktop: ResolvePersonalDesktop::<Identity, Impl, OFFSET>,
            DeletePersonalDesktopAssignment: DeletePersonalDesktopAssignment::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ITSWkspEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _ITSWkspEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _ITSWkspEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _ITSWkspEvents {
    const VTABLE: Self::Vtable = { _ITSWkspEvents_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
