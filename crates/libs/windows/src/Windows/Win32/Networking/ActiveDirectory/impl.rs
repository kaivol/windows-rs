#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADs_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Class(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GUID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ADsPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Schema(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInfo(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetInfo(this: &Self::This) -> ::windows_core::Result<()>;
    fn Get(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Put(this: &Self::This, bstrname: &::windows_core::BSTR, vprop: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetEx(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PutEx(this: &Self::This, lncontrolcode: i32, bstrname: &::windows_core::BSTR, vprop: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetInfoEx(this: &Self::This, vproperties: &super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Class<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Class(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADsPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Schema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Schema(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfo(this).into())
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfo(this).into())
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Get(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Put<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Put(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&vprop)).into())
        }
        unsafe extern "system" fn GetEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEx(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutEx(this, ::core::mem::transmute_copy(&lncontrolcode), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&vprop)).into())
        }
        unsafe extern "system" fn GetInfoEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfoEx(this, ::core::mem::transmute(&vproperties), ::core::mem::transmute_copy(&lnreserved)).into())
        }
        IADs_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Class: Class::<Identity, Impl, OFFSET>,
            GUID: GUID::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Schema: Schema::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
            PutEx: PutEx::<Identity, Impl, OFFSET>,
            GetInfoEx: GetInfoEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsADSystemInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn UserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ComputerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SiteName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainShortName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainDNSName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ForestDNSName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PDCRoleOwner(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SchemaRoleOwner(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsNativeMode(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAnyDCName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDCSiteName(this: &Self::This, szserver: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RefreshSchemaCache(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetTrees(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsADSystemInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsADSystemInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SiteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SiteName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainShortName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainShortName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainDNSName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainDNSName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ForestDNSName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ForestDNSName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PDCRoleOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PDCRoleOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SchemaRoleOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SchemaRoleOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsNativeMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsNativeMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAnyDCName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdcname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAnyDCName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdcname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDCSiteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, pszsitename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDCSiteName(this, ::core::mem::transmute(&szserver)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszsitename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RefreshSchemaCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshSchemaCache(this).into())
        }
        unsafe extern "system" fn GetTrees<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsADSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvtrees: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTrees(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvtrees, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsADSystemInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            SiteName: SiteName::<Identity, Impl, OFFSET>,
            DomainShortName: DomainShortName::<Identity, Impl, OFFSET>,
            DomainDNSName: DomainDNSName::<Identity, Impl, OFFSET>,
            ForestDNSName: ForestDNSName::<Identity, Impl, OFFSET>,
            PDCRoleOwner: PDCRoleOwner::<Identity, Impl, OFFSET>,
            SchemaRoleOwner: SchemaRoleOwner::<Identity, Impl, OFFSET>,
            IsNativeMode: IsNativeMode::<Identity, Impl, OFFSET>,
            GetAnyDCName: GetAnyDCName::<Identity, Impl, OFFSET>,
            GetDCSiteName: GetDCSiteName::<Identity, Impl, OFFSET>,
            RefreshSchemaCache: RefreshSchemaCache::<Identity, Impl, OFFSET>,
            GetTrees: GetTrees::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsAccessControlEntry_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AccessMask(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAccessMask(this: &Self::This, lnaccessmask: i32) -> ::windows_core::Result<()>;
    fn AceType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAceType(this: &Self::This, lnacetype: i32) -> ::windows_core::Result<()>;
    fn AceFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAceFlags(this: &Self::This, lnaceflags: i32) -> ::windows_core::Result<()>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetFlags(this: &Self::This, lnflags: i32) -> ::windows_core::Result<()>;
    fn ObjectType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectType(this: &Self::This, bstrobjecttype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InheritedObjectType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetInheritedObjectType(this: &Self::This, bstrinheritedobjecttype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Trustee(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTrustee(this: &Self::This, bstrtrustee: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsAccessControlEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsAccessControlEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccessMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccessMask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccessMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnaccessmask: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccessMask(this, ::core::mem::transmute_copy(&lnaccessmask)).into())
        }
        unsafe extern "system" fn AceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AceType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnacetype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAceType(this, ::core::mem::transmute_copy(&lnacetype)).into())
        }
        unsafe extern "system" fn AceFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AceFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAceFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnaceflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAceFlags(this, ::core::mem::transmute_copy(&lnaceflags)).into())
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&lnflags)).into())
        }
        unsafe extern "system" fn ObjectType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetObjectType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrobjecttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectType(this, ::core::mem::transmute(&bstrobjecttype)).into())
        }
        unsafe extern "system" fn InheritedObjectType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InheritedObjectType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInheritedObjectType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinheritedobjecttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInheritedObjectType(this, ::core::mem::transmute(&bstrinheritedobjecttype)).into())
        }
        unsafe extern "system" fn Trustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Trustee(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTrustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrustee(this, ::core::mem::transmute(&bstrtrustee)).into())
        }
        IADsAccessControlEntry_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AccessMask: AccessMask::<Identity, Impl, OFFSET>,
            SetAccessMask: SetAccessMask::<Identity, Impl, OFFSET>,
            AceType: AceType::<Identity, Impl, OFFSET>,
            SetAceType: SetAceType::<Identity, Impl, OFFSET>,
            AceFlags: AceFlags::<Identity, Impl, OFFSET>,
            SetAceFlags: SetAceFlags::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            ObjectType: ObjectType::<Identity, Impl, OFFSET>,
            SetObjectType: SetObjectType::<Identity, Impl, OFFSET>,
            InheritedObjectType: InheritedObjectType::<Identity, Impl, OFFSET>,
            SetInheritedObjectType: SetInheritedObjectType::<Identity, Impl, OFFSET>,
            Trustee: Trustee::<Identity, Impl, OFFSET>,
            SetTrustee: SetTrustee::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsAccessControlList_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AclRevision(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAclRevision(this: &Self::This, lnaclrevision: i32) -> ::windows_core::Result<()>;
    fn AceCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAceCount(this: &Self::This, lnacecount: i32) -> ::windows_core::Result<()>;
    fn AddAce(this: &Self::This, paccesscontrolentry: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn RemoveAce(this: &Self::This, paccesscontrolentry: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn CopyAccessList(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsAccessControlList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsAccessControlList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AclRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AclRevision(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAclRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnaclrevision: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAclRevision(this, ::core::mem::transmute_copy(&lnaclrevision)).into())
        }
        unsafe extern "system" fn AceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AceCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnacecount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAceCount(this, ::core::mem::transmute_copy(&lnacecount)).into())
        }
        unsafe extern "system" fn AddAce<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAce(this, ::windows_core::from_raw_borrowed(&paccesscontrolentry)).into())
        }
        unsafe extern "system" fn RemoveAce<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAce(this, ::windows_core::from_raw_borrowed(&paccesscontrolentry)).into())
        }
        unsafe extern "system" fn CopyAccessList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaccesscontrollist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyAccessList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccesscontrollist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAccessControlList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsAccessControlList_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AclRevision: AclRevision::<Identity, Impl, OFFSET>,
            SetAclRevision: SetAclRevision::<Identity, Impl, OFFSET>,
            AceCount: AceCount::<Identity, Impl, OFFSET>,
            SetAceCount: SetAceCount::<Identity, Impl, OFFSET>,
            AddAce: AddAce::<Identity, Impl, OFFSET>,
            RemoveAce: RemoveAce::<Identity, Impl, OFFSET>,
            CopyAccessList: CopyAccessList::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsAcl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ProtectedAttrName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProtectedAttrName(this: &Self::This, bstrprotectedattrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SubjectName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubjectName(this: &Self::This, bstrsubjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Privileges(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPrivileges(this: &Self::This, lnprivileges: i32) -> ::windows_core::Result<()>;
    fn CopyAcl(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsAcl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsAcl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProtectedAttrName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProtectedAttrName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProtectedAttrName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotectedattrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtectedAttrName(this, ::core::mem::transmute(&bstrprotectedattrname)).into())
        }
        unsafe extern "system" fn SubjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubjectName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsubjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubjectName(this, ::core::mem::transmute(&bstrsubjectname)).into())
        }
        unsafe extern "system" fn Privileges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Privileges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivileges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnprivileges: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivileges(this, ::core::mem::transmute_copy(&lnprivileges)).into())
        }
        unsafe extern "system" fn CopyAcl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAcl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppacl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyAcl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppacl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsAcl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProtectedAttrName: ProtectedAttrName::<Identity, Impl, OFFSET>,
            SetProtectedAttrName: SetProtectedAttrName::<Identity, Impl, OFFSET>,
            SubjectName: SubjectName::<Identity, Impl, OFFSET>,
            SetSubjectName: SetSubjectName::<Identity, Impl, OFFSET>,
            Privileges: Privileges::<Identity, Impl, OFFSET>,
            SetPrivileges: SetPrivileges::<Identity, Impl, OFFSET>,
            CopyAcl: CopyAcl::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IADsAggregatee_Impl: ::windows_core::BaseImpl {
    fn ConnectAsAggregatee(this: &Self::This, pouterunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DisconnectAsAggregatee(this: &Self::This) -> ::windows_core::Result<()>;
    fn RelinquishInterface(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreInterface(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IADsAggregatee {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsAggregatee {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectAsAggregatee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pouterunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectAsAggregatee(this, ::windows_core::from_raw_borrowed(&pouterunknown)).into())
        }
        unsafe extern "system" fn DisconnectAsAggregatee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectAsAggregatee(this).into())
        }
        unsafe extern "system" fn RelinquishInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RelinquishInterface(this, ::core::mem::transmute_copy(&riid)).into())
        }
        unsafe extern "system" fn RestoreInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregatee_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreInterface(this, ::core::mem::transmute_copy(&riid)).into())
        }
        IADsAggregatee_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectAsAggregatee: ConnectAsAggregatee::<Identity, Impl, OFFSET>,
            DisconnectAsAggregatee: DisconnectAsAggregatee::<Identity, Impl, OFFSET>,
            RelinquishInterface: RelinquishInterface::<Identity, Impl, OFFSET>,
            RestoreInterface: RestoreInterface::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IADsAggregator_Impl: ::windows_core::BaseImpl {
    fn ConnectAsAggregator(this: &Self::This, paggregatee: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DisconnectAsAggregator(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IADsAggregator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsAggregator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectAsAggregator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregatee: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectAsAggregator(this, ::windows_core::from_raw_borrowed(&paggregatee)).into())
        }
        unsafe extern "system" fn DisconnectAsAggregator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsAggregator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectAsAggregator(this).into())
        }
        IADsAggregator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectAsAggregator: ConnectAsAggregator::<Identity, Impl, OFFSET>,
            DisconnectAsAggregator: DisconnectAsAggregator::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsBackLink_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RemoteID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRemoteID(this: &Self::This, lnremoteid: i32) -> ::windows_core::Result<()>;
    fn ObjectName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectName(this: &Self::This, bstrobjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsBackLink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsBackLink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RemoteID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnremoteid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteID(this, ::core::mem::transmute_copy(&lnremoteid)).into())
        }
        unsafe extern "system" fn ObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsBackLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectName(this, ::core::mem::transmute(&bstrobjectname)).into())
        }
        IADsBackLink_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RemoteID: RemoteID::<Identity, Impl, OFFSET>,
            SetRemoteID: SetRemoteID::<Identity, Impl, OFFSET>,
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsCaseIgnoreList_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CaseIgnoreList(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetCaseIgnoreList(this: &Self::This, vcaseignorelist: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsCaseIgnoreList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsCaseIgnoreList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CaseIgnoreList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CaseIgnoreList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCaseIgnoreList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCaseIgnoreList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcaseignorelist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCaseIgnoreList(this, ::core::mem::transmute(&vcaseignorelist)).into())
        }
        IADsCaseIgnoreList_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CaseIgnoreList: CaseIgnoreList::<Identity, Impl, OFFSET>,
            SetCaseIgnoreList: SetCaseIgnoreList::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsClass_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn PrimaryInterface(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCLSID(this: &Self::This, bstrclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOID(this: &Self::This, bstroid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Abstract(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAbstract(this: &Self::This, fabstract: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Auxiliary(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAuxiliary(this: &Self::This, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn MandatoryProperties(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetMandatoryProperties(this: &Self::This, vmandatoryproperties: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn OptionalProperties(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOptionalProperties(this: &Self::This, voptionalproperties: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn NamingProperties(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetNamingProperties(this: &Self::This, vnamingproperties: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DerivedFrom(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDerivedFrom(this: &Self::This, vderivedfrom: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AuxDerivedFrom(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetAuxDerivedFrom(this: &Self::This, vauxderivedfrom: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PossibleSuperiors(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPossibleSuperiors(this: &Self::This, vpossiblesuperiors: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Containment(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetContainment(this: &Self::This, vcontainment: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Container(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetContainer(this: &Self::This, fcontainer: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn HelpFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHelpFileName(this: &Self::This, bstrhelpfilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HelpFileContext(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHelpFileContext(this: &Self::This, lnhelpfilecontext: i32) -> ::windows_core::Result<()>;
    fn Qualifiers(this: &Self::This) -> ::windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsClass {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsClass {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrimaryInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrimaryInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCLSID(this, ::core::mem::transmute(&bstrclsid)).into())
        }
        unsafe extern "system" fn OID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOID(this, ::core::mem::transmute(&bstroid)).into())
        }
        unsafe extern "system" fn Abstract<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Abstract(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAbstract<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fabstract: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAbstract(this, ::core::mem::transmute_copy(&fabstract)).into())
        }
        unsafe extern "system" fn Auxiliary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Auxiliary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuxiliary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuxiliary(this, ::core::mem::transmute_copy(&fauxiliary)).into())
        }
        unsafe extern "system" fn MandatoryProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MandatoryProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMandatoryProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vmandatoryproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMandatoryProperties(this, ::core::mem::transmute(&vmandatoryproperties)).into())
        }
        unsafe extern "system" fn OptionalProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OptionalProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOptionalProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voptionalproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOptionalProperties(this, ::core::mem::transmute(&voptionalproperties)).into())
        }
        unsafe extern "system" fn NamingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamingProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamingProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vnamingproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamingProperties(this, ::core::mem::transmute(&vnamingproperties)).into())
        }
        unsafe extern "system" fn DerivedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DerivedFrom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDerivedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDerivedFrom(this, ::core::mem::transmute(&vderivedfrom)).into())
        }
        unsafe extern "system" fn AuxDerivedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuxDerivedFrom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuxDerivedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vauxderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuxDerivedFrom(this, ::core::mem::transmute(&vauxderivedfrom)).into())
        }
        unsafe extern "system" fn PossibleSuperiors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PossibleSuperiors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPossibleSuperiors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vpossiblesuperiors: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPossibleSuperiors(this, ::core::mem::transmute(&vpossiblesuperiors)).into())
        }
        unsafe extern "system" fn Containment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Containment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContainment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcontainment: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContainment(this, ::core::mem::transmute(&vcontainment)).into())
        }
        unsafe extern "system" fn Container<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Container(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcontainer: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContainer(this, ::core::mem::transmute_copy(&fcontainer)).into())
        }
        unsafe extern "system" fn HelpFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HelpFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhelpfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpFileName(this, ::core::mem::transmute(&bstrhelpfilename)).into())
        }
        unsafe extern "system" fn HelpFileContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HelpFileContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHelpFileContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnhelpfilecontext: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpFileContext(this, ::core::mem::transmute_copy(&lnhelpfilecontext)).into())
        }
        unsafe extern "system" fn Qualifiers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Qualifiers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualifiers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsClass_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrimaryInterface: PrimaryInterface::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Abstract: Abstract::<Identity, Impl, OFFSET>,
            SetAbstract: SetAbstract::<Identity, Impl, OFFSET>,
            Auxiliary: Auxiliary::<Identity, Impl, OFFSET>,
            SetAuxiliary: SetAuxiliary::<Identity, Impl, OFFSET>,
            MandatoryProperties: MandatoryProperties::<Identity, Impl, OFFSET>,
            SetMandatoryProperties: SetMandatoryProperties::<Identity, Impl, OFFSET>,
            OptionalProperties: OptionalProperties::<Identity, Impl, OFFSET>,
            SetOptionalProperties: SetOptionalProperties::<Identity, Impl, OFFSET>,
            NamingProperties: NamingProperties::<Identity, Impl, OFFSET>,
            SetNamingProperties: SetNamingProperties::<Identity, Impl, OFFSET>,
            DerivedFrom: DerivedFrom::<Identity, Impl, OFFSET>,
            SetDerivedFrom: SetDerivedFrom::<Identity, Impl, OFFSET>,
            AuxDerivedFrom: AuxDerivedFrom::<Identity, Impl, OFFSET>,
            SetAuxDerivedFrom: SetAuxDerivedFrom::<Identity, Impl, OFFSET>,
            PossibleSuperiors: PossibleSuperiors::<Identity, Impl, OFFSET>,
            SetPossibleSuperiors: SetPossibleSuperiors::<Identity, Impl, OFFSET>,
            Containment: Containment::<Identity, Impl, OFFSET>,
            SetContainment: SetContainment::<Identity, Impl, OFFSET>,
            Container: Container::<Identity, Impl, OFFSET>,
            SetContainer: SetContainer::<Identity, Impl, OFFSET>,
            HelpFileName: HelpFileName::<Identity, Impl, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, Impl, OFFSET>,
            HelpFileContext: HelpFileContext::<Identity, Impl, OFFSET>,
            SetHelpFileContext: SetHelpFileContext::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Add(this: &Self::This, bstrname: &::windows_core::BSTR, vitem: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, bstritemtoberemoved: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetObject(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vitem: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&vitem)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&bstritemtoberemoved)).into())
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvitem: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsComputer_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn ComputerID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Site(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Location(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocation(this: &Self::This, bstrlocation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrimaryUser(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrimaryUser(this: &Self::This, bstrprimaryuser: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Owner(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOwner(this: &Self::This, bstrowner: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Division(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDivision(this: &Self::This, bstrdivision: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Department(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDepartment(this: &Self::This, bstrdepartment: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Role(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRole(this: &Self::This, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OperatingSystem(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOperatingSystem(this: &Self::This, bstroperatingsystem: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OperatingSystemVersion(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOperatingSystemVersion(this: &Self::This, bstroperatingsystemversion: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Model(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetModel(this: &Self::This, bstrmodel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Processor(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProcessor(this: &Self::This, bstrprocessor: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ProcessorCount(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProcessorCount(this: &Self::This, bstrprocessorcount: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MemorySize(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetMemorySize(this: &Self::This, bstrmemorysize: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StorageCapacity(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStorageCapacity(this: &Self::This, bstrstoragecapacity: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NetAddresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetNetAddresses(this: &Self::This, vnetaddresses: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsComputer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsComputer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ComputerID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputerID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Site<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Site(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn Location<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Location(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocation(this, ::core::mem::transmute(&bstrlocation)).into())
        }
        unsafe extern "system" fn PrimaryUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrimaryUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrimaryUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprimaryuser: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrimaryUser(this, ::core::mem::transmute(&bstrprimaryuser)).into())
        }
        unsafe extern "system" fn Owner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Owner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOwner(this, ::core::mem::transmute(&bstrowner)).into())
        }
        unsafe extern "system" fn Division<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Division(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDivision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdivision: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDivision(this, ::core::mem::transmute(&bstrdivision)).into())
        }
        unsafe extern "system" fn Department<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Department(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDepartment(this, ::core::mem::transmute(&bstrdepartment)).into())
        }
        unsafe extern "system" fn Role<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Role(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRole(this, ::core::mem::transmute(&bstrrole)).into())
        }
        unsafe extern "system" fn OperatingSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OperatingSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOperatingSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroperatingsystem: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOperatingSystem(this, ::core::mem::transmute(&bstroperatingsystem)).into())
        }
        unsafe extern "system" fn OperatingSystemVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OperatingSystemVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOperatingSystemVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroperatingsystemversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOperatingSystemVersion(this, ::core::mem::transmute(&bstroperatingsystemversion)).into())
        }
        unsafe extern "system" fn Model<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Model(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmodel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModel(this, ::core::mem::transmute(&bstrmodel)).into())
        }
        unsafe extern "system" fn Processor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Processor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProcessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprocessor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProcessor(this, ::core::mem::transmute(&bstrprocessor)).into())
        }
        unsafe extern "system" fn ProcessorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProcessorCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProcessorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprocessorcount: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProcessorCount(this, ::core::mem::transmute(&bstrprocessorcount)).into())
        }
        unsafe extern "system" fn MemorySize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MemorySize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMemorySize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmemorysize: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMemorySize(this, ::core::mem::transmute(&bstrmemorysize)).into())
        }
        unsafe extern "system" fn StorageCapacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StorageCapacity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStorageCapacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrstoragecapacity: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStorageCapacity(this, ::core::mem::transmute(&bstrstoragecapacity)).into())
        }
        unsafe extern "system" fn NetAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetAddresses(this, ::core::mem::transmute(&vnetaddresses)).into())
        }
        IADsComputer_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ComputerID: ComputerID::<Identity, Impl, OFFSET>,
            Site: Site::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            PrimaryUser: PrimaryUser::<Identity, Impl, OFFSET>,
            SetPrimaryUser: SetPrimaryUser::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
            SetRole: SetRole::<Identity, Impl, OFFSET>,
            OperatingSystem: OperatingSystem::<Identity, Impl, OFFSET>,
            SetOperatingSystem: SetOperatingSystem::<Identity, Impl, OFFSET>,
            OperatingSystemVersion: OperatingSystemVersion::<Identity, Impl, OFFSET>,
            SetOperatingSystemVersion: SetOperatingSystemVersion::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Processor: Processor::<Identity, Impl, OFFSET>,
            SetProcessor: SetProcessor::<Identity, Impl, OFFSET>,
            ProcessorCount: ProcessorCount::<Identity, Impl, OFFSET>,
            SetProcessorCount: SetProcessorCount::<Identity, Impl, OFFSET>,
            MemorySize: MemorySize::<Identity, Impl, OFFSET>,
            SetMemorySize: SetMemorySize::<Identity, Impl, OFFSET>,
            StorageCapacity: StorageCapacity::<Identity, Impl, OFFSET>,
            SetStorageCapacity: SetStorageCapacity::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsComputerOperations_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Shutdown(this: &Self::This, breboot: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsComputerOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsComputerOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsComputerOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, breboot: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this, ::core::mem::transmute_copy(&breboot)).into())
        }
        IADsComputerOperations_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsContainer_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Filter(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFilter(this: &Self::This, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Hints(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetHints(this: &Self::This, vhints: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetObject(this: &Self::This, classname: &::windows_core::BSTR, relativename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Create(this: &Self::This, classname: &::windows_core::BSTR, relativename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Delete(this: &Self::This, bstrclassname: &::windows_core::BSTR, bstrrelativename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CopyHere(this: &Self::This, sourcename: &::windows_core::BSTR, newname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn MoveHere(this: &Self::This, sourcename: &::windows_core::BSTR, newname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Filter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Filter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilter(this, ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn Hints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vhints: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHints(this, ::core::mem::transmute(&vhints)).into())
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classname: ::std::mem::MaybeUninit<::windows_core::BSTR>, relativename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this, ::core::mem::transmute(&classname), ::core::mem::transmute(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classname: ::std::mem::MaybeUninit<::windows_core::BSTR>, relativename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute(&classname), ::core::mem::transmute(&relativename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclassname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrelativename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&bstrclassname), ::core::mem::transmute(&bstrrelativename)).into())
        }
        unsafe extern "system" fn CopyHere<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyHere(this, ::core::mem::transmute(&sourcename), ::core::mem::transmute(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveHere<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveHere(this, ::core::mem::transmute(&sourcename), ::core::mem::transmute(&newname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsContainer_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
            Hints: Hints::<Identity, Impl, OFFSET>,
            SetHints: SetHints::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            CopyHere: CopyHere::<Identity, Impl, OFFSET>,
            MoveHere: MoveHere::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDNWithBinary_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn BinaryValue(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetBinaryValue(this: &Self::This, vbinaryvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DNString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDNString(this: &Self::This, bstrdnstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsDNWithBinary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsDNWithBinary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BinaryValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BinaryValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBinaryValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbinaryvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBinaryValue(this, ::core::mem::transmute(&vbinaryvalue)).into())
        }
        unsafe extern "system" fn DNString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DNString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithBinary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDNString(this, ::core::mem::transmute(&bstrdnstring)).into())
        }
        IADsDNWithBinary_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BinaryValue: BinaryValue::<Identity, Impl, OFFSET>,
            SetBinaryValue: SetBinaryValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDNWithString_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StringValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStringValue(this: &Self::This, bstrstringvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DNString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDNString(this: &Self::This, bstrdnstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsDNWithString {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsDNWithString {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StringValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrstringvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStringValue(this, ::core::mem::transmute(&bstrstringvalue)).into())
        }
        unsafe extern "system" fn DNString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DNString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDNWithString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDNString(this, ::core::mem::transmute(&bstrdnstring)).into())
        }
        IADsDNWithString_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StringValue: StringValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDeleteOps_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DeleteObject(this: &Self::This, lnflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsDeleteOps {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDeleteOps_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsDeleteOps {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDeleteOps_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteObject(this, ::core::mem::transmute_copy(&lnflags)).into())
        }
        IADsDeleteOps_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteObject: DeleteObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsDomain_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn IsWorkgroup(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MinPasswordLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMinPasswordLength(this: &Self::This, lnminpasswordlength: i32) -> ::windows_core::Result<()>;
    fn MinPasswordAge(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMinPasswordAge(this: &Self::This, lnminpasswordage: i32) -> ::windows_core::Result<()>;
    fn MaxPasswordAge(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxPasswordAge(this: &Self::This, lnmaxpasswordage: i32) -> ::windows_core::Result<()>;
    fn MaxBadPasswordsAllowed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxBadPasswordsAllowed(this: &Self::This, lnmaxbadpasswordsallowed: i32) -> ::windows_core::Result<()>;
    fn PasswordHistoryLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPasswordHistoryLength(this: &Self::This, lnpasswordhistorylength: i32) -> ::windows_core::Result<()>;
    fn PasswordAttributes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPasswordAttributes(this: &Self::This, lnpasswordattributes: i32) -> ::windows_core::Result<()>;
    fn AutoUnlockInterval(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAutoUnlockInterval(this: &Self::This, lnautounlockinterval: i32) -> ::windows_core::Result<()>;
    fn LockoutObservationInterval(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLockoutObservationInterval(this: &Self::This, lnlockoutobservationinterval: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsDomain {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsDomain {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsWorkgroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWorkgroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinPasswordLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinPasswordLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinPasswordLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnminpasswordlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinPasswordLength(this, ::core::mem::transmute_copy(&lnminpasswordlength)).into())
        }
        unsafe extern "system" fn MinPasswordAge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinPasswordAge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinPasswordAge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnminpasswordage: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinPasswordAge(this, ::core::mem::transmute_copy(&lnminpasswordage)).into())
        }
        unsafe extern "system" fn MaxPasswordAge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxPasswordAge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxPasswordAge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnmaxpasswordage: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxPasswordAge(this, ::core::mem::transmute_copy(&lnmaxpasswordage)).into())
        }
        unsafe extern "system" fn MaxBadPasswordsAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxBadPasswordsAllowed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxBadPasswordsAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxBadPasswordsAllowed(this, ::core::mem::transmute_copy(&lnmaxbadpasswordsallowed)).into())
        }
        unsafe extern "system" fn PasswordHistoryLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasswordHistoryLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPasswordHistoryLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnpasswordhistorylength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPasswordHistoryLength(this, ::core::mem::transmute_copy(&lnpasswordhistorylength)).into())
        }
        unsafe extern "system" fn PasswordAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasswordAttributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPasswordAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnpasswordattributes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPasswordAttributes(this, ::core::mem::transmute_copy(&lnpasswordattributes)).into())
        }
        unsafe extern "system" fn AutoUnlockInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoUnlockInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoUnlockInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnautounlockinterval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoUnlockInterval(this, ::core::mem::transmute_copy(&lnautounlockinterval)).into())
        }
        unsafe extern "system" fn LockoutObservationInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LockoutObservationInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLockoutObservationInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsDomain_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnlockoutobservationinterval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLockoutObservationInterval(this, ::core::mem::transmute_copy(&lnlockoutobservationinterval)).into())
        }
        IADsDomain_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsWorkgroup: IsWorkgroup::<Identity, Impl, OFFSET>,
            MinPasswordLength: MinPasswordLength::<Identity, Impl, OFFSET>,
            SetMinPasswordLength: SetMinPasswordLength::<Identity, Impl, OFFSET>,
            MinPasswordAge: MinPasswordAge::<Identity, Impl, OFFSET>,
            SetMinPasswordAge: SetMinPasswordAge::<Identity, Impl, OFFSET>,
            MaxPasswordAge: MaxPasswordAge::<Identity, Impl, OFFSET>,
            SetMaxPasswordAge: SetMaxPasswordAge::<Identity, Impl, OFFSET>,
            MaxBadPasswordsAllowed: MaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            SetMaxBadPasswordsAllowed: SetMaxBadPasswordsAllowed::<Identity, Impl, OFFSET>,
            PasswordHistoryLength: PasswordHistoryLength::<Identity, Impl, OFFSET>,
            SetPasswordHistoryLength: SetPasswordHistoryLength::<Identity, Impl, OFFSET>,
            PasswordAttributes: PasswordAttributes::<Identity, Impl, OFFSET>,
            SetPasswordAttributes: SetPasswordAttributes::<Identity, Impl, OFFSET>,
            AutoUnlockInterval: AutoUnlockInterval::<Identity, Impl, OFFSET>,
            SetAutoUnlockInterval: SetAutoUnlockInterval::<Identity, Impl, OFFSET>,
            LockoutObservationInterval: LockoutObservationInterval::<Identity, Impl, OFFSET>,
            SetLockoutObservationInterval: SetLockoutObservationInterval::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsEmail_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetType(this: &Self::This, lntype: i32) -> ::windows_core::Result<()>;
    fn Address(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAddress(this: &Self::This, bstraddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsEmail {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsEmail {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&lntype)).into())
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsEmail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAddress(this, ::core::mem::transmute(&bstraddress)).into())
        }
        IADsEmail_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsExtension_Impl: ::windows_core::BaseImpl {
    fn Operate(this: &Self::This, dwcode: u32, vardata1: &super::super::System::Variant::VARIANT, vardata2: &super::super::System::Variant::VARIANT, vardata3: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PrivateGetIDsOfNames(this: &Self::This, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows_core::Result<i32>;
    fn PrivateInvoke(this: &Self::This, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsExtension {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsExtension {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Operate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcode: u32, vardata1: super::super::System::Variant::VARIANT, vardata2: super::super::System::Variant::VARIANT, vardata3: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Operate(this, ::core::mem::transmute_copy(&dwcode), ::core::mem::transmute(&vardata1), ::core::mem::transmute(&vardata2), ::core::mem::transmute(&vardata3)).into())
        }
        unsafe extern "system" fn PrivateGetIDsOfNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateGetIDsOfNames(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgdispid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateInvoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrivateInvoke(this, ::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into())
        }
        IADsExtension_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Operate: Operate::<Identity, Impl, OFFSET>,
            PrivateGetIDsOfNames: PrivateGetIDsOfNames::<Identity, Impl, OFFSET>,
            PrivateInvoke: PrivateInvoke::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFaxNumber_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn TelephoneNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTelephoneNumber(this: &Self::This, bstrtelephonenumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Parameters(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetParameters(this: &Self::This, vparameters: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsFaxNumber {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsFaxNumber {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephoneNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTelephoneNumber(this, ::core::mem::transmute(&bstrtelephonenumber)).into())
        }
        unsafe extern "system" fn Parameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFaxNumber_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vparameters: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute(&vparameters)).into())
        }
        IADsFaxNumber_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFileService_Impl: ::windows_core::BaseImpl + IADsService_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxUserCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxUserCount(this: &Self::This, lnmaxusercount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsFileService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADsService);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsFileService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn MaxUserCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxUserCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxUserCount(this, ::core::mem::transmute_copy(&lnmaxusercount)).into())
        }
        IADsFileService_Vtbl {
            base__: <IADsService as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFileServiceOperations_Impl: ::windows_core::BaseImpl + IADsServiceOperations_Impl {
    fn Sessions(this: &Self::This) -> ::windows_core::Result<IADsCollection>;
    fn Resources(this: &Self::This) -> ::windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsFileServiceOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADsServiceOperations);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsFileServiceOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Sessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsessions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sessions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsessions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Resources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsFileServiceOperations_Vtbl {
            base__: <IADsServiceOperations as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Sessions: Sessions::<Identity, Impl, OFFSET>,
            Resources: Resources::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsFileShare_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn CurrentUserCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HostComputer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHostComputer(this: &Self::This, bstrhostcomputer: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPath(this: &Self::This, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxUserCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxUserCount(this: &Self::This, lnmaxusercount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsFileShare {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsFileShare {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentUserCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentUserCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn HostComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HostComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHostComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHostComputer(this, ::core::mem::transmute(&bstrhostcomputer)).into())
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&bstrpath)).into())
        }
        unsafe extern "system" fn MaxUserCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxUserCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxUserCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsFileShare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxUserCount(this, ::core::mem::transmute_copy(&lnmaxusercount)).into())
        }
        IADsFileShare_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentUserCount: CurrentUserCount::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            MaxUserCount: MaxUserCount::<Identity, Impl, OFFSET>,
            SetMaxUserCount: SetMaxUserCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsGroup_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Members(this: &Self::This) -> ::windows_core::Result<IADsMembers>;
    fn IsMember(this: &Self::This, bstrmember: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(this: &Self::This, bstrnewitem: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, bstritemtoberemoved: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmembers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Members(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmembers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmember: ::std::mem::MaybeUninit<::windows_core::BSTR>, bmember: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMember(this, ::core::mem::transmute(&bstrmember)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bmember, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnewitem: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute(&bstrnewitem)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&bstritemtoberemoved)).into())
        }
        IADsGroup_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            IsMember: IsMember::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsHold_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectName(this: &Self::This, bstrobjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Amount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAmount(this: &Self::This, lnamount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsHold {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsHold {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectName(this, ::core::mem::transmute(&bstrobjectname)).into())
        }
        unsafe extern "system" fn Amount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Amount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAmount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsHold_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnamount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAmount(this, ::core::mem::transmute_copy(&lnamount)).into())
        }
        IADsHold_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Amount: Amount::<Identity, Impl, OFFSET>,
            SetAmount: SetAmount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsLargeInteger_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn HighPart(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetHighPart(this: &Self::This, lnhighpart: i32) -> ::windows_core::Result<()>;
    fn LowPart(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLowPart(this: &Self::This, lnlowpart: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsLargeInteger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsLargeInteger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HighPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HighPart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHighPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnhighpart: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHighPart(this, ::core::mem::transmute_copy(&lnhighpart)).into())
        }
        unsafe extern "system" fn LowPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LowPart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLowPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLargeInteger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnlowpart: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLowPart(this, ::core::mem::transmute_copy(&lnlowpart)).into())
        }
        IADsLargeInteger_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HighPart: HighPart::<Identity, Impl, OFFSET>,
            SetHighPart: SetHighPart::<Identity, Impl, OFFSET>,
            LowPart: LowPart::<Identity, Impl, OFFSET>,
            SetLowPart: SetLowPart::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsLocality_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalityName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalityName(this: &Self::This, bstrlocalityname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PostalAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPostalAddress(this: &Self::This, bstrpostaladdress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SeeAlso(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(this: &Self::This, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsLocality {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsLocality {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalityName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalityName(this, ::core::mem::transmute(&bstrlocalityname)).into())
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostalAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostalAddress(this, ::core::mem::transmute(&bstrpostaladdress)).into())
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SeeAlso(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsLocality_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSeeAlso(this, ::core::mem::transmute(&vseealso)).into())
        }
        IADsLocality_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsMembers_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Filter(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFilter(this: &Self::This, pvfilter: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsMembers {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsMembers {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Filter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Filter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsMembers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvfilter: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilter(this, ::core::mem::transmute(&pvfilter)).into())
        }
        IADsMembers_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Filter: Filter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsNameTranslate_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetChaseReferral(this: &Self::This, lnchasereferral: i32) -> ::windows_core::Result<()>;
    fn Init(this: &Self::This, lnsettype: i32, bstradspath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InitEx(this: &Self::This, lnsettype: i32, bstradspath: &::windows_core::BSTR, bstruserid: &::windows_core::BSTR, bstrdomain: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Set(this: &Self::This, lnsettype: i32, bstradspath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Get(this: &Self::This, lnformattype: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEx(this: &Self::This, lnformattype: i32, pvar: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetEx(this: &Self::This, lnformattype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsNameTranslate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsNameTranslate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetChaseReferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnchasereferral: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChaseReferral(this, ::core::mem::transmute_copy(&lnchasereferral)).into())
        }
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute(&bstradspath)).into())
        }
        unsafe extern "system" fn InitEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruserid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitEx(this, ::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute(&bstradspath), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrdomain), ::core::mem::transmute(&bstrpassword)).into())
        }
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::core::mem::transmute_copy(&lnsettype), ::core::mem::transmute(&bstradspath)).into())
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Get(this, ::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEx(this, ::core::mem::transmute_copy(&lnformattype), ::core::mem::transmute(&pvar)).into())
        }
        unsafe extern "system" fn GetEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNameTranslate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEx(this, ::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsNameTranslate_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetChaseReferral: SetChaseReferral::<Identity, Impl, OFFSET>,
            Init: Init::<Identity, Impl, OFFSET>,
            InitEx: InitEx::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            SetEx: SetEx::<Identity, Impl, OFFSET>,
            GetEx: GetEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsNamespaces_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn DefaultContainer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDefaultContainer(this: &Self::This, bstrdefaultcontainer: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsNamespaces {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsNamespaces {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DefaultContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNamespaces_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdefaultcontainer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultContainer(this, ::core::mem::transmute(&bstrdefaultcontainer)).into())
        }
        IADsNamespaces_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DefaultContainer: DefaultContainer::<Identity, Impl, OFFSET>,
            SetDefaultContainer: SetDefaultContainer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsNetAddress_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AddressType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAddressType(this: &Self::This, lnaddresstype: i32) -> ::windows_core::Result<()>;
    fn Address(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetAddress(this: &Self::This, vaddress: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsNetAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsNetAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddressType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddressType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAddressType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnaddresstype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAddressType(this, ::core::mem::transmute_copy(&lnaddresstype)).into())
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsNetAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vaddress: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAddress(this, ::core::mem::transmute(&vaddress)).into())
        }
        IADsNetAddress_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddressType: AddressType::<Identity, Impl, OFFSET>,
            SetAddressType: SetAddressType::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsO_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalityName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalityName(this: &Self::This, bstrlocalityname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PostalAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPostalAddress(this: &Self::This, bstrpostaladdress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TelephoneNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTelephoneNumber(this: &Self::This, bstrtelephonenumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FaxNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFaxNumber(this: &Self::This, bstrfaxnumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SeeAlso(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(this: &Self::This, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsO {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsO {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalityName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalityName(this, ::core::mem::transmute(&bstrlocalityname)).into())
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostalAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostalAddress(this, ::core::mem::transmute(&bstrpostaladdress)).into())
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephoneNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTelephoneNumber(this, ::core::mem::transmute(&bstrtelephonenumber)).into())
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FaxNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFaxNumber(this, ::core::mem::transmute(&bstrfaxnumber)).into())
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SeeAlso(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSeeAlso(this, ::core::mem::transmute(&vseealso)).into())
        }
        IADsO_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsOU_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalityName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalityName(this: &Self::This, bstrlocalityname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PostalAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPostalAddress(this: &Self::This, bstrpostaladdress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TelephoneNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTelephoneNumber(this: &Self::This, bstrtelephonenumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FaxNumber(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFaxNumber(this: &Self::This, bstrfaxnumber: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SeeAlso(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(this: &Self::This, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn BusinessCategory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBusinessCategory(this: &Self::This, bstrbusinesscategory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsOU {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsOU {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn LocalityName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalityName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalityName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalityName(this, ::core::mem::transmute(&bstrlocalityname)).into())
        }
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostalAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostalAddress(this, ::core::mem::transmute(&bstrpostaladdress)).into())
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephoneNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTelephoneNumber(this, ::core::mem::transmute(&bstrtelephonenumber)).into())
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FaxNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFaxNumber(this, ::core::mem::transmute(&bstrfaxnumber)).into())
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SeeAlso(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSeeAlso(this, ::core::mem::transmute(&vseealso)).into())
        }
        unsafe extern "system" fn BusinessCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BusinessCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBusinessCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOU_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbusinesscategory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBusinessCategory(this, ::core::mem::transmute(&bstrbusinesscategory)).into())
        }
        IADsOU_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LocalityName: LocalityName::<Identity, Impl, OFFSET>,
            SetLocalityName: SetLocalityName::<Identity, Impl, OFFSET>,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            BusinessCategory: BusinessCategory::<Identity, Impl, OFFSET>,
            SetBusinessCategory: SetBusinessCategory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsObjectOptions_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetOption(this: &Self::This, lnoption: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOption(this: &Self::This, lnoption: i32, vvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsObjectOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsObjectOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnoption: i32, pvvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOption(this, ::core::mem::transmute_copy(&lnoption)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsObjectOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnoption: i32, vvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOption(this, ::core::mem::transmute_copy(&lnoption), ::core::mem::transmute(&vvalue)).into())
        }
        IADsObjectOptions_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsOctetList_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OctetList(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOctetList(this: &Self::This, voctetlist: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsOctetList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsOctetList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OctetList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OctetList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOctetList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOctetList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voctetlist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOctetList(this, ::core::mem::transmute(&voctetlist)).into())
        }
        IADsOctetList_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OctetList: OctetList::<Identity, Impl, OFFSET>,
            SetOctetList: SetOctetList::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsOpenDSObject_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn OpenDSObject(this: &Self::This, lpszdnname: &::windows_core::BSTR, lpszusername: &::windows_core::BSTR, lpszpassword: &::windows_core::BSTR, lnreserved: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsOpenDSObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOpenDSObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsOpenDSObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenDSObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsOpenDSObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszdnname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnreserved: i32, ppoledsobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenDSObject(this, ::core::mem::transmute(&lpszdnname), ::core::mem::transmute(&lpszusername), ::core::mem::transmute(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoledsobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsOpenDSObject_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenDSObject: OpenDSObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPath_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Type(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetType(this: &Self::This, lntype: i32) -> ::windows_core::Result<()>;
    fn VolumeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVolumeName(this: &Self::This, bstrvolumename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPath(this: &Self::This, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPath {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPath {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute_copy(&lntype)).into())
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VolumeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVolumeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrvolumename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolumeName(this, ::core::mem::transmute(&bstrvolumename)).into())
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&bstrpath)).into())
        }
        IADsPath_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPathname_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Set(this: &Self::This, bstradspath: &::windows_core::BSTR, lnsettype: i32) -> ::windows_core::Result<()>;
    fn SetDisplayType(this: &Self::This, lndisplaytype: i32) -> ::windows_core::Result<()>;
    fn Retrieve(this: &Self::This, lnformattype: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetNumElements(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetElement(this: &Self::This, lnelementindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddLeafElement(this: &Self::This, bstrleafelement: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveLeafElement(this: &Self::This) -> ::windows_core::Result<()>;
    fn CopyPath(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetEscapedElement(this: &Self::This, lnreserved: i32, bstrinstr: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EscapedMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEscapedMode(this: &Self::This, lnescapedmode: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPathname {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPathname {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnsettype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::core::mem::transmute(&bstradspath), ::core::mem::transmute_copy(&lnsettype)).into())
        }
        unsafe extern "system" fn SetDisplayType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lndisplaytype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayType(this, ::core::mem::transmute_copy(&lndisplaytype)).into())
        }
        unsafe extern "system" fn Retrieve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Retrieve(this, ::core::mem::transmute_copy(&lnformattype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plnnumpathelements: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumElements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plnnumpathelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetElement(this, ::core::mem::transmute_copy(&lnelementindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddLeafElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrleafelement: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLeafElement(this, ::core::mem::transmute(&bstrleafelement)).into())
        }
        unsafe extern "system" fn RemoveLeafElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLeafElement(this).into())
        }
        unsafe extern "system" fn CopyPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppadspath: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadspath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEscapedElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnreserved: i32, bstrinstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstroutstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEscapedElement(this, ::core::mem::transmute_copy(&lnreserved), ::core::mem::transmute(&bstrinstr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstroutstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EscapedMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EscapedMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEscapedMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPathname_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnescapedmode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEscapedMode(this, ::core::mem::transmute_copy(&lnescapedmode)).into())
        }
        IADsPathname_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Set: Set::<Identity, Impl, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, Impl, OFFSET>,
            Retrieve: Retrieve::<Identity, Impl, OFFSET>,
            GetNumElements: GetNumElements::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            AddLeafElement: AddLeafElement::<Identity, Impl, OFFSET>,
            RemoveLeafElement: RemoveLeafElement::<Identity, Impl, OFFSET>,
            CopyPath: CopyPath::<Identity, Impl, OFFSET>,
            GetEscapedElement: GetEscapedElement::<Identity, Impl, OFFSET>,
            EscapedMode: EscapedMode::<Identity, Impl, OFFSET>,
            SetEscapedMode: SetEscapedMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPostalAddress_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn PostalAddress(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPostalAddress(this: &Self::This, vpostaladdress: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPostalAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPostalAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostalAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPostalAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vpostaladdress: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostalAddress(this, ::core::mem::transmute(&vpostaladdress)).into())
        }
        IADsPostalAddress_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PostalAddress: PostalAddress::<Identity, Impl, OFFSET>,
            SetPostalAddress: SetPostalAddress::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintJob_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn HostPrintQueue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn User(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TimeSubmitted(this: &Self::This) -> ::windows_core::Result<f64>;
    fn TotalPages(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Size(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPriority(this: &Self::This, lnpriority: i32) -> ::windows_core::Result<()>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetStartTime(this: &Self::This, dastarttime: f64) -> ::windows_core::Result<()>;
    fn UntilTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetUntilTime(this: &Self::This, dauntiltime: f64) -> ::windows_core::Result<()>;
    fn Notify(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNotify(this: &Self::This, bstrnotify: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NotifyPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNotifyPath(this: &Self::This, bstrnotifypath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPrintJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPrintJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HostPrintQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HostPrintQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TimeSubmitted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TimeSubmitted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalPages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&lnpriority)).into())
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartTime(this, ::core::mem::transmute_copy(&dastarttime)).into())
        }
        unsafe extern "system" fn UntilTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UntilTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUntilTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUntilTime(this, ::core::mem::transmute_copy(&dauntiltime)).into())
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Notify(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnotify: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotify(this, ::core::mem::transmute(&bstrnotify)).into())
        }
        unsafe extern "system" fn NotifyPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NotifyPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotifyPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnotifypath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotifyPath(this, ::core::mem::transmute(&bstrnotifypath)).into())
        }
        IADsPrintJob_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HostPrintQueue: HostPrintQueue::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            TimeSubmitted: TimeSubmitted::<Identity, Impl, OFFSET>,
            TotalPages: TotalPages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            SetNotify: SetNotify::<Identity, Impl, OFFSET>,
            NotifyPath: NotifyPath::<Identity, Impl, OFFSET>,
            SetNotifyPath: SetNotifyPath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintJobOperations_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TimeElapsed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PagesPrinted(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Position(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPosition(this: &Self::This, lnposition: i32) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPrintJobOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPrintJobOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TimeElapsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TimeElapsed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PagesPrinted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PagesPrinted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Position<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Position(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnposition: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPosition(this, ::core::mem::transmute_copy(&lnposition)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintJobOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        IADsPrintJobOperations_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            TimeElapsed: TimeElapsed::<Identity, Impl, OFFSET>,
            PagesPrinted: PagesPrinted::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintQueue_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn PrinterPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrinterPath(this: &Self::This, bstrprinterpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Model(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetModel(this: &Self::This, bstrmodel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Datatype(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDatatype(this: &Self::This, bstrdatatype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrintProcessor(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrintProcessor(this: &Self::This, bstrprintprocessor: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Location(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocation(this: &Self::This, bstrlocation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetStartTime(this: &Self::This, dastarttime: f64) -> ::windows_core::Result<()>;
    fn UntilTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetUntilTime(this: &Self::This, dauntiltime: f64) -> ::windows_core::Result<()>;
    fn DefaultJobPriority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDefaultJobPriority(this: &Self::This, lndefaultjobpriority: i32) -> ::windows_core::Result<()>;
    fn Priority(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPriority(this: &Self::This, lnpriority: i32) -> ::windows_core::Result<()>;
    fn BannerPage(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBannerPage(this: &Self::This, bstrbannerpage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrintDevices(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPrintDevices(this: &Self::This, vprintdevices: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn NetAddresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetNetAddresses(this: &Self::This, vnetaddresses: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPrintQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPrintQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrinterPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrinterPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrinterPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprinterpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrinterPath(this, ::core::mem::transmute(&bstrprinterpath)).into())
        }
        unsafe extern "system" fn Model<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Model(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmodel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModel(this, ::core::mem::transmute(&bstrmodel)).into())
        }
        unsafe extern "system" fn Datatype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Datatype(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDatatype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdatatype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDatatype(this, ::core::mem::transmute(&bstrdatatype)).into())
        }
        unsafe extern "system" fn PrintProcessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrintProcessor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintProcessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprintprocessor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintProcessor(this, ::core::mem::transmute(&bstrprintprocessor)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn Location<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Location(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocation(this, ::core::mem::transmute(&bstrlocation)).into())
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartTime(this, ::core::mem::transmute_copy(&dastarttime)).into())
        }
        unsafe extern "system" fn UntilTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UntilTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUntilTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUntilTime(this, ::core::mem::transmute_copy(&dauntiltime)).into())
        }
        unsafe extern "system" fn DefaultJobPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultJobPriority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultJobPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lndefaultjobpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultJobPriority(this, ::core::mem::transmute_copy(&lndefaultjobpriority)).into())
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Priority(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPriority(this, ::core::mem::transmute_copy(&lnpriority)).into())
        }
        unsafe extern "system" fn BannerPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BannerPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBannerPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbannerpage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBannerPage(this, ::core::mem::transmute(&bstrbannerpage)).into())
        }
        unsafe extern "system" fn PrintDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrintDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vprintdevices: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintDevices(this, ::core::mem::transmute(&vprintdevices)).into())
        }
        unsafe extern "system" fn NetAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetAddresses(this, ::core::mem::transmute(&vnetaddresses)).into())
        }
        IADsPrintQueue_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrinterPath: PrinterPath::<Identity, Impl, OFFSET>,
            SetPrinterPath: SetPrinterPath::<Identity, Impl, OFFSET>,
            Model: Model::<Identity, Impl, OFFSET>,
            SetModel: SetModel::<Identity, Impl, OFFSET>,
            Datatype: Datatype::<Identity, Impl, OFFSET>,
            SetDatatype: SetDatatype::<Identity, Impl, OFFSET>,
            PrintProcessor: PrintProcessor::<Identity, Impl, OFFSET>,
            SetPrintProcessor: SetPrintProcessor::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            SetLocation: SetLocation::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            UntilTime: UntilTime::<Identity, Impl, OFFSET>,
            SetUntilTime: SetUntilTime::<Identity, Impl, OFFSET>,
            DefaultJobPriority: DefaultJobPriority::<Identity, Impl, OFFSET>,
            SetDefaultJobPriority: SetDefaultJobPriority::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            BannerPage: BannerPage::<Identity, Impl, OFFSET>,
            SetBannerPage: SetBannerPage::<Identity, Impl, OFFSET>,
            PrintDevices: PrintDevices::<Identity, Impl, OFFSET>,
            SetPrintDevices: SetPrintDevices::<Identity, Impl, OFFSET>,
            NetAddresses: NetAddresses::<Identity, Impl, OFFSET>,
            SetNetAddresses: SetNetAddresses::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPrintQueueOperations_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PrintJobs(this: &Self::This) -> ::windows_core::Result<IADsCollection>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn Purge(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPrintQueueOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPrintQueueOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrintJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrintJobs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this).into())
        }
        unsafe extern "system" fn Purge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPrintQueueOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Purge(this).into())
        }
        IADsPrintQueueOperations_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            PrintJobs: PrintJobs::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsProperty_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn OID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOID(this: &Self::This, bstroid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Syntax(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSyntax(this: &Self::This, bstrsyntax: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxRange(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxRange(this: &Self::This, lnmaxrange: i32) -> ::windows_core::Result<()>;
    fn MinRange(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMinRange(this: &Self::This, lnminrange: i32) -> ::windows_core::Result<()>;
    fn MultiValued(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMultiValued(this: &Self::This, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Qualifiers(this: &Self::This) -> ::windows_core::Result<IADsCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOID(this, ::core::mem::transmute(&bstroid)).into())
        }
        unsafe extern "system" fn Syntax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Syntax(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSyntax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsyntax: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyntax(this, ::core::mem::transmute(&bstrsyntax)).into())
        }
        unsafe extern "system" fn MaxRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnmaxrange: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxRange(this, ::core::mem::transmute_copy(&lnmaxrange)).into())
        }
        unsafe extern "system" fn MinRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMinRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnminrange: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMinRange(this, ::core::mem::transmute_copy(&lnminrange)).into())
        }
        unsafe extern "system" fn MultiValued<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MultiValued(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMultiValued<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultiValued(this, ::core::mem::transmute_copy(&fmultivalued)).into())
        }
        unsafe extern "system" fn Qualifiers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Qualifiers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualifiers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsProperty_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OID: OID::<Identity, Impl, OFFSET>,
            SetOID: SetOID::<Identity, Impl, OFFSET>,
            Syntax: Syntax::<Identity, Impl, OFFSET>,
            SetSyntax: SetSyntax::<Identity, Impl, OFFSET>,
            MaxRange: MaxRange::<Identity, Impl, OFFSET>,
            SetMaxRange: SetMaxRange::<Identity, Impl, OFFSET>,
            MinRange: MinRange::<Identity, Impl, OFFSET>,
            SetMinRange: SetMinRange::<Identity, Impl, OFFSET>,
            MultiValued: MultiValued::<Identity, Impl, OFFSET>,
            SetMultiValued: SetMultiValued::<Identity, Impl, OFFSET>,
            Qualifiers: Qualifiers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyEntry_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ADsType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetADsType(this: &Self::This, lnadstype: i32) -> ::windows_core::Result<()>;
    fn ControlCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetControlCode(this: &Self::This, lncontrolcode: i32) -> ::windows_core::Result<()>;
    fn Values(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValues(this: &Self::This, vvalues: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPropertyEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPropertyEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn ADsType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADsType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetADsType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetADsType(this, ::core::mem::transmute_copy(&lnadstype)).into())
        }
        unsafe extern "system" fn ControlCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControlCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetControlCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lncontrolcode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControlCode(this, ::core::mem::transmute_copy(&lncontrolcode)).into())
        }
        unsafe extern "system" fn Values<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Values(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vvalues: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValues(this, ::core::mem::transmute(&vvalues)).into())
        }
        IADsPropertyEntry_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clear: Clear::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            ControlCode: ControlCode::<Identity, Impl, OFFSET>,
            SetControlCode: SetControlCode::<Identity, Impl, OFFSET>,
            Values: Values::<Identity, Impl, OFFSET>,
            SetValues: SetValues::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyList_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn PropertyCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Next(this: &Self::This, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celements: i32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Item(this: &Self::This, varindex: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetPropertyItem(this: &Self::This, bstrname: &::windows_core::BSTR, lnadstype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PutPropertyItem(this: &Self::This, vardata: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ResetPropertyItem(this: &Self::This, varentry: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PurgePropertyList(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPropertyList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPropertyList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropertyCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&pvariant)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celements: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celements)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&varindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnadstype: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyItem(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&lnadstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PutPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardata: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutPropertyItem(this, ::core::mem::transmute(&vardata)).into())
        }
        unsafe extern "system" fn ResetPropertyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varentry: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetPropertyItem(this, ::core::mem::transmute(&varentry)).into())
        }
        unsafe extern "system" fn PurgePropertyList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PurgePropertyList(this).into())
        }
        IADsPropertyList_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropertyCount: PropertyCount::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetPropertyItem: GetPropertyItem::<Identity, Impl, OFFSET>,
            PutPropertyItem: PutPropertyItem::<Identity, Impl, OFFSET>,
            ResetPropertyItem: ResetPropertyItem::<Identity, Impl, OFFSET>,
            PurgePropertyList: PurgePropertyList::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyValue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn ADsType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetADsType(this: &Self::This, lnadstype: i32) -> ::windows_core::Result<()>;
    fn DNString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDNString(this: &Self::This, bstrdnstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CaseExactString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCaseExactString(this: &Self::This, bstrcaseexactstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CaseIgnoreString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCaseIgnoreString(this: &Self::This, bstrcaseignorestring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PrintableString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPrintableString(this: &Self::This, bstrprintablestring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NumericString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNumericString(this: &Self::This, bstrnumericstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Boolean(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBoolean(this: &Self::This, lnboolean: i32) -> ::windows_core::Result<()>;
    fn Integer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetInteger(this: &Self::This, lninteger: i32) -> ::windows_core::Result<()>;
    fn OctetString(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOctetString(this: &Self::This, voctetstring: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SecurityDescriptor(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(this: &Self::This, psecuritydescriptor: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn LargeInteger(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetLargeInteger(this: &Self::This, plargeinteger: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn UTCTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetUTCTime(this: &Self::This, dautctime: f64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPropertyValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPropertyValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn ADsType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADsType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetADsType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetADsType(this, ::core::mem::transmute_copy(&lnadstype)).into())
        }
        unsafe extern "system" fn DNString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DNString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDNString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDNString(this, ::core::mem::transmute(&bstrdnstring)).into())
        }
        unsafe extern "system" fn CaseExactString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CaseExactString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCaseExactString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcaseexactstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCaseExactString(this, ::core::mem::transmute(&bstrcaseexactstring)).into())
        }
        unsafe extern "system" fn CaseIgnoreString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CaseIgnoreString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCaseIgnoreString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcaseignorestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCaseIgnoreString(this, ::core::mem::transmute(&bstrcaseignorestring)).into())
        }
        unsafe extern "system" fn PrintableString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrintableString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintableString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprintablestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintableString(this, ::core::mem::transmute(&bstrprintablestring)).into())
        }
        unsafe extern "system" fn NumericString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumericString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNumericString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnumericstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumericString(this, ::core::mem::transmute(&bstrnumericstring)).into())
        }
        unsafe extern "system" fn Boolean<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Boolean(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBoolean<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnboolean: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBoolean(this, ::core::mem::transmute_copy(&lnboolean)).into())
        }
        unsafe extern "system" fn Integer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Integer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInteger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lninteger: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInteger(this, ::core::mem::transmute_copy(&lninteger)).into())
        }
        unsafe extern "system" fn OctetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OctetString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOctetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voctetstring: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOctetString(this, ::core::mem::transmute(&voctetstring)).into())
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecurityDescriptor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecuritydescriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityDescriptor(this, ::windows_core::from_raw_borrowed(&psecuritydescriptor)).into())
        }
        unsafe extern "system" fn LargeInteger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LargeInteger(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLargeInteger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plargeinteger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLargeInteger(this, ::windows_core::from_raw_borrowed(&plargeinteger)).into())
        }
        unsafe extern "system" fn UTCTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UTCTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUTCTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dautctime: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUTCTime(this, ::core::mem::transmute_copy(&dautctime)).into())
        }
        IADsPropertyValue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clear: Clear::<Identity, Impl, OFFSET>,
            ADsType: ADsType::<Identity, Impl, OFFSET>,
            SetADsType: SetADsType::<Identity, Impl, OFFSET>,
            DNString: DNString::<Identity, Impl, OFFSET>,
            SetDNString: SetDNString::<Identity, Impl, OFFSET>,
            CaseExactString: CaseExactString::<Identity, Impl, OFFSET>,
            SetCaseExactString: SetCaseExactString::<Identity, Impl, OFFSET>,
            CaseIgnoreString: CaseIgnoreString::<Identity, Impl, OFFSET>,
            SetCaseIgnoreString: SetCaseIgnoreString::<Identity, Impl, OFFSET>,
            PrintableString: PrintableString::<Identity, Impl, OFFSET>,
            SetPrintableString: SetPrintableString::<Identity, Impl, OFFSET>,
            NumericString: NumericString::<Identity, Impl, OFFSET>,
            SetNumericString: SetNumericString::<Identity, Impl, OFFSET>,
            Boolean: Boolean::<Identity, Impl, OFFSET>,
            SetBoolean: SetBoolean::<Identity, Impl, OFFSET>,
            Integer: Integer::<Identity, Impl, OFFSET>,
            SetInteger: SetInteger::<Identity, Impl, OFFSET>,
            OctetString: OctetString::<Identity, Impl, OFFSET>,
            SetOctetString: SetOctetString::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            LargeInteger: LargeInteger::<Identity, Impl, OFFSET>,
            SetLargeInteger: SetLargeInteger::<Identity, Impl, OFFSET>,
            UTCTime: UTCTime::<Identity, Impl, OFFSET>,
            SetUTCTime: SetUTCTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsPropertyValue2_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetObjectProperty(this: &Self::This, lnadstype: *mut i32, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PutObjectProperty(this: &Self::This, lnadstype: i32, vprop: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsPropertyValue2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsPropertyValue2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectProperty(this, ::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute_copy(&pvprop)).into())
        }
        unsafe extern "system" fn PutObjectProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsPropertyValue2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnadstype: i32, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutObjectProperty(this, ::core::mem::transmute_copy(&lnadstype), ::core::mem::transmute(&vprop)).into())
        }
        IADsPropertyValue2_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectProperty: GetObjectProperty::<Identity, Impl, OFFSET>,
            PutObjectProperty: PutObjectProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsReplicaPointer_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ServerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServerName(this: &Self::This, bstrservername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReplicaType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetReplicaType(this: &Self::This, lnreplicatype: i32) -> ::windows_core::Result<()>;
    fn ReplicaNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetReplicaNumber(this: &Self::This, lnreplicanumber: i32) -> ::windows_core::Result<()>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetCount(this: &Self::This, lncount: i32) -> ::windows_core::Result<()>;
    fn ReplicaAddressHints(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetReplicaAddressHints(this: &Self::This, vreplicaaddresshints: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsReplicaPointer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsReplicaPointer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerName(this, ::core::mem::transmute(&bstrservername)).into())
        }
        unsafe extern "system" fn ReplicaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplicaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReplicaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnreplicatype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReplicaType(this, ::core::mem::transmute_copy(&lnreplicatype)).into())
        }
        unsafe extern "system" fn ReplicaNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplicaNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReplicaNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnreplicanumber: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReplicaNumber(this, ::core::mem::transmute_copy(&lnreplicanumber)).into())
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lncount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCount(this, ::core::mem::transmute_copy(&lncount)).into())
        }
        unsafe extern "system" fn ReplicaAddressHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplicaAddressHints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReplicaAddressHints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsReplicaPointer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vreplicaaddresshints: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReplicaAddressHints(this, ::core::mem::transmute(&vreplicaaddresshints)).into())
        }
        IADsReplicaPointer_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServerName: ServerName::<Identity, Impl, OFFSET>,
            SetServerName: SetServerName::<Identity, Impl, OFFSET>,
            ReplicaType: ReplicaType::<Identity, Impl, OFFSET>,
            SetReplicaType: SetReplicaType::<Identity, Impl, OFFSET>,
            ReplicaNumber: ReplicaNumber::<Identity, Impl, OFFSET>,
            SetReplicaNumber: SetReplicaNumber::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            SetCount: SetCount::<Identity, Impl, OFFSET>,
            ReplicaAddressHints: ReplicaAddressHints::<Identity, Impl, OFFSET>,
            SetReplicaAddressHints: SetReplicaAddressHints::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsResource_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn User(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LockCount(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LockCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsResource_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            LockCount: LockCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSecurityDescriptor_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Revision(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRevision(this: &Self::This, lnrevision: i32) -> ::windows_core::Result<()>;
    fn Control(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetControl(this: &Self::This, lncontrol: i32) -> ::windows_core::Result<()>;
    fn Owner(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOwner(this: &Self::This, bstrowner: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OwnerDefaulted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOwnerDefaulted(this: &Self::This, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Group(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetGroup(this: &Self::This, bstrgroup: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GroupDefaulted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGroupDefaulted(this: &Self::This, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DiscretionaryAcl(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetDiscretionaryAcl(this: &Self::This, pdiscretionaryacl: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn DaclDefaulted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDaclDefaulted(this: &Self::This, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SystemAcl(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSystemAcl(this: &Self::This, psystemacl: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn SaclDefaulted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSaclDefaulted(this: &Self::This, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn CopySecurityDescriptor(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsSecurityDescriptor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsSecurityDescriptor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Revision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Revision(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnrevision: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRevision(this, ::core::mem::transmute_copy(&lnrevision)).into())
        }
        unsafe extern "system" fn Control<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Control(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lncontrol: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControl(this, ::core::mem::transmute_copy(&lncontrol)).into())
        }
        unsafe extern "system" fn Owner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Owner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOwner(this, ::core::mem::transmute(&bstrowner)).into())
        }
        unsafe extern "system" fn OwnerDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerDefaulted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOwnerDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOwnerDefaulted(this, ::core::mem::transmute_copy(&fownerdefaulted)).into())
        }
        unsafe extern "system" fn Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Group(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroup(this, ::core::mem::transmute(&bstrgroup)).into())
        }
        unsafe extern "system" fn GroupDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GroupDefaulted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroupDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroupDefaulted(this, ::core::mem::transmute_copy(&fgroupdefaulted)).into())
        }
        unsafe extern "system" fn DiscretionaryAcl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiscretionaryAcl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDiscretionaryAcl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiscretionaryacl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiscretionaryAcl(this, ::windows_core::from_raw_borrowed(&pdiscretionaryacl)).into())
        }
        unsafe extern "system" fn DaclDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DaclDefaulted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDaclDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDaclDefaulted(this, ::core::mem::transmute_copy(&fdacldefaulted)).into())
        }
        unsafe extern "system" fn SystemAcl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SystemAcl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSystemAcl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psystemacl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSystemAcl(this, ::windows_core::from_raw_borrowed(&psystemacl)).into())
        }
        unsafe extern "system" fn SaclDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaclDefaulted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSaclDefaulted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSaclDefaulted(this, ::core::mem::transmute_copy(&fsacldefaulted)).into())
        }
        unsafe extern "system" fn CopySecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopySecurityDescriptor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecuritydescriptor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsSecurityDescriptor_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Revision: Revision::<Identity, Impl, OFFSET>,
            SetRevision: SetRevision::<Identity, Impl, OFFSET>,
            Control: Control::<Identity, Impl, OFFSET>,
            SetControl: SetControl::<Identity, Impl, OFFSET>,
            Owner: Owner::<Identity, Impl, OFFSET>,
            SetOwner: SetOwner::<Identity, Impl, OFFSET>,
            OwnerDefaulted: OwnerDefaulted::<Identity, Impl, OFFSET>,
            SetOwnerDefaulted: SetOwnerDefaulted::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            GroupDefaulted: GroupDefaulted::<Identity, Impl, OFFSET>,
            SetGroupDefaulted: SetGroupDefaulted::<Identity, Impl, OFFSET>,
            DiscretionaryAcl: DiscretionaryAcl::<Identity, Impl, OFFSET>,
            SetDiscretionaryAcl: SetDiscretionaryAcl::<Identity, Impl, OFFSET>,
            DaclDefaulted: DaclDefaulted::<Identity, Impl, OFFSET>,
            SetDaclDefaulted: SetDaclDefaulted::<Identity, Impl, OFFSET>,
            SystemAcl: SystemAcl::<Identity, Impl, OFFSET>,
            SetSystemAcl: SetSystemAcl::<Identity, Impl, OFFSET>,
            SaclDefaulted: SaclDefaulted::<Identity, Impl, OFFSET>,
            SetSaclDefaulted: SetSaclDefaulted::<Identity, Impl, OFFSET>,
            CopySecurityDescriptor: CopySecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSecurityUtility_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetSecurityDescriptor(this: &Self::This, varpath: &super::super::System::Variant::VARIANT, lpathformat: i32, lformat: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSecurityDescriptor(this: &Self::This, varpath: &super::super::System::Variant::VARIANT, lpathformat: i32, vardata: &super::super::System::Variant::VARIANT, ldataformat: i32) -> ::windows_core::Result<()>;
    fn ConvertSecurityDescriptor(this: &Self::This, varsd: &super::super::System::Variant::VARIANT, ldataformat: i32, loutformat: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SecurityMask(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSecurityMask(this: &Self::This, lnsecuritymask: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsSecurityUtility {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsSecurityUtility {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, lformat: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityDescriptor(this, ::core::mem::transmute(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute_copy(&lformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, vardata: super::super::System::Variant::VARIANT, ldataformat: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityDescriptor(this, ::core::mem::transmute(&varpath), ::core::mem::transmute_copy(&lpathformat), ::core::mem::transmute(&vardata), ::core::mem::transmute_copy(&ldataformat)).into())
        }
        unsafe extern "system" fn ConvertSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varsd: super::super::System::Variant::VARIANT, ldataformat: i32, loutformat: i32, presult: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConvertSecurityDescriptor(this, ::core::mem::transmute(&varsd), ::core::mem::transmute_copy(&ldataformat), ::core::mem::transmute_copy(&loutformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SecurityMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecurityMask(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSecurityUtility_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnsecuritymask: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityMask(this, ::core::mem::transmute_copy(&lnsecuritymask)).into())
        }
        IADsSecurityUtility_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            ConvertSecurityDescriptor: ConvertSecurityDescriptor::<Identity, Impl, OFFSET>,
            SecurityMask: SecurityMask::<Identity, Impl, OFFSET>,
            SetSecurityMask: SetSecurityMask::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsService_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn HostComputer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHostComputer(this: &Self::This, bstrhostcomputer: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplayName(this: &Self::This, bstrdisplayname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVersion(this: &Self::This, bstrversion: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetServiceType(this: &Self::This, lnservicetype: i32) -> ::windows_core::Result<()>;
    fn StartType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetStartType(this: &Self::This, lnstarttype: i32) -> ::windows_core::Result<()>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPath(this: &Self::This, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartupParameters(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStartupParameters(this: &Self::This, bstrstartupparameters: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ErrorControl(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetErrorControl(this: &Self::This, lnerrorcontrol: i32) -> ::windows_core::Result<()>;
    fn LoadOrderGroup(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLoadOrderGroup(this: &Self::This, bstrloadordergroup: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceAccountName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceAccountName(this: &Self::This, bstrserviceaccountname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceAccountPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceAccountPath(this: &Self::This, bstrserviceaccountpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Dependencies(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetDependencies(this: &Self::This, vdependencies: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HostComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HostComputer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHostComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHostComputer(this, ::core::mem::transmute(&bstrhostcomputer)).into())
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdisplayname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&bstrdisplayname)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVersion(this, ::core::mem::transmute(&bstrversion)).into())
        }
        unsafe extern "system" fn ServiceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnservicetype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceType(this, ::core::mem::transmute_copy(&lnservicetype)).into())
        }
        unsafe extern "system" fn StartType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnstarttype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartType(this, ::core::mem::transmute_copy(&lnstarttype)).into())
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&bstrpath)).into())
        }
        unsafe extern "system" fn StartupParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartupParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartupParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrstartupparameters: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartupParameters(this, ::core::mem::transmute(&bstrstartupparameters)).into())
        }
        unsafe extern "system" fn ErrorControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ErrorControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetErrorControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnerrorcontrol: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorControl(this, ::core::mem::transmute_copy(&lnerrorcontrol)).into())
        }
        unsafe extern "system" fn LoadOrderGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadOrderGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLoadOrderGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrloadordergroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoadOrderGroup(this, ::core::mem::transmute(&bstrloadordergroup)).into())
        }
        unsafe extern "system" fn ServiceAccountName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceAccountName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceAccountName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrserviceaccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceAccountName(this, ::core::mem::transmute(&bstrserviceaccountname)).into())
        }
        unsafe extern "system" fn ServiceAccountPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceAccountPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceAccountPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrserviceaccountpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceAccountPath(this, ::core::mem::transmute(&bstrserviceaccountpath)).into())
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vdependencies: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDependencies(this, ::core::mem::transmute(&vdependencies)).into())
        }
        IADsService_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HostComputer: HostComputer::<Identity, Impl, OFFSET>,
            SetHostComputer: SetHostComputer::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            ServiceType: ServiceType::<Identity, Impl, OFFSET>,
            SetServiceType: SetServiceType::<Identity, Impl, OFFSET>,
            StartType: StartType::<Identity, Impl, OFFSET>,
            SetStartType: SetStartType::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            StartupParameters: StartupParameters::<Identity, Impl, OFFSET>,
            SetStartupParameters: SetStartupParameters::<Identity, Impl, OFFSET>,
            ErrorControl: ErrorControl::<Identity, Impl, OFFSET>,
            SetErrorControl: SetErrorControl::<Identity, Impl, OFFSET>,
            LoadOrderGroup: LoadOrderGroup::<Identity, Impl, OFFSET>,
            SetLoadOrderGroup: SetLoadOrderGroup::<Identity, Impl, OFFSET>,
            ServiceAccountName: ServiceAccountName::<Identity, Impl, OFFSET>,
            SetServiceAccountName: SetServiceAccountName::<Identity, Impl, OFFSET>,
            ServiceAccountPath: ServiceAccountPath::<Identity, Impl, OFFSET>,
            SetServiceAccountPath: SetServiceAccountPath::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            SetDependencies: SetDependencies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsServiceOperations_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn Continue(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPassword(this: &Self::This, bstrnewpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsServiceOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsServiceOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn Continue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Continue(this).into())
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsServiceOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnewpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassword(this, ::core::mem::transmute(&bstrnewpassword)).into())
        }
        IADsServiceOperations_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSession_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn User(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Computer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ComputerPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ConnectTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn IdleTime(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Computer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Computer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputerPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputerPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConnectTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IdleTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IdleTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsSession_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            User: User::<Identity, Impl, OFFSET>,
            UserPath: UserPath::<Identity, Impl, OFFSET>,
            Computer: Computer::<Identity, Impl, OFFSET>,
            ComputerPath: ComputerPath::<Identity, Impl, OFFSET>,
            ConnectTime: ConnectTime::<Identity, Impl, OFFSET>,
            IdleTime: IdleTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsSyntax_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn OleAutoDataType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetOleAutoDataType(this: &Self::This, lnoleautodatatype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsSyntax {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsSyntax {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OleAutoDataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OleAutoDataType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOleAutoDataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsSyntax_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnoleautodatatype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOleAutoDataType(this, ::core::mem::transmute_copy(&lnoleautodatatype)).into())
        }
        IADsSyntax_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OleAutoDataType: OleAutoDataType::<Identity, Impl, OFFSET>,
            SetOleAutoDataType: SetOleAutoDataType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsTimestamp_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn WholeSeconds(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetWholeSeconds(this: &Self::This, lnwholeseconds: i32) -> ::windows_core::Result<()>;
    fn EventID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEventID(this: &Self::This, lneventid: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsTimestamp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsTimestamp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WholeSeconds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WholeSeconds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWholeSeconds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnwholeseconds: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWholeSeconds(this, ::core::mem::transmute_copy(&lnwholeseconds)).into())
        }
        unsafe extern "system" fn EventID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTimestamp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lneventid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventID(this, ::core::mem::transmute_copy(&lneventid)).into())
        }
        IADsTimestamp_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WholeSeconds: WholeSeconds::<Identity, Impl, OFFSET>,
            SetWholeSeconds: SetWholeSeconds::<Identity, Impl, OFFSET>,
            EventID: EventID::<Identity, Impl, OFFSET>,
            SetEventID: SetEventID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsTypedName_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ObjectName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetObjectName(this: &Self::This, bstrobjectname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Level(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLevel(this: &Self::This, lnlevel: i32) -> ::windows_core::Result<()>;
    fn Interval(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetInterval(this: &Self::This, lninterval: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsTypedName {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsTypedName {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectName(this, ::core::mem::transmute(&bstrobjectname)).into())
        }
        unsafe extern "system" fn Level<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Level(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnlevel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLevel(this, ::core::mem::transmute_copy(&lnlevel)).into())
        }
        unsafe extern "system" fn Interval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Interval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsTypedName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lninterval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterval(this, ::core::mem::transmute_copy(&lninterval)).into())
        }
        IADsTypedName_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ObjectName: ObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            SetLevel: SetLevel::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsUser_Impl: ::windows_core::BaseImpl + IADs_Impl {
    fn BadLoginAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BadLoginCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LastLogin(this: &Self::This) -> ::windows_core::Result<f64>;
    fn LastLogoff(this: &Self::This) -> ::windows_core::Result<f64>;
    fn LastFailedLogin(this: &Self::This) -> ::windows_core::Result<f64>;
    fn PasswordLastChanged(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Division(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDivision(this: &Self::This, bstrdivision: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Department(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDepartment(this: &Self::This, bstrdepartment: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EmployeeID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEmployeeID(this: &Self::This, bstremployeeid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FullName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFullName(this: &Self::This, bstrfullname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FirstName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetFirstName(this: &Self::This, bstrfirstname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LastName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLastName(this: &Self::This, bstrlastname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OtherName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOtherName(this: &Self::This, bstrothername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NamePrefix(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNamePrefix(this: &Self::This, bstrnameprefix: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NameSuffix(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetNameSuffix(this: &Self::This, bstrnamesuffix: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetTitle(this: &Self::This, bstrtitle: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Manager(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetManager(this: &Self::This, bstrmanager: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TelephoneHome(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephoneHome(this: &Self::This, vtelephonehome: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TelephoneMobile(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephoneMobile(this: &Self::This, vtelephonemobile: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TelephoneNumber(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephoneNumber(this: &Self::This, vtelephonenumber: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TelephonePager(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetTelephonePager(this: &Self::This, vtelephonepager: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn FaxNumber(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetFaxNumber(this: &Self::This, vfaxnumber: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn OfficeLocations(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetOfficeLocations(this: &Self::This, vofficelocations: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PostalAddresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPostalAddresses(this: &Self::This, vpostaladdresses: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PostalCodes(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPostalCodes(this: &Self::This, vpostalcodes: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SeeAlso(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetSeeAlso(this: &Self::This, vseealso: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AccountDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAccountDisabled(this: &Self::This, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AccountExpirationDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetAccountExpirationDate(this: &Self::This, daaccountexpirationdate: f64) -> ::windows_core::Result<()>;
    fn GraceLoginsAllowed(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetGraceLoginsAllowed(this: &Self::This, lngraceloginsallowed: i32) -> ::windows_core::Result<()>;
    fn GraceLoginsRemaining(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetGraceLoginsRemaining(this: &Self::This, lngraceloginsremaining: i32) -> ::windows_core::Result<()>;
    fn IsAccountLocked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsAccountLocked(this: &Self::This, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn LoginHours(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetLoginHours(this: &Self::This, vloginhours: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn LoginWorkstations(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetLoginWorkstations(this: &Self::This, vloginworkstations: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn MaxLogins(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxLogins(this: &Self::This, lnmaxlogins: i32) -> ::windows_core::Result<()>;
    fn MaxStorage(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxStorage(this: &Self::This, lnmaxstorage: i32) -> ::windows_core::Result<()>;
    fn PasswordExpirationDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetPasswordExpirationDate(this: &Self::This, dapasswordexpirationdate: f64) -> ::windows_core::Result<()>;
    fn PasswordMinimumLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPasswordMinimumLength(this: &Self::This, lnpasswordminimumlength: i32) -> ::windows_core::Result<()>;
    fn PasswordRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPasswordRequired(this: &Self::This, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RequireUniquePassword(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRequireUniquePassword(this: &Self::This, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EmailAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEmailAddress(this: &Self::This, bstremailaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HomeDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHomeDirectory(this: &Self::This, bstrhomedirectory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Languages(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetLanguages(this: &Self::This, vlanguages: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Profile(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProfile(this: &Self::This, bstrprofile: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LoginScript(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLoginScript(this: &Self::This, bstrloginscript: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Picture(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetPicture(this: &Self::This, vpicture: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn HomePage(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHomePage(this: &Self::This, bstrhomepage: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Groups(this: &Self::This) -> ::windows_core::Result<IADsMembers>;
    fn SetPassword(this: &Self::This, newpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangePassword(this: &Self::This, bstroldpassword: &::windows_core::BSTR, bstrnewpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsUser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IADs);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsUser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BadLoginAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BadLoginAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BadLoginCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BadLoginCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastLogin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastLogin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastLogoff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastLogoff(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastFailedLogin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastFailedLogin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PasswordLastChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasswordLastChanged(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&bstrdescription)).into())
        }
        unsafe extern "system" fn Division<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Division(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDivision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdivision: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDivision(this, ::core::mem::transmute(&bstrdivision)).into())
        }
        unsafe extern "system" fn Department<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Department(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDepartment(this, ::core::mem::transmute(&bstrdepartment)).into())
        }
        unsafe extern "system" fn EmployeeID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EmployeeID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEmployeeID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstremployeeid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEmployeeID(this, ::core::mem::transmute(&bstremployeeid)).into())
        }
        unsafe extern "system" fn FullName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FullName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFullName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfullname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFullName(this, ::core::mem::transmute(&bstrfullname)).into())
        }
        unsafe extern "system" fn FirstName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFirstName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfirstname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFirstName(this, ::core::mem::transmute(&bstrfirstname)).into())
        }
        unsafe extern "system" fn LastName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlastname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastName(this, ::core::mem::transmute(&bstrlastname)).into())
        }
        unsafe extern "system" fn OtherName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OtherName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOtherName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrothername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOtherName(this, ::core::mem::transmute(&bstrothername)).into())
        }
        unsafe extern "system" fn NamePrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamePrefix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamePrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnameprefix: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamePrefix(this, ::core::mem::transmute(&bstrnameprefix)).into())
        }
        unsafe extern "system" fn NameSuffix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NameSuffix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNameSuffix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnamesuffix: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNameSuffix(this, ::core::mem::transmute(&bstrnamesuffix)).into())
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitle(this, ::core::mem::transmute(&bstrtitle)).into())
        }
        unsafe extern "system" fn Manager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Manager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmanager: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetManager(this, ::core::mem::transmute(&bstrmanager)).into())
        }
        unsafe extern "system" fn TelephoneHome<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephoneHome(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTelephoneHome<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtelephonehome: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTelephoneHome(this, ::core::mem::transmute(&vtelephonehome)).into())
        }
        unsafe extern "system" fn TelephoneMobile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephoneMobile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTelephoneMobile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtelephonemobile: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTelephoneMobile(this, ::core::mem::transmute(&vtelephonemobile)).into())
        }
        unsafe extern "system" fn TelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephoneNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTelephoneNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtelephonenumber: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTelephoneNumber(this, ::core::mem::transmute(&vtelephonenumber)).into())
        }
        unsafe extern "system" fn TelephonePager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephonePager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTelephonePager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtelephonepager: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTelephonePager(this, ::core::mem::transmute(&vtelephonepager)).into())
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FaxNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vfaxnumber: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFaxNumber(this, ::core::mem::transmute(&vfaxnumber)).into())
        }
        unsafe extern "system" fn OfficeLocations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OfficeLocations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOfficeLocations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vofficelocations: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOfficeLocations(this, ::core::mem::transmute(&vofficelocations)).into())
        }
        unsafe extern "system" fn PostalAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostalAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostalAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vpostaladdresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostalAddresses(this, ::core::mem::transmute(&vpostaladdresses)).into())
        }
        unsafe extern "system" fn PostalCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PostalCodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPostalCodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vpostalcodes: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPostalCodes(this, ::core::mem::transmute(&vpostalcodes)).into())
        }
        unsafe extern "system" fn SeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SeeAlso(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSeeAlso<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSeeAlso(this, ::core::mem::transmute(&vseealso)).into())
        }
        unsafe extern "system" fn AccountDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccountDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccountDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccountDisabled(this, ::core::mem::transmute_copy(&faccountdisabled)).into())
        }
        unsafe extern "system" fn AccountExpirationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccountExpirationDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccountExpirationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, daaccountexpirationdate: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccountExpirationDate(this, ::core::mem::transmute_copy(&daaccountexpirationdate)).into())
        }
        unsafe extern "system" fn GraceLoginsAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GraceLoginsAllowed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGraceLoginsAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lngraceloginsallowed: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraceLoginsAllowed(this, ::core::mem::transmute_copy(&lngraceloginsallowed)).into())
        }
        unsafe extern "system" fn GraceLoginsRemaining<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GraceLoginsRemaining(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGraceLoginsRemaining<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lngraceloginsremaining: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraceLoginsRemaining(this, ::core::mem::transmute_copy(&lngraceloginsremaining)).into())
        }
        unsafe extern "system" fn IsAccountLocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAccountLocked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsAccountLocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsAccountLocked(this, ::core::mem::transmute_copy(&fisaccountlocked)).into())
        }
        unsafe extern "system" fn LoginHours<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoginHours(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLoginHours<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vloginhours: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoginHours(this, ::core::mem::transmute(&vloginhours)).into())
        }
        unsafe extern "system" fn LoginWorkstations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoginWorkstations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLoginWorkstations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vloginworkstations: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoginWorkstations(this, ::core::mem::transmute(&vloginworkstations)).into())
        }
        unsafe extern "system" fn MaxLogins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxLogins(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxLogins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnmaxlogins: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxLogins(this, ::core::mem::transmute_copy(&lnmaxlogins)).into())
        }
        unsafe extern "system" fn MaxStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxStorage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnmaxstorage: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxStorage(this, ::core::mem::transmute_copy(&lnmaxstorage)).into())
        }
        unsafe extern "system" fn PasswordExpirationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasswordExpirationDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPasswordExpirationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dapasswordexpirationdate: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPasswordExpirationDate(this, ::core::mem::transmute_copy(&dapasswordexpirationdate)).into())
        }
        unsafe extern "system" fn PasswordMinimumLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasswordMinimumLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPasswordMinimumLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnpasswordminimumlength: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPasswordMinimumLength(this, ::core::mem::transmute_copy(&lnpasswordminimumlength)).into())
        }
        unsafe extern "system" fn PasswordRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasswordRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPasswordRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPasswordRequired(this, ::core::mem::transmute_copy(&fpasswordrequired)).into())
        }
        unsafe extern "system" fn RequireUniquePassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequireUniquePassword(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRequireUniquePassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequireUniquePassword(this, ::core::mem::transmute_copy(&frequireuniquepassword)).into())
        }
        unsafe extern "system" fn EmailAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EmailAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEmailAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstremailaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEmailAddress(this, ::core::mem::transmute(&bstremailaddress)).into())
        }
        unsafe extern "system" fn HomeDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HomeDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHomeDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhomedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHomeDirectory(this, ::core::mem::transmute(&bstrhomedirectory)).into())
        }
        unsafe extern "system" fn Languages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Languages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vlanguages: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguages(this, ::core::mem::transmute(&vlanguages)).into())
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprofile: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProfile(this, ::core::mem::transmute(&bstrprofile)).into())
        }
        unsafe extern "system" fn LoginScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoginScript(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLoginScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrloginscript: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLoginScript(this, ::core::mem::transmute(&bstrloginscript)).into())
        }
        unsafe extern "system" fn Picture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Picture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPicture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vpicture: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPicture(this, ::core::mem::transmute(&vpicture)).into())
        }
        unsafe extern "system" fn HomePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HomePage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHomePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrhomepage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHomePage(this, ::core::mem::transmute(&bstrhomepage)).into())
        }
        unsafe extern "system" fn Groups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Groups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassword(this, ::core::mem::transmute(&newpassword)).into())
        }
        unsafe extern "system" fn ChangePassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroldpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangePassword(this, ::core::mem::transmute(&bstroldpassword), ::core::mem::transmute(&bstrnewpassword)).into())
        }
        IADsUser_Vtbl {
            base__: <IADs as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BadLoginAddress: BadLoginAddress::<Identity, Impl, OFFSET>,
            BadLoginCount: BadLoginCount::<Identity, Impl, OFFSET>,
            LastLogin: LastLogin::<Identity, Impl, OFFSET>,
            LastLogoff: LastLogoff::<Identity, Impl, OFFSET>,
            LastFailedLogin: LastFailedLogin::<Identity, Impl, OFFSET>,
            PasswordLastChanged: PasswordLastChanged::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Division: Division::<Identity, Impl, OFFSET>,
            SetDivision: SetDivision::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            EmployeeID: EmployeeID::<Identity, Impl, OFFSET>,
            SetEmployeeID: SetEmployeeID::<Identity, Impl, OFFSET>,
            FullName: FullName::<Identity, Impl, OFFSET>,
            SetFullName: SetFullName::<Identity, Impl, OFFSET>,
            FirstName: FirstName::<Identity, Impl, OFFSET>,
            SetFirstName: SetFirstName::<Identity, Impl, OFFSET>,
            LastName: LastName::<Identity, Impl, OFFSET>,
            SetLastName: SetLastName::<Identity, Impl, OFFSET>,
            OtherName: OtherName::<Identity, Impl, OFFSET>,
            SetOtherName: SetOtherName::<Identity, Impl, OFFSET>,
            NamePrefix: NamePrefix::<Identity, Impl, OFFSET>,
            SetNamePrefix: SetNamePrefix::<Identity, Impl, OFFSET>,
            NameSuffix: NameSuffix::<Identity, Impl, OFFSET>,
            SetNameSuffix: SetNameSuffix::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            Manager: Manager::<Identity, Impl, OFFSET>,
            SetManager: SetManager::<Identity, Impl, OFFSET>,
            TelephoneHome: TelephoneHome::<Identity, Impl, OFFSET>,
            SetTelephoneHome: SetTelephoneHome::<Identity, Impl, OFFSET>,
            TelephoneMobile: TelephoneMobile::<Identity, Impl, OFFSET>,
            SetTelephoneMobile: SetTelephoneMobile::<Identity, Impl, OFFSET>,
            TelephoneNumber: TelephoneNumber::<Identity, Impl, OFFSET>,
            SetTelephoneNumber: SetTelephoneNumber::<Identity, Impl, OFFSET>,
            TelephonePager: TelephonePager::<Identity, Impl, OFFSET>,
            SetTelephonePager: SetTelephonePager::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            OfficeLocations: OfficeLocations::<Identity, Impl, OFFSET>,
            SetOfficeLocations: SetOfficeLocations::<Identity, Impl, OFFSET>,
            PostalAddresses: PostalAddresses::<Identity, Impl, OFFSET>,
            SetPostalAddresses: SetPostalAddresses::<Identity, Impl, OFFSET>,
            PostalCodes: PostalCodes::<Identity, Impl, OFFSET>,
            SetPostalCodes: SetPostalCodes::<Identity, Impl, OFFSET>,
            SeeAlso: SeeAlso::<Identity, Impl, OFFSET>,
            SetSeeAlso: SetSeeAlso::<Identity, Impl, OFFSET>,
            AccountDisabled: AccountDisabled::<Identity, Impl, OFFSET>,
            SetAccountDisabled: SetAccountDisabled::<Identity, Impl, OFFSET>,
            AccountExpirationDate: AccountExpirationDate::<Identity, Impl, OFFSET>,
            SetAccountExpirationDate: SetAccountExpirationDate::<Identity, Impl, OFFSET>,
            GraceLoginsAllowed: GraceLoginsAllowed::<Identity, Impl, OFFSET>,
            SetGraceLoginsAllowed: SetGraceLoginsAllowed::<Identity, Impl, OFFSET>,
            GraceLoginsRemaining: GraceLoginsRemaining::<Identity, Impl, OFFSET>,
            SetGraceLoginsRemaining: SetGraceLoginsRemaining::<Identity, Impl, OFFSET>,
            IsAccountLocked: IsAccountLocked::<Identity, Impl, OFFSET>,
            SetIsAccountLocked: SetIsAccountLocked::<Identity, Impl, OFFSET>,
            LoginHours: LoginHours::<Identity, Impl, OFFSET>,
            SetLoginHours: SetLoginHours::<Identity, Impl, OFFSET>,
            LoginWorkstations: LoginWorkstations::<Identity, Impl, OFFSET>,
            SetLoginWorkstations: SetLoginWorkstations::<Identity, Impl, OFFSET>,
            MaxLogins: MaxLogins::<Identity, Impl, OFFSET>,
            SetMaxLogins: SetMaxLogins::<Identity, Impl, OFFSET>,
            MaxStorage: MaxStorage::<Identity, Impl, OFFSET>,
            SetMaxStorage: SetMaxStorage::<Identity, Impl, OFFSET>,
            PasswordExpirationDate: PasswordExpirationDate::<Identity, Impl, OFFSET>,
            SetPasswordExpirationDate: SetPasswordExpirationDate::<Identity, Impl, OFFSET>,
            PasswordMinimumLength: PasswordMinimumLength::<Identity, Impl, OFFSET>,
            SetPasswordMinimumLength: SetPasswordMinimumLength::<Identity, Impl, OFFSET>,
            PasswordRequired: PasswordRequired::<Identity, Impl, OFFSET>,
            SetPasswordRequired: SetPasswordRequired::<Identity, Impl, OFFSET>,
            RequireUniquePassword: RequireUniquePassword::<Identity, Impl, OFFSET>,
            SetRequireUniquePassword: SetRequireUniquePassword::<Identity, Impl, OFFSET>,
            EmailAddress: EmailAddress::<Identity, Impl, OFFSET>,
            SetEmailAddress: SetEmailAddress::<Identity, Impl, OFFSET>,
            HomeDirectory: HomeDirectory::<Identity, Impl, OFFSET>,
            SetHomeDirectory: SetHomeDirectory::<Identity, Impl, OFFSET>,
            Languages: Languages::<Identity, Impl, OFFSET>,
            SetLanguages: SetLanguages::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            SetProfile: SetProfile::<Identity, Impl, OFFSET>,
            LoginScript: LoginScript::<Identity, Impl, OFFSET>,
            SetLoginScript: SetLoginScript::<Identity, Impl, OFFSET>,
            Picture: Picture::<Identity, Impl, OFFSET>,
            SetPicture: SetPicture::<Identity, Impl, OFFSET>,
            HomePage: HomePage::<Identity, Impl, OFFSET>,
            SetHomePage: SetHomePage::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            ChangePassword: ChangePassword::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsWinNTSystemInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn UserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ComputerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DomainName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PDC(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IADsWinNTSystemInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADsWinNTSystemInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DomainName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DomainName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADsWinNTSystemInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IADsWinNTSystemInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UserName: UserName::<Identity, Impl, OFFSET>,
            ComputerName: ComputerName::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
            PDC: PDC::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ICommonQuery_Impl: ::windows_core::BaseImpl {
    fn OpenQueryWindow(this: &Self::This, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for ICommonQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommonQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenQueryWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenQueryWindow(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pquerywnd), ::core::mem::transmute_copy(&ppdataobject)).into())
        }
        ICommonQuery_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenQueryWindow: OpenQueryWindow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDirectoryObject_Impl: ::windows_core::BaseImpl {
    fn GetObjectInformation(this: &Self::This) -> ::windows_core::Result<*mut ADS_OBJECT_INFO>;
    fn GetObjectAttributes(this: &Self::This, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn SetObjectAttributes(this: &Self::This, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows_core::Result<u32>;
    fn CreateDSObject(this: &Self::This, pszrdnname: &::windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn DeleteDSObject(this: &Self::This, pszrdnname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IDirectoryObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectoryObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectAttributes(this, ::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes), ::core::mem::transmute_copy(&ppattributeentries), ::core::mem::transmute_copy(&pdwnumattributesreturned)).into())
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetObjectAttributes(this, ::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwnumattributesmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDSObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrdnname: ::windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDSObject(this, ::core::mem::transmute(&pszrdnname), ::core::mem::transmute_copy(&pattributeentries), ::core::mem::transmute_copy(&dwnumattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteDSObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrdnname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDSObject(this, ::core::mem::transmute(&pszrdnname)).into())
        }
        IDirectoryObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectInformation: GetObjectInformation::<Identity, Impl, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, Impl, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, Impl, OFFSET>,
            CreateDSObject: CreateDSObject::<Identity, Impl, OFFSET>,
            DeleteDSObject: DeleteDSObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySchemaMgmt_Impl: ::windows_core::BaseImpl {
    fn EnumAttributes(this: &Self::This, ppszattrnames: *const ::windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows_core::Result<()>;
    fn CreateAttributeDefinition(this: &Self::This, pszattributename: &::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::Result<()>;
    fn WriteAttributeDefinition(this: &Self::This, pszattributename: &::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::Result<()>;
    fn DeleteAttributeDefinition(this: &Self::This, pszattributename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumClasses(this: &Self::This, ppszclassnames: *const ::windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows_core::Result<()>;
    fn WriteClassDefinition(this: &Self::This, pszclassname: &::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::Result<()>;
    fn CreateClassDefinition(this: &Self::This, pszclassname: &::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::Result<()>;
    fn DeleteClassDefinition(this: &Self::This, pszclassname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectorySchemaMgmt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectorySchemaMgmt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszattrnames: *const ::windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumAttributes(this, ::core::mem::transmute_copy(&ppszattrnames), ::core::mem::transmute_copy(&dwnumattributes), ::core::mem::transmute_copy(&ppattrdefinition), ::core::mem::transmute_copy(&pdwnumattributes)).into())
        }
        unsafe extern "system" fn CreateAttributeDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAttributeDefinition(this, ::core::mem::transmute(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into())
        }
        unsafe extern "system" fn WriteAttributeDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteAttributeDefinition(this, ::core::mem::transmute(&pszattributename), ::core::mem::transmute_copy(&pattributedefinition)).into())
        }
        unsafe extern "system" fn DeleteAttributeDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAttributeDefinition(this, ::core::mem::transmute(&pszattributename)).into())
        }
        unsafe extern "system" fn EnumClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszclassnames: *const ::windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumClasses(this, ::core::mem::transmute_copy(&ppszclassnames), ::core::mem::transmute_copy(&dwnumclasses), ::core::mem::transmute_copy(&ppclassdefinition), ::core::mem::transmute_copy(&pdwnumclasses)).into())
        }
        unsafe extern "system" fn WriteClassDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteClassDefinition(this, ::core::mem::transmute(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into())
        }
        unsafe extern "system" fn CreateClassDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateClassDefinition(this, ::core::mem::transmute(&pszclassname), ::core::mem::transmute_copy(&pclassdefinition)).into())
        }
        unsafe extern "system" fn DeleteClassDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySchemaMgmt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteClassDefinition(this, ::core::mem::transmute(&pszclassname)).into())
        }
        IDirectorySchemaMgmt_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumAttributes: EnumAttributes::<Identity, Impl, OFFSET>,
            CreateAttributeDefinition: CreateAttributeDefinition::<Identity, Impl, OFFSET>,
            WriteAttributeDefinition: WriteAttributeDefinition::<Identity, Impl, OFFSET>,
            DeleteAttributeDefinition: DeleteAttributeDefinition::<Identity, Impl, OFFSET>,
            EnumClasses: EnumClasses::<Identity, Impl, OFFSET>,
            WriteClassDefinition: WriteClassDefinition::<Identity, Impl, OFFSET>,
            CreateClassDefinition: CreateClassDefinition::<Identity, Impl, OFFSET>,
            DeleteClassDefinition: DeleteClassDefinition::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectorySearch_Impl: ::windows_core::BaseImpl {
    fn SetSearchPreference(this: &Self::This, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows_core::Result<()>;
    fn ExecuteSearch(this: &Self::This, pszsearchfilter: &::windows_core::PCWSTR, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32) -> ::windows_core::Result<ADS_SEARCH_HANDLE>;
    fn AbandonSearch(this: &Self::This, phsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::Result<()>;
    fn GetFirstRow(this: &Self::This, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT;
    fn GetNextRow(this: &Self::This, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT;
    fn GetPreviousRow(this: &Self::This, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT;
    fn GetNextColumnName(this: &Self::This, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
    fn GetColumn(this: &Self::This, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: &::windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows_core::Result<()>;
    fn FreeColumn(this: &Self::This, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows_core::Result<()>;
    fn CloseSearchHandle(this: &Self::This, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectorySearch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectorySearch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSearchPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSearchPreference(this, ::core::mem::transmute_copy(&psearchprefs), ::core::mem::transmute_copy(&dwnumprefs)).into())
        }
        unsafe extern "system" fn ExecuteSearch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsearchfilter: ::windows_core::PCWSTR, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, phsearchresult: *mut ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecuteSearch(this, ::core::mem::transmute(&pszsearchfilter), ::core::mem::transmute_copy(&pattributenames), ::core::mem::transmute_copy(&dwnumberattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phsearchresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AbandonSearch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbandonSearch(this, ::core::mem::transmute_copy(&phsearchresult)).into())
        }
        unsafe extern "system" fn GetFirstRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFirstRow(this, ::core::mem::transmute_copy(&hsearchresult)))
        }
        unsafe extern "system" fn GetNextRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextRow(this, ::core::mem::transmute_copy(&hsearchresult)))
        }
        unsafe extern "system" fn GetPreviousRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPreviousRow(this, ::core::mem::transmute_copy(&hsearchresult)))
        }
        unsafe extern "system" fn GetNextColumnName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextColumnName(this, ::core::mem::transmute_copy(&hsearchhandle), ::core::mem::transmute_copy(&ppszcolumnname)))
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: ::windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumn(this, ::core::mem::transmute_copy(&hsearchresult), ::core::mem::transmute(&szcolumnname), ::core::mem::transmute_copy(&psearchcolumn)).into())
        }
        unsafe extern "system" fn FreeColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeColumn(this, ::core::mem::transmute_copy(&psearchcolumn)).into())
        }
        unsafe extern "system" fn CloseSearchHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectorySearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseSearchHandle(this, ::core::mem::transmute_copy(&hsearchresult)).into())
        }
        IDirectorySearch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSearchPreference: SetSearchPreference::<Identity, Impl, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, Impl, OFFSET>,
            AbandonSearch: AbandonSearch::<Identity, Impl, OFFSET>,
            GetFirstRow: GetFirstRow::<Identity, Impl, OFFSET>,
            GetNextRow: GetNextRow::<Identity, Impl, OFFSET>,
            GetPreviousRow: GetPreviousRow::<Identity, Impl, OFFSET>,
            GetNextColumnName: GetNextColumnName::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            FreeColumn: FreeColumn::<Identity, Impl, OFFSET>,
            CloseSearchHandle: CloseSearchHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsAdminCreateObj_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, padscontainerobj: ::core::option::Option<&IADsContainer>, padscopysource: ::core::option::Option<&IADs>, lpszclassname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CreateModal(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<IADs>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IDsAdminCreateObj {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsAdminCreateObj {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&padscontainerobj), ::windows_core::from_raw_borrowed(&padscopysource), ::core::mem::transmute(&lpszclassname)).into())
        }
        unsafe extern "system" fn CreateModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminCreateObj_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateModal(this, ::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadsobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDsAdminCreateObj_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateModal: CreateModal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDsAdminNewObj_Impl: ::windows_core::BaseImpl {
    fn SetButtons(this: &Self::This, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPageCounts(this: &Self::This, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDsAdminNewObj {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsAdminNewObj {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetButtons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetButtons(this, ::core::mem::transmute_copy(&ncurrindex), ::core::mem::transmute_copy(&bvalid)).into())
        }
        unsafe extern "system" fn GetPageCounts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObj_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPageCounts(this, ::core::mem::transmute_copy(&pntotal), ::core::mem::transmute_copy(&pnstartindex)).into())
        }
        IDsAdminNewObj_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetButtons: SetButtons::<Identity, Impl, OFFSET>,
            GetPageCounts: GetPageCounts::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsAdminNewObjExt_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, padscontainerobj: ::core::option::Option<&IADsContainer>, padscopysource: ::core::option::Option<&IADs>, lpszclassname: &::windows_core::PCWSTR, pdsadminnewobj: ::core::option::Option<&IDsAdminNewObj>, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows_core::Result<()>;
    fn AddPages(this: &Self::This, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetObject(this: &Self::This, padsobj: ::core::option::Option<&IADs>) -> ::windows_core::Result<()>;
    fn WriteData(this: &Self::This, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows_core::Result<()>;
    fn OnError(this: &Self::This, hwnd: super::super::Foundation::HWND, hr: ::windows_core::HRESULT, ucontext: u32) -> ::windows_core::Result<()>;
    fn GetSummaryInfo(this: &Self::This, pbstrtext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IDsAdminNewObjExt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsAdminNewObjExt {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows_core::PCWSTR, pdsadminnewobj: *mut ::core::ffi::c_void, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&padscontainerobj), ::windows_core::from_raw_borrowed(&padscopysource), ::core::mem::transmute(&lpszclassname), ::windows_core::from_raw_borrowed(&pdsadminnewobj), ::core::mem::transmute_copy(&pdispinfo)).into())
        }
        unsafe extern "system" fn AddPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPages(this, ::core::mem::transmute_copy(&lpfnaddpage), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn SetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padsobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObject(this, ::windows_core::from_raw_borrowed(&padsobj)).into())
        }
        unsafe extern "system" fn WriteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteData(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ucontext)).into())
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: ::windows_core::HRESULT, ucontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnError(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&ucontext)).into())
        }
        unsafe extern "system" fn GetSummaryInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjExt_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSummaryInfo(this, ::core::mem::transmute_copy(&pbstrtext)).into())
        }
        IDsAdminNewObjExt_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
            SetObject: SetObject::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
            GetSummaryInfo: GetSummaryInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDsAdminNewObjPrimarySite_Impl: ::windows_core::BaseImpl {
    fn CreateNew(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDsAdminNewObjPrimarySite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsAdminNewObjPrimarySite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNew(this, ::core::mem::transmute(&pszname)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNewObjPrimarySite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        IDsAdminNewObjPrimarySite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateNew: CreateNew::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDsAdminNotifyHandler_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pextrainfo: ::core::option::Option<&super::super::System::Com::IDataObject>, pueventflags: *mut u32) -> ::windows_core::Result<()>;
    fn Begin(this: &Self::This, uevent: u32, parg1: ::core::option::Option<&super::super::System::Com::IDataObject>, parg2: ::core::option::Option<&super::super::System::Com::IDataObject>, puflags: *mut u32, pbstr: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Notify(this: &Self::This, nitem: u32, uflags: u32) -> ::windows_core::Result<()>;
    fn End(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDsAdminNotifyHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsAdminNotifyHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextrainfo: *mut ::core::ffi::c_void, pueventflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pextrainfo), ::core::mem::transmute_copy(&pueventflags)).into())
        }
        unsafe extern "system" fn Begin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uevent: u32, parg1: *mut ::core::ffi::c_void, parg2: *mut ::core::ffi::c_void, puflags: *mut u32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin(this, ::core::mem::transmute_copy(&uevent), ::windows_core::from_raw_borrowed(&parg1), ::windows_core::from_raw_borrowed(&parg2), ::core::mem::transmute_copy(&puflags), ::core::mem::transmute_copy(&pbstr)).into())
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitem: u32, uflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&uflags)).into())
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsAdminNotifyHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this).into())
        }
        IDsAdminNotifyHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDsBrowseDomainTree_Impl: ::windows_core::BaseImpl {
    fn BrowseTo(this: &Self::This, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut ::windows_core::PWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetDomains(this: &Self::This, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows_core::Result<()>;
    fn FreeDomains(this: &Self::This, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows_core::Result<()>;
    fn FlushCachedDomains(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetComputer(this: &Self::This, pszcomputername: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDsBrowseDomainTree {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsBrowseDomainTree {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BrowseTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut ::windows_core::PWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BrowseTo(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ppsztargetpath), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetDomains<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDomains(this, ::core::mem::transmute_copy(&ppdomaintree), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn FreeDomains<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeDomains(this, ::core::mem::transmute_copy(&ppdomaintree)).into())
        }
        unsafe extern "system" fn FlushCachedDomains<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushCachedDomains(this).into())
        }
        unsafe extern "system" fn SetComputer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsBrowseDomainTree_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcomputername: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputer(this, ::core::mem::transmute(&pszcomputername), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword)).into())
        }
        IDsBrowseDomainTree_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BrowseTo: BrowseTo::<Identity, Impl, OFFSET>,
            GetDomains: GetDomains::<Identity, Impl, OFFSET>,
            FreeDomains: FreeDomains::<Identity, Impl, OFFSET>,
            FlushCachedDomains: FlushCachedDomains::<Identity, Impl, OFFSET>,
            SetComputer: SetComputer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDsDisplaySpecifier_Impl: ::windows_core::BaseImpl {
    fn SetServer(this: &Self::This, pszserver: &::windows_core::PCWSTR, pszusername: &::windows_core::PCWSTR, pszpassword: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn SetLanguageID(this: &Self::This, langid: u16) -> ::windows_core::Result<()>;
    fn GetDisplaySpecifier(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetIconLocation(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, dwflags: u32, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows_core::Result<()>;
    fn GetIcon(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON;
    fn GetFriendlyClassName(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::Result<()>;
    fn GetFriendlyAttributeName(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, pszattributename: &::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: u32) -> ::windows_core::Result<()>;
    fn IsClassContainer(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, pszadspath: &::windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL;
    fn GetClassCreationInfo(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows_core::Result<()>;
    fn EnumClassAttributes(this: &Self::This, pszobjectclass: &::windows_core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn GetAttributeADsType(this: &Self::This, pszattributename: &::windows_core::PCWSTR) -> ADSTYPE;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IDsDisplaySpecifier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsDisplaySpecifier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserver: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServer(this, ::core::mem::transmute(&pszserver), ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn SetLanguageID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguageID(this, ::core::mem::transmute_copy(&langid)).into())
        }
        unsafe extern "system" fn GetDisplaySpecifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplaySpecifier(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetIconLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, dwflags: u32, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIconLocation(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer), ::core::mem::transmute_copy(&presid)).into())
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIcon(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cxicon), ::core::mem::transmute_copy(&cyicon)))
        }
        unsafe extern "system" fn GetFriendlyClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFriendlyClassName(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into())
        }
        unsafe extern "system" fn GetFriendlyAttributeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszattributename: ::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFriendlyAttributeName(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute(&pszattributename), ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchbuffer)).into())
        }
        unsafe extern "system" fn IsClassContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszadspath: ::windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsClassContainer(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute(&pszadspath), ::core::mem::transmute_copy(&dwflags)))
        }
        unsafe extern "system" fn GetClassCreationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClassCreationInfo(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&ppdscci)).into())
        }
        unsafe extern "system" fn EnumClassAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumClassAttributes(this, ::core::mem::transmute(&pszobjectclass), ::core::mem::transmute_copy(&pcbenum), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn GetAttributeADsType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsDisplaySpecifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR) -> ADSTYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeADsType(this, ::core::mem::transmute(&pszattributename)))
        }
        IDsDisplaySpecifier_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, Impl, OFFSET>,
            GetDisplaySpecifier: GetDisplaySpecifier::<Identity, Impl, OFFSET>,
            GetIconLocation: GetIconLocation::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetFriendlyClassName: GetFriendlyClassName::<Identity, Impl, OFFSET>,
            GetFriendlyAttributeName: GetFriendlyAttributeName::<Identity, Impl, OFFSET>,
            IsClassContainer: IsClassContainer::<Identity, Impl, OFFSET>,
            GetClassCreationInfo: GetClassCreationInfo::<Identity, Impl, OFFSET>,
            EnumClassAttributes: EnumClassAttributes::<Identity, Impl, OFFSET>,
            GetAttributeADsType: GetAttributeADsType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPicker_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows_core::Result<()>;
    fn InvokeDialog(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IDsObjectPicker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsObjectPicker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&pinitinfo)).into())
        }
        unsafe extern "system" fn InvokeDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsObjectPicker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InvokeDialog(this, ::core::mem::transmute_copy(&hwndparent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdoselections, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDsObjectPicker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            InvokeDialog: InvokeDialog::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDsObjectPickerCredentials_Impl: ::windows_core::BaseImpl + IDsObjectPicker_Impl {
    fn SetCredentials(this: &Self::This, szusername: &::windows_core::PCWSTR, szpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IDsObjectPickerCredentials {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDsObjectPicker);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDsObjectPickerCredentials {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDsObjectPickerCredentials_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szusername: ::windows_core::PCWSTR, szpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentials(this, ::core::mem::transmute(&szusername), ::core::mem::transmute(&szpassword)).into())
        }
        IDsObjectPickerCredentials_Vtbl { base__: <IDsObjectPicker as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetCredentials: SetCredentials::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistQuery_Impl: ::windows_core::BaseImpl + super::super::System::Com::IPersist_Impl {
    fn WriteString(this: &Self::This, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReadString(this: &Self::This, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::Result<()>;
    fn WriteInt(this: &Self::This, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, value: i32) -> ::windows_core::Result<()>;
    fn ReadInt(this: &Self::This, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pvalue: *mut i32) -> ::windows_core::Result<()>;
    fn WriteStruct(this: &Self::This, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::Result<()>;
    fn ReadStruct(this: &Self::This, psection: &::windows_core::PCWSTR, pvaluename: &::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IPersistQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IPersist);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteString(this, ::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute(&pvalue)).into())
        }
        unsafe extern "system" fn ReadString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadString(this, ::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cchbuffer)).into())
        }
        unsafe extern "system" fn WriteInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteInt(this, ::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn ReadInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadInt(this, ::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn WriteStruct<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStruct(this, ::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into())
        }
        unsafe extern "system" fn ReadStruct<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadStruct(this, ::core::mem::transmute(&psection), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&pstruct), ::core::mem::transmute_copy(&cbstruct)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IPersistQuery_Vtbl {
            base__: <super::super::System::Com::IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            ReadString: ReadString::<Identity, Impl, OFFSET>,
            WriteInt: WriteInt::<Identity, Impl, OFFSET>,
            ReadInt: ReadInt::<Identity, Impl, OFFSET>,
            WriteStruct: WriteStruct::<Identity, Impl, OFFSET>,
            ReadStruct: ReadStruct::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrivateDispatch_Impl: ::windows_core::BaseImpl {
    fn ADSIInitializeDispatchManager(this: &Self::This, dwextensionid: i32) -> ::windows_core::Result<()>;
    fn ADSIGetTypeInfoCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ADSIGetTypeInfo(this: &Self::This, itinfo: u32, lcid: u32) -> ::windows_core::Result<super::super::System::Com::ITypeInfo>;
    fn ADSIGetIDsOfNames(this: &Self::This, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows_core::Result<i32>;
    fn ADSIInvoke(this: &Self::This, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrivateDispatch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrivateDispatch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ADSIInitializeDispatchManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwextensionid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ADSIInitializeDispatchManager(this, ::core::mem::transmute_copy(&dwextensionid)).into())
        }
        unsafe extern "system" fn ADSIGetTypeInfoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADSIGetTypeInfoCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ADSIGetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADSIGetTypeInfo(this, ::core::mem::transmute_copy(&itinfo), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ADSIGetIDsOfNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ADSIGetIDsOfNames(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rgdispid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ADSIInvoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateDispatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ADSIInvoke(this, ::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into())
        }
        IPrivateDispatch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ADSIInitializeDispatchManager: ADSIInitializeDispatchManager::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfoCount: ADSIGetTypeInfoCount::<Identity, Impl, OFFSET>,
            ADSIGetTypeInfo: ADSIGetTypeInfo::<Identity, Impl, OFFSET>,
            ADSIGetIDsOfNames: ADSIGetIDsOfNames::<Identity, Impl, OFFSET>,
            ADSIInvoke: ADSIInvoke::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPrivateUnknown_Impl: ::windows_core::BaseImpl {
    fn ADSIInitializeObject(this: &Self::This, lpszusername: &::windows_core::BSTR, lpszpassword: &::windows_core::BSTR, lnreserved: i32) -> ::windows_core::Result<()>;
    fn ADSIReleaseObject(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrivateUnknown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrivateUnknown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ADSIInitializeObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnreserved: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ADSIInitializeObject(this, ::core::mem::transmute(&lpszusername), ::core::mem::transmute(&lpszpassword), ::core::mem::transmute_copy(&lnreserved)).into())
        }
        unsafe extern "system" fn ADSIReleaseObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrivateUnknown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ADSIReleaseObject(this).into())
        }
        IPrivateUnknown_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ADSIInitializeObject: ADSIInitializeObject::<Identity, Impl, OFFSET>,
            ADSIReleaseObject: ADSIReleaseObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IQueryForm_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, hkform: super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
    fn AddForms(this: &Self::This, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn AddPages(this: &Self::This, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IQueryForm {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQueryForm {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&hkform)).into())
        }
        unsafe extern "system" fn AddForms<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddForms(this, ::core::mem::transmute_copy(&paddformsproc), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn AddPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryForm_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPages(this, ::core::mem::transmute_copy(&paddpagesproc), ::core::mem::transmute_copy(&lparam)).into())
        }
        IQueryForm_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            AddForms: AddForms::<Identity, Impl, OFFSET>,
            AddPages: AddPages::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
