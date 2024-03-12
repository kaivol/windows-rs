#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertSrvSetup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CAErrorId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CAErrorString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InitializeDefaults(this: &Self::This, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetCASetupProperty(this: &Self::This, propertyid: CASetupProperty) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetCASetupProperty(this: &Self::This, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn IsPropertyEditable(this: &Self::This, propertyid: CASetupProperty) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSupportedCATypes(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetProviderNameList(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetKeyLengthList(this: &Self::This, bstrprovidername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetHashAlgorithmList(this: &Self::This, bstrprovidername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetPrivateKeyContainerList(this: &Self::This, bstrprovidername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetExistingCACertificates(this: &Self::This) -> ::windows_core::Result<ICertSrvSetupKeyInformationCollection>;
    fn CAImportPFX(this: &Self::This, bstrfilename: &::windows_core::BSTR, bstrpasswd: &::windows_core::BSTR, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<ICertSrvSetupKeyInformation>;
    fn SetCADistinguishedName(this: &Self::This, bstrcadn: &::windows_core::BSTR, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetDatabaseInformation(this: &Self::This, bstrdbdirectory: &::windows_core::BSTR, bstrlogdirectory: &::windows_core::BSTR, bstrsharedfolder: &::windows_core::BSTR, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetParentCAInformation(this: &Self::This, bstrcaconfiguration: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetWebCAInformation(this: &Self::This, bstrcaconfiguration: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Install(this: &Self::This) -> ::windows_core::Result<()>;
    fn PreUnInstall(this: &Self::This, bclientonly: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn PostUnInstall(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICertSrvSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICertSrvSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CAErrorId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CAErrorId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CAErrorString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CAErrorString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeDefaults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bserver: super::super::Foundation::VARIANT_BOOL, bclient: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeDefaults(this, ::core::mem::transmute_copy(&bserver), ::core::mem::transmute_copy(&bclient)).into())
        }
        unsafe extern "system" fn GetCASetupProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCASetupProperty(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCASetupProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCASetupProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into())
        }
        unsafe extern "system" fn IsPropertyEditable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: CASetupProperty, pbeditable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPropertyEditable(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbeditable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedCATypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcatypes: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedCATypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderNameList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderNameList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKeyLengthList(this, ::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHashAlgorithmList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHashAlgorithmList(this, ::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrivateKeyContainerList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrivateKeyContainerList(this, ::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExistingCACertificates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExistingCACertificates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CAImportPFX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpasswd: ::std::mem::MaybeUninit<::windows_core::BSTR>, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CAImportPFX(this, ::core::mem::transmute(&bstrfilename), ::core::mem::transmute(&bstrpasswd), ::core::mem::transmute_copy(&boverwriteexistingkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCADistinguishedName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcadn: ::std::mem::MaybeUninit<::windows_core::BSTR>, bignoreunicode: super::super::Foundation::VARIANT_BOOL, boverwriteexistingkey: super::super::Foundation::VARIANT_BOOL, boverwriteexistingcainds: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCADistinguishedName(this, ::core::mem::transmute(&bstrcadn), ::core::mem::transmute_copy(&bignoreunicode), ::core::mem::transmute_copy(&boverwriteexistingkey), ::core::mem::transmute_copy(&boverwriteexistingcainds)).into())
        }
        unsafe extern "system" fn SetDatabaseInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdbdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlogdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsharedfolder: ::std::mem::MaybeUninit<::windows_core::BSTR>, bforceoverwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDatabaseInformation(this, ::core::mem::transmute(&bstrdbdirectory), ::core::mem::transmute(&bstrlogdirectory), ::core::mem::transmute(&bstrsharedfolder), ::core::mem::transmute_copy(&bforceoverwrite)).into())
        }
        unsafe extern "system" fn SetParentCAInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParentCAInformation(this, ::core::mem::transmute(&bstrcaconfiguration)).into())
        }
        unsafe extern "system" fn SetWebCAInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcaconfiguration: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWebCAInformation(this, ::core::mem::transmute(&bstrcaconfiguration)).into())
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Install(this).into())
        }
        unsafe extern "system" fn PreUnInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bclientonly: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreUnInstall(this, ::core::mem::transmute_copy(&bclientonly)).into())
        }
        unsafe extern "system" fn PostUnInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostUnInstall(this).into())
        }
        ICertSrvSetup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CAErrorId: CAErrorId::<Identity, Impl, OFFSET>,
            CAErrorString: CAErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetCASetupProperty: GetCASetupProperty::<Identity, Impl, OFFSET>,
            SetCASetupProperty: SetCASetupProperty::<Identity, Impl, OFFSET>,
            IsPropertyEditable: IsPropertyEditable::<Identity, Impl, OFFSET>,
            GetSupportedCATypes: GetSupportedCATypes::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            GetHashAlgorithmList: GetHashAlgorithmList::<Identity, Impl, OFFSET>,
            GetPrivateKeyContainerList: GetPrivateKeyContainerList::<Identity, Impl, OFFSET>,
            GetExistingCACertificates: GetExistingCACertificates::<Identity, Impl, OFFSET>,
            CAImportPFX: CAImportPFX::<Identity, Impl, OFFSET>,
            SetCADistinguishedName: SetCADistinguishedName::<Identity, Impl, OFFSET>,
            SetDatabaseInformation: SetDatabaseInformation::<Identity, Impl, OFFSET>,
            SetParentCAInformation: SetParentCAInformation::<Identity, Impl, OFFSET>,
            SetWebCAInformation: SetWebCAInformation::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertSrvSetupKeyInformation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetProviderName(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLength(this: &Self::This, lval: i32) -> ::windows_core::Result<()>;
    fn Existing(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetExisting(this: &Self::This, bval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ContainerName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetContainerName(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HashAlgorithm(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetHashAlgorithm(this: &Self::This, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExistingCACertificate(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExistingCACertificate(this: &Self::This, varval: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICertSrvSetupKeyInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICertSrvSetupKeyInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProviderName(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLength(this, ::core::mem::transmute_copy(&lval)).into())
        }
        unsafe extern "system" fn Existing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Existing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExisting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExisting(this, ::core::mem::transmute_copy(&bval)).into())
        }
        unsafe extern "system" fn ContainerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContainerName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContainerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContainerName(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HashAlgorithm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHashAlgorithm(this, ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn ExistingCACertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExistingCACertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExistingCACertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varval: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExistingCACertificate(this, ::core::mem::transmute(&varval)).into())
        }
        ICertSrvSetupKeyInformation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProviderName: ProviderName::<Identity, Impl, OFFSET>,
            SetProviderName: SetProviderName::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            Existing: Existing::<Identity, Impl, OFFSET>,
            SetExisting: SetExisting::<Identity, Impl, OFFSET>,
            ContainerName: ContainerName::<Identity, Impl, OFFSET>,
            SetContainerName: SetContainerName::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            ExistingCACertificate: ExistingCACertificate::<Identity, Impl, OFFSET>,
            SetExistingCACertificate: SetExistingCACertificate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertSrvSetupKeyInformationCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, pikeyinformation: ::core::option::Option<&ICertSrvSetupKeyInformation>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICertSrvSetupKeyInformationCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICertSrvSetupKeyInformationCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertSrvSetupKeyInformationCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pikeyinformation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&pikeyinformation)).into())
        }
        ICertSrvSetupKeyInformationCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificateEnrollmentPolicyServerSetup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InitializeInstallDefaults(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, propertyid: CEPSetupProperty) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Install(this: &Self::This) -> ::windows_core::Result<()>;
    fn UnInstall(this: &Self::This, pauthkeybasedrenewal: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICertificateEnrollmentPolicyServerSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICertificateEnrollmentPolicyServerSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ErrorString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ErrorString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeInstallDefaults(this).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: CEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into())
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Install(this).into())
        }
        unsafe extern "system" fn UnInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentPolicyServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthkeybasedrenewal: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnInstall(this, ::core::mem::transmute_copy(&pauthkeybasedrenewal)).into())
        }
        ICertificateEnrollmentPolicyServerSetup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificateEnrollmentServerSetup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ErrorString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InitializeInstallDefaults(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, propertyid: CESSetupProperty) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetApplicationPoolCredentials(this: &Self::This, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Install(this: &Self::This) -> ::windows_core::Result<()>;
    fn UnInstall(this: &Self::This, pcaconfig: *const super::super::System::Variant::VARIANT, pauthentication: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICertificateEnrollmentServerSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICertificateEnrollmentServerSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ErrorString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ErrorString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeInstallDefaults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeInstallDefaults(this).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: CESSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into())
        }
        unsafe extern "system" fn SetApplicationPoolCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationPoolCredentials(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)).into())
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Install(this).into())
        }
        unsafe extern "system" fn UnInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICertificateEnrollmentServerSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaconfig: *const super::super::System::Variant::VARIANT, pauthentication: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnInstall(this, ::core::mem::transmute_copy(&pcaconfig), ::core::mem::transmute_copy(&pauthentication)).into())
        }
        ICertificateEnrollmentServerSetup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ErrorString: ErrorString::<Identity, Impl, OFFSET>,
            InitializeInstallDefaults: InitializeInstallDefaults::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetApplicationPoolCredentials: SetApplicationPoolCredentials::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            UnInstall: UnInstall::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMSCEPSetup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn MSCEPErrorId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MSCEPErrorString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InitializeDefaults(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMSCEPSetupProperty(this: &Self::This, propertyid: MSCEPSetupProperty) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetMSCEPSetupProperty(this: &Self::This, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetAccountInformation(this: &Self::This, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsMSCEPStoreEmpty(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetProviderNameList(this: &Self::This, bexchange: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetKeyLengthList(this: &Self::This, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Install(this: &Self::This) -> ::windows_core::Result<()>;
    fn PreUnInstall(this: &Self::This) -> ::windows_core::Result<()>;
    fn PostUnInstall(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMSCEPSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMSCEPSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MSCEPErrorId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MSCEPErrorId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MSCEPErrorString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MSCEPErrorString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeDefaults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeDefaults(this).into())
        }
        unsafe extern "system" fn GetMSCEPSetupProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMSCEPSetupProperty(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMSCEPSetupProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: MSCEPSetupProperty, ppropertyvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMSCEPSetupProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyvalue)).into())
        }
        unsafe extern "system" fn SetAccountInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccountInformation(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)).into())
        }
        unsafe extern "system" fn IsMSCEPStoreEmpty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbempty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMSCEPStoreEmpty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbempty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderNameList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderNameList(this, ::core::mem::transmute_copy(&bexchange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKeyLengthList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bexchange: super::super::Foundation::VARIANT_BOOL, bstrprovidername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKeyLengthList(this, ::core::mem::transmute_copy(&bexchange), ::core::mem::transmute(&bstrprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Install(this).into())
        }
        unsafe extern "system" fn PreUnInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreUnInstall(this).into())
        }
        unsafe extern "system" fn PostUnInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMSCEPSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostUnInstall(this).into())
        }
        IMSCEPSetup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MSCEPErrorId: MSCEPErrorId::<Identity, Impl, OFFSET>,
            MSCEPErrorString: MSCEPErrorString::<Identity, Impl, OFFSET>,
            InitializeDefaults: InitializeDefaults::<Identity, Impl, OFFSET>,
            GetMSCEPSetupProperty: GetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetMSCEPSetupProperty: SetMSCEPSetupProperty::<Identity, Impl, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, Impl, OFFSET>,
            IsMSCEPStoreEmpty: IsMSCEPStoreEmpty::<Identity, Impl, OFFSET>,
            GetProviderNameList: GetProviderNameList::<Identity, Impl, OFFSET>,
            GetKeyLengthList: GetKeyLengthList::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            PreUnInstall: PreUnInstall::<Identity, Impl, OFFSET>,
            PostUnInstall: PostUnInstall::<Identity, Impl, OFFSET>,
        }
    };
}
