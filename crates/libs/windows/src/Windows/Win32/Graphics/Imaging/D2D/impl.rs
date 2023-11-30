#[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICImageEncoder_Impl: ::windows_core::BaseImpl {
    fn WriteFrame(this: &Self::This, pimage: ::core::option::Option<&super::super::Direct2D::ID2D1Image>, pframeencode: ::core::option::Option<&super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> ::windows_core::Result<()>;
    fn WriteFrameThumbnail(this: &Self::This, pimage: ::core::option::Option<&super::super::Direct2D::ID2D1Image>, pframeencode: ::core::option::Option<&super::IWICBitmapFrameEncode>, pimageparameters: *const super::WICImageParameters) -> ::windows_core::Result<()>;
    fn WriteThumbnail(this: &Self::This, pimage: ::core::option::Option<&super::super::Direct2D::ID2D1Image>, pencoder: ::core::option::Option<&super::IWICBitmapEncoder>, pimageparameters: *const super::WICImageParameters) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IWICImageEncoder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICImageEncoder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteFrame(this, ::windows_core::from_raw_borrowed(&pimage), ::windows_core::from_raw_borrowed(&pframeencode), ::core::mem::transmute_copy(&pimageparameters)).into())
        }
        unsafe extern "system" fn WriteFrameThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteFrameThumbnail(this, ::windows_core::from_raw_borrowed(&pimage), ::windows_core::from_raw_borrowed(&pframeencode), ::core::mem::transmute_copy(&pimageparameters)).into())
        }
        unsafe extern "system" fn WriteThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImageEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pencoder: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteThumbnail(this, ::windows_core::from_raw_borrowed(&pimage), ::windows_core::from_raw_borrowed(&pencoder), ::core::mem::transmute_copy(&pimageparameters)).into())
        }
        IWICImageEncoder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteFrame: WriteFrame::<Identity, Impl, OFFSET>,
            WriteFrameThumbnail: WriteFrameThumbnail::<Identity, Impl, OFFSET>,
            WriteThumbnail: WriteThumbnail::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory2_Impl: ::windows_core::BaseImpl + super::IWICImagingFactory_Impl {
    fn CreateImageEncoder(this: &Self::This, pd2ddevice: ::core::option::Option<&super::super::Direct2D::ID2D1Device>) -> ::windows_core::Result<IWICImageEncoder>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IWICImagingFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IWICImagingFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICImagingFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateImageEncoder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pd2ddevice: *mut ::core::ffi::c_void, ppwicimageencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateImageEncoder(this, ::windows_core::from_raw_borrowed(&pd2ddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwicimageencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICImagingFactory2_Vtbl {
            base__: <super::IWICImagingFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateImageEncoder: CreateImageEncoder::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
