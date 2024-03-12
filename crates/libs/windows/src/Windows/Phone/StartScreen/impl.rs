#[doc = "Required features: `\"UI_Notifications\"`"]
#[cfg(feature = "UI_Notifications")]
pub trait IToastNotificationManagerStatics3_Impl: ::windows_core::BaseImpl {
    fn CreateToastNotifierForSecondaryTile(this: &Self::This, tileid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::UI::Notifications::ToastNotifier>;
}
#[cfg(feature = "UI_Notifications")]
impl ::windows_core::Iids for IToastNotificationManagerStatics3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "UI_Notifications")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IToastNotificationManagerStatics3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateToastNotifierForSecondaryTile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateToastNotifierForSecondaryTile(this, ::core::mem::transmute(&tileid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IToastNotificationManagerStatics3_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateToastNotifierForSecondaryTile: CreateToastNotifierForSecondaryTile::<Identity, Impl, OFFSET>,
        }
    };
}
