#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerConfigurationInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRadialControllerConfigurationInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadialControllerConfigurationInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRadialControllerConfigurationInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadialControllerConfigurationInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IRadialControllerConfigurationInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerIndependentInputSourceInterop_Impl: ::windows_core::BaseImpl {
    fn CreateForWindow(this: &Self::This, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRadialControllerIndependentInputSourceInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRadialControllerIndependentInputSourceInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateForWindow(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IRadialControllerIndependentInputSourceInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerInterop_Impl: ::windows_core::BaseImpl {
    fn CreateForWindow(this: &Self::This, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRadialControllerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadialControllerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRadialControllerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadialControllerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateForWindow(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IRadialControllerInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
