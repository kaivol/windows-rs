#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait II2cDeviceStatics_Impl: ::windows_core::BaseImpl {
    fn GetDeviceSelector(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(this: &Self::This, friendlyname: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn FromIdAsync(this: &Self::This, deviceid: &::windows_core::HSTRING, settings: ::core::option::Option<&I2cConnectionSettings>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for II2cDeviceStatics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for II2cDeviceStatics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceSelector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceSelector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceSelectorFromFriendlyName(this, ::core::mem::transmute(&friendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FromIdAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceStatics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FromIdAsync(this, ::core::mem::transmute(&deviceid), ::windows_core::from_raw_borrowed(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        II2cDeviceStatics_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceSelector: GetDeviceSelector::<Identity, Impl, OFFSET>,
            GetDeviceSelectorFromFriendlyName: GetDeviceSelectorFromFriendlyName::<Identity, Impl, OFFSET>,
            FromIdAsync: FromIdAsync::<Identity, Impl, OFFSET>,
        }
    };
}
