#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionAffineTransform2DEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetInterpolationMode(this: &Self::This, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows_core::Result<()>;
    fn SetBorderMode(this: &Self::This, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::Result<()>;
    fn SetTransformMatrix(this: &Self::This, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()>;
    fn SetTransformMatrixElement(this: &Self::This, row: i32, column: i32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetTransformMatrixElement2(this: &Self::This, row: i32, column: i32, value: f32) -> ::windows_core::Result<()>;
    fn SetSharpness(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetSharpness2(this: &Self::This, sharpness: f32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionAffineTransform2DEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionAffineTransform2DEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterpolationMode(this, ::core::mem::transmute_copy(&interpolationmode)).into())
        }
        unsafe extern "system" fn SetBorderMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBorderMode(this, ::core::mem::transmute_copy(&bordermode)).into())
        }
        unsafe extern "system" fn SetTransformMatrix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformMatrix(this, ::core::mem::transmute_copy(&transformmatrix)).into())
        }
        unsafe extern "system" fn SetTransformMatrixElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformMatrixElement(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetTransformMatrixElement2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformMatrixElement2(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetSharpness<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSharpness(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetSharpness2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAffineTransform2DEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSharpness2(this, ::core::mem::transmute_copy(&sharpness)).into())
        }
        IDCompositionAffineTransform2DEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInterpolationMode: SetInterpolationMode::<Identity, Impl, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, Impl, OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Identity, Impl, OFFSET>,
            SetTransformMatrixElement: SetTransformMatrixElement::<Identity, Impl, OFFSET>,
            SetTransformMatrixElement2: SetTransformMatrixElement2::<Identity, Impl, OFFSET>,
            SetSharpness: SetSharpness::<Identity, Impl, OFFSET>,
            SetSharpness2: SetSharpness2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionAnimation_Impl: ::windows_core::BaseImpl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetAbsoluteBeginTime(this: &Self::This, begintime: i64) -> ::windows_core::Result<()>;
    fn AddCubic(this: &Self::This, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::Result<()>;
    fn AddSinusoidal(this: &Self::This, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::Result<()>;
    fn AddRepeat(this: &Self::This, beginoffset: f64, durationtorepeat: f64) -> ::windows_core::Result<()>;
    fn End(this: &Self::This, endoffset: f64, endvalue: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionAnimation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAnimation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionAnimation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAnimation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn SetAbsoluteBeginTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAnimation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAbsoluteBeginTime(this, ::core::mem::transmute_copy(&begintime)).into())
        }
        unsafe extern "system" fn AddCubic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAnimation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddCubic(this, ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&constantcoefficient), ::core::mem::transmute_copy(&linearcoefficient), ::core::mem::transmute_copy(&quadraticcoefficient), ::core::mem::transmute_copy(&cubiccoefficient)).into())
        }
        unsafe extern "system" fn AddSinusoidal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAnimation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSinusoidal(this, ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&bias), ::core::mem::transmute_copy(&amplitude), ::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&phase)).into())
        }
        unsafe extern "system" fn AddRepeat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAnimation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRepeat(this, ::core::mem::transmute_copy(&beginoffset), ::core::mem::transmute_copy(&durationtorepeat)).into())
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionAnimation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this, ::core::mem::transmute_copy(&endoffset), ::core::mem::transmute_copy(&endvalue)).into())
        }
        IDCompositionAnimation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetAbsoluteBeginTime: SetAbsoluteBeginTime::<Identity, Impl, OFFSET>,
            AddCubic: AddCubic::<Identity, Impl, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, Impl, OFFSET>,
            AddRepeat: AddRepeat::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionArithmeticCompositeEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetCoefficients(this: &Self::This, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::Result<()>;
    fn SetClampOutput(this: &Self::This, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetCoefficient1(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCoefficient12(this: &Self::This, coeffcient1: f32) -> ::windows_core::Result<()>;
    fn SetCoefficient2(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCoefficient22(this: &Self::This, coefficient2: f32) -> ::windows_core::Result<()>;
    fn SetCoefficient3(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCoefficient32(this: &Self::This, coefficient3: f32) -> ::windows_core::Result<()>;
    fn SetCoefficient4(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCoefficient42(this: &Self::This, coefficient4: f32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionArithmeticCompositeEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionArithmeticCompositeEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCoefficients<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficients(this, ::core::mem::transmute_copy(&coefficients)).into())
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClampOutput(this, ::core::mem::transmute_copy(&clampoutput)).into())
        }
        unsafe extern "system" fn SetCoefficient1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient1(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCoefficient12<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient12(this, ::core::mem::transmute_copy(&coeffcient1)).into())
        }
        unsafe extern "system" fn SetCoefficient2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient2(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCoefficient22<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient22(this, ::core::mem::transmute_copy(&coefficient2)).into())
        }
        unsafe extern "system" fn SetCoefficient3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient3(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCoefficient32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient32(this, ::core::mem::transmute_copy(&coefficient3)).into())
        }
        unsafe extern "system" fn SetCoefficient4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient4(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCoefficient42<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionArithmeticCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoefficient42(this, ::core::mem::transmute_copy(&coefficient4)).into())
        }
        IDCompositionArithmeticCompositeEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCoefficients: SetCoefficients::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
            SetCoefficient1: SetCoefficient1::<Identity, Impl, OFFSET>,
            SetCoefficient12: SetCoefficient12::<Identity, Impl, OFFSET>,
            SetCoefficient2: SetCoefficient2::<Identity, Impl, OFFSET>,
            SetCoefficient22: SetCoefficient22::<Identity, Impl, OFFSET>,
            SetCoefficient3: SetCoefficient3::<Identity, Impl, OFFSET>,
            SetCoefficient32: SetCoefficient32::<Identity, Impl, OFFSET>,
            SetCoefficient4: SetCoefficient4::<Identity, Impl, OFFSET>,
            SetCoefficient42: SetCoefficient42::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBlendEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetMode(this: &Self::This, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for IDCompositionBlendEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBlendEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionBlendEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBlendEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        IDCompositionBlendEffect_Vtbl { base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetMode: SetMode::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionBrightnessEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetWhitePoint(this: &Self::This, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()>;
    fn SetBlackPoint(this: &Self::This, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()>;
    fn SetWhitePointX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetWhitePointX2(this: &Self::This, whitepointx: f32) -> ::windows_core::Result<()>;
    fn SetWhitePointY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetWhitePointY2(this: &Self::This, whitepointy: f32) -> ::windows_core::Result<()>;
    fn SetBlackPointX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBlackPointX2(this: &Self::This, blackpointx: f32) -> ::windows_core::Result<()>;
    fn SetBlackPointY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBlackPointY2(this: &Self::This, blackpointy: f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for IDCompositionBrightnessEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionBrightnessEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetWhitePoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWhitePoint(this, ::core::mem::transmute_copy(&whitepoint)).into())
        }
        unsafe extern "system" fn SetBlackPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlackPoint(this, ::core::mem::transmute_copy(&blackpoint)).into())
        }
        unsafe extern "system" fn SetWhitePointX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWhitePointX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetWhitePointX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWhitePointX2(this, ::core::mem::transmute_copy(&whitepointx)).into())
        }
        unsafe extern "system" fn SetWhitePointY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWhitePointY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetWhitePointY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWhitePointY2(this, ::core::mem::transmute_copy(&whitepointy)).into())
        }
        unsafe extern "system" fn SetBlackPointX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlackPointX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBlackPointX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlackPointX2(this, ::core::mem::transmute_copy(&blackpointx)).into())
        }
        unsafe extern "system" fn SetBlackPointY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlackPointY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBlackPointY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionBrightnessEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlackPointY2(this, ::core::mem::transmute_copy(&blackpointy)).into())
        }
        IDCompositionBrightnessEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetWhitePoint: SetWhitePoint::<Identity, Impl, OFFSET>,
            SetBlackPoint: SetBlackPoint::<Identity, Impl, OFFSET>,
            SetWhitePointX: SetWhitePointX::<Identity, Impl, OFFSET>,
            SetWhitePointX2: SetWhitePointX2::<Identity, Impl, OFFSET>,
            SetWhitePointY: SetWhitePointY::<Identity, Impl, OFFSET>,
            SetWhitePointY2: SetWhitePointY2::<Identity, Impl, OFFSET>,
            SetBlackPointX: SetBlackPointX::<Identity, Impl, OFFSET>,
            SetBlackPointX2: SetBlackPointX2::<Identity, Impl, OFFSET>,
            SetBlackPointY: SetBlackPointY::<Identity, Impl, OFFSET>,
            SetBlackPointY2: SetBlackPointY2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionClip_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IDCompositionClip {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionClip_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionClip {
    const VTABLE: Self::Vtable = { IDCompositionClip_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionColorMatrixEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetMatrix(this: &Self::This, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows_core::Result<()>;
    fn SetMatrixElement(this: &Self::This, row: i32, column: i32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetMatrixElement2(this: &Self::This, row: i32, column: i32, value: f32) -> ::windows_core::Result<()>;
    fn SetAlphaMode(this: &Self::This, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows_core::Result<()>;
    fn SetClampOutput(this: &Self::This, clamp: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionColorMatrixEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionColorMatrixEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMatrix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrix(this, ::core::mem::transmute_copy(&matrix)).into())
        }
        unsafe extern "system" fn SetMatrixElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrixElement(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrixElement2(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetAlphaMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionColorMatrixEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClampOutput(this, ::core::mem::transmute_copy(&clamp)).into())
        }
        IDCompositionColorMatrixEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, Impl, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, Impl, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionCompositeEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetMode(this: &Self::This, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for IDCompositionCompositeEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionCompositeEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionCompositeEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionCompositeEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        IDCompositionCompositeEffect_Vtbl { base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetMode: SetMode::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionDelegatedInkTrail_Impl: ::windows_core::BaseImpl {
    fn AddTrailPoints(this: &Self::This, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32) -> ::windows_core::Result<u32>;
    fn AddTrailPointsWithPrediction(this: &Self::This, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32) -> ::windows_core::Result<u32>;
    fn RemoveTrailPoints(this: &Self::This, generationid: u32) -> ::windows_core::Result<()>;
    fn StartNewTrail(this: &Self::This, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for IDCompositionDelegatedInkTrail {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionDelegatedInkTrail {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddTrailPoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddTrailPoints(this, ::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(generationid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddTrailPointsWithPrediction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddTrailPointsWithPrediction(this, ::core::mem::transmute_copy(&inkpoints), ::core::mem::transmute_copy(&inkpointscount), ::core::mem::transmute_copy(&predictedinkpoints), ::core::mem::transmute_copy(&predictedinkpointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(generationid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveTrailPoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveTrailPoints(this, ::core::mem::transmute_copy(&generationid)).into())
        }
        unsafe extern "system" fn StartNewTrail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDelegatedInkTrail_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartNewTrail(this, ::core::mem::transmute_copy(&color)).into())
        }
        IDCompositionDelegatedInkTrail_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddTrailPoints: AddTrailPoints::<Identity, Impl, OFFSET>,
            AddTrailPointsWithPrediction: AddTrailPointsWithPrediction::<Identity, Impl, OFFSET>,
            RemoveTrailPoints: RemoveTrailPoints::<Identity, Impl, OFFSET>,
            StartNewTrail: StartNewTrail::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDCompositionDesktopDevice_Impl: ::windows_core::BaseImpl + IDCompositionDevice2_Impl {
    fn CreateTargetForHwnd(this: &Self::This, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> ::windows_core::Result<IDCompositionTarget>;
    fn CreateSurfaceFromHandle(this: &Self::This, handle: super::super::Foundation::HANDLE) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateSurfaceFromHwnd(this: &Self::This, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDCompositionDesktopDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionDevice2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionDesktopDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTargetForHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTargetForHwnd(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(target, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurfaceFromHandle(this, ::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDesktopDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurfaceFromHwnd(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDCompositionDesktopDevice_Vtbl {
            base__: <IDCompositionDevice2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDCompositionDevice_Impl: ::windows_core::BaseImpl {
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForCommitCompletion(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetFrameStatistics(this: &Self::This, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::Result<()>;
    fn CreateTargetForHwnd(this: &Self::This, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL) -> ::windows_core::Result<IDCompositionTarget>;
    fn CreateVisual(this: &Self::This) -> ::windows_core::Result<IDCompositionVisual>;
    fn CreateSurface(this: &Self::This, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(this: &Self::This, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface>;
    fn CreateSurfaceFromHandle(this: &Self::This, handle: super::super::Foundation::HANDLE) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateSurfaceFromHwnd(this: &Self::This, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateTranslateTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(this: &Self::This, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows_core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(this: &Self::This, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows_core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(this: &Self::This) -> ::windows_core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(this: &Self::This) -> ::windows_core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(this: &Self::This) -> ::windows_core::Result<IDCompositionAnimation>;
    fn CheckDeviceState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IDCompositionDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForCommitCompletion(this).into())
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameStatistics(this, ::core::mem::transmute_copy(&statistics)).into())
        }
        unsafe extern "system" fn CreateTargetForHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTargetForHwnd(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&topmost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(target, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVisual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVisual(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visual, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurface(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVirtualSurface(this, ::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(virtualsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurfaceFromHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurfaceFromHandle(this, ::core::mem::transmute_copy(&handle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurfaceFromHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurfaceFromHwnd(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTranslateTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(translatetransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateScaleTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scaletransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRotateTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rotatetransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSkewTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(skewtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMatrixTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransformGroup(this, ::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTranslateTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(translatetransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateScaleTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scaletransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRotateTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rotatetransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMatrixTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransform3DGroup(this, ::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform3dgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEffectGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(effectgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clip: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRectangleClip(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAnimation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAnimation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(animation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckDeviceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckDeviceState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfvalid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDCompositionDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Commit: Commit::<Identity, Impl, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
            CreateTargetForHwnd: CreateTargetForHwnd::<Identity, Impl, OFFSET>,
            CreateVisual: CreateVisual::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHandle: CreateSurfaceFromHandle::<Identity, Impl, OFFSET>,
            CreateSurfaceFromHwnd: CreateSurfaceFromHwnd::<Identity, Impl, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, Impl, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, Impl, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, Impl, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, Impl, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, Impl, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, Impl, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, Impl, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, Impl, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, Impl, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, Impl, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, Impl, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, Impl, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, Impl, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, Impl, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice2_Impl: ::windows_core::BaseImpl {
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForCommitCompletion(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetFrameStatistics(this: &Self::This, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::Result<()>;
    fn CreateVisual(this: &Self::This) -> ::windows_core::Result<IDCompositionVisual2>;
    fn CreateSurfaceFactory(this: &Self::This, renderingdevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDCompositionSurfaceFactory>;
    fn CreateSurface(this: &Self::This, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(this: &Self::This, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface>;
    fn CreateTranslateTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionTranslateTransform>;
    fn CreateScaleTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionScaleTransform>;
    fn CreateRotateTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionRotateTransform>;
    fn CreateSkewTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionSkewTransform>;
    fn CreateMatrixTransform(this: &Self::This) -> ::windows_core::Result<IDCompositionMatrixTransform>;
    fn CreateTransformGroup(this: &Self::This, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows_core::Result<IDCompositionTransform>;
    fn CreateTranslateTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionTranslateTransform3D>;
    fn CreateScaleTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionScaleTransform3D>;
    fn CreateRotateTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionRotateTransform3D>;
    fn CreateMatrixTransform3D(this: &Self::This) -> ::windows_core::Result<IDCompositionMatrixTransform3D>;
    fn CreateTransform3DGroup(this: &Self::This, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows_core::Result<IDCompositionTransform3D>;
    fn CreateEffectGroup(this: &Self::This) -> ::windows_core::Result<IDCompositionEffectGroup>;
    fn CreateRectangleClip(this: &Self::This) -> ::windows_core::Result<IDCompositionRectangleClip>;
    fn CreateAnimation(this: &Self::This) -> ::windows_core::Result<IDCompositionAnimation>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IDCompositionDevice2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionDevice2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn WaitForCommitCompletion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForCommitCompletion(this).into())
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameStatistics(this, ::core::mem::transmute_copy(&statistics)).into())
        }
        unsafe extern "system" fn CreateVisual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVisual(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visual, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurfaceFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurfaceFactory(this, ::windows_core::from_raw_borrowed(&renderingdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surfacefactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurface(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVirtualSurface(this, ::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(virtualsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTranslateTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTranslateTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(translatetransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateScaleTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateScaleTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scaletransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRotateTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRotateTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rotatetransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSkewTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSkewTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(skewtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMatrixTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTransformGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransformGroup(this, ::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTranslateTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTranslateTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(translatetransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateScaleTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateScaleTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scaletransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRotateTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRotateTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rotatetransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMatrixTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMatrixTransform3D(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform3d, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTransform3DGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransform3DGroup(this, ::core::mem::transmute_copy(&transforms3d), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform3dgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEffectGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEffectGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(effectgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRectangleClip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clip: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRectangleClip(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAnimation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAnimation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(animation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDCompositionDevice2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Commit: Commit::<Identity, Impl, OFFSET>,
            WaitForCommitCompletion: WaitForCommitCompletion::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
            CreateVisual: CreateVisual::<Identity, Impl, OFFSET>,
            CreateSurfaceFactory: CreateSurfaceFactory::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, Impl, OFFSET>,
            CreateTranslateTransform: CreateTranslateTransform::<Identity, Impl, OFFSET>,
            CreateScaleTransform: CreateScaleTransform::<Identity, Impl, OFFSET>,
            CreateRotateTransform: CreateRotateTransform::<Identity, Impl, OFFSET>,
            CreateSkewTransform: CreateSkewTransform::<Identity, Impl, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, Impl, OFFSET>,
            CreateTransformGroup: CreateTransformGroup::<Identity, Impl, OFFSET>,
            CreateTranslateTransform3D: CreateTranslateTransform3D::<Identity, Impl, OFFSET>,
            CreateScaleTransform3D: CreateScaleTransform3D::<Identity, Impl, OFFSET>,
            CreateRotateTransform3D: CreateRotateTransform3D::<Identity, Impl, OFFSET>,
            CreateMatrixTransform3D: CreateMatrixTransform3D::<Identity, Impl, OFFSET>,
            CreateTransform3DGroup: CreateTransform3DGroup::<Identity, Impl, OFFSET>,
            CreateEffectGroup: CreateEffectGroup::<Identity, Impl, OFFSET>,
            CreateRectangleClip: CreateRectangleClip::<Identity, Impl, OFFSET>,
            CreateAnimation: CreateAnimation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionDevice3_Impl: ::windows_core::BaseImpl + IDCompositionDevice2_Impl {
    fn CreateGaussianBlurEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionGaussianBlurEffect>;
    fn CreateBrightnessEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionBrightnessEffect>;
    fn CreateColorMatrixEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionColorMatrixEffect>;
    fn CreateShadowEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionShadowEffect>;
    fn CreateHueRotationEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionHueRotationEffect>;
    fn CreateSaturationEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionSaturationEffect>;
    fn CreateTurbulenceEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionTurbulenceEffect>;
    fn CreateLinearTransferEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionLinearTransferEffect>;
    fn CreateTableTransferEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionTableTransferEffect>;
    fn CreateCompositeEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionCompositeEffect>;
    fn CreateBlendEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionBlendEffect>;
    fn CreateArithmeticCompositeEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionArithmeticCompositeEffect>;
    fn CreateAffineTransform2DEffect(this: &Self::This) -> ::windows_core::Result<IDCompositionAffineTransform2DEffect>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IDCompositionDevice3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionDevice2);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionDevice3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateGaussianBlurEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGaussianBlurEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gaussianblureffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBrightnessEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brightnesseffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBrightnessEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brightnesseffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorMatrixEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorMatrixEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colormatrixeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateShadowEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shadoweffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateShadowEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shadoweffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateHueRotationEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, huerotationeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateHueRotationEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(huerotationeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSaturationEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, saturationeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSaturationEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(saturationeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTurbulenceEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTurbulenceEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(turbulenceeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearTransferEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearTransferEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineartransfereffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTableTransferEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTableTransferEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tabletransfereffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCompositeEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCompositeEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compositeeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlendEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blendeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlendEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blendeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateArithmeticCompositeEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateArithmeticCompositeEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(arithmeticcompositeeffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAffineTransform2DEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAffineTransform2DEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(affinetransform2deffect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDCompositionDevice3_Vtbl {
            base__: <IDCompositionDevice2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateGaussianBlurEffect: CreateGaussianBlurEffect::<Identity, Impl, OFFSET>,
            CreateBrightnessEffect: CreateBrightnessEffect::<Identity, Impl, OFFSET>,
            CreateColorMatrixEffect: CreateColorMatrixEffect::<Identity, Impl, OFFSET>,
            CreateShadowEffect: CreateShadowEffect::<Identity, Impl, OFFSET>,
            CreateHueRotationEffect: CreateHueRotationEffect::<Identity, Impl, OFFSET>,
            CreateSaturationEffect: CreateSaturationEffect::<Identity, Impl, OFFSET>,
            CreateTurbulenceEffect: CreateTurbulenceEffect::<Identity, Impl, OFFSET>,
            CreateLinearTransferEffect: CreateLinearTransferEffect::<Identity, Impl, OFFSET>,
            CreateTableTransferEffect: CreateTableTransferEffect::<Identity, Impl, OFFSET>,
            CreateCompositeEffect: CreateCompositeEffect::<Identity, Impl, OFFSET>,
            CreateBlendEffect: CreateBlendEffect::<Identity, Impl, OFFSET>,
            CreateArithmeticCompositeEffect: CreateArithmeticCompositeEffect::<Identity, Impl, OFFSET>,
            CreateAffineTransform2DEffect: CreateAffineTransform2DEffect::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionDeviceDebug_Impl: ::windows_core::BaseImpl {
    fn EnableDebugCounters(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableDebugCounters(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionDeviceDebug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDeviceDebug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionDeviceDebug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableDebugCounters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDeviceDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableDebugCounters(this).into())
        }
        unsafe extern "system" fn DisableDebugCounters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionDeviceDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableDebugCounters(this).into())
        }
        IDCompositionDeviceDebug_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableDebugCounters: EnableDebugCounters::<Identity, Impl, OFFSET>,
            DisableDebugCounters: DisableDebugCounters::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionEffect_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IDCompositionEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionEffect {
    const VTABLE: Self::Vtable = { IDCompositionEffect_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IDCompositionEffectGroup_Impl: ::windows_core::BaseImpl + IDCompositionEffect_Impl {
    fn SetOpacity(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOpacity2(this: &Self::This, opacity: f32) -> ::windows_core::Result<()>;
    fn SetTransform3D(this: &Self::This, transform3d: ::core::option::Option<&IDCompositionTransform3D>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionEffectGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionEffect);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionEffectGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionEffectGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionEffectGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacity(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOpacity2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionEffectGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacity2(this, ::core::mem::transmute_copy(&opacity)).into())
        }
        unsafe extern "system" fn SetTransform3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionEffectGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform3d: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform3D(this, ::windows_core::from_raw_borrowed(&transform3d)).into())
        }
        IDCompositionEffectGroup_Vtbl {
            base__: <IDCompositionEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, Impl, OFFSET>,
            SetTransform3D: SetTransform3D::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionFilterEffect_Impl: ::windows_core::BaseImpl + IDCompositionEffect_Impl {
    fn SetInput(this: &Self::This, index: u32, input: ::core::option::Option<&::windows_core::IUnknown>, flags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionFilterEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionEffect);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionFilterEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionFilterEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionFilterEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInput(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&input), ::core::mem::transmute_copy(&flags)).into())
        }
        IDCompositionFilterEffect_Vtbl { base__: <IDCompositionEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetInput: SetInput::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionGaussianBlurEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetStandardDeviation2(this: &Self::This, amount: f32) -> ::windows_core::Result<()>;
    fn SetBorderMode(this: &Self::This, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for IDCompositionGaussianBlurEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionGaussianBlurEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetStandardDeviation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStandardDeviation(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStandardDeviation2(this, ::core::mem::transmute_copy(&amount)).into())
        }
        unsafe extern "system" fn SetBorderMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionGaussianBlurEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBorderMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        IDCompositionGaussianBlurEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetStandardDeviation: SetStandardDeviation::<Identity, Impl, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, Impl, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionHueRotationEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetAngle(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAngle2(this: &Self::This, amountdegrees: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionHueRotationEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionHueRotationEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAngle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngle(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAngle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionHueRotationEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngle2(this, ::core::mem::transmute_copy(&amountdegrees)).into())
        }
        IDCompositionHueRotationEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAngle: SetAngle::<Identity, Impl, OFFSET>,
            SetAngle2: SetAngle2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionInkTrailDevice_Impl: ::windows_core::BaseImpl {
    fn CreateDelegatedInkTrail(this: &Self::This) -> ::windows_core::Result<IDCompositionDelegatedInkTrail>;
    fn CreateDelegatedInkTrailForSwapChain(this: &Self::This, swapchain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDCompositionDelegatedInkTrail>;
}
impl ::windows_core::Iids for IDCompositionInkTrailDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionInkTrailDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDelegatedInkTrail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDelegatedInkTrail(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inktrail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDelegatedInkTrailForSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionInkTrailDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDelegatedInkTrailForSwapChain(this, ::windows_core::from_raw_borrowed(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inktrail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDCompositionInkTrailDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDelegatedInkTrail: CreateDelegatedInkTrail::<Identity, Impl, OFFSET>,
            CreateDelegatedInkTrailForSwapChain: CreateDelegatedInkTrailForSwapChain::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionLinearTransferEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetRedYIntercept(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetRedYIntercept2(this: &Self::This, redyintercept: f32) -> ::windows_core::Result<()>;
    fn SetRedSlope(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetRedSlope2(this: &Self::This, redslope: f32) -> ::windows_core::Result<()>;
    fn SetRedDisable(this: &Self::This, reddisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetGreenYIntercept(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetGreenYIntercept2(this: &Self::This, greenyintercept: f32) -> ::windows_core::Result<()>;
    fn SetGreenSlope(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetGreenSlope2(this: &Self::This, greenslope: f32) -> ::windows_core::Result<()>;
    fn SetGreenDisable(this: &Self::This, greendisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBlueYIntercept(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBlueYIntercept2(this: &Self::This, blueyintercept: f32) -> ::windows_core::Result<()>;
    fn SetBlueSlope(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBlueSlope2(this: &Self::This, blueslope: f32) -> ::windows_core::Result<()>;
    fn SetBlueDisable(this: &Self::This, bluedisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetAlphaYIntercept(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAlphaYIntercept2(this: &Self::This, alphayintercept: f32) -> ::windows_core::Result<()>;
    fn SetAlphaSlope(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAlphaSlope2(this: &Self::This, alphaslope: f32) -> ::windows_core::Result<()>;
    fn SetAlphaDisable(this: &Self::This, alphadisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetClampOutput(this: &Self::This, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDCompositionLinearTransferEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionLinearTransferEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRedYIntercept<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedYIntercept(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetRedYIntercept2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedYIntercept2(this, ::core::mem::transmute_copy(&redyintercept)).into())
        }
        unsafe extern "system" fn SetRedSlope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedSlope(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetRedSlope2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedSlope2(this, ::core::mem::transmute_copy(&redslope)).into())
        }
        unsafe extern "system" fn SetRedDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedDisable(this, ::core::mem::transmute_copy(&reddisable)).into())
        }
        unsafe extern "system" fn SetGreenYIntercept<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenYIntercept(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetGreenYIntercept2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenYIntercept2(this, ::core::mem::transmute_copy(&greenyintercept)).into())
        }
        unsafe extern "system" fn SetGreenSlope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenSlope(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetGreenSlope2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenSlope2(this, ::core::mem::transmute_copy(&greenslope)).into())
        }
        unsafe extern "system" fn SetGreenDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenDisable(this, ::core::mem::transmute_copy(&greendisable)).into())
        }
        unsafe extern "system" fn SetBlueYIntercept<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueYIntercept(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBlueYIntercept2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueYIntercept2(this, ::core::mem::transmute_copy(&blueyintercept)).into())
        }
        unsafe extern "system" fn SetBlueSlope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueSlope(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBlueSlope2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueSlope2(this, ::core::mem::transmute_copy(&blueslope)).into())
        }
        unsafe extern "system" fn SetBlueDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueDisable(this, ::core::mem::transmute_copy(&bluedisable)).into())
        }
        unsafe extern "system" fn SetAlphaYIntercept<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaYIntercept(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAlphaYIntercept2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaYIntercept2(this, ::core::mem::transmute_copy(&alphayintercept)).into())
        }
        unsafe extern "system" fn SetAlphaSlope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaSlope(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAlphaSlope2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaSlope2(this, ::core::mem::transmute_copy(&alphaslope)).into())
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaDisable(this, ::core::mem::transmute_copy(&alphadisable)).into())
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionLinearTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClampOutput(this, ::core::mem::transmute_copy(&clampoutput)).into())
        }
        IDCompositionLinearTransferEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRedYIntercept: SetRedYIntercept::<Identity, Impl, OFFSET>,
            SetRedYIntercept2: SetRedYIntercept2::<Identity, Impl, OFFSET>,
            SetRedSlope: SetRedSlope::<Identity, Impl, OFFSET>,
            SetRedSlope2: SetRedSlope2::<Identity, Impl, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, Impl, OFFSET>,
            SetGreenYIntercept: SetGreenYIntercept::<Identity, Impl, OFFSET>,
            SetGreenYIntercept2: SetGreenYIntercept2::<Identity, Impl, OFFSET>,
            SetGreenSlope: SetGreenSlope::<Identity, Impl, OFFSET>,
            SetGreenSlope2: SetGreenSlope2::<Identity, Impl, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, Impl, OFFSET>,
            SetBlueYIntercept: SetBlueYIntercept::<Identity, Impl, OFFSET>,
            SetBlueYIntercept2: SetBlueYIntercept2::<Identity, Impl, OFFSET>,
            SetBlueSlope: SetBlueSlope::<Identity, Impl, OFFSET>,
            SetBlueSlope2: SetBlueSlope2::<Identity, Impl, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, Impl, OFFSET>,
            SetAlphaYIntercept: SetAlphaYIntercept::<Identity, Impl, OFFSET>,
            SetAlphaYIntercept2: SetAlphaYIntercept2::<Identity, Impl, OFFSET>,
            SetAlphaSlope: SetAlphaSlope::<Identity, Impl, OFFSET>,
            SetAlphaSlope2: SetAlphaSlope2::<Identity, Impl, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`"]
#[cfg(feature = "Foundation_Numerics")]
pub trait IDCompositionMatrixTransform_Impl: ::windows_core::BaseImpl + IDCompositionTransform_Impl {
    fn SetMatrix(this: &Self::This, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()>;
    fn SetMatrixElement(this: &Self::This, row: i32, column: i32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetMatrixElement2(this: &Self::This, row: i32, column: i32, value: f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::Iids for IDCompositionMatrixTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform);
}
#[cfg(feature = "Foundation_Numerics")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionMatrixTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMatrix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrix(this, ::core::mem::transmute_copy(&matrix)).into())
        }
        unsafe extern "system" fn SetMatrixElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrixElement(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrixElement2(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into())
        }
        IDCompositionMatrixTransform_Vtbl {
            base__: <IDCompositionTransform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, Impl, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`"]
#[cfg(feature = "Foundation_Numerics")]
pub trait IDCompositionMatrixTransform3D_Impl: ::windows_core::BaseImpl + IDCompositionTransform3D_Impl {
    fn SetMatrix(this: &Self::This, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::Result<()>;
    fn SetMatrixElement(this: &Self::This, row: i32, column: i32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetMatrixElement2(this: &Self::This, row: i32, column: i32, value: f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::Iids for IDCompositionMatrixTransform3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform3D);
}
#[cfg(feature = "Foundation_Numerics")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionMatrixTransform3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMatrix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrix(this, ::core::mem::transmute_copy(&matrix)).into())
        }
        unsafe extern "system" fn SetMatrixElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrixElement(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetMatrixElement2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionMatrixTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrixElement2(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column), ::core::mem::transmute_copy(&value)).into())
        }
        IDCompositionMatrixTransform3D_Vtbl {
            base__: <IDCompositionTransform3D as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            SetMatrixElement: SetMatrixElement::<Identity, Impl, OFFSET>,
            SetMatrixElement2: SetMatrixElement2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionRectangleClip_Impl: ::windows_core::BaseImpl + IDCompositionClip_Impl {
    fn SetLeft(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetLeft2(this: &Self::This, left: f32) -> ::windows_core::Result<()>;
    fn SetTop(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetTop2(this: &Self::This, top: f32) -> ::windows_core::Result<()>;
    fn SetRight(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetRight2(this: &Self::This, right: f32) -> ::windows_core::Result<()>;
    fn SetBottom(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBottom2(this: &Self::This, bottom: f32) -> ::windows_core::Result<()>;
    fn SetTopLeftRadiusX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetTopLeftRadiusX2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn SetTopLeftRadiusY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetTopLeftRadiusY2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn SetTopRightRadiusX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetTopRightRadiusX2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn SetTopRightRadiusY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetTopRightRadiusY2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn SetBottomLeftRadiusX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBottomLeftRadiusX2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn SetBottomLeftRadiusY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBottomLeftRadiusY2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn SetBottomRightRadiusX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBottomRightRadiusX2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
    fn SetBottomRightRadiusY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBottomRightRadiusY2(this: &Self::This, radius: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionRectangleClip {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionClip);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionRectangleClip {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLeft(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetLeft2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLeft2(this, ::core::mem::transmute_copy(&left)).into())
        }
        unsafe extern "system" fn SetTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTop(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetTop2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, top: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTop2(this, ::core::mem::transmute_copy(&top)).into())
        }
        unsafe extern "system" fn SetRight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRight(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetRight2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, right: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRight2(this, ::core::mem::transmute_copy(&right)).into())
        }
        unsafe extern "system" fn SetBottom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottom(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBottom2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottom2(this, ::core::mem::transmute_copy(&bottom)).into())
        }
        unsafe extern "system" fn SetTopLeftRadiusX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopLeftRadiusX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetTopLeftRadiusX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopLeftRadiusX2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn SetTopLeftRadiusY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopLeftRadiusY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetTopLeftRadiusY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopLeftRadiusY2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn SetTopRightRadiusX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopRightRadiusX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetTopRightRadiusX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopRightRadiusX2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn SetTopRightRadiusY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopRightRadiusY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetTopRightRadiusY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTopRightRadiusY2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn SetBottomLeftRadiusX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomLeftRadiusX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBottomLeftRadiusX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomLeftRadiusX2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn SetBottomLeftRadiusY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomLeftRadiusY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBottomLeftRadiusY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomLeftRadiusY2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn SetBottomRightRadiusX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomRightRadiusX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBottomRightRadiusX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomRightRadiusX2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        unsafe extern "system" fn SetBottomRightRadiusY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomRightRadiusY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBottomRightRadiusY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRectangleClip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottomRightRadiusY2(this, ::core::mem::transmute_copy(&radius)).into())
        }
        IDCompositionRectangleClip_Vtbl {
            base__: <IDCompositionClip as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLeft: SetLeft::<Identity, Impl, OFFSET>,
            SetLeft2: SetLeft2::<Identity, Impl, OFFSET>,
            SetTop: SetTop::<Identity, Impl, OFFSET>,
            SetTop2: SetTop2::<Identity, Impl, OFFSET>,
            SetRight: SetRight::<Identity, Impl, OFFSET>,
            SetRight2: SetRight2::<Identity, Impl, OFFSET>,
            SetBottom: SetBottom::<Identity, Impl, OFFSET>,
            SetBottom2: SetBottom2::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusX: SetTopLeftRadiusX::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusX2: SetTopLeftRadiusX2::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusY: SetTopLeftRadiusY::<Identity, Impl, OFFSET>,
            SetTopLeftRadiusY2: SetTopLeftRadiusY2::<Identity, Impl, OFFSET>,
            SetTopRightRadiusX: SetTopRightRadiusX::<Identity, Impl, OFFSET>,
            SetTopRightRadiusX2: SetTopRightRadiusX2::<Identity, Impl, OFFSET>,
            SetTopRightRadiusY: SetTopRightRadiusY::<Identity, Impl, OFFSET>,
            SetTopRightRadiusY2: SetTopRightRadiusY2::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusX: SetBottomLeftRadiusX::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusX2: SetBottomLeftRadiusX2::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusY: SetBottomLeftRadiusY::<Identity, Impl, OFFSET>,
            SetBottomLeftRadiusY2: SetBottomLeftRadiusY2::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusX: SetBottomRightRadiusX::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusX2: SetBottomRightRadiusX2::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusY: SetBottomRightRadiusY::<Identity, Impl, OFFSET>,
            SetBottomRightRadiusY2: SetBottomRightRadiusY2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionRotateTransform_Impl: ::windows_core::BaseImpl + IDCompositionTransform_Impl {
    fn SetAngle(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAngle2(this: &Self::This, angle: f32) -> ::windows_core::Result<()>;
    fn SetCenterX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterX2(this: &Self::This, centerx: f32) -> ::windows_core::Result<()>;
    fn SetCenterY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterY2(this: &Self::This, centery: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionRotateTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionRotateTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAngle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngle(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAngle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngle2(this, ::core::mem::transmute_copy(&angle)).into())
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX2(this, ::core::mem::transmute_copy(&centerx)).into())
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY2(this, ::core::mem::transmute_copy(&centery)).into())
        }
        IDCompositionRotateTransform_Vtbl {
            base__: <IDCompositionTransform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAngle: SetAngle::<Identity, Impl, OFFSET>,
            SetAngle2: SetAngle2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionRotateTransform3D_Impl: ::windows_core::BaseImpl + IDCompositionTransform3D_Impl {
    fn SetAngle(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAngle2(this: &Self::This, angle: f32) -> ::windows_core::Result<()>;
    fn SetAxisX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAxisX2(this: &Self::This, axisx: f32) -> ::windows_core::Result<()>;
    fn SetAxisY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAxisY2(this: &Self::This, axisy: f32) -> ::windows_core::Result<()>;
    fn SetAxisZ(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAxisZ2(this: &Self::This, axisz: f32) -> ::windows_core::Result<()>;
    fn SetCenterX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterX2(this: &Self::This, centerx: f32) -> ::windows_core::Result<()>;
    fn SetCenterY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterY2(this: &Self::This, centery: f32) -> ::windows_core::Result<()>;
    fn SetCenterZ(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterZ2(this: &Self::This, centerz: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionRotateTransform3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform3D);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionRotateTransform3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAngle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngle(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAngle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngle2(this, ::core::mem::transmute_copy(&angle)).into())
        }
        unsafe extern "system" fn SetAxisX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAxisX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAxisX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAxisX2(this, ::core::mem::transmute_copy(&axisx)).into())
        }
        unsafe extern "system" fn SetAxisY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAxisY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAxisY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAxisY2(this, ::core::mem::transmute_copy(&axisy)).into())
        }
        unsafe extern "system" fn SetAxisZ<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAxisZ(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAxisZ2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAxisZ2(this, ::core::mem::transmute_copy(&axisz)).into())
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX2(this, ::core::mem::transmute_copy(&centerx)).into())
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY2(this, ::core::mem::transmute_copy(&centery)).into())
        }
        unsafe extern "system" fn SetCenterZ<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterZ(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterZ2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionRotateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterZ2(this, ::core::mem::transmute_copy(&centerz)).into())
        }
        IDCompositionRotateTransform3D_Vtbl {
            base__: <IDCompositionTransform3D as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAngle: SetAngle::<Identity, Impl, OFFSET>,
            SetAngle2: SetAngle2::<Identity, Impl, OFFSET>,
            SetAxisX: SetAxisX::<Identity, Impl, OFFSET>,
            SetAxisX2: SetAxisX2::<Identity, Impl, OFFSET>,
            SetAxisY: SetAxisY::<Identity, Impl, OFFSET>,
            SetAxisY2: SetAxisY2::<Identity, Impl, OFFSET>,
            SetAxisZ: SetAxisZ::<Identity, Impl, OFFSET>,
            SetAxisZ2: SetAxisZ2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, Impl, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionSaturationEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetSaturation(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetSaturation2(this: &Self::This, ratio: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionSaturationEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSaturationEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionSaturationEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSaturation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSaturationEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSaturation(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetSaturation2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSaturationEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSaturation2(this, ::core::mem::transmute_copy(&ratio)).into())
        }
        IDCompositionSaturationEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSaturation: SetSaturation::<Identity, Impl, OFFSET>,
            SetSaturation2: SetSaturation2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionScaleTransform_Impl: ::windows_core::BaseImpl + IDCompositionTransform_Impl {
    fn SetScaleX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetScaleX2(this: &Self::This, scalex: f32) -> ::windows_core::Result<()>;
    fn SetScaleY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetScaleY2(this: &Self::This, scaley: f32) -> ::windows_core::Result<()>;
    fn SetCenterX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterX2(this: &Self::This, centerx: f32) -> ::windows_core::Result<()>;
    fn SetCenterY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterY2(this: &Self::This, centery: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionScaleTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionScaleTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetScaleX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetScaleX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleX2(this, ::core::mem::transmute_copy(&scalex)).into())
        }
        unsafe extern "system" fn SetScaleY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetScaleY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleY2(this, ::core::mem::transmute_copy(&scaley)).into())
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX2(this, ::core::mem::transmute_copy(&centerx)).into())
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY2(this, ::core::mem::transmute_copy(&centery)).into())
        }
        IDCompositionScaleTransform_Vtbl {
            base__: <IDCompositionTransform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetScaleX: SetScaleX::<Identity, Impl, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, Impl, OFFSET>,
            SetScaleY: SetScaleY::<Identity, Impl, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionScaleTransform3D_Impl: ::windows_core::BaseImpl + IDCompositionTransform3D_Impl {
    fn SetScaleX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetScaleX2(this: &Self::This, scalex: f32) -> ::windows_core::Result<()>;
    fn SetScaleY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetScaleY2(this: &Self::This, scaley: f32) -> ::windows_core::Result<()>;
    fn SetScaleZ(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetScaleZ2(this: &Self::This, scalez: f32) -> ::windows_core::Result<()>;
    fn SetCenterX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterX2(this: &Self::This, centerx: f32) -> ::windows_core::Result<()>;
    fn SetCenterY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterY2(this: &Self::This, centery: f32) -> ::windows_core::Result<()>;
    fn SetCenterZ(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterZ2(this: &Self::This, centerz: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionScaleTransform3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform3D);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionScaleTransform3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetScaleX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetScaleX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleX2(this, ::core::mem::transmute_copy(&scalex)).into())
        }
        unsafe extern "system" fn SetScaleY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetScaleY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleY2(this, ::core::mem::transmute_copy(&scaley)).into())
        }
        unsafe extern "system" fn SetScaleZ<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleZ(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetScaleZ2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScaleZ2(this, ::core::mem::transmute_copy(&scalez)).into())
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX2(this, ::core::mem::transmute_copy(&centerx)).into())
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY2(this, ::core::mem::transmute_copy(&centery)).into())
        }
        unsafe extern "system" fn SetCenterZ<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterZ(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterZ2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionScaleTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterZ2(this, ::core::mem::transmute_copy(&centerz)).into())
        }
        IDCompositionScaleTransform3D_Vtbl {
            base__: <IDCompositionTransform3D as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetScaleX: SetScaleX::<Identity, Impl, OFFSET>,
            SetScaleX2: SetScaleX2::<Identity, Impl, OFFSET>,
            SetScaleY: SetScaleY::<Identity, Impl, OFFSET>,
            SetScaleY2: SetScaleY2::<Identity, Impl, OFFSET>,
            SetScaleZ: SetScaleZ::<Identity, Impl, OFFSET>,
            SetScaleZ2: SetScaleZ2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
            SetCenterZ: SetCenterZ::<Identity, Impl, OFFSET>,
            SetCenterZ2: SetCenterZ2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait IDCompositionShadowEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetStandardDeviation(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetStandardDeviation2(this: &Self::This, amount: f32) -> ::windows_core::Result<()>;
    fn SetColor(this: &Self::This, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::Result<()>;
    fn SetRed(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetRed2(this: &Self::This, amount: f32) -> ::windows_core::Result<()>;
    fn SetGreen(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetGreen2(this: &Self::This, amount: f32) -> ::windows_core::Result<()>;
    fn SetBlue(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBlue2(this: &Self::This, amount: f32) -> ::windows_core::Result<()>;
    fn SetAlpha(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAlpha2(this: &Self::This, amount: f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for IDCompositionShadowEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionShadowEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetStandardDeviation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStandardDeviation(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetStandardDeviation2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStandardDeviation2(this, ::core::mem::transmute_copy(&amount)).into())
        }
        unsafe extern "system" fn SetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColor(this, ::core::mem::transmute_copy(&color)).into())
        }
        unsafe extern "system" fn SetRed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRed(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetRed2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRed2(this, ::core::mem::transmute_copy(&amount)).into())
        }
        unsafe extern "system" fn SetGreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreen(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetGreen2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreen2(this, ::core::mem::transmute_copy(&amount)).into())
        }
        unsafe extern "system" fn SetBlue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlue(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBlue2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlue2(this, ::core::mem::transmute_copy(&amount)).into())
        }
        unsafe extern "system" fn SetAlpha<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlpha(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAlpha2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionShadowEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlpha2(this, ::core::mem::transmute_copy(&amount)).into())
        }
        IDCompositionShadowEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetStandardDeviation: SetStandardDeviation::<Identity, Impl, OFFSET>,
            SetStandardDeviation2: SetStandardDeviation2::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            SetRed: SetRed::<Identity, Impl, OFFSET>,
            SetRed2: SetRed2::<Identity, Impl, OFFSET>,
            SetGreen: SetGreen::<Identity, Impl, OFFSET>,
            SetGreen2: SetGreen2::<Identity, Impl, OFFSET>,
            SetBlue: SetBlue::<Identity, Impl, OFFSET>,
            SetBlue2: SetBlue2::<Identity, Impl, OFFSET>,
            SetAlpha: SetAlpha::<Identity, Impl, OFFSET>,
            SetAlpha2: SetAlpha2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionSkewTransform_Impl: ::windows_core::BaseImpl + IDCompositionTransform_Impl {
    fn SetAngleX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAngleX2(this: &Self::This, anglex: f32) -> ::windows_core::Result<()>;
    fn SetAngleY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAngleY2(this: &Self::This, angley: f32) -> ::windows_core::Result<()>;
    fn SetCenterX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterX2(this: &Self::This, centerx: f32) -> ::windows_core::Result<()>;
    fn SetCenterY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetCenterY2(this: &Self::This, centery: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionSkewTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionSkewTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAngleX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngleX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAngleX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngleX2(this, ::core::mem::transmute_copy(&anglex)).into())
        }
        unsafe extern "system" fn SetAngleY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngleY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAngleY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAngleY2(this, ::core::mem::transmute_copy(&angley)).into())
        }
        unsafe extern "system" fn SetCenterX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterX2(this, ::core::mem::transmute_copy(&centerx)).into())
        }
        unsafe extern "system" fn SetCenterY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetCenterY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSkewTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenterY2(this, ::core::mem::transmute_copy(&centery)).into())
        }
        IDCompositionSkewTransform_Vtbl {
            base__: <IDCompositionTransform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAngleX: SetAngleX::<Identity, Impl, OFFSET>,
            SetAngleX2: SetAngleX2::<Identity, Impl, OFFSET>,
            SetAngleY: SetAngleY::<Identity, Impl, OFFSET>,
            SetAngleY2: SetAngleY2::<Identity, Impl, OFFSET>,
            SetCenterX: SetCenterX::<Identity, Impl, OFFSET>,
            SetCenterX2: SetCenterX2::<Identity, Impl, OFFSET>,
            SetCenterY: SetCenterY::<Identity, Impl, OFFSET>,
            SetCenterY2: SetCenterY2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionSurface_Impl: ::windows_core::BaseImpl {
    fn BeginDraw(this: &Self::This, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn EndDraw(this: &Self::This) -> ::windows_core::Result<()>;
    fn SuspendDraw(this: &Self::This) -> ::windows_core::Result<()>;
    fn ResumeDraw(this: &Self::This) -> ::windows_core::Result<()>;
    fn Scroll(this: &Self::This, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDCompositionSurface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionSurface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDraw(this, ::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)).into())
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDraw(this).into())
        }
        unsafe extern "system" fn SuspendDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspendDraw(this).into())
        }
        unsafe extern "system" fn ResumeDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeDraw(this).into())
        }
        unsafe extern "system" fn Scroll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Scroll(this, ::core::mem::transmute_copy(&scrollrect), ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&offsetx), ::core::mem::transmute_copy(&offsety)).into())
        }
        IDCompositionSurface_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDCompositionSurfaceFactory_Impl: ::windows_core::BaseImpl {
    fn CreateSurface(this: &Self::This, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface>;
    fn CreateVirtualSurface(this: &Self::This, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IDCompositionSurfaceFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionSurfaceFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurface(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(surface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVirtualSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionSurfaceFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVirtualSurface(this, ::core::mem::transmute_copy(&initialwidth), ::core::mem::transmute_copy(&initialheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(virtualsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDCompositionSurfaceFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            CreateVirtualSurface: CreateVirtualSurface::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionTableTransferEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetRedTable(this: &Self::This, tablevalues: *const f32, count: u32) -> ::windows_core::Result<()>;
    fn SetGreenTable(this: &Self::This, tablevalues: *const f32, count: u32) -> ::windows_core::Result<()>;
    fn SetBlueTable(this: &Self::This, tablevalues: *const f32, count: u32) -> ::windows_core::Result<()>;
    fn SetAlphaTable(this: &Self::This, tablevalues: *const f32, count: u32) -> ::windows_core::Result<()>;
    fn SetRedDisable(this: &Self::This, reddisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetGreenDisable(this: &Self::This, greendisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBlueDisable(this: &Self::This, bluedisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetAlphaDisable(this: &Self::This, alphadisable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetClampOutput(this: &Self::This, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetRedTableValue(this: &Self::This, index: u32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetRedTableValue2(this: &Self::This, index: u32, value: f32) -> ::windows_core::Result<()>;
    fn SetGreenTableValue(this: &Self::This, index: u32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetGreenTableValue2(this: &Self::This, index: u32, value: f32) -> ::windows_core::Result<()>;
    fn SetBlueTableValue(this: &Self::This, index: u32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetBlueTableValue2(this: &Self::This, index: u32, value: f32) -> ::windows_core::Result<()>;
    fn SetAlphaTableValue(this: &Self::This, index: u32, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetAlphaTableValue2(this: &Self::This, index: u32, value: f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDCompositionTableTransferEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionTableTransferEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRedTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedTable(this, ::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn SetGreenTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenTable(this, ::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn SetBlueTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueTable(this, ::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn SetAlphaTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaTable(this, ::core::mem::transmute_copy(&tablevalues), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn SetRedDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedDisable(this, ::core::mem::transmute_copy(&reddisable)).into())
        }
        unsafe extern "system" fn SetGreenDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenDisable(this, ::core::mem::transmute_copy(&greendisable)).into())
        }
        unsafe extern "system" fn SetBlueDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueDisable(this, ::core::mem::transmute_copy(&bluedisable)).into())
        }
        unsafe extern "system" fn SetAlphaDisable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaDisable(this, ::core::mem::transmute_copy(&alphadisable)).into())
        }
        unsafe extern "system" fn SetClampOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClampOutput(this, ::core::mem::transmute_copy(&clampoutput)).into())
        }
        unsafe extern "system" fn SetRedTableValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedTableValue(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetRedTableValue2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRedTableValue2(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetGreenTableValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenTableValue(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetGreenTableValue2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGreenTableValue2(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetBlueTableValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueTableValue(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetBlueTableValue2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlueTableValue2(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn SetAlphaTableValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaTableValue(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetAlphaTableValue2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTableTransferEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlphaTableValue2(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into())
        }
        IDCompositionTableTransferEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRedTable: SetRedTable::<Identity, Impl, OFFSET>,
            SetGreenTable: SetGreenTable::<Identity, Impl, OFFSET>,
            SetBlueTable: SetBlueTable::<Identity, Impl, OFFSET>,
            SetAlphaTable: SetAlphaTable::<Identity, Impl, OFFSET>,
            SetRedDisable: SetRedDisable::<Identity, Impl, OFFSET>,
            SetGreenDisable: SetGreenDisable::<Identity, Impl, OFFSET>,
            SetBlueDisable: SetBlueDisable::<Identity, Impl, OFFSET>,
            SetAlphaDisable: SetAlphaDisable::<Identity, Impl, OFFSET>,
            SetClampOutput: SetClampOutput::<Identity, Impl, OFFSET>,
            SetRedTableValue: SetRedTableValue::<Identity, Impl, OFFSET>,
            SetRedTableValue2: SetRedTableValue2::<Identity, Impl, OFFSET>,
            SetGreenTableValue: SetGreenTableValue::<Identity, Impl, OFFSET>,
            SetGreenTableValue2: SetGreenTableValue2::<Identity, Impl, OFFSET>,
            SetBlueTableValue: SetBlueTableValue::<Identity, Impl, OFFSET>,
            SetBlueTableValue2: SetBlueTableValue2::<Identity, Impl, OFFSET>,
            SetAlphaTableValue: SetAlphaTableValue::<Identity, Impl, OFFSET>,
            SetAlphaTableValue2: SetAlphaTableValue2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionTarget_Impl: ::windows_core::BaseImpl {
    fn SetRoot(this: &Self::This, visual: ::core::option::Option<&IDCompositionVisual>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRoot(this, ::windows_core::from_raw_borrowed(&visual)).into())
        }
        IDCompositionTarget_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetRoot: SetRoot::<Identity, Impl, OFFSET> }
    };
}
pub trait IDCompositionTransform_Impl: ::windows_core::BaseImpl + IDCompositionTransform3D_Impl {}
impl ::windows_core::Iids for IDCompositionTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform3D);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionTransform {
    const VTABLE: Self::Vtable = { IDCompositionTransform_Vtbl { base__: <IDCompositionTransform3D as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IDCompositionTransform3D_Impl: ::windows_core::BaseImpl + IDCompositionEffect_Impl {}
impl ::windows_core::Iids for IDCompositionTransform3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionEffect);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTransform3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionTransform3D {
    const VTABLE: Self::Vtable = { IDCompositionTransform3D_Vtbl { base__: <IDCompositionEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IDCompositionTranslateTransform_Impl: ::windows_core::BaseImpl + IDCompositionTransform_Impl {
    fn SetOffsetX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetX2(this: &Self::This, offsetx: f32) -> ::windows_core::Result<()>;
    fn SetOffsetY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetY2(this: &Self::This, offsety: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionTranslateTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionTranslateTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOffsetX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetX2(this, ::core::mem::transmute_copy(&offsetx)).into())
        }
        unsafe extern "system" fn SetOffsetY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetY2(this, ::core::mem::transmute_copy(&offsety)).into())
        }
        IDCompositionTranslateTransform_Vtbl {
            base__: <IDCompositionTransform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOffsetX: SetOffsetX::<Identity, Impl, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, Impl, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, Impl, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDCompositionTranslateTransform3D_Impl: ::windows_core::BaseImpl + IDCompositionTransform3D_Impl {
    fn SetOffsetX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetX2(this: &Self::This, offsetx: f32) -> ::windows_core::Result<()>;
    fn SetOffsetY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetY2(this: &Self::This, offsety: f32) -> ::windows_core::Result<()>;
    fn SetOffsetZ(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetZ2(this: &Self::This, offsetz: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDCompositionTranslateTransform3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionTransform3D);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionTranslateTransform3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOffsetX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetX2(this, ::core::mem::transmute_copy(&offsetx)).into())
        }
        unsafe extern "system" fn SetOffsetY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetY2(this, ::core::mem::transmute_copy(&offsety)).into())
        }
        unsafe extern "system" fn SetOffsetZ<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetZ(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTranslateTransform3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetZ2(this, ::core::mem::transmute_copy(&offsetz)).into())
        }
        IDCompositionTranslateTransform3D_Vtbl {
            base__: <IDCompositionTransform3D as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOffsetX: SetOffsetX::<Identity, Impl, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, Impl, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, Impl, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, Impl, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, Impl, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionTurbulenceEffect_Impl: ::windows_core::BaseImpl + IDCompositionFilterEffect_Impl {
    fn SetOffset(this: &Self::This, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()>;
    fn SetBaseFrequency(this: &Self::This, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()>;
    fn SetSize(this: &Self::This, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()>;
    fn SetNumOctaves(this: &Self::This, numoctaves: u32) -> ::windows_core::Result<()>;
    fn SetSeed(this: &Self::This, seed: u32) -> ::windows_core::Result<()>;
    fn SetNoise(this: &Self::This, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows_core::Result<()>;
    fn SetStitchable(this: &Self::This, stitchable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionTurbulenceEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionFilterEffect);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionTurbulenceEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffset(this, ::core::mem::transmute_copy(&offset)).into())
        }
        unsafe extern "system" fn SetBaseFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBaseFrequency(this, ::core::mem::transmute_copy(&frequency)).into())
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, ::core::mem::transmute_copy(&size)).into())
        }
        unsafe extern "system" fn SetNumOctaves<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumOctaves(this, ::core::mem::transmute_copy(&numoctaves)).into())
        }
        unsafe extern "system" fn SetSeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSeed(this, ::core::mem::transmute_copy(&seed)).into())
        }
        unsafe extern "system" fn SetNoise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNoise(this, ::core::mem::transmute_copy(&noise)).into())
        }
        unsafe extern "system" fn SetStitchable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionTurbulenceEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStitchable(this, ::core::mem::transmute_copy(&stitchable)).into())
        }
        IDCompositionTurbulenceEffect_Vtbl {
            base__: <IDCompositionFilterEffect as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOffset: SetOffset::<Identity, Impl, OFFSET>,
            SetBaseFrequency: SetBaseFrequency::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            SetNumOctaves: SetNumOctaves::<Identity, Impl, OFFSET>,
            SetSeed: SetSeed::<Identity, Impl, OFFSET>,
            SetNoise: SetNoise::<Identity, Impl, OFFSET>,
            SetStitchable: SetStitchable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDCompositionVirtualSurface_Impl: ::windows_core::BaseImpl + IDCompositionSurface_Impl {
    fn Resize(this: &Self::This, width: u32, height: u32) -> ::windows_core::Result<()>;
    fn Trim(this: &Self::This, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDCompositionVirtualSurface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionSurface);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVirtualSurface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionVirtualSurface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Resize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVirtualSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn Trim<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVirtualSurface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Trim(this, ::core::mem::transmute_copy(&rectangles), ::core::mem::transmute_copy(&count)).into())
        }
        IDCompositionVirtualSurface_Vtbl {
            base__: <IDCompositionSurface as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Trim: Trim::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual_Impl: ::windows_core::BaseImpl {
    fn SetOffsetX(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetX2(this: &Self::This, offsetx: f32) -> ::windows_core::Result<()>;
    fn SetOffsetY(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetY2(this: &Self::This, offsety: f32) -> ::windows_core::Result<()>;
    fn SetTransform(this: &Self::This, transform: ::core::option::Option<&IDCompositionTransform>) -> ::windows_core::Result<()>;
    fn SetTransform2(this: &Self::This, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()>;
    fn SetTransformParent(this: &Self::This, visual: ::core::option::Option<&IDCompositionVisual>) -> ::windows_core::Result<()>;
    fn SetEffect(this: &Self::This, effect: ::core::option::Option<&IDCompositionEffect>) -> ::windows_core::Result<()>;
    fn SetBitmapInterpolationMode(this: &Self::This, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()>;
    fn SetBorderMode(this: &Self::This, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()>;
    fn SetClip(this: &Self::This, clip: ::core::option::Option<&IDCompositionClip>) -> ::windows_core::Result<()>;
    fn SetClip2(this: &Self::This, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()>;
    fn SetContent(this: &Self::This, content: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn AddVisual(this: &Self::This, visual: ::core::option::Option<&IDCompositionVisual>, insertabove: super::super::Foundation::BOOL, referencevisual: ::core::option::Option<&IDCompositionVisual>) -> ::windows_core::Result<()>;
    fn RemoveVisual(this: &Self::This, visual: ::core::option::Option<&IDCompositionVisual>) -> ::windows_core::Result<()>;
    fn RemoveAllVisuals(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCompositeMode(this: &Self::This, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionVisual {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionVisual {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOffsetX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetX(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetX2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetX2(this, ::core::mem::transmute_copy(&offsetx)).into())
        }
        unsafe extern "system" fn SetOffsetY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetY(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetY2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetY2(this, ::core::mem::transmute_copy(&offsety)).into())
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform(this, ::windows_core::from_raw_borrowed(&transform)).into())
        }
        unsafe extern "system" fn SetTransform2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform2(this, ::core::mem::transmute_copy(&matrix)).into())
        }
        unsafe extern "system" fn SetTransformParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformParent(this, ::windows_core::from_raw_borrowed(&visual)).into())
        }
        unsafe extern "system" fn SetEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEffect(this, ::windows_core::from_raw_borrowed(&effect)).into())
        }
        unsafe extern "system" fn SetBitmapInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitmapInterpolationMode(this, ::core::mem::transmute_copy(&interpolationmode)).into())
        }
        unsafe extern "system" fn SetBorderMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBorderMode(this, ::core::mem::transmute_copy(&bordermode)).into())
        }
        unsafe extern "system" fn SetClip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clip: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClip(this, ::windows_core::from_raw_borrowed(&clip)).into())
        }
        unsafe extern "system" fn SetClip2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClip2(this, ::core::mem::transmute_copy(&rect)).into())
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&content)).into())
        }
        unsafe extern "system" fn AddVisual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void, insertabove: super::super::Foundation::BOOL, referencevisual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddVisual(this, ::windows_core::from_raw_borrowed(&visual), ::core::mem::transmute_copy(&insertabove), ::windows_core::from_raw_borrowed(&referencevisual)).into())
        }
        unsafe extern "system" fn RemoveVisual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveVisual(this, ::windows_core::from_raw_borrowed(&visual)).into())
        }
        unsafe extern "system" fn RemoveAllVisuals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllVisuals(this).into())
        }
        unsafe extern "system" fn SetCompositeMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositeMode(this, ::core::mem::transmute_copy(&compositemode)).into())
        }
        IDCompositionVisual_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOffsetX: SetOffsetX::<Identity, Impl, OFFSET>,
            SetOffsetX2: SetOffsetX2::<Identity, Impl, OFFSET>,
            SetOffsetY: SetOffsetY::<Identity, Impl, OFFSET>,
            SetOffsetY2: SetOffsetY2::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            SetTransform2: SetTransform2::<Identity, Impl, OFFSET>,
            SetTransformParent: SetTransformParent::<Identity, Impl, OFFSET>,
            SetEffect: SetEffect::<Identity, Impl, OFFSET>,
            SetBitmapInterpolationMode: SetBitmapInterpolationMode::<Identity, Impl, OFFSET>,
            SetBorderMode: SetBorderMode::<Identity, Impl, OFFSET>,
            SetClip: SetClip::<Identity, Impl, OFFSET>,
            SetClip2: SetClip2::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            AddVisual: AddVisual::<Identity, Impl, OFFSET>,
            RemoveVisual: RemoveVisual::<Identity, Impl, OFFSET>,
            RemoveAllVisuals: RemoveAllVisuals::<Identity, Impl, OFFSET>,
            SetCompositeMode: SetCompositeMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual2_Impl: ::windows_core::BaseImpl + IDCompositionVisual_Impl {
    fn SetOpacityMode(this: &Self::This, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::Result<()>;
    fn SetBackFaceVisibility(this: &Self::This, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionVisual2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionVisual);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionVisual2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOpacityMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacityMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn SetBackFaceVisibility<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackFaceVisibility(this, ::core::mem::transmute_copy(&visibility)).into())
        }
        IDCompositionVisual2_Vtbl {
            base__: <IDCompositionVisual as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOpacityMode: SetOpacityMode::<Identity, Impl, OFFSET>,
            SetBackFaceVisibility: SetBackFaceVisibility::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisual3_Impl: ::windows_core::BaseImpl + IDCompositionVisualDebug_Impl {
    fn SetDepthMode(this: &Self::This, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows_core::Result<()>;
    fn SetOffsetZ(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOffsetZ2(this: &Self::This, offsetz: f32) -> ::windows_core::Result<()>;
    fn SetOpacity(this: &Self::This, animation: ::core::option::Option<&IDCompositionAnimation>) -> ::windows_core::Result<()>;
    fn SetOpacity2(this: &Self::This, opacity: f32) -> ::windows_core::Result<()>;
    fn SetTransform3(this: &Self::This, transform: ::core::option::Option<&IDCompositionTransform3D>) -> ::windows_core::Result<()>;
    fn SetTransform4(this: &Self::This, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows_core::Result<()>;
    fn SetVisible(this: &Self::This, visible: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionVisual3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionVisualDebug);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionVisual3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDepthMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDepthMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn SetOffsetZ<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetZ(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOffsetZ2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffsetZ2(this, ::core::mem::transmute_copy(&offsetz)).into())
        }
        unsafe extern "system" fn SetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacity(this, ::windows_core::from_raw_borrowed(&animation)).into())
        }
        unsafe extern "system" fn SetOpacity2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacity2(this, ::core::mem::transmute_copy(&opacity)).into())
        }
        unsafe extern "system" fn SetTransform3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform3(this, ::windows_core::from_raw_borrowed(&transform)).into())
        }
        unsafe extern "system" fn SetTransform4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform4(this, ::core::mem::transmute_copy(&matrix)).into())
        }
        unsafe extern "system" fn SetVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisual3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVisible(this, ::core::mem::transmute_copy(&visible)).into())
        }
        IDCompositionVisual3_Vtbl {
            base__: <IDCompositionVisualDebug as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDepthMode: SetDepthMode::<Identity, Impl, OFFSET>,
            SetOffsetZ: SetOffsetZ::<Identity, Impl, OFFSET>,
            SetOffsetZ2: SetOffsetZ2::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity2: SetOpacity2::<Identity, Impl, OFFSET>,
            SetTransform3: SetTransform3::<Identity, Impl, OFFSET>,
            SetTransform4: SetTransform4::<Identity, Impl, OFFSET>,
            SetVisible: SetVisible::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDCompositionVisualDebug_Impl: ::windows_core::BaseImpl + IDCompositionVisual2_Impl {
    fn EnableHeatMap(this: &Self::This, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()>;
    fn DisableHeatMap(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnableRedrawRegions(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisableRedrawRegions(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDCompositionVisualDebug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDCompositionVisual2);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisualDebug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCompositionVisualDebug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableHeatMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisualDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableHeatMap(this, ::core::mem::transmute_copy(&color)).into())
        }
        unsafe extern "system" fn DisableHeatMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisualDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableHeatMap(this).into())
        }
        unsafe extern "system" fn EnableRedrawRegions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisualDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableRedrawRegions(this).into())
        }
        unsafe extern "system" fn DisableRedrawRegions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCompositionVisualDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableRedrawRegions(this).into())
        }
        IDCompositionVisualDebug_Vtbl {
            base__: <IDCompositionVisual2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableHeatMap: EnableHeatMap::<Identity, Impl, OFFSET>,
            DisableHeatMap: DisableHeatMap::<Identity, Impl, OFFSET>,
            EnableRedrawRegions: EnableRedrawRegions::<Identity, Impl, OFFSET>,
            DisableRedrawRegions: DisableRedrawRegions::<Identity, Impl, OFFSET>,
        }
    };
}
