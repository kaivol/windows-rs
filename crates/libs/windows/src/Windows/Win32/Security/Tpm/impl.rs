#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager_Impl: ::windows_core::BaseImpl {
    fn CreateVirtualSmartCard(this: &Self::This, pszfriendlyname: &::windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut ::windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DestroyVirtualSmartCard(this: &Self::This, pszinstanceid: &::windows_core::PCWSTR, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITpmVirtualSmartCardManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITpmVirtualSmartCardManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVirtualSmartCard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::CreateVirtualSmartCard(
                    this,
                    ::core::mem::transmute(&pszfriendlyname),
                    ::core::mem::transmute_copy(&badminalgid),
                    ::core::mem::transmute_copy(&pbadminkey),
                    ::core::mem::transmute_copy(&cbadminkey),
                    ::core::mem::transmute_copy(&pbadminkcv),
                    ::core::mem::transmute_copy(&cbadminkcv),
                    ::core::mem::transmute_copy(&pbpuk),
                    ::core::mem::transmute_copy(&cbpuk),
                    ::core::mem::transmute_copy(&pbpin),
                    ::core::mem::transmute_copy(&cbpin),
                    ::core::mem::transmute_copy(&fgenerate),
                    ::windows_core::from_raw_borrowed(&pstatuscallback),
                    ::core::mem::transmute_copy(&ppszinstanceid),
                    ::core::mem::transmute_copy(&pfneedreboot),
                )
                .into()
            })
        }
        unsafe extern "system" fn DestroyVirtualSmartCard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinstanceid: ::windows_core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestroyVirtualSmartCard(this, ::core::mem::transmute(&pszinstanceid), ::windows_core::from_raw_borrowed(&pstatuscallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfneedreboot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITpmVirtualSmartCardManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVirtualSmartCard: CreateVirtualSmartCard::<Identity, Impl, OFFSET>,
            DestroyVirtualSmartCard: DestroyVirtualSmartCard::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager2_Impl: ::windows_core::BaseImpl + ITpmVirtualSmartCardManager_Impl {
    fn CreateVirtualSmartCardWithPinPolicy(this: &Self::This, pszfriendlyname: &::windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>, ppszinstanceid: *mut ::windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITpmVirtualSmartCardManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITpmVirtualSmartCardManager);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITpmVirtualSmartCardManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVirtualSmartCardWithPinPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows_core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::CreateVirtualSmartCardWithPinPolicy(
                    this,
                    ::core::mem::transmute(&pszfriendlyname),
                    ::core::mem::transmute_copy(&badminalgid),
                    ::core::mem::transmute_copy(&pbadminkey),
                    ::core::mem::transmute_copy(&cbadminkey),
                    ::core::mem::transmute_copy(&pbadminkcv),
                    ::core::mem::transmute_copy(&cbadminkcv),
                    ::core::mem::transmute_copy(&pbpuk),
                    ::core::mem::transmute_copy(&cbpuk),
                    ::core::mem::transmute_copy(&pbpin),
                    ::core::mem::transmute_copy(&cbpin),
                    ::core::mem::transmute_copy(&pbpinpolicy),
                    ::core::mem::transmute_copy(&cbpinpolicy),
                    ::core::mem::transmute_copy(&fgenerate),
                    ::windows_core::from_raw_borrowed(&pstatuscallback),
                    ::core::mem::transmute_copy(&ppszinstanceid),
                    ::core::mem::transmute_copy(&pfneedreboot),
                )
                .into()
            })
        }
        ITpmVirtualSmartCardManager2_Vtbl {
            base__: <ITpmVirtualSmartCardManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVirtualSmartCardWithPinPolicy: CreateVirtualSmartCardWithPinPolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITpmVirtualSmartCardManager3_Impl: ::windows_core::BaseImpl + ITpmVirtualSmartCardManager2_Impl {
    fn CreateVirtualSmartCardWithAttestation(this: &Self::This, pszfriendlyname: &::windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::core::option::Option<&ITpmVirtualSmartCardManagerStatusCallback>) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITpmVirtualSmartCardManager3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITpmVirtualSmartCardManager2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManager3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITpmVirtualSmartCardManager3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateVirtualSmartCardWithAttestation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManager3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows_core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::CreateVirtualSmartCardWithAttestation(
                    this,
                    ::core::mem::transmute(&pszfriendlyname),
                    ::core::mem::transmute_copy(&badminalgid),
                    ::core::mem::transmute_copy(&pbadminkey),
                    ::core::mem::transmute_copy(&cbadminkey),
                    ::core::mem::transmute_copy(&pbadminkcv),
                    ::core::mem::transmute_copy(&cbadminkcv),
                    ::core::mem::transmute_copy(&pbpuk),
                    ::core::mem::transmute_copy(&cbpuk),
                    ::core::mem::transmute_copy(&pbpin),
                    ::core::mem::transmute_copy(&cbpin),
                    ::core::mem::transmute_copy(&pbpinpolicy),
                    ::core::mem::transmute_copy(&cbpinpolicy),
                    ::core::mem::transmute_copy(&attestationtype),
                    ::core::mem::transmute_copy(&fgenerate),
                    ::windows_core::from_raw_borrowed(&pstatuscallback),
                ) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(ppszinstanceid, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        ITpmVirtualSmartCardManager3_Vtbl {
            base__: <ITpmVirtualSmartCardManager2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateVirtualSmartCardWithAttestation: CreateVirtualSmartCardWithAttestation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITpmVirtualSmartCardManagerStatusCallback_Impl: ::windows_core::BaseImpl {
    fn ReportProgress(this: &Self::This, status: TPMVSCMGR_STATUS) -> ::windows_core::Result<()>;
    fn ReportError(this: &Self::This, error: TPMVSCMGR_ERROR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITpmVirtualSmartCardManagerStatusCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITpmVirtualSmartCardManagerStatusCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportProgress(this, ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn ReportError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITpmVirtualSmartCardManagerStatusCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportError(this, ::core::mem::transmute_copy(&error)).into())
        }
        ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportProgress: ReportProgress::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
