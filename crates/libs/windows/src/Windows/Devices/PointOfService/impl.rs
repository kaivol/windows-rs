pub trait ICashDrawerEventSourceEventArgs_Impl: ::windows_core::BaseImpl {
    fn CashDrawer(this: &Self::This) -> ::windows_core::Result<CashDrawer>;
}
impl ::windows_core::Iids for ICashDrawerEventSourceEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICashDrawerEventSourceEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICashDrawerEventSourceEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CashDrawer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICashDrawerEventSourceEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CashDrawer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICashDrawerEventSourceEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CashDrawer: CashDrawer::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICommonClaimedPosPrinterStation_Impl: ::windows_core::BaseImpl {
    fn SetCharactersPerLine(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn CharactersPerLine(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetLineHeight(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn LineHeight(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetLineSpacing(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn LineSpacing(this: &Self::This) -> ::windows_core::Result<u32>;
    fn LineWidth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetIsLetterQuality(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn IsLetterQuality(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsPaperNearEnd(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetColorCartridge(this: &Self::This, value: PosPrinterColorCartridge) -> ::windows_core::Result<()>;
    fn ColorCartridge(this: &Self::This) -> ::windows_core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsCartridgeRemoved(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsCartridgeEmpty(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsHeadCleaning(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsPaperEmpty(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsReadyToPrint(this: &Self::This) -> ::windows_core::Result<bool>;
    fn ValidateData(this: &Self::This, data: &::windows_core::HSTRING) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for ICommonClaimedPosPrinterStation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommonClaimedPosPrinterStation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCharactersPerLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCharactersPerLine(this, value).into())
        }
        unsafe extern "system" fn CharactersPerLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CharactersPerLine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLineHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineHeight(this, value).into())
        }
        unsafe extern "system" fn LineHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LineHeight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineSpacing(this, value).into())
        }
        unsafe extern "system" fn LineSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LineSpacing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LineWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LineWidth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsLetterQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsLetterQuality(this, value).into())
        }
        unsafe extern "system" fn IsLetterQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLetterQuality(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPaperNearEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPaperNearEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColorCartridge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorCartridge(this, value).into())
        }
        unsafe extern "system" fn ColorCartridge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColorCartridge(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCoverOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCoverOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCartridgeRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCartridgeRemoved(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCartridgeEmpty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCartridgeEmpty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsHeadCleaning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsHeadCleaning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPaperEmpty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPaperEmpty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsReadyToPrint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsReadyToPrint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValidateData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonClaimedPosPrinterStation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValidateData(this, ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICommonClaimedPosPrinterStation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCharactersPerLine: SetCharactersPerLine::<Identity, Impl, OFFSET>,
            CharactersPerLine: CharactersPerLine::<Identity, Impl, OFFSET>,
            SetLineHeight: SetLineHeight::<Identity, Impl, OFFSET>,
            LineHeight: LineHeight::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
            LineSpacing: LineSpacing::<Identity, Impl, OFFSET>,
            LineWidth: LineWidth::<Identity, Impl, OFFSET>,
            SetIsLetterQuality: SetIsLetterQuality::<Identity, Impl, OFFSET>,
            IsLetterQuality: IsLetterQuality::<Identity, Impl, OFFSET>,
            IsPaperNearEnd: IsPaperNearEnd::<Identity, Impl, OFFSET>,
            SetColorCartridge: SetColorCartridge::<Identity, Impl, OFFSET>,
            ColorCartridge: ColorCartridge::<Identity, Impl, OFFSET>,
            IsCoverOpen: IsCoverOpen::<Identity, Impl, OFFSET>,
            IsCartridgeRemoved: IsCartridgeRemoved::<Identity, Impl, OFFSET>,
            IsCartridgeEmpty: IsCartridgeEmpty::<Identity, Impl, OFFSET>,
            IsHeadCleaning: IsHeadCleaning::<Identity, Impl, OFFSET>,
            IsPaperEmpty: IsPaperEmpty::<Identity, Impl, OFFSET>,
            IsReadyToPrint: IsReadyToPrint::<Identity, Impl, OFFSET>,
            ValidateData: ValidateData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonPosPrintStationCapabilities_Impl: ::windows_core::BaseImpl {
    fn IsPrinterPresent(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsDualColorSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn ColorCartridgeCapabilities(this: &Self::This) -> ::windows_core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(this: &Self::This) -> ::windows_core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsItalicSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsUnderlineSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsDoubleHighPrintSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsDoubleWidePrintSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsPaperEmptySensorSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsPaperNearEndSensorSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SupportedCharactersPerLine(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for ICommonPosPrintStationCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommonPosPrintStationCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsPrinterPresent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPrinterPresent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDualColorSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDualColorSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ColorCartridgeCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColorCartridgeCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CartridgeSensors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CartridgeSensors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsBoldSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBoldSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsItalicSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsItalicSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsUnderlineSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUnderlineSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDoubleHighPrintSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDoubleHighPrintSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDoubleWidePrintSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDoubleWidePrintSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDoubleHighDoubleWidePrintSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDoubleHighDoubleWidePrintSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPaperEmptySensorSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPaperEmptySensorSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPaperNearEndSensorSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPaperNearEndSensorSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedCharactersPerLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonPosPrintStationCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedCharactersPerLine(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICommonPosPrintStationCapabilities_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsPrinterPresent: IsPrinterPresent::<Identity, Impl, OFFSET>,
            IsDualColorSupported: IsDualColorSupported::<Identity, Impl, OFFSET>,
            ColorCartridgeCapabilities: ColorCartridgeCapabilities::<Identity, Impl, OFFSET>,
            CartridgeSensors: CartridgeSensors::<Identity, Impl, OFFSET>,
            IsBoldSupported: IsBoldSupported::<Identity, Impl, OFFSET>,
            IsItalicSupported: IsItalicSupported::<Identity, Impl, OFFSET>,
            IsUnderlineSupported: IsUnderlineSupported::<Identity, Impl, OFFSET>,
            IsDoubleHighPrintSupported: IsDoubleHighPrintSupported::<Identity, Impl, OFFSET>,
            IsDoubleWidePrintSupported: IsDoubleWidePrintSupported::<Identity, Impl, OFFSET>,
            IsDoubleHighDoubleWidePrintSupported: IsDoubleHighDoubleWidePrintSupported::<Identity, Impl, OFFSET>,
            IsPaperEmptySensorSupported: IsPaperEmptySensorSupported::<Identity, Impl, OFFSET>,
            IsPaperNearEndSensorSupported: IsPaperNearEndSensorSupported::<Identity, Impl, OFFSET>,
            SupportedCharactersPerLine: SupportedCharactersPerLine::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait ICommonReceiptSlipCapabilities_Impl: ::windows_core::BaseImpl + ICommonPosPrintStationCapabilities_Impl {
    fn IsBarcodeSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsBitmapSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsLeft90RotationSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsRight90RotationSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn Is180RotationSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn IsPrintAreaSupported(this: &Self::This) -> ::windows_core::Result<bool>;
    fn RuledLineCapabilities(this: &Self::This) -> ::windows_core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for ICommonReceiptSlipCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<ICommonPosPrintStationCapabilities as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommonReceiptSlipCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsBarcodeSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBarcodeSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsBitmapSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBitmapSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLeft90RotationSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLeft90RotationSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRight90RotationSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRight90RotationSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Is180RotationSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Is180RotationSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPrintAreaSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPrintAreaSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RuledLineCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RuledLineCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedBarcodeRotations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedBarcodeRotations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedBitmapRotations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommonReceiptSlipCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedBitmapRotations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICommonReceiptSlipCapabilities_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsBarcodeSupported: IsBarcodeSupported::<Identity, Impl, OFFSET>,
            IsBitmapSupported: IsBitmapSupported::<Identity, Impl, OFFSET>,
            IsLeft90RotationSupported: IsLeft90RotationSupported::<Identity, Impl, OFFSET>,
            IsRight90RotationSupported: IsRight90RotationSupported::<Identity, Impl, OFFSET>,
            Is180RotationSupported: Is180RotationSupported::<Identity, Impl, OFFSET>,
            IsPrintAreaSupported: IsPrintAreaSupported::<Identity, Impl, OFFSET>,
            RuledLineCapabilities: RuledLineCapabilities::<Identity, Impl, OFFSET>,
            SupportedBarcodeRotations: SupportedBarcodeRotations::<Identity, Impl, OFFSET>,
            SupportedBitmapRotations: SupportedBitmapRotations::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPosPrinterJob_Impl: ::windows_core::BaseImpl {
    fn Print(this: &Self::This, data: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn PrintLine(this: &Self::This, data: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn PrintNewline(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExecuteAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPosPrinterJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPosPrinterJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Print<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Print(this, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn PrintLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintLine(this, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn PrintNewline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintNewline(this).into())
        }
        unsafe extern "system" fn ExecuteAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPosPrinterJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecuteAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPosPrinterJob_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Print: Print::<Identity, Impl, OFFSET>,
            PrintLine: PrintLine::<Identity, Impl, OFFSET>,
            PrintNewline: PrintNewline::<Identity, Impl, OFFSET>,
            ExecuteAsync: ExecuteAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
pub trait IReceiptOrSlipJob_Impl: ::windows_core::BaseImpl + IPosPrinterJob_Impl {
    fn SetBarcodeRotation(this: &Self::This, value: PosPrinterRotation) -> ::windows_core::Result<()>;
    fn SetPrintRotation(this: &Self::This, value: PosPrinterRotation, includebitmaps: bool) -> ::windows_core::Result<()>;
    fn SetPrintArea(this: &Self::This, value: &super::super::Foundation::Rect) -> ::windows_core::Result<()>;
    fn SetBitmap(this: &Self::This, bitmapnumber: u32, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows_core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(this: &Self::This, bitmapnumber: u32, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()>;
    fn SetCustomAlignedBitmap(this: &Self::This, bitmapnumber: u32, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows_core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(this: &Self::This, bitmapnumber: u32, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()>;
    fn PrintSavedBitmap(this: &Self::This, bitmapnumber: u32) -> ::windows_core::Result<()>;
    fn DrawRuledLine(this: &Self::This, positionlist: &::windows_core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows_core::Result<()>;
    fn PrintBarcode(this: &Self::This, data: &::windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows_core::Result<()>;
    fn PrintBarcodeCustomAlign(this: &Self::This, data: &::windows_core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows_core::Result<()>;
    fn PrintBitmap(this: &Self::This, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows_core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(this: &Self::This, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::Result<()>;
    fn PrintCustomAlignedBitmap(this: &Self::This, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows_core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(this: &Self::This, bitmap: ::core::option::Option<&super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl ::windows_core::Iids for IReceiptOrSlipJob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPosPrinterJob as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReceiptOrSlipJob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetBarcodeRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBarcodeRotation(this, value).into())
        }
        unsafe extern "system" fn SetPrintRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintRotation(this, value, includebitmaps).into())
        }
        unsafe extern "system" fn SetPrintArea<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintArea(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn SetBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitmap(this, bitmapnumber, ::windows_core::from_raw_borrowed(&bitmap), alignment).into())
        }
        unsafe extern "system" fn SetBitmapCustomWidthStandardAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitmapCustomWidthStandardAlign(this, bitmapnumber, ::windows_core::from_raw_borrowed(&bitmap), alignment, width).into())
        }
        unsafe extern "system" fn SetCustomAlignedBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCustomAlignedBitmap(this, bitmapnumber, ::windows_core::from_raw_borrowed(&bitmap), alignmentdistance).into())
        }
        unsafe extern "system" fn SetBitmapCustomWidthCustomAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitmapCustomWidthCustomAlign(this, bitmapnumber, ::windows_core::from_raw_borrowed(&bitmap), alignmentdistance, width).into())
        }
        unsafe extern "system" fn PrintSavedBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintSavedBitmap(this, bitmapnumber).into())
        }
        unsafe extern "system" fn DrawRuledLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positionlist: ::std::mem::MaybeUninit<::windows_core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawRuledLine(this, ::core::mem::transmute(&positionlist), linedirection, linewidth, linestyle, linecolor).into())
        }
        unsafe extern "system" fn PrintBarcode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintBarcode(this, ::core::mem::transmute(&data), symbology, height, width, textposition, alignment).into())
        }
        unsafe extern "system" fn PrintBarcodeCustomAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintBarcodeCustomAlign(this, ::core::mem::transmute(&data), symbology, height, width, textposition, alignmentdistance).into())
        }
        unsafe extern "system" fn PrintBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintBitmap(this, ::windows_core::from_raw_borrowed(&bitmap), alignment).into())
        }
        unsafe extern "system" fn PrintBitmapCustomWidthStandardAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignment: PosPrinterAlignment, width: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintBitmapCustomWidthStandardAlign(this, ::windows_core::from_raw_borrowed(&bitmap), alignment, width).into())
        }
        unsafe extern "system" fn PrintCustomAlignedBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintCustomAlignedBitmap(this, ::windows_core::from_raw_borrowed(&bitmap), alignmentdistance).into())
        }
        unsafe extern "system" fn PrintBitmapCustomWidthCustomAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReceiptOrSlipJob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, alignmentdistance: u32, width: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrintBitmapCustomWidthCustomAlign(this, ::windows_core::from_raw_borrowed(&bitmap), alignmentdistance, width).into())
        }
        IReceiptOrSlipJob_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetBarcodeRotation: SetBarcodeRotation::<Identity, Impl, OFFSET>,
            SetPrintRotation: SetPrintRotation::<Identity, Impl, OFFSET>,
            SetPrintArea: SetPrintArea::<Identity, Impl, OFFSET>,
            SetBitmap: SetBitmap::<Identity, Impl, OFFSET>,
            SetBitmapCustomWidthStandardAlign: SetBitmapCustomWidthStandardAlign::<Identity, Impl, OFFSET>,
            SetCustomAlignedBitmap: SetCustomAlignedBitmap::<Identity, Impl, OFFSET>,
            SetBitmapCustomWidthCustomAlign: SetBitmapCustomWidthCustomAlign::<Identity, Impl, OFFSET>,
            PrintSavedBitmap: PrintSavedBitmap::<Identity, Impl, OFFSET>,
            DrawRuledLine: DrawRuledLine::<Identity, Impl, OFFSET>,
            PrintBarcode: PrintBarcode::<Identity, Impl, OFFSET>,
            PrintBarcodeCustomAlign: PrintBarcodeCustomAlign::<Identity, Impl, OFFSET>,
            PrintBitmap: PrintBitmap::<Identity, Impl, OFFSET>,
            PrintBitmapCustomWidthStandardAlign: PrintBitmapCustomWidthStandardAlign::<Identity, Impl, OFFSET>,
            PrintCustomAlignedBitmap: PrintCustomAlignedBitmap::<Identity, Impl, OFFSET>,
            PrintBitmapCustomWidthCustomAlign: PrintBitmapCustomWidthCustomAlign::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
