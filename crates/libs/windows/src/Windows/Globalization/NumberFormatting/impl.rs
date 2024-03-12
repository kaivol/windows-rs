pub trait INumberFormatter_Impl: ::windows_core::BaseImpl {
    fn FormatInt(this: &Self::This, value: i64) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn FormatUInt(this: &Self::This, value: u64) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn FormatDouble(this: &Self::This, value: f64) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for INumberFormatter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INumberFormatter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FormatInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatInt(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FormatUInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatUInt(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FormatDouble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatDouble(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INumberFormatter_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FormatInt: FormatInt::<Identity, Impl, OFFSET>,
            FormatUInt: FormatUInt::<Identity, Impl, OFFSET>,
            FormatDouble: FormatDouble::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INumberFormatter2_Impl: ::windows_core::BaseImpl {
    fn FormatInt(this: &Self::This, value: i64) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn FormatUInt(this: &Self::This, value: u64) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn FormatDouble(this: &Self::This, value: f64) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for INumberFormatter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INumberFormatter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FormatInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatInt(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FormatUInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatUInt(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FormatDouble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatDouble(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INumberFormatter2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FormatInt: FormatInt::<Identity, Impl, OFFSET>,
            FormatUInt: FormatUInt::<Identity, Impl, OFFSET>,
            FormatDouble: FormatDouble::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait INumberFormatterOptions_Impl: ::windows_core::BaseImpl {
    fn Languages(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>;
    fn GeographicRegion(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn IntegerDigits(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetIntegerDigits(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn FractionDigits(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetFractionDigits(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn IsGrouped(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetIsGrouped(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn IsDecimalPointAlwaysDisplayed(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetIsDecimalPointAlwaysDisplayed(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn NumeralSystem(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetNumeralSystem(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn ResolvedLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ResolvedGeographicRegion(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for INumberFormatterOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INumberFormatterOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Languages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Languages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GeographicRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GeographicRegion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IntegerDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IntegerDigits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIntegerDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIntegerDigits(this, value).into())
        }
        unsafe extern "system" fn FractionDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FractionDigits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFractionDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFractionDigits(this, value).into())
        }
        unsafe extern "system" fn IsGrouped<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsGrouped(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsGrouped<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsGrouped(this, value).into())
        }
        unsafe extern "system" fn IsDecimalPointAlwaysDisplayed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDecimalPointAlwaysDisplayed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsDecimalPointAlwaysDisplayed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsDecimalPointAlwaysDisplayed(this, value).into())
        }
        unsafe extern "system" fn NumeralSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumeralSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNumeralSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumeralSystem(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ResolvedLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResolvedLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResolvedGeographicRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberFormatterOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResolvedGeographicRegion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INumberFormatterOptions_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Languages: Languages::<Identity, Impl, OFFSET>,
            GeographicRegion: GeographicRegion::<Identity, Impl, OFFSET>,
            IntegerDigits: IntegerDigits::<Identity, Impl, OFFSET>,
            SetIntegerDigits: SetIntegerDigits::<Identity, Impl, OFFSET>,
            FractionDigits: FractionDigits::<Identity, Impl, OFFSET>,
            SetFractionDigits: SetFractionDigits::<Identity, Impl, OFFSET>,
            IsGrouped: IsGrouped::<Identity, Impl, OFFSET>,
            SetIsGrouped: SetIsGrouped::<Identity, Impl, OFFSET>,
            IsDecimalPointAlwaysDisplayed: IsDecimalPointAlwaysDisplayed::<Identity, Impl, OFFSET>,
            SetIsDecimalPointAlwaysDisplayed: SetIsDecimalPointAlwaysDisplayed::<Identity, Impl, OFFSET>,
            NumeralSystem: NumeralSystem::<Identity, Impl, OFFSET>,
            SetNumeralSystem: SetNumeralSystem::<Identity, Impl, OFFSET>,
            ResolvedLanguage: ResolvedLanguage::<Identity, Impl, OFFSET>,
            ResolvedGeographicRegion: ResolvedGeographicRegion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait INumberParser_Impl: ::windows_core::BaseImpl {
    fn ParseInt(this: &Self::This, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(this: &Self::This, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(this: &Self::This, text: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for INumberParser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INumberParser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseInt(this, ::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ParseUInt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseUInt(this, ::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ParseDouble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseDouble(this, ::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INumberParser_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseInt: ParseInt::<Identity, Impl, OFFSET>,
            ParseUInt: ParseUInt::<Identity, Impl, OFFSET>,
            ParseDouble: ParseDouble::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INumberRounder_Impl: ::windows_core::BaseImpl {
    fn RoundInt32(this: &Self::This, value: i32) -> ::windows_core::Result<i32>;
    fn RoundUInt32(this: &Self::This, value: u32) -> ::windows_core::Result<u32>;
    fn RoundInt64(this: &Self::This, value: i64) -> ::windows_core::Result<i64>;
    fn RoundUInt64(this: &Self::This, value: u64) -> ::windows_core::Result<u64>;
    fn RoundSingle(this: &Self::This, value: f32) -> ::windows_core::Result<f32>;
    fn RoundDouble(this: &Self::This, value: f64) -> ::windows_core::Result<f64>;
}
impl ::windows_core::Iids for INumberRounder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INumberRounder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RoundInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoundInt32(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoundUInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoundUInt32(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoundInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoundInt64(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoundUInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoundUInt64(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoundSingle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoundSingle(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RoundDouble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RoundDouble(this, value) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INumberRounder_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RoundInt32: RoundInt32::<Identity, Impl, OFFSET>,
            RoundUInt32: RoundUInt32::<Identity, Impl, OFFSET>,
            RoundInt64: RoundInt64::<Identity, Impl, OFFSET>,
            RoundUInt64: RoundUInt64::<Identity, Impl, OFFSET>,
            RoundSingle: RoundSingle::<Identity, Impl, OFFSET>,
            RoundDouble: RoundDouble::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INumberRounderOption_Impl: ::windows_core::BaseImpl {
    fn NumberRounder(this: &Self::This) -> ::windows_core::Result<INumberRounder>;
    fn SetNumberRounder(this: &Self::This, value: ::core::option::Option<&INumberRounder>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INumberRounderOption {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounderOption_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INumberRounderOption {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NumberRounder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounderOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberRounder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNumberRounder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INumberRounderOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumberRounder(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        INumberRounderOption_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NumberRounder: NumberRounder::<Identity, Impl, OFFSET>,
            SetNumberRounder: SetNumberRounder::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISignedZeroOption_Impl: ::windows_core::BaseImpl {
    fn IsZeroSigned(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetIsZeroSigned(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISignedZeroOption {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISignedZeroOption_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISignedZeroOption {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsZeroSigned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISignedZeroOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsZeroSigned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsZeroSigned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISignedZeroOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsZeroSigned(this, value).into())
        }
        ISignedZeroOption_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsZeroSigned: IsZeroSigned::<Identity, Impl, OFFSET>,
            SetIsZeroSigned: SetIsZeroSigned::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISignificantDigitsOption_Impl: ::windows_core::BaseImpl {
    fn SignificantDigits(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSignificantDigits(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISignificantDigitsOption {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISignificantDigitsOption_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISignificantDigitsOption {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SignificantDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISignificantDigitsOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SignificantDigits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignificantDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISignificantDigitsOption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignificantDigits(this, value).into())
        }
        ISignificantDigitsOption_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SignificantDigits: SignificantDigits::<Identity, Impl, OFFSET>,
            SetSignificantDigits: SetSignificantDigits::<Identity, Impl, OFFSET>,
        }
    };
}
