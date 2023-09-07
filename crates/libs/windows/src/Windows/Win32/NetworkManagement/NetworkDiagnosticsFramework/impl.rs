#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagExtensibleHelper_Impl: ::windows_core::BaseImpl {
    fn ResolveAttributes(this: &Self::This, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetDiagExtensibleHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagExtensibleHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetDiagExtensibleHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResolveAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagExtensibleHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveAttributes(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgkeyattributes), ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&prgmatchvalues)).into())
        }
        INetDiagExtensibleHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResolveAttributes: ResolveAttributes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelper_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows_core::Result<()>;
    fn GetDiagnosticsInfo(this: &Self::This) -> ::windows_core::Result<*mut DiagnosticsInfo>;
    fn GetKeyAttributes(this: &Self::This, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::Result<()>;
    fn LowHealth(this: &Self::This, pwszinstancedescription: &::windows_core::PCWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::Result<()>;
    fn HighUtilization(this: &Self::This, pwszinstancedescription: &::windows_core::PCWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::Result<()>;
    fn GetLowerHypotheses(this: &Self::This, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()>;
    fn GetDownStreamHypotheses(this: &Self::This, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()>;
    fn GetHigherHypotheses(this: &Self::This, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()>;
    fn GetUpStreamHypotheses(this: &Self::This, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::Result<()>;
    fn Repair(this: &Self::This, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::Result<()>;
    fn Validate(this: &Self::This, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::Result<()>;
    fn GetRepairInfo(this: &Self::This, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows_core::Result<()>;
    fn GetLifeTime(this: &Self::This) -> ::windows_core::Result<LIFE_TIME>;
    fn SetLifeTime(this: &Self::This, lifetime: &LIFE_TIME) -> ::windows_core::Result<()>;
    fn GetCacheTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetAttributes(this: &Self::This, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cleanup(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetDiagHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetDiagHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgattributes)).into())
        }
        unsafe extern "system" fn GetDiagnosticsInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDiagnosticsInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKeyAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKeyAttributes(this, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)).into())
        }
        unsafe extern "system" fn LowHealth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows_core::PCWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LowHealth(this, ::core::mem::transmute(&pwszinstancedescription), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn HighUtilization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows_core::PCWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HighUtilization(this, ::core::mem::transmute(&pwszinstancedescription), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn GetLowerHypotheses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLowerHypotheses(this, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into())
        }
        unsafe extern "system" fn GetDownStreamHypotheses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDownStreamHypotheses(this, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into())
        }
        unsafe extern "system" fn GetHigherHypotheses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHigherHypotheses(this, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into())
        }
        unsafe extern "system" fn GetUpStreamHypotheses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUpStreamHypotheses(this, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprghypotheses)).into())
        }
        unsafe extern "system" fn Repair<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Repair(this, ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Validate(this, ::core::mem::transmute_copy(&problem), ::core::mem::transmute_copy(&pdeferredtime), ::core::mem::transmute_copy(&pstatus)).into())
        }
        unsafe extern "system" fn GetRepairInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRepairInfo(this, ::core::mem::transmute_copy(&problem), ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&ppinfo)).into())
        }
        unsafe extern "system" fn GetLifeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plifetime: *mut LIFE_TIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLifeTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plifetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLifeTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lifetime: LIFE_TIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLifeTime(this, ::core::mem::transmute(&lifetime)).into())
        }
        unsafe extern "system" fn GetCacheTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCacheTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcachetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributes(this, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributes)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Cleanup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cleanup(this).into())
        }
        INetDiagHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetDiagnosticsInfo: GetDiagnosticsInfo::<Identity, Impl, OFFSET>,
            GetKeyAttributes: GetKeyAttributes::<Identity, Impl, OFFSET>,
            LowHealth: LowHealth::<Identity, Impl, OFFSET>,
            HighUtilization: HighUtilization::<Identity, Impl, OFFSET>,
            GetLowerHypotheses: GetLowerHypotheses::<Identity, Impl, OFFSET>,
            GetDownStreamHypotheses: GetDownStreamHypotheses::<Identity, Impl, OFFSET>,
            GetHigherHypotheses: GetHigherHypotheses::<Identity, Impl, OFFSET>,
            GetUpStreamHypotheses: GetUpStreamHypotheses::<Identity, Impl, OFFSET>,
            Repair: Repair::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
            GetRepairInfo: GetRepairInfo::<Identity, Impl, OFFSET>,
            GetLifeTime: GetLifeTime::<Identity, Impl, OFFSET>,
            SetLifeTime: SetLifeTime::<Identity, Impl, OFFSET>,
            GetCacheTime: GetCacheTime::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Cleanup: Cleanup::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetDiagHelperEx_Impl: ::windows_core::BaseImpl {
    fn ReconfirmLowHealth(this: &Self::This, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut ::windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::Result<()>;
    fn SetUtilities(this: &Self::This, putilities: ::core::option::Option<&INetDiagHelperUtilFactory>) -> ::windows_core::Result<()>;
    fn ReproduceFailure(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for INetDiagHelperEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetDiagHelperEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReconfirmLowHealth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut ::windows_core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReconfirmLowHealth(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&presults), ::core::mem::transmute_copy(&ppwszupdateddescription), ::core::mem::transmute_copy(&pupdatedstatus)).into())
        }
        unsafe extern "system" fn SetUtilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, putilities: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUtilities(this, ::windows_core::from_raw_borrowed(&putilities)).into())
        }
        unsafe extern "system" fn ReproduceFailure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReproduceFailure(this).into())
        }
        INetDiagHelperEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReconfirmLowHealth: ReconfirmLowHealth::<Identity, Impl, OFFSET>,
            SetUtilities: SetUtilities::<Identity, Impl, OFFSET>,
            ReproduceFailure: ReproduceFailure::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INetDiagHelperInfo_Impl: ::windows_core::BaseImpl {
    fn GetAttributeInfo(this: &Self::This, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetDiagHelperInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetDiagHelperInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeInfo(this, ::core::mem::transmute_copy(&pcelt), ::core::mem::transmute_copy(&pprgattributeinfos)).into())
        }
        INetDiagHelperInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAttributeInfo: GetAttributeInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INetDiagHelperUtilFactory_Impl: ::windows_core::BaseImpl {
    fn CreateUtilityInstance(this: &Self::This, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INetDiagHelperUtilFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperUtilFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetDiagHelperUtilFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateUtilityInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetDiagHelperUtilFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateUtilityInstance(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        INetDiagHelperUtilFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateUtilityInstance: CreateUtilityInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
