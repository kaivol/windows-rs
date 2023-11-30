#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IPointerPointTransform_Impl: ::windows_core::BaseImpl {
    fn Inverse(this: &Self::This) -> ::windows_core::Result<IPointerPointTransform>;
    fn TryTransform(this: &Self::This, inpoint: &super::super::Foundation::Point, outpoint: &mut super::super::Foundation::Point) -> ::windows_core::Result<bool>;
    fn TransformBounds(this: &Self::This, rect: &super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IPointerPointTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerPointTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPointerPointTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Inverse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerPointTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Inverse(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TryTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerPointTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inpoint: super::super::Foundation::Point, outpoint: *mut super::super::Foundation::Point, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TryTransform(this, ::core::mem::transmute(&inpoint), ::core::mem::transmute_copy(&outpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransformBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerPointTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransformBounds(this, ::core::mem::transmute(&rect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPointerPointTransform_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Inverse: Inverse::<Identity, Impl, OFFSET>,
            TryTransform: TryTransform::<Identity, Impl, OFFSET>,
            TransformBounds: TransformBounds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
