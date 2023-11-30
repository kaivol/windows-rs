#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IAccessoryNotificationTriggerDetails_Impl: ::windows_core::BaseImpl {
    fn TimeCreated(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AppId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AccessoryNotificationType(this: &Self::This) -> ::windows_core::Result<AccessoryNotificationType>;
    fn StartedProcessing(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetStartedProcessing(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IAccessoryNotificationTriggerDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessoryNotificationTriggerDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TimeCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TimeCreated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AccessoryNotificationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccessoryNotificationType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartedProcessing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartedProcessing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartedProcessing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartedProcessing(this, value).into())
        }
        IAccessoryNotificationTriggerDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TimeCreated: TimeCreated::<Identity, Impl, OFFSET>,
            AppDisplayName: AppDisplayName::<Identity, Impl, OFFSET>,
            AppId: AppId::<Identity, Impl, OFFSET>,
            AccessoryNotificationType: AccessoryNotificationType::<Identity, Impl, OFFSET>,
            StartedProcessing: StartedProcessing::<Identity, Impl, OFFSET>,
            SetStartedProcessing: SetStartedProcessing::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
