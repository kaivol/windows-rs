#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugExtendedProperty_Impl: ::windows_core::BaseImpl + IDebugProperty_Impl {
    fn GetExtendedPropertyInfo(this: &Self::This, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> ::windows_core::Result<()>;
    fn EnumExtendedMembers(this: &Self::This, dwfieldspec: u32, nradix: u32) -> ::windows_core::Result<IEnumDebugExtendedPropertyInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDebugExtendedProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugProperty);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExtendedProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugExtendedProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExtendedPropertyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExtendedProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendedPropertyInfo(this, ::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&pextendedpropertyinfo)).into())
        }
        unsafe extern "system" fn EnumExtendedMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExtendedProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppeepi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumExtendedMembers(this, ::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeepi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugExtendedProperty_Vtbl {
            base__: <IDebugProperty as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetExtendedPropertyInfo: GetExtendedPropertyInfo::<Identity, Impl, OFFSET>,
            EnumExtendedMembers: EnumExtendedMembers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugProperty_Impl: ::windows_core::BaseImpl {
    fn GetPropertyInfo(this: &Self::This, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows_core::Result<()>;
    fn GetExtendedInfo(this: &Self::This, cinfos: u32, rgguidextendedinfo: *const ::windows_core::GUID, rgvar: *mut super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetValueAsString(this: &Self::This, pszvalue: &::windows_core::PCWSTR, nradix: u32) -> ::windows_core::Result<()>;
    fn EnumMembers(this: &Self::This, dwfieldspec: u32, nradix: u32, refiid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumDebugPropertyInfo>;
    fn GetParent(this: &Self::This) -> ::windows_core::Result<IDebugProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDebugProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyInfo(this, ::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&ppropertyinfo)).into())
        }
        unsafe extern "system" fn GetExtendedInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinfos: u32, rgguidextendedinfo: *const ::windows_core::GUID, rgvar: *mut super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendedInfo(this, ::core::mem::transmute_copy(&cinfos), ::core::mem::transmute_copy(&rgguidextendedinfo), ::core::mem::transmute_copy(&rgvar)).into())
        }
        unsafe extern "system" fn SetValueAsString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvalue: ::windows_core::PCWSTR, nradix: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueAsString(this, ::core::mem::transmute(&pszvalue), ::core::mem::transmute_copy(&nradix)).into())
        }
        unsafe extern "system" fn EnumMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, refiid: *const ::windows_core::GUID, ppepi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumMembers(this, ::core::mem::transmute_copy(&dwfieldspec), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&refiid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppepi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdebugprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            GetExtendedInfo: GetExtendedInfo::<Identity, Impl, OFFSET>,
            SetValueAsString: SetValueAsString::<Identity, Impl, OFFSET>,
            EnumMembers: EnumMembers::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDebugPropertyEnumType_All_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IDebugPropertyEnumType_All {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugPropertyEnumType_All_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugPropertyEnumType_All {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugPropertyEnumType_All_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__idebugpropertyenumtype_all0000: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__idebugpropertyenumtype_all0000, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugPropertyEnumType_All_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetName: GetName::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDebugPropertyEnumType_Arguments_Impl: ::windows_core::BaseImpl + IDebugPropertyEnumType_All_Impl {}
impl ::windows_core::Iids for IDebugPropertyEnumType_Arguments {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugPropertyEnumType_All);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugPropertyEnumType_Arguments_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugPropertyEnumType_Arguments {
    const VTABLE: Self::Vtable = { IDebugPropertyEnumType_Arguments_Vtbl { base__: <IDebugPropertyEnumType_All as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDebugPropertyEnumType_Locals_Impl: ::windows_core::BaseImpl + IDebugPropertyEnumType_All_Impl {}
impl ::windows_core::Iids for IDebugPropertyEnumType_Locals {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugPropertyEnumType_All);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugPropertyEnumType_Locals_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugPropertyEnumType_Locals {
    const VTABLE: Self::Vtable = { IDebugPropertyEnumType_Locals_Vtbl { base__: <IDebugPropertyEnumType_All as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDebugPropertyEnumType_LocalsPlusArgs_Impl: ::windows_core::BaseImpl + IDebugPropertyEnumType_All_Impl {}
impl ::windows_core::Iids for IDebugPropertyEnumType_LocalsPlusArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugPropertyEnumType_All);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugPropertyEnumType_LocalsPlusArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugPropertyEnumType_LocalsPlusArgs {
    const VTABLE: Self::Vtable = { IDebugPropertyEnumType_LocalsPlusArgs_Vtbl { base__: <IDebugPropertyEnumType_All as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDebugPropertyEnumType_Registers_Impl: ::windows_core::BaseImpl + IDebugPropertyEnumType_All_Impl {}
impl ::windows_core::Iids for IDebugPropertyEnumType_Registers {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugPropertyEnumType_All);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugPropertyEnumType_Registers_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugPropertyEnumType_Registers {
    const VTABLE: Self::Vtable = { IDebugPropertyEnumType_Registers_Vtbl { base__: <IDebugPropertyEnumType_All as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumDebugExtendedPropertyInfo_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDebugExtendedPropertyInfo>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumDebugExtendedPropertyInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDebugExtendedPropertyInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgextendedpropertyinfo), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pedpe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pedpe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDebugExtendedPropertyInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumDebugPropertyInfo_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDebugPropertyInfo>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumDebugPropertyInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDebugPropertyInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pi), ::core::mem::transmute_copy(&pceltsfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppepi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppepi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDebugPropertyInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IObjectSafety_Impl: ::windows_core::BaseImpl {
    fn GetInterfaceSafetyOptions(this: &Self::This, riid: *const ::windows_core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> ::windows_core::Result<()>;
    fn SetInterfaceSafetyOptions(this: &Self::This, riid: *const ::windows_core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectSafety {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectSafety_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectSafety {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInterfaceSafetyOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectSafety_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterfaceSafetyOptions(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdwsupportedoptions), ::core::mem::transmute_copy(&pdwenabledoptions)).into())
        }
        unsafe extern "system" fn SetInterfaceSafetyOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectSafety_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterfaceSafetyOptions(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwoptionsetmask), ::core::mem::transmute_copy(&dwenabledoptions)).into())
        }
        IObjectSafety_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInterfaceSafetyOptions: GetInterfaceSafetyOptions::<Identity, Impl, OFFSET>,
            SetInterfaceSafetyOptions: SetInterfaceSafetyOptions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IPerPropertyBrowsing2_Impl: ::windows_core::BaseImpl {
    fn GetDisplayString(this: &Self::This, dispid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MapPropertyToPage(this: &Self::This, dispid: i32) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetPredefinedStrings(this: &Self::This, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> ::windows_core::Result<()>;
    fn SetPredefinedValue(this: &Self::This, dispid: i32, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::Iids for IPerPropertyBrowsing2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Ole")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPerPropertyBrowsing2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDisplayString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayString(this, ::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsidproppage: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapPropertyToPage(this, ::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsidproppage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPredefinedStrings(this, ::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&pcastrings), ::core::mem::transmute_copy(&pcacookies)).into())
        }
        unsafe extern "system" fn SetPredefinedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPredefinedValue(this, ::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&dwcookie)).into())
        }
        IPerPropertyBrowsing2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDisplayString: GetDisplayString::<Identity, Impl, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, Impl, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, Impl, OFFSET>,
            SetPredefinedValue: SetPredefinedValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
