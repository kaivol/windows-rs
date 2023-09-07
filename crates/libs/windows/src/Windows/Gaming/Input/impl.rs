#[doc = "Required features: `\"Foundation\"`, `\"System\"`"]
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IGameController_Impl: ::windows_core::BaseImpl {
    fn HeadsetConnected(this: &Self::This, value: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetConnected(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn HeadsetDisconnected(this: &Self::This, value: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetDisconnected(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn UserChanged(this: &Self::This, value: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserChanged(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn Headset(this: &Self::This) -> ::windows_core::Result<Headset>;
    fn IsWireless(this: &Self::This) -> ::windows_core::Result<bool>;
    fn User(this: &Self::This) -> ::windows_core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows_core::Iids for IGameController {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameController {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HeadsetConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HeadsetConnected(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveHeadsetConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveHeadsetConnected(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn HeadsetDisconnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HeadsetDisconnected(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveHeadsetDisconnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveHeadsetDisconnected(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn UserChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserChanged(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveUserChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveUserChanged(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn Headset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Headset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsWireless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsWireless(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGameController_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HeadsetConnected: HeadsetConnected::<Identity, Impl, OFFSET>,
            RemoveHeadsetConnected: RemoveHeadsetConnected::<Identity, Impl, OFFSET>,
            HeadsetDisconnected: HeadsetDisconnected::<Identity, Impl, OFFSET>,
            RemoveHeadsetDisconnected: RemoveHeadsetDisconnected::<Identity, Impl, OFFSET>,
            UserChanged: UserChanged::<Identity, Impl, OFFSET>,
            RemoveUserChanged: RemoveUserChanged::<Identity, Impl, OFFSET>,
            Headset: Headset::<Identity, Impl, OFFSET>,
            IsWireless: IsWireless::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Devices_Power\"`"]
#[cfg(feature = "Devices_Power")]
pub trait IGameControllerBatteryInfo_Impl: ::windows_core::BaseImpl {
    fn TryGetBatteryReport(this: &Self::This) -> ::windows_core::Result<super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "Devices_Power")]
impl ::windows_core::Iids for IGameControllerBatteryInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Devices_Power")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerBatteryInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameControllerBatteryInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TryGetBatteryReport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerBatteryInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TryGetBatteryReport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGameControllerBatteryInfo_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TryGetBatteryReport: TryGetBatteryReport::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
