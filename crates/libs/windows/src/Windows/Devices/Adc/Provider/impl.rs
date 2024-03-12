pub trait IAdcControllerProvider_Impl: ::windows_core::BaseImpl {
    fn ChannelCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ResolutionInBits(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MinValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MaxValue(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ChannelMode(this: &Self::This) -> ::windows_core::Result<ProviderAdcChannelMode>;
    fn SetChannelMode(this: &Self::This, value: ProviderAdcChannelMode) -> ::windows_core::Result<()>;
    fn IsChannelModeSupported(this: &Self::This, channelmode: ProviderAdcChannelMode) -> ::windows_core::Result<bool>;
    fn AcquireChannel(this: &Self::This, channel: i32) -> ::windows_core::Result<()>;
    fn ReleaseChannel(this: &Self::This, channel: i32) -> ::windows_core::Result<()>;
    fn ReadValue(this: &Self::This, channelnumber: i32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IAdcControllerProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdcControllerProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResolutionInBits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResolutionInBits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChannelMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ProviderAdcChannelMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChannelMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChannelMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ProviderAdcChannelMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelMode(this, value).into())
        }
        unsafe extern "system" fn IsChannelModeSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channelmode: ProviderAdcChannelMode, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsChannelModeSupported(this, channelmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AcquireChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AcquireChannel(this, channel).into())
        }
        unsafe extern "system" fn ReleaseChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseChannel(this, channel).into())
        }
        unsafe extern "system" fn ReadValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadValue(this, channelnumber) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAdcControllerProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ChannelCount: ChannelCount::<Identity, Impl, OFFSET>,
            ResolutionInBits: ResolutionInBits::<Identity, Impl, OFFSET>,
            MinValue: MinValue::<Identity, Impl, OFFSET>,
            MaxValue: MaxValue::<Identity, Impl, OFFSET>,
            ChannelMode: ChannelMode::<Identity, Impl, OFFSET>,
            SetChannelMode: SetChannelMode::<Identity, Impl, OFFSET>,
            IsChannelModeSupported: IsChannelModeSupported::<Identity, Impl, OFFSET>,
            AcquireChannel: AcquireChannel::<Identity, Impl, OFFSET>,
            ReleaseChannel: ReleaseChannel::<Identity, Impl, OFFSET>,
            ReadValue: ReadValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IAdcProvider_Impl: ::windows_core::BaseImpl {
    fn GetControllers(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IAdcProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdcProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetControllers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdcProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetControllers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAdcProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetControllers: GetControllers::<Identity, Impl, OFFSET>,
        }
    };
}
