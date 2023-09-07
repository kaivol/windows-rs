#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
pub trait IPdfRendererNative_Impl: ::windows_core::BaseImpl {
    fn RenderPageToSurface(this: &Self::This, pdfpage: ::core::option::Option<&::windows_core::IUnknown>, psurface: ::core::option::Option<&super::super::super::Graphics::Dxgi::IDXGISurface>, offset: &super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::Result<()>;
    fn RenderPageToDeviceContext(this: &Self::This, pdfpage: ::core::option::Option<&::windows_core::IUnknown>, pd2ddevicecontext: ::core::option::Option<&super::super::super::Graphics::Direct2D::ID2D1DeviceContext>, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl ::windows_core::Iids for IPdfRendererNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPdfRendererNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPdfRendererNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RenderPageToSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPdfRendererNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenderPageToSurface(this, ::windows_core::from_raw_borrowed(&pdfpage), ::windows_core::from_raw_borrowed(&psurface), ::core::mem::transmute(&offset), ::core::mem::transmute_copy(&prenderparams)).into())
        }
        unsafe extern "system" fn RenderPageToDeviceContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPdfRendererNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: *mut ::core::ffi::c_void, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenderPageToDeviceContext(this, ::windows_core::from_raw_borrowed(&pdfpage), ::windows_core::from_raw_borrowed(&pd2ddevicecontext), ::core::mem::transmute_copy(&prenderparams)).into())
        }
        IPdfRendererNative_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RenderPageToSurface: RenderPageToSurface::<Identity, Impl, OFFSET>,
            RenderPageToDeviceContext: RenderPageToDeviceContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
