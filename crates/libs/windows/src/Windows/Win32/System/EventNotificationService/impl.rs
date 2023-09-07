#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensLogon_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Logon(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Logoff(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartShell(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisplayLock(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisplayUnlock(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartScreenSaver(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StopScreenSaver(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISensLogon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensLogon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Logon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Logon(this, ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn Logoff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Logoff(this, ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn StartShell<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartShell(this, ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn DisplayLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayLock(this, ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn DisplayUnlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayUnlock(this, ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn StartScreenSaver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartScreenSaver(this, ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn StopScreenSaver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopScreenSaver(this, ::core::mem::transmute(&bstrusername)).into())
        }
        ISensLogon_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Logon: Logon::<Identity, Impl, OFFSET>,
            Logoff: Logoff::<Identity, Impl, OFFSET>,
            StartShell: StartShell::<Identity, Impl, OFFSET>,
            DisplayLock: DisplayLock::<Identity, Impl, OFFSET>,
            DisplayUnlock: DisplayUnlock::<Identity, Impl, OFFSET>,
            StartScreenSaver: StartScreenSaver::<Identity, Impl, OFFSET>,
            StopScreenSaver: StopScreenSaver::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensLogon2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Logon(this: &Self::This, bstrusername: &::windows_core::BSTR, dwsessionid: u32) -> ::windows_core::Result<()>;
    fn Logoff(this: &Self::This, bstrusername: &::windows_core::BSTR, dwsessionid: u32) -> ::windows_core::Result<()>;
    fn SessionDisconnect(this: &Self::This, bstrusername: &::windows_core::BSTR, dwsessionid: u32) -> ::windows_core::Result<()>;
    fn SessionReconnect(this: &Self::This, bstrusername: &::windows_core::BSTR, dwsessionid: u32) -> ::windows_core::Result<()>;
    fn PostShell(this: &Self::This, bstrusername: &::windows_core::BSTR, dwsessionid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISensLogon2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensLogon2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Logon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Logon(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into())
        }
        unsafe extern "system" fn Logoff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Logoff(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into())
        }
        unsafe extern "system" fn SessionDisconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SessionDisconnect(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into())
        }
        unsafe extern "system" fn SessionReconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SessionReconnect(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into())
        }
        unsafe extern "system" fn PostShell<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsessionid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostShell(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into())
        }
        ISensLogon2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Logon: Logon::<Identity, Impl, OFFSET>,
            Logoff: Logoff::<Identity, Impl, OFFSET>,
            SessionDisconnect: SessionDisconnect::<Identity, Impl, OFFSET>,
            SessionReconnect: SessionReconnect::<Identity, Impl, OFFSET>,
            PostShell: PostShell::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensNetwork_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ConnectionMade(this: &Self::This, bstrconnection: &::windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows_core::Result<()>;
    fn ConnectionMadeNoQOCInfo(this: &Self::This, bstrconnection: &::windows_core::BSTR, ultype: u32) -> ::windows_core::Result<()>;
    fn ConnectionLost(this: &Self::This, bstrconnection: &::windows_core::BSTR, ultype: SENS_CONNECTION_TYPE) -> ::windows_core::Result<()>;
    fn DestinationReachable(this: &Self::This, bstrdestination: &::windows_core::BSTR, bstrconnection: &::windows_core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows_core::Result<()>;
    fn DestinationReachableNoQOCInfo(this: &Self::This, bstrdestination: &::windows_core::BSTR, bstrconnection: &::windows_core::BSTR, ultype: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISensNetwork {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensNetwork {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectionMade<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnection: ::std::mem::MaybeUninit<::windows_core::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectionMade(this, ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into())
        }
        unsafe extern "system" fn ConnectionMadeNoQOCInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnection: ::std::mem::MaybeUninit<::windows_core::BSTR>, ultype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectionMadeNoQOCInfo(this, ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into())
        }
        unsafe extern "system" fn ConnectionLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnection: ::std::mem::MaybeUninit<::windows_core::BSTR>, ultype: SENS_CONNECTION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectionLost(this, ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into())
        }
        unsafe extern "system" fn DestinationReachable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdestination: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrconnection: ::std::mem::MaybeUninit<::windows_core::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestinationReachable(this, ::core::mem::transmute(&bstrdestination), ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into())
        }
        unsafe extern "system" fn DestinationReachableNoQOCInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdestination: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrconnection: ::std::mem::MaybeUninit<::windows_core::BSTR>, ultype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestinationReachableNoQOCInfo(this, ::core::mem::transmute(&bstrdestination), ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into())
        }
        ISensNetwork_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectionMade: ConnectionMade::<Identity, Impl, OFFSET>,
            ConnectionMadeNoQOCInfo: ConnectionMadeNoQOCInfo::<Identity, Impl, OFFSET>,
            ConnectionLost: ConnectionLost::<Identity, Impl, OFFSET>,
            DestinationReachable: DestinationReachable::<Identity, Impl, OFFSET>,
            DestinationReachableNoQOCInfo: DestinationReachableNoQOCInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISensOnNow_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn OnACPower(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnBatteryPower(this: &Self::This, dwbatterylifepercent: u32) -> ::windows_core::Result<()>;
    fn BatteryLow(this: &Self::This, dwbatterylifepercent: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISensOnNow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensOnNow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnACPower<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnACPower(this).into())
        }
        unsafe extern "system" fn OnBatteryPower<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBatteryPower(this, ::core::mem::transmute_copy(&dwbatterylifepercent)).into())
        }
        unsafe extern "system" fn BatteryLow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BatteryLow(this, ::core::mem::transmute_copy(&dwbatterylifepercent)).into())
        }
        ISensOnNow_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnACPower: OnACPower::<Identity, Impl, OFFSET>,
            OnBatteryPower: OnBatteryPower::<Identity, Impl, OFFSET>,
            BatteryLow: BatteryLow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
