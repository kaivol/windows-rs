#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrintDocumentPackageStatusEvent_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn PackageStatusUpdated(this: &Self::This, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPrintDocumentPackageStatusEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintDocumentPackageStatusEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PackageStatusUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagestatus: *const PrintDocumentPackageStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PackageStatusUpdated(this, ::core::mem::transmute_copy(&packagestatus)).into())
        }
        IPrintDocumentPackageStatusEvent_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PackageStatusUpdated: PackageStatusUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPrintDocumentPackageTarget_Impl: ::windows_core::BaseImpl {
    fn GetPackageTargetTypes(this: &Self::This, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetPackageTarget(this: &Self::This, guidtargettype: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintDocumentPackageTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintDocumentPackageTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPackageTargetTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetcount: *mut u32, targettypes: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPackageTargetTypes(this, ::core::mem::transmute_copy(&targetcount), ::core::mem::transmute_copy(&targettypes)).into())
        }
        unsafe extern "system" fn GetPackageTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidtargettype: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPackageTarget(this, ::core::mem::transmute_copy(&guidtargettype), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtarget)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IPrintDocumentPackageTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPackageTargetTypes: GetPackageTargetTypes::<Identity, Impl, OFFSET>,
            GetPackageTarget: GetPackageTarget::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPrintDocumentPackageTarget2_Impl: ::windows_core::BaseImpl {
    fn GetIsTargetIppPrinter(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetTargetIppPrintDevice(this: &Self::This, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPrintDocumentPackageTarget2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTarget2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintDocumentPackageTarget2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIsTargetIppPrinter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTarget2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isippprinter: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsTargetIppPrinter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isippprinter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetIppPrintDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTarget2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvtarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTargetIppPrintDevice(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvtarget)).into())
        }
        IPrintDocumentPackageTarget2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIsTargetIppPrinter: GetIsTargetIppPrinter::<Identity, Impl, OFFSET>,
            GetTargetIppPrintDevice: GetTargetIppPrintDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPrintDocumentPackageTargetFactory_Impl: ::windows_core::BaseImpl {
    fn CreateDocumentPackageTargetForPrintJob(this: &Self::This, printername: &::windows_core::PCWSTR, jobname: &::windows_core::PCWSTR, joboutputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, jobprintticketstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IPrintDocumentPackageTarget>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IPrintDocumentPackageTargetFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintDocumentPackageTargetFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDocumentPackageTargetForPrintJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentPackageTargetFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printername: ::windows_core::PCWSTR, jobname: ::windows_core::PCWSTR, joboutputstream: *mut ::core::ffi::c_void, jobprintticketstream: *mut ::core::ffi::c_void, docpackagetarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDocumentPackageTargetForPrintJob(this, ::core::mem::transmute(&printername), ::core::mem::transmute(&jobname), ::windows_core::from_raw_borrowed(&joboutputstream), ::windows_core::from_raw_borrowed(&jobprintticketstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(docpackagetarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintDocumentPackageTargetFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDocumentPackageTargetForPrintJob: CreateDocumentPackageTargetForPrintJob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsPrintJob_Impl: ::windows_core::BaseImpl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetJobStatus(this: &Self::This, jobstatus: *mut XPS_JOB_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsPrintJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsPrintJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn GetJobStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPrintJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobstatus: *mut XPS_JOB_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetJobStatus(this, ::core::mem::transmute_copy(&jobstatus)).into())
        }
        IXpsPrintJob_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetJobStatus: GetJobStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXpsPrintJobStream_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::ISequentialStream_Impl {
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IXpsPrintJobStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::ISequentialStream);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPrintJobStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsPrintJobStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsPrintJobStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IXpsPrintJobStream_Vtbl {
            base__: <super::super::super::System::Com::ISequentialStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
