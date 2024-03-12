#[doc = "Required features: `\"UI_ViewManagement\"`"]
#[cfg(feature = "UI_ViewManagement")]
pub trait ILauncherViewOptions_Impl: ::windows_core::BaseImpl {
    fn DesiredRemainingView(this: &Self::This) -> ::windows_core::Result<super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(this: &Self::This, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()>;
}
#[cfg(feature = "UI_ViewManagement")]
impl ::windows_core::Iids for ILauncherViewOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "UI_ViewManagement")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILauncherViewOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILauncherViewOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DesiredRemainingView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILauncherViewOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DesiredRemainingView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDesiredRemainingView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILauncherViewOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDesiredRemainingView(this, value).into())
        }
        ILauncherViewOptions_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DesiredRemainingView: DesiredRemainingView::<Identity, Impl, OFFSET>,
            SetDesiredRemainingView: SetDesiredRemainingView::<Identity, Impl, OFFSET>,
        }
    };
}
