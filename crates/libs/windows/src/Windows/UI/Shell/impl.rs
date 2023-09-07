pub trait IAdaptiveCard_Impl: ::windows_core::BaseImpl {
    fn ToJson(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IAdaptiveCard {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdaptiveCard_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdaptiveCard {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ToJson<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdaptiveCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToJson(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAdaptiveCard_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ToJson: ToJson::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAdaptiveCardBuilderStatics_Impl: ::windows_core::BaseImpl {
    fn CreateAdaptiveCardFromJson(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<IAdaptiveCard>;
}
impl ::windows_core::Iids for IAdaptiveCardBuilderStatics {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdaptiveCardBuilderStatics_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdaptiveCardBuilderStatics {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateAdaptiveCardFromJson<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdaptiveCardBuilderStatics_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAdaptiveCardFromJson(this, ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAdaptiveCardBuilderStatics_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateAdaptiveCardFromJson: CreateAdaptiveCardFromJson::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
