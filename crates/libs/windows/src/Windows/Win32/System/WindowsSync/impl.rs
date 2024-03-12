#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAsynchronousDataRetriever_Impl: ::windows_core::BaseImpl {
    fn GetIdParameters(this: &Self::This, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()>;
    fn RegisterCallback(this: &Self::This, pdataretrievercallback: ::core::option::Option<&IDataRetrieverCallback>) -> ::windows_core::Result<()>;
    fn RevokeCallback(this: &Self::This, pdataretrievercallback: ::core::option::Option<&IDataRetrieverCallback>) -> ::windows_core::Result<()>;
    fn LoadChangeData(this: &Self::This, ploadchangecontext: ::core::option::Option<&ILoadChangeContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAsynchronousDataRetriever {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsynchronousDataRetriever {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdParameters(this, ::core::mem::transmute_copy(&pidparameters)).into())
        }
        unsafe extern "system" fn RegisterCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterCallback(this, ::windows_core::from_raw_borrowed(&pdataretrievercallback)).into())
        }
        unsafe extern "system" fn RevokeCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeCallback(this, ::windows_core::from_raw_borrowed(&pdataretrievercallback)).into())
        }
        unsafe extern "system" fn LoadChangeData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploadchangecontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadChangeData(this, ::windows_core::from_raw_borrowed(&ploadchangecontext)).into())
        }
        IAsynchronousDataRetriever_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, Impl, OFFSET>,
            RevokeCallback: RevokeCallback::<Identity, Impl, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IChangeConflict_Impl: ::windows_core::BaseImpl {
    fn GetDestinationProviderConflictingChange(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetSourceProviderConflictingData(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetResolveActionForChange(this: &Self::This, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()>;
    fn SetResolveActionForChange(this: &Self::This, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()>;
    fn GetResolveActionForChangeUnit(this: &Self::This, pchangeunit: ::core::option::Option<&ISyncChangeUnit>, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()>;
    fn SetResolveActionForChangeUnit(this: &Self::This, pchangeunit: ::core::option::Option<&ISyncChangeUnit>, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IChangeConflict {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChangeConflict {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDestinationProviderConflictingChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceProviderConflictingChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDestinationProviderConflictingData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceProviderConflictingData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResolveActionForChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResolveActionForChange(this, ::core::mem::transmute_copy(&presolveaction)).into())
        }
        unsafe extern "system" fn SetResolveActionForChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResolveActionForChange(this, ::core::mem::transmute_copy(&resolveaction)).into())
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResolveActionForChangeUnit(this, ::windows_core::from_raw_borrowed(&pchangeunit), ::core::mem::transmute_copy(&presolveaction)).into())
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResolveActionForChangeUnit(this, ::windows_core::from_raw_borrowed(&pchangeunit), ::core::mem::transmute_copy(&resolveaction)).into())
        }
        IChangeConflict_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, Impl, OFFSET>,
            GetResolveActionForChange: GetResolveActionForChange::<Identity, Impl, OFFSET>,
            SetResolveActionForChange: SetResolveActionForChange::<Identity, Impl, OFFSET>,
            GetResolveActionForChangeUnit: GetResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            SetResolveActionForChangeUnit: SetResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IChangeUnitException_Impl: ::windows_core::BaseImpl {
    fn GetItemId(this: &Self::This, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetChangeUnitId(this: &Self::This, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetClockVector(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IChangeUnitException {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChangeUnitException {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemId(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeUnitId(this, ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetClockVector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClockVector(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        IChangeUnitException_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemId: GetItemId::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IChangeUnitListFilterInfo_Impl: ::windows_core::BaseImpl + ISyncFilterInfo_Impl {
    fn Initialize(this: &Self::This, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows_core::Result<()>;
    fn GetChangeUnitIdCount(this: &Self::This, pdwchangeunitidcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetChangeUnitId(this: &Self::This, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IChangeUnitListFilterInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncFilterInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChangeUnitListFilterInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&ppbchangeunitids), ::core::mem::transmute_copy(&dwchangeunitcount)).into())
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeUnitIdCount(this, ::core::mem::transmute_copy(&pdwchangeunitidcount)).into())
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeUnitId(this, ::core::mem::transmute_copy(&dwchangeunitidindex), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        IChangeUnitListFilterInfo_Vtbl {
            base__: <ISyncFilterInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetChangeUnitIdCount: GetChangeUnitIdCount::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IClockVector_Impl: ::windows_core::BaseImpl {
    fn GetClockVectorElements(this: &Self::This, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetClockVectorElementCount(this: &Self::This, pdwcount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IClockVector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClockVector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClockVector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClockVectorElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClockVectorElements(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppienumclockvector)).into())
        }
        unsafe extern "system" fn GetClockVectorElementCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClockVectorElementCount(this, ::core::mem::transmute_copy(&pdwcount)).into())
        }
        IClockVector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClockVectorElements: GetClockVectorElements::<Identity, Impl, OFFSET>,
            GetClockVectorElementCount: GetClockVectorElementCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IClockVectorElement_Impl: ::windows_core::BaseImpl {
    fn GetReplicaKey(this: &Self::This, pdwreplicakey: *mut u32) -> ::windows_core::Result<()>;
    fn GetTickCount(this: &Self::This, pulltickcount: *mut u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IClockVectorElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClockVectorElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClockVectorElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetReplicaKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClockVectorElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReplicaKey(this, ::core::mem::transmute_copy(&pdwreplicakey)).into())
        }
        unsafe extern "system" fn GetTickCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClockVectorElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTickCount(this, ::core::mem::transmute_copy(&pulltickcount)).into())
        }
        IClockVectorElement_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetReplicaKey: GetReplicaKey::<Identity, Impl, OFFSET>,
            GetTickCount: GetTickCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICombinedFilterInfo_Impl: ::windows_core::BaseImpl + ISyncFilterInfo_Impl {
    fn GetFilterCount(this: &Self::This, pdwfiltercount: *mut u32) -> ::windows_core::Result<()>;
    fn GetFilterInfo(this: &Self::This, dwfilterindex: u32) -> ::windows_core::Result<ISyncFilterInfo>;
    fn GetFilterCombinationType(this: &Self::This, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICombinedFilterInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncFilterInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICombinedFilterInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilterCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilterCount(this, ::core::mem::transmute_copy(&pdwfiltercount)).into())
        }
        unsafe extern "system" fn GetFilterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilterInfo(this, ::core::mem::transmute_copy(&dwfilterindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifilterinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilterCombinationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilterCombinationType(this, ::core::mem::transmute_copy(&pfiltercombinationtype)).into())
        }
        ICombinedFilterInfo_Vtbl {
            base__: <ISyncFilterInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFilterCount: GetFilterCount::<Identity, Impl, OFFSET>,
            GetFilterInfo: GetFilterInfo::<Identity, Impl, OFFSET>,
            GetFilterCombinationType: GetFilterCombinationType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IConstraintConflict_Impl: ::windows_core::BaseImpl {
    fn GetDestinationProviderConflictingChange(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn GetDestinationProviderOriginalChange(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetSourceProviderConflictingData(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetDestinationProviderOriginalData(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetConstraintResolveActionForChange(this: &Self::This, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()>;
    fn SetConstraintResolveActionForChange(this: &Self::This, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()>;
    fn GetConstraintResolveActionForChangeUnit(this: &Self::This, pchangeunit: ::core::option::Option<&ISyncChangeUnit>, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()>;
    fn SetConstraintResolveActionForChangeUnit(this: &Self::This, pchangeunit: ::core::option::Option<&ISyncChangeUnit>, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::Result<()>;
    fn GetConstraintConflictReason(this: &Self::This, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows_core::Result<()>;
    fn IsTemporary(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IConstraintConflict {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConstraintConflict {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDestinationProviderConflictingChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceProviderConflictingChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pporiginalchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDestinationProviderOriginalChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pporiginalchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDestinationProviderConflictingData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceProviderConflictingData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconflictingdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDestinationProviderOriginalData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pporiginaldata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstraintResolveActionForChange(this, ::core::mem::transmute_copy(&pconstraintresolveaction)).into())
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConstraintResolveActionForChange(this, ::core::mem::transmute_copy(&constraintresolveaction)).into())
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstraintResolveActionForChangeUnit(this, ::windows_core::from_raw_borrowed(&pchangeunit), ::core::mem::transmute_copy(&pconstraintresolveaction)).into())
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConstraintResolveActionForChangeUnit(this, ::windows_core::from_raw_borrowed(&pchangeunit), ::core::mem::transmute_copy(&constraintresolveaction)).into())
        }
        unsafe extern "system" fn GetConstraintConflictReason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConstraintConflictReason(this, ::core::mem::transmute_copy(&pconstraintconflictreason)).into())
        }
        unsafe extern "system" fn IsTemporary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsTemporary(this).into())
        }
        IConstraintConflict_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderOriginalChange: GetDestinationProviderOriginalChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, Impl, OFFSET>,
            GetDestinationProviderOriginalData: GetDestinationProviderOriginalData::<Identity, Impl, OFFSET>,
            GetConstraintResolveActionForChange: GetConstraintResolveActionForChange::<Identity, Impl, OFFSET>,
            SetConstraintResolveActionForChange: SetConstraintResolveActionForChange::<Identity, Impl, OFFSET>,
            GetConstraintResolveActionForChangeUnit: GetConstraintResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            SetConstraintResolveActionForChangeUnit: SetConstraintResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            GetConstraintConflictReason: GetConstraintConflictReason::<Identity, Impl, OFFSET>,
            IsTemporary: IsTemporary::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IConstructReplicaKeyMap_Impl: ::windows_core::BaseImpl {
    fn FindOrAddReplica(this: &Self::This, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IConstructReplicaKeyMap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstructReplicaKeyMap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConstructReplicaKeyMap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindOrAddReplica<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConstructReplicaKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindOrAddReplica(this, ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into())
        }
        IConstructReplicaKeyMap_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindOrAddReplica: FindOrAddReplica::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICoreFragment_Impl: ::windows_core::BaseImpl {
    fn NextColumn(this: &Self::This, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows_core::Result<()>;
    fn NextRange(this: &Self::This, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::core::option::Option<IClockVector>) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetColumnCount(this: &Self::This, pcolumncount: *mut u32) -> ::windows_core::Result<()>;
    fn GetRangeCount(this: &Self::This, prangecount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICoreFragment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreFragment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NextColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextColumn(this, ::core::mem::transmute_copy(&pchangeunitid), ::core::mem::transmute_copy(&pchangeunitidsize)).into())
        }
        unsafe extern "system" fn NextRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextRange(this, ::core::mem::transmute_copy(&pitemid), ::core::mem::transmute_copy(&pitemidsize), ::core::mem::transmute_copy(&piclockvector)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn GetColumnCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnCount(this, ::core::mem::transmute_copy(&pcolumncount)).into())
        }
        unsafe extern "system" fn GetRangeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRangeCount(this, ::core::mem::transmute_copy(&prangecount)).into())
        }
        ICoreFragment_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NextColumn: NextColumn::<Identity, Impl, OFFSET>,
            NextRange: NextRange::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetRangeCount: GetRangeCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICoreFragmentInspector_Impl: ::windows_core::BaseImpl {
    fn NextCoreFragments(this: &Self::This, requestedcount: u32, ppicorefragments: *mut ::core::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICoreFragmentInspector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragmentInspector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreFragmentInspector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NextCoreFragments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragmentInspector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut *mut ::core::ffi::c_void, pfetchedcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextCoreFragments(this, ::core::mem::transmute_copy(&requestedcount), ::core::mem::transmute_copy(&ppicorefragments), ::core::mem::transmute_copy(&pfetchedcount)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFragmentInspector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        ICoreFragmentInspector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NextCoreFragments: NextCoreFragments::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICustomFilterInfo_Impl: ::windows_core::BaseImpl + ISyncFilterInfo_Impl {
    fn GetSyncFilter(this: &Self::This) -> ::windows_core::Result<ISyncFilter>;
}
impl ::windows_core::Iids for ICustomFilterInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncFilterInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomFilterInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICustomFilterInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisyncfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICustomFilterInfo_Vtbl { base__: <ISyncFilterInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSyncFilter: GetSyncFilter::<Identity, Impl, OFFSET> }
    };
}
pub trait IDataRetrieverCallback_Impl: ::windows_core::BaseImpl {
    fn LoadChangeDataComplete(this: &Self::This, punkdata: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn LoadChangeDataError(this: &Self::This, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDataRetrieverCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataRetrieverCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataRetrieverCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadChangeDataComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataRetrieverCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadChangeDataComplete(this, ::windows_core::from_raw_borrowed(&punkdata)).into())
        }
        unsafe extern "system" fn LoadChangeDataError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataRetrieverCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadChangeDataError(this, ::core::mem::transmute_copy(&hrerror)).into())
        }
        IDataRetrieverCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadChangeDataComplete: LoadChangeDataComplete::<Identity, Impl, OFFSET>,
            LoadChangeDataError: LoadChangeDataError::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumChangeUnitExceptions_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cexceptions: u32, ppchangeunitexception: *mut ::core::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cexceptions: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumChangeUnitExceptions>;
}
impl ::windows_core::Iids for IEnumChangeUnitExceptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumChangeUnitExceptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppchangeunitexception), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cexceptions)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumChangeUnitExceptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumClockVector_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, csyncversions: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumClockVector>;
}
impl ::windows_core::Iids for IEnumClockVector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumClockVector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&csyncversions)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumClockVector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumFeedClockVector_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, csyncversions: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumFeedClockVector>;
}
impl ::windows_core::Iids for IEnumFeedClockVector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumFeedClockVector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cclockvectorelements), ::core::mem::transmute_copy(&ppiclockvectorelements), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&csyncversions)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumFeedClockVector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumItemIds_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumItemIds {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemIds_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumItemIds {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemIds_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into())
        }
        IEnumItemIds_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
}
pub trait IEnumRangeExceptions_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cexceptions: u32, pprangeexception: *mut ::core::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cexceptions: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumRangeExceptions>;
}
impl ::windows_core::Iids for IEnumRangeExceptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumRangeExceptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&pprangeexception), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cexceptions)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumRangeExceptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumSingleItemExceptions_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cexceptions: u32, ppsingleitemexception: *mut ::core::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cexceptions: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSingleItemExceptions>;
}
impl ::windows_core::Iids for IEnumSingleItemExceptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSingleItemExceptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cexceptions), ::core::mem::transmute_copy(&ppsingleitemexception), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cexceptions)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSingleItemExceptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumSyncChangeUnits_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cchanges: u32, ppchangeunit: *mut ::core::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cchanges: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSyncChangeUnits>;
}
impl ::windows_core::Iids for IEnumSyncChangeUnits {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSyncChangeUnits {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchangeunit), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cchanges)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSyncChangeUnits_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumSyncChanges_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cchanges: u32, ppchange: *mut ::core::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cchanges: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSyncChanges>;
}
impl ::windows_core::Iids for IEnumSyncChanges {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSyncChanges {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cchanges), ::core::mem::transmute_copy(&ppchange), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cchanges)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSyncChanges_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderConfigUIInfos_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::core::option::Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cfactories: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSyncProviderConfigUIInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IEnumSyncProviderConfigUIInfos {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSyncProviderConfigUIInfos {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cfactories), ::core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cfactories)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSyncProviderConfigUIInfos_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderInfos_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cinstances: u32, ppsyncproviderinfo: *mut ::core::option::Option<ISyncProviderInfo>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cinstances: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSyncProviderInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IEnumSyncProviderInfos {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSyncProviderInfos {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cinstances), ::core::mem::transmute_copy(&ppsyncproviderinfo), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cinstances)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSyncProviderInfos_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFeedClockVector_Impl: ::windows_core::BaseImpl + IClockVector_Impl {
    fn GetUpdateCount(this: &Self::This, pdwupdatecount: *mut u32) -> ::windows_core::Result<()>;
    fn IsNoConflictsSpecified(this: &Self::This, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFeedClockVector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IClockVector);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedClockVector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedClockVector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUpdateCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUpdateCount(this, ::core::mem::transmute_copy(&pdwupdatecount)).into())
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedClockVector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsNoConflictsSpecified(this, ::core::mem::transmute_copy(&pfisnoconflictsspecified)).into())
        }
        IFeedClockVector_Vtbl {
            base__: <IClockVector as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUpdateCount: GetUpdateCount::<Identity, Impl, OFFSET>,
            IsNoConflictsSpecified: IsNoConflictsSpecified::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFeedClockVectorElement_Impl: ::windows_core::BaseImpl + IClockVectorElement_Impl {
    fn GetSyncTime(this: &Self::This, psynctime: *mut SYNC_TIME) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This, pbflags: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFeedClockVectorElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IClockVectorElement);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedClockVectorElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedClockVectorElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedClockVectorElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSyncTime(this, ::core::mem::transmute_copy(&psynctime)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedClockVectorElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlags(this, ::core::mem::transmute_copy(&pbflags)).into())
        }
        IFeedClockVectorElement_Vtbl {
            base__: <IClockVectorElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSyncTime: GetSyncTime::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFilterKeyMap_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This, pdwcount: *mut u32) -> ::windows_core::Result<()>;
    fn AddFilter(this: &Self::This, pisyncfilter: ::core::option::Option<&ISyncFilter>, pdwfilterkey: *mut u32) -> ::windows_core::Result<()>;
    fn GetFilter(this: &Self::This, dwfilterkey: u32) -> ::windows_core::Result<ISyncFilter>;
    fn Serialize(this: &Self::This, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFilterKeyMap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFilterKeyMap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pdwcount)).into())
        }
        unsafe extern "system" fn AddFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::core::ffi::c_void, pdwfilterkey: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFilter(this, ::windows_core::from_raw_borrowed(&pisyncfilter), ::core::mem::transmute_copy(&pdwfilterkey)).into())
        }
        unsafe extern "system" fn GetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilter(this, ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppisyncfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&pbfilterkeymap), ::core::mem::transmute_copy(&pcbfilterkeymap)).into())
        }
        IFilterKeyMap_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            AddFilter: AddFilter::<Identity, Impl, OFFSET>,
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFilterRequestCallback_Impl: ::windows_core::BaseImpl {
    fn RequestFilter(this: &Self::This, pfilter: ::core::option::Option<&::windows_core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFilterRequestCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterRequestCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFilterRequestCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterRequestCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestFilter(this, ::windows_core::from_raw_borrowed(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into())
        }
        IFilterRequestCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RequestFilter: RequestFilter::<Identity, Impl, OFFSET> }
    };
}
pub trait IFilterTrackingProvider_Impl: ::windows_core::BaseImpl {
    fn SpecifyTrackedFilters(this: &Self::This, pcallback: ::core::option::Option<&IFilterTrackingRequestCallback>) -> ::windows_core::Result<()>;
    fn AddTrackedFilter(this: &Self::This, pfilter: ::core::option::Option<&ISyncFilter>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFilterTrackingProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFilterTrackingProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SpecifyTrackedFilters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SpecifyTrackedFilters(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn AddTrackedFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTrackedFilter(this, ::windows_core::from_raw_borrowed(&pfilter)).into())
        }
        IFilterTrackingProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SpecifyTrackedFilters: SpecifyTrackedFilters::<Identity, Impl, OFFSET>,
            AddTrackedFilter: AddTrackedFilter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFilterTrackingRequestCallback_Impl: ::windows_core::BaseImpl {
    fn RequestTrackedFilter(this: &Self::This, pfilter: ::core::option::Option<&ISyncFilter>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFilterTrackingRequestCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingRequestCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFilterTrackingRequestCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestTrackedFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingRequestCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestTrackedFilter(this, ::windows_core::from_raw_borrowed(&pfilter)).into())
        }
        IFilterTrackingRequestCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestTrackedFilter: RequestTrackedFilter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFilterTrackingSyncChangeBuilder_Impl: ::windows_core::BaseImpl {
    fn AddFilterChange(this: &Self::This, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows_core::Result<()>;
    fn SetAllChangeUnitsPresentFlag(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFilterTrackingSyncChangeBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFilterTrackingSyncChangeBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddFilterChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFilterChange(this, ::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into())
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllChangeUnitsPresentFlag(this).into())
        }
        IFilterTrackingSyncChangeBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddFilterChange: AddFilterChange::<Identity, Impl, OFFSET>,
            SetAllChangeUnitsPresentFlag: SetAllChangeUnitsPresentFlag::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IForgottenKnowledge_Impl: ::windows_core::BaseImpl + ISyncKnowledge_Impl {
    fn ForgetToVersion(this: &Self::This, pknowledge: ::core::option::Option<&ISyncKnowledge>, pversion: *const SYNC_VERSION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IForgottenKnowledge {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncKnowledge);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IForgottenKnowledge_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IForgottenKnowledge {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ForgetToVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IForgottenKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pversion: *const SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForgetToVersion(this, ::windows_core::from_raw_borrowed(&pknowledge), ::core::mem::transmute_copy(&pversion)).into())
        }
        IForgottenKnowledge_Vtbl { base__: <ISyncKnowledge as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ForgetToVersion: ForgetToVersion::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IKnowledgeSyncProvider_Impl: ::windows_core::BaseImpl + ISyncProvider_Impl {
    fn BeginSession(this: &Self::This, role: SYNC_PROVIDER_ROLE, psessionstate: ::core::option::Option<&ISyncSessionState>) -> ::windows_core::Result<()>;
    fn GetSyncBatchParameters(this: &Self::This, ppsyncknowledge: *mut ::core::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetChangeBatch(this: &Self::This, dwbatchsize: u32, psyncknowledge: ::core::option::Option<&ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetFullEnumerationChangeBatch(this: &Self::This, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::core::option::Option<&ISyncKnowledge>, ppsyncchangebatch: *mut ::core::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ProcessChangeBatch(this: &Self::This, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::core::option::Option<&ISyncChangeBatch>, punkdataretriever: ::core::option::Option<&::windows_core::IUnknown>, pcallback: ::core::option::Option<&ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::Result<()>;
    fn ProcessFullEnumerationChangeBatch(this: &Self::This, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::core::option::Option<&ISyncFullEnumerationChangeBatch>, punkdataretriever: ::core::option::Option<&::windows_core::IUnknown>, pcallback: ::core::option::Option<&ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::Result<()>;
    fn EndSession(this: &Self::This, psessionstate: ::core::option::Option<&ISyncSessionState>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IKnowledgeSyncProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncProvider);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IKnowledgeSyncProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginSession(this, ::core::mem::transmute_copy(&role), ::windows_core::from_raw_borrowed(&psessionstate)).into())
        }
        unsafe extern "system" fn GetSyncBatchParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut *mut ::core::ffi::c_void, pdwrequestedbatchsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSyncBatchParameters(this, ::core::mem::transmute_copy(&ppsyncknowledge), ::core::mem::transmute_copy(&pdwrequestedbatchsize)).into())
        }
        unsafe extern "system" fn GetChangeBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeBatch(this, ::core::mem::transmute_copy(&dwbatchsize), ::windows_core::from_raw_borrowed(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into())
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFullEnumerationChangeBatch(this, ::core::mem::transmute_copy(&dwbatchsize), ::core::mem::transmute_copy(&pblowerenumerationbound), ::windows_core::from_raw_borrowed(&psyncknowledge), ::core::mem::transmute_copy(&ppsyncchangebatch), ::core::mem::transmute_copy(&ppunkdataretriever)).into())
        }
        unsafe extern "system" fn ProcessChangeBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessChangeBatch(this, ::core::mem::transmute_copy(&resolutionpolicy), ::windows_core::from_raw_borrowed(&psourcechangebatch), ::windows_core::from_raw_borrowed(&punkdataretriever), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into())
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessFullEnumerationChangeBatch(this, ::core::mem::transmute_copy(&resolutionpolicy), ::windows_core::from_raw_borrowed(&psourcechangebatch), ::windows_core::from_raw_borrowed(&punkdataretriever), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&psyncsessionstatistics)).into())
        }
        unsafe extern "system" fn EndSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessionstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndSession(this, ::windows_core::from_raw_borrowed(&psessionstate)).into())
        }
        IKnowledgeSyncProvider_Vtbl {
            base__: <ISyncProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginSession: BeginSession::<Identity, Impl, OFFSET>,
            GetSyncBatchParameters: GetSyncBatchParameters::<Identity, Impl, OFFSET>,
            GetChangeBatch: GetChangeBatch::<Identity, Impl, OFFSET>,
            GetFullEnumerationChangeBatch: GetFullEnumerationChangeBatch::<Identity, Impl, OFFSET>,
            ProcessChangeBatch: ProcessChangeBatch::<Identity, Impl, OFFSET>,
            ProcessFullEnumerationChangeBatch: ProcessFullEnumerationChangeBatch::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILoadChangeContext_Impl: ::windows_core::BaseImpl {
    fn GetSyncChange(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn SetRecoverableErrorOnChange(this: &Self::This, hrerror: ::windows_core::HRESULT, perrordata: ::core::option::Option<&IRecoverableErrorData>) -> ::windows_core::Result<()>;
    fn SetRecoverableErrorOnChangeUnit(this: &Self::This, hrerror: ::windows_core::HRESULT, pchangeunit: ::core::option::Option<&ISyncChangeUnit>, perrordata: ::core::option::Option<&IRecoverableErrorData>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILoadChangeContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILoadChangeContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, perrordata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecoverableErrorOnChange(this, ::core::mem::transmute_copy(&hrerror), ::windows_core::from_raw_borrowed(&perrordata)).into())
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, pchangeunit: *mut ::core::ffi::c_void, perrordata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRecoverableErrorOnChangeUnit(this, ::core::mem::transmute_copy(&hrerror), ::windows_core::from_raw_borrowed(&pchangeunit), ::windows_core::from_raw_borrowed(&perrordata)).into())
        }
        ILoadChangeContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSyncChange: GetSyncChange::<Identity, Impl, OFFSET>,
            SetRecoverableErrorOnChange: SetRecoverableErrorOnChange::<Identity, Impl, OFFSET>,
            SetRecoverableErrorOnChangeUnit: SetRecoverableErrorOnChangeUnit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IProviderConverter_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pisyncprovider: ::core::option::Option<&ISyncProvider>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IProviderConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProviderConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProviderConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncprovider: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pisyncprovider)).into())
        }
        IProviderConverter_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
pub trait IRangeException_Impl: ::windows_core::BaseImpl {
    fn GetClosedRangeStart(this: &Self::This, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetClosedRangeEnd(this: &Self::This, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetClockVector(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRangeException {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRangeException {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClosedRangeStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClosedRangeStart(this, ::core::mem::transmute_copy(&pbclosedrangestart), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetClosedRangeEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClosedRangeEnd(this, ::core::mem::transmute_copy(&pbclosedrangeend), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetClockVector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClockVector(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        IRangeException_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClosedRangeStart: GetClosedRangeStart::<Identity, Impl, OFFSET>,
            GetClosedRangeEnd: GetClosedRangeEnd::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRecoverableError_Impl: ::windows_core::BaseImpl {
    fn GetStage(this: &Self::This, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows_core::Result<()>;
    fn GetProvider(this: &Self::This, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows_core::Result<()>;
    fn GetChangeWithRecoverableError(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn GetRecoverableErrorDataForChange(this: &Self::This, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows_core::Result<()>;
    fn GetRecoverableErrorDataForChangeUnit(this: &Self::This, pchangeunit: ::core::option::Option<&ISyncChangeUnit>, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRecoverableError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRecoverableError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStage(this, ::core::mem::transmute_copy(&pstage)).into())
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProvider(this, ::core::mem::transmute_copy(&pproviderrole)).into())
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChangeWithRecoverableError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchangewithrecoverableerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRecoverableErrorDataForChange(this, ::core::mem::transmute_copy(&phrerror), ::core::mem::transmute_copy(&pperrordata)).into())
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, phrerror: *mut ::windows_core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRecoverableErrorDataForChangeUnit(this, ::windows_core::from_raw_borrowed(&pchangeunit), ::core::mem::transmute_copy(&phrerror), ::core::mem::transmute_copy(&pperrordata)).into())
        }
        IRecoverableError_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStage: GetStage::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetChangeWithRecoverableError: GetChangeWithRecoverableError::<Identity, Impl, OFFSET>,
            GetRecoverableErrorDataForChange: GetRecoverableErrorDataForChange::<Identity, Impl, OFFSET>,
            GetRecoverableErrorDataForChangeUnit: GetRecoverableErrorDataForChangeUnit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRecoverableErrorData_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pcszitemdisplayname: &::windows_core::PCWSTR, pcszerrordescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetItemDisplayName(this: &Self::This, pszitemdisplayname: &::windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows_core::Result<()>;
    fn GetErrorDescription(this: &Self::This, pszerrordescription: &::windows_core::PCWSTR, pccherrordescription: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRecoverableErrorData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRecoverableErrorData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcszitemdisplayname: ::windows_core::PCWSTR, pcszerrordescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pcszitemdisplayname), ::core::mem::transmute(&pcszerrordescription)).into())
        }
        unsafe extern "system" fn GetItemDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszitemdisplayname: ::windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemDisplayName(this, ::core::mem::transmute(&pszitemdisplayname), ::core::mem::transmute_copy(&pcchitemdisplayname)).into())
        }
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszerrordescription: ::windows_core::PCWSTR, pccherrordescription: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrorDescription(this, ::core::mem::transmute(&pszerrordescription), ::core::mem::transmute_copy(&pccherrordescription)).into())
        }
        IRecoverableErrorData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetItemDisplayName: GetItemDisplayName::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IRegisteredSyncProvider_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pcontextpropertystore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn GetInstanceId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IRegisteredSyncProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRegisteredSyncProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pcontextpropertystore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::windows_core::from_raw_borrowed(&pcontextpropertystore)).into())
        }
        unsafe extern "system" fn GetInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstanceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IRegisteredSyncProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IReplicaKeyMap_Impl: ::windows_core::BaseImpl {
    fn LookupReplicaKey(this: &Self::This, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::Result<()>;
    fn LookupReplicaId(this: &Self::This, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn Serialize(this: &Self::This, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IReplicaKeyMap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReplicaKeyMap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LookupReplicaKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LookupReplicaKey(this, ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pdwreplicakey)).into())
        }
        unsafe extern "system" fn LookupReplicaId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LookupReplicaId(this, ::core::mem::transmute_copy(&dwreplicakey), ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&pbreplicakeymap), ::core::mem::transmute_copy(&pcbreplicakeymap)).into())
        }
        IReplicaKeyMap_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LookupReplicaKey: LookupReplicaKey::<Identity, Impl, OFFSET>,
            LookupReplicaId: LookupReplicaId::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRequestFilteredSync_Impl: ::windows_core::BaseImpl {
    fn SpecifyFilter(this: &Self::This, pcallback: ::core::option::Option<&IFilterRequestCallback>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRequestFilteredSync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRequestFilteredSync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRequestFilteredSync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SpecifyFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRequestFilteredSync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SpecifyFilter(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        IRequestFilteredSync_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SpecifyFilter: SpecifyFilter::<Identity, Impl, OFFSET> }
    };
}
pub trait ISingleItemException_Impl: ::windows_core::BaseImpl {
    fn GetItemId(this: &Self::This, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetClockVector(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISingleItemException {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleItemException_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISingleItemException {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleItemException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemId(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetClockVector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISingleItemException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClockVector(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        ISingleItemException_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemId: GetItemId::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISupportFilteredSync_Impl: ::windows_core::BaseImpl {
    fn AddFilter(this: &Self::This, pfilter: ::core::option::Option<&::windows_core::IUnknown>, filteringtype: FILTERING_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISupportFilteredSync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportFilteredSync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISupportFilteredSync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportFilteredSync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFilter(this, ::windows_core::from_raw_borrowed(&pfilter), ::core::mem::transmute_copy(&filteringtype)).into())
        }
        ISupportFilteredSync_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddFilter: AddFilter::<Identity, Impl, OFFSET> }
    };
}
pub trait ISupportLastWriteTime_Impl: ::windows_core::BaseImpl {
    fn GetItemChangeTime(this: &Self::This, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::Result<()>;
    fn GetChangeUnitChangeTime(this: &Self::This, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISupportLastWriteTime {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportLastWriteTime_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISupportLastWriteTime {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemChangeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportLastWriteTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemChangeTime(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pulltimestamp)).into())
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISupportLastWriteTime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeUnitChangeTime(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pulltimestamp)).into())
        }
        ISupportLastWriteTime_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemChangeTime: GetItemChangeTime::<Identity, Impl, OFFSET>,
            GetChangeUnitChangeTime: GetChangeUnitChangeTime::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncCallback_Impl: ::windows_core::BaseImpl {
    fn OnProgress(this: &Self::This, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()>;
    fn OnChange(this: &Self::This, psyncchange: ::core::option::Option<&ISyncChange>) -> ::windows_core::Result<()>;
    fn OnConflict(this: &Self::This, pconflict: ::core::option::Option<&IChangeConflict>) -> ::windows_core::Result<()>;
    fn OnFullEnumerationNeeded(this: &Self::This, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::Result<()>;
    fn OnRecoverableError(this: &Self::This, precoverableerror: ::core::option::Option<&IRecoverableError>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&syncstage), ::core::mem::transmute_copy(&dwcompletedwork), ::core::mem::transmute_copy(&dwtotalwork)).into())
        }
        unsafe extern "system" fn OnChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChange(this, ::windows_core::from_raw_borrowed(&psyncchange)).into())
        }
        unsafe extern "system" fn OnConflict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconflict: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConflict(this, ::windows_core::from_raw_borrowed(&pconflict)).into())
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFullEnumerationNeeded(this, ::core::mem::transmute_copy(&pfullenumerationaction)).into())
        }
        unsafe extern "system" fn OnRecoverableError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precoverableerror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRecoverableError(this, ::windows_core::from_raw_borrowed(&precoverableerror)).into())
        }
        ISyncCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnChange: OnChange::<Identity, Impl, OFFSET>,
            OnConflict: OnConflict::<Identity, Impl, OFFSET>,
            OnFullEnumerationNeeded: OnFullEnumerationNeeded::<Identity, Impl, OFFSET>,
            OnRecoverableError: OnRecoverableError::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncCallback2_Impl: ::windows_core::BaseImpl + ISyncCallback_Impl {
    fn OnChangeApplied(this: &Self::This, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::Result<()>;
    fn OnChangeFailed(this: &Self::This, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncCallback2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncCallback);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncCallback2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChangeApplied<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChangeApplied(this, ::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into())
        }
        unsafe extern "system" fn OnChangeFailed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChangeFailed(this, ::core::mem::transmute_copy(&dwchangesapplied), ::core::mem::transmute_copy(&dwchangesfailed)).into())
        }
        ISyncCallback2_Vtbl {
            base__: <ISyncCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnChangeApplied: OnChangeApplied::<Identity, Impl, OFFSET>,
            OnChangeFailed: OnChangeFailed::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncChange_Impl: ::windows_core::BaseImpl {
    fn GetOwnerReplicaId(this: &Self::This, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetRootItemId(this: &Self::This, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetChangeVersion(this: &Self::This, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()>;
    fn GetCreationVersion(this: &Self::This, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This, pdwflags: *mut u32) -> ::windows_core::Result<()>;
    fn GetWorkEstimate(this: &Self::This, pdwwork: *mut u32) -> ::windows_core::Result<()>;
    fn GetChangeUnits(this: &Self::This) -> ::windows_core::Result<IEnumSyncChangeUnits>;
    fn GetMadeWithKnowledge(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledge(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn SetWorkEstimate(this: &Self::This, dwwork: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOwnerReplicaId(this, ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetRootItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRootItemId(this, ::core::mem::transmute_copy(&pbrootitemid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetChangeVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeVersion(this, ::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into())
        }
        unsafe extern "system" fn GetCreationVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationVersion(this, ::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlags(this, ::core::mem::transmute_copy(&pdwflags)).into())
        }
        unsafe extern "system" fn GetWorkEstimate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWorkEstimate(this, ::core::mem::transmute_copy(&pdwwork)).into())
        }
        unsafe extern "system" fn GetChangeUnits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChangeUnits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMadeWithKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmadewithknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWorkEstimate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkEstimate(this, ::core::mem::transmute_copy(&dwwork)).into())
        }
        ISyncChange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, Impl, OFFSET>,
            GetRootItemId: GetRootItemId::<Identity, Impl, OFFSET>,
            GetChangeVersion: GetChangeVersion::<Identity, Impl, OFFSET>,
            GetCreationVersion: GetCreationVersion::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetWorkEstimate: GetWorkEstimate::<Identity, Impl, OFFSET>,
            GetChangeUnits: GetChangeUnits::<Identity, Impl, OFFSET>,
            GetMadeWithKnowledge: GetMadeWithKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, Impl, OFFSET>,
            SetWorkEstimate: SetWorkEstimate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch_Impl: ::windows_core::BaseImpl + ISyncChangeBatchBase_Impl {
    fn BeginUnorderedGroup(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndUnorderedGroup(this: &Self::This, pmadewithknowledge: ::core::option::Option<&ISyncKnowledge>, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AddLoggedConflict(this: &Self::This, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncChangeBatch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncChangeBatchBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBatch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginUnorderedGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUnorderedGroup(this).into())
        }
        unsafe extern "system" fn EndUnorderedGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmadewithknowledge: *mut ::core::ffi::c_void, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndUnorderedGroup(this, ::windows_core::from_raw_borrowed(&pmadewithknowledge), ::core::mem::transmute_copy(&fallchangesforknowledge)).into())
        }
        unsafe extern "system" fn AddLoggedConflict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddLoggedConflict(this, ::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwworkforchange), ::windows_core::from_raw_borrowed(&pconflictknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchangebuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncChangeBatch_Vtbl {
            base__: <ISyncChangeBatchBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginUnorderedGroup: BeginUnorderedGroup::<Identity, Impl, OFFSET>,
            EndUnorderedGroup: EndUnorderedGroup::<Identity, Impl, OFFSET>,
            AddLoggedConflict: AddLoggedConflict::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatch2_Impl: ::windows_core::BaseImpl + ISyncChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(this: &Self::This, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder>;
    fn AddMergeTombstoneLoggedConflict(this: &Self::This, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncChangeBatch2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncChangeBatch);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatch2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBatch2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatch2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddMergeTombstoneMetadataToGroup(this, ::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchangebuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatch2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddMergeTombstoneLoggedConflict(this, ::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange), ::windows_core::from_raw_borrowed(&pconflictknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchangebuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncChangeBatch2_Vtbl {
            base__: <ISyncChangeBatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, Impl, OFFSET>,
            AddMergeTombstoneLoggedConflict: AddMergeTombstoneLoggedConflict::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchAdvanced_Impl: ::windows_core::BaseImpl {
    fn GetFilterInfo(this: &Self::This) -> ::windows_core::Result<ISyncFilterInfo>;
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(this: &Self::This) -> ::windows_core::Result<ISyncChangeBatch>;
    fn GetUpperBoundItemId(this: &Self::This, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetBatchLevelKnowledgeShouldBeApplied(this: &Self::This, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncChangeBatchAdvanced {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBatchAdvanced {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilterInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfilterinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppchangebatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConvertFullEnumerationChangeBatchToRegularChangeBatch(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchangebatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUpperBoundItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUpperBoundItemId(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBatchLevelKnowledgeShouldBeApplied(this, ::core::mem::transmute_copy(&pfbatchknowledgeshouldbeapplied)).into())
        }
        ISyncChangeBatchAdvanced_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFilterInfo: GetFilterInfo::<Identity, Impl, OFFSET>,
            ConvertFullEnumerationChangeBatchToRegularChangeBatch: ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Identity, Impl, OFFSET>,
            GetUpperBoundItemId: GetUpperBoundItemId::<Identity, Impl, OFFSET>,
            GetBatchLevelKnowledgeShouldBeApplied: GetBatchLevelKnowledgeShouldBeApplied::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBase_Impl: ::windows_core::BaseImpl {
    fn GetChangeEnumerator(this: &Self::This) -> ::windows_core::Result<IEnumSyncChanges>;
    fn GetIsLastBatch(this: &Self::This, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetWorkEstimateForBatch(this: &Self::This, pdwworkforbatch: *mut u32) -> ::windows_core::Result<()>;
    fn GetRemainingWorkEstimateForSession(this: &Self::This, pdwremainingworkforsession: *mut u32) -> ::windows_core::Result<()>;
    fn BeginOrderedGroup(this: &Self::This, pblowerbound: *const u8) -> ::windows_core::Result<()>;
    fn EndOrderedGroup(this: &Self::This, pbupperbound: *const u8, pmadewithknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<()>;
    fn AddItemMetadataToGroup(this: &Self::This, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder>;
    fn GetLearnedKnowledge(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetPrerequisiteKnowledge(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetSourceForgottenKnowledge(this: &Self::This) -> ::windows_core::Result<IForgottenKnowledge>;
    fn SetLastBatch(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetWorkEstimateForBatch(this: &Self::This, dwworkforbatch: u32) -> ::windows_core::Result<()>;
    fn SetRemainingWorkEstimateForSession(this: &Self::This, dwremainingworkforsession: u32) -> ::windows_core::Result<()>;
    fn Serialize(this: &Self::This, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncChangeBatchBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBatchBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChangeEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChangeEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIsLastBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIsLastBatch(this, ::core::mem::transmute_copy(&pflastbatch)).into())
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWorkEstimateForBatch(this, ::core::mem::transmute_copy(&pdwworkforbatch)).into())
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRemainingWorkEstimateForSession(this, ::core::mem::transmute_copy(&pdwremainingworkforsession)).into())
        }
        unsafe extern "system" fn BeginOrderedGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginOrderedGroup(this, ::core::mem::transmute_copy(&pblowerbound)).into())
        }
        unsafe extern "system" fn EndOrderedGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndOrderedGroup(this, ::core::mem::transmute_copy(&pbupperbound), ::windows_core::from_raw_borrowed(&pmadewithknowledge)).into())
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddItemMetadataToGroup(this, ::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchangebuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrerequisiteKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprerequisteknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceForgottenKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsourceforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastBatch(this).into())
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkEstimateForBatch(this, ::core::mem::transmute_copy(&dwworkforbatch)).into())
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemainingWorkEstimateForSession(this, ::core::mem::transmute_copy(&dwremainingworkforsession)).into())
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&pbchangebatch), ::core::mem::transmute_copy(&pcbchangebatch)).into())
        }
        ISyncChangeBatchBase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetChangeEnumerator: GetChangeEnumerator::<Identity, Impl, OFFSET>,
            GetIsLastBatch: GetIsLastBatch::<Identity, Impl, OFFSET>,
            GetWorkEstimateForBatch: GetWorkEstimateForBatch::<Identity, Impl, OFFSET>,
            GetRemainingWorkEstimateForSession: GetRemainingWorkEstimateForSession::<Identity, Impl, OFFSET>,
            BeginOrderedGroup: BeginOrderedGroup::<Identity, Impl, OFFSET>,
            EndOrderedGroup: EndOrderedGroup::<Identity, Impl, OFFSET>,
            AddItemMetadataToGroup: AddItemMetadataToGroup::<Identity, Impl, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetSourceForgottenKnowledge: GetSourceForgottenKnowledge::<Identity, Impl, OFFSET>,
            SetLastBatch: SetLastBatch::<Identity, Impl, OFFSET>,
            SetWorkEstimateForBatch: SetWorkEstimateForBatch::<Identity, Impl, OFFSET>,
            SetRemainingWorkEstimateForSession: SetRemainingWorkEstimateForSession::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchBase2_Impl: ::windows_core::BaseImpl + ISyncChangeBatchBase_Impl {
    fn SerializeWithOptions(this: &Self::This, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncChangeBatchBase2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncChangeBatchBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBatchBase2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SerializeWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchBase2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SerializeWithOptions(this, ::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into())
        }
        ISyncChangeBatchBase2_Vtbl {
            base__: <ISyncChangeBatchBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SerializeWithOptions: SerializeWithOptions::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncChangeBatchWithFilterKeyMap_Impl: ::windows_core::BaseImpl {
    fn GetFilterKeyMap(this: &Self::This) -> ::windows_core::Result<IFilterKeyMap>;
    fn SetFilterKeyMap(this: &Self::This, pifilterkeymap: ::core::option::Option<&IFilterKeyMap>) -> ::windows_core::Result<()>;
    fn SetFilterForgottenKnowledge(this: &Self::This, dwfilterkey: u32, pfilterforgottenknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<()>;
    fn GetFilteredReplicaLearnedKnowledge(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>;
}
impl ::windows_core::Iids for ISyncChangeBatchWithFilterKeyMap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBatchWithFilterKeyMap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilterKeyMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilterKeyMap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifilterkeymap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFilterKeyMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pifilterkeymap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilterKeyMap(this, ::windows_core::from_raw_borrowed(&pifilterkeymap)).into())
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilterForgottenKnowledge(this, ::core::mem::transmute_copy(&dwfilterkey), ::windows_core::from_raw_borrowed(&pfilterforgottenknowledge)).into())
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredReplicaLearnedKnowledge(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedFilterForgottenKnowledge(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedfilterforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedfilterforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncChangeBatchWithFilterKeyMap_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFilterKeyMap: GetFilterKeyMap::<Identity, Impl, OFFSET>,
            SetFilterKeyMap: SetFilterKeyMap::<Identity, Impl, OFFSET>,
            SetFilterForgottenKnowledge: SetFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeBatchWithPrerequisite_Impl: ::windows_core::BaseImpl + ISyncChangeBatchBase_Impl {
    fn SetPrerequisiteKnowledge(this: &Self::This, pprerequisiteknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<()>;
    fn GetLearnedKnowledgeWithPrerequisite(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(this: &Self::This) -> ::windows_core::Result<IForgottenKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncChangeBatchWithPrerequisite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncChangeBatchBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBatchWithPrerequisite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrerequisiteKnowledge(this, ::windows_core::from_raw_borrowed(&pprerequisiteknowledge)).into())
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedwithprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedKnowledgeWithPrerequisite(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedwithprerequisiteknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedForgottenKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncChangeBatchWithPrerequisite_Vtbl {
            base__: <ISyncChangeBatchBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPrerequisiteKnowledge: SetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncChangeBuilder_Impl: ::windows_core::BaseImpl {
    fn AddChangeUnitMetadata(this: &Self::This, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncChangeBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddChangeUnitMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddChangeUnitMetadata(this, ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pchangeunitversion)).into())
        }
        ISyncChangeBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddChangeUnitMetadata: AddChangeUnitMetadata::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncChangeUnit_Impl: ::windows_core::BaseImpl {
    fn GetItemChange(this: &Self::This) -> ::windows_core::Result<ISyncChange>;
    fn GetChangeUnitId(this: &Self::This, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetChangeUnitVersion(this: &Self::This, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncChangeUnit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeUnit {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeUnitId(this, ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetChangeUnitVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeUnitVersion(this, ::core::mem::transmute_copy(&pbcurrentreplicaid), ::core::mem::transmute_copy(&pversion)).into())
        }
        ISyncChangeUnit_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemChange: GetItemChange::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
            GetChangeUnitVersion: GetChangeUnitVersion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncChangeWithFilterKeyMap_Impl: ::windows_core::BaseImpl {
    fn GetFilterCount(this: &Self::This, pdwfiltercount: *mut u32) -> ::windows_core::Result<()>;
    fn GetFilterChange(this: &Self::This, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows_core::Result<()>;
    fn GetAllChangeUnitsPresentFlag(this: &Self::This, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFilterForgottenKnowledge(this: &Self::This, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedKnowledge(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>, pnewmoveins: ::core::option::Option<&IEnumItemIds>, dwfilterkey: u32) -> ::windows_core::Result<ISyncKnowledge>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncChangeWithFilterKeyMap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeWithFilterKeyMap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilterCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilterCount(this, ::core::mem::transmute_copy(&pdwfiltercount)).into())
        }
        unsafe extern "system" fn GetFilterChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilterChange(this, ::core::mem::transmute_copy(&dwfilterkey), ::core::mem::transmute_copy(&pfilterchange)).into())
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAllChangeUnitsPresentFlag(this, ::core::mem::transmute_copy(&pfallchangeunitspresent)).into())
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilterForgottenKnowledge(this, ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifilterforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredReplicaLearnedKnowledge(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedFilterForgottenKnowledge(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedfilterforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge), ::windows_core::from_raw_borrowed(&pnewmoveins), ::core::mem::transmute_copy(&dwfilterkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedfilterforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncChangeWithFilterKeyMap_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFilterCount: GetFilterCount::<Identity, Impl, OFFSET>,
            GetFilterChange: GetFilterChange::<Identity, Impl, OFFSET>,
            GetAllChangeUnitsPresentFlag: GetAllChangeUnitsPresentFlag::<Identity, Impl, OFFSET>,
            GetFilterForgottenKnowledge: GetFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncChangeWithPrerequisite_Impl: ::windows_core::BaseImpl {
    fn GetPrerequisiteKnowledge(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledgeWithPrerequisite(this: &Self::This, pdestinationknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<ISyncKnowledge>;
}
impl ::windows_core::Iids for ISyncChangeWithPrerequisite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncChangeWithPrerequisite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrerequisiteKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprerequisiteknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedknowledgewithprerequisite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedKnowledgeWithPrerequisite(this, ::windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedknowledgewithprerequisite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncChangeWithPrerequisite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncConstraintCallback_Impl: ::windows_core::BaseImpl {
    fn OnConstraintConflict(this: &Self::This, pconflict: ::core::option::Option<&IConstraintConflict>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncConstraintCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncConstraintCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncConstraintCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConstraintConflict<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncConstraintCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconflict: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConstraintConflict(this, ::windows_core::from_raw_borrowed(&pconflict)).into())
        }
        ISyncConstraintCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnConstraintConflict: OnConstraintConflict::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncDataConverter_Impl: ::windows_core::BaseImpl {
    fn ConvertDataRetrieverFromProviderFormat(this: &Self::This, punkdataretrieverin: ::core::option::Option<&::windows_core::IUnknown>, penumsyncchanges: ::core::option::Option<&IEnumSyncChanges>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ConvertDataRetrieverToProviderFormat(this: &Self::This, punkdataretrieverin: ::core::option::Option<&::windows_core::IUnknown>, penumsyncchanges: ::core::option::Option<&IEnumSyncChanges>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ConvertDataFromProviderFormat(this: &Self::This, pdatacontext: ::core::option::Option<&ILoadChangeContext>, punkdatain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ConvertDataToProviderFormat(this: &Self::This, pdatacontext: ::core::option::Option<&ILoadChangeContext>, punkdataout: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for ISyncDataConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncDataConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConvertDataRetrieverFromProviderFormat(this, ::windows_core::from_raw_borrowed(&punkdataretrieverin), ::windows_core::from_raw_borrowed(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdataout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConvertDataRetrieverToProviderFormat(this, ::windows_core::from_raw_borrowed(&punkdataretrieverin), ::windows_core::from_raw_borrowed(&penumsyncchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdataout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatacontext: *mut ::core::ffi::c_void, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConvertDataFromProviderFormat(this, ::windows_core::from_raw_borrowed(&pdatacontext), ::windows_core::from_raw_borrowed(&punkdatain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdataout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatacontext: *mut ::core::ffi::c_void, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConvertDataToProviderFormat(this, ::windows_core::from_raw_borrowed(&pdatacontext), ::windows_core::from_raw_borrowed(&punkdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdataout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncDataConverter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConvertDataRetrieverFromProviderFormat: ConvertDataRetrieverFromProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataRetrieverToProviderFormat: ConvertDataRetrieverToProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataFromProviderFormat: ConvertDataFromProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataToProviderFormat: ConvertDataToProviderFormat::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncFilter_Impl: ::windows_core::BaseImpl {
    fn IsIdentical(this: &Self::This, psyncfilter: ::core::option::Option<&ISyncFilter>) -> ::windows_core::Result<()>;
    fn Serialize(this: &Self::This, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsIdentical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsIdentical(this, ::windows_core::from_raw_borrowed(&psyncfilter)).into())
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&pbsyncfilter), ::core::mem::transmute_copy(&pcbsyncfilter)).into())
        }
        ISyncFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsIdentical: IsIdentical::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncFilterDeserializer_Impl: ::windows_core::BaseImpl {
    fn DeserializeSyncFilter(this: &Self::This, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows_core::Result<ISyncFilter>;
}
impl ::windows_core::Iids for ISyncFilterDeserializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilterDeserializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncFilterDeserializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeserializeSyncFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilterDeserializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeserializeSyncFilter(this, ::core::mem::transmute_copy(&pbsyncfilter), ::core::mem::transmute_copy(&dwcbsyncfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppisyncfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncFilterDeserializer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeserializeSyncFilter: DeserializeSyncFilter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncFilterInfo_Impl: ::windows_core::BaseImpl {
    fn Serialize(this: &Self::This, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncFilterInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilterInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncFilterInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbbuffer)).into())
        }
        ISyncFilterInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Serialize: Serialize::<Identity, Impl, OFFSET> }
    };
}
pub trait ISyncFilterInfo2_Impl: ::windows_core::BaseImpl + ISyncFilterInfo_Impl {
    fn GetFlags(this: &Self::This, pdwflags: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncFilterInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncFilterInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilterInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncFilterInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFilterInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlags(this, ::core::mem::transmute_copy(&pdwflags)).into())
        }
        ISyncFilterInfo2_Vtbl { base__: <ISyncFilterInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetFlags: GetFlags::<Identity, Impl, OFFSET> }
    };
}
pub trait ISyncFullEnumerationChange_Impl: ::windows_core::BaseImpl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(this: &Self::This) -> ::windows_core::Result<IForgottenKnowledge>;
}
impl ::windows_core::Iids for ISyncFullEnumerationChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncFullEnumerationChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedForgottenKnowledge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedforgottenknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncFullEnumerationChange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch_Impl: ::windows_core::BaseImpl + ISyncChangeBatchBase_Impl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn GetClosedLowerBoundItemId(this: &Self::This, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetClosedUpperBoundItemId(this: &Self::This, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncFullEnumerationChangeBatch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncChangeBatchBase);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncFullEnumerationChangeBatch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplearnedknowledgeafterrecoverycomplete, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClosedLowerBoundItemId(this, ::core::mem::transmute_copy(&pbclosedlowerbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClosedUpperBoundItemId(this, ::core::mem::transmute_copy(&pbclosedupperbounditemid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        ISyncFullEnumerationChangeBatch_Vtbl {
            base__: <ISyncChangeBatchBase as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetClosedLowerBoundItemId: GetClosedLowerBoundItemId::<Identity, Impl, OFFSET>,
            GetClosedUpperBoundItemId: GetClosedUpperBoundItemId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncFullEnumerationChangeBatch2_Impl: ::windows_core::BaseImpl + ISyncFullEnumerationChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(this: &Self::This, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows_core::Result<ISyncChangeBuilder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncFullEnumerationChangeBatch2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncFullEnumerationChangeBatch);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncFullEnumerationChangeBatch2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddMergeTombstoneMetadataToGroup(this, ::core::mem::transmute_copy(&pbownerreplicaid), ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pchangeversion), ::core::mem::transmute_copy(&pcreationversion), ::core::mem::transmute_copy(&dwworkforchange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchangebuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncFullEnumerationChangeBatch2_Vtbl {
            base__: <ISyncFullEnumerationChangeBatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledge_Impl: ::windows_core::BaseImpl {
    fn GetOwnerReplicaId(this: &Self::This, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
    fn Serialize(this: &Self::This, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::Result<()>;
    fn SetLocalTickCount(this: &Self::This, ulltickcount: u64) -> ::windows_core::Result<()>;
    fn ContainsChange(this: &Self::This, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()>;
    fn ContainsChangeUnit(this: &Self::This, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::Result<()>;
    fn GetScopeVector(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetReplicaKeyMap(this: &Self::This) -> ::windows_core::Result<IReplicaKeyMap>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ISyncKnowledge>;
    fn ConvertVersion(this: &Self::This, pknowledgein: ::core::option::Option<&ISyncKnowledge>, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::Result<()>;
    fn MapRemoteToLocal(this: &Self::This, premoteknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<ISyncKnowledge>;
    fn Union(this: &Self::This, pknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<()>;
    fn ProjectOntoItem(this: &Self::This, pbitemid: *const u8) -> ::windows_core::Result<ISyncKnowledge>;
    fn ProjectOntoChangeUnit(this: &Self::This, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<ISyncKnowledge>;
    fn ProjectOntoRange(this: &Self::This, psrngsyncrange: *const SYNC_RANGE) -> ::windows_core::Result<ISyncKnowledge>;
    fn ExcludeItem(this: &Self::This, pbitemid: *const u8) -> ::windows_core::Result<()>;
    fn ExcludeChangeUnit(this: &Self::This, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()>;
    fn ContainsKnowledge(this: &Self::This, pknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<()>;
    fn FindMinTickCountForReplica(this: &Self::This, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::Result<()>;
    fn GetRangeExceptions(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetSingleItemExceptions(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetChangeUnitExceptions(this: &Self::This, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FindClockVectorForItem(this: &Self::This, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FindClockVectorForChangeUnit(this: &Self::This, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetVersion(this: &Self::This, pdwversion: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncKnowledge {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncKnowledge {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOwnerReplicaId(this, ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&fserializereplicakeymap), ::core::mem::transmute_copy(&pbknowledge), ::core::mem::transmute_copy(&pcbknowledge)).into())
        }
        unsafe extern "system" fn SetLocalTickCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalTickCount(this, ::core::mem::transmute_copy(&ulltickcount)).into())
        }
        unsafe extern "system" fn ContainsChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContainsChange(this, ::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pgiditemid), ::core::mem::transmute_copy(&psyncversion)).into())
        }
        unsafe extern "system" fn ContainsChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContainsChangeUnit(this, ::core::mem::transmute_copy(&pbversionownerreplicaid), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&psyncversion)).into())
        }
        unsafe extern "system" fn GetScopeVector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScopeVector(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn GetReplicaKeyMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReplicaKeyMap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreplicakeymap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclonedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledgein: *mut ::core::ffi::c_void, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertVersion(this, ::windows_core::from_raw_borrowed(&pknowledgein), ::core::mem::transmute_copy(&pbcurrentownerid), ::core::mem::transmute_copy(&pversionin), ::core::mem::transmute_copy(&pbnewownerid), ::core::mem::transmute_copy(&pcbidsize), ::core::mem::transmute_copy(&pversionout)).into())
        }
        unsafe extern "system" fn MapRemoteToLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, premoteknowledge: *mut ::core::ffi::c_void, ppmappedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapRemoteToLocal(this, ::windows_core::from_raw_borrowed(&premoteknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmappedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Union<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Union(this, ::windows_core::from_raw_borrowed(&pknowledge)).into())
        }
        unsafe extern "system" fn ProjectOntoItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProjectOntoItem(this, ::core::mem::transmute_copy(&pbitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppknowledgeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProjectOntoChangeUnit(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppknowledgeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProjectOntoRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProjectOntoRange(this, ::core::mem::transmute_copy(&psrngsyncrange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppknowledgeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExcludeItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExcludeItem(this, ::core::mem::transmute_copy(&pbitemid)).into())
        }
        unsafe extern "system" fn ExcludeChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExcludeChangeUnit(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into())
        }
        unsafe extern "system" fn ContainsKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContainsKnowledge(this, ::windows_core::from_raw_borrowed(&pknowledge)).into())
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindMinTickCountForReplica(this, ::core::mem::transmute_copy(&pbreplicaid), ::core::mem::transmute_copy(&pullreplicatickcount)).into())
        }
        unsafe extern "system" fn GetRangeExceptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRangeExceptions(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn GetSingleItemExceptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSingleItemExceptions(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChangeUnitExceptions(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn FindClockVectorForItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindClockVectorForItem(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindClockVectorForChangeUnit(this, ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersion(this, ::core::mem::transmute_copy(&pdwversion)).into())
        }
        ISyncKnowledge_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            SetLocalTickCount: SetLocalTickCount::<Identity, Impl, OFFSET>,
            ContainsChange: ContainsChange::<Identity, Impl, OFFSET>,
            ContainsChangeUnit: ContainsChangeUnit::<Identity, Impl, OFFSET>,
            GetScopeVector: GetScopeVector::<Identity, Impl, OFFSET>,
            GetReplicaKeyMap: GetReplicaKeyMap::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            ConvertVersion: ConvertVersion::<Identity, Impl, OFFSET>,
            MapRemoteToLocal: MapRemoteToLocal::<Identity, Impl, OFFSET>,
            Union: Union::<Identity, Impl, OFFSET>,
            ProjectOntoItem: ProjectOntoItem::<Identity, Impl, OFFSET>,
            ProjectOntoChangeUnit: ProjectOntoChangeUnit::<Identity, Impl, OFFSET>,
            ProjectOntoRange: ProjectOntoRange::<Identity, Impl, OFFSET>,
            ExcludeItem: ExcludeItem::<Identity, Impl, OFFSET>,
            ExcludeChangeUnit: ExcludeChangeUnit::<Identity, Impl, OFFSET>,
            ContainsKnowledge: ContainsKnowledge::<Identity, Impl, OFFSET>,
            FindMinTickCountForReplica: FindMinTickCountForReplica::<Identity, Impl, OFFSET>,
            GetRangeExceptions: GetRangeExceptions::<Identity, Impl, OFFSET>,
            GetSingleItemExceptions: GetSingleItemExceptions::<Identity, Impl, OFFSET>,
            GetChangeUnitExceptions: GetChangeUnitExceptions::<Identity, Impl, OFFSET>,
            FindClockVectorForItem: FindClockVectorForItem::<Identity, Impl, OFFSET>,
            FindClockVectorForChangeUnit: FindClockVectorForChangeUnit::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncKnowledge2_Impl: ::windows_core::BaseImpl + ISyncKnowledge_Impl {
    fn GetIdParameters(this: &Self::This, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()>;
    fn ProjectOntoColumnSet(this: &Self::This, ppcolumns: *const *const u8, count: u32) -> ::windows_core::Result<ISyncKnowledge2>;
    fn SerializeWithOptions(this: &Self::This, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetLowestUncontainedId(this: &Self::This, pisyncknowledge: ::core::option::Option<&ISyncKnowledge2>, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetInspector(this: &Self::This, riid: *const ::windows_core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetMinimumSupportedVersion(this: &Self::This, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows_core::Result<()>;
    fn GetStatistics(this: &Self::This, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows_core::Result<()>;
    fn ContainsKnowledgeForItem(this: &Self::This, pknowledge: ::core::option::Option<&ISyncKnowledge>, pbitemid: *const u8) -> ::windows_core::Result<()>;
    fn ContainsKnowledgeForChangeUnit(this: &Self::This, pknowledge: ::core::option::Option<&ISyncKnowledge>, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::Result<()>;
    fn ProjectOntoKnowledgeWithPrerequisite(this: &Self::This, pprerequisiteknowledge: ::core::option::Option<&ISyncKnowledge>, ptemplateknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<ISyncKnowledge>;
    fn Complement(this: &Self::This, psyncknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<ISyncKnowledge>;
    fn IntersectsWithKnowledge(this: &Self::This, psyncknowledge: ::core::option::Option<&ISyncKnowledge>) -> ::windows_core::Result<()>;
    fn GetKnowledgeCookie(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CompareToKnowledgeCookie(this: &Self::This, pknowledgecookie: ::core::option::Option<&::windows_core::IUnknown>, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncKnowledge2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncKnowledge);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncKnowledge2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdParameters(this, ::core::mem::transmute_copy(&pidparameters)).into())
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProjectOntoColumnSet(this, ::core::mem::transmute_copy(&ppcolumns), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiknowledgeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SerializeWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SerializeWithOptions(this, ::core::mem::transmute_copy(&targetformatversion), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pdwserializedsize)).into())
        }
        unsafe extern "system" fn GetLowestUncontainedId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisyncknowledge: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLowestUncontainedId(this, ::windows_core::from_raw_borrowed(&pisyncknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pcbitemidsize)).into())
        }
        unsafe extern "system" fn GetInspector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInspector(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppiinspector)).into())
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMinimumSupportedVersion(this, ::core::mem::transmute_copy(&pversion)).into())
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatistics(this, ::core::mem::transmute_copy(&which), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContainsKnowledgeForItem(this, ::windows_core::from_raw_borrowed(&pknowledge), ::core::mem::transmute_copy(&pbitemid)).into())
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContainsKnowledgeForChangeUnit(this, ::windows_core::from_raw_borrowed(&pknowledge), ::core::mem::transmute_copy(&pbitemid), ::core::mem::transmute_copy(&pbchangeunitid)).into())
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: *mut ::core::ffi::c_void, ptemplateknowledge: *mut ::core::ffi::c_void, ppprojectedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProjectOntoKnowledgeWithPrerequisite(this, ::windows_core::from_raw_borrowed(&pprerequisiteknowledge), ::windows_core::from_raw_borrowed(&ptemplateknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprojectedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Complement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncknowledge: *mut ::core::ffi::c_void, ppcomplementedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Complement(this, ::windows_core::from_raw_borrowed(&psyncknowledge)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomplementedknowledge, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncknowledge: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IntersectsWithKnowledge(this, ::windows_core::from_raw_borrowed(&psyncknowledge)).into())
        }
        unsafe extern "system" fn GetKnowledgeCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKnowledgeCookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppknowledgecookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompareToKnowledgeCookie(this, ::windows_core::from_raw_borrowed(&pknowledgecookie), ::core::mem::transmute_copy(&presult)).into())
        }
        ISyncKnowledge2_Vtbl {
            base__: <ISyncKnowledge as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            ProjectOntoColumnSet: ProjectOntoColumnSet::<Identity, Impl, OFFSET>,
            SerializeWithOptions: SerializeWithOptions::<Identity, Impl, OFFSET>,
            GetLowestUncontainedId: GetLowestUncontainedId::<Identity, Impl, OFFSET>,
            GetInspector: GetInspector::<Identity, Impl, OFFSET>,
            GetMinimumSupportedVersion: GetMinimumSupportedVersion::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            ContainsKnowledgeForItem: ContainsKnowledgeForItem::<Identity, Impl, OFFSET>,
            ContainsKnowledgeForChangeUnit: ContainsKnowledgeForChangeUnit::<Identity, Impl, OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite: ProjectOntoKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
            Complement: Complement::<Identity, Impl, OFFSET>,
            IntersectsWithKnowledge: IntersectsWithKnowledge::<Identity, Impl, OFFSET>,
            GetKnowledgeCookie: GetKnowledgeCookie::<Identity, Impl, OFFSET>,
            CompareToKnowledgeCookie: CompareToKnowledgeCookie::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncMergeTombstoneChange_Impl: ::windows_core::BaseImpl {
    fn GetWinnerItemId(this: &Self::This, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISyncMergeTombstoneChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncMergeTombstoneChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncMergeTombstoneChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWinnerItemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncMergeTombstoneChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWinnerItemId(this, ::core::mem::transmute_copy(&pbwinneritemid), ::core::mem::transmute_copy(&pcbidsize)).into())
        }
        ISyncMergeTombstoneChange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWinnerItemId: GetWinnerItemId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncProvider_Impl: ::windows_core::BaseImpl {
    fn GetIdParameters(this: &Self::This, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdParameters(this, ::core::mem::transmute_copy(&pidparameters)).into())
        }
        ISyncProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUI_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pconfigurationproperties: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn GetRegisteredProperties(this: &Self::This) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateAndRegisterNewSyncProvider(this: &Self::This, hwndparent: super::super::Foundation::HWND, punkcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<ISyncProviderInfo>;
    fn ModifySyncProvider(this: &Self::This, hwndparent: super::super::Foundation::HWND, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pproviderinfo: ::core::option::Option<&ISyncProviderInfo>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISyncProviderConfigUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncProviderConfigUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pguidcontenttype: *const ::windows_core::GUID, pconfigurationproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&pguidcontenttype), ::windows_core::from_raw_borrowed(&pconfigurationproperties)).into())
        }
        unsafe extern "system" fn GetRegisteredProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegisteredProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfiguiproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAndRegisterNewSyncProvider(this, ::core::mem::transmute_copy(&hwndparent), ::windows_core::from_raw_borrowed(&punkcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproviderinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifySyncProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifySyncProvider(this, ::core::mem::transmute_copy(&hwndparent), ::windows_core::from_raw_borrowed(&punkcontext), ::windows_core::from_raw_borrowed(&pproviderinfo)).into())
        }
        ISyncProviderConfigUI_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            GetRegisteredProperties: GetRegisteredProperties::<Identity, Impl, OFFSET>,
            CreateAndRegisterNewSyncProvider: CreateAndRegisterNewSyncProvider::<Identity, Impl, OFFSET>,
            ModifySyncProvider: ModifySyncProvider::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUIInfo_Impl: ::windows_core::BaseImpl + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProviderConfigUI(this: &Self::This, dwclscontext: u32) -> ::windows_core::Result<ISyncProviderConfigUI>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISyncProviderConfigUIInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::UI::Shell::PropertiesSystem::IPropertyStore);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderConfigUIInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncProviderConfigUIInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncProviderConfigUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderConfigUIInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderConfigUI(this, ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncproviderconfigui, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncProviderConfigUIInfo_Vtbl {
            base__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSyncProviderConfigUI: GetSyncProviderConfigUI::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderInfo_Impl: ::windows_core::BaseImpl + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProvider(this: &Self::This, dwclscontext: u32) -> ::windows_core::Result<IRegisteredSyncProvider>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISyncProviderInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::UI::Shell::PropertiesSystem::IPropertyStore);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncProviderInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProvider(this, ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncProviderInfo_Vtbl {
            base__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSyncProvider: GetSyncProvider::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderRegistration_Impl: ::windows_core::BaseImpl {
    fn CreateSyncProviderConfigUIRegistrationInstance(this: &Self::This, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> ::windows_core::Result<ISyncProviderConfigUIInfo>;
    fn UnregisterSyncProviderConfigUI(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumerateSyncProviderConfigUIs(this: &Self::This, pguidcontenttype: *const ::windows_core::GUID, dwsupportedarchitecture: u32) -> ::windows_core::Result<IEnumSyncProviderConfigUIInfos>;
    fn CreateSyncProviderRegistrationInstance(this: &Self::This, pproviderconfiguration: *const SyncProviderConfiguration) -> ::windows_core::Result<ISyncProviderInfo>;
    fn UnregisterSyncProvider(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetSyncProviderConfigUIInfoforProvider(this: &Self::This, pguidproviderinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderConfigUIInfo>;
    fn EnumerateSyncProviders(this: &Self::This, pguidcontenttype: *const ::windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows_core::GUID, dwsupportedarchitecture: u32) -> ::windows_core::Result<IEnumSyncProviderInfos>;
    fn GetSyncProviderInfo(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderInfo>;
    fn GetSyncProviderFromInstanceId(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32) -> ::windows_core::Result<IRegisteredSyncProvider>;
    fn GetSyncProviderConfigUIInfo(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<ISyncProviderConfigUIInfo>;
    fn GetSyncProviderConfigUIFromInstanceId(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32) -> ::windows_core::Result<ISyncProviderConfigUI>;
    fn GetSyncProviderState(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn SetSyncProviderState(this: &Self::This, pguidinstanceid: *const ::windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows_core::Result<()>;
    fn RegisterForEvent(this: &Self::This, phevent: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn RevokeEvent(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn GetChange(this: &Self::This, hevent: super::super::Foundation::HANDLE) -> ::windows_core::Result<ISyncRegistrationChange>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISyncProviderRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncProviderRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSyncProviderConfigUIRegistrationInstance(this, ::core::mem::transmute_copy(&pconfiguiconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfiguiinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterSyncProviderConfigUI(this, ::core::mem::transmute_copy(&pguidinstanceid)).into())
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateSyncProviderConfigUIs(this, ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumsyncproviderconfiguiinfos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSyncProviderRegistrationInstance(this, ::core::mem::transmute_copy(&pproviderconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproviderinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterSyncProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterSyncProvider(this, ::core::mem::transmute_copy(&pguidinstanceid)).into())
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows_core::GUID, ppproviderconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderConfigUIInfoforProvider(this, ::core::mem::transmute_copy(&pguidproviderinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproviderconfiguiinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateSyncProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateSyncProviders(this, ::core::mem::transmute_copy(&pguidcontenttype), ::core::mem::transmute_copy(&dwstateflagstofiltermask), ::core::mem::transmute_copy(&dwstateflagstofilter), ::core::mem::transmute_copy(&refproviderclsid), ::core::mem::transmute_copy(&dwsupportedarchitecture)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumsyncproviderinfos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSyncProviderInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderInfo(this, ::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproviderinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderFromInstanceId(this, ::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderConfigUIInfo(this, ::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfiguiinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwclscontext: u32, ppconfigui: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderConfigUIFromInstanceId(this, ::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwclscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfigui, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSyncProviderState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, pdwstateflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderState(this, ::core::mem::transmute_copy(&pguidinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstateflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSyncProviderState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyncProviderState(this, ::core::mem::transmute_copy(&pguidinstanceid), ::core::mem::transmute_copy(&dwstateflagsmask), ::core::mem::transmute_copy(&dwstateflags)).into())
        }
        unsafe extern "system" fn RegisterForEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterForEvent(this, ::core::mem::transmute_copy(&phevent)).into())
        }
        unsafe extern "system" fn RevokeEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeEvent(this, ::core::mem::transmute_copy(&hevent)).into())
        }
        unsafe extern "system" fn GetChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChange(this, ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncProviderRegistration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSyncProviderConfigUIRegistrationInstance: CreateSyncProviderConfigUIRegistrationInstance::<Identity, Impl, OFFSET>,
            UnregisterSyncProviderConfigUI: UnregisterSyncProviderConfigUI::<Identity, Impl, OFFSET>,
            EnumerateSyncProviderConfigUIs: EnumerateSyncProviderConfigUIs::<Identity, Impl, OFFSET>,
            CreateSyncProviderRegistrationInstance: CreateSyncProviderRegistrationInstance::<Identity, Impl, OFFSET>,
            UnregisterSyncProvider: UnregisterSyncProvider::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIInfoforProvider: GetSyncProviderConfigUIInfoforProvider::<Identity, Impl, OFFSET>,
            EnumerateSyncProviders: EnumerateSyncProviders::<Identity, Impl, OFFSET>,
            GetSyncProviderInfo: GetSyncProviderInfo::<Identity, Impl, OFFSET>,
            GetSyncProviderFromInstanceId: GetSyncProviderFromInstanceId::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIInfo: GetSyncProviderConfigUIInfo::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIFromInstanceId: GetSyncProviderConfigUIFromInstanceId::<Identity, Impl, OFFSET>,
            GetSyncProviderState: GetSyncProviderState::<Identity, Impl, OFFSET>,
            SetSyncProviderState: SetSyncProviderState::<Identity, Impl, OFFSET>,
            RegisterForEvent: RegisterForEvent::<Identity, Impl, OFFSET>,
            RevokeEvent: RevokeEvent::<Identity, Impl, OFFSET>,
            GetChange: GetChange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncRegistrationChange_Impl: ::windows_core::BaseImpl {
    fn GetEvent(this: &Self::This) -> ::windows_core::Result<SYNC_REGISTRATION_EVENT>;
    fn GetInstanceId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for ISyncRegistrationChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncRegistrationChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncRegistrationChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncRegistrationChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEvent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psreevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncRegistrationChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstanceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncRegistrationChange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISyncSessionExtendedErrorInfo_Impl: ::windows_core::BaseImpl {
    fn GetSyncProviderWithError(this: &Self::This) -> ::windows_core::Result<ISyncProvider>;
}
impl ::windows_core::Iids for ISyncSessionExtendedErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncSessionExtendedErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncProviderWithError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncProviderWithError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproviderwitherror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyncSessionExtendedErrorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSyncProviderWithError: GetSyncProviderWithError::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionState_Impl: ::windows_core::BaseImpl {
    fn IsCanceled(this: &Self::This, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetInfoForChangeApplication(this: &Self::This, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::Result<()>;
    fn LoadInfoFromChangeApplication(this: &Self::This, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeStart(this: &Self::This, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeEnd(this: &Self::This, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::Result<()>;
    fn SetForgottenKnowledgeRecoveryRange(this: &Self::This, prange: *const SYNC_RANGE) -> ::windows_core::Result<()>;
    fn OnProgress(this: &Self::This, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncSessionState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncSessionState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsCanceled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsCanceled(this, ::core::mem::transmute_copy(&pfiscanceled)).into())
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfoForChangeApplication(this, ::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&pcbchangeapplierinfo)).into())
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadInfoFromChangeApplication(this, ::core::mem::transmute_copy(&pbchangeapplierinfo), ::core::mem::transmute_copy(&cbchangeapplierinfo)).into())
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForgottenKnowledgeRecoveryRangeStart(this, ::core::mem::transmute_copy(&pbrangestart), ::core::mem::transmute_copy(&pcbrangestart)).into())
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForgottenKnowledgeRecoveryRangeEnd(this, ::core::mem::transmute_copy(&pbrangeend), ::core::mem::transmute_copy(&pcbrangeend)).into())
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForgottenKnowledgeRecoveryRange(this, ::core::mem::transmute_copy(&prange)).into())
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&provider), ::core::mem::transmute_copy(&syncstage), ::core::mem::transmute_copy(&dwcompletedwork), ::core::mem::transmute_copy(&dwtotalwork)).into())
        }
        ISyncSessionState_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsCanceled: IsCanceled::<Identity, Impl, OFFSET>,
            GetInfoForChangeApplication: GetInfoForChangeApplication::<Identity, Impl, OFFSET>,
            LoadInfoFromChangeApplication: LoadInfoFromChangeApplication::<Identity, Impl, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart: GetForgottenKnowledgeRecoveryRangeStart::<Identity, Impl, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd: GetForgottenKnowledgeRecoveryRangeEnd::<Identity, Impl, OFFSET>,
            SetForgottenKnowledgeRecoveryRange: SetForgottenKnowledgeRecoveryRange::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISyncSessionState2_Impl: ::windows_core::BaseImpl + ISyncSessionState_Impl {
    fn SetProviderWithError(this: &Self::This, fself: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSessionErrorStatus(this: &Self::This, phrsessionerror: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISyncSessionState2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISyncSessionState);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyncSessionState2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProviderWithError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProviderWithError(this, ::core::mem::transmute_copy(&fself)).into())
        }
        unsafe extern "system" fn GetSessionErrorStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyncSessionState2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSessionErrorStatus(this, ::core::mem::transmute_copy(&phrsessionerror)).into())
        }
        ISyncSessionState2_Vtbl {
            base__: <ISyncSessionState as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProviderWithError: SetProviderWithError::<Identity, Impl, OFFSET>,
            GetSessionErrorStatus: GetSessionErrorStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronousDataRetriever_Impl: ::windows_core::BaseImpl {
    fn GetIdParameters(this: &Self::This, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::Result<()>;
    fn LoadChangeData(this: &Self::This, ploadchangecontext: ::core::option::Option<&ILoadChangeContext>) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISynchronousDataRetriever {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronousDataRetriever_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISynchronousDataRetriever {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronousDataRetriever_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdParameters(this, ::core::mem::transmute_copy(&pidparameters)).into())
        }
        unsafe extern "system" fn LoadChangeData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronousDataRetriever_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploadchangecontext: *mut ::core::ffi::c_void, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadChangeData(this, ::windows_core::from_raw_borrowed(&ploadchangecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISynchronousDataRetriever_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, Impl, OFFSET>,
        }
    };
}
