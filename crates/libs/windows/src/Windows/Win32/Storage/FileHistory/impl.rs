#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFhConfigMgr_Impl: ::windows_core::BaseImpl {
    fn LoadConfiguration(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateDefaultConfiguration(this: &Self::This, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SaveConfiguration(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddRemoveExcludeRule(this: &Self::This, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetIncludeExcludeRules(this: &Self::This, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY) -> ::windows_core::Result<IFhScopeIterator>;
    fn GetLocalPolicy(this: &Self::This, localpolicytype: FH_LOCAL_POLICY_TYPE) -> ::windows_core::Result<u64>;
    fn SetLocalPolicy(this: &Self::This, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows_core::Result<()>;
    fn GetBackupStatus(this: &Self::This) -> ::windows_core::Result<FH_BACKUP_STATUS>;
    fn SetBackupStatus(this: &Self::This, backupstatus: FH_BACKUP_STATUS) -> ::windows_core::Result<()>;
    fn GetDefaultTarget(this: &Self::This) -> ::windows_core::Result<IFhTarget>;
    fn ValidateTarget(this: &Self::This, targeturl: &::windows_core::BSTR) -> ::windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ProvisionAndSetNewTarget(this: &Self::This, targeturl: &::windows_core::BSTR, targetname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangeDefaultTargetRecommendation(this: &Self::This, recommend: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn QueryProtectionStatus(this: &Self::This, protectionstate: *mut u32, protecteduntiltime: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFhConfigMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFhConfigMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadConfiguration(this).into())
        }
        unsafe extern "system" fn CreateDefaultConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDefaultConfiguration(this, ::core::mem::transmute_copy(&overwriteifexists)).into())
        }
        unsafe extern "system" fn SaveConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveConfiguration(this).into())
        }
        unsafe extern "system" fn AddRemoveExcludeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRemoveExcludeRule(this, ::core::mem::transmute_copy(&add), ::core::mem::transmute_copy(&category), ::core::mem::transmute(&item)).into())
        }
        unsafe extern "system" fn GetIncludeExcludeRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIncludeExcludeRules(this, ::core::mem::transmute_copy(&include), ::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iterator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalPolicy(this, ::core::mem::transmute_copy(&localpolicytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(policyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalPolicy(this, ::core::mem::transmute_copy(&localpolicytype), ::core::mem::transmute_copy(&policyvalue)).into())
        }
        unsafe extern "system" fn GetBackupStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackupStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(backupstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBackupStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackupStatus(this, ::core::mem::transmute_copy(&backupstatus)).into())
        }
        unsafe extern "system" fn GetDefaultTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaulttarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaulttarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValidateTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValidateTarget(this, ::core::mem::transmute(&targeturl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validationresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProvisionAndSetNewTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>, targetname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProvisionAndSetNewTarget(this, ::core::mem::transmute(&targeturl), ::core::mem::transmute(&targetname)).into())
        }
        unsafe extern "system" fn ChangeDefaultTargetRecommendation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recommend: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeDefaultTargetRecommendation(this, ::core::mem::transmute_copy(&recommend)).into())
        }
        unsafe extern "system" fn QueryProtectionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhConfigMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryProtectionStatus(this, ::core::mem::transmute_copy(&protectionstate), ::core::mem::transmute_copy(&protecteduntiltime)).into())
        }
        IFhConfigMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadConfiguration: LoadConfiguration::<Identity, Impl, OFFSET>,
            CreateDefaultConfiguration: CreateDefaultConfiguration::<Identity, Impl, OFFSET>,
            SaveConfiguration: SaveConfiguration::<Identity, Impl, OFFSET>,
            AddRemoveExcludeRule: AddRemoveExcludeRule::<Identity, Impl, OFFSET>,
            GetIncludeExcludeRules: GetIncludeExcludeRules::<Identity, Impl, OFFSET>,
            GetLocalPolicy: GetLocalPolicy::<Identity, Impl, OFFSET>,
            SetLocalPolicy: SetLocalPolicy::<Identity, Impl, OFFSET>,
            GetBackupStatus: GetBackupStatus::<Identity, Impl, OFFSET>,
            SetBackupStatus: SetBackupStatus::<Identity, Impl, OFFSET>,
            GetDefaultTarget: GetDefaultTarget::<Identity, Impl, OFFSET>,
            ValidateTarget: ValidateTarget::<Identity, Impl, OFFSET>,
            ProvisionAndSetNewTarget: ProvisionAndSetNewTarget::<Identity, Impl, OFFSET>,
            ChangeDefaultTargetRecommendation: ChangeDefaultTargetRecommendation::<Identity, Impl, OFFSET>,
            QueryProtectionStatus: QueryProtectionStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFhReassociation_Impl: ::windows_core::BaseImpl {
    fn ValidateTarget(this: &Self::This, targeturl: &::windows_core::BSTR) -> ::windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ScanTargetForConfigurations(this: &Self::This, targeturl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetConfigurationDetails(this: &Self::This, index: u32, username: *mut ::windows_core::BSTR, pcname: *mut ::windows_core::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn SelectConfiguration(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn PerformReassociation(this: &Self::This, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFhReassociation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFhReassociation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ValidateTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValidateTarget(this, ::core::mem::transmute(&targeturl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validationresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScanTargetForConfigurations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targeturl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScanTargetForConfigurations(this, ::core::mem::transmute(&targeturl)).into())
        }
        unsafe extern "system" fn GetConfigurationDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, username: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConfigurationDetails(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&pcname), ::core::mem::transmute_copy(&backuptime)).into())
        }
        unsafe extern "system" fn SelectConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectConfiguration(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn PerformReassociation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhReassociation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PerformReassociation(this, ::core::mem::transmute_copy(&overwriteifexists)).into())
        }
        IFhReassociation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ValidateTarget: ValidateTarget::<Identity, Impl, OFFSET>,
            ScanTargetForConfigurations: ScanTargetForConfigurations::<Identity, Impl, OFFSET>,
            GetConfigurationDetails: GetConfigurationDetails::<Identity, Impl, OFFSET>,
            SelectConfiguration: SelectConfiguration::<Identity, Impl, OFFSET>,
            PerformReassociation: PerformReassociation::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFhScopeIterator_Impl: ::windows_core::BaseImpl {
    fn MoveToNextItem(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetItem(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IFhScopeIterator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhScopeIterator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFhScopeIterator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveToNextItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhScopeIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveToNextItem(this).into())
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhScopeIterator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFhScopeIterator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveToNextItem: MoveToNextItem::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFhTarget_Impl: ::windows_core::BaseImpl {
    fn GetStringProperty(this: &Self::This, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetNumericalProperty(this: &Self::This, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IFhTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFhTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStringProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringProperty(this, ::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumericalProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFhTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumericalProperty(this, ::core::mem::transmute_copy(&propertytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFhTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStringProperty: GetStringProperty::<Identity, Impl, OFFSET>,
            GetNumericalProperty: GetNumericalProperty::<Identity, Impl, OFFSET>,
        }
    };
}
