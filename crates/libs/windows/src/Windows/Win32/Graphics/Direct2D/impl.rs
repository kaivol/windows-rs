pub trait ID2D1AnalysisTransform_Impl: ::windows_core::BaseImpl {
    fn ProcessAnalysisResults(this: &Self::This, analysisdata: *const u8, analysisdatacount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1AnalysisTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1AnalysisTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1AnalysisTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessAnalysisResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1AnalysisTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysisdata: *const u8, analysisdatacount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessAnalysisResults(this, ::core::mem::transmute_copy(&analysisdata), ::core::mem::transmute_copy(&analysisdatacount)).into())
        }
        ID2D1AnalysisTransform_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProcessAnalysisResults: ProcessAnalysisResults::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1Bitmap_Impl: ::windows_core::BaseImpl + ID2D1Image_Impl {
    fn GetSize(this: &Self::This) -> Common::D2D_SIZE_F;
    fn GetPixelSize(this: &Self::This) -> Common::D2D_SIZE_U;
    fn GetPixelFormat(this: &Self::This) -> Common::D2D1_PIXEL_FORMAT;
    fn GetDpi(this: &Self::This, dpix: *mut f32, dpiy: *mut f32);
    fn CopyFromBitmap(this: &Self::This, destpoint: *const Common::D2D_POINT_2U, bitmap: ::core::option::Option<&ID2D1Bitmap>, srcrect: *const Common::D2D_RECT_U) -> ::windows_core::Result<()>;
    fn CopyFromRenderTarget(this: &Self::This, destpoint: *const Common::D2D_POINT_2U, rendertarget: ::core::option::Option<&ID2D1RenderTarget>, srcrect: *const Common::D2D_RECT_U) -> ::windows_core::Result<()>;
    fn CopyFromMemory(this: &Self::This, dstrect: *const Common::D2D_RECT_U, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID2D1Bitmap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Image);
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Bitmap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetSize(this))
        }
        unsafe extern "system" fn GetPixelSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetPixelSize(this))
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetPixelFormat(this))
        }
        unsafe extern "system" fn GetDpi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDpi(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)))
        }
        unsafe extern "system" fn CopyFromBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, bitmap: *mut ::core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyFromBitmap(this, ::core::mem::transmute_copy(&destpoint), ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&srcrect)).into())
        }
        unsafe extern "system" fn CopyFromRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destpoint: *const Common::D2D_POINT_2U, rendertarget: *mut ::core::ffi::c_void, srcrect: *const Common::D2D_RECT_U) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyFromRenderTarget(this, ::core::mem::transmute_copy(&destpoint), ::windows_core::from_raw_borrowed(&rendertarget), ::core::mem::transmute_copy(&srcrect)).into())
        }
        unsafe extern "system" fn CopyFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dstrect: *const Common::D2D_RECT_U, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyFromMemory(this, ::core::mem::transmute_copy(&dstrect), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&pitch)).into())
        }
        ID2D1Bitmap_Vtbl {
            base__: <ID2D1Image as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetPixelSize: GetPixelSize::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetDpi: GetDpi::<Identity, Impl, OFFSET>,
            CopyFromBitmap: CopyFromBitmap::<Identity, Impl, OFFSET>,
            CopyFromRenderTarget: CopyFromRenderTarget::<Identity, Impl, OFFSET>,
            CopyFromMemory: CopyFromMemory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1Bitmap1_Impl: ::windows_core::BaseImpl + ID2D1Bitmap_Impl {
    fn GetColorContext(this: &Self::This, colorcontext: *mut ::core::option::Option<ID2D1ColorContext>);
    fn GetOptions(this: &Self::This) -> D2D1_BITMAP_OPTIONS;
    fn GetSurface(this: &Self::This) -> ::windows_core::Result<super::Dxgi::IDXGISurface>;
    fn Map(this: &Self::This, options: D2D1_MAP_OPTIONS) -> ::windows_core::Result<D2D1_MAPPED_RECT>;
    fn Unmap(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID2D1Bitmap1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Bitmap);
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Bitmap1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorContext(this, ::core::mem::transmute_copy(&colorcontext)))
        }
        unsafe extern "system" fn GetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_OPTIONS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptions(this))
        }
        unsafe extern "system" fn GetSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgisurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dxgisurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_MAP_OPTIONS, mappedrect: *mut D2D1_MAPPED_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Map(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mappedrect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Bitmap1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this).into())
        }
        ID2D1Bitmap1_Vtbl {
            base__: <ID2D1Bitmap as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColorContext: GetColorContext::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            GetSurface: GetSurface::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`"]
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1BitmapBrush_Impl: ::windows_core::BaseImpl + ID2D1Brush_Impl {
    fn SetExtendModeX(this: &Self::This, extendmodex: D2D1_EXTEND_MODE);
    fn SetExtendModeY(this: &Self::This, extendmodey: D2D1_EXTEND_MODE);
    fn SetInterpolationMode(this: &Self::This, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE);
    fn SetBitmap(this: &Self::This, bitmap: ::core::option::Option<&ID2D1Bitmap>);
    fn GetExtendModeX(this: &Self::This) -> D2D1_EXTEND_MODE;
    fn GetExtendModeY(this: &Self::This) -> D2D1_EXTEND_MODE;
    fn GetInterpolationMode(this: &Self::This) -> D2D1_BITMAP_INTERPOLATION_MODE;
    fn GetBitmap(this: &Self::This, bitmap: *mut ::core::option::Option<ID2D1Bitmap>);
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::Iids for ID2D1BitmapBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Brush);
}
#[cfg(feature = "Foundation_Numerics")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1BitmapBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetExtendModeX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtendModeX(this, ::core::mem::transmute_copy(&extendmodex)))
        }
        unsafe extern "system" fn SetExtendModeY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtendModeY(this, ::core::mem::transmute_copy(&extendmodey)))
        }
        unsafe extern "system" fn SetInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterpolationMode(this, ::core::mem::transmute_copy(&interpolationmode)))
        }
        unsafe extern "system" fn SetBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitmap(this, ::windows_core::from_raw_borrowed(&bitmap)))
        }
        unsafe extern "system" fn GetExtendModeX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendModeX(this))
        }
        unsafe extern "system" fn GetExtendModeY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendModeY(this))
        }
        unsafe extern "system" fn GetInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_BITMAP_INTERPOLATION_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterpolationMode(this))
        }
        unsafe extern "system" fn GetBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBitmap(this, ::core::mem::transmute_copy(&bitmap)))
        }
        ID2D1BitmapBrush_Vtbl {
            base__: <ID2D1Brush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetExtendModeX: SetExtendModeX::<Identity, Impl, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, Impl, OFFSET>,
            SetInterpolationMode: SetInterpolationMode::<Identity, Impl, OFFSET>,
            SetBitmap: SetBitmap::<Identity, Impl, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, Impl, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, Impl, OFFSET>,
            GetInterpolationMode: GetInterpolationMode::<Identity, Impl, OFFSET>,
            GetBitmap: GetBitmap::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`"]
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1BitmapBrush1_Impl: ::windows_core::BaseImpl + ID2D1BitmapBrush_Impl {
    fn SetInterpolationMode1(this: &Self::This, interpolationmode: D2D1_INTERPOLATION_MODE);
    fn GetInterpolationMode1(this: &Self::This) -> D2D1_INTERPOLATION_MODE;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::Iids for ID2D1BitmapBrush1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1BitmapBrush);
}
#[cfg(feature = "Foundation_Numerics")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1BitmapBrush1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInterpolationMode1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterpolationMode1(this, ::core::mem::transmute_copy(&interpolationmode)))
        }
        unsafe extern "system" fn GetInterpolationMode1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapBrush1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterpolationMode1(this))
        }
        ID2D1BitmapBrush1_Vtbl {
            base__: <ID2D1BitmapBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInterpolationMode1: SetInterpolationMode1::<Identity, Impl, OFFSET>,
            GetInterpolationMode1: GetInterpolationMode1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1BitmapRenderTarget_Impl: ::windows_core::BaseImpl + ID2D1RenderTarget_Impl {
    fn GetBitmap(this: &Self::This) -> ::windows_core::Result<ID2D1Bitmap>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1BitmapRenderTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1RenderTarget);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapRenderTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1BitmapRenderTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBitmap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1BitmapRenderTarget_Vtbl { base__: <ID2D1RenderTarget as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBitmap: GetBitmap::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1BlendTransform_Impl: ::windows_core::BaseImpl + ID2D1ConcreteTransform_Impl {
    fn SetDescription(this: &Self::This, description: *const D2D1_BLEND_DESCRIPTION);
    fn GetDescription(this: &Self::This, description: *mut D2D1_BLEND_DESCRIPTION);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1BlendTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1ConcreteTransform);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BlendTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1BlendTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BlendTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *const D2D1_BLEND_DESCRIPTION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute_copy(&description)))
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BlendTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut D2D1_BLEND_DESCRIPTION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescription(this, ::core::mem::transmute_copy(&description)))
        }
        ID2D1BlendTransform_Vtbl {
            base__: <ID2D1ConcreteTransform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1BorderTransform_Impl: ::windows_core::BaseImpl + ID2D1ConcreteTransform_Impl {
    fn SetExtendModeX(this: &Self::This, extendmode: D2D1_EXTEND_MODE);
    fn SetExtendModeY(this: &Self::This, extendmode: D2D1_EXTEND_MODE);
    fn GetExtendModeX(this: &Self::This) -> D2D1_EXTEND_MODE;
    fn GetExtendModeY(this: &Self::This) -> D2D1_EXTEND_MODE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1BorderTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1ConcreteTransform);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BorderTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1BorderTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetExtendModeX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BorderTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtendModeX(this, ::core::mem::transmute_copy(&extendmode)))
        }
        unsafe extern "system" fn SetExtendModeY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BorderTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmode: D2D1_EXTEND_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtendModeY(this, ::core::mem::transmute_copy(&extendmode)))
        }
        unsafe extern "system" fn GetExtendModeX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BorderTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendModeX(this))
        }
        unsafe extern "system" fn GetExtendModeY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BorderTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendModeY(this))
        }
        ID2D1BorderTransform_Vtbl {
            base__: <ID2D1ConcreteTransform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetExtendModeX: SetExtendModeX::<Identity, Impl, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, Impl, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, Impl, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1BoundsAdjustmentTransform_Impl: ::windows_core::BaseImpl + ID2D1TransformNode_Impl {
    fn SetOutputBounds(this: &Self::This, outputbounds: *const super::super::Foundation::RECT);
    fn GetOutputBounds(this: &Self::This, outputbounds: *mut super::super::Foundation::RECT);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1BoundsAdjustmentTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1TransformNode);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BoundsAdjustmentTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1BoundsAdjustmentTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOutputBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BoundsAdjustmentTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputbounds: *const super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputBounds(this, ::core::mem::transmute_copy(&outputbounds)))
        }
        unsafe extern "system" fn GetOutputBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1BoundsAdjustmentTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputbounds: *mut super::super::Foundation::RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputBounds(this, ::core::mem::transmute_copy(&outputbounds)))
        }
        ID2D1BoundsAdjustmentTransform_Vtbl {
            base__: <ID2D1TransformNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOutputBounds: SetOutputBounds::<Identity, Impl, OFFSET>,
            GetOutputBounds: GetOutputBounds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`"]
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1Brush_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn SetOpacity(this: &Self::This, opacity: f32);
    fn SetTransform(this: &Self::This, transform: *const super::super::super::Foundation::Numerics::Matrix3x2);
    fn GetOpacity(this: &Self::This) -> f32;
    fn GetTransform(this: &Self::This, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::Iids for ID2D1Brush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(feature = "Foundation_Numerics")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Brush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Brush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Brush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacity(this, ::core::mem::transmute_copy(&opacity)))
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Brush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform(this, ::core::mem::transmute_copy(&transform)))
        }
        unsafe extern "system" fn GetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Brush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOpacity(this))
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Brush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransform(this, ::core::mem::transmute_copy(&transform)))
        }
        ID2D1Brush_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            GetOpacity: GetOpacity::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1ColorContext_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetColorSpace(this: &Self::This) -> D2D1_COLOR_SPACE;
    fn GetProfileSize(this: &Self::This) -> u32;
    fn GetProfile(this: &Self::This, profile: *mut u8, profilesize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1ColorContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ColorContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorSpace(this))
        }
        unsafe extern "system" fn GetProfileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProfileSize(this))
        }
        unsafe extern "system" fn GetProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: *mut u8, profilesize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProfile(this, ::core::mem::transmute_copy(&profile), ::core::mem::transmute_copy(&profilesize)).into())
        }
        ID2D1ColorContext_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColorSpace: GetColorSpace::<Identity, Impl, OFFSET>,
            GetProfileSize: GetProfileSize::<Identity, Impl, OFFSET>,
            GetProfile: GetProfile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID2D1ColorContext1_Impl: ::windows_core::BaseImpl + ID2D1ColorContext_Impl {
    fn GetColorContextType(this: &Self::This) -> D2D1_COLOR_CONTEXT_TYPE;
    fn GetDXGIColorSpace(this: &Self::This) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE;
    fn GetSimpleColorProfile(this: &Self::This, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for ID2D1ColorContext1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1ColorContext);
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ColorContext1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColorContextType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorContextType(this))
        }
        unsafe extern "system" fn GetDXGIColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDXGIColorSpace(this))
        }
        unsafe extern "system" fn GetSimpleColorProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ColorContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSimpleColorProfile(this, ::core::mem::transmute_copy(&simpleprofile)).into())
        }
        ID2D1ColorContext1_Vtbl {
            base__: <ID2D1ColorContext as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColorContextType: GetColorContextType::<Identity, Impl, OFFSET>,
            GetDXGIColorSpace: GetDXGIColorSpace::<Identity, Impl, OFFSET>,
            GetSimpleColorProfile: GetSimpleColorProfile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1CommandList_Impl: ::windows_core::BaseImpl + ID2D1Image_Impl {
    fn Stream(this: &Self::This, sink: ::core::option::Option<&ID2D1CommandSink>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1CommandList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Image);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1CommandList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Stream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stream(this, ::windows_core::from_raw_borrowed(&sink)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ID2D1CommandList_Vtbl {
            base__: <ID2D1Image as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Stream: Stream::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink_Impl: ::windows_core::BaseImpl {
    fn BeginDraw(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndDraw(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetAntialiasMode(this: &Self::This, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows_core::Result<()>;
    fn SetTags(this: &Self::This, tag1: u64, tag2: u64) -> ::windows_core::Result<()>;
    fn SetTextAntialiasMode(this: &Self::This, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows_core::Result<()>;
    fn SetTextRenderingParams(this: &Self::This, textrenderingparams: ::core::option::Option<&super::DirectWrite::IDWriteRenderingParams>) -> ::windows_core::Result<()>;
    fn SetTransform(this: &Self::This, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()>;
    fn SetPrimitiveBlend(this: &Self::This, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_core::Result<()>;
    fn SetUnitMode(this: &Self::This, unitmode: D2D1_UNIT_MODE) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This, color: *const Common::D2D1_COLOR_F) -> ::windows_core::Result<()>;
    fn DrawGlyphRun(this: &Self::This, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: ::core::option::Option<&ID2D1Brush>, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows_core::Result<()>;
    fn DrawLine(this: &Self::This, point0: &Common::D2D_POINT_2F, point1: &Common::D2D_POINT_2F, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>) -> ::windows_core::Result<()>;
    fn DrawGeometry(this: &Self::This, geometry: ::core::option::Option<&ID2D1Geometry>, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>) -> ::windows_core::Result<()>;
    fn DrawRectangle(this: &Self::This, rect: *const Common::D2D_RECT_F, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>) -> ::windows_core::Result<()>;
    fn DrawBitmap(this: &Self::This, bitmap: ::core::option::Option<&ID2D1Bitmap>, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> ::windows_core::Result<()>;
    fn DrawImage(this: &Self::This, image: ::core::option::Option<&ID2D1Image>, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows_core::Result<()>;
    fn DrawGdiMetafile(this: &Self::This, gdimetafile: ::core::option::Option<&ID2D1GdiMetafile>, targetoffset: *const Common::D2D_POINT_2F) -> ::windows_core::Result<()>;
    fn FillMesh(this: &Self::This, mesh: ::core::option::Option<&ID2D1Mesh>, brush: ::core::option::Option<&ID2D1Brush>) -> ::windows_core::Result<()>;
    fn FillOpacityMask(this: &Self::This, opacitymask: ::core::option::Option<&ID2D1Bitmap>, brush: ::core::option::Option<&ID2D1Brush>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows_core::Result<()>;
    fn FillGeometry(this: &Self::This, geometry: ::core::option::Option<&ID2D1Geometry>, brush: ::core::option::Option<&ID2D1Brush>, opacitybrush: ::core::option::Option<&ID2D1Brush>) -> ::windows_core::Result<()>;
    fn FillRectangle(this: &Self::This, rect: *const Common::D2D_RECT_F, brush: ::core::option::Option<&ID2D1Brush>) -> ::windows_core::Result<()>;
    fn PushAxisAlignedClip(this: &Self::This, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows_core::Result<()>;
    fn PushLayer(this: &Self::This, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: ::core::option::Option<&ID2D1Layer>) -> ::windows_core::Result<()>;
    fn PopAxisAlignedClip(this: &Self::This) -> ::windows_core::Result<()>;
    fn PopLayer(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1CommandSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1CommandSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDraw(this).into())
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDraw(this).into())
        }
        unsafe extern "system" fn SetAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAntialiasMode(this, ::core::mem::transmute_copy(&antialiasmode)).into())
        }
        unsafe extern "system" fn SetTags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTags(this, ::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)).into())
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextAntialiasMode(this, ::core::mem::transmute_copy(&textantialiasmode)).into())
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextRenderingParams(this, ::windows_core::from_raw_borrowed(&textrenderingparams)).into())
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform(this, ::core::mem::transmute_copy(&transform)).into())
        }
        unsafe extern "system" fn SetPrimitiveBlend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrimitiveBlend(this, ::core::mem::transmute_copy(&primitiveblend)).into())
        }
        unsafe extern "system" fn SetUnitMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnitMode(this, ::core::mem::transmute_copy(&unitmode)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this, ::core::mem::transmute_copy(&color)).into())
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGlyphRun(this, ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::windows_core::from_raw_borrowed(&foregroundbrush), ::core::mem::transmute_copy(&measuringmode)).into())
        }
        unsafe extern "system" fn DrawLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawLine(this, ::core::mem::transmute(&point0), ::core::mem::transmute(&point1), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)).into())
        }
        unsafe extern "system" fn DrawGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGeometry(this, ::windows_core::from_raw_borrowed(&geometry), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)).into())
        }
        unsafe extern "system" fn DrawRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawRectangle(this, ::core::mem::transmute_copy(&rect), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)).into())
        }
        unsafe extern "system" fn DrawBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawBitmap(this, ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&opacity), ::core::mem::transmute_copy(&interpolationmode), ::core::mem::transmute_copy(&sourcerectangle), ::core::mem::transmute_copy(&perspectivetransform)).into())
        }
        unsafe extern "system" fn DrawImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawImage(this, ::windows_core::from_raw_borrowed(&image), ::core::mem::transmute_copy(&targetoffset), ::core::mem::transmute_copy(&imagerectangle), ::core::mem::transmute_copy(&interpolationmode), ::core::mem::transmute_copy(&compositemode)).into())
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGdiMetafile(this, ::windows_core::from_raw_borrowed(&gdimetafile), ::core::mem::transmute_copy(&targetoffset)).into())
        }
        unsafe extern "system" fn FillMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mesh: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillMesh(this, ::windows_core::from_raw_borrowed(&mesh), ::windows_core::from_raw_borrowed(&brush)).into())
        }
        unsafe extern "system" fn FillOpacityMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillOpacityMask(this, ::windows_core::from_raw_borrowed(&opacitymask), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&sourcerectangle)).into())
        }
        unsafe extern "system" fn FillGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, opacitybrush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillGeometry(this, ::windows_core::from_raw_borrowed(&geometry), ::windows_core::from_raw_borrowed(&brush), ::windows_core::from_raw_borrowed(&opacitybrush)).into())
        }
        unsafe extern "system" fn FillRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillRectangle(this, ::core::mem::transmute_copy(&rect), ::windows_core::from_raw_borrowed(&brush)).into())
        }
        unsafe extern "system" fn PushAxisAlignedClip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushAxisAlignedClip(this, ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&antialiasmode)).into())
        }
        unsafe extern "system" fn PushLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushLayer(this, ::core::mem::transmute_copy(&layerparameters1), ::windows_core::from_raw_borrowed(&layer)).into())
        }
        unsafe extern "system" fn PopAxisAlignedClip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopAxisAlignedClip(this).into())
        }
        unsafe extern "system" fn PopLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopLayer(this).into())
        }
        ID2D1CommandSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            SetAntialiasMode: SetAntialiasMode::<Identity, Impl, OFFSET>,
            SetTags: SetTags::<Identity, Impl, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, Impl, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            SetPrimitiveBlend: SetPrimitiveBlend::<Identity, Impl, OFFSET>,
            SetUnitMode: SetUnitMode::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, Impl, OFFSET>,
            DrawLine: DrawLine::<Identity, Impl, OFFSET>,
            DrawGeometry: DrawGeometry::<Identity, Impl, OFFSET>,
            DrawRectangle: DrawRectangle::<Identity, Impl, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, Impl, OFFSET>,
            DrawImage: DrawImage::<Identity, Impl, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, Impl, OFFSET>,
            FillMesh: FillMesh::<Identity, Impl, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, Impl, OFFSET>,
            FillGeometry: FillGeometry::<Identity, Impl, OFFSET>,
            FillRectangle: FillRectangle::<Identity, Impl, OFFSET>,
            PushAxisAlignedClip: PushAxisAlignedClip::<Identity, Impl, OFFSET>,
            PushLayer: PushLayer::<Identity, Impl, OFFSET>,
            PopAxisAlignedClip: PopAxisAlignedClip::<Identity, Impl, OFFSET>,
            PopLayer: PopLayer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink1_Impl: ::windows_core::BaseImpl + ID2D1CommandSink_Impl {
    fn SetPrimitiveBlend1(this: &Self::This, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1CommandSink1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1CommandSink);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1CommandSink1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPrimitiveBlend1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrimitiveBlend1(this, ::core::mem::transmute_copy(&primitiveblend)).into())
        }
        ID2D1CommandSink1_Vtbl { base__: <ID2D1CommandSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetPrimitiveBlend1: SetPrimitiveBlend1::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink2_Impl: ::windows_core::BaseImpl + ID2D1CommandSink1_Impl {
    fn DrawInk(this: &Self::This, ink: ::core::option::Option<&ID2D1Ink>, brush: ::core::option::Option<&ID2D1Brush>, inkstyle: ::core::option::Option<&ID2D1InkStyle>) -> ::windows_core::Result<()>;
    fn DrawGradientMesh(this: &Self::This, gradientmesh: ::core::option::Option<&ID2D1GradientMesh>) -> ::windows_core::Result<()>;
    fn DrawGdiMetafile2(this: &Self::This, gdimetafile: ::core::option::Option<&ID2D1GdiMetafile>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1CommandSink2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1CommandSink1);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1CommandSink2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DrawInk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ink: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInk(this, ::windows_core::from_raw_borrowed(&ink), ::windows_core::from_raw_borrowed(&brush), ::windows_core::from_raw_borrowed(&inkstyle)).into())
        }
        unsafe extern "system" fn DrawGradientMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientmesh: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGradientMesh(this, ::windows_core::from_raw_borrowed(&gradientmesh)).into())
        }
        unsafe extern "system" fn DrawGdiMetafile2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGdiMetafile2(this, ::windows_core::from_raw_borrowed(&gdimetafile), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&sourcerectangle)).into())
        }
        ID2D1CommandSink2_Vtbl {
            base__: <ID2D1CommandSink1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DrawInk: DrawInk::<Identity, Impl, OFFSET>,
            DrawGradientMesh: DrawGradientMesh::<Identity, Impl, OFFSET>,
            DrawGdiMetafile2: DrawGdiMetafile2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink3_Impl: ::windows_core::BaseImpl + ID2D1CommandSink2_Impl {
    fn DrawSpriteBatch(this: &Self::This, spritebatch: ::core::option::Option<&ID2D1SpriteBatch>, startindex: u32, spritecount: u32, bitmap: ::core::option::Option<&ID2D1Bitmap>, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1CommandSink3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1CommandSink2);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1CommandSink3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DrawSpriteBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritebatch: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawSpriteBatch(this, ::windows_core::from_raw_borrowed(&spritebatch), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&spritecount), ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&interpolationmode), ::core::mem::transmute_copy(&spriteoptions)).into())
        }
        ID2D1CommandSink3_Vtbl { base__: <ID2D1CommandSink2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DrawSpriteBatch: DrawSpriteBatch::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink4_Impl: ::windows_core::BaseImpl + ID2D1CommandSink3_Impl {
    fn SetPrimitiveBlend2(this: &Self::This, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1CommandSink4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1CommandSink3);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1CommandSink4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPrimitiveBlend2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrimitiveBlend2(this, ::core::mem::transmute_copy(&primitiveblend)).into())
        }
        ID2D1CommandSink4_Vtbl {
            base__: <ID2D1CommandSink3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPrimitiveBlend2: SetPrimitiveBlend2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1CommandSink5_Impl: ::windows_core::BaseImpl + ID2D1CommandSink4_Impl {
    fn BlendImage(this: &Self::This, image: ::core::option::Option<&ID2D1Image>, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1CommandSink5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1CommandSink4);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1CommandSink5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BlendImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1CommandSink5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BlendImage(this, ::windows_core::from_raw_borrowed(&image), ::core::mem::transmute_copy(&blendmode), ::core::mem::transmute_copy(&targetoffset), ::core::mem::transmute_copy(&imagerectangle), ::core::mem::transmute_copy(&interpolationmode)).into())
        }
        ID2D1CommandSink5_Vtbl { base__: <ID2D1CommandSink4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, BlendImage: BlendImage::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ComputeInfo_Impl: ::windows_core::BaseImpl + ID2D1RenderInfo_Impl {
    fn SetComputeShaderConstantBuffer(this: &Self::This, buffer: *const u8, buffercount: u32) -> ::windows_core::Result<()>;
    fn SetComputeShader(this: &Self::This, shaderid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetResourceTexture(this: &Self::This, textureindex: u32, resourcetexture: ::core::option::Option<&ID2D1ResourceTexture>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1ComputeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1RenderInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ComputeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ComputeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetComputeShaderConstantBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ComputeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeShaderConstantBuffer(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffercount)).into())
        }
        unsafe extern "system" fn SetComputeShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ComputeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeShader(this, ::core::mem::transmute_copy(&shaderid)).into())
        }
        unsafe extern "system" fn SetResourceTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ComputeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResourceTexture(this, ::core::mem::transmute_copy(&textureindex), ::windows_core::from_raw_borrowed(&resourcetexture)).into())
        }
        ID2D1ComputeInfo_Vtbl {
            base__: <ID2D1RenderInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetComputeShaderConstantBuffer: SetComputeShaderConstantBuffer::<Identity, Impl, OFFSET>,
            SetComputeShader: SetComputeShader::<Identity, Impl, OFFSET>,
            SetResourceTexture: SetResourceTexture::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ComputeTransform_Impl: ::windows_core::BaseImpl + ID2D1Transform_Impl {
    fn SetComputeInfo(this: &Self::This, computeinfo: ::core::option::Option<&ID2D1ComputeInfo>) -> ::windows_core::Result<()>;
    fn CalculateThreadgroups(this: &Self::This, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1ComputeTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Transform);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ComputeTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ComputeTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetComputeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ComputeTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, computeinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetComputeInfo(this, ::windows_core::from_raw_borrowed(&computeinfo)).into())
        }
        unsafe extern "system" fn CalculateThreadgroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ComputeTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, dimensionx: *mut u32, dimensiony: *mut u32, dimensionz: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CalculateThreadgroups(this, ::core::mem::transmute_copy(&outputrect), ::core::mem::transmute_copy(&dimensionx), ::core::mem::transmute_copy(&dimensiony), ::core::mem::transmute_copy(&dimensionz)).into())
        }
        ID2D1ComputeTransform_Vtbl {
            base__: <ID2D1Transform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetComputeInfo: SetComputeInfo::<Identity, Impl, OFFSET>,
            CalculateThreadgroups: CalculateThreadgroups::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ConcreteTransform_Impl: ::windows_core::BaseImpl + ID2D1TransformNode_Impl {
    fn SetOutputBuffer(this: &Self::This, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows_core::Result<()>;
    fn SetCached(this: &Self::This, iscached: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1ConcreteTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1TransformNode);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ConcreteTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ConcreteTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOutputBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ConcreteTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputBuffer(this, ::core::mem::transmute_copy(&bufferprecision), ::core::mem::transmute_copy(&channeldepth)).into())
        }
        unsafe extern "system" fn SetCached<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ConcreteTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCached(this, ::core::mem::transmute_copy(&iscached)))
        }
        ID2D1ConcreteTransform_Vtbl {
            base__: <ID2D1TransformNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOutputBuffer: SetOutputBuffer::<Identity, Impl, OFFSET>,
            SetCached: SetCached::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DCRenderTarget_Impl: ::windows_core::BaseImpl + ID2D1RenderTarget_Impl {
    fn BindDC(this: &Self::This, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1DCRenderTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1RenderTarget);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DCRenderTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DCRenderTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DCRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, psubrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindDC(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&psubrect)).into())
        }
        ID2D1DCRenderTarget_Vtbl { base__: <ID2D1RenderTarget as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, BindDC: BindDC::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`"]
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn CreateDeviceContext(this: &Self::This, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows_core::Result<ID2D1DeviceContext>;
    fn CreatePrintControl(this: &Self::This, wicfactory: ::core::option::Option<&super::Imaging::IWICImagingFactory>, documenttarget: ::core::option::Option<&super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES) -> ::windows_core::Result<ID2D1PrintControl>;
    fn SetMaximumTextureMemory(this: &Self::This, maximuminbytes: u64);
    fn GetMaximumTextureMemory(this: &Self::This) -> u64;
    fn ClearResources(this: &Self::This, millisecondssinceuse: u32);
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ::windows_core::Iids for ID2D1Device {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Device {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDeviceContext(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePrintControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicfactory: *mut ::core::ffi::c_void, documenttarget: *mut ::core::ffi::c_void, printcontrolproperties: *const D2D1_PRINT_CONTROL_PROPERTIES, printcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePrintControl(this, ::windows_core::from_raw_borrowed(&wicfactory), ::windows_core::from_raw_borrowed(&documenttarget), ::core::mem::transmute_copy(&printcontrolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printcontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaximumTextureMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximuminbytes: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumTextureMemory(this, ::core::mem::transmute_copy(&maximuminbytes)))
        }
        unsafe extern "system" fn GetMaximumTextureMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaximumTextureMemory(this))
        }
        unsafe extern "system" fn ClearResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, millisecondssinceuse: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearResources(this, ::core::mem::transmute_copy(&millisecondssinceuse)))
        }
        ID2D1Device_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDeviceContext: CreateDeviceContext::<Identity, Impl, OFFSET>,
            CreatePrintControl: CreatePrintControl::<Identity, Impl, OFFSET>,
            SetMaximumTextureMemory: SetMaximumTextureMemory::<Identity, Impl, OFFSET>,
            GetMaximumTextureMemory: GetMaximumTextureMemory::<Identity, Impl, OFFSET>,
            ClearResources: ClearResources::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`"]
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device1_Impl: ::windows_core::BaseImpl + ID2D1Device_Impl {
    fn GetRenderingPriority(this: &Self::This) -> D2D1_RENDERING_PRIORITY;
    fn SetRenderingPriority(this: &Self::This, renderingpriority: D2D1_RENDERING_PRIORITY);
    fn CreateDeviceContext2(this: &Self::This, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows_core::Result<ID2D1DeviceContext1>;
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ::windows_core::Iids for ID2D1Device1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Device);
}
#[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Device1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRenderingPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_RENDERING_PRIORITY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRenderingPriority(this))
        }
        unsafe extern "system" fn SetRenderingPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingpriority: D2D1_RENDERING_PRIORITY) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRenderingPriority(this, ::core::mem::transmute_copy(&renderingpriority)))
        }
        unsafe extern "system" fn CreateDeviceContext2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDeviceContext2(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecontext1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Device1_Vtbl {
            base__: <ID2D1Device as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRenderingPriority: GetRenderingPriority::<Identity, Impl, OFFSET>,
            SetRenderingPriority: SetRenderingPriority::<Identity, Impl, OFFSET>,
            CreateDeviceContext2: CreateDeviceContext2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device2_Impl: ::windows_core::BaseImpl + ID2D1Device1_Impl {
    fn CreateDeviceContext3(this: &Self::This, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows_core::Result<ID2D1DeviceContext2>;
    fn FlushDeviceContexts(this: &Self::This, bitmap: ::core::option::Option<&ID2D1Bitmap>);
    fn GetDxgiDevice(this: &Self::This) -> ::windows_core::Result<super::Dxgi::IDXGIDevice>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ::windows_core::Iids for ID2D1Device2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Device1);
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Device2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceContext3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDeviceContext3(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecontext2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FlushDeviceContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushDeviceContexts(this, ::windows_core::from_raw_borrowed(&bitmap)))
        }
        unsafe extern "system" fn GetDxgiDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDxgiDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dxgidevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Device2_Vtbl {
            base__: <ID2D1Device1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDeviceContext3: CreateDeviceContext3::<Identity, Impl, OFFSET>,
            FlushDeviceContexts: FlushDeviceContexts::<Identity, Impl, OFFSET>,
            GetDxgiDevice: GetDxgiDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device3_Impl: ::windows_core::BaseImpl + ID2D1Device2_Impl {
    fn CreateDeviceContext4(this: &Self::This, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows_core::Result<ID2D1DeviceContext3>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ::windows_core::Iids for ID2D1Device3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Device2);
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Device3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceContext4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext3: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDeviceContext4(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecontext3, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Device3_Vtbl { base__: <ID2D1Device2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDeviceContext4: CreateDeviceContext4::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device4_Impl: ::windows_core::BaseImpl + ID2D1Device3_Impl {
    fn CreateDeviceContext5(this: &Self::This, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows_core::Result<ID2D1DeviceContext4>;
    fn SetMaximumColorGlyphCacheMemory(this: &Self::This, maximuminbytes: u64);
    fn GetMaximumColorGlyphCacheMemory(this: &Self::This) -> u64;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ::windows_core::Iids for ID2D1Device4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Device3);
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Device4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceContext5<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext4: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDeviceContext5(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecontext4, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaximumColorGlyphCacheMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximuminbytes: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaximumColorGlyphCacheMemory(this, ::core::mem::transmute_copy(&maximuminbytes)))
        }
        unsafe extern "system" fn GetMaximumColorGlyphCacheMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaximumColorGlyphCacheMemory(this))
        }
        ID2D1Device4_Vtbl {
            base__: <ID2D1Device3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDeviceContext5: CreateDeviceContext5::<Identity, Impl, OFFSET>,
            SetMaximumColorGlyphCacheMemory: SetMaximumColorGlyphCacheMemory::<Identity, Impl, OFFSET>,
            GetMaximumColorGlyphCacheMemory: GetMaximumColorGlyphCacheMemory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device5_Impl: ::windows_core::BaseImpl + ID2D1Device4_Impl {
    fn CreateDeviceContext6(this: &Self::This, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows_core::Result<ID2D1DeviceContext5>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ::windows_core::Iids for ID2D1Device5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Device4);
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Device5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceContext6<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext5: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDeviceContext6(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecontext5, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Device5_Vtbl { base__: <ID2D1Device4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDeviceContext6: CreateDeviceContext6::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`"]
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
pub trait ID2D1Device6_Impl: ::windows_core::BaseImpl + ID2D1Device5_Impl {
    fn CreateDeviceContext7(this: &Self::This, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows_core::Result<ID2D1DeviceContext6>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl ::windows_core::Iids for ID2D1Device6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Device5);
}
#[cfg(all(feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Device6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDeviceContext7<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Device6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext6: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDeviceContext7(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecontext6, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Device6_Vtbl { base__: <ID2D1Device5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDeviceContext7: CreateDeviceContext7::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext_Impl: ::windows_core::BaseImpl + ID2D1RenderTarget_Impl {
    fn CreateBitmap2(this: &Self::This, size: &Common::D2D_SIZE_U, sourcedata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows_core::Result<ID2D1Bitmap1>;
    fn CreateBitmapFromWicBitmap2(this: &Self::This, wicbitmapsource: ::core::option::Option<&super::Imaging::IWICBitmapSource>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows_core::Result<ID2D1Bitmap1>;
    fn CreateColorContext(this: &Self::This, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32) -> ::windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromFilename(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromWicColorContext(this: &Self::This, wiccolorcontext: ::core::option::Option<&super::Imaging::IWICColorContext>) -> ::windows_core::Result<ID2D1ColorContext>;
    fn CreateBitmapFromDxgiSurface(this: &Self::This, surface: ::core::option::Option<&super::Dxgi::IDXGISurface>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows_core::Result<ID2D1Bitmap1>;
    fn CreateEffect(this: &Self::This, effectid: *const ::windows_core::GUID) -> ::windows_core::Result<ID2D1Effect>;
    fn CreateGradientStopCollection2(this: &Self::This, straightalphagradientstops: *const D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows_core::Result<ID2D1GradientStopCollection1>;
    fn CreateImageBrush(this: &Self::This, image: ::core::option::Option<&ID2D1Image>, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> ::windows_core::Result<ID2D1ImageBrush>;
    fn CreateBitmapBrush2(this: &Self::This, bitmap: ::core::option::Option<&ID2D1Bitmap>, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> ::windows_core::Result<ID2D1BitmapBrush1>;
    fn CreateCommandList(this: &Self::This) -> ::windows_core::Result<ID2D1CommandList>;
    fn IsDxgiFormatSupported(this: &Self::This, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL;
    fn IsBufferPrecisionSupported(this: &Self::This, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL;
    fn GetImageLocalBounds(this: &Self::This, image: ::core::option::Option<&ID2D1Image>) -> ::windows_core::Result<Common::D2D_RECT_F>;
    fn GetImageWorldBounds(this: &Self::This, image: ::core::option::Option<&ID2D1Image>) -> ::windows_core::Result<Common::D2D_RECT_F>;
    fn GetGlyphRunWorldBounds(this: &Self::This, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows_core::Result<Common::D2D_RECT_F>;
    fn GetDevice(this: &Self::This, device: *mut ::core::option::Option<ID2D1Device>);
    fn SetTarget(this: &Self::This, image: ::core::option::Option<&ID2D1Image>);
    fn GetTarget(this: &Self::This, image: *mut ::core::option::Option<ID2D1Image>);
    fn SetRenderingControls(this: &Self::This, renderingcontrols: *const D2D1_RENDERING_CONTROLS);
    fn GetRenderingControls(this: &Self::This, renderingcontrols: *mut D2D1_RENDERING_CONTROLS);
    fn SetPrimitiveBlend(this: &Self::This, primitiveblend: D2D1_PRIMITIVE_BLEND);
    fn GetPrimitiveBlend(this: &Self::This) -> D2D1_PRIMITIVE_BLEND;
    fn SetUnitMode(this: &Self::This, unitmode: D2D1_UNIT_MODE);
    fn GetUnitMode(this: &Self::This) -> D2D1_UNIT_MODE;
    fn DrawGlyphRun2(this: &Self::This, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: ::core::option::Option<&ID2D1Brush>, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn DrawImage(this: &Self::This, image: ::core::option::Option<&ID2D1Image>, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE);
    fn DrawGdiMetafile(this: &Self::This, gdimetafile: ::core::option::Option<&ID2D1GdiMetafile>, targetoffset: *const Common::D2D_POINT_2F);
    fn DrawBitmap2(this: &Self::This, bitmap: ::core::option::Option<&ID2D1Bitmap>, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F);
    fn PushLayer2(this: &Self::This, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: ::core::option::Option<&ID2D1Layer>);
    fn InvalidateEffectInputRectangle(this: &Self::This, effect: ::core::option::Option<&ID2D1Effect>, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows_core::Result<()>;
    fn GetEffectInvalidRectangleCount(this: &Self::This, effect: ::core::option::Option<&ID2D1Effect>) -> ::windows_core::Result<u32>;
    fn GetEffectInvalidRectangles(this: &Self::This, effect: ::core::option::Option<&ID2D1Effect>, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> ::windows_core::Result<()>;
    fn GetEffectRequiredInputRectangles(this: &Self::This, rendereffect: ::core::option::Option<&ID2D1Effect>, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows_core::Result<()>;
    fn FillOpacityMask2(this: &Self::This, opacitymask: ::core::option::Option<&ID2D1Bitmap>, brush: ::core::option::Option<&ID2D1Brush>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1DeviceContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1RenderTarget);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DeviceContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBitmap2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, sourcedata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmap2(this, ::core::mem::transmute(&size), ::core::mem::transmute_copy(&sourcedata), ::core::mem::transmute_copy(&pitch), ::core::mem::transmute_copy(&bitmapproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromWicBitmap2(this, ::windows_core::from_raw_borrowed(&wicbitmapsource), ::core::mem::transmute_copy(&bitmapproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContext(this, ::core::mem::transmute_copy(&space), ::core::mem::transmute_copy(&profile), ::core::mem::transmute_copy(&profilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromFilename(this, ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wiccolorcontext: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromWicColorContext(this, ::windows_core::from_raw_borrowed(&wiccolorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromDxgiSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromDxgiSurface(this, ::windows_core::from_raw_borrowed(&surface), ::core::mem::transmute_copy(&bitmapproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows_core::GUID, effect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEffect(this, ::core::mem::transmute_copy(&effectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(effect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGradientStopCollection2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straightalphagradientstops: *const D2D1_GRADIENT_STOP, straightalphagradientstopscount: u32, preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE, gradientstopcollection1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGradientStopCollection2(this, ::core::mem::transmute_copy(&straightalphagradientstops), ::core::mem::transmute_copy(&straightalphagradientstopscount), ::core::mem::transmute_copy(&preinterpolationspace), ::core::mem::transmute_copy(&postinterpolationspace), ::core::mem::transmute_copy(&bufferprecision), ::core::mem::transmute_copy(&extendmode), ::core::mem::transmute_copy(&colorinterpolationmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstopcollection1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateImageBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateImageBrush(this, ::windows_core::from_raw_borrowed(&image), ::core::mem::transmute_copy(&imagebrushproperties), ::core::mem::transmute_copy(&brushproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagebrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapBrush2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapBrush2(this, ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&bitmapbrushproperties), ::core::mem::transmute_copy(&brushproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmapbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCommandList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCommandList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commandlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDxgiFormatSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDxgiFormatSupported(this, ::core::mem::transmute_copy(&format)))
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsBufferPrecisionSupported(this, ::core::mem::transmute_copy(&bufferprecision)))
        }
        unsafe extern "system" fn GetImageLocalBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, localbounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageLocalBounds(this, ::windows_core::from_raw_borrowed(&image)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(localbounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImageWorldBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, worldbounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageWorldBounds(this, ::windows_core::from_raw_borrowed(&image)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(worldbounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlyphRunWorldBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlyphRunWorldBounds(this, ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&measuringmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevice(this, ::core::mem::transmute_copy(&device)))
        }
        unsafe extern "system" fn SetTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTarget(this, ::windows_core::from_raw_borrowed(&image)))
        }
        unsafe extern "system" fn GetTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTarget(this, ::core::mem::transmute_copy(&image)))
        }
        unsafe extern "system" fn SetRenderingControls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRenderingControls(this, ::core::mem::transmute_copy(&renderingcontrols)))
        }
        unsafe extern "system" fn GetRenderingControls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingcontrols: *mut D2D1_RENDERING_CONTROLS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRenderingControls(this, ::core::mem::transmute_copy(&renderingcontrols)))
        }
        unsafe extern "system" fn SetPrimitiveBlend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, primitiveblend: D2D1_PRIMITIVE_BLEND) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrimitiveBlend(this, ::core::mem::transmute_copy(&primitiveblend)))
        }
        unsafe extern "system" fn GetPrimitiveBlend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrimitiveBlend(this))
        }
        unsafe extern "system" fn SetUnitMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unitmode: D2D1_UNIT_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnitMode(this, ::core::mem::transmute_copy(&unitmode)))
        }
        unsafe extern "system" fn GetUnitMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_UNIT_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUnitMode(this))
        }
        unsafe extern "system" fn DrawGlyphRun2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGlyphRun2(this, ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::windows_core::from_raw_borrowed(&foregroundbrush), ::core::mem::transmute_copy(&measuringmode)))
        }
        unsafe extern "system" fn DrawImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawImage(this, ::windows_core::from_raw_borrowed(&image), ::core::mem::transmute_copy(&targetoffset), ::core::mem::transmute_copy(&imagerectangle), ::core::mem::transmute_copy(&interpolationmode), ::core::mem::transmute_copy(&compositemode)))
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, targetoffset: *const Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGdiMetafile(this, ::windows_core::from_raw_borrowed(&gdimetafile), ::core::mem::transmute_copy(&targetoffset)))
        }
        unsafe extern "system" fn DrawBitmap2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F, perspectivetransform: *const Common::D2D_MATRIX_4X4_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawBitmap2(this, ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&opacity), ::core::mem::transmute_copy(&interpolationmode), ::core::mem::transmute_copy(&sourcerectangle), ::core::mem::transmute_copy(&perspectivetransform)))
        }
        unsafe extern "system" fn PushLayer2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushLayer2(this, ::core::mem::transmute_copy(&layerparameters), ::windows_core::from_raw_borrowed(&layer)))
        }
        unsafe extern "system" fn InvalidateEffectInputRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateEffectInputRectangle(this, ::windows_core::from_raw_borrowed(&effect), ::core::mem::transmute_copy(&input), ::core::mem::transmute_copy(&inputrectangle)).into())
        }
        unsafe extern "system" fn GetEffectInvalidRectangleCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, rectanglecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEffectInvalidRectangleCount(this, ::windows_core::from_raw_borrowed(&effect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectanglecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEffectInvalidRectangles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, rectangles: *mut Common::D2D_RECT_F, rectanglescount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectInvalidRectangles(this, ::windows_core::from_raw_borrowed(&effect), ::core::mem::transmute_copy(&rectangles), ::core::mem::transmute_copy(&rectanglescount)).into())
        }
        unsafe extern "system" fn GetEffectRequiredInputRectangles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendereffect: *mut ::core::ffi::c_void, renderimagerectangle: *const Common::D2D_RECT_F, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEffectRequiredInputRectangles(this, ::windows_core::from_raw_borrowed(&rendereffect), ::core::mem::transmute_copy(&renderimagerectangle), ::core::mem::transmute_copy(&inputdescriptions), ::core::mem::transmute_copy(&requiredinputrects), ::core::mem::transmute_copy(&inputcount)).into())
        }
        unsafe extern "system" fn FillOpacityMask2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillOpacityMask2(this, ::windows_core::from_raw_borrowed(&opacitymask), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&sourcerectangle)))
        }
        ID2D1DeviceContext_Vtbl {
            base__: <ID2D1RenderTarget as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateBitmap2: CreateBitmap2::<Identity, Impl, OFFSET>,
            CreateBitmapFromWicBitmap2: CreateBitmapFromWicBitmap2::<Identity, Impl, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, Impl, OFFSET>,
            CreateColorContextFromFilename: CreateColorContextFromFilename::<Identity, Impl, OFFSET>,
            CreateColorContextFromWicColorContext: CreateColorContextFromWicColorContext::<Identity, Impl, OFFSET>,
            CreateBitmapFromDxgiSurface: CreateBitmapFromDxgiSurface::<Identity, Impl, OFFSET>,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            CreateGradientStopCollection2: CreateGradientStopCollection2::<Identity, Impl, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, Impl, OFFSET>,
            CreateBitmapBrush2: CreateBitmapBrush2::<Identity, Impl, OFFSET>,
            CreateCommandList: CreateCommandList::<Identity, Impl, OFFSET>,
            IsDxgiFormatSupported: IsDxgiFormatSupported::<Identity, Impl, OFFSET>,
            IsBufferPrecisionSupported: IsBufferPrecisionSupported::<Identity, Impl, OFFSET>,
            GetImageLocalBounds: GetImageLocalBounds::<Identity, Impl, OFFSET>,
            GetImageWorldBounds: GetImageWorldBounds::<Identity, Impl, OFFSET>,
            GetGlyphRunWorldBounds: GetGlyphRunWorldBounds::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            SetTarget: SetTarget::<Identity, Impl, OFFSET>,
            GetTarget: GetTarget::<Identity, Impl, OFFSET>,
            SetRenderingControls: SetRenderingControls::<Identity, Impl, OFFSET>,
            GetRenderingControls: GetRenderingControls::<Identity, Impl, OFFSET>,
            SetPrimitiveBlend: SetPrimitiveBlend::<Identity, Impl, OFFSET>,
            GetPrimitiveBlend: GetPrimitiveBlend::<Identity, Impl, OFFSET>,
            SetUnitMode: SetUnitMode::<Identity, Impl, OFFSET>,
            GetUnitMode: GetUnitMode::<Identity, Impl, OFFSET>,
            DrawGlyphRun2: DrawGlyphRun2::<Identity, Impl, OFFSET>,
            DrawImage: DrawImage::<Identity, Impl, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, Impl, OFFSET>,
            DrawBitmap2: DrawBitmap2::<Identity, Impl, OFFSET>,
            PushLayer2: PushLayer2::<Identity, Impl, OFFSET>,
            InvalidateEffectInputRectangle: InvalidateEffectInputRectangle::<Identity, Impl, OFFSET>,
            GetEffectInvalidRectangleCount: GetEffectInvalidRectangleCount::<Identity, Impl, OFFSET>,
            GetEffectInvalidRectangles: GetEffectInvalidRectangles::<Identity, Impl, OFFSET>,
            GetEffectRequiredInputRectangles: GetEffectRequiredInputRectangles::<Identity, Impl, OFFSET>,
            FillOpacityMask2: FillOpacityMask2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext1_Impl: ::windows_core::BaseImpl + ID2D1DeviceContext_Impl {
    fn CreateFilledGeometryRealization(this: &Self::This, geometry: ::core::option::Option<&ID2D1Geometry>, flatteningtolerance: f32) -> ::windows_core::Result<ID2D1GeometryRealization>;
    fn CreateStrokedGeometryRealization(this: &Self::This, geometry: ::core::option::Option<&ID2D1Geometry>, flatteningtolerance: f32, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>) -> ::windows_core::Result<ID2D1GeometryRealization>;
    fn DrawGeometryRealization(this: &Self::This, geometryrealization: ::core::option::Option<&ID2D1GeometryRealization>, brush: ::core::option::Option<&ID2D1Brush>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1DeviceContext1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1DeviceContext);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DeviceContext1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFilledGeometryRealization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, flatteningtolerance: f32, geometryrealization: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFilledGeometryRealization(this, ::windows_core::from_raw_borrowed(&geometry), ::core::mem::transmute_copy(&flatteningtolerance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometryrealization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStrokedGeometryRealization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, flatteningtolerance: f32, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, geometryrealization: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStrokedGeometryRealization(this, ::windows_core::from_raw_borrowed(&geometry), ::core::mem::transmute_copy(&flatteningtolerance), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometryrealization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawGeometryRealization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometryrealization: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGeometryRealization(this, ::windows_core::from_raw_borrowed(&geometryrealization), ::windows_core::from_raw_borrowed(&brush)))
        }
        ID2D1DeviceContext1_Vtbl {
            base__: <ID2D1DeviceContext as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFilledGeometryRealization: CreateFilledGeometryRealization::<Identity, Impl, OFFSET>,
            CreateStrokedGeometryRealization: CreateStrokedGeometryRealization::<Identity, Impl, OFFSET>,
            DrawGeometryRealization: DrawGeometryRealization::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext2_Impl: ::windows_core::BaseImpl + ID2D1DeviceContext1_Impl {
    fn CreateInk(this: &Self::This, startpoint: *const D2D1_INK_POINT) -> ::windows_core::Result<ID2D1Ink>;
    fn CreateInkStyle(this: &Self::This, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES) -> ::windows_core::Result<ID2D1InkStyle>;
    fn CreateGradientMesh(this: &Self::This, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> ::windows_core::Result<ID2D1GradientMesh>;
    fn CreateImageSourceFromWic(this: &Self::This, wicbitmapsource: ::core::option::Option<&super::Imaging::IWICBitmapSource>, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> ::windows_core::Result<ID2D1ImageSourceFromWic>;
    fn CreateLookupTable3D(this: &Self::This, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32) -> ::windows_core::Result<ID2D1LookupTable3D>;
    fn CreateImageSourceFromDxgi(this: &Self::This, surfaces: *const ::core::option::Option<super::Dxgi::IDXGISurface>, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> ::windows_core::Result<ID2D1ImageSource>;
    fn GetGradientMeshWorldBounds(this: &Self::This, gradientmesh: ::core::option::Option<&ID2D1GradientMesh>) -> ::windows_core::Result<Common::D2D_RECT_F>;
    fn DrawInk(this: &Self::This, ink: ::core::option::Option<&ID2D1Ink>, brush: ::core::option::Option<&ID2D1Brush>, inkstyle: ::core::option::Option<&ID2D1InkStyle>);
    fn DrawGradientMesh(this: &Self::This, gradientmesh: ::core::option::Option<&ID2D1GradientMesh>);
    fn DrawGdiMetafile2(this: &Self::This, gdimetafile: ::core::option::Option<&ID2D1GdiMetafile>, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F);
    fn CreateTransformedImageSource(this: &Self::This, imagesource: ::core::option::Option<&ID2D1ImageSource>, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> ::windows_core::Result<ID2D1TransformedImageSource>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1DeviceContext2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1DeviceContext1);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DeviceContext2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT, ink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInk(this, ::core::mem::transmute_copy(&startpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInkStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES, inkstyle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInkStyle(this, ::core::mem::transmute_copy(&inkstyleproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(inkstyle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGradientMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32, gradientmesh: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGradientMesh(this, ::core::mem::transmute_copy(&patches), ::core::mem::transmute_copy(&patchescount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientmesh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateImageSourceFromWic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::core::ffi::c_void, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE, imagesource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateImageSourceFromWic(this, ::windows_core::from_raw_borrowed(&wicbitmapsource), ::core::mem::transmute_copy(&loadingoptions), ::core::mem::transmute_copy(&alphamode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagesource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLookupTable3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLookupTable3D(this, ::core::mem::transmute_copy(&precision), ::core::mem::transmute_copy(&extents), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&strides)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookuptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateImageSourceFromDxgi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surfaces: *const *mut ::core::ffi::c_void, surfacecount: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, imagesource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateImageSourceFromDxgi(this, ::core::mem::transmute_copy(&surfaces), ::core::mem::transmute_copy(&surfacecount), ::core::mem::transmute_copy(&colorspace), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagesource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGradientMeshWorldBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientmesh: *mut ::core::ffi::c_void, pbounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGradientMeshWorldBounds(this, ::windows_core::from_raw_borrowed(&gradientmesh)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawInk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ink: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInk(this, ::windows_core::from_raw_borrowed(&ink), ::windows_core::from_raw_borrowed(&brush), ::windows_core::from_raw_borrowed(&inkstyle)))
        }
        unsafe extern "system" fn DrawGradientMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientmesh: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGradientMesh(this, ::windows_core::from_raw_borrowed(&gradientmesh)))
        }
        unsafe extern "system" fn DrawGdiMetafile2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdimetafile: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGdiMetafile2(this, ::windows_core::from_raw_borrowed(&gdimetafile), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&sourcerectangle)))
        }
        unsafe extern "system" fn CreateTransformedImageSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagesource: *mut ::core::ffi::c_void, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, transformedimagesource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransformedImageSource(this, ::windows_core::from_raw_borrowed(&imagesource), ::core::mem::transmute_copy(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformedimagesource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1DeviceContext2_Vtbl {
            base__: <ID2D1DeviceContext1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInk: CreateInk::<Identity, Impl, OFFSET>,
            CreateInkStyle: CreateInkStyle::<Identity, Impl, OFFSET>,
            CreateGradientMesh: CreateGradientMesh::<Identity, Impl, OFFSET>,
            CreateImageSourceFromWic: CreateImageSourceFromWic::<Identity, Impl, OFFSET>,
            CreateLookupTable3D: CreateLookupTable3D::<Identity, Impl, OFFSET>,
            CreateImageSourceFromDxgi: CreateImageSourceFromDxgi::<Identity, Impl, OFFSET>,
            GetGradientMeshWorldBounds: GetGradientMeshWorldBounds::<Identity, Impl, OFFSET>,
            DrawInk: DrawInk::<Identity, Impl, OFFSET>,
            DrawGradientMesh: DrawGradientMesh::<Identity, Impl, OFFSET>,
            DrawGdiMetafile2: DrawGdiMetafile2::<Identity, Impl, OFFSET>,
            CreateTransformedImageSource: CreateTransformedImageSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext3_Impl: ::windows_core::BaseImpl + ID2D1DeviceContext2_Impl {
    fn CreateSpriteBatch(this: &Self::This) -> ::windows_core::Result<ID2D1SpriteBatch>;
    fn DrawSpriteBatch(this: &Self::This, spritebatch: ::core::option::Option<&ID2D1SpriteBatch>, startindex: u32, spritecount: u32, bitmap: ::core::option::Option<&ID2D1Bitmap>, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1DeviceContext3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1DeviceContext2);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DeviceContext3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSpriteBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritebatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSpriteBatch(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(spritebatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawSpriteBatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritebatch: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut ::core::ffi::c_void, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawSpriteBatch(this, ::windows_core::from_raw_borrowed(&spritebatch), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&spritecount), ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&interpolationmode), ::core::mem::transmute_copy(&spriteoptions)))
        }
        ID2D1DeviceContext3_Vtbl {
            base__: <ID2D1DeviceContext2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSpriteBatch: CreateSpriteBatch::<Identity, Impl, OFFSET>,
            DrawSpriteBatch: DrawSpriteBatch::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1DeviceContext4_Impl: ::windows_core::BaseImpl + ID2D1DeviceContext3_Impl {
    fn CreateSvgGlyphStyle(this: &Self::This) -> ::windows_core::Result<ID2D1SvgGlyphStyle>;
    fn DrawText2(this: &Self::This, string: &::windows_core::PCWSTR, stringlength: u32, textformat: ::core::option::Option<&super::DirectWrite::IDWriteTextFormat>, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: ::core::option::Option<&ID2D1Brush>, svgglyphstyle: ::core::option::Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn DrawTextLayout2(this: &Self::This, origin: &Common::D2D_POINT_2F, textlayout: ::core::option::Option<&super::DirectWrite::IDWriteTextLayout>, defaultfillbrush: ::core::option::Option<&ID2D1Brush>, svgglyphstyle: ::core::option::Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS);
    fn DrawColorBitmapGlyphRun(this: &Self::This, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION);
    fn DrawSvgGlyphRun(this: &Self::This, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: ::core::option::Option<&ID2D1Brush>, svgglyphstyle: ::core::option::Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn GetColorBitmapGlyphImage(this: &Self::This, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: &Common::D2D_POINT_2F, fontface: ::core::option::Option<&super::DirectWrite::IDWriteFontFace>, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1Image>) -> ::windows_core::Result<()>;
    fn GetSvgGlyphImage(this: &Self::This, glyphorigin: &Common::D2D_POINT_2F, fontface: ::core::option::Option<&super::DirectWrite::IDWriteFontFace>, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: ::core::option::Option<&ID2D1Brush>, svgglyphstyle: ::core::option::Option<&ID2D1SvgGlyphStyle>, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1CommandList>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1DeviceContext4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1DeviceContext3);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DeviceContext4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSvgGlyphStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, svgglyphstyle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSvgGlyphStyle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(svgglyphstyle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawText2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: ::windows_core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawText2(this, ::core::mem::transmute(&string), ::core::mem::transmute_copy(&stringlength), ::windows_core::from_raw_borrowed(&textformat), ::core::mem::transmute_copy(&layoutrect), ::windows_core::from_raw_borrowed(&defaultfillbrush), ::windows_core::from_raw_borrowed(&svgglyphstyle), ::core::mem::transmute_copy(&colorpaletteindex), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&measuringmode)))
        }
        unsafe extern "system" fn DrawTextLayout2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: *mut ::core::ffi::c_void, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawTextLayout2(this, ::core::mem::transmute(&origin), ::windows_core::from_raw_borrowed(&textlayout), ::windows_core::from_raw_borrowed(&defaultfillbrush), ::windows_core::from_raw_borrowed(&svgglyphstyle), ::core::mem::transmute_copy(&colorpaletteindex), ::core::mem::transmute_copy(&options)))
        }
        unsafe extern "system" fn DrawColorBitmapGlyphRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawColorBitmapGlyphRun(this, ::core::mem::transmute_copy(&glyphimageformat), ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&bitmapsnapoption)))
        }
        unsafe extern "system" fn DrawSvgGlyphRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawSvgGlyphRun(this, ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::windows_core::from_raw_borrowed(&defaultfillbrush), ::windows_core::from_raw_borrowed(&svgglyphstyle), ::core::mem::transmute_copy(&colorpaletteindex), ::core::mem::transmute_copy(&measuringmode)))
        }
        unsafe extern "system" fn GetColorBitmapGlyphImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: *mut ::core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetColorBitmapGlyphImage(this, ::core::mem::transmute_copy(&glyphimageformat), ::core::mem::transmute(&glyphorigin), ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute_copy(&glyphindex), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&glyphtransform), ::core::mem::transmute_copy(&glyphimage)).into()
            })
        }
        unsafe extern "system" fn GetSvgGlyphImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphorigin: Common::D2D_POINT_2F, fontface: *mut ::core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: super::super::Foundation::BOOL, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, defaultfillbrush: *mut ::core::ffi::c_void, svgglyphstyle: *mut ::core::ffi::c_void, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetSvgGlyphImage(
                    this,
                    ::core::mem::transmute(&glyphorigin),
                    ::windows_core::from_raw_borrowed(&fontface),
                    ::core::mem::transmute_copy(&fontemsize),
                    ::core::mem::transmute_copy(&glyphindex),
                    ::core::mem::transmute_copy(&issideways),
                    ::core::mem::transmute_copy(&worldtransform),
                    ::windows_core::from_raw_borrowed(&defaultfillbrush),
                    ::windows_core::from_raw_borrowed(&svgglyphstyle),
                    ::core::mem::transmute_copy(&colorpaletteindex),
                    ::core::mem::transmute_copy(&glyphtransform),
                    ::core::mem::transmute_copy(&glyphimage),
                )
                .into()
            })
        }
        ID2D1DeviceContext4_Vtbl {
            base__: <ID2D1DeviceContext3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSvgGlyphStyle: CreateSvgGlyphStyle::<Identity, Impl, OFFSET>,
            DrawText2: DrawText2::<Identity, Impl, OFFSET>,
            DrawTextLayout2: DrawTextLayout2::<Identity, Impl, OFFSET>,
            DrawColorBitmapGlyphRun: DrawColorBitmapGlyphRun::<Identity, Impl, OFFSET>,
            DrawSvgGlyphRun: DrawSvgGlyphRun::<Identity, Impl, OFFSET>,
            GetColorBitmapGlyphImage: GetColorBitmapGlyphImage::<Identity, Impl, OFFSET>,
            GetSvgGlyphImage: GetSvgGlyphImage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1DeviceContext5_Impl: ::windows_core::BaseImpl + ID2D1DeviceContext4_Impl {
    fn CreateSvgDocument(this: &Self::This, inputxmlstream: ::core::option::Option<&super::super::System::Com::IStream>, viewportsize: &Common::D2D_SIZE_F) -> ::windows_core::Result<ID2D1SvgDocument>;
    fn DrawSvgDocument(this: &Self::This, svgdocument: ::core::option::Option<&ID2D1SvgDocument>);
    fn CreateColorContextFromDxgiColorSpace(this: &Self::This, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<ID2D1ColorContext1>;
    fn CreateColorContextFromSimpleColorProfile(this: &Self::This, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> ::windows_core::Result<ID2D1ColorContext1>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1DeviceContext5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1DeviceContext4);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DeviceContext5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSvgDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputxmlstream: *mut ::core::ffi::c_void, viewportsize: Common::D2D_SIZE_F, svgdocument: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSvgDocument(this, ::windows_core::from_raw_borrowed(&inputxmlstream), ::core::mem::transmute(&viewportsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(svgdocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawSvgDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, svgdocument: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawSvgDocument(this, ::windows_core::from_raw_borrowed(&svgdocument)))
        }
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromDxgiColorSpace(this, ::core::mem::transmute_copy(&colorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromSimpleColorProfile(this, ::core::mem::transmute_copy(&simpleprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1DeviceContext5_Vtbl {
            base__: <ID2D1DeviceContext4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSvgDocument: CreateSvgDocument::<Identity, Impl, OFFSET>,
            DrawSvgDocument: DrawSvgDocument::<Identity, Impl, OFFSET>,
            CreateColorContextFromDxgiColorSpace: CreateColorContextFromDxgiColorSpace::<Identity, Impl, OFFSET>,
            CreateColorContextFromSimpleColorProfile: CreateColorContextFromSimpleColorProfile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1DeviceContext6_Impl: ::windows_core::BaseImpl + ID2D1DeviceContext5_Impl {
    fn BlendImage(this: &Self::This, image: ::core::option::Option<&ID2D1Image>, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1DeviceContext6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1DeviceContext5);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DeviceContext6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BlendImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DeviceContext6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, blendmode: Common::D2D1_BLEND_MODE, targetoffset: *const Common::D2D_POINT_2F, imagerectangle: *const Common::D2D_RECT_F, interpolationmode: D2D1_INTERPOLATION_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BlendImage(this, ::windows_core::from_raw_borrowed(&image), ::core::mem::transmute_copy(&blendmode), ::core::mem::transmute_copy(&targetoffset), ::core::mem::transmute_copy(&imagerectangle), ::core::mem::transmute_copy(&interpolationmode)))
        }
        ID2D1DeviceContext6_Vtbl { base__: <ID2D1DeviceContext5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, BlendImage: BlendImage::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1DrawInfo_Impl: ::windows_core::BaseImpl + ID2D1RenderInfo_Impl {
    fn SetPixelShaderConstantBuffer(this: &Self::This, buffer: *const u8, buffercount: u32) -> ::windows_core::Result<()>;
    fn SetResourceTexture(this: &Self::This, textureindex: u32, resourcetexture: ::core::option::Option<&ID2D1ResourceTexture>) -> ::windows_core::Result<()>;
    fn SetVertexShaderConstantBuffer(this: &Self::This, buffer: *const u8, buffercount: u32) -> ::windows_core::Result<()>;
    fn SetPixelShader(this: &Self::This, shaderid: *const ::windows_core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> ::windows_core::Result<()>;
    fn SetVertexProcessing(this: &Self::This, vertexbuffer: ::core::option::Option<&ID2D1VertexBuffer>, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1DrawInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1RenderInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DrawInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPixelShaderConstantBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelShaderConstantBuffer(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffercount)).into())
        }
        unsafe extern "system" fn SetResourceTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textureindex: u32, resourcetexture: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResourceTexture(this, ::core::mem::transmute_copy(&textureindex), ::windows_core::from_raw_borrowed(&resourcetexture)).into())
        }
        unsafe extern "system" fn SetVertexShaderConstantBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *const u8, buffercount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVertexShaderConstantBuffer(this, ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffercount)).into())
        }
        unsafe extern "system" fn SetPixelShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows_core::GUID, pixeloptions: D2D1_PIXEL_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelShader(this, ::core::mem::transmute_copy(&shaderid), ::core::mem::transmute_copy(&pixeloptions)).into())
        }
        unsafe extern "system" fn SetVertexProcessing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexbuffer: *mut ::core::ffi::c_void, vertexoptions: D2D1_VERTEX_OPTIONS, blenddescription: *const D2D1_BLEND_DESCRIPTION, vertexrange: *const D2D1_VERTEX_RANGE, vertexshader: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVertexProcessing(this, ::windows_core::from_raw_borrowed(&vertexbuffer), ::core::mem::transmute_copy(&vertexoptions), ::core::mem::transmute_copy(&blenddescription), ::core::mem::transmute_copy(&vertexrange), ::core::mem::transmute_copy(&vertexshader)).into())
        }
        ID2D1DrawInfo_Vtbl {
            base__: <ID2D1RenderInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPixelShaderConstantBuffer: SetPixelShaderConstantBuffer::<Identity, Impl, OFFSET>,
            SetResourceTexture: SetResourceTexture::<Identity, Impl, OFFSET>,
            SetVertexShaderConstantBuffer: SetVertexShaderConstantBuffer::<Identity, Impl, OFFSET>,
            SetPixelShader: SetPixelShader::<Identity, Impl, OFFSET>,
            SetVertexProcessing: SetVertexProcessing::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1DrawTransform_Impl: ::windows_core::BaseImpl + ID2D1Transform_Impl {
    fn SetDrawInfo(this: &Self::This, drawinfo: ::core::option::Option<&ID2D1DrawInfo>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1DrawTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Transform);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DrawTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDrawInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDrawInfo(this, ::windows_core::from_raw_borrowed(&drawinfo)).into())
        }
        ID2D1DrawTransform_Vtbl { base__: <ID2D1Transform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetDrawInfo: SetDrawInfo::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1DrawingStateBlock_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetDescription(this: &Self::This, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION);
    fn SetDescription(this: &Self::This, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION);
    fn SetTextRenderingParams(this: &Self::This, textrenderingparams: ::core::option::Option<&super::DirectWrite::IDWriteRenderingParams>);
    fn GetTextRenderingParams(this: &Self::This, textrenderingparams: *mut ::core::option::Option<super::DirectWrite::IDWriteRenderingParams>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1DrawingStateBlock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DrawingStateBlock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescription(this, ::core::mem::transmute_copy(&statedescription)))
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute_copy(&statedescription)))
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextRenderingParams(this, ::windows_core::from_raw_borrowed(&textrenderingparams)))
        }
        unsafe extern "system" fn GetTextRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextRenderingParams(this, ::core::mem::transmute_copy(&textrenderingparams)))
        }
        ID2D1DrawingStateBlock_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, Impl, OFFSET>,
            GetTextRenderingParams: GetTextRenderingParams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
pub trait ID2D1DrawingStateBlock1_Impl: ::windows_core::BaseImpl + ID2D1DrawingStateBlock_Impl {
    fn GetDescription2(this: &Self::This, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1);
    fn SetDescription2(this: &Self::This, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl ::windows_core::Iids for ID2D1DrawingStateBlock1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1DrawingStateBlock);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1DrawingStateBlock1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDescription2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescription2(this, ::core::mem::transmute_copy(&statedescription)))
        }
        unsafe extern "system" fn SetDescription2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1DrawingStateBlock1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription2(this, ::core::mem::transmute_copy(&statedescription)))
        }
        ID2D1DrawingStateBlock1_Vtbl {
            base__: <ID2D1DrawingStateBlock as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDescription2: GetDescription2::<Identity, Impl, OFFSET>,
            SetDescription2: SetDescription2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1Effect_Impl: ::windows_core::BaseImpl + ID2D1Properties_Impl {
    fn SetInput(this: &Self::This, index: u32, input: ::core::option::Option<&ID2D1Image>, invalidate: super::super::Foundation::BOOL);
    fn SetInputCount(this: &Self::This, inputcount: u32) -> ::windows_core::Result<()>;
    fn GetInput(this: &Self::This, index: u32, input: *mut ::core::option::Option<ID2D1Image>);
    fn GetInputCount(this: &Self::This) -> u32;
    fn GetOutput(this: &Self::This, outputimage: *mut ::core::option::Option<ID2D1Image>);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1Effect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Properties);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Effect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Effect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, invalidate: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInput(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&input), ::core::mem::transmute_copy(&invalidate)))
        }
        unsafe extern "system" fn SetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputCount(this, ::core::mem::transmute_copy(&inputcount)).into())
        }
        unsafe extern "system" fn GetInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, input: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInput(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&input)))
        }
        unsafe extern "system" fn GetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputCount(this))
        }
        unsafe extern "system" fn GetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Effect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputimage: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutput(this, ::core::mem::transmute_copy(&outputimage)))
        }
        ID2D1Effect_Vtbl {
            base__: <ID2D1Properties as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInput: SetInput::<Identity, Impl, OFFSET>,
            SetInputCount: SetInputCount::<Identity, Impl, OFFSET>,
            GetInput: GetInput::<Identity, Impl, OFFSET>,
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext_Impl: ::windows_core::BaseImpl {
    fn GetDpi(this: &Self::This, dpix: *mut f32, dpiy: *mut f32);
    fn CreateEffect(this: &Self::This, effectid: *const ::windows_core::GUID) -> ::windows_core::Result<ID2D1Effect>;
    fn GetMaximumSupportedFeatureLevel(this: &Self::This, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32) -> ::windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn CreateTransformNodeFromEffect(this: &Self::This, effect: ::core::option::Option<&ID2D1Effect>) -> ::windows_core::Result<ID2D1TransformNode>;
    fn CreateBlendTransform(this: &Self::This, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION) -> ::windows_core::Result<ID2D1BlendTransform>;
    fn CreateBorderTransform(this: &Self::This, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE) -> ::windows_core::Result<ID2D1BorderTransform>;
    fn CreateOffsetTransform(this: &Self::This, offset: &super::super::Foundation::POINT) -> ::windows_core::Result<ID2D1OffsetTransform>;
    fn CreateBoundsAdjustmentTransform(this: &Self::This, outputrectangle: *const super::super::Foundation::RECT) -> ::windows_core::Result<ID2D1BoundsAdjustmentTransform>;
    fn LoadPixelShader(this: &Self::This, shaderid: *const ::windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_core::Result<()>;
    fn LoadVertexShader(this: &Self::This, resourceid: *const ::windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_core::Result<()>;
    fn LoadComputeShader(this: &Self::This, resourceid: *const ::windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_core::Result<()>;
    fn IsShaderLoaded(this: &Self::This, shaderid: *const ::windows_core::GUID) -> super::super::Foundation::BOOL;
    fn CreateResourceTexture(this: &Self::This, resourceid: *const ::windows_core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32) -> ::windows_core::Result<ID2D1ResourceTexture>;
    fn FindResourceTexture(this: &Self::This, resourceid: *const ::windows_core::GUID) -> ::windows_core::Result<ID2D1ResourceTexture>;
    fn CreateVertexBuffer(this: &Self::This, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const ::windows_core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES) -> ::windows_core::Result<ID2D1VertexBuffer>;
    fn FindVertexBuffer(this: &Self::This, resourceid: *const ::windows_core::GUID) -> ::windows_core::Result<ID2D1VertexBuffer>;
    fn CreateColorContext(this: &Self::This, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32) -> ::windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromFilename(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<ID2D1ColorContext>;
    fn CreateColorContextFromWicColorContext(this: &Self::This, wiccolorcontext: ::core::option::Option<&super::Imaging::IWICColorContext>) -> ::windows_core::Result<ID2D1ColorContext>;
    fn CheckFeatureSupport(this: &Self::This, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::Result<()>;
    fn IsBufferPrecisionSupported(this: &Self::This, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1EffectContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1EffectContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDpi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDpi(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)))
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows_core::GUID, effect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEffect(this, ::core::mem::transmute_copy(&effectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(effect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaximumSupportedFeatureLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, featurelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevelscount: u32, maximumsupportedfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaximumSupportedFeatureLevel(this, ::core::mem::transmute_copy(&featurelevels), ::core::mem::transmute_copy(&featurelevelscount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maximumsupportedfeaturelevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTransformNodeFromEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, transformnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransformNodeFromEffect(this, ::windows_core::from_raw_borrowed(&effect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlendTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlendTransform(this, ::core::mem::transmute_copy(&numinputs), ::core::mem::transmute_copy(&blenddescription)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBorderTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBorderTransform(this, ::core::mem::transmute_copy(&extendmodex), ::core::mem::transmute_copy(&extendmodey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateOffsetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateOffsetTransform(this, ::core::mem::transmute(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBoundsAdjustmentTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputrectangle: *const super::super::Foundation::RECT, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBoundsAdjustmentTransform(this, ::core::mem::transmute_copy(&outputrectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadPixelShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadPixelShader(this, ::core::mem::transmute_copy(&shaderid), ::core::mem::transmute_copy(&shaderbuffer), ::core::mem::transmute_copy(&shaderbuffercount)).into())
        }
        unsafe extern "system" fn LoadVertexShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadVertexShader(this, ::core::mem::transmute_copy(&resourceid), ::core::mem::transmute_copy(&shaderbuffer), ::core::mem::transmute_copy(&shaderbuffercount)).into())
        }
        unsafe extern "system" fn LoadComputeShader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows_core::GUID, shaderbuffer: *const u8, shaderbuffercount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadComputeShader(this, ::core::mem::transmute_copy(&resourceid), ::core::mem::transmute_copy(&shaderbuffer), ::core::mem::transmute_copy(&shaderbuffercount)).into())
        }
        unsafe extern "system" fn IsShaderLoaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shaderid: *const ::windows_core::GUID) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsShaderLoaded(this, ::core::mem::transmute_copy(&shaderid)))
        }
        unsafe extern "system" fn CreateResourceTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows_core::GUID, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: *const u8, strides: *const u32, datasize: u32, resourcetexture: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateResourceTexture(this, ::core::mem::transmute_copy(&resourceid), ::core::mem::transmute_copy(&resourcetextureproperties), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&strides), ::core::mem::transmute_copy(&datasize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcetexture, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindResourceTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows_core::GUID, resourcetexture: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindResourceTexture(this, ::core::mem::transmute_copy(&resourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcetexture, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: *const ::windows_core::GUID, customvertexbufferproperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES, buffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVertexBuffer(this, ::core::mem::transmute_copy(&vertexbufferproperties), ::core::mem::transmute_copy(&resourceid), ::core::mem::transmute_copy(&customvertexbufferproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindVertexBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *const ::windows_core::GUID, buffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindVertexBuffer(this, ::core::mem::transmute_copy(&resourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, space: D2D1_COLOR_SPACE, profile: *const u8, profilesize: u32, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContext(this, ::core::mem::transmute_copy(&space), ::core::mem::transmute_copy(&profile), ::core::mem::transmute_copy(&profilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContextFromFilename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromFilename(this, ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContextFromWicColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wiccolorcontext: *mut ::core::ffi::c_void, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromWicColorContext(this, ::windows_core::from_raw_borrowed(&wiccolorcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckFeatureSupport(this, ::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&featuresupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into())
        }
        unsafe extern "system" fn IsBufferPrecisionSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsBufferPrecisionSupported(this, ::core::mem::transmute_copy(&bufferprecision)))
        }
        ID2D1EffectContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDpi: GetDpi::<Identity, Impl, OFFSET>,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            GetMaximumSupportedFeatureLevel: GetMaximumSupportedFeatureLevel::<Identity, Impl, OFFSET>,
            CreateTransformNodeFromEffect: CreateTransformNodeFromEffect::<Identity, Impl, OFFSET>,
            CreateBlendTransform: CreateBlendTransform::<Identity, Impl, OFFSET>,
            CreateBorderTransform: CreateBorderTransform::<Identity, Impl, OFFSET>,
            CreateOffsetTransform: CreateOffsetTransform::<Identity, Impl, OFFSET>,
            CreateBoundsAdjustmentTransform: CreateBoundsAdjustmentTransform::<Identity, Impl, OFFSET>,
            LoadPixelShader: LoadPixelShader::<Identity, Impl, OFFSET>,
            LoadVertexShader: LoadVertexShader::<Identity, Impl, OFFSET>,
            LoadComputeShader: LoadComputeShader::<Identity, Impl, OFFSET>,
            IsShaderLoaded: IsShaderLoaded::<Identity, Impl, OFFSET>,
            CreateResourceTexture: CreateResourceTexture::<Identity, Impl, OFFSET>,
            FindResourceTexture: FindResourceTexture::<Identity, Impl, OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Identity, Impl, OFFSET>,
            FindVertexBuffer: FindVertexBuffer::<Identity, Impl, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, Impl, OFFSET>,
            CreateColorContextFromFilename: CreateColorContextFromFilename::<Identity, Impl, OFFSET>,
            CreateColorContextFromWicColorContext: CreateColorContextFromWicColorContext::<Identity, Impl, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            IsBufferPrecisionSupported: IsBufferPrecisionSupported::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext1_Impl: ::windows_core::BaseImpl + ID2D1EffectContext_Impl {
    fn CreateLookupTable3D(this: &Self::This, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32) -> ::windows_core::Result<ID2D1LookupTable3D>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1EffectContext1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1EffectContext);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1EffectContext1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLookupTable3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precision: D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLookupTable3D(this, ::core::mem::transmute_copy(&precision), ::core::mem::transmute_copy(&extents), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&strides)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookuptable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1EffectContext1_Vtbl {
            base__: <ID2D1EffectContext as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateLookupTable3D: CreateLookupTable3D::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1EffectContext2_Impl: ::windows_core::BaseImpl + ID2D1EffectContext1_Impl {
    fn CreateColorContextFromDxgiColorSpace(this: &Self::This, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<ID2D1ColorContext1>;
    fn CreateColorContextFromSimpleColorProfile(this: &Self::This, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> ::windows_core::Result<ID2D1ColorContext1>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1EffectContext2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1EffectContext1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1EffectContext2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromDxgiColorSpace(this, ::core::mem::transmute_copy(&colorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectContext2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContextFromSimpleColorProfile(this, ::core::mem::transmute_copy(&simpleprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1EffectContext2_Vtbl {
            base__: <ID2D1EffectContext1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateColorContextFromDxgiColorSpace: CreateColorContextFromDxgiColorSpace::<Identity, Impl, OFFSET>,
            CreateColorContextFromSimpleColorProfile: CreateColorContextFromSimpleColorProfile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1EffectImpl_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, effectcontext: ::core::option::Option<&ID2D1EffectContext>, transformgraph: ::core::option::Option<&ID2D1TransformGraph>) -> ::windows_core::Result<()>;
    fn PrepareForRender(this: &Self::This, changetype: D2D1_CHANGE_TYPE) -> ::windows_core::Result<()>;
    fn SetGraph(this: &Self::This, transformgraph: ::core::option::Option<&ID2D1TransformGraph>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1EffectImpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectImpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1EffectImpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectcontext: *mut ::core::ffi::c_void, transformgraph: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&effectcontext), ::windows_core::from_raw_borrowed(&transformgraph)).into())
        }
        unsafe extern "system" fn PrepareForRender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: D2D1_CHANGE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareForRender(this, ::core::mem::transmute_copy(&changetype)).into())
        }
        unsafe extern "system" fn SetGraph<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EffectImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transformgraph: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGraph(this, ::windows_core::from_raw_borrowed(&transformgraph)).into())
        }
        ID2D1EffectImpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareForRender: PrepareForRender::<Identity, Impl, OFFSET>,
            SetGraph: SetGraph::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1EllipseGeometry_Impl: ::windows_core::BaseImpl + ID2D1Geometry_Impl {
    fn GetEllipse(this: &Self::This, ellipse: *mut D2D1_ELLIPSE);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1EllipseGeometry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Geometry);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EllipseGeometry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1EllipseGeometry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEllipse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1EllipseGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *mut D2D1_ELLIPSE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEllipse(this, ::core::mem::transmute_copy(&ellipse)))
        }
        ID2D1EllipseGeometry_Vtbl { base__: <ID2D1Geometry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetEllipse: GetEllipse::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1Factory_Impl: ::windows_core::BaseImpl {
    fn ReloadSystemMetrics(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDesktopDpi(this: &Self::This, dpix: *mut f32, dpiy: *mut f32);
    fn CreateRectangleGeometry(this: &Self::This, rectangle: *const Common::D2D_RECT_F) -> ::windows_core::Result<ID2D1RectangleGeometry>;
    fn CreateRoundedRectangleGeometry(this: &Self::This, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows_core::Result<ID2D1RoundedRectangleGeometry>;
    fn CreateEllipseGeometry(this: &Self::This, ellipse: *const D2D1_ELLIPSE) -> ::windows_core::Result<ID2D1EllipseGeometry>;
    fn CreateGeometryGroup(this: &Self::This, fillmode: Common::D2D1_FILL_MODE, geometries: *const ::core::option::Option<ID2D1Geometry>, geometriescount: u32) -> ::windows_core::Result<ID2D1GeometryGroup>;
    fn CreateTransformedGeometry(this: &Self::This, sourcegeometry: ::core::option::Option<&ID2D1Geometry>, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<ID2D1TransformedGeometry>;
    fn CreatePathGeometry(this: &Self::This) -> ::windows_core::Result<ID2D1PathGeometry>;
    fn CreateStrokeStyle(this: &Self::This, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32) -> ::windows_core::Result<ID2D1StrokeStyle>;
    fn CreateDrawingStateBlock(this: &Self::This, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: ::core::option::Option<&super::DirectWrite::IDWriteRenderingParams>) -> ::windows_core::Result<ID2D1DrawingStateBlock>;
    fn CreateWicBitmapRenderTarget(this: &Self::This, target: ::core::option::Option<&super::Imaging::IWICBitmap>, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows_core::Result<ID2D1RenderTarget>;
    fn CreateHwndRenderTarget(this: &Self::This, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows_core::Result<ID2D1HwndRenderTarget>;
    fn CreateDxgiSurfaceRenderTarget(this: &Self::This, dxgisurface: ::core::option::Option<&super::Dxgi::IDXGISurface>, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows_core::Result<ID2D1RenderTarget>;
    fn CreateDCRenderTarget(this: &Self::This, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows_core::Result<ID2D1DCRenderTarget>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1Factory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReloadSystemMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReloadSystemMetrics(this).into())
        }
        unsafe extern "system" fn GetDesktopDpi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesktopDpi(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)))
        }
        unsafe extern "system" fn CreateRectangleGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rectangle: *const Common::D2D_RECT_F, rectanglegeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRectangleGeometry(this, ::core::mem::transmute_copy(&rectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rectanglegeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrectangle: *const D2D1_ROUNDED_RECT, roundedrectanglegeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRoundedRectangleGeometry(this, ::core::mem::transmute_copy(&roundedrectangle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(roundedrectanglegeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEllipseGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, ellipsegeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEllipseGeometry(this, ::core::mem::transmute_copy(&ellipse)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ellipsegeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGeometryGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, geometries: *const *mut ::core::ffi::c_void, geometriescount: u32, geometrygroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGeometryGroup(this, ::core::mem::transmute_copy(&fillmode), ::core::mem::transmute_copy(&geometries), ::core::mem::transmute_copy(&geometriescount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometrygroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTransformedGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcegeometry: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2, transformedgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTransformedGeometry(this, ::windows_core::from_raw_borrowed(&sourcegeometry), ::core::mem::transmute_copy(&transform)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transformedgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePathGeometry(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pathgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStrokeStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStrokeStyle(this, ::core::mem::transmute_copy(&strokestyleproperties), ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokestyle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDrawingStateBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION, textrenderingparams: *mut ::core::ffi::c_void, drawingstateblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDrawingStateBlock(this, ::core::mem::transmute_copy(&drawingstatedescription), ::windows_core::from_raw_borrowed(&textrenderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(drawingstateblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateWicBitmapRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateWicBitmapRenderTarget(this, ::windows_core::from_raw_borrowed(&target), ::core::mem::transmute_copy(&rendertargetproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rendertarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateHwndRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES, hwndrendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateHwndRenderTarget(this, ::core::mem::transmute_copy(&rendertargetproperties), ::core::mem::transmute_copy(&hwndrendertargetproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hwndrendertarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDxgiSurfaceRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgisurface: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDxgiSurfaceRenderTarget(this, ::windows_core::from_raw_borrowed(&dxgisurface), ::core::mem::transmute_copy(&rendertargetproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rendertarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDCRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, dcrendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDCRenderTarget(this, ::core::mem::transmute_copy(&rendertargetproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dcrendertarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReloadSystemMetrics: ReloadSystemMetrics::<Identity, Impl, OFFSET>,
            GetDesktopDpi: GetDesktopDpi::<Identity, Impl, OFFSET>,
            CreateRectangleGeometry: CreateRectangleGeometry::<Identity, Impl, OFFSET>,
            CreateRoundedRectangleGeometry: CreateRoundedRectangleGeometry::<Identity, Impl, OFFSET>,
            CreateEllipseGeometry: CreateEllipseGeometry::<Identity, Impl, OFFSET>,
            CreateGeometryGroup: CreateGeometryGroup::<Identity, Impl, OFFSET>,
            CreateTransformedGeometry: CreateTransformedGeometry::<Identity, Impl, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, Impl, OFFSET>,
            CreateStrokeStyle: CreateStrokeStyle::<Identity, Impl, OFFSET>,
            CreateDrawingStateBlock: CreateDrawingStateBlock::<Identity, Impl, OFFSET>,
            CreateWicBitmapRenderTarget: CreateWicBitmapRenderTarget::<Identity, Impl, OFFSET>,
            CreateHwndRenderTarget: CreateHwndRenderTarget::<Identity, Impl, OFFSET>,
            CreateDxgiSurfaceRenderTarget: CreateDxgiSurfaceRenderTarget::<Identity, Impl, OFFSET>,
            CreateDCRenderTarget: CreateDCRenderTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory1_Impl: ::windows_core::BaseImpl + ID2D1Factory_Impl {
    fn CreateDevice(this: &Self::This, dxgidevice: ::core::option::Option<&super::Dxgi::IDXGIDevice>) -> ::windows_core::Result<ID2D1Device>;
    fn CreateStrokeStyle2(this: &Self::This, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32) -> ::windows_core::Result<ID2D1StrokeStyle1>;
    fn CreatePathGeometry2(this: &Self::This) -> ::windows_core::Result<ID2D1PathGeometry1>;
    fn CreateDrawingStateBlock2(this: &Self::This, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: ::core::option::Option<&super::DirectWrite::IDWriteRenderingParams>) -> ::windows_core::Result<ID2D1DrawingStateBlock1>;
    fn CreateGdiMetafile(this: &Self::This, metafilestream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<ID2D1GdiMetafile>;
    fn RegisterEffectFromStream(this: &Self::This, classid: *const ::windows_core::GUID, propertyxml: ::core::option::Option<&super::super::System::Com::IStream>, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows_core::Result<()>;
    fn RegisterEffectFromString(this: &Self::This, classid: *const ::windows_core::GUID, propertyxml: &::windows_core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows_core::Result<()>;
    fn UnregisterEffect(this: &Self::This, classid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetRegisteredEffects(this: &Self::This, effects: *mut ::windows_core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> ::windows_core::Result<()>;
    fn GetEffectProperties(this: &Self::This, effectid: *const ::windows_core::GUID) -> ::windows_core::Result<ID2D1Properties>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1Factory1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Factory);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice(this, ::windows_core::from_raw_borrowed(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(d2ddevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStrokeStyle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: *const f32, dashescount: u32, strokestyle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStrokeStyle2(this, ::core::mem::transmute_copy(&strokestyleproperties), ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokestyle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePathGeometry2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePathGeometry2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pathgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDrawingStateBlock2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1, textrenderingparams: *mut ::core::ffi::c_void, drawingstateblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDrawingStateBlock2(this, ::core::mem::transmute_copy(&drawingstatedescription), ::windows_core::from_raw_borrowed(&textrenderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(drawingstateblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGdiMetafile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metafilestream: *mut ::core::ffi::c_void, metafile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGdiMetafile(this, ::windows_core::from_raw_borrowed(&metafilestream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metafile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterEffectFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: *const ::windows_core::GUID, propertyxml: *mut ::core::ffi::c_void, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterEffectFromStream(this, ::core::mem::transmute_copy(&classid), ::windows_core::from_raw_borrowed(&propertyxml), ::core::mem::transmute_copy(&bindings), ::core::mem::transmute_copy(&bindingscount), ::core::mem::transmute_copy(&effectfactory)).into())
        }
        unsafe extern "system" fn RegisterEffectFromString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: *const ::windows_core::GUID, propertyxml: ::windows_core::PCWSTR, bindings: *const D2D1_PROPERTY_BINDING, bindingscount: u32, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterEffectFromString(this, ::core::mem::transmute_copy(&classid), ::core::mem::transmute(&propertyxml), ::core::mem::transmute_copy(&bindings), ::core::mem::transmute_copy(&bindingscount), ::core::mem::transmute_copy(&effectfactory)).into())
        }
        unsafe extern "system" fn UnregisterEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterEffect(this, ::core::mem::transmute_copy(&classid)).into())
        }
        unsafe extern "system" fn GetRegisteredEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effects: *mut ::windows_core::GUID, effectscount: u32, effectsreturned: *mut u32, effectsregistered: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegisteredEffects(this, ::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&effectscount), ::core::mem::transmute_copy(&effectsreturned), ::core::mem::transmute_copy(&effectsregistered)).into())
        }
        unsafe extern "system" fn GetEffectProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectid: *const ::windows_core::GUID, properties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEffectProperties(this, ::core::mem::transmute_copy(&effectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory1_Vtbl {
            base__: <ID2D1Factory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            CreateStrokeStyle2: CreateStrokeStyle2::<Identity, Impl, OFFSET>,
            CreatePathGeometry2: CreatePathGeometry2::<Identity, Impl, OFFSET>,
            CreateDrawingStateBlock2: CreateDrawingStateBlock2::<Identity, Impl, OFFSET>,
            CreateGdiMetafile: CreateGdiMetafile::<Identity, Impl, OFFSET>,
            RegisterEffectFromStream: RegisterEffectFromStream::<Identity, Impl, OFFSET>,
            RegisterEffectFromString: RegisterEffectFromString::<Identity, Impl, OFFSET>,
            UnregisterEffect: UnregisterEffect::<Identity, Impl, OFFSET>,
            GetRegisteredEffects: GetRegisteredEffects::<Identity, Impl, OFFSET>,
            GetEffectProperties: GetEffectProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory2_Impl: ::windows_core::BaseImpl + ID2D1Factory1_Impl {
    fn CreateDevice2(this: &Self::This, dxgidevice: ::core::option::Option<&super::Dxgi::IDXGIDevice>) -> ::windows_core::Result<ID2D1Device1>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1Factory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Factory1);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice1: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice2(this, ::windows_core::from_raw_borrowed(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(d2ddevice1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory2_Vtbl { base__: <ID2D1Factory1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDevice2: CreateDevice2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory3_Impl: ::windows_core::BaseImpl + ID2D1Factory2_Impl {
    fn CreateDevice3(this: &Self::This, dxgidevice: ::core::option::Option<&super::Dxgi::IDXGIDevice>) -> ::windows_core::Result<ID2D1Device2>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1Factory3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Factory2);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice3(this, ::windows_core::from_raw_borrowed(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(d2ddevice2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory3_Vtbl { base__: <ID2D1Factory2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDevice3: CreateDevice3::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory4_Impl: ::windows_core::BaseImpl + ID2D1Factory3_Impl {
    fn CreateDevice4(this: &Self::This, dxgidevice: ::core::option::Option<&super::Dxgi::IDXGIDevice>) -> ::windows_core::Result<ID2D1Device3>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1Factory4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Factory3);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice3: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice4(this, ::windows_core::from_raw_borrowed(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(d2ddevice3, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory4_Vtbl { base__: <ID2D1Factory3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDevice4: CreateDevice4::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory5_Impl: ::windows_core::BaseImpl + ID2D1Factory4_Impl {
    fn CreateDevice5(this: &Self::This, dxgidevice: ::core::option::Option<&super::Dxgi::IDXGIDevice>) -> ::windows_core::Result<ID2D1Device4>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1Factory5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Factory4);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice5<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice4: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice5(this, ::windows_core::from_raw_borrowed(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(d2ddevice4, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory5_Vtbl { base__: <ID2D1Factory4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDevice5: CreateDevice5::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory6_Impl: ::windows_core::BaseImpl + ID2D1Factory5_Impl {
    fn CreateDevice6(this: &Self::This, dxgidevice: ::core::option::Option<&super::Dxgi::IDXGIDevice>) -> ::windows_core::Result<ID2D1Device5>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1Factory6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Factory5);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice6<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice5: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice6(this, ::windows_core::from_raw_borrowed(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(d2ddevice5, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory6_Vtbl { base__: <ID2D1Factory5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDevice6: CreateDevice6::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
pub trait ID2D1Factory7_Impl: ::windows_core::BaseImpl + ID2D1Factory6_Impl {
    fn CreateDevice7(this: &Self::This, dxgidevice: ::core::option::Option<&super::Dxgi::IDXGIDevice>) -> ::windows_core::Result<ID2D1Device6>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1Factory7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Factory6);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Factory7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDevice7<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Factory7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgidevice: *mut ::core::ffi::c_void, d2ddevice6: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDevice7(this, ::windows_core::from_raw_borrowed(&dxgidevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(d2ddevice6, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Factory7_Vtbl { base__: <ID2D1Factory6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateDevice7: CreateDevice7::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ID2D1GdiInteropRenderTarget_Impl: ::windows_core::BaseImpl {
    fn GetDC(this: &Self::This, mode: D2D1_DC_INITIALIZE_MODE) -> ::windows_core::Result<super::Gdi::HDC>;
    fn ReleaseDC(this: &Self::This, update: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for ID2D1GdiInteropRenderTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiInteropRenderTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GdiInteropRenderTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiInteropRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: D2D1_DC_INITIALIZE_MODE, hdc: *mut super::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDC(this, ::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiInteropRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, update: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&update)).into())
        }
        ID2D1GdiInteropRenderTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GdiMetafile_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn Stream(this: &Self::This, sink: ::core::option::Option<&ID2D1GdiMetafileSink>) -> ::windows_core::Result<()>;
    fn GetBounds(this: &Self::This) -> ::windows_core::Result<Common::D2D_RECT_F>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1GdiMetafile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GdiMetafile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Stream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stream(this, ::windows_core::from_raw_borrowed(&sink)).into())
        }
        unsafe extern "system" fn GetBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBounds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1GdiMetafile_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Stream: Stream::<Identity, Impl, OFFSET>,
            GetBounds: GetBounds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GdiMetafile1_Impl: ::windows_core::BaseImpl + ID2D1GdiMetafile_Impl {
    fn GetDpi(this: &Self::This, dpix: *mut f32, dpiy: *mut f32) -> ::windows_core::Result<()>;
    fn GetSourceBounds(this: &Self::This) -> ::windows_core::Result<Common::D2D_RECT_F>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1GdiMetafile1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1GdiMetafile);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafile1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GdiMetafile1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDpi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafile1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDpi(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into())
        }
        unsafe extern "system" fn GetSourceBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafile1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceBounds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1GdiMetafile1_Vtbl {
            base__: <ID2D1GdiMetafile as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDpi: GetDpi::<Identity, Impl, OFFSET>,
            GetSourceBounds: GetSourceBounds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1GdiMetafileSink_Impl: ::windows_core::BaseImpl {
    fn ProcessRecord(this: &Self::This, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1GdiMetafileSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafileSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GdiMetafileSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafileSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessRecord(this, ::core::mem::transmute_copy(&recordtype), ::core::mem::transmute_copy(&recorddata), ::core::mem::transmute_copy(&recorddatasize)).into())
        }
        ID2D1GdiMetafileSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ProcessRecord: ProcessRecord::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1GdiMetafileSink1_Impl: ::windows_core::BaseImpl + ID2D1GdiMetafileSink_Impl {
    fn ProcessRecord2(this: &Self::This, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32, flags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1GdiMetafileSink1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1GdiMetafileSink);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafileSink1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GdiMetafileSink1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessRecord2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GdiMetafileSink1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recordtype: u32, recorddata: *const ::core::ffi::c_void, recorddatasize: u32, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessRecord2(this, ::core::mem::transmute_copy(&recordtype), ::core::mem::transmute_copy(&recorddata), ::core::mem::transmute_copy(&recorddatasize), ::core::mem::transmute_copy(&flags)).into())
        }
        ID2D1GdiMetafileSink1_Vtbl { base__: <ID2D1GdiMetafileSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ProcessRecord2: ProcessRecord2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1Geometry_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetBounds(this: &Self::This, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<Common::D2D_RECT_F>;
    fn GetWidenedBounds(this: &Self::This, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> ::windows_core::Result<Common::D2D_RECT_F>;
    fn StrokeContainsPoint(this: &Self::This, point: &Common::D2D_POINT_2F, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn FillContainsPoint(this: &Self::This, point: &Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareWithGeometry(this: &Self::This, inputgeometry: ::core::option::Option<&ID2D1Geometry>, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> ::windows_core::Result<D2D1_GEOMETRY_RELATION>;
    fn Simplify(this: &Self::This, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::core::option::Option<&Common::ID2D1SimplifiedGeometrySink>) -> ::windows_core::Result<()>;
    fn Tessellate(this: &Self::This, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: ::core::option::Option<&ID2D1TessellationSink>) -> ::windows_core::Result<()>;
    fn CombineWithGeometry(this: &Self::This, inputgeometry: ::core::option::Option<&ID2D1Geometry>, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::core::option::Option<&Common::ID2D1SimplifiedGeometrySink>) -> ::windows_core::Result<()>;
    fn Outline(this: &Self::This, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::core::option::Option<&Common::ID2D1SimplifiedGeometrySink>) -> ::windows_core::Result<()>;
    fn ComputeArea(this: &Self::This, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> ::windows_core::Result<f32>;
    fn ComputeLength(this: &Self::This, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32) -> ::windows_core::Result<f32>;
    fn ComputePointAtLength(this: &Self::This, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> ::windows_core::Result<()>;
    fn Widen(this: &Self::This, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::core::option::Option<&Common::ID2D1SimplifiedGeometrySink>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1Geometry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Geometry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBounds(this, ::core::mem::transmute_copy(&worldtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWidenedBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, bounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWidenedBounds(this, ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StrokeContainsPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrokeContainsPoint(this, ::core::mem::transmute(&point), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contains, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FillContainsPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, contains: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FillContainsPoint(this, ::core::mem::transmute(&point), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contains, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareWithGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputgeometry: *mut ::core::ffi::c_void, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, relation: *mut D2D1_GEOMETRY_RELATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareWithGeometry(this, ::windows_core::from_raw_borrowed(&inputgeometry), ::core::mem::transmute_copy(&inputgeometrytransform), ::core::mem::transmute_copy(&flatteningtolerance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Simplify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Simplify(this, ::core::mem::transmute_copy(&simplificationoption), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance), ::windows_core::from_raw_borrowed(&geometrysink)).into())
        }
        unsafe extern "system" fn Tessellate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, tessellationsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Tessellate(this, ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance), ::windows_core::from_raw_borrowed(&tessellationsink)).into())
        }
        unsafe extern "system" fn CombineWithGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputgeometry: *mut ::core::ffi::c_void, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CombineWithGeometry(this, ::windows_core::from_raw_borrowed(&inputgeometry), ::core::mem::transmute_copy(&combinemode), ::core::mem::transmute_copy(&inputgeometrytransform), ::core::mem::transmute_copy(&flatteningtolerance), ::windows_core::from_raw_borrowed(&geometrysink)).into())
        }
        unsafe extern "system" fn Outline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Outline(this, ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance), ::windows_core::from_raw_borrowed(&geometrysink)).into())
        }
        unsafe extern "system" fn ComputeArea<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, area: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputeArea(this, ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(area, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputeLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, length: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputeLength(this, ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputePointAtLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: f32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, point: *mut Common::D2D_POINT_2F, unittangentvector: *mut Common::D2D_POINT_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ComputePointAtLength(this, ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance), ::core::mem::transmute_copy(&point), ::core::mem::transmute_copy(&unittangentvector)).into())
        }
        unsafe extern "system" fn Widen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Geometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Widen(this, ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance), ::windows_core::from_raw_borrowed(&geometrysink)).into())
        }
        ID2D1Geometry_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBounds: GetBounds::<Identity, Impl, OFFSET>,
            GetWidenedBounds: GetWidenedBounds::<Identity, Impl, OFFSET>,
            StrokeContainsPoint: StrokeContainsPoint::<Identity, Impl, OFFSET>,
            FillContainsPoint: FillContainsPoint::<Identity, Impl, OFFSET>,
            CompareWithGeometry: CompareWithGeometry::<Identity, Impl, OFFSET>,
            Simplify: Simplify::<Identity, Impl, OFFSET>,
            Tessellate: Tessellate::<Identity, Impl, OFFSET>,
            CombineWithGeometry: CombineWithGeometry::<Identity, Impl, OFFSET>,
            Outline: Outline::<Identity, Impl, OFFSET>,
            ComputeArea: ComputeArea::<Identity, Impl, OFFSET>,
            ComputeLength: ComputeLength::<Identity, Impl, OFFSET>,
            ComputePointAtLength: ComputePointAtLength::<Identity, Impl, OFFSET>,
            Widen: Widen::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1GeometryGroup_Impl: ::windows_core::BaseImpl + ID2D1Geometry_Impl {
    fn GetFillMode(this: &Self::This) -> Common::D2D1_FILL_MODE;
    fn GetSourceGeometryCount(this: &Self::This) -> u32;
    fn GetSourceGeometries(this: &Self::This, geometries: *mut ::core::option::Option<ID2D1Geometry>, geometriescount: u32);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1GeometryGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Geometry);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometryGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GeometryGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFillMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometryGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> Common::D2D1_FILL_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFillMode(this))
        }
        unsafe extern "system" fn GetSourceGeometryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometryGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourceGeometryCount(this))
        }
        unsafe extern "system" fn GetSourceGeometries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometryGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometries: *mut *mut ::core::ffi::c_void, geometriescount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourceGeometries(this, ::core::mem::transmute_copy(&geometries), ::core::mem::transmute_copy(&geometriescount)))
        }
        ID2D1GeometryGroup_Vtbl {
            base__: <ID2D1Geometry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFillMode: GetFillMode::<Identity, Impl, OFFSET>,
            GetSourceGeometryCount: GetSourceGeometryCount::<Identity, Impl, OFFSET>,
            GetSourceGeometries: GetSourceGeometries::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1GeometryRealization_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {}
impl ::windows_core::Iids for ID2D1GeometryRealization {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometryRealization_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GeometryRealization {
    const VTABLE: Self::Vtable = { ID2D1GeometryRealization_Vtbl { base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GeometrySink_Impl: ::windows_core::BaseImpl + Common::ID2D1SimplifiedGeometrySink_Impl {
    fn AddLine(this: &Self::This, point: &Common::D2D_POINT_2F);
    fn AddBezier(this: &Self::This, bezier: *const Common::D2D1_BEZIER_SEGMENT);
    fn AddQuadraticBezier(this: &Self::This, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT);
    fn AddQuadraticBeziers(this: &Self::This, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32);
    fn AddArc(this: &Self::This, arc: *const D2D1_ARC_SEGMENT);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1GeometrySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(Common::ID2D1SimplifiedGeometrySink);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometrySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GeometrySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLine(this, ::core::mem::transmute(&point)))
        }
        unsafe extern "system" fn AddBezier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bezier: *const Common::D2D1_BEZIER_SEGMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddBezier(this, ::core::mem::transmute_copy(&bezier)))
        }
        unsafe extern "system" fn AddQuadraticBezier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddQuadraticBezier(this, ::core::mem::transmute_copy(&bezier)))
        }
        unsafe extern "system" fn AddQuadraticBeziers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddQuadraticBeziers(this, ::core::mem::transmute_copy(&beziers), ::core::mem::transmute_copy(&bezierscount)))
        }
        unsafe extern "system" fn AddArc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arc: *const D2D1_ARC_SEGMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddArc(this, ::core::mem::transmute_copy(&arc)))
        }
        ID2D1GeometrySink_Vtbl {
            base__: <Common::ID2D1SimplifiedGeometrySink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddLine: AddLine::<Identity, Impl, OFFSET>,
            AddBezier: AddBezier::<Identity, Impl, OFFSET>,
            AddQuadraticBezier: AddQuadraticBezier::<Identity, Impl, OFFSET>,
            AddQuadraticBeziers: AddQuadraticBeziers::<Identity, Impl, OFFSET>,
            AddArc: AddArc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientMesh_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetPatchCount(this: &Self::This) -> u32;
    fn GetPatches(this: &Self::This, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1GradientMesh {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientMesh_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GradientMesh {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPatchCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientMesh_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatchCount(this))
        }
        unsafe extern "system" fn GetPatches<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientMesh_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatches(this, ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&patches), ::core::mem::transmute_copy(&patchescount)).into())
        }
        ID2D1GradientMesh_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPatchCount: GetPatchCount::<Identity, Impl, OFFSET>,
            GetPatches: GetPatches::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientStopCollection_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetGradientStopCount(this: &Self::This) -> u32;
    fn GetGradientStops(this: &Self::This, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32);
    fn GetColorInterpolationGamma(this: &Self::This) -> D2D1_GAMMA;
    fn GetExtendMode(this: &Self::This) -> D2D1_EXTEND_MODE;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1GradientStopCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GradientStopCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGradientStopCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGradientStopCount(this))
        }
        unsafe extern "system" fn GetGradientStops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGradientStops(this, ::core::mem::transmute_copy(&gradientstops), ::core::mem::transmute_copy(&gradientstopscount)))
        }
        unsafe extern "system" fn GetColorInterpolationGamma<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_GAMMA {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorInterpolationGamma(this))
        }
        unsafe extern "system" fn GetExtendMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendMode(this))
        }
        ID2D1GradientStopCollection_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGradientStopCount: GetGradientStopCount::<Identity, Impl, OFFSET>,
            GetGradientStops: GetGradientStops::<Identity, Impl, OFFSET>,
            GetColorInterpolationGamma: GetColorInterpolationGamma::<Identity, Impl, OFFSET>,
            GetExtendMode: GetExtendMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GradientStopCollection1_Impl: ::windows_core::BaseImpl + ID2D1GradientStopCollection_Impl {
    fn GetGradientStops1(this: &Self::This, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32);
    fn GetPreInterpolationSpace(this: &Self::This) -> D2D1_COLOR_SPACE;
    fn GetPostInterpolationSpace(this: &Self::This) -> D2D1_COLOR_SPACE;
    fn GetBufferPrecision(this: &Self::This) -> D2D1_BUFFER_PRECISION;
    fn GetColorInterpolationMode(this: &Self::This) -> D2D1_COLOR_INTERPOLATION_MODE;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1GradientStopCollection1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1GradientStopCollection);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1GradientStopCollection1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGradientStops1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *mut D2D1_GRADIENT_STOP, gradientstopscount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGradientStops1(this, ::core::mem::transmute_copy(&gradientstops), ::core::mem::transmute_copy(&gradientstopscount)))
        }
        unsafe extern "system" fn GetPreInterpolationSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPreInterpolationSpace(this))
        }
        unsafe extern "system" fn GetPostInterpolationSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_SPACE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPostInterpolationSpace(this))
        }
        unsafe extern "system" fn GetBufferPrecision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_BUFFER_PRECISION {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferPrecision(this))
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1GradientStopCollection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_COLOR_INTERPOLATION_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorInterpolationMode(this))
        }
        ID2D1GradientStopCollection1_Vtbl {
            base__: <ID2D1GradientStopCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGradientStops1: GetGradientStops1::<Identity, Impl, OFFSET>,
            GetPreInterpolationSpace: GetPreInterpolationSpace::<Identity, Impl, OFFSET>,
            GetPostInterpolationSpace: GetPostInterpolationSpace::<Identity, Impl, OFFSET>,
            GetBufferPrecision: GetBufferPrecision::<Identity, Impl, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1HwndRenderTarget_Impl: ::windows_core::BaseImpl + ID2D1RenderTarget_Impl {
    fn CheckWindowState(this: &Self::This) -> D2D1_WINDOW_STATE;
    fn Resize(this: &Self::This, pixelsize: *const Common::D2D_SIZE_U) -> ::windows_core::Result<()>;
    fn GetHwnd(this: &Self::This) -> super::super::Foundation::HWND;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1HwndRenderTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1RenderTarget);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1HwndRenderTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1HwndRenderTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CheckWindowState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1HwndRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_WINDOW_STATE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckWindowState(this))
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1HwndRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelsize: *const Common::D2D_SIZE_U) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resize(this, ::core::mem::transmute_copy(&pixelsize)).into())
        }
        unsafe extern "system" fn GetHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1HwndRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HWND {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHwnd(this))
        }
        ID2D1HwndRenderTarget_Vtbl {
            base__: <ID2D1RenderTarget as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CheckWindowState: CheckWindowState::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            GetHwnd: GetHwnd::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1Image_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {}
impl ::windows_core::Iids for ID2D1Image {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Image_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Image {
    const VTABLE: Self::Vtable = { ID2D1Image_Vtbl { base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1ImageBrush_Impl: ::windows_core::BaseImpl + ID2D1Brush_Impl {
    fn SetImage(this: &Self::This, image: ::core::option::Option<&ID2D1Image>);
    fn SetExtendModeX(this: &Self::This, extendmodex: D2D1_EXTEND_MODE);
    fn SetExtendModeY(this: &Self::This, extendmodey: D2D1_EXTEND_MODE);
    fn SetInterpolationMode(this: &Self::This, interpolationmode: D2D1_INTERPOLATION_MODE);
    fn SetSourceRectangle(this: &Self::This, sourcerectangle: *const Common::D2D_RECT_F);
    fn GetImage(this: &Self::This, image: *mut ::core::option::Option<ID2D1Image>);
    fn GetExtendModeX(this: &Self::This) -> D2D1_EXTEND_MODE;
    fn GetExtendModeY(this: &Self::This) -> D2D1_EXTEND_MODE;
    fn GetInterpolationMode(this: &Self::This) -> D2D1_INTERPOLATION_MODE;
    fn GetSourceRectangle(this: &Self::This, sourcerectangle: *mut Common::D2D_RECT_F);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1ImageBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Brush);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ImageBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImage(this, ::windows_core::from_raw_borrowed(&image)))
        }
        unsafe extern "system" fn SetExtendModeX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodex: D2D1_EXTEND_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtendModeX(this, ::core::mem::transmute_copy(&extendmodex)))
        }
        unsafe extern "system" fn SetExtendModeY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendmodey: D2D1_EXTEND_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtendModeY(this, ::core::mem::transmute_copy(&extendmodey)))
        }
        unsafe extern "system" fn SetInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interpolationmode: D2D1_INTERPOLATION_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterpolationMode(this, ::core::mem::transmute_copy(&interpolationmode)))
        }
        unsafe extern "system" fn SetSourceRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcerectangle: *const Common::D2D_RECT_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSourceRectangle(this, ::core::mem::transmute_copy(&sourcerectangle)))
        }
        unsafe extern "system" fn GetImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImage(this, ::core::mem::transmute_copy(&image)))
        }
        unsafe extern "system" fn GetExtendModeX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendModeX(this))
        }
        unsafe extern "system" fn GetExtendModeY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_EXTEND_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendModeY(this))
        }
        unsafe extern "system" fn GetInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_INTERPOLATION_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterpolationMode(this))
        }
        unsafe extern "system" fn GetSourceRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcerectangle: *mut Common::D2D_RECT_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourceRectangle(this, ::core::mem::transmute_copy(&sourcerectangle)))
        }
        ID2D1ImageBrush_Vtbl {
            base__: <ID2D1Brush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetImage: SetImage::<Identity, Impl, OFFSET>,
            SetExtendModeX: SetExtendModeX::<Identity, Impl, OFFSET>,
            SetExtendModeY: SetExtendModeY::<Identity, Impl, OFFSET>,
            SetInterpolationMode: SetInterpolationMode::<Identity, Impl, OFFSET>,
            SetSourceRectangle: SetSourceRectangle::<Identity, Impl, OFFSET>,
            GetImage: GetImage::<Identity, Impl, OFFSET>,
            GetExtendModeX: GetExtendModeX::<Identity, Impl, OFFSET>,
            GetExtendModeY: GetExtendModeY::<Identity, Impl, OFFSET>,
            GetInterpolationMode: GetInterpolationMode::<Identity, Impl, OFFSET>,
            GetSourceRectangle: GetSourceRectangle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1ImageSource_Impl: ::windows_core::BaseImpl + ID2D1Image_Impl {
    fn OfferResources(this: &Self::This) -> ::windows_core::Result<()>;
    fn TryReclaimResources(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1ImageSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Image);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ImageSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OfferResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OfferResources(this).into())
        }
        unsafe extern "system" fn TryReclaimResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcesdiscarded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TryReclaimResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcesdiscarded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1ImageSource_Vtbl {
            base__: <ID2D1Image as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OfferResources: OfferResources::<Identity, Impl, OFFSET>,
            TryReclaimResources: TryReclaimResources::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1ImageSourceFromWic_Impl: ::windows_core::BaseImpl + ID2D1ImageSource_Impl {
    fn EnsureCached(this: &Self::This, rectangletofill: *const Common::D2D_RECT_U) -> ::windows_core::Result<()>;
    fn TrimCache(this: &Self::This, rectangletopreserve: *const Common::D2D_RECT_U) -> ::windows_core::Result<()>;
    fn GetSource(this: &Self::This, wicbitmapsource: *mut ::core::option::Option<super::Imaging::IWICBitmapSource>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1ImageSourceFromWic {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1ImageSource);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageSourceFromWic_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ImageSourceFromWic {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnsureCached<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageSourceFromWic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rectangletofill: *const Common::D2D_RECT_U) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnsureCached(this, ::core::mem::transmute_copy(&rectangletofill)).into())
        }
        unsafe extern "system" fn TrimCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageSourceFromWic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rectangletopreserve: *const Common::D2D_RECT_U) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TrimCache(this, ::core::mem::transmute_copy(&rectangletopreserve)).into())
        }
        unsafe extern "system" fn GetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ImageSourceFromWic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSource(this, ::core::mem::transmute_copy(&wicbitmapsource)))
        }
        ID2D1ImageSourceFromWic_Vtbl {
            base__: <ID2D1ImageSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnsureCached: EnsureCached::<Identity, Impl, OFFSET>,
            TrimCache: TrimCache::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1Ink_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn SetStartPoint(this: &Self::This, startpoint: *const D2D1_INK_POINT);
    fn GetStartPoint(this: &Self::This) -> D2D1_INK_POINT;
    fn AddSegments(this: &Self::This, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_core::Result<()>;
    fn RemoveSegmentsAtEnd(this: &Self::This, segmentscount: u32) -> ::windows_core::Result<()>;
    fn SetSegments(this: &Self::This, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_core::Result<()>;
    fn SetSegmentAtEnd(this: &Self::This, segment: *const D2D1_INK_BEZIER_SEGMENT) -> ::windows_core::Result<()>;
    fn GetSegmentCount(this: &Self::This) -> u32;
    fn GetSegments(this: &Self::This, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_core::Result<()>;
    fn StreamAsGeometry(this: &Self::This, inkstyle: ::core::option::Option<&ID2D1InkStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: ::core::option::Option<&Common::ID2D1SimplifiedGeometrySink>) -> ::windows_core::Result<()>;
    fn GetBounds(this: &Self::This, inkstyle: ::core::option::Option<&ID2D1InkStyle>, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<Common::D2D_RECT_F>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1Ink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Ink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const D2D1_INK_POINT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartPoint(this, ::core::mem::transmute_copy(&startpoint)))
        }
        unsafe extern "system" fn GetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut D2D1_INK_POINT) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetStartPoint(this))
        }
        unsafe extern "system" fn AddSegments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSegments(this, ::core::mem::transmute_copy(&segments), ::core::mem::transmute_copy(&segmentscount)).into())
        }
        unsafe extern "system" fn RemoveSegmentsAtEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSegmentsAtEnd(this, ::core::mem::transmute_copy(&segmentscount)).into())
        }
        unsafe extern "system" fn SetSegments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSegments(this, ::core::mem::transmute_copy(&startsegment), ::core::mem::transmute_copy(&segments), ::core::mem::transmute_copy(&segmentscount)).into())
        }
        unsafe extern "system" fn SetSegmentAtEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segment: *const D2D1_INK_BEZIER_SEGMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSegmentAtEnd(this, ::core::mem::transmute_copy(&segment)).into())
        }
        unsafe extern "system" fn GetSegmentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSegmentCount(this))
        }
        unsafe extern "system" fn GetSegments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSegments(this, ::core::mem::transmute_copy(&startsegment), ::core::mem::transmute_copy(&segments), ::core::mem::transmute_copy(&segmentscount)).into())
        }
        unsafe extern "system" fn StreamAsGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StreamAsGeometry(this, ::windows_core::from_raw_borrowed(&inkstyle), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance), ::windows_core::from_raw_borrowed(&geometrysink)).into())
        }
        unsafe extern "system" fn GetBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Ink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkstyle: *mut ::core::ffi::c_void, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, bounds: *mut Common::D2D_RECT_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBounds(this, ::windows_core::from_raw_borrowed(&inkstyle), ::core::mem::transmute_copy(&worldtransform)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Ink_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            AddSegments: AddSegments::<Identity, Impl, OFFSET>,
            RemoveSegmentsAtEnd: RemoveSegmentsAtEnd::<Identity, Impl, OFFSET>,
            SetSegments: SetSegments::<Identity, Impl, OFFSET>,
            SetSegmentAtEnd: SetSegmentAtEnd::<Identity, Impl, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, Impl, OFFSET>,
            GetSegments: GetSegments::<Identity, Impl, OFFSET>,
            StreamAsGeometry: StreamAsGeometry::<Identity, Impl, OFFSET>,
            GetBounds: GetBounds::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`"]
#[cfg(feature = "Foundation_Numerics")]
pub trait ID2D1InkStyle_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn SetNibTransform(this: &Self::This, transform: *const super::super::super::Foundation::Numerics::Matrix3x2);
    fn GetNibTransform(this: &Self::This, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    fn SetNibShape(this: &Self::This, nibshape: D2D1_INK_NIB_SHAPE);
    fn GetNibShape(this: &Self::This) -> D2D1_INK_NIB_SHAPE;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::Iids for ID2D1InkStyle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(feature = "Foundation_Numerics")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1InkStyle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1InkStyle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNibTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1InkStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNibTransform(this, ::core::mem::transmute_copy(&transform)))
        }
        unsafe extern "system" fn GetNibTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1InkStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNibTransform(this, ::core::mem::transmute_copy(&transform)))
        }
        unsafe extern "system" fn SetNibShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1InkStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nibshape: D2D1_INK_NIB_SHAPE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNibShape(this, ::core::mem::transmute_copy(&nibshape)))
        }
        unsafe extern "system" fn GetNibShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1InkStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_INK_NIB_SHAPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNibShape(this))
        }
        ID2D1InkStyle_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNibTransform: SetNibTransform::<Identity, Impl, OFFSET>,
            GetNibTransform: GetNibTransform::<Identity, Impl, OFFSET>,
            SetNibShape: SetNibShape::<Identity, Impl, OFFSET>,
            GetNibShape: GetNibShape::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1Layer_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetSize(this: &Self::This) -> Common::D2D_SIZE_F;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1Layer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Layer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Layer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Layer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetSize(this))
        }
        ID2D1Layer_Vtbl { base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSize: GetSize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1LinearGradientBrush_Impl: ::windows_core::BaseImpl + ID2D1Brush_Impl {
    fn SetStartPoint(this: &Self::This, startpoint: &Common::D2D_POINT_2F);
    fn SetEndPoint(this: &Self::This, endpoint: &Common::D2D_POINT_2F);
    fn GetStartPoint(this: &Self::This) -> Common::D2D_POINT_2F;
    fn GetEndPoint(this: &Self::This) -> Common::D2D_POINT_2F;
    fn GetGradientStopCollection(this: &Self::This, gradientstopcollection: *mut ::core::option::Option<ID2D1GradientStopCollection>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1LinearGradientBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Brush);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1LinearGradientBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1LinearGradientBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1LinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartPoint(this, ::core::mem::transmute(&startpoint)))
        }
        unsafe extern "system" fn SetEndPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1LinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEndPoint(this, ::core::mem::transmute(&endpoint)))
        }
        unsafe extern "system" fn GetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1LinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetStartPoint(this))
        }
        unsafe extern "system" fn GetEndPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1LinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetEndPoint(this))
        }
        unsafe extern "system" fn GetGradientStopCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1LinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGradientStopCollection(this, ::core::mem::transmute_copy(&gradientstopcollection)))
        }
        ID2D1LinearGradientBrush_Vtbl {
            base__: <ID2D1Brush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, Impl, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, Impl, OFFSET>,
            GetGradientStopCollection: GetGradientStopCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1LookupTable3D_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {}
impl ::windows_core::Iids for ID2D1LookupTable3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1LookupTable3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1LookupTable3D {
    const VTABLE: Self::Vtable = { ID2D1LookupTable3D_Vtbl { base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1Mesh_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn Open(this: &Self::This) -> ::windows_core::Result<ID2D1TessellationSink>;
}
impl ::windows_core::Iids for ID2D1Mesh {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Mesh_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Mesh {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Mesh_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tessellationsink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tessellationsink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Mesh_Vtbl { base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Open: Open::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1Multithread_Impl: ::windows_core::BaseImpl {
    fn GetMultithreadProtected(this: &Self::This) -> super::super::Foundation::BOOL;
    fn Enter(this: &Self::This);
    fn Leave(this: &Self::This);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1Multithread {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Multithread_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Multithread {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMultithreadProtected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMultithreadProtected(this))
        }
        unsafe extern "system" fn Enter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enter(this))
        }
        unsafe extern "system" fn Leave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Multithread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Leave(this))
        }
        ID2D1Multithread_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, Impl, OFFSET>,
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1OffsetTransform_Impl: ::windows_core::BaseImpl + ID2D1TransformNode_Impl {
    fn SetOffset(this: &Self::This, offset: &super::super::Foundation::POINT);
    fn GetOffset(this: &Self::This) -> super::super::Foundation::POINT;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1OffsetTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1TransformNode);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1OffsetTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1OffsetTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1OffsetTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: super::super::Foundation::POINT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffset(this, ::core::mem::transmute(&offset)))
        }
        unsafe extern "system" fn GetOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1OffsetTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::POINT) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetOffset(this))
        }
        ID2D1OffsetTransform_Vtbl {
            base__: <ID2D1TransformNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOffset: SetOffset::<Identity, Impl, OFFSET>,
            GetOffset: GetOffset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1PathGeometry_Impl: ::windows_core::BaseImpl + ID2D1Geometry_Impl {
    fn Open(this: &Self::This) -> ::windows_core::Result<ID2D1GeometrySink>;
    fn Stream(this: &Self::This, geometrysink: ::core::option::Option<&ID2D1GeometrySink>) -> ::windows_core::Result<()>;
    fn GetSegmentCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFigureCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1PathGeometry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Geometry);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PathGeometry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1PathGeometry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PathGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometrysink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometrysink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PathGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stream(this, ::windows_core::from_raw_borrowed(&geometrysink)).into())
        }
        unsafe extern "system" fn GetSegmentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PathGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSegmentCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFigureCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PathGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFigureCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1PathGeometry_Vtbl {
            base__: <ID2D1Geometry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, Impl, OFFSET>,
            GetFigureCount: GetFigureCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1PathGeometry1_Impl: ::windows_core::BaseImpl + ID2D1PathGeometry_Impl {
    fn ComputePointAndSegmentAtLength(this: &Self::This, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1PathGeometry1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1PathGeometry);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PathGeometry1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1PathGeometry1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ComputePointAndSegmentAtLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PathGeometry1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: f32, startsegment: u32, worldtransform: *const super::super::super::Foundation::Numerics::Matrix3x2, flatteningtolerance: f32, pointdescription: *mut D2D1_POINT_DESCRIPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ComputePointAndSegmentAtLength(this, ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&startsegment), ::core::mem::transmute_copy(&worldtransform), ::core::mem::transmute_copy(&flatteningtolerance), ::core::mem::transmute_copy(&pointdescription)).into())
        }
        ID2D1PathGeometry1_Vtbl {
            base__: <ID2D1PathGeometry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ComputePointAndSegmentAtLength: ComputePointAndSegmentAtLength::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
pub trait ID2D1PrintControl_Impl: ::windows_core::BaseImpl {
    fn AddPage(this: &Self::This, commandlist: ::core::option::Option<&ID2D1CommandList>, pagesize: &Common::D2D_SIZE_F, pageprintticketstream: ::core::option::Option<&super::super::System::Com::IStream>, tag1: *mut u64, tag2: *mut u64) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1PrintControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PrintControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1PrintControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PrintControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandlist: *mut ::core::ffi::c_void, pagesize: Common::D2D_SIZE_F, pageprintticketstream: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPage(this, ::windows_core::from_raw_borrowed(&commandlist), ::core::mem::transmute(&pagesize), ::windows_core::from_raw_borrowed(&pageprintticketstream), ::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1PrintControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ID2D1PrintControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1Properties_Impl: ::windows_core::BaseImpl {
    fn GetPropertyCount(this: &Self::This) -> u32;
    fn GetPropertyName(this: &Self::This, index: u32, name: ::windows_core::PWSTR, namecount: u32) -> ::windows_core::Result<()>;
    fn GetPropertyNameLength(this: &Self::This, index: u32) -> u32;
    fn GetType(this: &Self::This, index: u32) -> D2D1_PROPERTY_TYPE;
    fn GetPropertyIndex(this: &Self::This, name: &::windows_core::PCWSTR) -> u32;
    fn SetValueByName(this: &Self::This, name: &::windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows_core::Result<()>;
    fn GetValueByName(this: &Self::This, name: &::windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows_core::Result<()>;
    fn GetValueSize(this: &Self::This, index: u32) -> u32;
    fn GetSubProperties(this: &Self::This, index: u32) -> ::windows_core::Result<ID2D1Properties>;
}
impl ::windows_core::Iids for ID2D1Properties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Properties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyCount(this))
        }
        unsafe extern "system" fn GetPropertyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, name: ::windows_core::PWSTR, namecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyName(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&namecount)).into())
        }
        unsafe extern "system" fn GetPropertyNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyNameLength(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> D2D1_PROPERTY_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetPropertyIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyIndex(this, ::core::mem::transmute(&name)))
        }
        unsafe extern "system" fn SetValueByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *const u8, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn GetValueByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValueByName(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, r#type: D2D1_PROPERTY_TYPE, data: *mut u8, datasize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValue(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datasize)).into())
        }
        unsafe extern "system" fn GetValueSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValueSize(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn GetSubProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, subproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubProperties(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Properties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetPropertyName: GetPropertyName::<Identity, Impl, OFFSET>,
            GetPropertyNameLength: GetPropertyNameLength::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetPropertyIndex: GetPropertyIndex::<Identity, Impl, OFFSET>,
            SetValueByName: SetValueByName::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValueByName: GetValueByName::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetValueSize: GetValueSize::<Identity, Impl, OFFSET>,
            GetSubProperties: GetSubProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RadialGradientBrush_Impl: ::windows_core::BaseImpl + ID2D1Brush_Impl {
    fn SetCenter(this: &Self::This, center: &Common::D2D_POINT_2F);
    fn SetGradientOriginOffset(this: &Self::This, gradientoriginoffset: &Common::D2D_POINT_2F);
    fn SetRadiusX(this: &Self::This, radiusx: f32);
    fn SetRadiusY(this: &Self::This, radiusy: f32);
    fn GetCenter(this: &Self::This) -> Common::D2D_POINT_2F;
    fn GetGradientOriginOffset(this: &Self::This) -> Common::D2D_POINT_2F;
    fn GetRadiusX(this: &Self::This) -> f32;
    fn GetRadiusY(this: &Self::This) -> f32;
    fn GetGradientStopCollection(this: &Self::This, gradientstopcollection: *mut ::core::option::Option<ID2D1GradientStopCollection>);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1RadialGradientBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Brush);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1RadialGradientBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCenter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenter(this, ::core::mem::transmute(&center)))
        }
        unsafe extern "system" fn SetGradientOriginOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientoriginoffset: Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGradientOriginOffset(this, ::core::mem::transmute(&gradientoriginoffset)))
        }
        unsafe extern "system" fn SetRadiusX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiusx: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRadiusX(this, ::core::mem::transmute_copy(&radiusx)))
        }
        unsafe extern "system" fn SetRadiusY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiusy: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRadiusY(this, ::core::mem::transmute_copy(&radiusy)))
        }
        unsafe extern "system" fn GetCenter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetCenter(this))
        }
        unsafe extern "system" fn GetGradientOriginOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_POINT_2F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetGradientOriginOffset(this))
        }
        unsafe extern "system" fn GetRadiusX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRadiusX(this))
        }
        unsafe extern "system" fn GetRadiusY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRadiusY(this))
        }
        unsafe extern "system" fn GetGradientStopCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstopcollection: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGradientStopCollection(this, ::core::mem::transmute_copy(&gradientstopcollection)))
        }
        ID2D1RadialGradientBrush_Vtbl {
            base__: <ID2D1Brush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCenter: SetCenter::<Identity, Impl, OFFSET>,
            SetGradientOriginOffset: SetGradientOriginOffset::<Identity, Impl, OFFSET>,
            SetRadiusX: SetRadiusX::<Identity, Impl, OFFSET>,
            SetRadiusY: SetRadiusY::<Identity, Impl, OFFSET>,
            GetCenter: GetCenter::<Identity, Impl, OFFSET>,
            GetGradientOriginOffset: GetGradientOriginOffset::<Identity, Impl, OFFSET>,
            GetRadiusX: GetRadiusX::<Identity, Impl, OFFSET>,
            GetRadiusY: GetRadiusY::<Identity, Impl, OFFSET>,
            GetGradientStopCollection: GetGradientStopCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RectangleGeometry_Impl: ::windows_core::BaseImpl + ID2D1Geometry_Impl {
    fn GetRect(this: &Self::This, rect: *mut Common::D2D_RECT_F);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1RectangleGeometry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Geometry);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RectangleGeometry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1RectangleGeometry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RectangleGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *mut Common::D2D_RECT_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRect(this, ::core::mem::transmute_copy(&rect)))
        }
        ID2D1RectangleGeometry_Vtbl { base__: <ID2D1Geometry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetRect: GetRect::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1RenderInfo_Impl: ::windows_core::BaseImpl {
    fn SetInputDescription(this: &Self::This, inputindex: u32, inputdescription: &D2D1_INPUT_DESCRIPTION) -> ::windows_core::Result<()>;
    fn SetOutputBuffer(this: &Self::This, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows_core::Result<()>;
    fn SetCached(this: &Self::This, iscached: super::super::Foundation::BOOL);
    fn SetInstructionCountHint(this: &Self::This, instructioncount: u32);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1RenderInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1RenderInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInputDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputDescription(this, ::core::mem::transmute_copy(&inputindex), ::core::mem::transmute(&inputdescription)).into())
        }
        unsafe extern "system" fn SetOutputBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputBuffer(this, ::core::mem::transmute_copy(&bufferprecision), ::core::mem::transmute_copy(&channeldepth)).into())
        }
        unsafe extern "system" fn SetCached<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iscached: super::super::Foundation::BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCached(this, ::core::mem::transmute_copy(&iscached)))
        }
        unsafe extern "system" fn SetInstructionCountHint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, instructioncount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInstructionCountHint(this, ::core::mem::transmute_copy(&instructioncount)))
        }
        ID2D1RenderInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInputDescription: SetInputDescription::<Identity, Impl, OFFSET>,
            SetOutputBuffer: SetOutputBuffer::<Identity, Impl, OFFSET>,
            SetCached: SetCached::<Identity, Impl, OFFSET>,
            SetInstructionCountHint: SetInstructionCountHint::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
pub trait ID2D1RenderTarget_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn CreateBitmap(this: &Self::This, size: &Common::D2D_SIZE_U, srcdata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows_core::Result<ID2D1Bitmap>;
    fn CreateBitmapFromWicBitmap(this: &Self::This, wicbitmapsource: ::core::option::Option<&super::Imaging::IWICBitmapSource>, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows_core::Result<ID2D1Bitmap>;
    fn CreateSharedBitmap(this: &Self::This, riid: *const ::windows_core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows_core::Result<()>;
    fn CreateBitmapBrush(this: &Self::This, bitmap: ::core::option::Option<&ID2D1Bitmap>, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> ::windows_core::Result<ID2D1BitmapBrush>;
    fn CreateSolidColorBrush(this: &Self::This, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES) -> ::windows_core::Result<ID2D1SolidColorBrush>;
    fn CreateGradientStopCollection(this: &Self::This, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows_core::Result<ID2D1GradientStopCollection>;
    fn CreateLinearGradientBrush(this: &Self::This, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: ::core::option::Option<&ID2D1GradientStopCollection>) -> ::windows_core::Result<ID2D1LinearGradientBrush>;
    fn CreateRadialGradientBrush(this: &Self::This, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: ::core::option::Option<&ID2D1GradientStopCollection>) -> ::windows_core::Result<ID2D1RadialGradientBrush>;
    fn CreateCompatibleRenderTarget(this: &Self::This, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows_core::Result<ID2D1BitmapRenderTarget>;
    fn CreateLayer(this: &Self::This, size: *const Common::D2D_SIZE_F) -> ::windows_core::Result<ID2D1Layer>;
    fn CreateMesh(this: &Self::This) -> ::windows_core::Result<ID2D1Mesh>;
    fn DrawLine(this: &Self::This, point0: &Common::D2D_POINT_2F, point1: &Common::D2D_POINT_2F, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>);
    fn DrawRectangle(this: &Self::This, rect: *const Common::D2D_RECT_F, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>);
    fn FillRectangle(this: &Self::This, rect: *const Common::D2D_RECT_F, brush: ::core::option::Option<&ID2D1Brush>);
    fn DrawRoundedRectangle(this: &Self::This, roundedrect: *const D2D1_ROUNDED_RECT, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>);
    fn FillRoundedRectangle(this: &Self::This, roundedrect: *const D2D1_ROUNDED_RECT, brush: ::core::option::Option<&ID2D1Brush>);
    fn DrawEllipse(this: &Self::This, ellipse: *const D2D1_ELLIPSE, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>);
    fn FillEllipse(this: &Self::This, ellipse: *const D2D1_ELLIPSE, brush: ::core::option::Option<&ID2D1Brush>);
    fn DrawGeometry(this: &Self::This, geometry: ::core::option::Option<&ID2D1Geometry>, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, strokestyle: ::core::option::Option<&ID2D1StrokeStyle>);
    fn FillGeometry(this: &Self::This, geometry: ::core::option::Option<&ID2D1Geometry>, brush: ::core::option::Option<&ID2D1Brush>, opacitybrush: ::core::option::Option<&ID2D1Brush>);
    fn FillMesh(this: &Self::This, mesh: ::core::option::Option<&ID2D1Mesh>, brush: ::core::option::Option<&ID2D1Brush>);
    fn FillOpacityMask(this: &Self::This, opacitymask: ::core::option::Option<&ID2D1Bitmap>, brush: ::core::option::Option<&ID2D1Brush>, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F);
    fn DrawBitmap(this: &Self::This, bitmap: ::core::option::Option<&ID2D1Bitmap>, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F);
    fn DrawText(this: &Self::This, string: &::windows_core::PCWSTR, stringlength: u32, textformat: ::core::option::Option<&super::DirectWrite::IDWriteTextFormat>, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: ::core::option::Option<&ID2D1Brush>, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn DrawTextLayout(this: &Self::This, origin: &Common::D2D_POINT_2F, textlayout: ::core::option::Option<&super::DirectWrite::IDWriteTextLayout>, defaultfillbrush: ::core::option::Option<&ID2D1Brush>, options: D2D1_DRAW_TEXT_OPTIONS);
    fn DrawGlyphRun(this: &Self::This, baselineorigin: &Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: ::core::option::Option<&ID2D1Brush>, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE);
    fn SetTransform(this: &Self::This, transform: *const super::super::super::Foundation::Numerics::Matrix3x2);
    fn GetTransform(this: &Self::This, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
    fn SetAntialiasMode(this: &Self::This, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn GetAntialiasMode(this: &Self::This) -> D2D1_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(this: &Self::This, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE);
    fn GetTextAntialiasMode(this: &Self::This) -> D2D1_TEXT_ANTIALIAS_MODE;
    fn SetTextRenderingParams(this: &Self::This, textrenderingparams: ::core::option::Option<&super::DirectWrite::IDWriteRenderingParams>);
    fn GetTextRenderingParams(this: &Self::This, textrenderingparams: *mut ::core::option::Option<super::DirectWrite::IDWriteRenderingParams>);
    fn SetTags(this: &Self::This, tag1: u64, tag2: u64);
    fn GetTags(this: &Self::This, tag1: *mut u64, tag2: *mut u64);
    fn PushLayer(this: &Self::This, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: ::core::option::Option<&ID2D1Layer>);
    fn PopLayer(this: &Self::This);
    fn Flush(this: &Self::This, tag1: *mut u64, tag2: *mut u64) -> ::windows_core::Result<()>;
    fn SaveDrawingState(this: &Self::This, drawingstateblock: ::core::option::Option<&ID2D1DrawingStateBlock>);
    fn RestoreDrawingState(this: &Self::This, drawingstateblock: ::core::option::Option<&ID2D1DrawingStateBlock>);
    fn PushAxisAlignedClip(this: &Self::This, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn PopAxisAlignedClip(this: &Self::This);
    fn Clear(this: &Self::This, clearcolor: *const Common::D2D1_COLOR_F);
    fn BeginDraw(this: &Self::This);
    fn EndDraw(this: &Self::This, tag1: *mut u64, tag2: *mut u64) -> ::windows_core::Result<()>;
    fn GetPixelFormat(this: &Self::This) -> Common::D2D1_PIXEL_FORMAT;
    fn SetDpi(this: &Self::This, dpix: f32, dpiy: f32);
    fn GetDpi(this: &Self::This, dpix: *mut f32, dpiy: *mut f32);
    fn GetSize(this: &Self::This) -> Common::D2D_SIZE_F;
    fn GetPixelSize(this: &Self::This) -> Common::D2D_SIZE_U;
    fn GetMaximumBitmapSize(this: &Self::This) -> u32;
    fn IsSupported(this: &Self::This, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl ::windows_core::Iids for ID2D1RenderTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1RenderTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: Common::D2D_SIZE_U, srcdata: *const ::core::ffi::c_void, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmap(this, ::core::mem::transmute(&size), ::core::mem::transmute_copy(&srcdata), ::core::mem::transmute_copy(&pitch), ::core::mem::transmute_copy(&bitmapproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wicbitmapsource: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromWicBitmap(this, ::windows_core::from_raw_borrowed(&wicbitmapsource), ::core::mem::transmute_copy(&bitmapproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSharedBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: *const D2D1_BITMAP_PROPERTIES, bitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSharedBitmap(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bitmapproperties), ::core::mem::transmute_copy(&bitmap)).into())
        }
        unsafe extern "system" fn CreateBitmapBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, bitmapbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapBrush(this, ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&bitmapbrushproperties), ::core::mem::transmute_copy(&brushproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmapbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F, brushproperties: *const D2D1_BRUSH_PROPERTIES, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSolidColorBrush(this, ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&brushproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(solidcolorbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGradientStopCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *const D2D1_GRADIENT_STOP, gradientstopscount: u32, colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE, gradientstopcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGradientStopCollection(this, ::core::mem::transmute_copy(&gradientstops), ::core::mem::transmute_copy(&gradientstopscount), ::core::mem::transmute_copy(&colorinterpolationgamma), ::core::mem::transmute_copy(&extendmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstopcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut ::core::ffi::c_void, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearGradientBrush(this, ::core::mem::transmute_copy(&lineargradientbrushproperties), ::core::mem::transmute_copy(&brushproperties), ::windows_core::from_raw_borrowed(&gradientstopcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineargradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: *const D2D1_BRUSH_PROPERTIES, gradientstopcollection: *mut ::core::ffi::c_void, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRadialGradientBrush(this, ::core::mem::transmute_copy(&radialgradientbrushproperties), ::core::mem::transmute_copy(&brushproperties), ::windows_core::from_raw_borrowed(&gradientstopcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radialgradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCompatibleRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredsize: *const Common::D2D_SIZE_F, desiredpixelsize: *const Common::D2D_SIZE_U, desiredformat: *const Common::D2D1_PIXEL_FORMAT, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS, bitmaprendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCompatibleRenderTarget(this, ::core::mem::transmute_copy(&desiredsize), ::core::mem::transmute_copy(&desiredpixelsize), ::core::mem::transmute_copy(&desiredformat), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmaprendertarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *const Common::D2D_SIZE_F, layer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLayer(this, ::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(layer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mesh: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMesh(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mesh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawLine(this, ::core::mem::transmute(&point0), ::core::mem::transmute(&point1), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)))
        }
        unsafe extern "system" fn DrawRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawRectangle(this, ::core::mem::transmute_copy(&rect), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)))
        }
        unsafe extern "system" fn FillRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rect: *const Common::D2D_RECT_F, brush: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillRectangle(this, ::core::mem::transmute_copy(&rect), ::windows_core::from_raw_borrowed(&brush)))
        }
        unsafe extern "system" fn DrawRoundedRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawRoundedRectangle(this, ::core::mem::transmute_copy(&roundedrect), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)))
        }
        unsafe extern "system" fn FillRoundedRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrect: *const D2D1_ROUNDED_RECT, brush: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillRoundedRectangle(this, ::core::mem::transmute_copy(&roundedrect), ::windows_core::from_raw_borrowed(&brush)))
        }
        unsafe extern "system" fn DrawEllipse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawEllipse(this, ::core::mem::transmute_copy(&ellipse), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)))
        }
        unsafe extern "system" fn FillEllipse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ellipse: *const D2D1_ELLIPSE, brush: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillEllipse(this, ::core::mem::transmute_copy(&ellipse), ::windows_core::from_raw_borrowed(&brush)))
        }
        unsafe extern "system" fn DrawGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, strokestyle: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGeometry(this, ::windows_core::from_raw_borrowed(&geometry), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::windows_core::from_raw_borrowed(&strokestyle)))
        }
        unsafe extern "system" fn FillGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, opacitybrush: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillGeometry(this, ::windows_core::from_raw_borrowed(&geometry), ::windows_core::from_raw_borrowed(&brush), ::windows_core::from_raw_borrowed(&opacitybrush)))
        }
        unsafe extern "system" fn FillMesh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mesh: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillMesh(this, ::windows_core::from_raw_borrowed(&mesh), ::windows_core::from_raw_borrowed(&brush)))
        }
        unsafe extern "system" fn FillOpacityMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymask: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: *const Common::D2D_RECT_F, sourcerectangle: *const Common::D2D_RECT_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FillOpacityMask(this, ::windows_core::from_raw_borrowed(&opacitymask), ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&content), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&sourcerectangle)))
        }
        unsafe extern "system" fn DrawBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, destinationrectangle: *const Common::D2D_RECT_F, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: *const Common::D2D_RECT_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawBitmap(this, ::windows_core::from_raw_borrowed(&bitmap), ::core::mem::transmute_copy(&destinationrectangle), ::core::mem::transmute_copy(&opacity), ::core::mem::transmute_copy(&interpolationmode), ::core::mem::transmute_copy(&sourcerectangle)))
        }
        unsafe extern "system" fn DrawText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: ::windows_core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: *mut ::core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawText(this, ::core::mem::transmute(&string), ::core::mem::transmute_copy(&stringlength), ::windows_core::from_raw_borrowed(&textformat), ::core::mem::transmute_copy(&layoutrect), ::windows_core::from_raw_borrowed(&defaultfillbrush), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&measuringmode)))
        }
        unsafe extern "system" fn DrawTextLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: Common::D2D_POINT_2F, textlayout: *mut ::core::ffi::c_void, defaultfillbrush: *mut ::core::ffi::c_void, options: D2D1_DRAW_TEXT_OPTIONS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawTextLayout(this, ::core::mem::transmute(&origin), ::windows_core::from_raw_borrowed(&textlayout), ::windows_core::from_raw_borrowed(&defaultfillbrush), ::core::mem::transmute_copy(&options)))
        }
        unsafe extern "system" fn DrawGlyphRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: *mut ::core::ffi::c_void, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGlyphRun(this, ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::windows_core::from_raw_borrowed(&foregroundbrush), ::core::mem::transmute_copy(&measuringmode)))
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform(this, ::core::mem::transmute_copy(&transform)))
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransform(this, ::core::mem::transmute_copy(&transform)))
        }
        unsafe extern "system" fn SetAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, antialiasmode: D2D1_ANTIALIAS_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAntialiasMode(this, ::core::mem::transmute_copy(&antialiasmode)))
        }
        unsafe extern "system" fn GetAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_ANTIALIAS_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAntialiasMode(this))
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextAntialiasMode(this, ::core::mem::transmute_copy(&textantialiasmode)))
        }
        unsafe extern "system" fn GetTextAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextAntialiasMode(this))
        }
        unsafe extern "system" fn SetTextRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextRenderingParams(this, ::windows_core::from_raw_borrowed(&textrenderingparams)))
        }
        unsafe extern "system" fn GetTextRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textrenderingparams: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextRenderingParams(this, ::core::mem::transmute_copy(&textrenderingparams)))
        }
        unsafe extern "system" fn SetTags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: u64, tag2: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTags(this, ::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)))
        }
        unsafe extern "system" fn GetTags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTags(this, ::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)))
        }
        unsafe extern "system" fn PushLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushLayer(this, ::core::mem::transmute_copy(&layerparameters), ::windows_core::from_raw_borrowed(&layer)))
        }
        unsafe extern "system" fn PopLayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopLayer(this))
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this, ::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)).into())
        }
        unsafe extern "system" fn SaveDrawingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstateblock: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveDrawingState(this, ::windows_core::from_raw_borrowed(&drawingstateblock)))
        }
        unsafe extern "system" fn RestoreDrawingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingstateblock: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreDrawingState(this, ::windows_core::from_raw_borrowed(&drawingstateblock)))
        }
        unsafe extern "system" fn PushAxisAlignedClip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushAxisAlignedClip(this, ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&antialiasmode)))
        }
        unsafe extern "system" fn PopAxisAlignedClip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopAxisAlignedClip(this))
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clearcolor: *const Common::D2D1_COLOR_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this, ::core::mem::transmute_copy(&clearcolor)))
        }
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDraw(this))
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag1: *mut u64, tag2: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDraw(this, ::core::mem::transmute_copy(&tag1), ::core::mem::transmute_copy(&tag2)).into())
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_PIXEL_FORMAT) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetPixelFormat(this))
        }
        unsafe extern "system" fn SetDpi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: f32, dpiy: f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDpi(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)))
        }
        unsafe extern "system" fn GetDpi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDpi(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)))
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetSize(this))
        }
        unsafe extern "system" fn GetPixelSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_U) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetPixelSize(this))
        }
        unsafe extern "system" fn GetMaximumBitmapSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaximumBitmapSize(this))
        }
        unsafe extern "system" fn IsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSupported(this, ::core::mem::transmute_copy(&rendertargetproperties)))
        }
        ID2D1RenderTarget_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateBitmap: CreateBitmap::<Identity, Impl, OFFSET>,
            CreateBitmapFromWicBitmap: CreateBitmapFromWicBitmap::<Identity, Impl, OFFSET>,
            CreateSharedBitmap: CreateSharedBitmap::<Identity, Impl, OFFSET>,
            CreateBitmapBrush: CreateBitmapBrush::<Identity, Impl, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, Impl, OFFSET>,
            CreateGradientStopCollection: CreateGradientStopCollection::<Identity, Impl, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, Impl, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, Impl, OFFSET>,
            CreateCompatibleRenderTarget: CreateCompatibleRenderTarget::<Identity, Impl, OFFSET>,
            CreateLayer: CreateLayer::<Identity, Impl, OFFSET>,
            CreateMesh: CreateMesh::<Identity, Impl, OFFSET>,
            DrawLine: DrawLine::<Identity, Impl, OFFSET>,
            DrawRectangle: DrawRectangle::<Identity, Impl, OFFSET>,
            FillRectangle: FillRectangle::<Identity, Impl, OFFSET>,
            DrawRoundedRectangle: DrawRoundedRectangle::<Identity, Impl, OFFSET>,
            FillRoundedRectangle: FillRoundedRectangle::<Identity, Impl, OFFSET>,
            DrawEllipse: DrawEllipse::<Identity, Impl, OFFSET>,
            FillEllipse: FillEllipse::<Identity, Impl, OFFSET>,
            DrawGeometry: DrawGeometry::<Identity, Impl, OFFSET>,
            FillGeometry: FillGeometry::<Identity, Impl, OFFSET>,
            FillMesh: FillMesh::<Identity, Impl, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, Impl, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, Impl, OFFSET>,
            DrawText: DrawText::<Identity, Impl, OFFSET>,
            DrawTextLayout: DrawTextLayout::<Identity, Impl, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            SetAntialiasMode: SetAntialiasMode::<Identity, Impl, OFFSET>,
            GetAntialiasMode: GetAntialiasMode::<Identity, Impl, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, Impl, OFFSET>,
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, Impl, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, Impl, OFFSET>,
            GetTextRenderingParams: GetTextRenderingParams::<Identity, Impl, OFFSET>,
            SetTags: SetTags::<Identity, Impl, OFFSET>,
            GetTags: GetTags::<Identity, Impl, OFFSET>,
            PushLayer: PushLayer::<Identity, Impl, OFFSET>,
            PopLayer: PopLayer::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            SaveDrawingState: SaveDrawingState::<Identity, Impl, OFFSET>,
            RestoreDrawingState: RestoreDrawingState::<Identity, Impl, OFFSET>,
            PushAxisAlignedClip: PushAxisAlignedClip::<Identity, Impl, OFFSET>,
            PopAxisAlignedClip: PopAxisAlignedClip::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            SetDpi: SetDpi::<Identity, Impl, OFFSET>,
            GetDpi: GetDpi::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetPixelSize: GetPixelSize::<Identity, Impl, OFFSET>,
            GetMaximumBitmapSize: GetMaximumBitmapSize::<Identity, Impl, OFFSET>,
            IsSupported: IsSupported::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1Resource_Impl: ::windows_core::BaseImpl {
    fn GetFactory(this: &Self::This, factory: *mut ::core::option::Option<ID2D1Factory>);
}
impl ::windows_core::Iids for ID2D1Resource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Resource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Resource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Resource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFactory(this, ::core::mem::transmute_copy(&factory)))
        }
        ID2D1Resource_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetFactory: GetFactory::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1ResourceTexture_Impl: ::windows_core::BaseImpl {
    fn Update(this: &Self::This, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1ResourceTexture {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ResourceTexture_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1ResourceTexture {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1ResourceTexture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minimumextents: *const u32, maximimumextents: *const u32, strides: *const u32, dimensions: u32, data: *const u8, datacount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::core::mem::transmute_copy(&minimumextents), ::core::mem::transmute_copy(&maximimumextents), ::core::mem::transmute_copy(&strides), ::core::mem::transmute_copy(&dimensions), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datacount)).into())
        }
        ID2D1ResourceTexture_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1RoundedRectangleGeometry_Impl: ::windows_core::BaseImpl + ID2D1Geometry_Impl {
    fn GetRoundedRect(this: &Self::This, roundedrect: *mut D2D1_ROUNDED_RECT);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1RoundedRectangleGeometry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Geometry);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RoundedRectangleGeometry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1RoundedRectangleGeometry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRoundedRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1RoundedRectangleGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roundedrect: *mut D2D1_ROUNDED_RECT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRoundedRect(this, ::core::mem::transmute_copy(&roundedrect)))
        }
        ID2D1RoundedRectangleGeometry_Vtbl { base__: <ID2D1Geometry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetRoundedRect: GetRoundedRect::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SolidColorBrush_Impl: ::windows_core::BaseImpl + ID2D1Brush_Impl {
    fn SetColor(this: &Self::This, color: *const Common::D2D1_COLOR_F);
    fn GetColor(this: &Self::This) -> Common::D2D1_COLOR_F;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1SolidColorBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Brush);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SolidColorBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SolidColorBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SolidColorBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColor(this, ::core::mem::transmute_copy(&color)))
        }
        unsafe extern "system" fn GetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SolidColorBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D1_COLOR_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetColor(this))
        }
        ID2D1SolidColorBrush_Vtbl {
            base__: <ID2D1Brush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            GetColor: GetColor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SourceTransform_Impl: ::windows_core::BaseImpl + ID2D1Transform_Impl {
    fn SetRenderInfo(this: &Self::This, renderinfo: ::core::option::Option<&ID2D1RenderInfo>) -> ::windows_core::Result<()>;
    fn Draw(this: &Self::This, target: ::core::option::Option<&ID2D1Bitmap1>, drawrect: *const super::super::Foundation::RECT, targetorigin: &Common::D2D_POINT_2U) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1SourceTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Transform);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SourceTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SourceTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRenderInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRenderInfo(this, ::windows_core::from_raw_borrowed(&renderinfo)).into())
        }
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, drawrect: *const super::super::Foundation::RECT, targetorigin: Common::D2D_POINT_2U) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::windows_core::from_raw_borrowed(&target), ::core::mem::transmute_copy(&drawrect), ::core::mem::transmute(&targetorigin)).into())
        }
        ID2D1SourceTransform_Vtbl {
            base__: <ID2D1Transform as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRenderInfo: SetRenderInfo::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1SpriteBatch_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn AddSprites(this: &Self::This, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows_core::Result<()>;
    fn SetSprites(this: &Self::This, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows_core::Result<()>;
    fn GetSprites(this: &Self::This, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()>;
    fn GetSpriteCount(this: &Self::This) -> u32;
    fn Clear(this: &Self::This);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1SpriteBatch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SpriteBatch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SpriteBatch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddSprites<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SpriteBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSprites(this, ::core::mem::transmute_copy(&spritecount), ::core::mem::transmute_copy(&destinationrectangles), ::core::mem::transmute_copy(&sourcerectangles), ::core::mem::transmute_copy(&colors), ::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&destinationrectanglesstride), ::core::mem::transmute_copy(&sourcerectanglesstride), ::core::mem::transmute_copy(&colorsstride), ::core::mem::transmute_copy(&transformsstride)).into())
        }
        unsafe extern "system" fn SetSprites<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SpriteBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *const Common::D2D_RECT_F, sourcerectangles: *const Common::D2D_RECT_U, colors: *const Common::D2D1_COLOR_F, transforms: *const super::super::super::Foundation::Numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::SetSprites(this, ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&spritecount), ::core::mem::transmute_copy(&destinationrectangles), ::core::mem::transmute_copy(&sourcerectangles), ::core::mem::transmute_copy(&colors), ::core::mem::transmute_copy(&transforms), ::core::mem::transmute_copy(&destinationrectanglesstride), ::core::mem::transmute_copy(&sourcerectanglesstride), ::core::mem::transmute_copy(&colorsstride), ::core::mem::transmute_copy(&transformsstride)).into()
            })
        }
        unsafe extern "system" fn GetSprites<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SpriteBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *mut Common::D2D_RECT_F, sourcerectangles: *mut Common::D2D_RECT_U, colors: *mut Common::D2D1_COLOR_F, transforms: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSprites(this, ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&spritecount), ::core::mem::transmute_copy(&destinationrectangles), ::core::mem::transmute_copy(&sourcerectangles), ::core::mem::transmute_copy(&colors), ::core::mem::transmute_copy(&transforms)).into())
        }
        unsafe extern "system" fn GetSpriteCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SpriteBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpriteCount(this))
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SpriteBatch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this))
        }
        ID2D1SpriteBatch_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddSprites: AddSprites::<Identity, Impl, OFFSET>,
            SetSprites: SetSprites::<Identity, Impl, OFFSET>,
            GetSprites: GetSprites::<Identity, Impl, OFFSET>,
            GetSpriteCount: GetSpriteCount::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1StrokeStyle_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetStartCap(this: &Self::This) -> D2D1_CAP_STYLE;
    fn GetEndCap(this: &Self::This) -> D2D1_CAP_STYLE;
    fn GetDashCap(this: &Self::This) -> D2D1_CAP_STYLE;
    fn GetMiterLimit(this: &Self::This) -> f32;
    fn GetLineJoin(this: &Self::This) -> D2D1_LINE_JOIN;
    fn GetDashOffset(this: &Self::This) -> f32;
    fn GetDashStyle(this: &Self::This) -> D2D1_DASH_STYLE;
    fn GetDashesCount(this: &Self::This) -> u32;
    fn GetDashes(this: &Self::This, dashes: *mut f32, dashescount: u32);
}
impl ::windows_core::Iids for ID2D1StrokeStyle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1StrokeStyle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStartCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStartCap(this))
        }
        unsafe extern "system" fn GetEndCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEndCap(this))
        }
        unsafe extern "system" fn GetDashCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_CAP_STYLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashCap(this))
        }
        unsafe extern "system" fn GetMiterLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMiterLimit(this))
        }
        unsafe extern "system" fn GetLineJoin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_LINE_JOIN {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineJoin(this))
        }
        unsafe extern "system" fn GetDashOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashOffset(this))
        }
        unsafe extern "system" fn GetDashStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_DASH_STYLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashStyle(this))
        }
        unsafe extern "system" fn GetDashesCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashesCount(this))
        }
        unsafe extern "system" fn GetDashes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashes(this, ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount)))
        }
        ID2D1StrokeStyle_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStartCap: GetStartCap::<Identity, Impl, OFFSET>,
            GetEndCap: GetEndCap::<Identity, Impl, OFFSET>,
            GetDashCap: GetDashCap::<Identity, Impl, OFFSET>,
            GetMiterLimit: GetMiterLimit::<Identity, Impl, OFFSET>,
            GetLineJoin: GetLineJoin::<Identity, Impl, OFFSET>,
            GetDashOffset: GetDashOffset::<Identity, Impl, OFFSET>,
            GetDashStyle: GetDashStyle::<Identity, Impl, OFFSET>,
            GetDashesCount: GetDashesCount::<Identity, Impl, OFFSET>,
            GetDashes: GetDashes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1StrokeStyle1_Impl: ::windows_core::BaseImpl + ID2D1StrokeStyle_Impl {
    fn GetStrokeTransformType(this: &Self::This) -> D2D1_STROKE_TRANSFORM_TYPE;
}
impl ::windows_core::Iids for ID2D1StrokeStyle1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1StrokeStyle);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1StrokeStyle1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStrokeTransformType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1StrokeStyle1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_STROKE_TRANSFORM_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStrokeTransformType(this))
        }
        ID2D1StrokeStyle1_Vtbl {
            base__: <ID2D1StrokeStyle as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStrokeTransformType: GetStrokeTransformType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1SvgAttribute_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetElement(this: &Self::This, element: *mut ::core::option::Option<ID2D1SvgElement>);
    fn Clone(this: &Self::This) -> ::windows_core::Result<ID2D1SvgAttribute>;
}
impl ::windows_core::Iids for ID2D1SvgAttribute {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgAttribute_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgAttribute {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetElement(this, ::core::mem::transmute_copy(&element)))
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attribute: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attribute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1SvgAttribute_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
pub trait ID2D1SvgDocument_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn SetViewportSize(this: &Self::This, viewportsize: &Common::D2D_SIZE_F) -> ::windows_core::Result<()>;
    fn GetViewportSize(this: &Self::This) -> Common::D2D_SIZE_F;
    fn SetRoot(this: &Self::This, root: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<()>;
    fn GetRoot(this: &Self::This, root: *mut ::core::option::Option<ID2D1SvgElement>);
    fn FindElementById(this: &Self::This, id: &::windows_core::PCWSTR) -> ::windows_core::Result<ID2D1SvgElement>;
    fn Serialize(this: &Self::This, outputxmlstream: ::core::option::Option<&super::super::System::Com::IStream>, subtree: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<()>;
    fn Deserialize(this: &Self::This, inputxmlstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<ID2D1SvgElement>;
    fn CreatePaint(this: &Self::This, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: &::windows_core::PCWSTR) -> ::windows_core::Result<ID2D1SvgPaint>;
    fn CreateStrokeDashArray(this: &Self::This, dashes: *const D2D1_SVG_LENGTH, dashescount: u32) -> ::windows_core::Result<ID2D1SvgStrokeDashArray>;
    fn CreatePointCollection(this: &Self::This, points: *const Common::D2D_POINT_2F, pointscount: u32) -> ::windows_core::Result<ID2D1SvgPointCollection>;
    fn CreatePathData(this: &Self::This, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32) -> ::windows_core::Result<ID2D1SvgPathData>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ID2D1SvgDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetViewportSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewportsize: Common::D2D_SIZE_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewportSize(this, ::core::mem::transmute(&viewportsize)).into())
        }
        unsafe extern "system" fn GetViewportSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Common::D2D_SIZE_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| *result__ = Impl::GetViewportSize(this))
        }
        unsafe extern "system" fn SetRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, root: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRoot(this, ::windows_core::from_raw_borrowed(&root)).into())
        }
        unsafe extern "system" fn GetRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRoot(this, ::core::mem::transmute_copy(&root)))
        }
        unsafe extern "system" fn FindElementById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::windows_core::PCWSTR, svgelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindElementById(this, ::core::mem::transmute(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(svgelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputxmlstream: *mut ::core::ffi::c_void, subtree: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::windows_core::from_raw_borrowed(&outputxmlstream), ::windows_core::from_raw_borrowed(&subtree)).into())
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputxmlstream: *mut ::core::ffi::c_void, subtree: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Deserialize(this, ::windows_core::from_raw_borrowed(&inputxmlstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subtree, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePaint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE, color: *const Common::D2D1_COLOR_F, id: ::windows_core::PCWSTR, paint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePaint(this, ::core::mem::transmute_copy(&painttype), ::core::mem::transmute_copy(&color), ::core::mem::transmute(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStrokeDashArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, strokedasharray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStrokeDashArray(this, ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokedasharray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePointCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, pointcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePointCollection(this, ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointscount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pointcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePathData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, pathdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePathData(this, ::core::mem::transmute_copy(&segmentdata), ::core::mem::transmute_copy(&segmentdatacount), ::core::mem::transmute_copy(&commands), ::core::mem::transmute_copy(&commandscount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pathdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1SvgDocument_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetViewportSize: SetViewportSize::<Identity, Impl, OFFSET>,
            GetViewportSize: GetViewportSize::<Identity, Impl, OFFSET>,
            SetRoot: SetRoot::<Identity, Impl, OFFSET>,
            GetRoot: GetRoot::<Identity, Impl, OFFSET>,
            FindElementById: FindElementById::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
            CreatePaint: CreatePaint::<Identity, Impl, OFFSET>,
            CreateStrokeDashArray: CreateStrokeDashArray::<Identity, Impl, OFFSET>,
            CreatePointCollection: CreatePointCollection::<Identity, Impl, OFFSET>,
            CreatePathData: CreatePathData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1SvgElement_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn GetDocument(this: &Self::This, document: *mut ::core::option::Option<ID2D1SvgDocument>);
    fn GetTagName(this: &Self::This, name: ::windows_core::PWSTR, namecount: u32) -> ::windows_core::Result<()>;
    fn GetTagNameLength(this: &Self::This) -> u32;
    fn IsTextContent(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetParent(this: &Self::This, parent: *mut ::core::option::Option<ID2D1SvgElement>);
    fn HasChildren(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetFirstChild(this: &Self::This, child: *mut ::core::option::Option<ID2D1SvgElement>);
    fn GetLastChild(this: &Self::This, child: *mut ::core::option::Option<ID2D1SvgElement>);
    fn GetPreviousChild(this: &Self::This, referencechild: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<ID2D1SvgElement>;
    fn GetNextChild(this: &Self::This, referencechild: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<ID2D1SvgElement>;
    fn InsertChildBefore(this: &Self::This, newchild: ::core::option::Option<&ID2D1SvgElement>, referencechild: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<()>;
    fn AppendChild(this: &Self::This, newchild: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<()>;
    fn ReplaceChild(this: &Self::This, newchild: ::core::option::Option<&ID2D1SvgElement>, oldchild: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<()>;
    fn RemoveChild(this: &Self::This, oldchild: ::core::option::Option<&ID2D1SvgElement>) -> ::windows_core::Result<()>;
    fn CreateChild(this: &Self::This, tagname: &::windows_core::PCWSTR) -> ::windows_core::Result<ID2D1SvgElement>;
    fn IsAttributeSpecified(this: &Self::This, name: &::windows_core::PCWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetSpecifiedAttributeCount(this: &Self::This) -> u32;
    fn GetSpecifiedAttributeName(this: &Self::This, index: u32, name: ::windows_core::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSpecifiedAttributeNameLength(this: &Self::This, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RemoveAttribute(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetTextValue(this: &Self::This, name: &::windows_core::PCWSTR, namecount: u32) -> ::windows_core::Result<()>;
    fn GetTextValue(this: &Self::This, name: ::windows_core::PWSTR, namecount: u32) -> ::windows_core::Result<()>;
    fn GetTextValueLength(this: &Self::This) -> u32;
    fn SetAttributeValue(this: &Self::This, name: &::windows_core::PCWSTR, value: ::core::option::Option<&ID2D1SvgAttribute>) -> ::windows_core::Result<()>;
    fn SetAttributeValue2(this: &Self::This, name: &::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows_core::Result<()>;
    fn SetAttributeValue3(this: &Self::This, name: &::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAttributeValue(this: &Self::This, name: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAttributeValue2(this: &Self::This, name: &::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows_core::Result<()>;
    fn GetAttributeValue3(this: &Self::This, name: &::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: ::windows_core::PWSTR, valuecount: u32) -> ::windows_core::Result<()>;
    fn GetAttributeValueLength(this: &Self::This, name: &::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1SvgElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocument(this, ::core::mem::transmute_copy(&document)))
        }
        unsafe extern "system" fn GetTagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PWSTR, namecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTagName(this, ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&namecount)).into())
        }
        unsafe extern "system" fn GetTagNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTagNameLength(this))
        }
        unsafe extern "system" fn IsTextContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsTextContent(this))
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParent(this, ::core::mem::transmute_copy(&parent)))
        }
        unsafe extern "system" fn HasChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasChildren(this))
        }
        unsafe extern "system" fn GetFirstChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFirstChild(this, ::core::mem::transmute_copy(&child)))
        }
        unsafe extern "system" fn GetLastChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastChild(this, ::core::mem::transmute_copy(&child)))
        }
        unsafe extern "system" fn GetPreviousChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, previouschild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousChild(this, ::windows_core::from_raw_borrowed(&referencechild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previouschild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, nextchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextChild(this, ::windows_core::from_raw_borrowed(&referencechild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertChildBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertChildBefore(this, ::windows_core::from_raw_borrowed(&newchild), ::windows_core::from_raw_borrowed(&referencechild)).into())
        }
        unsafe extern "system" fn AppendChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppendChild(this, ::windows_core::from_raw_borrowed(&newchild)).into())
        }
        unsafe extern "system" fn ReplaceChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplaceChild(this, ::windows_core::from_raw_borrowed(&newchild), ::windows_core::from_raw_borrowed(&oldchild)).into())
        }
        unsafe extern "system" fn RemoveChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveChild(this, ::windows_core::from_raw_borrowed(&oldchild)).into())
        }
        unsafe extern "system" fn CreateChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::windows_core::PCWSTR, newchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateChild(this, ::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAttributeSpecified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, inherited: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsAttributeSpecified(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&inherited)))
        }
        unsafe extern "system" fn GetSpecifiedAttributeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpecifiedAttributeCount(this))
        }
        unsafe extern "system" fn GetSpecifiedAttributeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, name: ::windows_core::PWSTR, namecount: u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpecifiedAttributeName(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&namecount), ::core::mem::transmute_copy(&inherited)).into())
        }
        unsafe extern "system" fn GetSpecifiedAttributeNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, namelength: *mut u32, inherited: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpecifiedAttributeNameLength(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&inherited)).into())
        }
        unsafe extern "system" fn RemoveAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAttribute(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn SetTextValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, namecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextValue(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&namecount)).into())
        }
        unsafe extern "system" fn GetTextValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PWSTR, namecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextValue(this, ::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&namecount)).into())
        }
        unsafe extern "system" fn GetTextValueLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextValueLength(this))
        }
        unsafe extern "system" fn SetAttributeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributeValue(this, ::core::mem::transmute(&name), ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn SetAttributeValue2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributeValue2(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&valuesizeinbytes)).into())
        }
        unsafe extern "system" fn SetAttributeValue3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributeValue3(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeValue(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetAttributeValue2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut ::core::ffi::c_void, valuesizeinbytes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeValue2(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&valuesizeinbytes)).into())
        }
        unsafe extern "system" fn GetAttributeValue3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: ::windows_core::PWSTR, valuecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeValue3(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&valuecount)).into())
        }
        unsafe extern "system" fn GetAttributeValueLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, valuelength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeValueLength(this, ::core::mem::transmute(&name), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valuelength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1SvgElement_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
            GetTagName: GetTagName::<Identity, Impl, OFFSET>,
            GetTagNameLength: GetTagNameLength::<Identity, Impl, OFFSET>,
            IsTextContent: IsTextContent::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
            HasChildren: HasChildren::<Identity, Impl, OFFSET>,
            GetFirstChild: GetFirstChild::<Identity, Impl, OFFSET>,
            GetLastChild: GetLastChild::<Identity, Impl, OFFSET>,
            GetPreviousChild: GetPreviousChild::<Identity, Impl, OFFSET>,
            GetNextChild: GetNextChild::<Identity, Impl, OFFSET>,
            InsertChildBefore: InsertChildBefore::<Identity, Impl, OFFSET>,
            AppendChild: AppendChild::<Identity, Impl, OFFSET>,
            ReplaceChild: ReplaceChild::<Identity, Impl, OFFSET>,
            RemoveChild: RemoveChild::<Identity, Impl, OFFSET>,
            CreateChild: CreateChild::<Identity, Impl, OFFSET>,
            IsAttributeSpecified: IsAttributeSpecified::<Identity, Impl, OFFSET>,
            GetSpecifiedAttributeCount: GetSpecifiedAttributeCount::<Identity, Impl, OFFSET>,
            GetSpecifiedAttributeName: GetSpecifiedAttributeName::<Identity, Impl, OFFSET>,
            GetSpecifiedAttributeNameLength: GetSpecifiedAttributeNameLength::<Identity, Impl, OFFSET>,
            RemoveAttribute: RemoveAttribute::<Identity, Impl, OFFSET>,
            SetTextValue: SetTextValue::<Identity, Impl, OFFSET>,
            GetTextValue: GetTextValue::<Identity, Impl, OFFSET>,
            GetTextValueLength: GetTextValueLength::<Identity, Impl, OFFSET>,
            SetAttributeValue: SetAttributeValue::<Identity, Impl, OFFSET>,
            SetAttributeValue2: SetAttributeValue2::<Identity, Impl, OFFSET>,
            SetAttributeValue3: SetAttributeValue3::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetAttributeValue2: GetAttributeValue2::<Identity, Impl, OFFSET>,
            GetAttributeValue3: GetAttributeValue3::<Identity, Impl, OFFSET>,
            GetAttributeValueLength: GetAttributeValueLength::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1SvgGlyphStyle_Impl: ::windows_core::BaseImpl + ID2D1Resource_Impl {
    fn SetFill(this: &Self::This, brush: ::core::option::Option<&ID2D1Brush>) -> ::windows_core::Result<()>;
    fn GetFill(this: &Self::This, brush: *mut ::core::option::Option<ID2D1Brush>);
    fn SetStroke(this: &Self::This, brush: ::core::option::Option<&ID2D1Brush>, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> ::windows_core::Result<()>;
    fn GetStrokeDashesCount(this: &Self::This) -> u32;
    fn GetStroke(this: &Self::This, brush: *mut ::core::option::Option<ID2D1Brush>, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32);
}
impl ::windows_core::Iids for ID2D1SvgGlyphStyle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Resource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgGlyphStyle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgGlyphStyle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFill<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgGlyphStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFill(this, ::windows_core::from_raw_borrowed(&brush)).into())
        }
        unsafe extern "system" fn GetFill<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgGlyphStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFill(this, ::core::mem::transmute_copy(&brush)))
        }
        unsafe extern "system" fn SetStroke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgGlyphStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStroke(this, ::windows_core::from_raw_borrowed(&brush), ::core::mem::transmute_copy(&strokewidth), ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount), ::core::mem::transmute_copy(&dashoffset)).into())
        }
        unsafe extern "system" fn GetStrokeDashesCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgGlyphStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStrokeDashesCount(this))
        }
        unsafe extern "system" fn GetStroke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgGlyphStyle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStroke(this, ::core::mem::transmute_copy(&brush), ::core::mem::transmute_copy(&strokewidth), ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount), ::core::mem::transmute_copy(&dashoffset)))
        }
        ID2D1SvgGlyphStyle_Vtbl {
            base__: <ID2D1Resource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFill: SetFill::<Identity, Impl, OFFSET>,
            GetFill: GetFill::<Identity, Impl, OFFSET>,
            SetStroke: SetStroke::<Identity, Impl, OFFSET>,
            GetStrokeDashesCount: GetStrokeDashesCount::<Identity, Impl, OFFSET>,
            GetStroke: GetStroke::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SvgPaint_Impl: ::windows_core::BaseImpl + ID2D1SvgAttribute_Impl {
    fn SetPaintType(this: &Self::This, painttype: D2D1_SVG_PAINT_TYPE) -> ::windows_core::Result<()>;
    fn GetPaintType(this: &Self::This) -> D2D1_SVG_PAINT_TYPE;
    fn SetColor(this: &Self::This, color: *const Common::D2D1_COLOR_F) -> ::windows_core::Result<()>;
    fn GetColor(this: &Self::This, color: *mut Common::D2D1_COLOR_F);
    fn SetId(this: &Self::This, id: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetId(this: &Self::This, id: ::windows_core::PWSTR, idcount: u32) -> ::windows_core::Result<()>;
    fn GetIdLength(this: &Self::This) -> u32;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1SvgPaint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1SvgAttribute);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgPaint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPaintType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPaintType(this, ::core::mem::transmute_copy(&painttype)).into())
        }
        unsafe extern "system" fn GetPaintType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPaintType(this))
        }
        unsafe extern "system" fn SetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const Common::D2D1_COLOR_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColor(this, ::core::mem::transmute_copy(&color)).into())
        }
        unsafe extern "system" fn GetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *mut Common::D2D1_COLOR_F) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColor(this, ::core::mem::transmute_copy(&color)))
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::core::mem::transmute(&id)).into())
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::windows_core::PWSTR, idcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetId(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&idcount)).into())
        }
        unsafe extern "system" fn GetIdLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPaint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdLength(this))
        }
        ID2D1SvgPaint_Vtbl {
            base__: <ID2D1SvgAttribute as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPaintType: SetPaintType::<Identity, Impl, OFFSET>,
            GetPaintType: GetPaintType::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            GetColor: GetColor::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetIdLength: GetIdLength::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SvgPathData_Impl: ::windows_core::BaseImpl + ID2D1SvgAttribute_Impl {
    fn RemoveSegmentDataAtEnd(this: &Self::This, datacount: u32) -> ::windows_core::Result<()>;
    fn UpdateSegmentData(this: &Self::This, data: *const f32, datacount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetSegmentData(this: &Self::This, data: *mut f32, datacount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetSegmentDataCount(this: &Self::This) -> u32;
    fn RemoveCommandsAtEnd(this: &Self::This, commandscount: u32) -> ::windows_core::Result<()>;
    fn UpdateCommands(this: &Self::This, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetCommands(this: &Self::This, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetCommandsCount(this: &Self::This) -> u32;
    fn CreatePathGeometry(this: &Self::This, fillmode: Common::D2D1_FILL_MODE) -> ::windows_core::Result<ID2D1PathGeometry1>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1SvgPathData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1SvgAttribute);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgPathData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RemoveSegmentDataAtEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datacount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSegmentDataAtEnd(this, ::core::mem::transmute_copy(&datacount)).into())
        }
        unsafe extern "system" fn UpdateSegmentData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *const f32, datacount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateSegmentData(this, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetSegmentData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut f32, datacount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSegmentData(this, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSegmentDataCount(this))
        }
        unsafe extern "system" fn RemoveCommandsAtEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveCommandsAtEnd(this, ::core::mem::transmute_copy(&commandscount)).into())
        }
        unsafe extern "system" fn UpdateCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateCommands(this, ::core::mem::transmute_copy(&commands), ::core::mem::transmute_copy(&commandscount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCommands(this, ::core::mem::transmute_copy(&commands), ::core::mem::transmute_copy(&commandscount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetCommandsCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCommandsCount(this))
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPathData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillmode: Common::D2D1_FILL_MODE, pathgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePathGeometry(this, ::core::mem::transmute_copy(&fillmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pathgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1SvgPathData_Vtbl {
            base__: <ID2D1SvgAttribute as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RemoveSegmentDataAtEnd: RemoveSegmentDataAtEnd::<Identity, Impl, OFFSET>,
            UpdateSegmentData: UpdateSegmentData::<Identity, Impl, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, Impl, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, Impl, OFFSET>,
            RemoveCommandsAtEnd: RemoveCommandsAtEnd::<Identity, Impl, OFFSET>,
            UpdateCommands: UpdateCommands::<Identity, Impl, OFFSET>,
            GetCommands: GetCommands::<Identity, Impl, OFFSET>,
            GetCommandsCount: GetCommandsCount::<Identity, Impl, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1SvgPointCollection_Impl: ::windows_core::BaseImpl + ID2D1SvgAttribute_Impl {
    fn RemovePointsAtEnd(this: &Self::This, pointscount: u32) -> ::windows_core::Result<()>;
    fn UpdatePoints(this: &Self::This, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetPoints(this: &Self::This, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetPointsCount(this: &Self::This) -> u32;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1SvgPointCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1SvgAttribute);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPointCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgPointCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RemovePointsAtEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPointCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointsAtEnd(this, ::core::mem::transmute_copy(&pointscount)).into())
        }
        unsafe extern "system" fn UpdatePoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPointCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *const Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdatePoints(this, ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointscount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetPoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPointCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *mut Common::D2D_POINT_2F, pointscount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPoints(this, ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointscount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetPointsCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgPointCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPointsCount(this))
        }
        ID2D1SvgPointCollection_Vtbl {
            base__: <ID2D1SvgAttribute as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RemovePointsAtEnd: RemovePointsAtEnd::<Identity, Impl, OFFSET>,
            UpdatePoints: UpdatePoints::<Identity, Impl, OFFSET>,
            GetPoints: GetPoints::<Identity, Impl, OFFSET>,
            GetPointsCount: GetPointsCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1SvgStrokeDashArray_Impl: ::windows_core::BaseImpl + ID2D1SvgAttribute_Impl {
    fn RemoveDashesAtEnd(this: &Self::This, dashescount: u32) -> ::windows_core::Result<()>;
    fn UpdateDashes(this: &Self::This, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn UpdateDashes2(this: &Self::This, dashes: *const f32, dashescount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetDashes(this: &Self::This, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetDashes2(this: &Self::This, dashes: *mut f32, dashescount: u32, startindex: u32) -> ::windows_core::Result<()>;
    fn GetDashesCount(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for ID2D1SvgStrokeDashArray {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1SvgAttribute);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgStrokeDashArray_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SvgStrokeDashArray {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RemoveDashesAtEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgStrokeDashArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashescount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveDashesAtEnd(this, ::core::mem::transmute_copy(&dashescount)).into())
        }
        unsafe extern "system" fn UpdateDashes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgStrokeDashArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateDashes(this, ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn UpdateDashes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgStrokeDashArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *const f32, dashescount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateDashes2(this, ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetDashes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgStrokeDashArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashes(this, ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetDashes2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgStrokeDashArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dashes: *mut f32, dashescount: u32, startindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashes2(this, ::core::mem::transmute_copy(&dashes), ::core::mem::transmute_copy(&dashescount), ::core::mem::transmute_copy(&startindex)).into())
        }
        unsafe extern "system" fn GetDashesCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SvgStrokeDashArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDashesCount(this))
        }
        ID2D1SvgStrokeDashArray_Vtbl {
            base__: <ID2D1SvgAttribute as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RemoveDashesAtEnd: RemoveDashesAtEnd::<Identity, Impl, OFFSET>,
            UpdateDashes: UpdateDashes::<Identity, Impl, OFFSET>,
            UpdateDashes2: UpdateDashes2::<Identity, Impl, OFFSET>,
            GetDashes: GetDashes::<Identity, Impl, OFFSET>,
            GetDashes2: GetDashes2::<Identity, Impl, OFFSET>,
            GetDashesCount: GetDashesCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1TessellationSink_Impl: ::windows_core::BaseImpl {
    fn AddTriangles(this: &Self::This, triangles: *const D2D1_TRIANGLE, trianglescount: u32);
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::windows_core::Iids for ID2D1TessellationSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TessellationSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1TessellationSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddTriangles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TessellationSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triangles: *const D2D1_TRIANGLE, trianglescount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTriangles(this, ::core::mem::transmute_copy(&triangles), ::core::mem::transmute_copy(&trianglescount)))
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TessellationSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ID2D1TessellationSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddTriangles: AddTriangles::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ID2D1Transform_Impl: ::windows_core::BaseImpl + ID2D1TransformNode_Impl {
    fn MapOutputRectToInputRects(this: &Self::This, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> ::windows_core::Result<()>;
    fn MapInputRectsToOutputRect(this: &Self::This, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn MapInvalidRect(this: &Self::This, inputindex: u32, invalidinputrect: &super::super::Foundation::RECT) -> ::windows_core::Result<super::super::Foundation::RECT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ID2D1Transform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1TransformNode);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Transform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1Transform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MapOutputRectToInputRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Transform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputrect: *const super::super::Foundation::RECT, inputrects: *mut super::super::Foundation::RECT, inputrectscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapOutputRectToInputRects(this, ::core::mem::transmute_copy(&outputrect), ::core::mem::transmute_copy(&inputrects), ::core::mem::transmute_copy(&inputrectscount)).into())
        }
        unsafe extern "system" fn MapInputRectsToOutputRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Transform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapInputRectsToOutputRect(this, ::core::mem::transmute_copy(&inputrects), ::core::mem::transmute_copy(&inputopaquesubrects), ::core::mem::transmute_copy(&inputrectcount), ::core::mem::transmute_copy(&outputrect), ::core::mem::transmute_copy(&outputopaquesubrect)).into())
        }
        unsafe extern "system" fn MapInvalidRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1Transform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputindex: u32, invalidinputrect: super::super::Foundation::RECT, invalidoutputrect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapInvalidRect(this, ::core::mem::transmute_copy(&inputindex), ::core::mem::transmute(&invalidinputrect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(invalidoutputrect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ID2D1Transform_Vtbl {
            base__: <ID2D1TransformNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MapOutputRectToInputRects: MapOutputRectToInputRects::<Identity, Impl, OFFSET>,
            MapInputRectsToOutputRect: MapInputRectsToOutputRect::<Identity, Impl, OFFSET>,
            MapInvalidRect: MapInvalidRect::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1TransformGraph_Impl: ::windows_core::BaseImpl {
    fn GetInputCount(this: &Self::This) -> u32;
    fn SetSingleTransformNode(this: &Self::This, node: ::core::option::Option<&ID2D1TransformNode>) -> ::windows_core::Result<()>;
    fn AddNode(this: &Self::This, node: ::core::option::Option<&ID2D1TransformNode>) -> ::windows_core::Result<()>;
    fn RemoveNode(this: &Self::This, node: ::core::option::Option<&ID2D1TransformNode>) -> ::windows_core::Result<()>;
    fn SetOutputNode(this: &Self::This, node: ::core::option::Option<&ID2D1TransformNode>) -> ::windows_core::Result<()>;
    fn ConnectNode(this: &Self::This, fromnode: ::core::option::Option<&ID2D1TransformNode>, tonode: ::core::option::Option<&ID2D1TransformNode>, tonodeinputindex: u32) -> ::windows_core::Result<()>;
    fn ConnectToEffectInput(this: &Self::This, toeffectinputindex: u32, node: ::core::option::Option<&ID2D1TransformNode>, tonodeinputindex: u32) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This);
    fn SetPassthroughGraph(this: &Self::This, effectinputindex: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1TransformGraph {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1TransformGraph {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputCount(this))
        }
        unsafe extern "system" fn SetSingleTransformNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSingleTransformNode(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        unsafe extern "system" fn AddNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNode(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        unsafe extern "system" fn RemoveNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveNode(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        unsafe extern "system" fn SetOutputNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutputNode(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        unsafe extern "system" fn ConnectNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fromnode: *mut ::core::ffi::c_void, tonode: *mut ::core::ffi::c_void, tonodeinputindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectNode(this, ::windows_core::from_raw_borrowed(&fromnode), ::windows_core::from_raw_borrowed(&tonode), ::core::mem::transmute_copy(&tonodeinputindex)).into())
        }
        unsafe extern "system" fn ConnectToEffectInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, toeffectinputindex: u32, node: *mut ::core::ffi::c_void, tonodeinputindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectToEffectInput(this, ::core::mem::transmute_copy(&toeffectinputindex), ::windows_core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&tonodeinputindex)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this))
        }
        unsafe extern "system" fn SetPassthroughGraph<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformGraph_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectinputindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassthroughGraph(this, ::core::mem::transmute_copy(&effectinputindex)).into())
        }
        ID2D1TransformGraph_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            SetSingleTransformNode: SetSingleTransformNode::<Identity, Impl, OFFSET>,
            AddNode: AddNode::<Identity, Impl, OFFSET>,
            RemoveNode: RemoveNode::<Identity, Impl, OFFSET>,
            SetOutputNode: SetOutputNode::<Identity, Impl, OFFSET>,
            ConnectNode: ConnectNode::<Identity, Impl, OFFSET>,
            ConnectToEffectInput: ConnectToEffectInput::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            SetPassthroughGraph: SetPassthroughGraph::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1TransformNode_Impl: ::windows_core::BaseImpl {
    fn GetInputCount(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for ID2D1TransformNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1TransformNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputCount(this))
        }
        ID2D1TransformNode_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetInputCount: GetInputCount::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait ID2D1TransformedGeometry_Impl: ::windows_core::BaseImpl + ID2D1Geometry_Impl {
    fn GetSourceGeometry(this: &Self::This, sourcegeometry: *mut ::core::option::Option<ID2D1Geometry>);
    fn GetTransform(this: &Self::This, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for ID2D1TransformedGeometry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Geometry);
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformedGeometry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1TransformedGeometry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSourceGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformedGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcegeometry: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourceGeometry(this, ::core::mem::transmute_copy(&sourcegeometry)))
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformedGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTransform(this, ::core::mem::transmute_copy(&transform)))
        }
        ID2D1TransformedGeometry_Vtbl {
            base__: <ID2D1Geometry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSourceGeometry: GetSourceGeometry::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1TransformedImageSource_Impl: ::windows_core::BaseImpl + ID2D1Image_Impl {
    fn GetSource(this: &Self::This, imagesource: *mut ::core::option::Option<ID2D1ImageSource>);
    fn GetProperties(this: &Self::This, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES);
}
impl ::windows_core::Iids for ID2D1TransformedImageSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ID2D1Image);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformedImageSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1TransformedImageSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformedImageSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagesource: *mut *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSource(this, ::core::mem::transmute_copy(&imagesource)))
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1TransformedImageSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&properties)))
        }
        ID2D1TransformedImageSource_Vtbl {
            base__: <ID2D1Image as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID2D1VertexBuffer_Impl: ::windows_core::BaseImpl {
    fn Map(this: &Self::This, data: *mut *mut u8, buffersize: u32) -> ::windows_core::Result<()>;
    fn Unmap(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1VertexBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1VertexBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1VertexBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Map<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1VertexBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, buffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Map(this, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&buffersize)).into())
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1VertexBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this).into())
        }
        ID2D1VertexBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
