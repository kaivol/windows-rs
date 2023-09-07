#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClock_Impl: ::windows_core::BaseImpl {
    fn GetTime(this: &Self::This) -> ::windows_core::Result<i64>;
    fn AdviseTime(this: &Self::This, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE) -> ::windows_core::Result<usize>;
    fn AdvisePeriodic(this: &Self::This, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE) -> ::windows_core::Result<usize>;
    fn Unadvise(this: &Self::This, dwadvisecookie: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IReferenceClock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReferenceClock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AdviseTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdviseTime(this, ::core::mem::transmute_copy(&basetime), ::core::mem::transmute_copy(&streamtime), ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwadvisecookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AdvisePeriodic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdvisePeriodic(this, ::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&periodtime), ::core::mem::transmute_copy(&hsemaphore)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwadvisecookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwadvisecookie: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwadvisecookie)).into())
        }
        IReferenceClock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            AdviseTime: AdviseTime::<Identity, Impl, OFFSET>,
            AdvisePeriodic: AdvisePeriodic::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClock2_Impl: ::windows_core::BaseImpl + IReferenceClock_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IReferenceClock2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IReferenceClock);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClock2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReferenceClock2 {
    const VTABLE: Self::Vtable = { IReferenceClock2_Vtbl { base__: <IReferenceClock as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IReferenceClockTimerControl_Impl: ::windows_core::BaseImpl {
    fn SetDefaultTimerResolution(this: &Self::This, timerresolution: i64) -> ::windows_core::Result<()>;
    fn GetDefaultTimerResolution(this: &Self::This) -> ::windows_core::Result<i64>;
}
impl ::windows_core::Iids for IReferenceClockTimerControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReferenceClockTimerControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDefaultTimerResolution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timerresolution: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultTimerResolution(this, ::core::mem::transmute_copy(&timerresolution)).into())
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimerresolution: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultTimerResolution(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptimerresolution, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IReferenceClockTimerControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDefaultTimerResolution: SetDefaultTimerResolution::<Identity, Impl, OFFSET>,
            GetDefaultTimerResolution: GetDefaultTimerResolution::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
