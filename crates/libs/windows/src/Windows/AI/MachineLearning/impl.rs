pub trait ILearningModelFeatureDescriptor_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Kind(this: &Self::This) -> ::windows_core::Result<LearningModelFeatureKind>;
    fn IsRequired(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for ILearningModelFeatureDescriptor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILearningModelFeatureDescriptor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Kind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelFeatureDescriptor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILearningModelFeatureDescriptor_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Kind: Kind::<Identity, Impl, OFFSET>,
            IsRequired: IsRequired::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILearningModelFeatureValue_Impl: ::windows_core::BaseImpl {
    fn Kind(this: &Self::This) -> ::windows_core::Result<LearningModelFeatureKind>;
}
impl ::windows_core::Iids for ILearningModelFeatureValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelFeatureValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILearningModelFeatureValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Kind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelFeatureValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILearningModelFeatureValue_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Kind: Kind::<Identity, Impl, OFFSET> }
    };
}
pub trait ILearningModelOperatorProvider_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ILearningModelOperatorProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILearningModelOperatorProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILearningModelOperatorProvider {
    const VTABLE: Self::Vtable = { ILearningModelOperatorProvider_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait ITensor_Impl: ::windows_core::BaseImpl + ILearningModelFeatureValue_Impl {
    fn TensorKind(this: &Self::This) -> ::windows_core::Result<TensorKind>;
    fn Shape(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for ITensor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<ILearningModelFeatureValue as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITensor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TensorKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TensorKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TensorKind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shape(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITensor_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TensorKind: TensorKind::<Identity, Impl, OFFSET>,
            Shape: Shape::<Identity, Impl, OFFSET>,
        }
    };
}
