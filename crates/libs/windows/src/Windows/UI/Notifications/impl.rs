#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IAdaptiveNotificationContent_Impl: ::windows_core::BaseImpl {
    fn Kind(this: &Self::This) -> ::windows_core::Result<AdaptiveNotificationContentKind>;
    fn Hints(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IAdaptiveNotificationContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdaptiveNotificationContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Kind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveNotificationContentKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Hints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAdaptiveNotificationContent_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Kind: Kind::<Identity, Impl, OFFSET>,
            Hints: Hints::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
