#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedEnvironmentInterop_Impl: ::windows_core::BaseImpl {
    fn GetHostHwndInterop(this: &Self::This, containerhwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IIsolatedEnvironmentInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsolatedEnvironmentInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIsolatedEnvironmentInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHostHwndInterop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIsolatedEnvironmentInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHostHwndInterop(this, ::core::mem::transmute_copy(&containerhwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hosthwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IIsolatedEnvironmentInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHostHwndInterop: GetHostHwndInterop::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
