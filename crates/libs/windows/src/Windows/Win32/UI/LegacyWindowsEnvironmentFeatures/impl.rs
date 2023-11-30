#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2_Impl: ::windows_core::BaseImpl {
    fn ReReadWallpaper(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetADObjectFlags(this: &Self::This, pdwflags: *mut u32, dwmask: u32) -> ::windows_core::Result<()>;
    fn UpdateAllDesktopSubscriptions(this: &Self::This) -> ::windows_core::Result<()>;
    fn MakeDynamicChanges(this: &Self::This, poleobj: ::core::option::Option<&super::super::System::Ole::IOleObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::Iids for IADesktopP2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Ole")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IADesktopP2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReReadWallpaper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReReadWallpaper(this).into())
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetADObjectFlags(this, ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&dwmask)).into())
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateAllDesktopSubscriptions(this).into())
        }
        unsafe extern "system" fn MakeDynamicChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poleobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeDynamicChanges(this, ::windows_core::from_raw_borrowed(&poleobj)).into())
        }
        IADesktopP2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReReadWallpaper: ReReadWallpaper::<Identity, Impl, OFFSET>,
            GetADObjectFlags: GetADObjectFlags::<Identity, Impl, OFFSET>,
            UpdateAllDesktopSubscriptions: UpdateAllDesktopSubscriptions::<Identity, Impl, OFFSET>,
            MakeDynamicChanges: MakeDynamicChanges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IActiveDesktopP_Impl: ::windows_core::BaseImpl {
    fn SetSafeMode(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn EnsureUpdateHTML(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetScheme(this: &Self::This, pwszschemename: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetScheme(this: &Self::This, pwszschemename: ::windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveDesktopP {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveDesktopP {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSafeMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSafeMode(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn EnsureUpdateHTML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnsureUpdateHTML(this).into())
        }
        unsafe extern "system" fn SetScheme<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScheme(this, ::core::mem::transmute(&pwszschemename), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetScheme<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScheme(this, ::core::mem::transmute_copy(&pwszschemename), ::core::mem::transmute_copy(&pdwcchbuffer), ::core::mem::transmute_copy(&dwflags)).into())
        }
        IActiveDesktopP_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSafeMode: SetSafeMode::<Identity, Impl, OFFSET>,
            EnsureUpdateHTML: EnsureUpdateHTML::<Identity, Impl, OFFSET>,
            SetScheme: SetScheme::<Identity, Impl, OFFSET>,
            GetScheme: GetScheme::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiator_Impl: ::windows_core::BaseImpl {
    fn IsMonikerInBriefcase(this: &Self::This, pmk: ::core::option::Option<&super::super::System::Com::IMoniker>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IBriefcaseInitiator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBriefcaseInitiator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBriefcaseInitiator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsMonikerInBriefcase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBriefcaseInitiator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsMonikerInBriefcase(this, ::windows_core::from_raw_borrowed(&pmk)).into())
        }
        IBriefcaseInitiator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsMonikerInBriefcase: IsMonikerInBriefcase::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::Result<()>;
    fn GetSpaceUsed(this: &Self::This, pdwlspaceused: *mut u64, picb: ::core::option::Option<&IEmptyVolumeCacheCallBack>) -> ::windows_core::Result<()>;
    fn Purge(this: &Self::This, dwlspacetofree: u64, picb: ::core::option::Option<&IEmptyVolumeCacheCallBack>) -> ::windows_core::Result<()>;
    fn ShowProperties(this: &Self::This, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This) -> ::windows_core::Result<EMPTY_VOLUME_CACHE_FLAGS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for IEmptyVolumeCache {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEmptyVolumeCache {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdwflags)).into())
        }
        unsafe extern "system" fn GetSpaceUsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpaceUsed(this, ::core::mem::transmute_copy(&pdwlspaceused), ::windows_core::from_raw_borrowed(&picb)).into())
        }
        unsafe extern "system" fn Purge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Purge(this, ::core::mem::transmute_copy(&dwlspacetofree), ::windows_core::from_raw_borrowed(&picb)).into())
        }
        unsafe extern "system" fn ShowProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowProperties(this, ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Deactivate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEmptyVolumeCache_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetSpaceUsed: GetSpaceUsed::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            ShowProperties: ShowProperties::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache2_Impl: ::windows_core::BaseImpl + IEmptyVolumeCache_Impl {
    fn InitializeEx(this: &Self::This, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows_core::PCWSTR, pcwszkeyname: &::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, ppwszbtntext: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::Iids for IEmptyVolumeCache2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IEmptyVolumeCache);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEmptyVolumeCache2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCache2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_core::PCWSTR, pcwszkeyname: ::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, ppwszbtntext: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeEx(this, ::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute(&pcwszkeyname), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&ppwszbtntext), ::core::mem::transmute_copy(&pdwflags)).into())
        }
        IEmptyVolumeCache2_Vtbl { base__: <IEmptyVolumeCache as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, InitializeEx: InitializeEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEmptyVolumeCacheCallBack_Impl: ::windows_core::BaseImpl {
    fn ScanProgress(this: &Self::This, dwlspaceused: u64, dwflags: u32, pcwszstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn PurgeProgress(this: &Self::This, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEmptyVolumeCacheCallBack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEmptyVolumeCacheCallBack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ScanProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScanProgress(this, ::core::mem::transmute_copy(&dwlspaceused), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into())
        }
        unsafe extern "system" fn PurgeProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PurgeProgress(this, ::core::mem::transmute_copy(&dwlspacefreed), ::core::mem::transmute_copy(&dwlspacetofree), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into())
        }
        IEmptyVolumeCacheCallBack_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ScanProgress: ScanProgress::<Identity, Impl, OFFSET>,
            PurgeProgress: PurgeProgress::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IReconcilableObject_Impl: ::windows_core::BaseImpl {
    fn Reconcile(this: &Self::This, pinitiator: ::core::option::Option<&IReconcileInitiator>, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::core::option::Option<super::super::System::Com::IMoniker>, ploutindex: *mut i32, pstgnewresidues: ::core::option::Option<&super::super::System::Com::StructuredStorage::IStorage>, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetProgressFeedbackMaxEstimate(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IReconcilableObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReconcilableObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reconcile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitiator: *mut ::core::ffi::c_void, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut *mut ::core::ffi::c_void, ploutindex: *mut i32, pstgnewresidues: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reconcile(this, ::windows_core::from_raw_borrowed(&pinitiator), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&hwndprogressfeedback), ::core::mem::transmute_copy(&ulcinput), ::core::mem::transmute_copy(&rgpmkotherinput), ::core::mem::transmute_copy(&ploutindex), ::windows_core::from_raw_borrowed(&pstgnewresidues), ::core::mem::transmute_copy(&pvreserved)).into())
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProgressFeedbackMaxEstimate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprogressmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IReconcilableObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reconcile: Reconcile::<Identity, Impl, OFFSET>,
            GetProgressFeedbackMaxEstimate: GetProgressFeedbackMaxEstimate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IReconcileInitiator_Impl: ::windows_core::BaseImpl {
    fn SetAbortCallback(this: &Self::This, punkforabort: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetProgressFeedback(this: &Self::This, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IReconcileInitiator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReconcileInitiator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAbortCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAbortCallback(this, ::windows_core::from_raw_borrowed(&punkforabort)).into())
        }
        unsafe extern "system" fn SetProgressFeedback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProgressFeedback(this, ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into())
        }
        IReconcileInitiator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAbortCallback: SetAbortCallback::<Identity, Impl, OFFSET>,
            SetProgressFeedback: SetProgressFeedback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
