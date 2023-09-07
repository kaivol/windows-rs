#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintManagerInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(this: &Self::This, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPrintManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&printmanager)).into())
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowPrintUIForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        IPrintManagerInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Printing\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowConfigurationNative_Impl: ::windows_core::BaseImpl {
    fn PrinterQueue(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterQueue>;
    fn DriverProperties(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
    fn UserProperties(this: &Self::This) -> ::windows_core::Result<super::super::super::Graphics::Printing::IPrinterPropertyBag>;
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPrintWorkflowConfigurationNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Printing", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintWorkflowConfigurationNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrinterQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrinterQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DriverProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DriverProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowConfigurationNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintWorkflowConfigurationNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrinterQueue: PrinterQueue::<Identity, Impl, OFFSET>,
            DriverProperties: DriverProperties::<Identity, Impl, OFFSET>,
            UserProperties: UserProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowObjectModelSourceFileContentNative_Impl: ::windows_core::BaseImpl {
    fn StartXpsOMGeneration(this: &Self::This, receiver: ::core::option::Option<&IPrintWorkflowXpsReceiver>) -> ::windows_core::Result<()>;
    fn ObjectFactory(this: &Self::This) -> ::windows_core::Result<super::super::super::Storage::Xps::IXpsOMObjectFactory1>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::Iids for IPrintWorkflowObjectModelSourceFileContentNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_Xps")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintWorkflowObjectModelSourceFileContentNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartXpsOMGeneration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartXpsOMGeneration(this, ::windows_core::from_raw_borrowed(&receiver)).into())
        }
        unsafe extern "system" fn ObjectFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowObjectModelSourceFileContentNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectFactory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintWorkflowObjectModelSourceFileContentNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartXpsOMGeneration: StartXpsOMGeneration::<Identity, Impl, OFFSET>,
            ObjectFactory: ObjectFactory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
pub trait IPrintWorkflowXpsObjectModelTargetPackageNative_Impl: ::windows_core::BaseImpl {
    fn DocumentPackageTarget(this: &Self::This) -> ::windows_core::Result<super::super::super::Storage::Xps::IXpsDocumentPackageTarget>;
}
#[cfg(feature = "Win32_Storage_Xps")]
impl ::windows_core::Iids for IPrintWorkflowXpsObjectModelTargetPackageNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_Xps")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintWorkflowXpsObjectModelTargetPackageNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DocumentPackageTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsObjectModelTargetPackageNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentPackageTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintWorkflowXpsObjectModelTargetPackageNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DocumentPackageTarget: DocumentPackageTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver_Impl: ::windows_core::BaseImpl {
    fn SetDocumentSequencePrintTicket(this: &Self::This, documentsequenceprintticket: ::core::option::Option<&super::super::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetDocumentSequenceUri(this: &Self::This, documentsequenceuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddDocumentData(this: &Self::This, documentid: u32, documentprintticket: ::core::option::Option<&super::super::Com::IStream>, documenturi: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddPage(this: &Self::This, documentid: u32, pageid: u32, pagereference: ::core::option::Option<&super::super::super::Storage::Xps::IXpsOMPageReference>, pageuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPrintWorkflowXpsReceiver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintWorkflowXpsReceiver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDocumentSequencePrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentSequencePrintTicket(this, ::windows_core::from_raw_borrowed(&documentsequenceprintticket)).into())
        }
        unsafe extern "system" fn SetDocumentSequenceUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequenceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentSequenceUri(this, ::core::mem::transmute(&documentsequenceuri)).into())
        }
        unsafe extern "system" fn AddDocumentData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentid: u32, documentprintticket: *mut ::core::ffi::c_void, documenturi: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDocumentData(this, ::core::mem::transmute_copy(&documentid), ::windows_core::from_raw_borrowed(&documentprintticket), ::core::mem::transmute(&documenturi)).into())
        }
        unsafe extern "system" fn AddPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentid: u32, pageid: u32, pagereference: *mut ::core::ffi::c_void, pageuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPage(this, ::core::mem::transmute_copy(&documentid), ::core::mem::transmute_copy(&pageid), ::windows_core::from_raw_borrowed(&pagereference), ::core::mem::transmute(&pageuri)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IPrintWorkflowXpsReceiver_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDocumentSequencePrintTicket: SetDocumentSequencePrintTicket::<Identity, Impl, OFFSET>,
            SetDocumentSequenceUri: SetDocumentSequenceUri::<Identity, Impl, OFFSET>,
            AddDocumentData: AddDocumentData::<Identity, Impl, OFFSET>,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
pub trait IPrintWorkflowXpsReceiver2_Impl: ::windows_core::BaseImpl + IPrintWorkflowXpsReceiver_Impl {
    fn Failed(this: &Self::This, xpserror: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPrintWorkflowXpsReceiver2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPrintWorkflowXpsReceiver);
}
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintWorkflowXpsReceiver2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Failed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintWorkflowXpsReceiver2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpserror: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Failed(this, ::core::mem::transmute_copy(&xpserror)).into())
        }
        IPrintWorkflowXpsReceiver2_Vtbl { base__: <IPrintWorkflowXpsReceiver as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Failed: Failed::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrinting3DManagerInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(this: &Self::This, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPrinting3DManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrinting3DManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, printmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&printmanager)).into())
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrinting3DManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowPrintUIForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        IPrinting3DManagerInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
