#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEffectivePermission_Impl: ::windows_core::BaseImpl {
    fn GetEffectivePermission(this: &Self::This, pguidobjecttype: *const ::windows_core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: &::windows_core::PCWSTR, psd: super::super::PSECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEffectivePermission {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEffectivePermission_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEffectivePermission {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEffectivePermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEffectivePermission_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows_core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: ::windows_core::PCWSTR, psd: super::super::PSECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectivePermission(this, ::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&pusersid), ::core::mem::transmute(&pszservername), ::core::mem::transmute_copy(&psd), ::core::mem::transmute_copy(&ppobjecttypelist), ::core::mem::transmute_copy(&pcobjecttypelistlength), ::core::mem::transmute_copy(&ppgrantedaccesslist), ::core::mem::transmute_copy(&pcgrantedaccesslistlength)).into())
        }
        IEffectivePermission_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEffectivePermission: GetEffectivePermission::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEffectivePermission2_Impl: ::windows_core::BaseImpl {
    fn ComputeEffectivePermissionWithSecondarySecurity(
        this: &Self::This,
        psid: super::super::super::Foundation::PSID,
        pdevicesid: super::super::super::Foundation::PSID,
        pszservername: &::windows_core::PCWSTR,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: *const super::super::TOKEN_GROUPS,
        pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pdevicegroups: *const super::super::TOKEN_GROUPS,
        pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEffectivePermission2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEffectivePermission2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEffectivePermission2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ComputeEffectivePermissionWithSecondarySecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEffectivePermission2_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            psid: super::super::super::Foundation::PSID,
            pdevicesid: super::super::super::Foundation::PSID,
            pszservername: ::windows_core::PCWSTR,
            psecurityobjects: *mut SECURITY_OBJECT,
            dwsecurityobjectcount: u32,
            pusergroups: *const super::super::TOKEN_GROUPS,
            pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pdevicegroups: *const super::super::TOKEN_GROUPS,
            pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
            pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
            pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
            peffpermresultlists: *mut EFFPERM_RESULT_LIST,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::ComputeEffectivePermissionWithSecondarySecurity(
                    this,
                    ::core::mem::transmute_copy(&psid),
                    ::core::mem::transmute_copy(&pdevicesid),
                    ::core::mem::transmute(&pszservername),
                    ::core::mem::transmute_copy(&psecurityobjects),
                    ::core::mem::transmute_copy(&dwsecurityobjectcount),
                    ::core::mem::transmute_copy(&pusergroups),
                    ::core::mem::transmute_copy(&pauthzusergroupsoperations),
                    ::core::mem::transmute_copy(&pdevicegroups),
                    ::core::mem::transmute_copy(&pauthzdevicegroupsoperations),
                    ::core::mem::transmute_copy(&pauthzuserclaims),
                    ::core::mem::transmute_copy(&pauthzuserclaimsoperations),
                    ::core::mem::transmute_copy(&pauthzdeviceclaims),
                    ::core::mem::transmute_copy(&pauthzdeviceclaimsoperations),
                    ::core::mem::transmute_copy(&peffpermresultlists),
                )
                .into()
            })
        }
        IEffectivePermission2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ComputeEffectivePermissionWithSecondarySecurity: ComputeEffectivePermissionWithSecondarySecurity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait ISecurityInformation_Impl: ::windows_core::BaseImpl {
    fn GetObjectInformation(this: &Self::This, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows_core::Result<()>;
    fn GetSecurity(this: &Self::This, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetSecurity(this: &Self::This, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::PSECURITY_DESCRIPTOR) -> ::windows_core::Result<()>;
    fn GetAccessRights(this: &Self::This, pguidobjecttype: *const ::windows_core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows_core::Result<()>;
    fn MapGeneric(this: &Self::This, pguidobjecttype: *const ::windows_core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows_core::Result<()>;
    fn GetInheritTypes(this: &Self::This, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows_core::Result<()>;
    fn PropertySheetPageCallback(this: &Self::This, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::windows_core::Iids for ISecurityInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInformation(this, ::core::mem::transmute_copy(&pobjectinfo)).into())
        }
        unsafe extern "system" fn GetSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSecurity(this, ::core::mem::transmute_copy(&requestedinformation), ::core::mem::transmute_copy(&ppsecuritydescriptor), ::core::mem::transmute_copy(&fdefault)).into())
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::PSECURITY_DESCRIPTOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurity(this, ::core::mem::transmute_copy(&securityinformation), ::core::mem::transmute_copy(&psecuritydescriptor)).into())
        }
        unsafe extern "system" fn GetAccessRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows_core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAccessRights(this, ::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaccess), ::core::mem::transmute_copy(&pcaccesses), ::core::mem::transmute_copy(&pidefaultaccess)).into())
        }
        unsafe extern "system" fn MapGeneric<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows_core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapGeneric(this, ::core::mem::transmute_copy(&pguidobjecttype), ::core::mem::transmute_copy(&paceflags), ::core::mem::transmute_copy(&pmask)).into())
        }
        unsafe extern "system" fn GetInheritTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInheritTypes(this, ::core::mem::transmute_copy(&ppinherittypes), ::core::mem::transmute_copy(&pcinherittypes)).into())
        }
        unsafe extern "system" fn PropertySheetPageCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PropertySheetPageCallback(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&upage)).into())
        }
        ISecurityInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectInformation: GetObjectInformation::<Identity, Impl, OFFSET>,
            GetSecurity: GetSecurity::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            GetAccessRights: GetAccessRights::<Identity, Impl, OFFSET>,
            MapGeneric: MapGeneric::<Identity, Impl, OFFSET>,
            GetInheritTypes: GetInheritTypes::<Identity, Impl, OFFSET>,
            PropertySheetPageCallback: PropertySheetPageCallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISecurityInformation2_Impl: ::windows_core::BaseImpl {
    fn IsDaclCanonical(this: &Self::This, pdacl: *const super::super::ACL) -> super::super::super::Foundation::BOOL;
    fn LookupSids(this: &Self::This, csids: u32, rgpsids: *const super::super::super::Foundation::PSID) -> ::windows_core::Result<super::super::super::System::Com::IDataObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISecurityInformation2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityInformation2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDaclCanonical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdacl: *const super::super::ACL) -> super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDaclCanonical(this, ::core::mem::transmute_copy(&pdacl)))
        }
        unsafe extern "system" fn LookupSids<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csids: u32, rgpsids: *const super::super::super::Foundation::PSID, ppdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupSids(this, ::core::mem::transmute_copy(&csids), ::core::mem::transmute_copy(&rgpsids)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISecurityInformation2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDaclCanonical: IsDaclCanonical::<Identity, Impl, OFFSET>,
            LookupSids: LookupSids::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityInformation3_Impl: ::windows_core::BaseImpl {
    fn GetFullResourceName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn OpenElevatedEditor(this: &Self::This, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISecurityInformation3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityInformation3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFullResourceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszresourcename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFullResourceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszresourcename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenElevatedEditor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenElevatedEditor(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&upage)).into())
        }
        ISecurityInformation3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFullResourceName: GetFullResourceName::<Identity, Impl, OFFSET>,
            OpenElevatedEditor: OpenElevatedEditor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityInformation4_Impl: ::windows_core::BaseImpl {
    fn GetSecondarySecurity(this: &Self::This, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISecurityInformation4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityInformation4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSecondarySecurity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInformation4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSecondarySecurity(this, ::core::mem::transmute_copy(&psecurityobjects), ::core::mem::transmute_copy(&psecurityobjectcount)).into())
        }
        ISecurityInformation4_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSecondarySecurity: GetSecondarySecurity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISecurityObjectTypeInfo_Impl: ::windows_core::BaseImpl {
    fn GetInheritSource(this: &Self::This, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISecurityObjectTypeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityObjectTypeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityObjectTypeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInheritSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityObjectTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInheritSource(this, ::core::mem::transmute_copy(&si), ::core::mem::transmute_copy(&pacl), ::core::mem::transmute_copy(&ppinheritarray)).into())
        }
        ISecurityObjectTypeInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInheritSource: GetInheritSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
