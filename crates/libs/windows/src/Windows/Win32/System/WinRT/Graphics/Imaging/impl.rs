pub trait ISoftwareBitmapNative_Impl: ::windows_core::BaseImpl {
    fn GetData(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISoftwareBitmapNative {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftwareBitmapNative_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISoftwareBitmapNative {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftwareBitmapNative_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        ISoftwareBitmapNative_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetData: GetData::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
pub trait ISoftwareBitmapNativeFactory_Impl: ::windows_core::BaseImpl {
    fn CreateFromWICBitmap(this: &Self::This, data: ::core::option::Option<&super::super::super::super::Graphics::Imaging::IWICBitmap>, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateFromMF2DBuffer2(this: &Self::This, data: ::core::option::Option<&super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for ISoftwareBitmapNativeFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftwareBitmapNativeFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISoftwareBitmapNativeFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromWICBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftwareBitmapNativeFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFromWICBitmap(this, ::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&forcereadonly), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateFromMF2DBuffer2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISoftwareBitmapNativeFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFromMF2DBuffer2(this, ::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&subtype), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&forcereadonly), ::core::mem::transmute_copy(&mindisplayaperture), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        ISoftwareBitmapNativeFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromWICBitmap: CreateFromWICBitmap::<Identity, Impl, OFFSET>,
            CreateFromMF2DBuffer2: CreateFromMF2DBuffer2::<Identity, Impl, OFFSET>,
        }
    };
}
