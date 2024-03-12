#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetworkConnections_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<INetworkConnection>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNetworkConnections>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumNetworkConnections {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetworkConnections {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNetworkConnections_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetworks_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<INetwork>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNetworks>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumNetworks {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetworks {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNetworks_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetwork_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, sznetworknewname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, szdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetNetworkId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDomainType(this: &Self::This) -> ::windows_core::Result<NLM_DOMAIN_TYPE>;
    fn GetNetworkConnections(this: &Self::This) -> ::windows_core::Result<IEnumNetworkConnections>;
    fn GetTimeCreatedAndConnected(this: &Self::This, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows_core::Result<()>;
    fn IsConnectedToInternet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(this: &Self::This) -> ::windows_core::Result<NLM_CONNECTIVITY>;
    fn GetCategory(this: &Self::This) -> ::windows_core::Result<NLM_NETWORK_CATEGORY>;
    fn SetCategory(this: &Self::This, newcategory: NLM_NETWORK_CATEGORY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetwork {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetwork {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psznetworkname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psznetworkname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sznetworknewname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&sznetworknewname)).into())
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&szdescription)).into())
        }
        unsafe extern "system" fn GetNetworkId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgdguidnetworkid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDomainType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnetworktype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetworkconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTimeCreatedAndConnected(this, ::core::mem::transmute_copy(&pdwlowdatetimecreated), ::core::mem::transmute_copy(&pdwhighdatetimecreated), ::core::mem::transmute_copy(&pdwlowdatetimeconnected), ::core::mem::transmute_copy(&pdwhighdatetimeconnected)).into())
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnectedToInternet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcategory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCategory(this, ::core::mem::transmute_copy(&newcategory)).into())
        }
        INetwork_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetNetworkId: GetNetworkId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetTimeCreatedAndConnected: GetTimeCreatedAndConnected::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            SetCategory: SetCategory::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetwork2_Impl: ::windows_core::BaseImpl + INetwork_Impl {
    fn IsDomainAuthenticatedBy(this: &Self::This, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetwork2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INetwork);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetwork2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetwork2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDomainAuthenticatedBy(this, ::core::mem::transmute_copy(&domainauthenticationkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetwork2_Vtbl {
            base__: <INetwork as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkConnection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetNetwork(this: &Self::This) -> ::windows_core::Result<INetwork>;
    fn IsConnectedToInternet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(this: &Self::This) -> ::windows_core::Result<NLM_CONNECTIVITY>;
    fn GetConnectionId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetAdapterId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDomainType(this: &Self::This) -> ::windows_core::Result<NLM_DOMAIN_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetworkConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNetwork<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetwork(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnectedToInternet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgdconnectionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAdapterId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAdapterId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgdadapterid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDomainType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdomaintype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetworkConnection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, Impl, OFFSET>,
            GetAdapterId: GetAdapterId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkConnection2_Impl: ::windows_core::BaseImpl + INetworkConnection_Impl {
    fn IsDomainAuthenticatedBy(this: &Self::This, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetworkConnection2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INetworkConnection);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkConnection2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDomainAuthenticatedBy(this, ::core::mem::transmute_copy(&domainauthenticationkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetworkConnection2_Vtbl {
            base__: <INetworkConnection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkConnectionCost_Impl: ::windows_core::BaseImpl {
    fn GetCost(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDataPlanStatus(this: &Self::This, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetworkConnectionCost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkConnectionCost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcost: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCost(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcost, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataPlanStatus(this, ::core::mem::transmute_copy(&pdataplanstatus)).into())
        }
        INetworkConnectionCost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetworkConnectionCostEvents_Impl: ::windows_core::BaseImpl {
    fn ConnectionCostChanged(this: &Self::This, connectionid: &::windows_core::GUID, newcost: u32) -> ::windows_core::Result<()>;
    fn ConnectionDataPlanStatusChanged(this: &Self::This, connectionid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetworkConnectionCostEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkConnectionCostEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectionCostChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID, newcost: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectionCostChanged(this, ::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&newcost)).into())
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectionDataPlanStatusChanged(this, ::core::mem::transmute(&connectionid)).into())
        }
        INetworkConnectionCostEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectionCostChanged: ConnectionCostChanged::<Identity, Impl, OFFSET>,
            ConnectionDataPlanStatusChanged: ConnectionDataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetworkConnectionEvents_Impl: ::windows_core::BaseImpl {
    fn NetworkConnectionConnectivityChanged(this: &Self::This, connectionid: &::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::Result<()>;
    fn NetworkConnectionPropertyChanged(this: &Self::This, connectionid: &::windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetworkConnectionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkConnectionEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NetworkConnectionConnectivityChanged(this, ::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&newconnectivity)).into())
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NetworkConnectionPropertyChanged(this, ::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&flags)).into())
        }
        INetworkConnectionEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NetworkConnectionConnectivityChanged: NetworkConnectionConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkConnectionPropertyChanged: NetworkConnectionPropertyChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkCostManager_Impl: ::windows_core::BaseImpl {
    fn GetCost(this: &Self::This, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
    fn GetDataPlanStatus(this: &Self::This, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
    fn SetDestinationAddresses(this: &Self::This, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetworkCostManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkCostManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCost(this, ::core::mem::transmute_copy(&pcost), ::core::mem::transmute_copy(&pdestipaddr)).into())
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataPlanStatus(this, ::core::mem::transmute_copy(&pdataplanstatus), ::core::mem::transmute_copy(&pdestipaddr)).into())
        }
        unsafe extern "system" fn SetDestinationAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestinationAddresses(this, ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&pdestipaddrlist), ::core::mem::transmute_copy(&bappend)).into())
        }
        INetworkCostManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
            SetDestinationAddresses: SetDestinationAddresses::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetworkCostManagerEvents_Impl: ::windows_core::BaseImpl {
    fn CostChanged(this: &Self::This, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
    fn DataPlanStatusChanged(this: &Self::This, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetworkCostManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkCostManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CostChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CostChanged(this, ::core::mem::transmute_copy(&newcost), ::core::mem::transmute_copy(&pdestaddr)).into())
        }
        unsafe extern "system" fn DataPlanStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DataPlanStatusChanged(this, ::core::mem::transmute_copy(&pdestaddr)).into())
        }
        INetworkCostManagerEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CostChanged: CostChanged::<Identity, Impl, OFFSET>,
            DataPlanStatusChanged: DataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetworkEvents_Impl: ::windows_core::BaseImpl {
    fn NetworkAdded(this: &Self::This, networkid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn NetworkDeleted(this: &Self::This, networkid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn NetworkConnectivityChanged(this: &Self::This, networkid: &::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::Result<()>;
    fn NetworkPropertyChanged(this: &Self::This, networkid: &::windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetworkEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NetworkAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NetworkAdded(this, ::core::mem::transmute(&networkid)).into())
        }
        unsafe extern "system" fn NetworkDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NetworkDeleted(this, ::core::mem::transmute(&networkid)).into())
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NetworkConnectivityChanged(this, ::core::mem::transmute(&networkid), ::core::mem::transmute_copy(&newconnectivity)).into())
        }
        unsafe extern "system" fn NetworkPropertyChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NetworkPropertyChanged(this, ::core::mem::transmute(&networkid), ::core::mem::transmute_copy(&flags)).into())
        }
        INetworkEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NetworkAdded: NetworkAdded::<Identity, Impl, OFFSET>,
            NetworkDeleted: NetworkDeleted::<Identity, Impl, OFFSET>,
            NetworkConnectivityChanged: NetworkConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkPropertyChanged: NetworkPropertyChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkListManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetNetworks(this: &Self::This, flags: NLM_ENUM_NETWORK) -> ::windows_core::Result<IEnumNetworks>;
    fn GetNetwork(this: &Self::This, gdnetworkid: &::windows_core::GUID) -> ::windows_core::Result<INetwork>;
    fn GetNetworkConnections(this: &Self::This) -> ::windows_core::Result<IEnumNetworkConnections>;
    fn GetNetworkConnection(this: &Self::This, gdnetworkconnectionid: &::windows_core::GUID) -> ::windows_core::Result<INetworkConnection>;
    fn IsConnectedToInternet(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(this: &Self::This) -> ::windows_core::Result<NLM_CONNECTIVITY>;
    fn SetSimulatedProfileInfo(this: &Self::This, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows_core::Result<()>;
    fn ClearSimulatedProfileInfo(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetworkListManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkListManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNetworks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworks(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNetwork<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows_core::GUID, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetwork(this, ::core::mem::transmute(&gdnetworkid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNetworkConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows_core::GUID, ppnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNetworkConnection(this, ::core::mem::transmute(&gdnetworkconnectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworkconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnectedToInternet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSimulatedProfileInfo(this, ::core::mem::transmute_copy(&psimulatedinfo)).into())
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearSimulatedProfileInfo(this).into())
        }
        INetworkListManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNetworks: GetNetworks::<Identity, Impl, OFFSET>,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetNetworkConnection: GetNetworkConnection::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            SetSimulatedProfileInfo: SetSimulatedProfileInfo::<Identity, Impl, OFFSET>,
            ClearSimulatedProfileInfo: ClearSimulatedProfileInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INetworkListManagerEvents_Impl: ::windows_core::BaseImpl {
    fn ConnectivityChanged(this: &Self::This, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetworkListManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkListManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectivityChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkListManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectivityChanged(this, ::core::mem::transmute_copy(&newconnectivity)).into())
        }
        INetworkListManagerEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectivityChanged: ConnectivityChanged::<Identity, Impl, OFFSET>,
        }
    };
}
