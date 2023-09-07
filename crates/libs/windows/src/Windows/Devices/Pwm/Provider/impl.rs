pub trait IPwmControllerProvider_Impl: ::windows_core::BaseImpl {
    fn PinCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ActualFrequency(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetDesiredFrequency(this: &Self::This, frequency: f64) -> ::windows_core::Result<f64>;
    fn MaxFrequency(this: &Self::This) -> ::windows_core::Result<f64>;
    fn MinFrequency(this: &Self::This) -> ::windows_core::Result<f64>;
    fn AcquirePin(this: &Self::This, pin: i32) -> ::windows_core::Result<()>;
    fn ReleasePin(this: &Self::This, pin: i32) -> ::windows_core::Result<()>;
    fn EnablePin(this: &Self::This, pin: i32) -> ::windows_core::Result<()>;
    fn DisablePin(this: &Self::This, pin: i32) -> ::windows_core::Result<()>;
    fn SetPulseParameters(this: &Self::This, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPwmControllerProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPwmControllerProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PinCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActualFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActualFrequency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frequency: f64, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetDesiredFrequency(this, frequency) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxFrequency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinFrequency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AcquirePin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquirePin(this, pin).into())
        }
        unsafe extern "system" fn ReleasePin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleasePin(this, pin).into())
        }
        unsafe extern "system" fn EnablePin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnablePin(this, pin).into())
        }
        unsafe extern "system" fn DisablePin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisablePin(this, pin).into())
        }
        unsafe extern "system" fn SetPulseParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPulseParameters(this, pin, dutycycle, invertpolarity).into())
        }
        IPwmControllerProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PinCount: PinCount::<Identity, Impl, OFFSET>,
            ActualFrequency: ActualFrequency::<Identity, Impl, OFFSET>,
            SetDesiredFrequency: SetDesiredFrequency::<Identity, Impl, OFFSET>,
            MaxFrequency: MaxFrequency::<Identity, Impl, OFFSET>,
            MinFrequency: MinFrequency::<Identity, Impl, OFFSET>,
            AcquirePin: AcquirePin::<Identity, Impl, OFFSET>,
            ReleasePin: ReleasePin::<Identity, Impl, OFFSET>,
            EnablePin: EnablePin::<Identity, Impl, OFFSET>,
            DisablePin: DisablePin::<Identity, Impl, OFFSET>,
            SetPulseParameters: SetPulseParameters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPwmProvider_Impl: ::windows_core::BaseImpl {
    fn GetControllers(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IPwmProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPwmProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetControllers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPwmProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetControllers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPwmProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetControllers: GetControllers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
