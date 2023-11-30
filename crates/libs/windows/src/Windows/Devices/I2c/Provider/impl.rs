pub trait II2cControllerProvider_Impl: ::windows_core::BaseImpl {
    fn GetDeviceProvider(this: &Self::This, settings: ::core::option::Option<&ProviderI2cConnectionSettings>) -> ::windows_core::Result<II2cDeviceProvider>;
}
impl ::windows_core::Iids for II2cControllerProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cControllerProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for II2cControllerProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceProvider(this, ::windows_core::from_raw_borrowed(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        II2cControllerProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceProvider: GetDeviceProvider::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait II2cDeviceProvider_Impl: ::windows_core::BaseImpl + super::super::super::Foundation::IClosable_Impl {
    fn DeviceId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Write(this: &Self::This, buffer: &[u8]) -> ::windows_core::Result<()>;
    fn WritePartial(this: &Self::This, buffer: &[u8]) -> ::windows_core::Result<ProviderI2cTransferResult>;
    fn Read(this: &Self::This, buffer: &mut [u8]) -> ::windows_core::Result<()>;
    fn ReadPartial(this: &Self::This, buffer: &mut [u8]) -> ::windows_core::Result<ProviderI2cTransferResult>;
    fn WriteRead(this: &Self::This, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<()>;
    fn WriteReadPartial(this: &Self::This, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<ProviderI2cTransferResult>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for II2cDeviceProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for II2cDeviceProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into())
        }
        unsafe extern "system" fn WritePartial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WritePartial(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into())
        }
        unsafe extern "system" fn ReadPartial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadPartial(this, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteRead(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)).into())
        }
        unsafe extern "system" fn WriteReadPartial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cDeviceProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteReadPartial(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        II2cDeviceProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            WritePartial: WritePartial::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            ReadPartial: ReadPartial::<Identity, Impl, OFFSET>,
            WriteRead: WriteRead::<Identity, Impl, OFFSET>,
            WriteReadPartial: WriteReadPartial::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait II2cProvider_Impl: ::windows_core::BaseImpl {
    fn GetControllersAsync(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for II2cProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for II2cProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetControllersAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: II2cProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetControllersAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        II2cProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetControllersAsync: GetControllersAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
