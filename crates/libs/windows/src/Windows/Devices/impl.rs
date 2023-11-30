#[doc = "Required features: `\"Devices_Adc_Provider\"`, `\"Devices_Gpio_Provider\"`, `\"Devices_I2c_Provider\"`, `\"Devices_Pwm_Provider\"`, `\"Devices_Spi_Provider\"`"]
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
pub trait ILowLevelDevicesAggregateProvider_Impl: ::windows_core::BaseImpl {
    fn AdcControllerProvider(this: &Self::This) -> ::windows_core::Result<Adc::Provider::IAdcControllerProvider>;
    fn PwmControllerProvider(this: &Self::This) -> ::windows_core::Result<Pwm::Provider::IPwmControllerProvider>;
    fn GpioControllerProvider(this: &Self::This) -> ::windows_core::Result<Gpio::Provider::IGpioControllerProvider>;
    fn I2cControllerProvider(this: &Self::This) -> ::windows_core::Result<I2c::Provider::II2cControllerProvider>;
    fn SpiControllerProvider(this: &Self::This) -> ::windows_core::Result<Spi::Provider::ISpiControllerProvider>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ::windows_core::Iids for ILowLevelDevicesAggregateProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILowLevelDevicesAggregateProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdcControllerProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdcControllerProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PwmControllerProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PwmControllerProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GpioControllerProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GpioControllerProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn I2cControllerProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::I2cControllerProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SpiControllerProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpiControllerProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILowLevelDevicesAggregateProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdcControllerProvider: AdcControllerProvider::<Identity, Impl, OFFSET>,
            PwmControllerProvider: PwmControllerProvider::<Identity, Impl, OFFSET>,
            GpioControllerProvider: GpioControllerProvider::<Identity, Impl, OFFSET>,
            I2cControllerProvider: I2cControllerProvider::<Identity, Impl, OFFSET>,
            SpiControllerProvider: SpiControllerProvider::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
