#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintDialogCallback_Impl: ::windows_core::BaseImpl {
    fn InitDone(this: &Self::This) -> ::windows_core::Result<()>;
    fn SelectionChange(this: &Self::This) -> ::windows_core::Result<()>;
    fn HandleMessage(this: &Self::This, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPrintDialogCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintDialogCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitDone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitDone(this).into())
        }
        unsafe extern "system" fn SelectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectionChange(this).into())
        }
        unsafe extern "system" fn HandleMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleMessage(this, ::core::mem::transmute_copy(&hdlg), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&presult)).into())
        }
        IPrintDialogCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitDone: InitDone::<Identity, Impl, OFFSET>,
            SelectionChange: SelectionChange::<Identity, Impl, OFFSET>,
            HandleMessage: HandleMessage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IPrintDialogServices_Impl: ::windows_core::BaseImpl {
    fn GetCurrentDevMode(this: &Self::This, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetCurrentPrinterName(this: &Self::This, pprintername: ::windows_core::PWSTR, pcchsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetCurrentPortName(this: &Self::This, pportname: ::windows_core::PWSTR, pcchsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IPrintDialogServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintDialogServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentDevMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentDevMode(this, ::core::mem::transmute_copy(&pdevmode), ::core::mem::transmute_copy(&pcbsize)).into())
        }
        unsafe extern "system" fn GetCurrentPrinterName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprintername: ::windows_core::PWSTR, pcchsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentPrinterName(this, ::core::mem::transmute_copy(&pprintername), ::core::mem::transmute_copy(&pcchsize)).into())
        }
        unsafe extern "system" fn GetCurrentPortName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDialogServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportname: ::windows_core::PWSTR, pcchsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentPortName(this, ::core::mem::transmute_copy(&pportname), ::core::mem::transmute_copy(&pcchsize)).into())
        }
        IPrintDialogServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentDevMode: GetCurrentDevMode::<Identity, Impl, OFFSET>,
            GetCurrentPrinterName: GetCurrentPrinterName::<Identity, Impl, OFFSET>,
            GetCurrentPortName: GetCurrentPortName::<Identity, Impl, OFFSET>,
        }
    };
}
