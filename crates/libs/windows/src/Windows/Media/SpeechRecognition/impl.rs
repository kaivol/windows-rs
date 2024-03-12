pub trait ISpeechRecognitionConstraint_Impl: ::windows_core::BaseImpl {
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetIsEnabled(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn Tag(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetTag(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<SpeechRecognitionConstraintType>;
    fn Probability(this: &Self::This) -> ::windows_core::Result<SpeechRecognitionConstraintProbability>;
    fn SetProbability(this: &Self::This, value: SpeechRecognitionConstraintProbability) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpeechRecognitionConstraint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechRecognitionConstraint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsEnabled(this, value).into())
        }
        unsafe extern "system" fn Tag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Tag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Probability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Probability(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProbability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechRecognitionConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProbability(this, value).into())
        }
        ISpeechRecognitionConstraint_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, Impl, OFFSET>,
            Tag: Tag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Probability: Probability::<Identity, Impl, OFFSET>,
            SetProbability: SetProbability::<Identity, Impl, OFFSET>,
        }
    };
}
