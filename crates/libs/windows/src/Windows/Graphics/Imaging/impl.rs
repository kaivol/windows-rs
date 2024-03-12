#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBitmapFrame_Impl: ::windows_core::BaseImpl {
    fn GetThumbnailAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>>;
    fn BitmapProperties(this: &Self::This) -> ::windows_core::Result<BitmapPropertiesView>;
    fn BitmapPixelFormat(this: &Self::This) -> ::windows_core::Result<BitmapPixelFormat>;
    fn BitmapAlphaMode(this: &Self::This) -> ::windows_core::Result<BitmapAlphaMode>;
    fn DpiX(this: &Self::This) -> ::windows_core::Result<f64>;
    fn DpiY(this: &Self::This) -> ::windows_core::Result<f64>;
    fn PixelWidth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn PixelHeight(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OrientedPixelWidth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OrientedPixelHeight(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPixelDataAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
    fn GetPixelDataTransformedAsync(this: &Self::This, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::core::option::Option<&BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IBitmapFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBitmapFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetThumbnailAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnailAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BitmapProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BitmapProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BitmapPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BitmapPixelFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BitmapAlphaMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BitmapAlphaMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DpiX<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DpiX(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DpiY<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DpiY(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PixelWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PixelWidth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PixelHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PixelHeight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OrientedPixelWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OrientedPixelWidth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OrientedPixelHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OrientedPixelHeight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelDataAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPixelDataAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelDataTransformedAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPixelDataTransformedAsync(this, pixelformat, alphamode, ::windows_core::from_raw_borrowed(&transform), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBitmapFrame_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetThumbnailAsync: GetThumbnailAsync::<Identity, Impl, OFFSET>,
            BitmapProperties: BitmapProperties::<Identity, Impl, OFFSET>,
            BitmapPixelFormat: BitmapPixelFormat::<Identity, Impl, OFFSET>,
            BitmapAlphaMode: BitmapAlphaMode::<Identity, Impl, OFFSET>,
            DpiX: DpiX::<Identity, Impl, OFFSET>,
            DpiY: DpiY::<Identity, Impl, OFFSET>,
            PixelWidth: PixelWidth::<Identity, Impl, OFFSET>,
            PixelHeight: PixelHeight::<Identity, Impl, OFFSET>,
            OrientedPixelWidth: OrientedPixelWidth::<Identity, Impl, OFFSET>,
            OrientedPixelHeight: OrientedPixelHeight::<Identity, Impl, OFFSET>,
            GetPixelDataAsync: GetPixelDataAsync::<Identity, Impl, OFFSET>,
            GetPixelDataTransformedAsync: GetPixelDataTransformedAsync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBitmapFrameWithSoftwareBitmap_Impl: ::windows_core::BaseImpl + IBitmapFrame_Impl {
    fn GetSoftwareBitmapAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapConvertedAsync(this: &Self::This, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
    fn GetSoftwareBitmapTransformedAsync(this: &Self::This, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::core::option::Option<&BitmapTransform>, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IBitmapFrameWithSoftwareBitmap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IBitmapFrame as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBitmapFrameWithSoftwareBitmap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSoftwareBitmapAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSoftwareBitmapAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSoftwareBitmapConvertedAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSoftwareBitmapConvertedAsync(this, pixelformat, alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSoftwareBitmapTransformedAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapFrameWithSoftwareBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSoftwareBitmapTransformedAsync(this, pixelformat, alphamode, ::windows_core::from_raw_borrowed(&transform), exiforientationmode, colormanagementmode) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBitmapFrameWithSoftwareBitmap_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSoftwareBitmapAsync: GetSoftwareBitmapAsync::<Identity, Impl, OFFSET>,
            GetSoftwareBitmapConvertedAsync: GetSoftwareBitmapConvertedAsync::<Identity, Impl, OFFSET>,
            GetSoftwareBitmapTransformedAsync: GetSoftwareBitmapTransformedAsync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IBitmapPropertiesView_Impl: ::windows_core::BaseImpl {
    fn GetPropertiesAsync(this: &Self::This, propertiestoretrieve: ::core::option::Option<&super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IBitmapPropertiesView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapPropertiesView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBitmapPropertiesView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertiesAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBitmapPropertiesView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertiesAsync(this, ::windows_core::from_raw_borrowed(&propertiestoretrieve)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBitmapPropertiesView_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertiesAsync: GetPropertiesAsync::<Identity, Impl, OFFSET>,
        }
    };
}
