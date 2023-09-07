pub trait IPrintCustomOptionDetails_Impl: ::windows_core::BaseImpl + IPrintOptionDetails_Impl {
    fn SetDisplayName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IPrintCustomOptionDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPrintOptionDetails as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintCustomOptionDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintCustomOptionDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintItemListOptionDetails_Impl: ::windows_core::BaseImpl + IPrintOptionDetails_Impl {
    fn Items(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IPrintItemListOptionDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPrintOptionDetails as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintItemListOptionDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintItemListOptionDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Items<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintItemListOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Items(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintItemListOptionDetails_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Items: Items::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPrintNumberOptionDetails_Impl: ::windows_core::BaseImpl + IPrintOptionDetails_Impl {
    fn MinValue(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MaxValue(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IPrintNumberOptionDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPrintOptionDetails as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintNumberOptionDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MinValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MinValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintNumberOptionDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MinValue: MinValue::<Identity, Impl, OFFSET>,
            MaxValue: MaxValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPrintOptionDetails_Impl: ::windows_core::BaseImpl {
    fn OptionId(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn OptionType(this: &Self::This) -> ::windows_core::Result<PrintOptionType>;
    fn SetErrorText(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn ErrorText(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetState(this: &Self::This, value: PrintOptionStates) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<PrintOptionStates>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn TrySetValue(this: &Self::This, value: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IPrintOptionDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintOptionDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OptionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OptionId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OptionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OptionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetErrorText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorText(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ErrorText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ErrorText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PrintOptionStates) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, value).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionStates) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrySetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrySetValue(this, ::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintOptionDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OptionId: OptionId::<Identity, Impl, OFFSET>,
            OptionType: OptionType::<Identity, Impl, OFFSET>,
            SetErrorText: SetErrorText::<Identity, Impl, OFFSET>,
            ErrorText: ErrorText::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            TrySetValue: TrySetValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPrintTextOptionDetails_Impl: ::windows_core::BaseImpl + IPrintOptionDetails_Impl {
    fn MaxCharacters(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IPrintTextOptionDetails {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPrintOptionDetails as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTextOptionDetails_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTextOptionDetails {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MaxCharacters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTextOptionDetails_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxCharacters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPrintTextOptionDetails_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MaxCharacters: MaxCharacters::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
