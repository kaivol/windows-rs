pub trait IGpioControllerProvider_Impl: ::windows_core::BaseImpl {
    fn PinCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn OpenPinProvider(this: &Self::This, pin: i32, sharingmode: ProviderGpioSharingMode) -> ::windows_core::Result<IGpioPinProvider>;
}
impl ::windows_core::Iids for IGpioControllerProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGpioControllerProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PinCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenPinProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenPinProvider(this, pin, sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGpioControllerProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PinCount: PinCount::<Identity, Impl, OFFSET>,
            OpenPinProvider: OpenPinProvider::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IGpioPinProvider_Impl: ::windows_core::BaseImpl {
    fn ValueChanged(this: &Self::This, handler: ::core::option::Option<&super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(this: &Self::This, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn DebounceTimeout(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(this: &Self::This, value: &super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()>;
    fn PinNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SharingMode(this: &Self::This) -> ::windows_core::Result<ProviderGpioSharingMode>;
    fn IsDriveModeSupported(this: &Self::This, drivemode: ProviderGpioPinDriveMode) -> ::windows_core::Result<bool>;
    fn GetDriveMode(this: &Self::This) -> ::windows_core::Result<ProviderGpioPinDriveMode>;
    fn SetDriveMode(this: &Self::This, value: ProviderGpioPinDriveMode) -> ::windows_core::Result<()>;
    fn Write(this: &Self::This, value: ProviderGpioPinValue) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This) -> ::windows_core::Result<ProviderGpioPinValue>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IGpioPinProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGpioPinProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ValueChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValueChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveValueChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveValueChanged(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn DebounceTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DebounceTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDebounceTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebounceTimeout(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn PinNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SharingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioSharingMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SharingMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDriveModeSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDriveModeSupported(this, drivemode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDriveMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinDriveMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDriveMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDriveMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinDriveMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDriveMode(this, value).into())
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ProviderGpioPinValue) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, value).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinValue) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Read(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGpioPinProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ValueChanged: ValueChanged::<Identity, Impl, OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Identity, Impl, OFFSET>,
            DebounceTimeout: DebounceTimeout::<Identity, Impl, OFFSET>,
            SetDebounceTimeout: SetDebounceTimeout::<Identity, Impl, OFFSET>,
            PinNumber: PinNumber::<Identity, Impl, OFFSET>,
            SharingMode: SharingMode::<Identity, Impl, OFFSET>,
            IsDriveModeSupported: IsDriveModeSupported::<Identity, Impl, OFFSET>,
            GetDriveMode: GetDriveMode::<Identity, Impl, OFFSET>,
            SetDriveMode: SetDriveMode::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IGpioProvider_Impl: ::windows_core::BaseImpl {
    fn GetControllers(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IGpioProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGpioProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetControllers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGpioProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetControllers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGpioProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetControllers: GetControllers::<Identity, Impl, OFFSET>,
        }
    };
}
