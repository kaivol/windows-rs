pub trait IPrintDocumentSource_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IPrintDocumentSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintDocumentSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintDocumentSource {
    const VTABLE: Self::Vtable = { IPrintDocumentSource_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPrintTaskOptionsCore_Impl: ::windows_core::BaseImpl {
    fn GetPageDescription(this: &Self::This, jobpagenumber: u32) -> ::windows_core::Result<PrintPageDescription>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPrintTaskOptionsCore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTaskOptionsCore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPageDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPageDescription(this, jobpagenumber) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintTaskOptionsCore_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPageDescription: GetPageDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPrintTaskOptionsCoreProperties_Impl: ::windows_core::BaseImpl {
    fn SetMediaSize(this: &Self::This, value: PrintMediaSize) -> ::windows_core::Result<()>;
    fn MediaSize(this: &Self::This) -> ::windows_core::Result<PrintMediaSize>;
    fn SetMediaType(this: &Self::This, value: PrintMediaType) -> ::windows_core::Result<()>;
    fn MediaType(this: &Self::This) -> ::windows_core::Result<PrintMediaType>;
    fn SetOrientation(this: &Self::This, value: PrintOrientation) -> ::windows_core::Result<()>;
    fn Orientation(this: &Self::This) -> ::windows_core::Result<PrintOrientation>;
    fn SetPrintQuality(this: &Self::This, value: PrintQuality) -> ::windows_core::Result<()>;
    fn PrintQuality(this: &Self::This) -> ::windows_core::Result<PrintQuality>;
    fn SetColorMode(this: &Self::This, value: PrintColorMode) -> ::windows_core::Result<()>;
    fn ColorMode(this: &Self::This) -> ::windows_core::Result<PrintColorMode>;
    fn SetDuplex(this: &Self::This, value: PrintDuplex) -> ::windows_core::Result<()>;
    fn Duplex(this: &Self::This) -> ::windows_core::Result<PrintDuplex>;
    fn SetCollation(this: &Self::This, value: PrintCollation) -> ::windows_core::Result<()>;
    fn Collation(this: &Self::This) -> ::windows_core::Result<PrintCollation>;
    fn SetStaple(this: &Self::This, value: PrintStaple) -> ::windows_core::Result<()>;
    fn Staple(this: &Self::This) -> ::windows_core::Result<PrintStaple>;
    fn SetHolePunch(this: &Self::This, value: PrintHolePunch) -> ::windows_core::Result<()>;
    fn HolePunch(this: &Self::This) -> ::windows_core::Result<PrintHolePunch>;
    fn SetBinding(this: &Self::This, value: PrintBinding) -> ::windows_core::Result<()>;
    fn Binding(this: &Self::This) -> ::windows_core::Result<PrintBinding>;
    fn MinCopies(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MaxCopies(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetNumberOfCopies(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn NumberOfCopies(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IPrintTaskOptionsCoreProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTaskOptionsCoreProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMediaSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintMediaSize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMediaSize(this, value).into())
        }
        unsafe extern "system" fn MediaSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaSize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintMediaType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMediaType(this, value).into())
        }
        unsafe extern "system" fn MediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintMediaType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintOrientation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOrientation(this, value).into())
        }
        unsafe extern "system" fn Orientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOrientation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Orientation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintQuality) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintQuality(this, value).into())
        }
        unsafe extern "system" fn PrintQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintQuality) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrintQuality(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColorMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintColorMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorMode(this, value).into())
        }
        unsafe extern "system" fn ColorMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintColorMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColorMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDuplex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintDuplex) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuplex(this, value).into())
        }
        unsafe extern "system" fn Duplex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintDuplex) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Duplex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCollation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintCollation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCollation(this, value).into())
        }
        unsafe extern "system" fn Collation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintCollation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Collation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStaple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintStaple) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStaple(this, value).into())
        }
        unsafe extern "system" fn Staple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintStaple) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Staple(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHolePunch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintHolePunch) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHolePunch(this, value).into())
        }
        unsafe extern "system" fn HolePunch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintHolePunch) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HolePunch(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBinding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintBinding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBinding(this, value).into())
        }
        unsafe extern "system" fn Binding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintBinding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Binding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MinCopies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinCopies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxCopies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxCopies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNumberOfCopies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumberOfCopies(this, value).into())
        }
        unsafe extern "system" fn NumberOfCopies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfCopies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintTaskOptionsCoreProperties_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMediaSize: SetMediaSize::<Identity, Impl, OFFSET>,
            MediaSize: MediaSize::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            SetOrientation: SetOrientation::<Identity, Impl, OFFSET>,
            Orientation: Orientation::<Identity, Impl, OFFSET>,
            SetPrintQuality: SetPrintQuality::<Identity, Impl, OFFSET>,
            PrintQuality: PrintQuality::<Identity, Impl, OFFSET>,
            SetColorMode: SetColorMode::<Identity, Impl, OFFSET>,
            ColorMode: ColorMode::<Identity, Impl, OFFSET>,
            SetDuplex: SetDuplex::<Identity, Impl, OFFSET>,
            Duplex: Duplex::<Identity, Impl, OFFSET>,
            SetCollation: SetCollation::<Identity, Impl, OFFSET>,
            Collation: Collation::<Identity, Impl, OFFSET>,
            SetStaple: SetStaple::<Identity, Impl, OFFSET>,
            Staple: Staple::<Identity, Impl, OFFSET>,
            SetHolePunch: SetHolePunch::<Identity, Impl, OFFSET>,
            HolePunch: HolePunch::<Identity, Impl, OFFSET>,
            SetBinding: SetBinding::<Identity, Impl, OFFSET>,
            Binding: Binding::<Identity, Impl, OFFSET>,
            MinCopies: MinCopies::<Identity, Impl, OFFSET>,
            MaxCopies: MaxCopies::<Identity, Impl, OFFSET>,
            SetNumberOfCopies: SetNumberOfCopies::<Identity, Impl, OFFSET>,
            NumberOfCopies: NumberOfCopies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintTaskOptionsCoreUIConfiguration_Impl: ::windows_core::BaseImpl {
    fn DisplayedOptions(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IPrintTaskOptionsCoreUIConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTaskOptionsCoreUIConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayedOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskOptionsCoreUIConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayedOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintTaskOptionsCoreUIConfiguration_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayedOptions: DisplayedOptions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
