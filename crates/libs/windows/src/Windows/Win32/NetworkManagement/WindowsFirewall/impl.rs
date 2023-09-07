#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDynamicPortMapping_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RemoteHost(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExternalPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Protocol(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InternalPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn InternalClient(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LeaseDuration(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RenewLease(this: &Self::This, lleasedurationdesired: i32) -> ::windows_core::Result<i32>;
    fn EditInternalClient(this: &Self::This, bstrinternalclient: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enable(this: &Self::This, vb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EditDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EditInternalPort(this: &Self::This, linternalport: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDynamicPortMapping {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDynamicPortMapping {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExternalIPAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExternalIPAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoteHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteHost(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExternalPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InternalPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InternalClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InternalClient(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LeaseDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LeaseDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RenewLease<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RenewLease(this, ::core::mem::transmute_copy(&lleasedurationdesired)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pleasedurationreturned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EditInternalClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditInternalClient(this, ::core::mem::transmute(&bstrinternalclient)).into())
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this, ::core::mem::transmute_copy(&vb)).into())
        }
        unsafe extern "system" fn EditDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn EditInternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditInternalPort(this, ::core::mem::transmute_copy(&linternalport)).into())
        }
        IDynamicPortMapping_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExternalIPAddress: ExternalIPAddress::<Identity, Impl, OFFSET>,
            RemoteHost: RemoteHost::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            InternalClient: InternalClient::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            LeaseDuration: LeaseDuration::<Identity, Impl, OFFSET>,
            RenewLease: RenewLease::<Identity, Impl, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            EditDescription: EditDescription::<Identity, Impl, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDynamicPortMappingCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, bstrremotehost: &::windows_core::BSTR, lexternalport: i32, bstrprotocol: &::windows_core::BSTR) -> ::windows_core::Result<IDynamicPortMapping>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Remove(this: &Self::This, bstrremotehost: &::windows_core::BSTR, lexternalport: i32, bstrprotocol: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, bstrremotehost: &::windows_core::BSTR, lexternalport: i32, bstrprotocol: &::windows_core::BSTR, linternalport: i32, bstrinternalclient: &::windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &::windows_core::BSTR, lleaseduration: i32) -> ::windows_core::Result<IDynamicPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDynamicPortMappingCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDynamicPortMappingCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdpm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdpm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternalport: i32, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, lleaseduration: i32, ppdpm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&linternalport), ::core::mem::transmute(&bstrinternalclient), ::core::mem::transmute_copy(&benabled), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute_copy(&lleaseduration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdpm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDynamicPortMappingCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumNetConnection_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<INetConnection>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNetConnection>;
}
impl ::windows_core::Iids for IEnumNetConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNetConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingEveryConnection_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNetSharingEveryConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumNetSharingEveryConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetSharingEveryConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNetSharingEveryConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingPortMapping_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNetSharingPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumNetSharingPortMapping {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetSharingPortMapping {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNetSharingPortMapping_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingPrivateConnection_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNetSharingPrivateConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumNetSharingPrivateConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetSharingPrivateConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNetSharingPrivateConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetSharingPublicConnection_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumNetSharingPublicConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumNetSharingPublicConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumNetSharingPublicConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumNetSharingPublicConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INATEventManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetExternalIPAddressCallback(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetNumberOfEntriesCallback(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INATEventManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INATEventManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INATEventManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetExternalIPAddressCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INATEventManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExternalIPAddressCallback(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INATEventManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumberOfEntriesCallback(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        INATEventManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetExternalIPAddressCallback: SetExternalIPAddressCallback::<Identity, Impl, OFFSET>,
            SetNumberOfEntriesCallback: SetNumberOfEntriesCallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INATExternalIPAddressCallback_Impl: ::windows_core::BaseImpl {
    fn NewExternalIPAddress(this: &Self::This, bstrnewexternalipaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INATExternalIPAddressCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INATExternalIPAddressCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INATExternalIPAddressCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NewExternalIPAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INATExternalIPAddressCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewExternalIPAddress(this, ::core::mem::transmute(&bstrnewexternalipaddress)).into())
        }
        INATExternalIPAddressCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NewExternalIPAddress: NewExternalIPAddress::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INATNumberOfEntriesCallback_Impl: ::windows_core::BaseImpl {
    fn NewNumberOfEntries(this: &Self::This, lnewnumberofentries: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INATNumberOfEntriesCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INATNumberOfEntriesCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INATNumberOfEntriesCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NewNumberOfEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INATNumberOfEntriesCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewNumberOfEntries(this, ::core::mem::transmute_copy(&lnewnumberofentries)).into())
        }
        INATNumberOfEntriesCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NewNumberOfEntries: NewNumberOfEntries::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INetConnection_Impl: ::windows_core::BaseImpl {
    fn Connect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Duplicate(this: &Self::This, pszwduplicatename: &::windows_core::PCWSTR) -> ::windows_core::Result<INetConnection>;
    fn GetProperties(this: &Self::This) -> ::windows_core::Result<*mut NETCON_PROPERTIES>;
    fn GetUiObjectClassId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Rename(this: &Self::This, pszwnewname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Duplicate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwduplicatename: ::windows_core::PCWSTR, ppcon: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Duplicate(this, ::core::mem::transmute(&pszwduplicatename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUiObjectClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUiObjectClassId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszwnewname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rename(this, ::core::mem::transmute(&pszwnewname)).into())
        }
        INetConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Duplicate: Duplicate::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetUiObjectClassId: GetUiObjectClassId::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetConnectionConnectUi_Impl: ::windows_core::BaseImpl {
    fn SetConnection(this: &Self::This, pcon: ::core::option::Option<&INetConnection>) -> ::windows_core::Result<()>;
    fn Connect(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetConnectionConnectUi {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetConnectionConnectUi {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcon: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnection(this, ::windows_core::from_raw_borrowed(&pcon)).into())
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into())
        }
        INetConnectionConnectUi_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetConnection: SetConnection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INetConnectionManager_Impl: ::windows_core::BaseImpl {
    fn EnumConnections(this: &Self::This, flags: NETCONMGR_ENUM_FLAGS) -> ::windows_core::Result<IEnumNetConnection>;
}
impl ::windows_core::Iids for INetConnectionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetConnectionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumConnections(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetConnectionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumConnections: EnumConnections::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetConnectionProps_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Guid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Status(this: &Self::This) -> ::windows_core::Result<NETCON_STATUS>;
    fn MediaType(this: &Self::This) -> ::windows_core::Result<NETCON_MEDIATYPE>;
    fn Characteristics(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetConnectionProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetConnectionProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Guid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Guid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmediatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Characteristics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Characteristics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetConnectionProps_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Guid: Guid::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Characteristics: Characteristics::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwAuthorizedApplication_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ProcessImageFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProcessImageFileName(this: &Self::This, imagefilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IpVersion(this: &Self::This) -> ::windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(this: &Self::This, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()>;
    fn Scope(this: &Self::This) -> ::windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(this: &Self::This, scope: NET_FW_SCOPE) -> ::windows_core::Result<()>;
    fn RemoteAddresses(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemoteAddresses(this: &Self::This, remoteaddrs: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwAuthorizedApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwAuthorizedApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn ProcessImageFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProcessImageFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagefilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProcessImageFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProcessImageFileName(this, ::core::mem::transmute(&imagefilename)).into())
        }
        unsafe extern "system" fn IpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IpVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ipversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpVersion(this, ::core::mem::transmute_copy(&ipversion)).into())
        }
        unsafe extern "system" fn Scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScope(this, ::core::mem::transmute_copy(&scope)).into())
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remoteaddrs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteAddresses(this, ::core::mem::transmute(&remoteaddrs)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        INetFwAuthorizedApplication_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            ProcessImageFileName: ProcessImageFileName::<Identity, Impl, OFFSET>,
            SetProcessImageFileName: SetProcessImageFileName::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwAuthorizedApplications_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, app: ::core::option::Option<&INetFwAuthorizedApplication>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, imagefilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Item(this: &Self::This, imagefilename: &::windows_core::BSTR) -> ::windows_core::Result<INetFwAuthorizedApplication>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwAuthorizedApplications {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwAuthorizedApplications {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, app: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&app)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&imagefilename)).into())
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, app: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&imagefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(app, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwAuthorizedApplications_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwIcmpSettings_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AllowOutboundDestinationUnreachable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundDestinationUnreachable(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowRedirect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowRedirect(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowInboundEchoRequest(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundEchoRequest(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowOutboundTimeExceeded(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundTimeExceeded(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowOutboundParameterProblem(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundParameterProblem(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowOutboundSourceQuench(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundSourceQuench(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowInboundRouterRequest(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundRouterRequest(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowInboundTimestampRequest(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundTimestampRequest(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowInboundMaskRequest(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundMaskRequest(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowOutboundPacketTooBig(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundPacketTooBig(this: &Self::This, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwIcmpSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwIcmpSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowOutboundDestinationUnreachable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowOutboundDestinationUnreachable(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowRedirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowRedirect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowRedirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowRedirect(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowInboundEchoRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowInboundEchoRequest(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowOutboundTimeExceeded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowOutboundTimeExceeded(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowOutboundParameterProblem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowOutboundParameterProblem(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowOutboundSourceQuench(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowOutboundSourceQuench(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowInboundRouterRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowInboundRouterRequest(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowInboundTimestampRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowInboundTimestampRequest(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowInboundMaskRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowInboundMaskRequest(this, ::core::mem::transmute_copy(&allow)).into())
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowOutboundPacketTooBig(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowOutboundPacketTooBig(this, ::core::mem::transmute_copy(&allow)).into())
        }
        INetFwIcmpSettings_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllowOutboundDestinationUnreachable: AllowOutboundDestinationUnreachable::<Identity, Impl, OFFSET>,
            SetAllowOutboundDestinationUnreachable: SetAllowOutboundDestinationUnreachable::<Identity, Impl, OFFSET>,
            AllowRedirect: AllowRedirect::<Identity, Impl, OFFSET>,
            SetAllowRedirect: SetAllowRedirect::<Identity, Impl, OFFSET>,
            AllowInboundEchoRequest: AllowInboundEchoRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundEchoRequest: SetAllowInboundEchoRequest::<Identity, Impl, OFFSET>,
            AllowOutboundTimeExceeded: AllowOutboundTimeExceeded::<Identity, Impl, OFFSET>,
            SetAllowOutboundTimeExceeded: SetAllowOutboundTimeExceeded::<Identity, Impl, OFFSET>,
            AllowOutboundParameterProblem: AllowOutboundParameterProblem::<Identity, Impl, OFFSET>,
            SetAllowOutboundParameterProblem: SetAllowOutboundParameterProblem::<Identity, Impl, OFFSET>,
            AllowOutboundSourceQuench: AllowOutboundSourceQuench::<Identity, Impl, OFFSET>,
            SetAllowOutboundSourceQuench: SetAllowOutboundSourceQuench::<Identity, Impl, OFFSET>,
            AllowInboundRouterRequest: AllowInboundRouterRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundRouterRequest: SetAllowInboundRouterRequest::<Identity, Impl, OFFSET>,
            AllowInboundTimestampRequest: AllowInboundTimestampRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundTimestampRequest: SetAllowInboundTimestampRequest::<Identity, Impl, OFFSET>,
            AllowInboundMaskRequest: AllowInboundMaskRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundMaskRequest: SetAllowInboundMaskRequest::<Identity, Impl, OFFSET>,
            AllowOutboundPacketTooBig: AllowOutboundPacketTooBig::<Identity, Impl, OFFSET>,
            SetAllowOutboundPacketTooBig: SetAllowOutboundPacketTooBig::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwMgr_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn LocalPolicy(this: &Self::This) -> ::windows_core::Result<INetFwPolicy>;
    fn CurrentProfileType(this: &Self::This) -> ::windows_core::Result<NET_FW_PROFILE_TYPE>;
    fn RestoreDefaults(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsPortAllowed(this: &Self::This, imagefilename: &::windows_core::BSTR, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: &::windows_core::BSTR, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn IsIcmpTypeAllowed(this: &Self::This, ipversion: NET_FW_IP_VERSION, localaddress: &::windows_core::BSTR, r#type: u8, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LocalPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localpolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(localpolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentProfileType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentProfileType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profiletype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RestoreDefaults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreDefaults(this).into())
        }
        unsafe extern "system" fn IsPortAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPortAllowed(this, ::core::mem::transmute(&imagefilename), ::core::mem::transmute_copy(&ipversion), ::core::mem::transmute_copy(&portnumber), ::core::mem::transmute(&localaddress), ::core::mem::transmute_copy(&ipprotocol), ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)).into())
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#type: u8, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsIcmpTypeAllowed(this, ::core::mem::transmute_copy(&ipversion), ::core::mem::transmute(&localaddress), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)).into())
        }
        INetFwMgr_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LocalPolicy: LocalPolicy::<Identity, Impl, OFFSET>,
            CurrentProfileType: CurrentProfileType::<Identity, Impl, OFFSET>,
            RestoreDefaults: RestoreDefaults::<Identity, Impl, OFFSET>,
            IsPortAllowed: IsPortAllowed::<Identity, Impl, OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwOpenPort_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IpVersion(this: &Self::This) -> ::windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(this: &Self::This, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()>;
    fn Protocol(this: &Self::This) -> ::windows_core::Result<NET_FW_IP_PROTOCOL>;
    fn SetProtocol(this: &Self::This, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<()>;
    fn Port(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPort(this: &Self::This, portnumber: i32) -> ::windows_core::Result<()>;
    fn Scope(this: &Self::This) -> ::windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(this: &Self::This, scope: NET_FW_SCOPE) -> ::windows_core::Result<()>;
    fn RemoteAddresses(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemoteAddresses(this: &Self::This, remoteaddrs: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BuiltIn(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwOpenPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwOpenPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn IpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IpVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ipversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpVersion(this, ::core::mem::transmute_copy(&ipversion)).into())
        }
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ipprotocol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtocol(this, ::core::mem::transmute_copy(&ipprotocol)).into())
        }
        unsafe extern "system" fn Port<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Port(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(portnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPort(this, ::core::mem::transmute_copy(&portnumber)).into())
        }
        unsafe extern "system" fn Scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScope(this, ::core::mem::transmute_copy(&scope)).into())
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remoteaddrs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteAddresses(this, ::core::mem::transmute(&remoteaddrs)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn BuiltIn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, builtin: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BuiltIn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(builtin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwOpenPort_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            SetProtocol: SetProtocol::<Identity, Impl, OFFSET>,
            Port: Port::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            BuiltIn: BuiltIn::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwOpenPorts_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, port: ::core::option::Option<&INetFwOpenPort>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<()>;
    fn Item(this: &Self::This, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<INetFwOpenPort>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwOpenPorts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwOpenPorts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, port: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&port)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&portnumber), ::core::mem::transmute_copy(&ipprotocol)).into())
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&portnumber), ::core::mem::transmute_copy(&ipprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(openport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwOpenPorts_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwPolicy_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfile(this: &Self::This) -> ::windows_core::Result<INetFwProfile>;
    fn GetProfileByType(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE) -> ::windows_core::Result<INetFwProfile>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwPolicy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwPolicy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentProfile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProfileByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProfileByType(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwPolicy_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentProfile: CurrentProfile::<Identity, Impl, OFFSET>,
            GetProfileByType: GetProfileByType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwPolicy2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfileTypes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_FirewallEnabled(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_FirewallEnabled(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn get_ExcludedInterfaces(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn put_ExcludedInterfaces(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2, interfaces: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn get_BlockAllInboundTraffic(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_BlockAllInboundTraffic(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn get_NotificationsDisabled(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_NotificationsDisabled(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn get_UnicastResponsesToMulticastBroadcastDisabled(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_UnicastResponsesToMulticastBroadcastDisabled(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Rules(this: &Self::This) -> ::windows_core::Result<INetFwRules>;
    fn ServiceRestriction(this: &Self::This) -> ::windows_core::Result<INetFwServiceRestriction>;
    fn EnableRuleGroup(this: &Self::This, profiletypesbitmask: i32, group: &::windows_core::BSTR, enable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsRuleGroupEnabled(this: &Self::This, profiletypesbitmask: i32, group: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RestoreLocalFirewallDefaults(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_DefaultInboundAction(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<NET_FW_ACTION>;
    fn put_DefaultInboundAction(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::Result<()>;
    fn get_DefaultOutboundAction(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<NET_FW_ACTION>;
    fn put_DefaultOutboundAction(this: &Self::This, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::Result<()>;
    fn get_IsRuleGroupCurrentlyEnabled(this: &Self::This, group: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LocalPolicyModifyState(this: &Self::This) -> ::windows_core::Result<NET_FW_MODIFY_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwPolicy2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwPolicy2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentProfileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentProfileTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profiletypesbitmask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_FirewallEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_FirewallEnabled(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_FirewallEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_FirewallEnabled(this, ::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn get_ExcludedInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ExcludedInterfaces(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_ExcludedInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_ExcludedInterfaces(this, ::core::mem::transmute_copy(&profiletype), ::core::mem::transmute(&interfaces)).into())
        }
        unsafe extern "system" fn get_BlockAllInboundTraffic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_BlockAllInboundTraffic(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(block, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_BlockAllInboundTraffic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_BlockAllInboundTraffic(this, ::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&block)).into())
        }
        unsafe extern "system" fn get_NotificationsDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_NotificationsDisabled(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_NotificationsDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_NotificationsDisabled(this, ::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&disabled)).into())
        }
        unsafe extern "system" fn get_UnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_UnicastResponsesToMulticastBroadcastDisabled(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_UnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_UnicastResponsesToMulticastBroadcastDisabled(this, ::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&disabled)).into())
        }
        unsafe extern "system" fn Rules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceRestriction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicerestriction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceRestriction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(servicerestriction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableRuleGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::std::mem::MaybeUninit<::windows_core::BSTR>, enable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableRuleGroup(this, ::core::mem::transmute_copy(&profiletypesbitmask), ::core::mem::transmute(&group), ::core::mem::transmute_copy(&enable)).into())
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::std::mem::MaybeUninit<::windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRuleGroupEnabled(this, ::core::mem::transmute_copy(&profiletypesbitmask), ::core::mem::transmute(&group)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreLocalFirewallDefaults(this).into())
        }
        unsafe extern "system" fn get_DefaultInboundAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_DefaultInboundAction(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_DefaultInboundAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_DefaultInboundAction(this, ::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn get_DefaultOutboundAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_DefaultOutboundAction(this, ::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_DefaultOutboundAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_DefaultOutboundAction(this, ::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn get_IsRuleGroupCurrentlyEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, group: ::std::mem::MaybeUninit<::windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_IsRuleGroupCurrentlyEnabled(this, ::core::mem::transmute(&group)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalPolicyModifyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalPolicyModifyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modifystate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwPolicy2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentProfileTypes: CurrentProfileTypes::<Identity, Impl, OFFSET>,
            get_FirewallEnabled: get_FirewallEnabled::<Identity, Impl, OFFSET>,
            put_FirewallEnabled: put_FirewallEnabled::<Identity, Impl, OFFSET>,
            get_ExcludedInterfaces: get_ExcludedInterfaces::<Identity, Impl, OFFSET>,
            put_ExcludedInterfaces: put_ExcludedInterfaces::<Identity, Impl, OFFSET>,
            get_BlockAllInboundTraffic: get_BlockAllInboundTraffic::<Identity, Impl, OFFSET>,
            put_BlockAllInboundTraffic: put_BlockAllInboundTraffic::<Identity, Impl, OFFSET>,
            get_NotificationsDisabled: get_NotificationsDisabled::<Identity, Impl, OFFSET>,
            put_NotificationsDisabled: put_NotificationsDisabled::<Identity, Impl, OFFSET>,
            get_UnicastResponsesToMulticastBroadcastDisabled: get_UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            put_UnicastResponsesToMulticastBroadcastDisabled: put_UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
            ServiceRestriction: ServiceRestriction::<Identity, Impl, OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Identity, Impl, OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Identity, Impl, OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Identity, Impl, OFFSET>,
            get_DefaultInboundAction: get_DefaultInboundAction::<Identity, Impl, OFFSET>,
            put_DefaultInboundAction: put_DefaultInboundAction::<Identity, Impl, OFFSET>,
            get_DefaultOutboundAction: get_DefaultOutboundAction::<Identity, Impl, OFFSET>,
            put_DefaultOutboundAction: put_DefaultOutboundAction::<Identity, Impl, OFFSET>,
            get_IsRuleGroupCurrentlyEnabled: get_IsRuleGroupCurrentlyEnabled::<Identity, Impl, OFFSET>,
            LocalPolicyModifyState: LocalPolicyModifyState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwProduct_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RuleCategories(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetRuleCategories(this: &Self::This, rulecategories: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplayName(this: &Self::This, displayname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PathToSignedProductExe(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwProduct {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwProduct {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RuleCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rulecategories: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RuleCategories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rulecategories, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRuleCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rulecategories: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRuleCategories(this, ::core::mem::transmute(&rulecategories)).into())
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&displayname)).into())
        }
        unsafe extern "system" fn PathToSignedProductExe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathToSignedProductExe(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwProduct_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RuleCategories: RuleCategories::<Identity, Impl, OFFSET>,
            SetRuleCategories: SetRuleCategories::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            PathToSignedProductExe: PathToSignedProductExe::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwProducts_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Register(this: &Self::This, product: ::core::option::Option<&INetFwProduct>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<INetFwProduct>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwProducts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwProducts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Register<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, product: *mut ::core::ffi::c_void, registration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Register(this, ::windows_core::from_raw_borrowed(&product)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, product: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(product, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwProducts_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwProfile_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<NET_FW_PROFILE_TYPE>;
    fn FirewallEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFirewallEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ExceptionsNotAllowed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetExceptionsNotAllowed(this: &Self::This, notallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn NotificationsDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNotificationsDisabled(this: &Self::This, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(this: &Self::This, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RemoteAdminSettings(this: &Self::This) -> ::windows_core::Result<INetFwRemoteAdminSettings>;
    fn IcmpSettings(this: &Self::This) -> ::windows_core::Result<INetFwIcmpSettings>;
    fn GloballyOpenPorts(this: &Self::This) -> ::windows_core::Result<INetFwOpenPorts>;
    fn Services(this: &Self::This) -> ::windows_core::Result<INetFwServices>;
    fn AuthorizedApplications(this: &Self::This) -> ::windows_core::Result<INetFwAuthorizedApplications>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwProfile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwProfile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirewallEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirewallEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFirewallEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExceptionsNotAllowed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExceptionsNotAllowed(this, ::core::mem::transmute_copy(&notallowed)).into())
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NotificationsDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotificationsDisabled(this, ::core::mem::transmute_copy(&disabled)).into())
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnicastResponsesToMulticastBroadcastDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnicastResponsesToMulticastBroadcastDisabled(this, ::core::mem::transmute_copy(&disabled)).into())
        }
        unsafe extern "system" fn RemoteAdminSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteAdminSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remoteadminsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IcmpSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icmpsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IcmpSettings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icmpsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, openports: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GloballyOpenPorts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(openports, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Services<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, services: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Services(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(services, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AuthorizedApplications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, apps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthorizedApplications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(apps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwProfile_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, Impl, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, Impl, OFFSET>,
            ExceptionsNotAllowed: ExceptionsNotAllowed::<Identity, Impl, OFFSET>,
            SetExceptionsNotAllowed: SetExceptionsNotAllowed::<Identity, Impl, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, Impl, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, Impl, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            RemoteAdminSettings: RemoteAdminSettings::<Identity, Impl, OFFSET>,
            IcmpSettings: IcmpSettings::<Identity, Impl, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, Impl, OFFSET>,
            Services: Services::<Identity, Impl, OFFSET>,
            AuthorizedApplications: AuthorizedApplications::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRemoteAdminSettings_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn IpVersion(this: &Self::This) -> ::windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(this: &Self::This, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()>;
    fn Scope(this: &Self::This) -> ::windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(this: &Self::This, scope: NET_FW_SCOPE) -> ::windows_core::Result<()>;
    fn RemoteAddresses(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemoteAddresses(this: &Self::This, remoteaddrs: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwRemoteAdminSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwRemoteAdminSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IpVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ipversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpVersion(this, ::core::mem::transmute_copy(&ipversion)).into())
        }
        unsafe extern "system" fn Scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScope(this, ::core::mem::transmute_copy(&scope)).into())
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remoteaddrs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteAddresses(this, ::core::mem::transmute(&remoteaddrs)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        INetFwRemoteAdminSettings_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRule_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, desc: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationName(this: &Self::This, imagefilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceName(this: &Self::This, servicename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Protocol(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetProtocol(this: &Self::This, protocol: i32) -> ::windows_core::Result<()>;
    fn LocalPorts(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalPorts(this: &Self::This, portnumbers: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemotePorts(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemotePorts(this: &Self::This, portnumbers: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalAddresses(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalAddresses(this: &Self::This, localaddrs: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoteAddresses(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemoteAddresses(this: &Self::This, remoteaddrs: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IcmpTypesAndCodes(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetIcmpTypesAndCodes(this: &Self::This, icmptypesandcodes: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Direction(this: &Self::This) -> ::windows_core::Result<NET_FW_RULE_DIRECTION>;
    fn SetDirection(this: &Self::This, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::Result<()>;
    fn Interfaces(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetInterfaces(this: &Self::This, interfaces: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn InterfaceTypes(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetInterfaceTypes(this: &Self::This, interfacetypes: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Grouping(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetGrouping(this: &Self::This, context: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Profiles(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetProfiles(this: &Self::This, profiletypesbitmask: i32) -> ::windows_core::Result<()>;
    fn EdgeTraversal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEdgeTraversal(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Action(this: &Self::This) -> ::windows_core::Result<NET_FW_ACTION>;
    fn SetAction(this: &Self::This, action: NET_FW_ACTION) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(desc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desc: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&desc)).into())
        }
        unsafe extern "system" fn ApplicationName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagefilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetApplicationName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationName(this, ::core::mem::transmute(&imagefilename)).into())
        }
        unsafe extern "system" fn ServiceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(servicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceName(this, ::core::mem::transmute(&servicename)).into())
        }
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(protocol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtocol(this, ::core::mem::transmute_copy(&protocol)).into())
        }
        unsafe extern "system" fn LocalPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumbers: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalPorts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(portnumbers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumbers: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalPorts(this, ::core::mem::transmute(&portnumbers)).into())
        }
        unsafe extern "system" fn RemotePorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumbers: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemotePorts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(portnumbers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemotePorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, portnumbers: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemotePorts(this, ::core::mem::transmute(&portnumbers)).into())
        }
        unsafe extern "system" fn LocalAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(localaddrs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalAddresses(this, ::core::mem::transmute(&localaddrs)).into())
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remoteaddrs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteAddresses(this, ::core::mem::transmute(&remoteaddrs)).into())
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IcmpTypesAndCodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icmptypesandcodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIcmpTypesAndCodes(this, ::core::mem::transmute(&icmptypesandcodes)).into())
        }
        unsafe extern "system" fn Direction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Direction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDirection(this, ::core::mem::transmute_copy(&dir)).into())
        }
        unsafe extern "system" fn Interfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Interfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterfaces(this, ::core::mem::transmute(&interfaces)).into())
        }
        unsafe extern "system" fn InterfaceTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfacetypes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterfaceTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfacetypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInterfaceTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfacetypes: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterfaceTypes(this, ::core::mem::transmute(&interfacetypes)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn Grouping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Grouping(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(context, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGrouping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGrouping(this, ::core::mem::transmute(&context)).into())
        }
        unsafe extern "system" fn Profiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profiletypesbitmask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProfiles(this, ::core::mem::transmute_copy(&profiletypesbitmask)).into())
        }
        unsafe extern "system" fn EdgeTraversal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EdgeTraversal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEdgeTraversal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEdgeTraversal(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn Action<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Action(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAction(this, ::core::mem::transmute_copy(&action)).into())
        }
        INetFwRule_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationName: ApplicationName::<Identity, Impl, OFFSET>,
            SetApplicationName: SetApplicationName::<Identity, Impl, OFFSET>,
            ServiceName: ServiceName::<Identity, Impl, OFFSET>,
            SetServiceName: SetServiceName::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            SetProtocol: SetProtocol::<Identity, Impl, OFFSET>,
            LocalPorts: LocalPorts::<Identity, Impl, OFFSET>,
            SetLocalPorts: SetLocalPorts::<Identity, Impl, OFFSET>,
            RemotePorts: RemotePorts::<Identity, Impl, OFFSET>,
            SetRemotePorts: SetRemotePorts::<Identity, Impl, OFFSET>,
            LocalAddresses: LocalAddresses::<Identity, Impl, OFFSET>,
            SetLocalAddresses: SetLocalAddresses::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            IcmpTypesAndCodes: IcmpTypesAndCodes::<Identity, Impl, OFFSET>,
            SetIcmpTypesAndCodes: SetIcmpTypesAndCodes::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            SetDirection: SetDirection::<Identity, Impl, OFFSET>,
            Interfaces: Interfaces::<Identity, Impl, OFFSET>,
            SetInterfaces: SetInterfaces::<Identity, Impl, OFFSET>,
            InterfaceTypes: InterfaceTypes::<Identity, Impl, OFFSET>,
            SetInterfaceTypes: SetInterfaceTypes::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Grouping: Grouping::<Identity, Impl, OFFSET>,
            SetGrouping: SetGrouping::<Identity, Impl, OFFSET>,
            Profiles: Profiles::<Identity, Impl, OFFSET>,
            SetProfiles: SetProfiles::<Identity, Impl, OFFSET>,
            EdgeTraversal: EdgeTraversal::<Identity, Impl, OFFSET>,
            SetEdgeTraversal: SetEdgeTraversal::<Identity, Impl, OFFSET>,
            Action: Action::<Identity, Impl, OFFSET>,
            SetAction: SetAction::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRule2_Impl: ::windows_core::BaseImpl + INetFwRule_Impl {
    fn EdgeTraversalOptions(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEdgeTraversalOptions(this: &Self::This, loptions: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwRule2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INetFwRule);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwRule2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EdgeTraversalOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EdgeTraversalOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(loptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEdgeTraversalOptions(this, ::core::mem::transmute_copy(&loptions)).into())
        }
        INetFwRule2_Vtbl {
            base__: <INetFwRule as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EdgeTraversalOptions: EdgeTraversalOptions::<Identity, Impl, OFFSET>,
            SetEdgeTraversalOptions: SetEdgeTraversalOptions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRule3_Impl: ::windows_core::BaseImpl + INetFwRule2_Impl {
    fn LocalAppPackageId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalAppPackageId(this: &Self::This, wszpackageid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalUserOwner(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalUserOwner(this: &Self::This, wszuserowner: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalUserAuthorizedList(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalUserAuthorizedList(this: &Self::This, wszuserauthlist: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoteUserAuthorizedList(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemoteUserAuthorizedList(this: &Self::This, wszuserauthlist: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoteMachineAuthorizedList(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemoteMachineAuthorizedList(this: &Self::This, wszuserauthlist: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SecureFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSecureFlags(this: &Self::This, loptions: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwRule3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(INetFwRule2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwRule3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LocalAppPackageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpackageid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalAppPackageId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wszpackageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalAppPackageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalAppPackageId(this, ::core::mem::transmute(&wszpackageid)).into())
        }
        unsafe extern "system" fn LocalUserOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserowner: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalUserOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wszuserowner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalUserOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalUserOwner(this, ::core::mem::transmute(&wszuserowner)).into())
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalUserAuthorizedList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wszuserauthlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalUserAuthorizedList(this, ::core::mem::transmute(&wszuserauthlist)).into())
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteUserAuthorizedList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wszuserauthlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteUserAuthorizedList(this, ::core::mem::transmute(&wszuserauthlist)).into())
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteMachineAuthorizedList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wszuserauthlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteMachineAuthorizedList(this, ::core::mem::transmute(&wszuserauthlist)).into())
        }
        unsafe extern "system" fn SecureFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecureFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(loptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecureFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecureFlags(this, ::core::mem::transmute_copy(&loptions)).into())
        }
        INetFwRule3_Vtbl {
            base__: <INetFwRule2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LocalAppPackageId: LocalAppPackageId::<Identity, Impl, OFFSET>,
            SetLocalAppPackageId: SetLocalAppPackageId::<Identity, Impl, OFFSET>,
            LocalUserOwner: LocalUserOwner::<Identity, Impl, OFFSET>,
            SetLocalUserOwner: SetLocalUserOwner::<Identity, Impl, OFFSET>,
            LocalUserAuthorizedList: LocalUserAuthorizedList::<Identity, Impl, OFFSET>,
            SetLocalUserAuthorizedList: SetLocalUserAuthorizedList::<Identity, Impl, OFFSET>,
            RemoteUserAuthorizedList: RemoteUserAuthorizedList::<Identity, Impl, OFFSET>,
            SetRemoteUserAuthorizedList: SetRemoteUserAuthorizedList::<Identity, Impl, OFFSET>,
            RemoteMachineAuthorizedList: RemoteMachineAuthorizedList::<Identity, Impl, OFFSET>,
            SetRemoteMachineAuthorizedList: SetRemoteMachineAuthorizedList::<Identity, Impl, OFFSET>,
            SecureFlags: SecureFlags::<Identity, Impl, OFFSET>,
            SetSecureFlags: SetSecureFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwRules_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, rule: ::core::option::Option<&INetFwRule>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Item(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<INetFwRule>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwRules {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwRules {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&rule)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwRules_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwService_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Type(this: &Self::This) -> ::windows_core::Result<NET_FW_SERVICE_TYPE>;
    fn Customized(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IpVersion(this: &Self::This) -> ::windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(this: &Self::This, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()>;
    fn Scope(this: &Self::This) -> ::windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(this: &Self::This, scope: NET_FW_SCOPE) -> ::windows_core::Result<()>;
    fn RemoteAddresses(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRemoteAddresses(this: &Self::This, remoteaddrs: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(this: &Self::This, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GloballyOpenPorts(this: &Self::This) -> ::windows_core::Result<INetFwOpenPorts>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Customized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customized: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Customized(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customized, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IpVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ipversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIpVersion(this, ::core::mem::transmute_copy(&ipversion)).into())
        }
        unsafe extern "system" fn Scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScope(this, ::core::mem::transmute_copy(&scope)).into())
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remoteaddrs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteAddresses(this, ::core::mem::transmute(&remoteaddrs)).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&enabled)).into())
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, openports: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GloballyOpenPorts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(openports, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwService_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Customized: Customized::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwServiceRestriction_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RestrictService(this: &Self::This, servicename: &::windows_core::BSTR, appname: &::windows_core::BSTR, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ServiceRestricted(this: &Self::This, servicename: &::windows_core::BSTR, appname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Rules(this: &Self::This) -> ::windows_core::Result<INetFwRules>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwServiceRestriction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwServiceRestriction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RestrictService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, appname: ::std::mem::MaybeUninit<::windows_core::BSTR>, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestrictService(this, ::core::mem::transmute(&servicename), ::core::mem::transmute(&appname), ::core::mem::transmute_copy(&restrictservice), ::core::mem::transmute_copy(&servicesidrestricted)).into())
        }
        unsafe extern "system" fn ServiceRestricted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, appname: ::std::mem::MaybeUninit<::windows_core::BSTR>, servicerestricted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceRestricted(this, ::core::mem::transmute(&servicename), ::core::mem::transmute(&appname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(servicerestricted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Rules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwServiceRestriction_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RestrictService: RestrictService::<Identity, Impl, OFFSET>,
            ServiceRestricted: ServiceRestricted::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetFwServices_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, svctype: NET_FW_SERVICE_TYPE) -> ::windows_core::Result<INetFwService>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetFwServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetFwServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&svctype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(service, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetFwServices_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingConfiguration_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SharingEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SharingConnectionType(this: &Self::This) -> ::windows_core::Result<SHARINGCONNECTIONTYPE>;
    fn DisableSharing(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnableSharing(this: &Self::This, r#type: SHARINGCONNECTIONTYPE) -> ::windows_core::Result<()>;
    fn InternetFirewallEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisableInternetFirewall(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnableInternetFirewall(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_EnumPortMappings(this: &Self::This, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPortMappingCollection>;
    fn AddPortMapping(this: &Self::This, bstrname: &::windows_core::BSTR, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: &::windows_core::BSTR, etargettype: ICS_TARGETTYPE) -> ::windows_core::Result<INetSharingPortMapping>;
    fn RemovePortMapping(this: &Self::This, pmapping: ::core::option::Option<&INetSharingPortMapping>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SharingEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SharingEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SharingConnectionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SharingConnectionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisableSharing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableSharing(this).into())
        }
        unsafe extern "system" fn EnableSharing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableSharing(this, ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn InternetFirewallEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InternetFirewallEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisableInternetFirewall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableInternetFirewall(this).into())
        }
        unsafe extern "system" fn EnableInternetFirewall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableInternetFirewall(this).into())
        }
        unsafe extern "system" fn get_EnumPortMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_EnumPortMappings(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcoll, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddPortMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddPortMapping(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&ucipprotocol), ::core::mem::transmute_copy(&usexternalport), ::core::mem::transmute_copy(&usinternalport), ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&bstrtargetnameoripaddress), ::core::mem::transmute_copy(&etargettype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmapping, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePortMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmapping: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePortMapping(this, ::windows_core::from_raw_borrowed(&pmapping)).into())
        }
        INetSharingConfiguration_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SharingEnabled: SharingEnabled::<Identity, Impl, OFFSET>,
            SharingConnectionType: SharingConnectionType::<Identity, Impl, OFFSET>,
            DisableSharing: DisableSharing::<Identity, Impl, OFFSET>,
            EnableSharing: EnableSharing::<Identity, Impl, OFFSET>,
            InternetFirewallEnabled: InternetFirewallEnabled::<Identity, Impl, OFFSET>,
            DisableInternetFirewall: DisableInternetFirewall::<Identity, Impl, OFFSET>,
            EnableInternetFirewall: EnableInternetFirewall::<Identity, Impl, OFFSET>,
            get_EnumPortMappings: get_EnumPortMappings::<Identity, Impl, OFFSET>,
            AddPortMapping: AddPortMapping::<Identity, Impl, OFFSET>,
            RemovePortMapping: RemovePortMapping::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingEveryConnectionCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingEveryConnectionCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingEveryConnectionCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetSharingEveryConnectionCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SharingInstalled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn get_EnumPublicConnections(this: &Self::This, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPublicConnectionCollection>;
    fn get_EnumPrivateConnections(this: &Self::This, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPrivateConnectionCollection>;
    fn get_INetSharingConfigurationForINetConnection(this: &Self::This, pnetconnection: ::core::option::Option<&INetConnection>) -> ::windows_core::Result<INetSharingConfiguration>;
    fn EnumEveryConnection(this: &Self::This) -> ::windows_core::Result<INetSharingEveryConnectionCollection>;
    fn get_NetConnectionProps(this: &Self::This, pnetconnection: ::core::option::Option<&INetConnection>) -> ::windows_core::Result<INetConnectionProps>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SharingInstalled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbinstalled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SharingInstalled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbinstalled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_EnumPublicConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_EnumPublicConnections(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcoll, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_EnumPrivateConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_EnumPrivateConnections(this, ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcoll, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_INetSharingConfigurationForINetConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetconnection: *mut ::core::ffi::c_void, ppnetsharingconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_INetSharingConfigurationForINetConnection(this, ::windows_core::from_raw_borrowed(&pnetconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetsharingconfiguration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumEveryConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumEveryConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcoll, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_NetConnectionProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetconnection: *mut ::core::ffi::c_void, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_NetConnectionProps(this, ::windows_core::from_raw_borrowed(&pnetconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetSharingManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SharingInstalled: SharingInstalled::<Identity, Impl, OFFSET>,
            get_EnumPublicConnections: get_EnumPublicConnections::<Identity, Impl, OFFSET>,
            get_EnumPrivateConnections: get_EnumPrivateConnections::<Identity, Impl, OFFSET>,
            get_INetSharingConfigurationForINetConnection: get_INetSharingConfigurationForINetConnection::<Identity, Impl, OFFSET>,
            EnumEveryConnection: EnumEveryConnection::<Identity, Impl, OFFSET>,
            get_NetConnectionProps: get_NetConnectionProps::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPortMapping_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Disable(this: &Self::This) -> ::windows_core::Result<()>;
    fn Enable(this: &Self::This) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<INetSharingPortMappingProps>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingPortMapping {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingPortMapping {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Disable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disable(this).into())
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnspmp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnspmp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        INetSharingPortMapping_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Disable: Disable::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPortMappingCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingPortMappingCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingPortMappingCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetSharingPortMappingCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPortMappingProps_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IPProtocol(this: &Self::This) -> ::windows_core::Result<u8>;
    fn ExternalPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn InternalPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Options(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TargetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TargetIPAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingPortMappingProps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingPortMappingProps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IPProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IPProtocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucipprot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExternalPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pusport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InternalPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pusport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Options<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Options(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TargetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtargetname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TargetIPAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TargetIPAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtargetipaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetSharingPortMappingProps_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            IPProtocol: IPProtocol::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            TargetName: TargetName::<Identity, Impl, OFFSET>,
            TargetIPAddress: TargetIPAddress::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPrivateConnectionCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingPrivateConnectionCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingPrivateConnectionCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetSharingPrivateConnectionCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetSharingPublicConnectionCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INetSharingPublicConnectionCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetSharingPublicConnectionCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INetSharingPublicConnectionCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IStaticPortMapping_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExternalPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn InternalPort(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Protocol(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InternalClient(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EditInternalClient(this: &Self::This, bstrinternalclient: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Enable(this: &Self::This, vb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EditDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EditInternalPort(this: &Self::This, linternalport: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IStaticPortMapping {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStaticPortMapping {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExternalIPAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExternalIPAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExternalPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InternalPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InternalClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InternalClient(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EditInternalClient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditInternalClient(this, ::core::mem::transmute(&bstrinternalclient)).into())
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this, ::core::mem::transmute_copy(&vb)).into())
        }
        unsafe extern "system" fn EditDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn EditInternalPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditInternalPort(this, ::core::mem::transmute_copy(&linternalport)).into())
        }
        IStaticPortMapping_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExternalIPAddress: ExternalIPAddress::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            InternalClient: InternalClient::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            EditDescription: EditDescription::<Identity, Impl, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IStaticPortMappingCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, lexternalport: i32, bstrprotocol: &::windows_core::BSTR) -> ::windows_core::Result<IStaticPortMapping>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Remove(this: &Self::This, lexternalport: i32, bstrprotocol: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, lexternalport: i32, bstrprotocol: &::windows_core::BSTR, linternalport: i32, bstrinternalclient: &::windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<IStaticPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IStaticPortMappingCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStaticPortMappingCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppspm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternalport: i32, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppspm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&linternalport), ::core::mem::transmute(&bstrinternalclient), ::core::mem::transmute_copy(&benabled), ::core::mem::transmute(&bstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStaticPortMappingCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUPnPNAT_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StaticPortMappingCollection(this: &Self::This) -> ::windows_core::Result<IStaticPortMappingCollection>;
    fn DynamicPortMappingCollection(this: &Self::This) -> ::windows_core::Result<IDynamicPortMappingCollection>;
    fn NATEventManager(this: &Self::This) -> ::windows_core::Result<INATEventManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUPnPNAT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUPnPNAT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StaticPortMappingCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppspms: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StaticPortMappingCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspms, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdpms: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DynamicPortMappingCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdpms, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NATEventManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NATEventManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUPnPNAT_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StaticPortMappingCollection: StaticPortMappingCollection::<Identity, Impl, OFFSET>,
            DynamicPortMappingCollection: DynamicPortMappingCollection::<Identity, Impl, OFFSET>,
            NATEventManager: NATEventManager::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
