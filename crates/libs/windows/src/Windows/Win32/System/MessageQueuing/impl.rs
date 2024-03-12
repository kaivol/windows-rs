#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQApplication_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn MachineIdOfMachineName(this: &Self::This, machinename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MachineIdOfMachineName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, machinename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MachineIdOfMachineName(this, ::core::mem::transmute(&machinename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQApplication_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MachineIdOfMachineName: MachineIdOfMachineName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQApplication2_Impl: ::windows_core::BaseImpl + IMSMQApplication_Impl {
    fn RegisterCertificate(this: &Self::This, flags: *const super::Variant::VARIANT, externalcertificate: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MachineNameOfMachineId(this: &Self::This, bstrguid: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MSMQVersionMajor(this: &Self::This) -> ::windows_core::Result<i16>;
    fn MSMQVersionMinor(this: &Self::This) -> ::windows_core::Result<i16>;
    fn MSMQVersionBuild(this: &Self::This) -> ::windows_core::Result<i16>;
    fn IsDsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQApplication2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQApplication);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQApplication2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *const super::Variant::VARIANT, externalcertificate: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterCertificate(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&externalcertificate)).into())
        }
        unsafe extern "system" fn MachineNameOfMachineId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrmachinename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MachineNameOfMachineId(this, ::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachinename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MSMQVersionMajor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MSMQVersionMajor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psmsmqversionmajor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MSMQVersionMinor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MSMQVersionMinor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psmsmqversionminor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MSMQVersionBuild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MSMQVersionBuild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psmsmqversionbuild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisdsenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQApplication2_Vtbl {
            base__: <IMSMQApplication as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterCertificate: RegisterCertificate::<Identity, Impl, OFFSET>,
            MachineNameOfMachineId: MachineNameOfMachineId::<Identity, Impl, OFFSET>,
            MSMQVersionMajor: MSMQVersionMajor::<Identity, Impl, OFFSET>,
            MSMQVersionMinor: MSMQVersionMinor::<Identity, Impl, OFFSET>,
            MSMQVersionBuild: MSMQVersionBuild::<Identity, Impl, OFFSET>,
            IsDsEnabled: IsDsEnabled::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQApplication3_Impl: ::windows_core::BaseImpl + IMSMQApplication2_Impl {
    fn ActiveQueues(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn PrivateQueues(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DirectoryServiceServer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BytesInAllQueues(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetMachine(this: &Self::This, bstrmachine: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Machine(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Connect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Tidy(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQApplication3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQApplication2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQApplication3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActiveQueues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActiveQueues(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvactivequeues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateQueues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateQueues(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvprivatequeues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DirectoryServiceServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DirectoryServiceServer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdirectoryserviceserver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BytesInAllQueues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BytesInAllQueues(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbytesinallqueues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMachine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmachine: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMachine(this, ::core::mem::transmute(&bstrmachine)).into())
        }
        unsafe extern "system" fn Machine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Machine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn Tidy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Tidy(this).into())
        }
        IMSMQApplication3_Vtbl {
            base__: <IMSMQApplication2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActiveQueues: ActiveQueues::<Identity, Impl, OFFSET>,
            PrivateQueues: PrivateQueues::<Identity, Impl, OFFSET>,
            DirectoryServiceServer: DirectoryServiceServer::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            BytesInAllQueues: BytesInAllQueues::<Identity, Impl, OFFSET>,
            SetMachine: SetMachine::<Identity, Impl, OFFSET>,
            Machine: Machine::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Tidy: Tidy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Item(this: &Self::This, index: *const super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: *const super::Variant::VARIANT, pvarret: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarret, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCoordinatedTransactionDispenser_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BeginTransaction(this: &Self::This) -> ::windows_core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQCoordinatedTransactionDispenser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQCoordinatedTransactionDispenser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQCoordinatedTransactionDispenser_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCoordinatedTransactionDispenser2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BeginTransaction(this: &Self::This) -> ::windows_core::Result<IMSMQTransaction2>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQCoordinatedTransactionDispenser2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQCoordinatedTransactionDispenser2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQCoordinatedTransactionDispenser2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQCoordinatedTransactionDispenser3_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BeginTransaction(this: &Self::This) -> ::windows_core::Result<IMSMQTransaction3>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQCoordinatedTransactionDispenser3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQCoordinatedTransactionDispenser3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQCoordinatedTransactionDispenser3_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQDestination_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Open(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsOpen(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IADs(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn putref_IADs(this: &Self::This, piads: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn ADsPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetADsPath(this: &Self::This, bstradspath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PathName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPathName(this: &Self::This, bstrpathname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FormatName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFormatName(this: &Self::This, bstrformatname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Destinations(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn putref_Destinations(this: &Self::This, pdestinations: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQDestination {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQDestination {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisopen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisopen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IADs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiads: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IADs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_IADs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piads: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_IADs(this, ::windows_core::from_raw_borrowed(&piads)).into())
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADsPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetADsPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetADsPath(this, ::core::mem::transmute(&bstradspath)).into())
        }
        unsafe extern "system" fn PathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPathName(this, ::core::mem::transmute(&bstrpathname)).into())
        }
        unsafe extern "system" fn FormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormatName(this, ::core::mem::transmute(&bstrformatname)).into())
        }
        unsafe extern "system" fn Destinations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdestinations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Destinations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestinations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_Destinations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinations: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_Destinations(this, ::windows_core::from_raw_borrowed(&pdestinations)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQDestination_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            IADs: IADs::<Identity, Impl, OFFSET>,
            putref_IADs: putref_IADs::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
            SetADsPath: SetADsPath::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            Destinations: Destinations::<Identity, Impl, OFFSET>,
            putref_Destinations: putref_Destinations::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQEvent {
    const VTABLE: Self::Vtable = { IMSMQEvent_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQEvent2_Impl: ::windows_core::BaseImpl + IMSMQEvent_Impl {
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQEvent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQEvent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQEvent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQEvent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQEvent2_Vtbl { base__: <IMSMQEvent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Properties: Properties::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQEvent3_Impl: ::windows_core::BaseImpl + IMSMQEvent2_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQEvent3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQEvent2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQEvent3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQEvent3 {
    const VTABLE: Self::Vtable = { IMSMQEvent3_Vtbl { base__: <IMSMQEvent2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQManagement_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Init(this: &Self::This, machine: *const super::Variant::VARIANT, pathname: *const super::Variant::VARIANT, formatname: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn FormatName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Machine(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MessageCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ForeignStatus(this: &Self::This) -> ::windows_core::Result<i32>;
    fn QueueType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IsLocal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TransactionalStatus(this: &Self::This) -> ::windows_core::Result<i32>;
    fn BytesInQueue(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQManagement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQManagement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, machine: *const super::Variant::VARIANT, pathname: *const super::Variant::VARIANT, formatname: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&machine), ::core::mem::transmute_copy(&pathname), ::core::mem::transmute_copy(&formatname)).into())
        }
        unsafe extern "system" fn FormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Machine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Machine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MessageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmessagecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ForeignStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ForeignStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plforeignstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plqueuetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfislocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfislocal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransactionalStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionalStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltransactionalstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BytesInQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BytesInQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbytesinqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQManagement_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            Machine: Machine::<Identity, Impl, OFFSET>,
            MessageCount: MessageCount::<Identity, Impl, OFFSET>,
            ForeignStatus: ForeignStatus::<Identity, Impl, OFFSET>,
            QueueType: QueueType::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            TransactionalStatus: TransactionalStatus::<Identity, Impl, OFFSET>,
            BytesInQueue: BytesInQueue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Class(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn AuthLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthLevel(this: &Self::This, lauthlevel: i32) -> ::windows_core::Result<()>;
    fn IsAuthenticated(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Delivery(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDelivery(this: &Self::This, ldelivery: i32) -> ::windows_core::Result<()>;
    fn Trace(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetTrace(this: &Self::This, ltrace: i32) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPriority(this: &Self::This, lpriority: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn ResponseQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn AppSpecific(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAppSpecific(this: &Self::This, lappspecific: i32) -> ::windows_core::Result<()>;
    fn SourceMachineGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BodyLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Body(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(this: &Self::This, varbody: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AdminQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(this: &Self::This, varmsgid: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Ack(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAck(this: &Self::This, lack: i32) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxTimeToReachQueue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(this: &Self::This, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()>;
    fn MaxTimeToReceive(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReceive(this: &Self::This, lmaxtimetoreceive: i32) -> ::windows_core::Result<()>;
    fn HashAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHashAlgorithm(this: &Self::This, lhashalg: i32) -> ::windows_core::Result<()>;
    fn EncryptAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEncryptAlgorithm(this: &Self::This, lencryptalg: i32) -> ::windows_core::Result<()>;
    fn SentTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn SenderCertificate(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(this: &Self::This, varsendercert: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SenderId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSenderIdType(this: &Self::This, lsenderidtype: i32) -> ::windows_core::Result<()>;
    fn Send(this: &Self::This, destinationqueue: ::core::option::Option<&IMSMQQueue>, transaction: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AttachCurrentSecurityContext(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQMessage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQMessage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Class<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Class(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthLevel(this, ::core::mem::transmute_copy(&lauthlevel)).into())
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAuthenticated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Delivery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelivery(this, ::core::mem::transmute_copy(&ldelivery)).into())
        }
        unsafe extern "system" fn Trace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Trace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrace(this, ::core::mem::transmute_copy(&ltrace)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&lpriority)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppSpecific(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppSpecific(this, ::core::mem::transmute_copy(&lappspecific)).into())
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourceMachineGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BodyLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Body<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Body(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varbody: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&varbody)).into())
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorrelationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCorrelationId(this, ::core::mem::transmute(&varmsgid)).into())
        }
        unsafe extern "system" fn Ack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Ack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAck(this, ::core::mem::transmute_copy(&lack)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReachQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReachQueue(this, ::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into())
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReceive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReceive(this, ::core::mem::transmute_copy(&lmaxtimetoreceive)).into())
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HashAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHashAlgorithm(this, ::core::mem::transmute_copy(&lhashalg)).into())
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EncryptAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncryptAlgorithm(this, ::core::mem::transmute_copy(&lencryptalg)).into())
        }
        unsafe extern "system" fn SentTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SentTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArrivedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderCertificate(this, ::core::mem::transmute(&varsendercert)).into())
        }
        unsafe extern "system" fn SenderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderIdType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderIdType(this, ::core::mem::transmute_copy(&lsenderidtype)).into())
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::windows_core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into())
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachCurrentSecurityContext(this).into())
        }
        IMSMQMessage_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Class(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn AuthLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthLevel(this: &Self::This, lauthlevel: i32) -> ::windows_core::Result<()>;
    fn IsAuthenticated(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Delivery(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDelivery(this: &Self::This, ldelivery: i32) -> ::windows_core::Result<()>;
    fn Trace(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetTrace(this: &Self::This, ltrace: i32) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPriority(this: &Self::This, lpriority: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn ResponseQueueInfo_v1(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn AppSpecific(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAppSpecific(this: &Self::This, lappspecific: i32) -> ::windows_core::Result<()>;
    fn SourceMachineGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BodyLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Body(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(this: &Self::This, varbody: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AdminQueueInfo_v1(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(this: &Self::This, varmsgid: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Ack(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAck(this: &Self::This, lack: i32) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxTimeToReachQueue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(this: &Self::This, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()>;
    fn MaxTimeToReceive(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReceive(this: &Self::This, lmaxtimetoreceive: i32) -> ::windows_core::Result<()>;
    fn HashAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHashAlgorithm(this: &Self::This, lhashalg: i32) -> ::windows_core::Result<()>;
    fn EncryptAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEncryptAlgorithm(this: &Self::This, lencryptalg: i32) -> ::windows_core::Result<()>;
    fn SentTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn SenderCertificate(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(this: &Self::This, varsendercert: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SenderId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSenderIdType(this: &Self::This, lsenderidtype: i32) -> ::windows_core::Result<()>;
    fn Send(this: &Self::This, destinationqueue: ::core::option::Option<&IMSMQQueue2>, transaction: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AttachCurrentSecurityContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn SenderVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Extension(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetExtension(this: &Self::This, varextension: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ConnectorTypeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetConnectorTypeGuid(this: &Self::This, bstrguidconnectortype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TransactionStatusQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn DestinationSymmetricKey(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetDestinationSymmetricKey(this: &Self::This, vardestsymmkey: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Signature(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSignature(this: &Self::This, varsignature: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AuthenticationProviderType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthenticationProviderType(this: &Self::This, lauthprovtype: i32) -> ::windows_core::Result<()>;
    fn AuthenticationProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAuthenticationProviderName(this: &Self::This, bstrauthprovname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSenderId(this: &Self::This, varsenderid: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MsgClass(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMsgClass(this: &Self::This, lmsgclass: i32) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsFirstInTransaction(this: &Self::This) -> ::windows_core::Result<i16>;
    fn IsLastInTransaction(this: &Self::This) -> ::windows_core::Result<i16>;
    fn ResponseQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows_core::Result<()>;
    fn AdminQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(this: &Self::This) -> ::windows_core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQMessage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQMessage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Class<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Class(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthLevel(this, ::core::mem::transmute_copy(&lauthlevel)).into())
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAuthenticated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Delivery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelivery(this, ::core::mem::transmute_copy(&ldelivery)).into())
        }
        unsafe extern "system" fn Trace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Trace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrace(this, ::core::mem::transmute_copy(&ltrace)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&lpriority)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo_v1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo_v1(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppSpecific(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppSpecific(this, ::core::mem::transmute_copy(&lappspecific)).into())
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourceMachineGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BodyLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Body<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Body(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varbody: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&varbody)).into())
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo_v1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo_v1(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorrelationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCorrelationId(this, ::core::mem::transmute(&varmsgid)).into())
        }
        unsafe extern "system" fn Ack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Ack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAck(this, ::core::mem::transmute_copy(&lack)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReachQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReachQueue(this, ::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into())
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReceive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReceive(this, ::core::mem::transmute_copy(&lmaxtimetoreceive)).into())
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HashAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHashAlgorithm(this, ::core::mem::transmute_copy(&lhashalg)).into())
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EncryptAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncryptAlgorithm(this, ::core::mem::transmute_copy(&lencryptalg)).into())
        }
        unsafe extern "system" fn SentTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SentTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArrivedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderCertificate(this, ::core::mem::transmute(&varsendercert)).into())
        }
        unsafe extern "system" fn SenderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderIdType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderIdType(this, ::core::mem::transmute_copy(&lsenderidtype)).into())
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::windows_core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into())
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachCurrentSecurityContext(this).into())
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Extension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Extension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarextension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varextension: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtension(this, ::core::mem::transmute(&varextension)).into())
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectorTypeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidconnectortype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectorTypeGuid(this, ::core::mem::transmute(&bstrguidconnectortype)).into())
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionStatusQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoxactstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationSymmetricKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardestsymmkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardestsymmkey: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestinationSymmetricKey(this, ::core::mem::transmute(&vardestsymmkey)).into())
        }
        unsafe extern "system" fn Signature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Signature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsignature: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignature(this, ::core::mem::transmute(&varsignature)).into())
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationProviderType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthprovtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationProviderType(this, ::core::mem::transmute_copy(&lauthprovtype)).into())
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthprovname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationProviderName(this, ::core::mem::transmute(&bstrauthprovname)).into())
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsenderid: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderId(this, ::core::mem::transmute(&varsenderid)).into())
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MsgClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmsgclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMsgClass(this, ::core::mem::transmute_copy(&lmsgclass)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarxactid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFirstInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLastInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceivedAuthenticationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psreceivedauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQMessage2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
            SenderVersion: SenderVersion::<Identity, Impl, OFFSET>,
            Extension: Extension::<Identity, Impl, OFFSET>,
            SetExtension: SetExtension::<Identity, Impl, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, Impl, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, Impl, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, Impl, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, Impl, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, Impl, OFFSET>,
            Signature: Signature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, Impl, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetSenderId: SetSenderId::<Identity, Impl, OFFSET>,
            MsgClass: MsgClass::<Identity, Impl, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            TransactionId: TransactionId::<Identity, Impl, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, Impl, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage3_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Class(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn AuthLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthLevel(this: &Self::This, lauthlevel: i32) -> ::windows_core::Result<()>;
    fn IsAuthenticated(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Delivery(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDelivery(this: &Self::This, ldelivery: i32) -> ::windows_core::Result<()>;
    fn Trace(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetTrace(this: &Self::This, ltrace: i32) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPriority(this: &Self::This, lpriority: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn ResponseQueueInfo_v1(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn AppSpecific(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAppSpecific(this: &Self::This, lappspecific: i32) -> ::windows_core::Result<()>;
    fn SourceMachineGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BodyLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Body(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(this: &Self::This, varbody: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AdminQueueInfo_v1(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(this: &Self::This, varmsgid: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Ack(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAck(this: &Self::This, lack: i32) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxTimeToReachQueue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(this: &Self::This, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()>;
    fn MaxTimeToReceive(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReceive(this: &Self::This, lmaxtimetoreceive: i32) -> ::windows_core::Result<()>;
    fn HashAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHashAlgorithm(this: &Self::This, lhashalg: i32) -> ::windows_core::Result<()>;
    fn EncryptAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEncryptAlgorithm(this: &Self::This, lencryptalg: i32) -> ::windows_core::Result<()>;
    fn SentTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo3>;
    fn SenderCertificate(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(this: &Self::This, varsendercert: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SenderId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSenderIdType(this: &Self::This, lsenderidtype: i32) -> ::windows_core::Result<()>;
    fn Send(this: &Self::This, destinationqueue: ::core::option::Option<&super::Com::IDispatch>, transaction: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AttachCurrentSecurityContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn SenderVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Extension(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetExtension(this: &Self::This, varextension: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ConnectorTypeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetConnectorTypeGuid(this: &Self::This, bstrguidconnectortype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TransactionStatusQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo3>;
    fn DestinationSymmetricKey(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetDestinationSymmetricKey(this: &Self::This, vardestsymmkey: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Signature(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSignature(this: &Self::This, varsignature: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AuthenticationProviderType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthenticationProviderType(this: &Self::This, lauthprovtype: i32) -> ::windows_core::Result<()>;
    fn AuthenticationProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAuthenticationProviderName(this: &Self::This, bstrauthprovname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSenderId(this: &Self::This, varsenderid: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MsgClass(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMsgClass(this: &Self::This, lmsgclass: i32) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsFirstInTransaction(this: &Self::This) -> ::windows_core::Result<i16>;
    fn IsLastInTransaction(this: &Self::This) -> ::windows_core::Result<i16>;
    fn ResponseQueueInfo_v2(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows_core::Result<()>;
    fn AdminQueueInfo_v2(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(this: &Self::This) -> ::windows_core::Result<i16>;
    fn ResponseQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo3>;
    fn putref_ResponseQueueInfo(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo3>) -> ::windows_core::Result<()>;
    fn AdminQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo3>;
    fn putref_AdminQueueInfo(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo3>) -> ::windows_core::Result<()>;
    fn ResponseDestination(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(this: &Self::This, pdestresponse: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Destination(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn LookupId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsAuthenticated2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(this: &Self::This) -> ::windows_core::Result<()>;
    fn SoapEnvelope(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CompoundMessage(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSoapHeader(this: &Self::This, bstrsoapheader: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSoapBody(this: &Self::This, bstrsoapbody: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQMessage3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQMessage3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Class<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Class(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthLevel(this, ::core::mem::transmute_copy(&lauthlevel)).into())
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAuthenticated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Delivery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelivery(this, ::core::mem::transmute_copy(&ldelivery)).into())
        }
        unsafe extern "system" fn Trace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Trace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrace(this, ::core::mem::transmute_copy(&ltrace)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&lpriority)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo_v1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo_v1(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppSpecific(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppSpecific(this, ::core::mem::transmute_copy(&lappspecific)).into())
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourceMachineGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BodyLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Body<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Body(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varbody: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&varbody)).into())
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo_v1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo_v1(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorrelationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCorrelationId(this, ::core::mem::transmute(&varmsgid)).into())
        }
        unsafe extern "system" fn Ack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Ack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAck(this, ::core::mem::transmute_copy(&lack)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReachQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReachQueue(this, ::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into())
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReceive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReceive(this, ::core::mem::transmute_copy(&lmaxtimetoreceive)).into())
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HashAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHashAlgorithm(this, ::core::mem::transmute_copy(&lhashalg)).into())
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EncryptAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncryptAlgorithm(this, ::core::mem::transmute_copy(&lencryptalg)).into())
        }
        unsafe extern "system" fn SentTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SentTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArrivedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderCertificate(this, ::core::mem::transmute(&varsendercert)).into())
        }
        unsafe extern "system" fn SenderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderIdType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderIdType(this, ::core::mem::transmute_copy(&lsenderidtype)).into())
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::windows_core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into())
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachCurrentSecurityContext(this).into())
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Extension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Extension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarextension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varextension: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtension(this, ::core::mem::transmute(&varextension)).into())
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectorTypeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidconnectortype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectorTypeGuid(this, ::core::mem::transmute(&bstrguidconnectortype)).into())
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionStatusQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoxactstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationSymmetricKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardestsymmkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardestsymmkey: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestinationSymmetricKey(this, ::core::mem::transmute(&vardestsymmkey)).into())
        }
        unsafe extern "system" fn Signature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Signature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsignature: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignature(this, ::core::mem::transmute(&varsignature)).into())
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationProviderType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthprovtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationProviderType(this, ::core::mem::transmute_copy(&lauthprovtype)).into())
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthprovname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationProviderName(this, ::core::mem::transmute(&bstrauthprovname)).into())
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsenderid: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderId(this, ::core::mem::transmute(&varsenderid)).into())
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MsgClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmsgclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMsgClass(this, ::core::mem::transmute_copy(&lmsgclass)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarxactid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFirstInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLastInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo_v2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo_v2(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo_v2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo_v2(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceivedAuthenticationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psreceivedauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn ResponseDestination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseDestination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestresponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseDestination(this, ::windows_core::from_raw_borrowed(&pdestresponse)).into())
        }
        unsafe extern "system" fn Destination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Destination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestdestination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarlookupid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAuthenticated2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFirstInTransaction2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLastInTransaction2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachCurrentSecurityContext2(this).into())
        }
        unsafe extern "system" fn SoapEnvelope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SoapEnvelope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsoapenvelope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompoundMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompoundMessage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcompoundmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSoapHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSoapHeader(this, ::core::mem::transmute(&bstrsoapheader)).into())
        }
        unsafe extern "system" fn SetSoapBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSoapBody(this, ::core::mem::transmute(&bstrsoapbody)).into())
        }
        IMSMQMessage3_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
            SenderVersion: SenderVersion::<Identity, Impl, OFFSET>,
            Extension: Extension::<Identity, Impl, OFFSET>,
            SetExtension: SetExtension::<Identity, Impl, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, Impl, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, Impl, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, Impl, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, Impl, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, Impl, OFFSET>,
            Signature: Signature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, Impl, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetSenderId: SetSenderId::<Identity, Impl, OFFSET>,
            MsgClass: MsgClass::<Identity, Impl, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            TransactionId: TransactionId::<Identity, Impl, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, Impl, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, Impl, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            LookupId: LookupId::<Identity, Impl, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, Impl, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, Impl, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, Impl, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, Impl, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, Impl, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, Impl, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQMessage4_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Class(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn AuthLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthLevel(this: &Self::This, lauthlevel: i32) -> ::windows_core::Result<()>;
    fn IsAuthenticated(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Delivery(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDelivery(this: &Self::This, ldelivery: i32) -> ::windows_core::Result<()>;
    fn Trace(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetTrace(this: &Self::This, ltrace: i32) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPriority(this: &Self::This, lpriority: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn ResponseQueueInfo_v1(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn AppSpecific(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAppSpecific(this: &Self::This, lappspecific: i32) -> ::windows_core::Result<()>;
    fn SourceMachineGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BodyLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Body(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetBody(this: &Self::This, varbody: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AdminQueueInfo_v1(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn CorrelationId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetCorrelationId(this: &Self::This, varmsgid: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Ack(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAck(this: &Self::This, lack: i32) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxTimeToReachQueue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(this: &Self::This, lmaxtimetoreachqueue: i32) -> ::windows_core::Result<()>;
    fn MaxTimeToReceive(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxTimeToReceive(this: &Self::This, lmaxtimetoreceive: i32) -> ::windows_core::Result<()>;
    fn HashAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHashAlgorithm(this: &Self::This, lhashalg: i32) -> ::windows_core::Result<()>;
    fn EncryptAlgorithm(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEncryptAlgorithm(this: &Self::This, lencryptalg: i32) -> ::windows_core::Result<()>;
    fn SentTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ArrivedTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DestinationQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo4>;
    fn SenderCertificate(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSenderCertificate(this: &Self::This, varsendercert: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SenderId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SenderIdType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSenderIdType(this: &Self::This, lsenderidtype: i32) -> ::windows_core::Result<()>;
    fn Send(this: &Self::This, destinationqueue: ::core::option::Option<&super::Com::IDispatch>, transaction: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AttachCurrentSecurityContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn SenderVersion(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Extension(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetExtension(this: &Self::This, varextension: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ConnectorTypeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetConnectorTypeGuid(this: &Self::This, bstrguidconnectortype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TransactionStatusQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo4>;
    fn DestinationSymmetricKey(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetDestinationSymmetricKey(this: &Self::This, vardestsymmkey: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Signature(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSignature(this: &Self::This, varsignature: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AuthenticationProviderType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthenticationProviderType(this: &Self::This, lauthprovtype: i32) -> ::windows_core::Result<()>;
    fn AuthenticationProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAuthenticationProviderName(this: &Self::This, bstrauthprovname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSenderId(this: &Self::This, varsenderid: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MsgClass(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMsgClass(this: &Self::This, lmsgclass: i32) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsFirstInTransaction(this: &Self::This) -> ::windows_core::Result<i16>;
    fn IsLastInTransaction(this: &Self::This) -> ::windows_core::Result<i16>;
    fn ResponseQueueInfo_v2(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows_core::Result<()>;
    fn AdminQueueInfo_v2(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(this: &Self::This) -> ::windows_core::Result<i16>;
    fn ResponseQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo4>;
    fn putref_ResponseQueueInfo(this: &Self::This, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo4>) -> ::windows_core::Result<()>;
    fn AdminQueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo4>;
    fn putref_AdminQueueInfo(this: &Self::This, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo4>) -> ::windows_core::Result<()>;
    fn ResponseDestination(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(this: &Self::This, pdestresponse: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Destination(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn LookupId(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsAuthenticated2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(this: &Self::This) -> ::windows_core::Result<()>;
    fn SoapEnvelope(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CompoundMessage(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSoapHeader(this: &Self::This, bstrsoapheader: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSoapBody(this: &Self::This, bstrsoapbody: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQMessage4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQMessage4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Class<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Class(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthLevel(this, ::core::mem::transmute_copy(&lauthlevel)).into())
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAuthenticated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Delivery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDelivery(this, ::core::mem::transmute_copy(&ldelivery)).into())
        }
        unsafe extern "system" fn Trace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Trace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrace(this, ::core::mem::transmute_copy(&ltrace)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&lpriority)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo_v1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo_v1(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppSpecific(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppSpecific(this, ::core::mem::transmute_copy(&lappspecific)).into())
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SourceMachineGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BodyLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Body<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Body(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varbody: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&varbody)).into())
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo_v1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo_v1(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorrelationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varmsgid: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCorrelationId(this, ::core::mem::transmute(&varmsgid)).into())
        }
        unsafe extern "system" fn Ack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Ack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAck(this, ::core::mem::transmute_copy(&lack)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReachQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReachQueue(this, ::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into())
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxTimeToReceive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxTimeToReceive(this, ::core::mem::transmute_copy(&lmaxtimetoreceive)).into())
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HashAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHashAlgorithm(this, ::core::mem::transmute_copy(&lhashalg)).into())
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EncryptAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEncryptAlgorithm(this, ::core::mem::transmute_copy(&lencryptalg)).into())
        }
        unsafe extern "system" fn SentTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SentTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ArrivedTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsendercert: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderCertificate(this, ::core::mem::transmute(&varsendercert)).into())
        }
        unsafe extern "system" fn SenderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderIdType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderIdType(this, ::core::mem::transmute_copy(&lsenderidtype)).into())
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::windows_core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into())
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachCurrentSecurityContext(this).into())
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SenderVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Extension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Extension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarextension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varextension: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtension(this, ::core::mem::transmute(&varextension)).into())
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectorTypeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidconnectortype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectorTypeGuid(this, ::core::mem::transmute(&bstrguidconnectortype)).into())
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionStatusQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoxactstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationSymmetricKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardestsymmkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardestsymmkey: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestinationSymmetricKey(this, ::core::mem::transmute(&vardestsymmkey)).into())
        }
        unsafe extern "system" fn Signature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Signature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsignature: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignature(this, ::core::mem::transmute(&varsignature)).into())
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationProviderType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthprovtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationProviderType(this, ::core::mem::transmute_copy(&lauthprovtype)).into())
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthprovname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationProviderName(this, ::core::mem::transmute(&bstrauthprovname)).into())
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsenderid: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSenderId(this, ::core::mem::transmute(&varsenderid)).into())
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MsgClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmsgclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMsgClass(this, ::core::mem::transmute_copy(&lmsgclass)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarxactid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFirstInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLastInTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo_v2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo_v2(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo_v2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo_v2(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceivedAuthenticationLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psreceivedauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinforesponse)).into())
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdminQueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_AdminQueueInfo(this, ::windows_core::from_raw_borrowed(&pqinfoadmin)).into())
        }
        unsafe extern "system" fn ResponseDestination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResponseDestination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestresponse: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_ResponseDestination(this, ::windows_core::from_raw_borrowed(&pdestresponse)).into())
        }
        unsafe extern "system" fn Destination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Destination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestdestination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarlookupid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAuthenticated2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFirstInTransaction2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLastInTransaction2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AttachCurrentSecurityContext2(this).into())
        }
        unsafe extern "system" fn SoapEnvelope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SoapEnvelope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsoapenvelope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompoundMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompoundMessage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcompoundmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSoapHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSoapHeader(this, ::core::mem::transmute(&bstrsoapheader)).into())
        }
        unsafe extern "system" fn SetSoapBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSoapBody(this, ::core::mem::transmute(&bstrsoapbody)).into())
        }
        IMSMQMessage4_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
            SenderVersion: SenderVersion::<Identity, Impl, OFFSET>,
            Extension: Extension::<Identity, Impl, OFFSET>,
            SetExtension: SetExtension::<Identity, Impl, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, Impl, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, Impl, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, Impl, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, Impl, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, Impl, OFFSET>,
            Signature: Signature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, Impl, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetSenderId: SetSenderId::<Identity, Impl, OFFSET>,
            MsgClass: MsgClass::<Identity, Impl, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            TransactionId: TransactionId::<Identity, Impl, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, Impl, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, Impl, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            LookupId: LookupId::<Identity, Impl, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, Impl, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, Impl, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, Impl, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, Impl, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, Impl, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, Impl, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQOutgoingQueueManagement_Impl: ::windows_core::BaseImpl + IMSMQManagement_Impl {
    fn State(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NextHops(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn EodGetSendInfo(this: &Self::This) -> ::windows_core::Result<IMSMQCollection>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn EodResend(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQOutgoingQueueManagement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQManagement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQOutgoingQueueManagement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextHops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextHops(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvnexthops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EodGetSendInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EodGetSendInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn EodResend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EodResend(this).into())
        }
        IMSMQOutgoingQueueManagement_Vtbl {
            base__: <IMSMQManagement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            State: State::<Identity, Impl, OFFSET>,
            NextHops: NextHops::<Identity, Impl, OFFSET>,
            EodGetSendInfo: EodGetSendInfo::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            EodResend: EodResend::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQPrivateDestination_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Handle(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetHandle(this: &Self::This, varhandle: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQPrivateDestination {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQPrivateDestination_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQPrivateDestination {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQPrivateDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQPrivateDestination_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varhandle: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHandle(this, ::core::mem::transmute(&varhandle)).into())
        }
        IMSMQPrivateDestination_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Handle: Handle::<Identity, Impl, OFFSET>,
            SetHandle: SetHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQPrivateEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Hwnd(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FireArrivedEvent(this: &Self::This, pq: ::core::option::Option<&IMSMQQueue>, msgcursor: i32) -> ::windows_core::Result<()>;
    fn FireArrivedErrorEvent(this: &Self::This, pq: ::core::option::Option<&IMSMQQueue>, hrstatus: ::windows_core::HRESULT, msgcursor: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQPrivateEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQPrivateEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Hwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hwnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FireArrivedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pq: *mut ::core::ffi::c_void, msgcursor: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireArrivedEvent(this, ::windows_core::from_raw_borrowed(&pq), ::core::mem::transmute_copy(&msgcursor)).into())
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pq: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, msgcursor: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireArrivedErrorEvent(this, ::windows_core::from_raw_borrowed(&pq), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&msgcursor)).into())
        }
        IMSMQPrivateEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Hwnd: Hwnd::<Identity, Impl, OFFSET>,
            FireArrivedEvent: FireArrivedEvent::<Identity, Impl, OFFSET>,
            FireArrivedErrorEvent: FireArrivedErrorEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn LookupQueue(this: &Self::This, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LookupQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupQueue(this, ::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQuery_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LookupQueue: LookupQueue::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn LookupQueue(this: &Self::This, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos2>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQuery2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQuery2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LookupQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupQueue(this, ::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQuery2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery3_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(this: &Self::This, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos3>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn LookupQueue(this: &Self::This, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQuery3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQuery3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LookupQueue_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupQueue_v2(this, ::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::LookupQueue(
                    this,
                    ::core::mem::transmute_copy(&queueguid),
                    ::core::mem::transmute_copy(&servicetypeguid),
                    ::core::mem::transmute_copy(&label),
                    ::core::mem::transmute_copy(&createtime),
                    ::core::mem::transmute_copy(&modifytime),
                    ::core::mem::transmute_copy(&relservicetype),
                    ::core::mem::transmute_copy(&rellabel),
                    ::core::mem::transmute_copy(&relcreatetime),
                    ::core::mem::transmute_copy(&relmodifytime),
                    ::core::mem::transmute_copy(&multicastaddress),
                    ::core::mem::transmute_copy(&relmulticastaddress),
                ) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        IMSMQQuery3_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LookupQueue_v2: LookupQueue_v2::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQuery4_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(this: &Self::This, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos4>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn LookupQueue(this: &Self::This, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQQueueInfos4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQuery4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQuery4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LookupQueue_v2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupQueue_v2(this, ::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Variant::VARIANT, servicetypeguid: *const super::Variant::VARIANT, label: *const super::Variant::VARIANT, createtime: *const super::Variant::VARIANT, modifytime: *const super::Variant::VARIANT, relservicetype: *const super::Variant::VARIANT, rellabel: *const super::Variant::VARIANT, relcreatetime: *const super::Variant::VARIANT, relmodifytime: *const super::Variant::VARIANT, multicastaddress: *const super::Variant::VARIANT, relmulticastaddress: *const super::Variant::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::LookupQueue(
                    this,
                    ::core::mem::transmute_copy(&queueguid),
                    ::core::mem::transmute_copy(&servicetypeguid),
                    ::core::mem::transmute_copy(&label),
                    ::core::mem::transmute_copy(&createtime),
                    ::core::mem::transmute_copy(&modifytime),
                    ::core::mem::transmute_copy(&relservicetype),
                    ::core::mem::transmute_copy(&rellabel),
                    ::core::mem::transmute_copy(&relcreatetime),
                    ::core::mem::transmute_copy(&relmodifytime),
                    ::core::mem::transmute_copy(&multicastaddress),
                    ::core::mem::transmute_copy(&relmulticastaddress),
                ) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        IMSMQQuery4_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LookupQueue_v2: LookupQueue_v2::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Access(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ShareMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn QueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IsOpen(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Receive(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn Peek(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn EnableNotification(this: &Self::This, event: ::core::option::Option<&IMSMQEvent>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReceiveCurrent(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekNext(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekCurrent(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Access<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Access(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShareMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Receive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Receive(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Peek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Peek(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableNotification(this, ::windows_core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveCurrent(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNext(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekCurrent(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueue_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Access(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ShareMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn QueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IsOpen(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Receive_v1(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn Peek_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn EnableNotification(this: &Self::This, event: ::core::option::Option<&IMSMQEvent2>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReceiveCurrent_v1(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn Receive(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage2>;
    fn Peek(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage2>;
    fn ReceiveCurrent(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage2>;
    fn PeekNext(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage2>;
    fn PeekCurrent(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage2>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueue2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueue2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Access<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Access(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShareMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Receive_v1(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Peek_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableNotification(this, ::windows_core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveCurrent_v1(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNext_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekCurrent_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Receive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Receive(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Peek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Peek(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveCurrent(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNext(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekCurrent(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueue2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive_v1: Receive_v1::<Identity, Impl, OFFSET>,
            Peek_v1: Peek_v1::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, Impl, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, Impl, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue3_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Access(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ShareMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn QueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo3>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IsOpen(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Receive_v1(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn Peek_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn EnableNotification(this: &Self::This, event: ::core::option::Option<&IMSMQEvent3>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReceiveCurrent_v1(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn Receive(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn Peek(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn ReceiveCurrent(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn PeekNext(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn PeekCurrent(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Handle2(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ReceiveByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn ReceiveNextByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn ReceivePreviousByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn ReceiveFirstByLookupId(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn ReceiveLastByLookupId(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn PeekByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn PeekNextByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn PeekPreviousByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn PeekFirstByLookupId(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn PeekLastByLookupId(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage3>;
    fn Purge(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsOpen2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueue3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueue3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Access<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Access(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShareMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Receive_v1(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Peek_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableNotification(this, ::windows_core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveCurrent_v1(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNext_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekCurrent_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Receive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Receive(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Peek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Peek(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveCurrent(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNext(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekCurrent(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveNextByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceivePreviousByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveFirstByLookupId(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveLastByLookupId(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNextByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekPreviousByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekFirstByLookupId(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekLastByLookupId(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Purge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Purge(this).into())
        }
        unsafe extern "system" fn IsOpen2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpen2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueue3_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive_v1: Receive_v1::<Identity, Impl, OFFSET>,
            Peek_v1: Peek_v1::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, Impl, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, Impl, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Handle2: Handle2::<Identity, Impl, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, Impl, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, Impl, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, Impl, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, Impl, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, Impl, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, Impl, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, Impl, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, Impl, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, Impl, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            IsOpen2: IsOpen2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueue4_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Access(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ShareMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn QueueInfo(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo4>;
    fn Handle(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IsOpen(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Receive_v1(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn Peek_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn EnableNotification(this: &Self::This, event: ::core::option::Option<&IMSMQEvent3>, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReceiveCurrent_v1(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage>;
    fn Receive(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn Peek(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn ReceiveCurrent(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn PeekNext(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn PeekCurrent(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Handle2(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ReceiveByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn ReceiveNextByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn ReceivePreviousByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn ReceiveFirstByLookupId(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn ReceiveLastByLookupId(this: &Self::This, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn PeekByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn PeekNextByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn PeekPreviousByLookupId(this: &Self::This, lookupid: &super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn PeekFirstByLookupId(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn PeekLastByLookupId(this: &Self::This, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
    fn Purge(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsOpen2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiveByLookupIdAllowPeek(this: &Self::This, lookupid: &super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT) -> ::windows_core::Result<IMSMQMessage4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueue4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueue4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Access<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Access(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShareMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Receive_v1(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Peek_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableNotification(this, ::windows_core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveCurrent_v1(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNext_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekCurrent_v1(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Receive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Receive(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Peek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Peek(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveCurrent(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNext(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, receivetimeout: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekCurrent(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Handle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveNextByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceivePreviousByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveFirstByLookupId(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveLastByLookupId(this, ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekNextByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekPreviousByLookupId(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekFirstByLookupId(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PeekLastByLookupId(this, ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Purge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Purge(this).into())
        }
        unsafe extern "system" fn IsOpen2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOpen2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookupid: super::Variant::VARIANT, transaction: *const super::Variant::VARIANT, wantdestinationqueue: *const super::Variant::VARIANT, wantbody: *const super::Variant::VARIANT, wantconnectortype: *const super::Variant::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReceiveByLookupIdAllowPeek(this, ::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueue4_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive_v1: Receive_v1::<Identity, Impl, OFFSET>,
            Peek_v1: Peek_v1::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, Impl, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, Impl, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Handle2: Handle2::<Identity, Impl, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, Impl, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, Impl, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, Impl, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, Impl, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, Impl, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, Impl, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, Impl, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, Impl, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, Impl, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            IsOpen2: IsOpen2::<Identity, Impl, OFFSET>,
            ReceiveByLookupIdAllowPeek: ReceiveByLookupIdAllowPeek::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn QueueGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServiceTypeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceTypeGuid(this: &Self::This, bstrguidservicetype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PathName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPathName(this: &Self::This, bstrpathname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FormatName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFormatName(this: &Self::This, bstrformatname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsTransactional(this: &Self::This) -> ::windows_core::Result<i16>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn Quota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetQuota(this: &Self::This, lquota: i32) -> ::windows_core::Result<()>;
    fn BasePriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBasePriority(this: &Self::This, lbasepriority: i32) -> ::windows_core::Result<()>;
    fn CreateTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthenticate(this: &Self::This, lauthenticate: i32) -> ::windows_core::Result<()>;
    fn JournalQuota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournalQuota(this: &Self::This, ljournalquota: i32) -> ::windows_core::Result<()>;
    fn IsWorldReadable(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Create(this: &Self::This, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueueGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceTypeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceTypeGuid(this, ::core::mem::transmute(&bstrguidservicetype)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn PathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPathName(this, ::core::mem::transmute(&bstrpathname)).into())
        }
        unsafe extern "system" fn FormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormatName(this, ::core::mem::transmute(&bstrformatname)).into())
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTransactional(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn Quota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Quota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuota(this, ::core::mem::transmute_copy(&lquota)).into())
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BasePriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBasePriority(this, ::core::mem::transmute_copy(&lbasepriority)).into())
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModifyTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Authenticate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticate(this, ::core::mem::transmute_copy(&lauthenticate)).into())
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JournalQuota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournalQuota(this, ::core::mem::transmute_copy(&ljournalquota)).into())
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorldReadable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this, ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this).into())
        }
        IMSMQQueueInfo_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn QueueGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServiceTypeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceTypeGuid(this: &Self::This, bstrguidservicetype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PathName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPathName(this: &Self::This, bstrpathname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FormatName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFormatName(this: &Self::This, bstrformatname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsTransactional(this: &Self::This) -> ::windows_core::Result<i16>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn Quota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetQuota(this: &Self::This, lquota: i32) -> ::windows_core::Result<()>;
    fn BasePriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBasePriority(this: &Self::This, lbasepriority: i32) -> ::windows_core::Result<()>;
    fn CreateTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthenticate(this: &Self::This, lauthenticate: i32) -> ::windows_core::Result<()>;
    fn JournalQuota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournalQuota(this: &Self::This, ljournalquota: i32) -> ::windows_core::Result<()>;
    fn IsWorldReadable(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Create(this: &Self::This, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue2>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This) -> ::windows_core::Result<()>;
    fn PathNameDNS(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Security(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSecurity(this: &Self::This, varsecurity: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueueGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceTypeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceTypeGuid(this, ::core::mem::transmute(&bstrguidservicetype)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn PathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPathName(this, ::core::mem::transmute(&bstrpathname)).into())
        }
        unsafe extern "system" fn FormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormatName(this, ::core::mem::transmute(&bstrformatname)).into())
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTransactional(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn Quota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Quota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuota(this, ::core::mem::transmute_copy(&lquota)).into())
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BasePriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBasePriority(this, ::core::mem::transmute_copy(&lbasepriority)).into())
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModifyTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Authenticate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticate(this, ::core::mem::transmute_copy(&lauthenticate)).into())
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JournalQuota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournalQuota(this, ::core::mem::transmute_copy(&ljournalquota)).into())
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorldReadable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this, ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this).into())
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathNameDNS(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathnamedns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsecurity: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurity(this, ::core::mem::transmute(&varsecurity)).into())
        }
        IMSMQQueueInfo2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo3_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn QueueGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServiceTypeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceTypeGuid(this: &Self::This, bstrguidservicetype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PathName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPathName(this: &Self::This, bstrpathname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FormatName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFormatName(this: &Self::This, bstrformatname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsTransactional(this: &Self::This) -> ::windows_core::Result<i16>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn Quota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetQuota(this: &Self::This, lquota: i32) -> ::windows_core::Result<()>;
    fn BasePriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBasePriority(this: &Self::This, lbasepriority: i32) -> ::windows_core::Result<()>;
    fn CreateTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthenticate(this: &Self::This, lauthenticate: i32) -> ::windows_core::Result<()>;
    fn JournalQuota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournalQuota(this: &Self::This, ljournalquota: i32) -> ::windows_core::Result<()>;
    fn IsWorldReadable(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Create(this: &Self::This, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue3>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This) -> ::windows_core::Result<()>;
    fn PathNameDNS(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Security(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSecurity(this: &Self::This, varsecurity: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn IsTransactional2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMulticastAddress(this: &Self::This, bstrmulticastaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ADsPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfo3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfo3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueueGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceTypeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceTypeGuid(this, ::core::mem::transmute(&bstrguidservicetype)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn PathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPathName(this, ::core::mem::transmute(&bstrpathname)).into())
        }
        unsafe extern "system" fn FormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormatName(this, ::core::mem::transmute(&bstrformatname)).into())
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTransactional(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn Quota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Quota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuota(this, ::core::mem::transmute_copy(&lquota)).into())
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BasePriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBasePriority(this, ::core::mem::transmute_copy(&lbasepriority)).into())
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModifyTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Authenticate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticate(this, ::core::mem::transmute_copy(&lauthenticate)).into())
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JournalQuota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournalQuota(this, ::core::mem::transmute_copy(&ljournalquota)).into())
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorldReadable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this, ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this).into())
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathNameDNS(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathnamedns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsecurity: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurity(this, ::core::mem::transmute(&varsecurity)).into())
        }
        unsafe extern "system" fn IsTransactional2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTransactional2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorldReadable2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MulticastAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MulticastAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmulticastaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMulticastAddress(this, ::core::mem::transmute(&bstrmulticastaddress)).into())
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADsPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueueInfo3_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, Impl, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, Impl, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, Impl, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfo4_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn QueueGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServiceTypeGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceTypeGuid(this: &Self::This, bstrguidservicetype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PathName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPathName(this: &Self::This, bstrpathname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FormatName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFormatName(this: &Self::This, bstrformatname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsTransactional(this: &Self::This) -> ::windows_core::Result<i16>;
    fn PrivLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivLevel(this: &Self::This, lprivlevel: i32) -> ::windows_core::Result<()>;
    fn Journal(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournal(this: &Self::This, ljournal: i32) -> ::windows_core::Result<()>;
    fn Quota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetQuota(this: &Self::This, lquota: i32) -> ::windows_core::Result<()>;
    fn BasePriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBasePriority(this: &Self::This, lbasepriority: i32) -> ::windows_core::Result<()>;
    fn CreateTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ModifyTime(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Authenticate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAuthenticate(this: &Self::This, lauthenticate: i32) -> ::windows_core::Result<()>;
    fn JournalQuota(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetJournalQuota(this: &Self::This, ljournalquota: i32) -> ::windows_core::Result<()>;
    fn IsWorldReadable(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Create(this: &Self::This, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, access: i32, sharemode: i32) -> ::windows_core::Result<IMSMQQueue4>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This) -> ::windows_core::Result<()>;
    fn PathNameDNS(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Security(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetSecurity(this: &Self::This, varsecurity: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn IsTransactional2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMulticastAddress(this: &Self::This, bstrmulticastaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ADsPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfo4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfo4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueueGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceTypeGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceTypeGuid(this, ::core::mem::transmute(&bstrguidservicetype)).into())
        }
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn PathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPathName(this, ::core::mem::transmute(&bstrpathname)).into())
        }
        unsafe extern "system" fn FormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormatName(this, ::core::mem::transmute(&bstrformatname)).into())
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTransactional(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivLevel(this, ::core::mem::transmute_copy(&lprivlevel)).into())
        }
        unsafe extern "system" fn Journal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Journal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournal(this, ::core::mem::transmute_copy(&ljournal)).into())
        }
        unsafe extern "system" fn Quota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Quota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuota(this, ::core::mem::transmute_copy(&lquota)).into())
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BasePriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBasePriority(this, ::core::mem::transmute_copy(&lbasepriority)).into())
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModifyTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Authenticate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticate(this, ::core::mem::transmute_copy(&lauthenticate)).into())
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JournalQuota(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJournalQuota(this, ::core::mem::transmute_copy(&ljournalquota)).into())
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorldReadable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Variant::VARIANT, isworldreadable: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this, ::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this).into())
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PathNameDNS(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathnamedns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Security<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Security(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsecurity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsecurity: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurity(this, ::core::mem::transmute(&varsecurity)).into())
        }
        unsafe extern "system" fn IsTransactional2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTransactional2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorldReadable2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MulticastAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MulticastAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmulticastaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMulticastAddress(this, ::core::mem::transmute(&bstrmulticastaddress)).into())
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADsPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueueInfo4_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, Impl, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, Impl, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, Impl, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfos {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfos {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueueInfos_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo2>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfos2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfos2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueueInfos2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos3_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo3>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfos3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfos3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueueInfos3_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueInfos4_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This) -> ::windows_core::Result<IMSMQQueueInfo4>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueInfos4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueInfos4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueueInfos4_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQQueueManagement_Impl: ::windows_core::BaseImpl + IMSMQManagement_Impl {
    fn JournalMessageCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn BytesInJournal(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn EodGetReceiveInfo(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQQueueManagement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQManagement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQQueueManagement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn JournalMessageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JournalMessageCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalmessagecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BytesInJournal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BytesInJournal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbytesinjournal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EodGetReceiveInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EodGetReceiveInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQQueueManagement_Vtbl {
            base__: <IMSMQManagement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            JournalMessageCount: JournalMessageCount::<Identity, Impl, OFFSET>,
            BytesInJournal: BytesInJournal::<Identity, Impl, OFFSET>,
            EodGetReceiveInfo: EodGetReceiveInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransaction_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Transaction(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Commit(this: &Self::This, fretaining: *const super::Variant::VARIANT, grftc: *const super::Variant::VARIANT, grfrm: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This, fretaining: *const super::Variant::VARIANT, fasync: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQTransaction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQTransaction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Transaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Transaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Variant::VARIANT, grftc: *const super::Variant::VARIANT, grfrm: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Variant::VARIANT, fasync: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this, ::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into())
        }
        IMSMQTransaction_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Transaction: Transaction::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransaction2_Impl: ::windows_core::BaseImpl + IMSMQTransaction_Impl {
    fn InitNew(this: &Self::This, vartransaction: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQTransaction2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQTransaction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQTransaction2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartransaction: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this, ::core::mem::transmute(&vartransaction)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQTransaction2_Vtbl {
            base__: <IMSMQTransaction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransaction3_Impl: ::windows_core::BaseImpl + IMSMQTransaction2_Impl {
    fn ITransaction(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQTransaction3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMSMQTransaction2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQTransaction3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ITransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransaction3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ITransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaritransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQTransaction3_Vtbl { base__: <IMSMQTransaction2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ITransaction: ITransaction::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransactionDispenser_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BeginTransaction(this: &Self::This) -> ::windows_core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQTransactionDispenser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQTransactionDispenser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQTransactionDispenser_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransactionDispenser2_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BeginTransaction(this: &Self::This) -> ::windows_core::Result<IMSMQTransaction2>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQTransactionDispenser2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQTransactionDispenser2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQTransactionDispenser2_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSMQTransactionDispenser3_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn BeginTransaction(this: &Self::This) -> ::windows_core::Result<IMSMQTransaction3>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSMQTransactionDispenser3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSMQTransactionDispenser3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginTransaction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMSMQTransactionDispenser3_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _DMSMQEventEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _DMSMQEventEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _DMSMQEventEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _DMSMQEventEvents {
    const VTABLE: Self::Vtable = { _DMSMQEventEvents_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
