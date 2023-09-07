#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICoreFrameworkInputViewInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICoreFrameworkInputViewInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFrameworkInputViewInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreFrameworkInputViewInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreFrameworkInputViewInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&coreframeworkinputview)).into())
        }
        ICoreFrameworkInputViewInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
