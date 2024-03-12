pub trait IEnumWIA_DEV_CAPS_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumWIA_DEV_CAPS {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumWIA_DEV_CAPS {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_CAPS_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumWIA_DEV_CAPS_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumWIA_DEV_INFO_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IWiaPropertyStorage>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumWIA_DEV_INFO>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumWIA_DEV_INFO {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumWIA_DEV_INFO {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_DEV_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(celt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumWIA_DEV_INFO_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumWIA_FORMAT_INFO_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumWIA_FORMAT_INFO {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumWIA_FORMAT_INFO {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumWIA_FORMAT_INFO_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumWiaItem_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppiwiaitem: *mut ::core::option::Option<IWiaItem>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumWiaItem>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumWiaItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumWiaItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppiwiaitem), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(celt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumWiaItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumWiaItem2_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumWiaItem2>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumWiaItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumWiaItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem2: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppiwiaitem2), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(celt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumWiaItem2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaAppErrorHandler_Impl: ::windows_core::BaseImpl {
    fn GetWindow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn ReportStatus(this: &Self::This, lflags: i32, pwiaitem2: ::core::option::Option<&IWiaItem2>, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaAppErrorHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaAppErrorHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaAppErrorHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaAppErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReportStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaAppErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportStatus(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pwiaitem2), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&lpercentcomplete)).into())
        }
        IWiaAppErrorHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            ReportStatus: ReportStatus::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaDataCallback_Impl: ::windows_core::BaseImpl {
    fn BandedDataCallback(this: &Self::This, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWiaDataCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaDataCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BandedDataCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BandedDataCallback(this, ::core::mem::transmute_copy(&lmessage), ::core::mem::transmute_copy(&lstatus), ::core::mem::transmute_copy(&lpercentcomplete), ::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&llength), ::core::mem::transmute_copy(&lreserved), ::core::mem::transmute_copy(&lreslength), ::core::mem::transmute_copy(&pbbuffer)).into())
        }
        IWiaDataCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BandedDataCallback: BandedDataCallback::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaDataTransfer_Impl: ::windows_core::BaseImpl {
    fn idtGetData(this: &Self::This, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: ::core::option::Option<&IWiaDataCallback>) -> ::windows_core::Result<()>;
    fn idtGetBandedData(this: &Self::This, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: ::core::option::Option<&IWiaDataCallback>) -> ::windows_core::Result<()>;
    fn idtQueryGetData(this: &Self::This, pfe: *const WIA_FORMAT_INFO) -> ::windows_core::Result<()>;
    fn idtEnumWIA_FORMAT_INFO(this: &Self::This) -> ::windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn idtGetExtendedTransferInfo(this: &Self::This, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IWiaDataTransfer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaDataTransfer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn idtGetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::idtGetData(this, ::core::mem::transmute_copy(&pmedium), ::windows_core::from_raw_borrowed(&piwiadatacallback)).into())
        }
        unsafe extern "system" fn idtGetBandedData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::idtGetBandedData(this, ::core::mem::transmute_copy(&pwiadatatransinfo), ::windows_core::from_raw_borrowed(&piwiadatacallback)).into())
        }
        unsafe extern "system" fn idtQueryGetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::idtQueryGetData(this, ::core::mem::transmute_copy(&pfe)).into())
        }
        unsafe extern "system" fn idtEnumWIA_FORMAT_INFO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::idtEnumWIA_FORMAT_INFO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDataTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::idtGetExtendedTransferInfo(this, ::core::mem::transmute_copy(&pextendedtransferinfo)).into())
        }
        IWiaDataTransfer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            idtGetData: idtGetData::<Identity, Impl, OFFSET>,
            idtGetBandedData: idtGetBandedData::<Identity, Impl, OFFSET>,
            idtQueryGetData: idtQueryGetData::<Identity, Impl, OFFSET>,
            idtEnumWIA_FORMAT_INFO: idtEnumWIA_FORMAT_INFO::<Identity, Impl, OFFSET>,
            idtGetExtendedTransferInfo: idtGetExtendedTransferInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaDevMgr_Impl: ::windows_core::BaseImpl {
    fn EnumDeviceInfo(this: &Self::This, lflag: i32) -> ::windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(this: &Self::This, bstrdeviceid: &::windows_core::BSTR) -> ::windows_core::Result<IWiaItem>;
    fn SelectDeviceDlg(this: &Self::This, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::windows_core::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem>) -> ::windows_core::Result<()>;
    fn SelectDeviceDlgID(this: &Self::This, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetImageDlg(this: &Self::This, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: ::core::option::Option<&IWiaItem>, bstrfilename: &::windows_core::BSTR, pguidformat: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RegisterEventCallbackProgram(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR, peventguid: *const ::windows_core::GUID, bstrcommandline: &::windows_core::BSTR, bstrname: &::windows_core::BSTR, bstrdescription: &::windows_core::BSTR, bstricon: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RegisterEventCallbackInterface(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR, peventguid: *const ::windows_core::GUID, piwiaeventcallback: ::core::option::Option<&IWiaEventCallback>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn RegisterEventCallbackCLSID(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: &::windows_core::BSTR, bstrdescription: &::windows_core::BSTR, bstricon: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddDeviceDlg(this: &Self::This, hwndparent: super::super::Foundation::HWND, lflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaDevMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaDevMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumDeviceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflag: i32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDeviceInfo(this, ::core::mem::transmute_copy(&lflag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwiaitemroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice(this, ::core::mem::transmute(&bstrdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwiaitemroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitemroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectDeviceDlg(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid), ::core::mem::transmute_copy(&ppitemroot)).into())
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectDeviceDlgID(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid)).into())
        }
        unsafe extern "system" fn GetImageDlg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pguidformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImageDlg(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lintent), ::windows_core::from_raw_borrowed(&pitemroot), ::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&pguidformat)).into())
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, peventguid: *const ::windows_core::GUID, bstrcommandline: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstricon: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterEventCallbackProgram(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstrcommandline), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into())
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, peventguid: *const ::windows_core::GUID, piwiaeventcallback: *mut ::core::ffi::c_void, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterEventCallbackInterface(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::windows_core::from_raw_borrowed(&piwiaeventcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstricon: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterEventCallbackCLSID(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into())
        }
        unsafe extern "system" fn AddDeviceDlg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDeviceDlg(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lflags)).into())
        }
        IWiaDevMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumDeviceInfo: EnumDeviceInfo::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, Impl, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, Impl, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, Impl, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, Impl, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, Impl, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, Impl, OFFSET>,
            AddDeviceDlg: AddDeviceDlg::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaDevMgr2_Impl: ::windows_core::BaseImpl {
    fn EnumDeviceInfo(this: &Self::This, lflags: i32) -> ::windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR) -> ::windows_core::Result<IWiaItem2>;
    fn SelectDeviceDlg(this: &Self::This, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::windows_core::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()>;
    fn SelectDeviceDlgID(this: &Self::This, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RegisterEventCallbackInterface(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR, peventguid: *const ::windows_core::GUID, piwiaeventcallback: ::core::option::Option<&IWiaEventCallback>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn RegisterEventCallbackProgram(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR, peventguid: *const ::windows_core::GUID, bstrfullappname: &::windows_core::BSTR, bstrcommandlinearg: &::windows_core::BSTR, bstrname: &::windows_core::BSTR, bstrdescription: &::windows_core::BSTR, bstricon: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RegisterEventCallbackCLSID(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: &::windows_core::BSTR, bstrdescription: &::windows_core::BSTR, bstricon: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetImageDlg(this: &Self::This, lflags: i32, bstrdeviceid: &::windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstrfoldername: &::windows_core::BSTR, bstrfilename: &::windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::windows_core::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaDevMgr2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaDevMgr2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumDeviceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDeviceInfo(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwiaitem2root: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwiaitem2root, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitemroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectDeviceDlg(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid), ::core::mem::transmute_copy(&ppitemroot)).into())
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectDeviceDlgID(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&ldevicetype), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pbstrdeviceid)).into())
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, peventguid: *const ::windows_core::GUID, piwiaeventcallback: *mut ::core::ffi::c_void, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterEventCallbackInterface(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::windows_core::from_raw_borrowed(&piwiaeventcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, peventguid: *const ::windows_core::GUID, bstrfullappname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcommandlinearg: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstricon: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterEventCallbackProgram(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstrfullappname), ::core::mem::transmute(&bstrcommandlinearg), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into())
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, peventguid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstricon: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterEventCallbackCLSID(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute(&bstricon)).into())
        }
        unsafe extern "system" fn GetImageDlg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDevMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::windows_core::BSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImageDlg(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&bstrfoldername), ::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&plnumfiles), ::core::mem::transmute_copy(&ppbstrfilepaths), ::core::mem::transmute_copy(&ppitem)).into())
        }
        IWiaDevMgr2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumDeviceInfo: EnumDeviceInfo::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, Impl, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, Impl, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, Impl, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, Impl, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, Impl, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaDrvItem_Impl: ::windows_core::BaseImpl {
    fn GetItemFlags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetDeviceSpecContext(this: &Self::This) -> ::windows_core::Result<*mut u8>;
    fn GetFullItemName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetItemName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AddItemToFolder(this: &Self::This, __midl__iwiadrvitem0004: ::core::option::Option<&IWiaDrvItem>) -> ::windows_core::Result<()>;
    fn UnlinkItemTree(this: &Self::This, __midl__iwiadrvitem0005: i32) -> ::windows_core::Result<()>;
    fn RemoveItemFromFolder(this: &Self::This, __midl__iwiadrvitem0006: i32) -> ::windows_core::Result<()>;
    fn FindItemByName(this: &Self::This, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: &::windows_core::BSTR) -> ::windows_core::Result<IWiaDrvItem>;
    fn FindChildItemByName(this: &Self::This, __midl__iwiadrvitem0010: &::windows_core::BSTR) -> ::windows_core::Result<IWiaDrvItem>;
    fn GetParentItem(this: &Self::This) -> ::windows_core::Result<IWiaDrvItem>;
    fn GetFirstChildItem(this: &Self::This) -> ::windows_core::Result<IWiaDrvItem>;
    fn GetNextSiblingItem(this: &Self::This) -> ::windows_core::Result<IWiaDrvItem>;
    fn DumpItemData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IWiaDrvItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaDrvItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0000, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceSpecContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceSpecContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0001, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFullItemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0002: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFullItemName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0002, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0003: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0003, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddItemToFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0004: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItemToFolder(this, ::windows_core::from_raw_borrowed(&__midl__iwiadrvitem0004)).into())
        }
        unsafe extern "system" fn UnlinkItemTree<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlinkItemTree(this, ::core::mem::transmute_copy(&__midl__iwiadrvitem0005)).into())
        }
        unsafe extern "system" fn RemoveItemFromFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItemFromFolder(this, ::core::mem::transmute_copy(&__midl__iwiadrvitem0006)).into())
        }
        unsafe extern "system" fn FindItemByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: ::std::mem::MaybeUninit<::windows_core::BSTR>, __midl__iwiadrvitem0009: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItemByName(this, ::core::mem::transmute_copy(&__midl__iwiadrvitem0007), ::core::mem::transmute(&__midl__iwiadrvitem0008)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0009, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindChildItemByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0010: ::std::mem::MaybeUninit<::windows_core::BSTR>, __midl__iwiadrvitem0011: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindChildItemByName(this, ::core::mem::transmute(&__midl__iwiadrvitem0010)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0011, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0012: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0012, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFirstChildItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0013: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFirstChildItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0013, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextSiblingItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0014: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextSiblingItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0014, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DumpItemData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaDrvItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0015: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DumpItemData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiadrvitem0015, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWiaDrvItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemFlags: GetItemFlags::<Identity, Impl, OFFSET>,
            GetDeviceSpecContext: GetDeviceSpecContext::<Identity, Impl, OFFSET>,
            GetFullItemName: GetFullItemName::<Identity, Impl, OFFSET>,
            GetItemName: GetItemName::<Identity, Impl, OFFSET>,
            AddItemToFolder: AddItemToFolder::<Identity, Impl, OFFSET>,
            UnlinkItemTree: UnlinkItemTree::<Identity, Impl, OFFSET>,
            RemoveItemFromFolder: RemoveItemFromFolder::<Identity, Impl, OFFSET>,
            FindItemByName: FindItemByName::<Identity, Impl, OFFSET>,
            FindChildItemByName: FindChildItemByName::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
            GetFirstChildItem: GetFirstChildItem::<Identity, Impl, OFFSET>,
            GetNextSiblingItem: GetNextSiblingItem::<Identity, Impl, OFFSET>,
            DumpItemData: DumpItemData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaErrorHandler_Impl: ::windows_core::BaseImpl {
    fn ReportStatus(this: &Self::This, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: ::core::option::Option<&IWiaItem2>, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::Result<()>;
    fn GetStatusDescription(this: &Self::This, lflags: i32, pwiaitem2: ::core::option::Option<&IWiaItem2>, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaErrorHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaErrorHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaErrorHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, lpercentcomplete: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportStatus(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&hwndparent), ::windows_core::from_raw_borrowed(&pwiaitem2), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&lpercentcomplete)).into())
        }
        unsafe extern "system" fn GetStatusDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatusDescription(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pwiaitem2), ::core::mem::transmute_copy(&hrstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWiaErrorHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportStatus: ReportStatus::<Identity, Impl, OFFSET>,
            GetStatusDescription: GetStatusDescription::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaEventCallback_Impl: ::windows_core::BaseImpl {
    fn ImageEventCallback(this: &Self::This, peventguid: *const ::windows_core::GUID, bstreventdescription: &::windows_core::BSTR, bstrdeviceid: &::windows_core::BSTR, bstrdevicedescription: &::windows_core::BSTR, dwdevicetype: u32, bstrfullitemname: &::windows_core::BSTR, puleventtype: *mut u32, ulreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWiaEventCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaEventCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaEventCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ImageEventCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows_core::GUID, bstreventdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdevicedescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwdevicetype: u32, bstrfullitemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, puleventtype: *mut u32, ulreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImageEventCallback(this, ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstreventdescription), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute(&bstrdevicedescription), ::core::mem::transmute_copy(&dwdevicetype), ::core::mem::transmute(&bstrfullitemname), ::core::mem::transmute_copy(&puleventtype), ::core::mem::transmute_copy(&ulreserved)).into())
        }
        IWiaEventCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ImageEventCallback: ImageEventCallback::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWiaImageFilter_Impl: ::windows_core::BaseImpl {
    fn InitializeFilter(this: &Self::This, pwiaitem2: ::core::option::Option<&IWiaItem2>, pwiatransfercallback: ::core::option::Option<&IWiaTransferCallback>) -> ::windows_core::Result<()>;
    fn SetNewCallback(this: &Self::This, pwiatransfercallback: ::core::option::Option<&IWiaTransferCallback>) -> ::windows_core::Result<()>;
    fn FilterPreviewImage(this: &Self::This, lflags: i32, pwiachilditem2: ::core::option::Option<&IWiaItem2>, inputimageextents: &super::super::Foundation::RECT, pinputstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn ApplyProperties(this: &Self::This, pwiapropertystorage: ::core::option::Option<&IWiaPropertyStorage>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWiaImageFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaImageFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiaitem2: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFilter(this, ::windows_core::from_raw_borrowed(&pwiaitem2), ::windows_core::from_raw_borrowed(&pwiatransfercallback)).into())
        }
        unsafe extern "system" fn SetNewCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNewCallback(this, ::windows_core::from_raw_borrowed(&pwiatransfercallback)).into())
        }
        unsafe extern "system" fn FilterPreviewImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiachilditem2: *mut ::core::ffi::c_void, inputimageextents: super::super::Foundation::RECT, pinputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FilterPreviewImage(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pwiachilditem2), ::core::mem::transmute(&inputimageextents), ::windows_core::from_raw_borrowed(&pinputstream)).into())
        }
        unsafe extern "system" fn ApplyProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaImageFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwiapropertystorage: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyProperties(this, ::windows_core::from_raw_borrowed(&pwiapropertystorage)).into())
        }
        IWiaImageFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeFilter: InitializeFilter::<Identity, Impl, OFFSET>,
            SetNewCallback: SetNewCallback::<Identity, Impl, OFFSET>,
            FilterPreviewImage: FilterPreviewImage::<Identity, Impl, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaItem_Impl: ::windows_core::BaseImpl {
    fn GetItemType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AnalyzeItem(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn EnumChildItems(this: &Self::This) -> ::windows_core::Result<IEnumWiaItem>;
    fn DeleteItem(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn CreateChildItem(this: &Self::This, lflags: i32, bstritemname: &::windows_core::BSTR, bstrfullitemname: &::windows_core::BSTR) -> ::windows_core::Result<IWiaItem>;
    fn EnumRegisterEventInfo(this: &Self::This, lflags: i32, peventguid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn FindItemByName(this: &Self::This, lflags: i32, bstrfullitemname: &::windows_core::BSTR) -> ::windows_core::Result<IWiaItem>;
    fn DeviceDlg(this: &Self::This, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::core::option::Option<IWiaItem>) -> ::windows_core::Result<()>;
    fn DeviceCommand(this: &Self::This, lflags: i32, pcmdguid: *const ::windows_core::GUID, piwiaitem: *mut ::core::option::Option<IWiaItem>) -> ::windows_core::Result<()>;
    fn GetRootItem(this: &Self::This) -> ::windows_core::Result<IWiaItem>;
    fn EnumDeviceCapabilities(this: &Self::This, lflags: i32) -> ::windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn DumpItemData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DumpDrvItemData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DumpTreeItemData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Diagnostic(this: &Self::This, ulsize: u32, pbuffer: *const u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AnalyzeItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnalyzeItem(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn EnumChildItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienumwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumChildItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwiaitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn CreateChildItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfullitemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppiwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateChildItem(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows_core::GUID, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRegisterEventInfo(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&peventguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindItemByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppiwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItemByName(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceDlg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::core::option::Option<IWiaItem>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceDlg(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lintent), ::core::mem::transmute_copy(&plitemcount), ::core::mem::transmute_copy(&ppiwiaitem)).into())
        }
        unsafe extern "system" fn DeviceCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows_core::GUID, piwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceCommand(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pcmdguid), ::core::mem::transmute_copy(&piwiaitem)).into())
        }
        unsafe extern "system" fn GetRootItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiaitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDeviceCapabilities(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwia_dev_caps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DumpItemData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DumpItemData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DumpDrvItemData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DumpDrvItemData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DumpTreeItemData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DumpTreeItemData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Diagnostic(this, ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pbuffer)).into())
        }
        IWiaItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemType: GetItemType::<Identity, Impl, OFFSET>,
            AnalyzeItem: AnalyzeItem::<Identity, Impl, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            CreateChildItem: CreateChildItem::<Identity, Impl, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, Impl, OFFSET>,
            FindItemByName: FindItemByName::<Identity, Impl, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, Impl, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, Impl, OFFSET>,
            GetRootItem: GetRootItem::<Identity, Impl, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, Impl, OFFSET>,
            DumpItemData: DumpItemData::<Identity, Impl, OFFSET>,
            DumpDrvItemData: DumpDrvItemData::<Identity, Impl, OFFSET>,
            DumpTreeItemData: DumpTreeItemData::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaItem2_Impl: ::windows_core::BaseImpl {
    fn CreateChildItem(this: &Self::This, litemflags: i32, lcreationflags: i32, bstritemname: &::windows_core::BSTR) -> ::windows_core::Result<IWiaItem2>;
    fn DeleteItem(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn EnumChildItems(this: &Self::This, pcategoryguid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumWiaItem2>;
    fn FindItemByName(this: &Self::This, lflags: i32, bstrfullitemname: &::windows_core::BSTR) -> ::windows_core::Result<IWiaItem2>;
    fn GetItemCategory(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetItemType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DeviceDlg(this: &Self::This, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: &::windows_core::BSTR, bstrfilename: &::windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::windows_core::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()>;
    fn DeviceCommand(this: &Self::This, lflags: i32, pcmdguid: *const ::windows_core::GUID, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>) -> ::windows_core::Result<()>;
    fn EnumDeviceCapabilities(this: &Self::This, lflags: i32) -> ::windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn CheckExtension(this: &Self::This, lflags: i32, bstrname: &::windows_core::BSTR, riidextensioninterface: *const ::windows_core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetExtension(this: &Self::This, lflags: i32, bstrname: &::windows_core::BSTR, riidextensioninterface: *const ::windows_core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetParentItem(this: &Self::This) -> ::windows_core::Result<IWiaItem2>;
    fn GetRootItem(this: &Self::This) -> ::windows_core::Result<IWiaItem2>;
    fn GetPreviewComponent(this: &Self::This, lflags: i32) -> ::windows_core::Result<IWiaPreview>;
    fn EnumRegisterEventInfo(this: &Self::This, lflags: i32, peventguid: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn Diagnostic(this: &Self::This, ulsize: u32, pbuffer: *const u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateChildItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateChildItem(this, ::core::mem::transmute_copy(&litemflags), ::core::mem::transmute_copy(&lcreationflags), ::core::mem::transmute(&bstritemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn EnumChildItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcategoryguid: *const ::windows_core::GUID, ppienumwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumChildItems(this, ::core::mem::transmute_copy(&pcategoryguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwiaitem2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindItemByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItemByName(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemcategoryguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemcategoryguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceDlg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut ::windows_core::BSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceDlg(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&bstrfoldername), ::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&plnumfiles), ::core::mem::transmute_copy(&ppbstrfilepaths), ::core::mem::transmute_copy(&ppitem)).into())
        }
        unsafe extern "system" fn DeviceCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows_core::GUID, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceCommand(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pcmdguid), ::core::mem::transmute_copy(&ppiwiaitem2)).into())
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDeviceCapabilities(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumwia_dev_caps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, riidextensioninterface: *const ::windows_core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckExtension(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&riidextensioninterface), ::core::mem::transmute_copy(&pbextensionexists)).into())
        }
        unsafe extern "system" fn GetExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, riidextensioninterface: *const ::windows_core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtension(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&riidextensioninterface), ::core::mem::transmute_copy(&ppout)).into())
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRootItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiaitem2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreviewComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppwiapreview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviewComponent(this, ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwiapreview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows_core::GUID, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRegisterEventInfo(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&peventguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Diagnostic(this, ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pbuffer)).into())
        }
        IWiaItem2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateChildItem: CreateChildItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, Impl, OFFSET>,
            FindItemByName: FindItemByName::<Identity, Impl, OFFSET>,
            GetItemCategory: GetItemCategory::<Identity, Impl, OFFSET>,
            GetItemType: GetItemType::<Identity, Impl, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, Impl, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, Impl, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, Impl, OFFSET>,
            CheckExtension: CheckExtension::<Identity, Impl, OFFSET>,
            GetExtension: GetExtension::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
            GetRootItem: GetRootItem::<Identity, Impl, OFFSET>,
            GetPreviewComponent: GetPreviewComponent::<Identity, Impl, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaItemExtras_Impl: ::windows_core::BaseImpl {
    fn GetExtendedErrorInfo(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Escape(this: &Self::This, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows_core::Result<()>;
    fn CancelPendingIO(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWiaItemExtras {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaItemExtras {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExtendedErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrerrortext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtendedErrorInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrerrortext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&dwescapecode), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdatasize)).into())
        }
        unsafe extern "system" fn CancelPendingIO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaItemExtras_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelPendingIO(this).into())
        }
        IWiaItemExtras_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetExtendedErrorInfo: GetExtendedErrorInfo::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            CancelPendingIO: CancelPendingIO::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaLog_Impl: ::windows_core::BaseImpl {
    fn InitializeLog(this: &Self::This, hinstance: i32) -> ::windows_core::Result<()>;
    fn hResult(this: &Self::This, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn Log(this: &Self::This, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWiaLog {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaLog {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeLog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hinstance: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeLog(this, ::core::mem::transmute_copy(&hinstance)).into())
        }
        unsafe extern "system" fn hResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::hResult(this, ::core::mem::transmute_copy(&hresult)).into())
        }
        unsafe extern "system" fn Log<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLog_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Log(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lresid), ::core::mem::transmute_copy(&ldetail), ::core::mem::transmute(&bstrtext)).into())
        }
        IWiaLog_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeLog: InitializeLog::<Identity, Impl, OFFSET>,
            hResult: hResult::<Identity, Impl, OFFSET>,
            Log: Log::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaLogEx_Impl: ::windows_core::BaseImpl {
    fn InitializeLogEx(this: &Self::This, hinstance: *const u8) -> ::windows_core::Result<()>;
    fn hResult(this: &Self::This, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn Log(this: &Self::This, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn hResultEx(this: &Self::This, lmethodid: i32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn LogEx(this: &Self::This, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWiaLogEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaLogEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeLogEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hinstance: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeLogEx(this, ::core::mem::transmute_copy(&hinstance)).into())
        }
        unsafe extern "system" fn hResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::hResult(this, ::core::mem::transmute_copy(&hresult)).into())
        }
        unsafe extern "system" fn Log<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Log(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lresid), ::core::mem::transmute_copy(&ldetail), ::core::mem::transmute(&bstrtext)).into())
        }
        unsafe extern "system" fn hResultEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmethodid: i32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::hResultEx(this, ::core::mem::transmute_copy(&lmethodid), ::core::mem::transmute_copy(&hresult)).into())
        }
        unsafe extern "system" fn LogEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaLogEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LogEx(this, ::core::mem::transmute_copy(&lmethodid), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&lresid), ::core::mem::transmute_copy(&ldetail), ::core::mem::transmute(&bstrtext)).into())
        }
        IWiaLogEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeLogEx: InitializeLogEx::<Identity, Impl, OFFSET>,
            hResult: hResult::<Identity, Impl, OFFSET>,
            Log: Log::<Identity, Impl, OFFSET>,
            hResultEx: hResultEx::<Identity, Impl, OFFSET>,
            LogEx: LogEx::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaMiniDrv_Impl: ::windows_core::BaseImpl {
    fn drvInitializeWia(this: &Self::This, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: &::windows_core::BSTR, __midl__iwiaminidrv0003: &::windows_core::BSTR, __midl__iwiaminidrv0004: ::core::option::Option<&::windows_core::IUnknown>, __midl__iwiaminidrv0005: ::core::option::Option<&::windows_core::IUnknown>, __midl__iwiaminidrv0006: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0007: *mut ::core::option::Option<::windows_core::IUnknown>, __midl__iwiaminidrv0008: *mut i32) -> ::windows_core::Result<()>;
    fn drvAcquireItemData(this: &Self::This, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows_core::Result<()>;
    fn drvInitItemProperties(this: &Self::This, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32) -> ::windows_core::Result<i32>;
    fn drvValidateItemProperties(this: &Self::This, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows_core::Result<i32>;
    fn drvWriteItemProperties(this: &Self::This, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT) -> ::windows_core::Result<i32>;
    fn drvReadItemProperties(this: &Self::This, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows_core::Result<i32>;
    fn drvLockWiaDevice(this: &Self::This, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32) -> ::windows_core::Result<i32>;
    fn drvUnLockWiaDevice(this: &Self::This, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32) -> ::windows_core::Result<i32>;
    fn drvAnalyzeItem(this: &Self::This, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows_core::Result<()>;
    fn drvGetDeviceErrorStr(this: &Self::This, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows_core::Result<()>;
    fn drvDeviceCommand(this: &Self::This, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows_core::GUID, __midl__iwiaminidrv0046: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0047: *mut i32) -> ::windows_core::Result<()>;
    fn drvGetCapabilities(this: &Self::This, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows_core::Result<()>;
    fn drvDeleteItem(this: &Self::This, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32) -> ::windows_core::Result<i32>;
    fn drvFreeDrvItemContext(this: &Self::This, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8) -> ::windows_core::Result<i32>;
    fn drvGetWiaFormatInfo(this: &Self::This, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows_core::Result<()>;
    fn drvNotifyPnpEvent(this: &Self::This, peventguid: *const ::windows_core::GUID, bstrdeviceid: &::windows_core::BSTR, ulreserved: u32) -> ::windows_core::Result<()>;
    fn drvUnInitializeWia(this: &Self::This, __midl__iwiaminidrv0064: *const u8) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IWiaMiniDrv {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaMiniDrv {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn drvInitializeWia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: ::std::mem::MaybeUninit<::windows_core::BSTR>, __midl__iwiaminidrv0003: ::std::mem::MaybeUninit<::windows_core::BSTR>, __midl__iwiaminidrv0004: *mut ::core::ffi::c_void, __midl__iwiaminidrv0005: *mut ::core::ffi::c_void, __midl__iwiaminidrv0006: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0007: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::drvInitializeWia(
                    this,
                    ::core::mem::transmute_copy(&__midl__iwiaminidrv0000),
                    ::core::mem::transmute_copy(&__midl__iwiaminidrv0001),
                    ::core::mem::transmute(&__midl__iwiaminidrv0002),
                    ::core::mem::transmute(&__midl__iwiaminidrv0003),
                    ::windows_core::from_raw_borrowed(&__midl__iwiaminidrv0004),
                    ::windows_core::from_raw_borrowed(&__midl__iwiaminidrv0005),
                    ::core::mem::transmute_copy(&__midl__iwiaminidrv0006),
                    ::core::mem::transmute_copy(&__midl__iwiaminidrv0007),
                    ::core::mem::transmute_copy(&__midl__iwiaminidrv0008),
                )
                .into()
            })
        }
        unsafe extern "system" fn drvAcquireItemData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvAcquireItemData(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0009), ::core::mem::transmute_copy(&__midl__iwiaminidrv0010), ::core::mem::transmute_copy(&__midl__iwiaminidrv0011), ::core::mem::transmute_copy(&__midl__iwiaminidrv0012)).into())
        }
        unsafe extern "system" fn drvInitItemProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvInitItemProperties(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0013), ::core::mem::transmute_copy(&__midl__iwiaminidrv0014)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0015, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvValidateItemProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvValidateItemProperties(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0016), ::core::mem::transmute_copy(&__midl__iwiaminidrv0017), ::core::mem::transmute_copy(&__midl__iwiaminidrv0018), ::core::mem::transmute_copy(&__midl__iwiaminidrv0019)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0020, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvWriteItemProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvWriteItemProperties(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0021), ::core::mem::transmute_copy(&__midl__iwiaminidrv0022), ::core::mem::transmute_copy(&__midl__iwiaminidrv0023)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0024, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvReadItemProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvReadItemProperties(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0025), ::core::mem::transmute_copy(&__midl__iwiaminidrv0026), ::core::mem::transmute_copy(&__midl__iwiaminidrv0027), ::core::mem::transmute_copy(&__midl__iwiaminidrv0028)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0029, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvLockWiaDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvLockWiaDevice(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0030), ::core::mem::transmute_copy(&__midl__iwiaminidrv0031)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0032, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvUnLockWiaDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvUnLockWiaDevice(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0033), ::core::mem::transmute_copy(&__midl__iwiaminidrv0034)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0035, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvAnalyzeItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvAnalyzeItem(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0036), ::core::mem::transmute_copy(&__midl__iwiaminidrv0037), ::core::mem::transmute_copy(&__midl__iwiaminidrv0038)).into())
        }
        unsafe extern "system" fn drvGetDeviceErrorStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvGetDeviceErrorStr(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0039), ::core::mem::transmute_copy(&__midl__iwiaminidrv0040), ::core::mem::transmute_copy(&__midl__iwiaminidrv0041), ::core::mem::transmute_copy(&__midl__iwiaminidrv0042)).into())
        }
        unsafe extern "system" fn drvDeviceCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows_core::GUID, __midl__iwiaminidrv0046: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0047: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvDeviceCommand(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0043), ::core::mem::transmute_copy(&__midl__iwiaminidrv0044), ::core::mem::transmute_copy(&__midl__iwiaminidrv0045), ::core::mem::transmute_copy(&__midl__iwiaminidrv0046), ::core::mem::transmute_copy(&__midl__iwiaminidrv0047)).into())
        }
        unsafe extern "system" fn drvGetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvGetCapabilities(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0048), ::core::mem::transmute_copy(&__midl__iwiaminidrv0049), ::core::mem::transmute_copy(&__midl__iwiaminidrv0050), ::core::mem::transmute_copy(&__midl__iwiaminidrv0051), ::core::mem::transmute_copy(&__midl__iwiaminidrv0052)).into())
        }
        unsafe extern "system" fn drvDeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvDeleteItem(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0053), ::core::mem::transmute_copy(&__midl__iwiaminidrv0054)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0055, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvFreeDrvItemContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::drvFreeDrvItemContext(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0056), ::core::mem::transmute_copy(&__midl__iwiaminidrv0057)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__iwiaminidrv0058, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn drvGetWiaFormatInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvGetWiaFormatInfo(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0059), ::core::mem::transmute_copy(&__midl__iwiaminidrv0060), ::core::mem::transmute_copy(&__midl__iwiaminidrv0061), ::core::mem::transmute_copy(&__midl__iwiaminidrv0062), ::core::mem::transmute_copy(&__midl__iwiaminidrv0063)).into())
        }
        unsafe extern "system" fn drvNotifyPnpEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventguid: *const ::windows_core::GUID, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ulreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvNotifyPnpEvent(this, ::core::mem::transmute_copy(&peventguid), ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&ulreserved)).into())
        }
        unsafe extern "system" fn drvUnInitializeWia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::drvUnInitializeWia(this, ::core::mem::transmute_copy(&__midl__iwiaminidrv0064)).into())
        }
        IWiaMiniDrv_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            drvInitializeWia: drvInitializeWia::<Identity, Impl, OFFSET>,
            drvAcquireItemData: drvAcquireItemData::<Identity, Impl, OFFSET>,
            drvInitItemProperties: drvInitItemProperties::<Identity, Impl, OFFSET>,
            drvValidateItemProperties: drvValidateItemProperties::<Identity, Impl, OFFSET>,
            drvWriteItemProperties: drvWriteItemProperties::<Identity, Impl, OFFSET>,
            drvReadItemProperties: drvReadItemProperties::<Identity, Impl, OFFSET>,
            drvLockWiaDevice: drvLockWiaDevice::<Identity, Impl, OFFSET>,
            drvUnLockWiaDevice: drvUnLockWiaDevice::<Identity, Impl, OFFSET>,
            drvAnalyzeItem: drvAnalyzeItem::<Identity, Impl, OFFSET>,
            drvGetDeviceErrorStr: drvGetDeviceErrorStr::<Identity, Impl, OFFSET>,
            drvDeviceCommand: drvDeviceCommand::<Identity, Impl, OFFSET>,
            drvGetCapabilities: drvGetCapabilities::<Identity, Impl, OFFSET>,
            drvDeleteItem: drvDeleteItem::<Identity, Impl, OFFSET>,
            drvFreeDrvItemContext: drvFreeDrvItemContext::<Identity, Impl, OFFSET>,
            drvGetWiaFormatInfo: drvGetWiaFormatInfo::<Identity, Impl, OFFSET>,
            drvNotifyPnpEvent: drvNotifyPnpEvent::<Identity, Impl, OFFSET>,
            drvUnInitializeWia: drvUnInitializeWia::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaMiniDrvCallBack_Impl: ::windows_core::BaseImpl {
    fn MiniDrvCallback(this: &Self::This, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaMiniDrvCallBack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrvCallBack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaMiniDrvCallBack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MiniDrvCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrvCallBack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MiniDrvCallback(this, ::core::mem::transmute_copy(&lreason), ::core::mem::transmute_copy(&lstatus), ::core::mem::transmute_copy(&lpercentcomplete), ::core::mem::transmute_copy(&loffset), ::core::mem::transmute_copy(&llength), ::core::mem::transmute_copy(&ptranctx), ::core::mem::transmute_copy(&lreserved)).into())
        }
        IWiaMiniDrvCallBack_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MiniDrvCallback: MiniDrvCallback::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaMiniDrvTransferCallback_Impl: ::windows_core::BaseImpl {
    fn GetNextStream(this: &Self::This, lflags: i32, bstritemname: &::windows_core::BSTR, bstrfullitemname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SendMessage(this: &Self::This, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWiaMiniDrvTransferCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrvTransferCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaMiniDrvTransferCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNextStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrvTransferCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfullitemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppistream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextStream(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppistream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaMiniDrvTransferCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMessage(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pwiatransferparams)).into())
        }
        IWiaMiniDrvTransferCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNextStream: GetNextStream::<Identity, Impl, OFFSET>,
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaNotifyDevMgr_Impl: ::windows_core::BaseImpl {
    fn NewDeviceArrival(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWiaNotifyDevMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaNotifyDevMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaNotifyDevMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NewDeviceArrival<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaNotifyDevMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewDeviceArrival(this).into())
        }
        IWiaNotifyDevMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NewDeviceArrival: NewDeviceArrival::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWiaPreview_Impl: ::windows_core::BaseImpl {
    fn GetNewPreview(this: &Self::This, lflags: i32, pwiaitem2: ::core::option::Option<&IWiaItem2>, pwiatransfercallback: ::core::option::Option<&IWiaTransferCallback>) -> ::windows_core::Result<()>;
    fn UpdatePreview(this: &Self::This, lflags: i32, pchildwiaitem2: ::core::option::Option<&IWiaItem2>, pwiatransfercallback: ::core::option::Option<&IWiaTransferCallback>) -> ::windows_core::Result<()>;
    fn DetectRegions(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWiaPreview {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaPreview {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNewPreview<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNewPreview(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pwiaitem2), ::windows_core::from_raw_borrowed(&pwiatransfercallback)).into())
        }
        unsafe extern "system" fn UpdatePreview<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pchildwiaitem2: *mut ::core::ffi::c_void, pwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdatePreview(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pchildwiaitem2), ::windows_core::from_raw_borrowed(&pwiatransfercallback)).into())
        }
        unsafe extern "system" fn DetectRegions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectRegions(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPreview_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IWiaPreview_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNewPreview: GetNewPreview::<Identity, Impl, OFFSET>,
            UpdatePreview: UpdatePreview::<Identity, Impl, OFFSET>,
            DetectRegions: DetectRegions::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWiaPropertyStorage_Impl: ::windows_core::BaseImpl {
    fn ReadMultiple(this: &Self::This, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn WriteMultiple(this: &Self::This, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::Result<()>;
    fn DeleteMultiple(this: &Self::This, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows_core::Result<()>;
    fn ReadPropertyNames(this: &Self::This, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn WritePropertyNames(this: &Self::This, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeletePropertyNames(this: &Self::This, cpropid: u32, rgpropid: *const u32) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, grfcommitflags: u32) -> ::windows_core::Result<()>;
    fn Revert(this: &Self::This) -> ::windows_core::Result<()>;
    fn Enum(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG>;
    fn SetTimes(this: &Self::This, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn SetClass(this: &Self::This, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Stat(this: &Self::This, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> ::windows_core::Result<()>;
    fn GetPropertyAttributes(this: &Self::This, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPropertyStream(this: &Self::This, pcompatibilityid: *mut ::windows_core::GUID, ppistream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetPropertyStream(this: &Self::This, pcompatibilityid: *mut ::windows_core::GUID, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWiaPropertyStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaPropertyStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadMultiple(this, ::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar)).into())
        }
        unsafe extern "system" fn WriteMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteMultiple(this, ::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar), ::core::mem::transmute_copy(&propidnamefirst)).into())
        }
        unsafe extern "system" fn DeleteMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMultiple(this, ::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec)).into())
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadPropertyNames(this, ::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into())
        }
        unsafe extern "system" fn WritePropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WritePropertyNames(this, ::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into())
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyNames(this, ::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&grfcommitflags)).into())
        }
        unsafe extern "system" fn Revert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Revert(this).into())
        }
        unsafe extern "system" fn Enum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimes(this, ::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into())
        }
        unsafe extern "system" fn SetClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClass(this, ::core::mem::transmute_copy(&clsid)).into())
        }
        unsafe extern "system" fn Stat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stat(this, ::core::mem::transmute_copy(&pstatpsstg)).into())
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyAttributes(this, ::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgflags), ::core::mem::transmute_copy(&rgpropvar)).into())
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulnumprops: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulnumprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows_core::GUID, ppistream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyStream(this, ::core::mem::transmute_copy(&pcompatibilityid), ::core::mem::transmute_copy(&ppistream)).into())
        }
        unsafe extern "system" fn SetPropertyStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows_core::GUID, pistream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyStream(this, ::core::mem::transmute_copy(&pcompatibilityid), ::windows_core::from_raw_borrowed(&pistream)).into())
        }
        IWiaPropertyStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadMultiple: ReadMultiple::<Identity, Impl, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, Impl, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, Impl, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, Impl, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, Impl, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            SetTimes: SetTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetPropertyStream: GetPropertyStream::<Identity, Impl, OFFSET>,
            SetPropertyStream: SetPropertyStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaSegmentationFilter_Impl: ::windows_core::BaseImpl {
    fn DetectRegions(this: &Self::This, lflags: i32, pinputstream: ::core::option::Option<&super::super::System::Com::IStream>, pwiaitem2: ::core::option::Option<&IWiaItem2>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWiaSegmentationFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaSegmentationFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaSegmentationFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DetectRegions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaSegmentationFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pinputstream: *mut ::core::ffi::c_void, pwiaitem2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectRegions(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&pinputstream), ::windows_core::from_raw_borrowed(&pwiaitem2)).into())
        }
        IWiaSegmentationFilter_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DetectRegions: DetectRegions::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransfer_Impl: ::windows_core::BaseImpl {
    fn Download(this: &Self::This, lflags: i32, piwiatransfercallback: ::core::option::Option<&IWiaTransferCallback>) -> ::windows_core::Result<()>;
    fn Upload(this: &Self::This, lflags: i32, psource: ::core::option::Option<&super::super::System::Com::IStream>, piwiatransfercallback: ::core::option::Option<&IWiaTransferCallback>) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumWIA_FORMAT_INFO(this: &Self::This) -> ::windows_core::Result<IEnumWIA_FORMAT_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWiaTransfer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaTransfer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, piwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&piwiatransfercallback)).into())
        }
        unsafe extern "system" fn Upload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, psource: *mut ::core::ffi::c_void, piwiatransfercallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Upload(this, ::core::mem::transmute_copy(&lflags), ::windows_core::from_raw_borrowed(&psource), ::windows_core::from_raw_borrowed(&piwiatransfercallback)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn EnumWIA_FORMAT_INFO<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransfer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumWIA_FORMAT_INFO(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWiaTransfer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Download: Download::<Identity, Impl, OFFSET>,
            Upload: Upload::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EnumWIA_FORMAT_INFO: EnumWIA_FORMAT_INFO::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransferCallback_Impl: ::windows_core::BaseImpl {
    fn TransferCallback(this: &Self::This, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::Result<()>;
    fn GetNextStream(this: &Self::This, lflags: i32, bstritemname: &::windows_core::BSTR, bstrfullitemname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWiaTransferCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransferCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaTransferCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TransferCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransferCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransferCallback(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pwiatransferparams)).into())
        }
        unsafe extern "system" fn GetNextStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaTransferCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfullitemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdestination: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextStream(this, ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrfullitemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWiaTransferCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TransferCallback: TransferCallback::<Identity, Impl, OFFSET>,
            GetNextStream: GetNextStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtension_Impl: ::windows_core::BaseImpl {
    fn DeviceDialog(this: &Self::This, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows_core::Result<()>;
    fn GetDeviceIcon(this: &Self::This, bstrdeviceid: &::windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::Result<()>;
    fn GetDeviceBitmapLogo(this: &Self::This, bstrdeviceid: &::windows_core::BSTR, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IWiaUIExtension {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaUIExtension {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceDialog(this, ::core::mem::transmute_copy(&pdevicedialogdata)).into())
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceIcon(this, ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&phicon), ::core::mem::transmute_copy(&nsize)).into())
        }
        unsafe extern "system" fn GetDeviceBitmapLogo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaUIExtension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceBitmapLogo(this, ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&phbitmap), ::core::mem::transmute_copy(&nmaxwidth), ::core::mem::transmute_copy(&nmaxheight)).into())
        }
        IWiaUIExtension_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceDialog: DeviceDialog::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
            GetDeviceBitmapLogo: GetDeviceBitmapLogo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtension2_Impl: ::windows_core::BaseImpl {
    fn DeviceDialog(this: &Self::This, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows_core::Result<()>;
    fn GetDeviceIcon(this: &Self::This, bstrdeviceid: &::windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IWiaUIExtension2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaUIExtension2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaUIExtension2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaUIExtension2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceDialog(this, ::core::mem::transmute_copy(&pdevicedialogdata)).into())
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaUIExtension2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceIcon(this, ::core::mem::transmute(&bstrdeviceid), ::core::mem::transmute_copy(&phicon), ::core::mem::transmute_copy(&nsize)).into())
        }
        IWiaUIExtension2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceDialog: DeviceDialog::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWiaVideo_Impl: ::windows_core::BaseImpl {
    fn PreviewVisible(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetPreviewVisible(this: &Self::This, bpreviewvisible: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ImagesDirectory(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetImagesDirectory(this: &Self::This, bstrimagedirectory: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CreateVideoByWiaDevID(this: &Self::This, bstrwiadeviceid: &::windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CreateVideoByDevNum(this: &Self::This, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CreateVideoByName(this: &Self::This, bstrfriendlyname: &::windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DestroyVideo(this: &Self::This) -> ::windows_core::Result<()>;
    fn Play(this: &Self::This) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn TakePicture(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ResizeVideo(this: &Self::This, bstretchtofitparent: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetCurrentState(this: &Self::This) -> ::windows_core::Result<WIAVIDEO_STATE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWiaVideo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWiaVideo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PreviewVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbpreviewvisible: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreviewVisible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpreviewvisible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPreviewVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpreviewvisible: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreviewVisible(this, ::core::mem::transmute_copy(&bpreviewvisible)).into())
        }
        unsafe extern "system" fn ImagesDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrimagedirectory: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImagesDirectory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrimagedirectory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetImagesDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrimagedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImagesDirectory(this, ::core::mem::transmute(&bstrimagedirectory)).into())
        }
        unsafe extern "system" fn CreateVideoByWiaDevID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrwiadeviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVideoByWiaDevID(this, ::core::mem::transmute(&bstrwiadeviceid), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bstretchtofitparent), ::core::mem::transmute_copy(&bautobeginplayback)).into())
        }
        unsafe extern "system" fn CreateVideoByDevNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVideoByDevNum(this, ::core::mem::transmute_copy(&uidevicenumber), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bstretchtofitparent), ::core::mem::transmute_copy(&bautobeginplayback)).into())
        }
        unsafe extern "system" fn CreateVideoByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateVideoByName(this, ::core::mem::transmute(&bstrfriendlyname), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bstretchtofitparent), ::core::mem::transmute_copy(&bautobeginplayback)).into())
        }
        unsafe extern "system" fn DestroyVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyVideo(this).into())
        }
        unsafe extern "system" fn Play<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Play(this).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn TakePicture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnewimagefilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TakePicture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnewimagefilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResizeVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstretchtofitparent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResizeVideo(this, ::core::mem::transmute_copy(&bstretchtofitparent)).into())
        }
        unsafe extern "system" fn GetCurrentState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWiaVideo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWiaVideo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PreviewVisible: PreviewVisible::<Identity, Impl, OFFSET>,
            SetPreviewVisible: SetPreviewVisible::<Identity, Impl, OFFSET>,
            ImagesDirectory: ImagesDirectory::<Identity, Impl, OFFSET>,
            SetImagesDirectory: SetImagesDirectory::<Identity, Impl, OFFSET>,
            CreateVideoByWiaDevID: CreateVideoByWiaDevID::<Identity, Impl, OFFSET>,
            CreateVideoByDevNum: CreateVideoByDevNum::<Identity, Impl, OFFSET>,
            CreateVideoByName: CreateVideoByName::<Identity, Impl, OFFSET>,
            DestroyVideo: DestroyVideo::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            TakePicture: TakePicture::<Identity, Impl, OFFSET>,
            ResizeVideo: ResizeVideo::<Identity, Impl, OFFSET>,
            GetCurrentState: GetCurrentState::<Identity, Impl, OFFSET>,
        }
    };
}
