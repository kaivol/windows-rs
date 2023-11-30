#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSCDefaultProduct_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn SetDefaultProduct(this: &Self::This, etype: SECURITY_PRODUCT_TYPE, pguid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSCDefaultProduct {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSCDefaultProduct_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSCDefaultProduct {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDefaultProduct<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSCDefaultProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultProduct(this, ::core::mem::transmute_copy(&etype), ::core::mem::transmute(&pguid)).into())
        }
        IWSCDefaultProduct_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDefaultProduct: SetDefaultProduct::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSCProductList_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Initialize(this: &Self::This, provider: &WSC_SECURITY_PROVIDER) -> ::windows_core::Result<()>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, index: u32) -> ::windows_core::Result<IWscProduct>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWSCProductList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSCProductList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&provider)).into())
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSCProductList_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWscProduct_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ProductName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductState(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_STATE>;
    fn SignatureStatus(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_SIGNATURE_STATUS>;
    fn RemediationPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductStateTimestamp(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductIsDefault(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWscProduct {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWscProduct {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProductName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProductState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SignatureStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SignatureStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemediationPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemediationPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProductStateTimestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductStateTimestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProductGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProductIsDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductIsDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWscProduct_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProductName: ProductName::<Identity, Impl, OFFSET>,
            ProductState: ProductState::<Identity, Impl, OFFSET>,
            SignatureStatus: SignatureStatus::<Identity, Impl, OFFSET>,
            RemediationPath: RemediationPath::<Identity, Impl, OFFSET>,
            ProductStateTimestamp: ProductStateTimestamp::<Identity, Impl, OFFSET>,
            ProductGuid: ProductGuid::<Identity, Impl, OFFSET>,
            ProductIsDefault: ProductIsDefault::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWscProduct2_Impl: ::windows_core::BaseImpl + IWscProduct_Impl {
    fn AntivirusScanSubstatus(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusSettingsSubstatus(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusProtectionUpdateSubstatus(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallDomainProfileSubstatus(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPrivateProfileSubstatus(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPublicProfileSubstatus(this: &Self::This) -> ::windows_core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWscProduct2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWscProduct);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWscProduct2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AntivirusScanSubstatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AntivirusScanSubstatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AntivirusSettingsSubstatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AntivirusSettingsSubstatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AntivirusProtectionUpdateSubstatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AntivirusProtectionUpdateSubstatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirewallDomainProfileSubstatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirewallDomainProfileSubstatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirewallPrivateProfileSubstatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirewallPrivateProfileSubstatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirewallPublicProfileSubstatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirewallPublicProfileSubstatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWscProduct2_Vtbl {
            base__: <IWscProduct as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AntivirusScanSubstatus: AntivirusScanSubstatus::<Identity, Impl, OFFSET>,
            AntivirusSettingsSubstatus: AntivirusSettingsSubstatus::<Identity, Impl, OFFSET>,
            AntivirusProtectionUpdateSubstatus: AntivirusProtectionUpdateSubstatus::<Identity, Impl, OFFSET>,
            FirewallDomainProfileSubstatus: FirewallDomainProfileSubstatus::<Identity, Impl, OFFSET>,
            FirewallPrivateProfileSubstatus: FirewallPrivateProfileSubstatus::<Identity, Impl, OFFSET>,
            FirewallPublicProfileSubstatus: FirewallPublicProfileSubstatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWscProduct3_Impl: ::windows_core::BaseImpl + IWscProduct2_Impl {
    fn AntivirusDaysUntilExpired(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWscProduct3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWscProduct2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWscProduct3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AntivirusDaysUntilExpired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWscProduct3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AntivirusDaysUntilExpired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdays, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWscProduct3_Vtbl {
            base__: <IWscProduct2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AntivirusDaysUntilExpired: AntivirusDaysUntilExpired::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
