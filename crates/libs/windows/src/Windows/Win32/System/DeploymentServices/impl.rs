#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportCacheable_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Dirty(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Discard(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportCacheable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportCacheable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Dirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dirty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdirty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Discard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Discard(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        IWdsTransportCacheable_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Dirty: Dirty::<Identity, Impl, OFFSET>,
            Discard: Discard::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportClient_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IWdsTransportSession>;
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MacAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IpAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PercentCompletion(this: &Self::This) -> ::windows_core::Result<u32>;
    fn JoinDuration(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CpuUtilization(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MemoryUtilization(this: &Self::This) -> ::windows_core::Result<u32>;
    fn NetworkUtilization(this: &Self::This) -> ::windows_core::Result<u32>;
    fn UserIdentity(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Disconnect(this: &Self::This, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MacAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszmacaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MacAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszmacaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IpAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IpAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszipaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PercentCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulpercentcompletion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PercentCompletion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpercentcompletion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JoinDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puljoinduration: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JoinDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljoinduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CpuUtilization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcpuutilization: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CpuUtilization(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcpuutilization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MemoryUtilization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulmemoryutilization: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MemoryUtilization(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulmemoryutilization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetworkUtilization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulnetworkutilization: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkUtilization(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulnetworkutilization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszuseridentity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserIdentity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszuseridentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this, ::core::mem::transmute_copy(&disconnectiontype)).into())
        }
        IWdsTransportClient_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            MacAddress: MacAddress::<Identity, Impl, OFFSET>,
            IpAddress: IpAddress::<Identity, Impl, OFFSET>,
            PercentCompletion: PercentCompletion::<Identity, Impl, OFFSET>,
            JoinDuration: JoinDuration::<Identity, Impl, OFFSET>,
            CpuUtilization: CpuUtilization::<Identity, Impl, OFFSET>,
            MemoryUtilization: MemoryUtilization::<Identity, Impl, OFFSET>,
            NetworkUtilization: NetworkUtilization::<Identity, Impl, OFFSET>,
            UserIdentity: UserIdentity::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn get_Item(this: &Self::This, ulindex: u32) -> ::windows_core::Result<super::Com::IDispatch>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportConfigurationManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ServicePolicy(this: &Self::This) -> ::windows_core::Result<IWdsTransportServicePolicy>;
    fn DiagnosticsPolicy(this: &Self::This) -> ::windows_core::Result<IWdsTransportDiagnosticsPolicy>;
    fn get_WdsTransportServicesRunning(this: &Self::This, brealtimestatus: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EnableWdsTransportServices(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableWdsTransportServices(this: &Self::This) -> ::windows_core::Result<()>;
    fn StartWdsTransportServices(this: &Self::This) -> ::windows_core::Result<()>;
    fn StopWdsTransportServices(this: &Self::This) -> ::windows_core::Result<()>;
    fn RestartWdsTransportServices(this: &Self::This) -> ::windows_core::Result<()>;
    fn NotifyWdsTransportServices(this: &Self::This, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportConfigurationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportConfigurationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServicePolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportservicepolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServicePolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportservicepolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DiagnosticsPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiagnosticsPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportdiagnosticspolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_WdsTransportServicesRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brealtimestatus: super::super::Foundation::VARIANT_BOOL, pbservicesrunning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_WdsTransportServicesRunning(this, ::core::mem::transmute_copy(&brealtimestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbservicesrunning, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableWdsTransportServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableWdsTransportServices(this).into())
        }
        unsafe extern "system" fn DisableWdsTransportServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableWdsTransportServices(this).into())
        }
        unsafe extern "system" fn StartWdsTransportServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartWdsTransportServices(this).into())
        }
        unsafe extern "system" fn StopWdsTransportServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopWdsTransportServices(this).into())
        }
        unsafe extern "system" fn RestartWdsTransportServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestartWdsTransportServices(this).into())
        }
        unsafe extern "system" fn NotifyWdsTransportServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyWdsTransportServices(this, ::core::mem::transmute_copy(&servicenotification)).into())
        }
        IWdsTransportConfigurationManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServicePolicy: ServicePolicy::<Identity, Impl, OFFSET>,
            DiagnosticsPolicy: DiagnosticsPolicy::<Identity, Impl, OFFSET>,
            get_WdsTransportServicesRunning: get_WdsTransportServicesRunning::<Identity, Impl, OFFSET>,
            EnableWdsTransportServices: EnableWdsTransportServices::<Identity, Impl, OFFSET>,
            DisableWdsTransportServices: DisableWdsTransportServices::<Identity, Impl, OFFSET>,
            StartWdsTransportServices: StartWdsTransportServices::<Identity, Impl, OFFSET>,
            StopWdsTransportServices: StopWdsTransportServices::<Identity, Impl, OFFSET>,
            RestartWdsTransportServices: RestartWdsTransportServices::<Identity, Impl, OFFSET>,
            NotifyWdsTransportServices: NotifyWdsTransportServices::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportConfigurationManager2_Impl: ::windows_core::BaseImpl + IWdsTransportConfigurationManager_Impl {
    fn MulticastSessionPolicy(this: &Self::This) -> ::windows_core::Result<IWdsTransportMulticastSessionPolicy>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportConfigurationManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportConfigurationManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportConfigurationManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MulticastSessionPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MulticastSessionPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportmulticastsessionpolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportConfigurationManager2_Vtbl {
            base__: <IWdsTransportConfigurationManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MulticastSessionPolicy: MulticastSessionPolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportContent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Namespace(this: &Self::This) -> ::windows_core::Result<IWdsTransportNamespace>;
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RetrieveSessions(this: &Self::This) -> ::windows_core::Result<IWdsTransportCollection>;
    fn Terminate(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Namespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Namespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RetrieveSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportsessions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveSessions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportsessions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this).into())
        }
        IWdsTransportContent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            RetrieveSessions: RetrieveSessions::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportContentProvider_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FilePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InitializationRoutine(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportContentProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportContentProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FilePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszfilepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FilePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszfilepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializationRoutine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszinitializationroutine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InitializationRoutine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszinitializationroutine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportContentProvider_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            FilePath: FilePath::<Identity, Impl, OFFSET>,
            InitializationRoutine: InitializationRoutine::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportDiagnosticsPolicy_Impl: ::windows_core::BaseImpl + IWdsTransportCacheable_Impl {
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Components(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetComponents(this: &Self::This, ulcomponents: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportDiagnosticsPolicy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportCacheable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportDiagnosticsPolicy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn Components<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcomponents: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Components(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcomponents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetComponents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcomponents: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComponents(this, ::core::mem::transmute_copy(&ulcomponents)).into())
        }
        IWdsTransportDiagnosticsPolicy_Vtbl {
            base__: <IWdsTransportCacheable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Components: Components::<Identity, Impl, OFFSET>,
            SetComponents: SetComponents::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetWdsTransportServer(this: &Self::This, bszservername: &::windows_core::BSTR) -> ::windows_core::Result<IWdsTransportServer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWdsTransportServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwdstransportserver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWdsTransportServer(this, ::core::mem::transmute(&bszservername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportserver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWdsTransportServer: GetWdsTransportServer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportMulticastSessionPolicy_Impl: ::windows_core::BaseImpl + IWdsTransportCacheable_Impl {
    fn SlowClientHandling(this: &Self::This) -> ::windows_core::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE>;
    fn SetSlowClientHandling(this: &Self::This, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_core::Result<()>;
    fn AutoDisconnectThreshold(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetAutoDisconnectThreshold(this: &Self::This, ulthreshold: u32) -> ::windows_core::Result<()>;
    fn MultistreamStreamCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMultistreamStreamCount(this: &Self::This, ulstreamcount: u32) -> ::windows_core::Result<()>;
    fn SlowClientFallback(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSlowClientFallback(this: &Self::This, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportMulticastSessionPolicy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportCacheable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportMulticastSessionPolicy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SlowClientHandling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SlowClientHandling(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pslowclienthandling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSlowClientHandling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSlowClientHandling(this, ::core::mem::transmute_copy(&slowclienthandling)).into())
        }
        unsafe extern "system" fn AutoDisconnectThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulthreshold: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoDisconnectThreshold(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulthreshold, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoDisconnectThreshold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulthreshold: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoDisconnectThreshold(this, ::core::mem::transmute_copy(&ulthreshold)).into())
        }
        unsafe extern "system" fn MultistreamStreamCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulstreamcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MultistreamStreamCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstreamcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMultistreamStreamCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstreamcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultistreamStreamCount(this, ::core::mem::transmute_copy(&ulstreamcount)).into())
        }
        unsafe extern "system" fn SlowClientFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclientfallback: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SlowClientFallback(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbclientfallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSlowClientFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSlowClientFallback(this, ::core::mem::transmute_copy(&bclientfallback)).into())
        }
        IWdsTransportMulticastSessionPolicy_Vtbl {
            base__: <IWdsTransportCacheable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SlowClientHandling: SlowClientHandling::<Identity, Impl, OFFSET>,
            SetSlowClientHandling: SetSlowClientHandling::<Identity, Impl, OFFSET>,
            AutoDisconnectThreshold: AutoDisconnectThreshold::<Identity, Impl, OFFSET>,
            SetAutoDisconnectThreshold: SetAutoDisconnectThreshold::<Identity, Impl, OFFSET>,
            MultistreamStreamCount: MultistreamStreamCount::<Identity, Impl, OFFSET>,
            SetMultistreamStreamCount: SetMultistreamStreamCount::<Identity, Impl, OFFSET>,
            SlowClientFallback: SlowClientFallback::<Identity, Impl, OFFSET>,
            SetSlowClientFallback: SetSlowClientFallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespace_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE>;
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bszname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFriendlyName(this: &Self::This, bszfriendlyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bszdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ContentProvider(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetContentProvider(this: &Self::This, bszcontentprovider: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Configuration(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetConfiguration(this: &Self::This, bszconfiguration: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Registered(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Tombstoned(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TombstoneTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TransmissionStarted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Register(this: &Self::This) -> ::windows_core::Result<()>;
    fn Deregister(this: &Self::This, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IWdsTransportNamespace>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn RetrieveContents(this: &Self::This) -> ::windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportNamespace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportNamespace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bszname)).into())
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszfriendlyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFriendlyName(this, ::core::mem::transmute(&bszfriendlyname)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bszdescription)).into())
        }
        unsafe extern "system" fn ContentProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszcontentprovider: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszcontentprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContentProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContentProvider(this, ::core::mem::transmute(&bszcontentprovider)).into())
        }
        unsafe extern "system" fn Configuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszconfiguration: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Configuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszconfiguration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszconfiguration: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConfiguration(this, ::core::mem::transmute(&bszconfiguration)).into())
        }
        unsafe extern "system" fn Registered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbregistered: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Registered(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbregistered, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Tombstoned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbtombstoned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Tombstoned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbtombstoned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TombstoneTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptombstonetime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TombstoneTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptombstonetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransmissionStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbtransmissionstarted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransmissionStarted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbtransmissionstarted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Register<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Register(this).into())
        }
        unsafe extern "system" fn Deregister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deregister(this, ::core::mem::transmute_copy(&bterminatesessions)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespaceclone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespaceclone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn RetrieveContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportcontents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveContents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportcontents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportNamespace_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ContentProvider: ContentProvider::<Identity, Impl, OFFSET>,
            SetContentProvider: SetContentProvider::<Identity, Impl, OFFSET>,
            Configuration: Configuration::<Identity, Impl, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            Registered: Registered::<Identity, Impl, OFFSET>,
            Tombstoned: Tombstoned::<Identity, Impl, OFFSET>,
            TombstoneTime: TombstoneTime::<Identity, Impl, OFFSET>,
            TransmissionStarted: TransmissionStarted::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Deregister: Deregister::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            RetrieveContents: RetrieveContents::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceAutoCast_Impl: ::windows_core::BaseImpl + IWdsTransportNamespace_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportNamespaceAutoCast {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportNamespace);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceAutoCast_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportNamespaceAutoCast {
    const VTABLE: Self::Vtable = { IWdsTransportNamespaceAutoCast_Vtbl { base__: <IWdsTransportNamespace as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn CreateNamespace(this: &Self::This, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: &::windows_core::BSTR, bszcontentprovider: &::windows_core::BSTR, bszconfiguration: &::windows_core::BSTR) -> ::windows_core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespace(this: &Self::This, bsznamespacename: &::windows_core::BSTR) -> ::windows_core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespaces(this: &Self::This, bszcontentprovider: &::windows_core::BSTR, bsznamespacename: &::windows_core::BSTR, bincludetombstones: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportNamespaceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportNamespaceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszcontentprovider: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszconfiguration: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNamespace(this, ::core::mem::transmute_copy(&namespacetype), ::core::mem::transmute(&bsznamespacename), ::core::mem::transmute(&bszcontentprovider), ::core::mem::transmute(&bszconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RetrieveNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsznamespacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveNamespace(this, ::core::mem::transmute(&bsznamespacename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RetrieveNamespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::std::mem::MaybeUninit<::windows_core::BSTR>, bsznamespacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bincludetombstones: super::super::Foundation::VARIANT_BOOL, ppwdstransportnamespaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveNamespaces(this, ::core::mem::transmute(&bszcontentprovider), ::core::mem::transmute(&bsznamespacename), ::core::mem::transmute_copy(&bincludetombstones)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportNamespaceManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateNamespace: CreateNamespace::<Identity, Impl, OFFSET>,
            RetrieveNamespace: RetrieveNamespace::<Identity, Impl, OFFSET>,
            RetrieveNamespaces: RetrieveNamespaces::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceScheduledCast_Impl: ::windows_core::BaseImpl + IWdsTransportNamespace_Impl {
    fn StartTransmission(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportNamespaceScheduledCast {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportNamespace);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportNamespaceScheduledCast {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartTransmission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartTransmission(this).into())
        }
        IWdsTransportNamespaceScheduledCast_Vtbl {
            base__: <IWdsTransportNamespace as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartTransmission: StartTransmission::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceScheduledCastAutoStart_Impl: ::windows_core::BaseImpl + IWdsTransportNamespaceScheduledCast_Impl {
    fn MinimumClients(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMinimumClients(this: &Self::This, ulminimumclients: u32) -> ::windows_core::Result<()>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetStartTime(this: &Self::This, starttime: f64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportNamespaceScheduledCastAutoStart {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportNamespaceScheduledCast);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportNamespaceScheduledCastAutoStart {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MinimumClients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulminimumclients: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinimumClients(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulminimumclients, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinimumClients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulminimumclients: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinimumClients(this, ::core::mem::transmute_copy(&ulminimumclients)).into())
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstarttime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstarttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartTime(this, ::core::mem::transmute_copy(&starttime)).into())
        }
        IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
            base__: <IWdsTransportNamespaceScheduledCast as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MinimumClients: MinimumClients::<Identity, Impl, OFFSET>,
            SetMinimumClients: SetMinimumClients::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceScheduledCastManualStart_Impl: ::windows_core::BaseImpl + IWdsTransportNamespaceScheduledCast_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportNamespaceScheduledCastManualStart {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportNamespaceScheduledCast);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastManualStart_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportNamespaceScheduledCastManualStart {
    const VTABLE: Self::Vtable = { IWdsTransportNamespaceScheduledCastManualStart_Vtbl { base__: <IWdsTransportNamespaceScheduledCast as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServer_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetupManager(this: &Self::This) -> ::windows_core::Result<IWdsTransportSetupManager>;
    fn ConfigurationManager(this: &Self::This) -> ::windows_core::Result<IWdsTransportConfigurationManager>;
    fn NamespaceManager(this: &Self::This) -> ::windows_core::Result<IWdsTransportNamespaceManager>;
    fn DisconnectClient(this: &Self::This, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportServer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportServer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetupManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportsetupmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetupManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportsetupmanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConfigurationManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportconfigurationmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConfigurationManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportconfigurationmanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NamespaceManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespacemanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamespaceManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespacemanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisconnectClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectClient(this, ::core::mem::transmute_copy(&ulclientid), ::core::mem::transmute_copy(&disconnectiontype)).into())
        }
        IWdsTransportServer_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetupManager: SetupManager::<Identity, Impl, OFFSET>,
            ConfigurationManager: ConfigurationManager::<Identity, Impl, OFFSET>,
            NamespaceManager: NamespaceManager::<Identity, Impl, OFFSET>,
            DisconnectClient: DisconnectClient::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServer2_Impl: ::windows_core::BaseImpl + IWdsTransportServer_Impl {
    fn TftpManager(this: &Self::This) -> ::windows_core::Result<IWdsTransportTftpManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportServer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportServer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportServer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TftpManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TftpManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransporttftpmanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportServer2_Vtbl { base__: <IWdsTransportServer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, TftpManager: TftpManager::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServicePolicy_Impl: ::windows_core::BaseImpl + IWdsTransportCacheable_Impl {
    fn get_IpAddressSource(this: &Self::This, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>;
    fn put_IpAddressSource(this: &Self::This, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_core::Result<()>;
    fn get_StartIpAddress(this: &Self::This, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_StartIpAddress(this: &Self::This, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_EndIpAddress(this: &Self::This, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_EndIpAddress(this: &Self::This, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartPort(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetStartPort(this: &Self::This, ulstartport: u32) -> ::windows_core::Result<()>;
    fn EndPort(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetEndPort(this: &Self::This, ulendport: u32) -> ::windows_core::Result<()>;
    fn NetworkProfile(this: &Self::This) -> ::windows_core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE>;
    fn SetNetworkProfile(this: &Self::This, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportServicePolicy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportCacheable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportServicePolicy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_IpAddressSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_IpAddressSource(this, ::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psourcetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_IpAddressSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_IpAddressSource(this, ::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&sourcetype)).into())
        }
        unsafe extern "system" fn get_StartIpAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_StartIpAddress(this, ::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszstartipaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_StartIpAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_StartIpAddress(this, ::core::mem::transmute_copy(&addresstype), ::core::mem::transmute(&bszstartipaddress)).into())
        }
        unsafe extern "system" fn get_EndIpAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_EndIpAddress(this, ::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszendipaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_EndIpAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_EndIpAddress(this, ::core::mem::transmute_copy(&addresstype), ::core::mem::transmute(&bszendipaddress)).into())
        }
        unsafe extern "system" fn StartPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulstartport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstartport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartport: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartPort(this, ::core::mem::transmute_copy(&ulstartport)).into())
        }
        unsafe extern "system" fn EndPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulendport: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulendport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEndPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulendport: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEndPort(this, ::core::mem::transmute_copy(&ulendport)).into())
        }
        unsafe extern "system" fn NetworkProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkProfile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprofiletype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkProfile(this, ::core::mem::transmute_copy(&profiletype)).into())
        }
        IWdsTransportServicePolicy_Vtbl {
            base__: <IWdsTransportCacheable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_IpAddressSource: get_IpAddressSource::<Identity, Impl, OFFSET>,
            put_IpAddressSource: put_IpAddressSource::<Identity, Impl, OFFSET>,
            get_StartIpAddress: get_StartIpAddress::<Identity, Impl, OFFSET>,
            put_StartIpAddress: put_StartIpAddress::<Identity, Impl, OFFSET>,
            get_EndIpAddress: get_EndIpAddress::<Identity, Impl, OFFSET>,
            put_EndIpAddress: put_EndIpAddress::<Identity, Impl, OFFSET>,
            StartPort: StartPort::<Identity, Impl, OFFSET>,
            SetStartPort: SetStartPort::<Identity, Impl, OFFSET>,
            EndPort: EndPort::<Identity, Impl, OFFSET>,
            SetEndPort: SetEndPort::<Identity, Impl, OFFSET>,
            NetworkProfile: NetworkProfile::<Identity, Impl, OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServicePolicy2_Impl: ::windows_core::BaseImpl + IWdsTransportServicePolicy_Impl {
    fn UdpPortPolicy(this: &Self::This) -> ::windows_core::Result<WDSTRANSPORT_UDP_PORT_POLICY>;
    fn SetUdpPortPolicy(this: &Self::This, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_core::Result<()>;
    fn TftpMaximumBlockSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetTftpMaximumBlockSize(this: &Self::This, ultftpmaximumblocksize: u32) -> ::windows_core::Result<()>;
    fn EnableTftpVariableWindowExtension(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableTftpVariableWindowExtension(this: &Self::This, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportServicePolicy2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportServicePolicy);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportServicePolicy2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UdpPortPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UdpPortPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pudpportpolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUdpPortPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUdpPortPolicy(this, ::core::mem::transmute_copy(&udpportpolicy)).into())
        }
        unsafe extern "system" fn TftpMaximumBlockSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TftpMaximumBlockSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultftpmaximumblocksize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTftpMaximumBlockSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ultftpmaximumblocksize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTftpMaximumBlockSize(this, ::core::mem::transmute_copy(&ultftpmaximumblocksize)).into())
        }
        unsafe extern "system" fn EnableTftpVariableWindowExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabletftpvariablewindowextension: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnableTftpVariableWindowExtension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabletftpvariablewindowextension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnableTftpVariableWindowExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnableTftpVariableWindowExtension(this, ::core::mem::transmute_copy(&benabletftpvariablewindowextension)).into())
        }
        IWdsTransportServicePolicy2_Vtbl {
            base__: <IWdsTransportServicePolicy as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UdpPortPolicy: UdpPortPolicy::<Identity, Impl, OFFSET>,
            SetUdpPortPolicy: SetUdpPortPolicy::<Identity, Impl, OFFSET>,
            TftpMaximumBlockSize: TftpMaximumBlockSize::<Identity, Impl, OFFSET>,
            SetTftpMaximumBlockSize: SetTftpMaximumBlockSize::<Identity, Impl, OFFSET>,
            EnableTftpVariableWindowExtension: EnableTftpVariableWindowExtension::<Identity, Impl, OFFSET>,
            SetEnableTftpVariableWindowExtension: SetEnableTftpVariableWindowExtension::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportSession_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Content(this: &Self::This) -> ::windows_core::Result<IWdsTransportContent>;
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn NetworkInterfaceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NetworkInterfaceAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TransferRate(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MasterClientId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn RetrieveClients(this: &Self::This) -> ::windows_core::Result<IWdsTransportCollection>;
    fn Terminate(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Content<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportcontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Content(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportcontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetworkInterfaceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfacename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkInterfaceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsznetworkinterfacename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetworkInterfaceAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfaceaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkInterfaceAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsznetworkinterfaceaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransferRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pultransferrate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransferRate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultransferrate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MasterClientId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulmasterclientid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MasterClientId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulmasterclientid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RetrieveClients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransportclients: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveClients(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportclients, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this).into())
        }
        IWdsTransportSession_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Content: Content::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            NetworkInterfaceName: NetworkInterfaceName::<Identity, Impl, OFFSET>,
            NetworkInterfaceAddress: NetworkInterfaceAddress::<Identity, Impl, OFFSET>,
            TransferRate: TransferRate::<Identity, Impl, OFFSET>,
            MasterClientId: MasterClientId::<Identity, Impl, OFFSET>,
            RetrieveClients: RetrieveClients::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportSetupManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Version(this: &Self::This) -> ::windows_core::Result<u64>;
    fn InstalledFeatures(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Protocols(this: &Self::This) -> ::windows_core::Result<u32>;
    fn RegisterContentProvider(this: &Self::This, bszname: &::windows_core::BSTR, bszdescription: &::windows_core::BSTR, bszfilepath: &::windows_core::BSTR, bszinitializationroutine: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeregisterContentProvider(this: &Self::This, bszname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportSetupManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportSetupManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstalledFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulinstalledfeatures: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstalledFeatures(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulinstalledfeatures, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Protocols<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulprotocols: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocols(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprotocols, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterContentProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszinitializationroutine: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterContentProvider(this, ::core::mem::transmute(&bszname), ::core::mem::transmute(&bszdescription), ::core::mem::transmute(&bszfilepath), ::core::mem::transmute(&bszinitializationroutine)).into())
        }
        unsafe extern "system" fn DeregisterContentProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeregisterContentProvider(this, ::core::mem::transmute(&bszname)).into())
        }
        IWdsTransportSetupManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Version: Version::<Identity, Impl, OFFSET>,
            InstalledFeatures: InstalledFeatures::<Identity, Impl, OFFSET>,
            Protocols: Protocols::<Identity, Impl, OFFSET>,
            RegisterContentProvider: RegisterContentProvider::<Identity, Impl, OFFSET>,
            DeregisterContentProvider: DeregisterContentProvider::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportSetupManager2_Impl: ::windows_core::BaseImpl + IWdsTransportSetupManager_Impl {
    fn TftpCapabilities(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ContentProviders(this: &Self::This) -> ::windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportSetupManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWdsTransportSetupManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportSetupManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TftpCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pultftpcapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TftpCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultftpcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContentProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprovidercollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentProviders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovidercollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportSetupManager2_Vtbl {
            base__: <IWdsTransportSetupManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TftpCapabilities: TftpCapabilities::<Identity, Impl, OFFSET>,
            ContentProviders: ContentProviders::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportTftpClient_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn FileName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IpAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Timeout(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CurrentFileOffset(this: &Self::This) -> ::windows_core::Result<u64>;
    fn FileSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn BlockSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn WindowSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportTftpClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportTftpClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszfilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IpAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IpAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszipaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Timeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pultimeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFileOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pul64currentoffset: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFileOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pul64currentoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pul64filesize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pul64filesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BlockSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulblocksize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BlockSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulblocksize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WindowSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulwindowsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WindowSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulwindowsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportTftpClient_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FileName: FileName::<Identity, Impl, OFFSET>,
            IpAddress: IpAddress::<Identity, Impl, OFFSET>,
            Timeout: Timeout::<Identity, Impl, OFFSET>,
            CurrentFileOffset: CurrentFileOffset::<Identity, Impl, OFFSET>,
            FileSize: FileSize::<Identity, Impl, OFFSET>,
            BlockSize: BlockSize::<Identity, Impl, OFFSET>,
            WindowSize: WindowSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportTftpManager_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn RetrieveTftpClients(this: &Self::This) -> ::windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWdsTransportTftpManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWdsTransportTftpManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RetrieveTftpClients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWdsTransportTftpManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpclients: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveTftpClients(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransporttftpclients, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWdsTransportTftpManager_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RetrieveTftpClients: RetrieveTftpClients::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
