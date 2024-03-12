pub trait IWICBitmap_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn Lock(this: &Self::This, prclock: *const WICRect, flags: u32) -> ::windows_core::Result<IWICBitmapLock>;
    fn SetPalette(this: &Self::This, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn SetResolution(this: &Self::This, dpix: f64, dpiy: f64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICBitmap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Lock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Lock(this, ::core::mem::transmute_copy(&prclock), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppilock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&pipalette)).into())
        }
        unsafe extern "system" fn SetResolution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResolution(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into())
        }
        IWICBitmap_Vtbl {
            base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Lock: Lock::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            SetResolution: SetResolution::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICBitmapClipper_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn Initialize(this: &Self::This, pisource: ::core::option::Option<&IWICBitmapSource>, prc: *const WICRect) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICBitmapClipper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapClipper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapClipper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapClipper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, prc: *const WICRect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&prc)).into())
        }
        IWICBitmapClipper_Vtbl { base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapCodecInfo_Impl: ::windows_core::BaseImpl + IWICComponentInfo_Impl {
    fn GetContainerFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetPixelFormats(this: &Self::This, cformats: u32, pguidpixelformats: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetColorManagementVersion(this: &Self::This, cchcolormanagementversion: u32, wzcolormanagementversion: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceManufacturer(this: &Self::This, cchdevicemanufacturer: u32, wzdevicemanufacturer: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceModels(this: &Self::This, cchdevicemodels: u32, wzdevicemodels: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetMimeTypes(this: &Self::This, cchmimetypes: u32, wzmimetypes: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetFileExtensions(this: &Self::This, cchfileextensions: u32, wzfileextensions: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn DoesSupportAnimation(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportChromakey(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportLossless(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportMultiframe(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn MatchesMimeType(this: &Self::This, wzmimetype: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICBitmapCodecInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICComponentInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapCodecInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainerFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPixelFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelFormats(this, ::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&pguidpixelformats), ::core::mem::transmute_copy(&pcactual)).into())
        }
        unsafe extern "system" fn GetColorManagementVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorManagementVersion(this, ::core::mem::transmute_copy(&cchcolormanagementversion), ::core::mem::transmute(&wzcolormanagementversion), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceManufacturer(this, ::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetDeviceModels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceModels(this, ::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetMimeTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMimeTypes(this, ::core::mem::transmute_copy(&cchmimetypes), ::core::mem::transmute(&wzmimetypes), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetFileExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileExtensions(this, ::core::mem::transmute_copy(&cchfileextensions), ::core::mem::transmute(&wzfileextensions), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn DoesSupportAnimation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesSupportAnimation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportanimation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoesSupportChromakey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesSupportChromakey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportchromakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoesSupportLossless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesSupportLossless(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportlossless, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoesSupportMultiframe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesSupportMultiframe(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportmultiframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MatchesMimeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzmimetype: ::windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MatchesMimeType(this, ::core::mem::transmute(&wzmimetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapCodecInfo_Vtbl {
            base__: <IWICComponentInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetPixelFormats: GetPixelFormats::<Identity, Impl, OFFSET>,
            GetColorManagementVersion: GetColorManagementVersion::<Identity, Impl, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, Impl, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, Impl, OFFSET>,
            GetMimeTypes: GetMimeTypes::<Identity, Impl, OFFSET>,
            GetFileExtensions: GetFileExtensions::<Identity, Impl, OFFSET>,
            DoesSupportAnimation: DoesSupportAnimation::<Identity, Impl, OFFSET>,
            DoesSupportChromakey: DoesSupportChromakey::<Identity, Impl, OFFSET>,
            DoesSupportLossless: DoesSupportLossless::<Identity, Impl, OFFSET>,
            DoesSupportMultiframe: DoesSupportMultiframe::<Identity, Impl, OFFSET>,
            MatchesMimeType: MatchesMimeType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICBitmapCodecProgressNotification_Impl: ::windows_core::BaseImpl {
    fn RegisterProgressNotification(this: &Self::This, pfnprogressnotification: PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICBitmapCodecProgressNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecProgressNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapCodecProgressNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterProgressNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapCodecProgressNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfnprogressnotification: PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterProgressNotification(this, ::core::mem::transmute_copy(&pfnprogressnotification), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwprogressflags)).into())
        }
        IWICBitmapCodecProgressNotification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterProgressNotification: RegisterProgressNotification::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICBitmapDecoder_Impl: ::windows_core::BaseImpl {
    fn QueryCapability(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<u32>;
    fn Initialize(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>, cacheoptions: WICDecodeOptions) -> ::windows_core::Result<()>;
    fn GetContainerFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDecoderInfo(this: &Self::This) -> ::windows_core::Result<IWICBitmapDecoderInfo>;
    fn CopyPalette(this: &Self::This, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn GetMetadataQueryReader(this: &Self::This) -> ::windows_core::Result<IWICMetadataQueryReader>;
    fn GetPreview(this: &Self::This) -> ::windows_core::Result<IWICBitmapSource>;
    fn GetColorContexts(this: &Self::This, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetThumbnail(this: &Self::This) -> ::windows_core::Result<IWICBitmapSource>;
    fn GetFrameCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFrame(this: &Self::This, index: u32) -> ::windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWICBitmapDecoder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapDecoder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pdwcapability: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryCapability(this, ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapability, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, cacheoptions: WICDecodeOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&cacheoptions)).into())
        }
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainerFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDecoderInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDecoderInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoderinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyPalette(this, ::windows_core::from_raw_borrowed(&pipalette)).into())
        }
        unsafe extern "system" fn GetMetadataQueryReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataQueryReader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataqueryreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreview<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreview(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut ::core::ffi::c_void, pcactualcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorContexts(this, ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into())
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnail(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppithumbnail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFrameCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrame(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapDecoder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryCapability: QueryCapability::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetDecoderInfo: GetDecoderInfo::<Identity, Impl, OFFSET>,
            CopyPalette: CopyPalette::<Identity, Impl, OFFSET>,
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, Impl, OFFSET>,
            GetPreview: GetPreview::<Identity, Impl, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
            GetFrameCount: GetFrameCount::<Identity, Impl, OFFSET>,
            GetFrame: GetFrame::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICBitmapDecoderInfo_Impl: ::windows_core::BaseImpl + IWICBitmapCodecInfo_Impl {
    fn GetPatterns(this: &Self::This, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows_core::Result<()>;
    fn MatchesPattern(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(this: &Self::This) -> ::windows_core::Result<IWICBitmapDecoder>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWICBitmapDecoderInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapCodecInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapDecoderInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPatterns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatterns(this, ::core::mem::transmute_copy(&cbsizepatterns), ::core::mem::transmute_copy(&ppatterns), ::core::mem::transmute_copy(&pcpatterns), ::core::mem::transmute_copy(&pcbpatternsactual)).into())
        }
        unsafe extern "system" fn MatchesPattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MatchesPattern(this, ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapDecoderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapdecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapDecoderInfo_Vtbl {
            base__: <IWICBitmapCodecInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPatterns: GetPatterns::<Identity, Impl, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapEncoder_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>, cacheoption: WICBitmapEncoderCacheOption) -> ::windows_core::Result<()>;
    fn GetContainerFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetEncoderInfo(this: &Self::This) -> ::windows_core::Result<IWICBitmapEncoderInfo>;
    fn SetColorContexts(this: &Self::This, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows_core::Result<()>;
    fn SetPalette(this: &Self::This, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn SetThumbnail(this: &Self::This, pithumbnail: ::core::option::Option<&IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn SetPreview(this: &Self::This, pipreview: ::core::option::Option<&IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn CreateNewFrame(this: &Self::This, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMetadataQueryWriter(this: &Self::This) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for IWICBitmapEncoder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapEncoder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, cacheoption: WICBitmapEncoderCacheOption) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&cacheoption)).into())
        }
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainerFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEncoderInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEncoderInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiencoderinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColorContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorContexts(this, ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into())
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&pipalette)).into())
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pithumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThumbnail(this, ::windows_core::from_raw_borrowed(&pithumbnail)).into())
        }
        unsafe extern "system" fn SetPreview<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipreview: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreview(this, ::windows_core::from_raw_borrowed(&pipreview)).into())
        }
        unsafe extern "system" fn CreateNewFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut *mut ::core::ffi::c_void, ppiencoderoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNewFrame(this, ::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&ppiencoderoptions)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataQueryWriter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapEncoder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetEncoderInfo: GetEncoderInfo::<Identity, Impl, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, Impl, OFFSET>,
            SetPreview: SetPreview::<Identity, Impl, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapEncoderInfo_Impl: ::windows_core::BaseImpl + IWICBitmapCodecInfo_Impl {
    fn CreateInstance(this: &Self::This) -> ::windows_core::Result<IWICBitmapEncoder>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICBitmapEncoderInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapCodecInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoderInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapEncoderInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapEncoderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapEncoderInfo_Vtbl { base__: <IWICBitmapCodecInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    };
}
pub trait IWICBitmapFlipRotator_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn Initialize(this: &Self::This, pisource: ::core::option::Option<&IWICBitmapSource>, options: WICBitmapTransformOptions) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICBitmapFlipRotator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFlipRotator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapFlipRotator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFlipRotator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, options: WICBitmapTransformOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&options)).into())
        }
        IWICBitmapFlipRotator_Vtbl { base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
pub trait IWICBitmapFrameDecode_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn GetMetadataQueryReader(this: &Self::This) -> ::windows_core::Result<IWICMetadataQueryReader>;
    fn GetColorContexts(this: &Self::This, ccount: u32, ppicolorcontexts: *mut ::core::option::Option<IWICColorContext>, pcactualcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetThumbnail(this: &Self::This) -> ::windows_core::Result<IWICBitmapSource>;
}
impl ::windows_core::Iids for IWICBitmapFrameDecode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapFrameDecode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetadataQueryReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataQueryReader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataqueryreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut ::core::ffi::c_void, pcactualcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorContexts(this, ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontexts), ::core::mem::transmute_copy(&pcactualcount)).into())
        }
        unsafe extern "system" fn GetThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppithumbnail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnail(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppithumbnail, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapFrameDecode_Vtbl {
            base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetadataQueryReader: GetMetadataQueryReader::<Identity, Impl, OFFSET>,
            GetColorContexts: GetColorContexts::<Identity, Impl, OFFSET>,
            GetThumbnail: GetThumbnail::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICBitmapFrameEncode_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, piencoderoptions: ::core::option::Option<&super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows_core::Result<()>;
    fn SetSize(this: &Self::This, uiwidth: u32, uiheight: u32) -> ::windows_core::Result<()>;
    fn SetResolution(this: &Self::This, dpix: f64, dpiy: f64) -> ::windows_core::Result<()>;
    fn SetPixelFormat(this: &Self::This, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetColorContexts(this: &Self::This, ccount: u32, ppicolorcontext: *const ::core::option::Option<IWICColorContext>) -> ::windows_core::Result<()>;
    fn SetPalette(this: &Self::This, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn SetThumbnail(this: &Self::This, pithumbnail: ::core::option::Option<&IWICBitmapSource>) -> ::windows_core::Result<()>;
    fn WritePixels(this: &Self::This, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows_core::Result<()>;
    fn WriteSource(this: &Self::This, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, prc: *const WICRect) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMetadataQueryWriter(this: &Self::This) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for IWICBitmapFrameEncode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapFrameEncode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piencoderoptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&piencoderoptions)).into())
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight)).into())
        }
        unsafe extern "system" fn SetResolution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResolution(this, ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy)).into())
        }
        unsafe extern "system" fn SetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelFormat(this, ::core::mem::transmute_copy(&ppixelformat)).into())
        }
        unsafe extern "system" fn SetColorContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorContexts(this, ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&ppicolorcontext)).into())
        }
        unsafe extern "system" fn SetPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPalette(this, ::windows_core::from_raw_borrowed(&pipalette)).into())
        }
        unsafe extern "system" fn SetThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pithumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThumbnail(this, ::windows_core::from_raw_borrowed(&pithumbnail)).into())
        }
        unsafe extern "system" fn WritePixels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WritePixels(this, ::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbpixels)).into())
        }
        unsafe extern "system" fn WriteSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, prc: *const WICRect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteSource(this, ::windows_core::from_raw_borrowed(&pibitmapsource), ::core::mem::transmute_copy(&prc)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataQueryWriter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapFrameEncode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            SetResolution: SetResolution::<Identity, Impl, OFFSET>,
            SetPixelFormat: SetPixelFormat::<Identity, Impl, OFFSET>,
            SetColorContexts: SetColorContexts::<Identity, Impl, OFFSET>,
            SetPalette: SetPalette::<Identity, Impl, OFFSET>,
            SetThumbnail: SetThumbnail::<Identity, Impl, OFFSET>,
            WritePixels: WritePixels::<Identity, Impl, OFFSET>,
            WriteSource: WriteSource::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICBitmapLock_Impl: ::windows_core::BaseImpl {
    fn GetSize(this: &Self::This, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::Result<()>;
    fn GetStride(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDataPointer(this: &Self::This, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn GetPixelFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IWICBitmapLock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapLock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSize(this, ::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into())
        }
        unsafe extern "system" fn GetStride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStride(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbstride, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataPointer(this, ::core::mem::transmute_copy(&pcbbuffersize), ::core::mem::transmute_copy(&ppbdata)).into())
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPixelFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixelformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapLock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetStride: GetStride::<Identity, Impl, OFFSET>,
            GetDataPointer: GetDataPointer::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICBitmapScaler_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn Initialize(this: &Self::This, pisource: ::core::option::Option<&IWICBitmapSource>, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICBitmapScaler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapScaler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapScaler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapScaler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&mode)).into())
        }
        IWICBitmapScaler_Vtbl { base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
pub trait IWICBitmapSource_Impl: ::windows_core::BaseImpl {
    fn GetSize(this: &Self::This, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::Result<()>;
    fn GetPixelFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetResolution(this: &Self::This, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows_core::Result<()>;
    fn CopyPalette(this: &Self::This, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn CopyPixels(this: &Self::This, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICBitmapSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSize(this, ::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into())
        }
        unsafe extern "system" fn GetPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPixelFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppixelformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResolution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResolution(this, ::core::mem::transmute_copy(&pdpix), ::core::mem::transmute_copy(&pdpiy)).into())
        }
        unsafe extern "system" fn CopyPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyPalette(this, ::windows_core::from_raw_borrowed(&pipalette)).into())
        }
        unsafe extern "system" fn CopyPixels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyPixels(this, ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into())
        }
        IWICBitmapSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, Impl, OFFSET>,
            GetResolution: GetResolution::<Identity, Impl, OFFSET>,
            CopyPalette: CopyPalette::<Identity, Impl, OFFSET>,
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICBitmapSourceTransform_Impl: ::windows_core::BaseImpl {
    fn CopyPixels(this: &Self::This, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows_core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::Result<()>;
    fn GetClosestSize(this: &Self::This, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::Result<()>;
    fn GetClosestPixelFormat(this: &Self::This, pguiddstformat: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn DoesSupportTransform(this: &Self::This, dsttransform: WICBitmapTransformOptions) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICBitmapSourceTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICBitmapSourceTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CopyPixels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows_core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyPixels(this, ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pguiddstformat), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&nstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into())
        }
        unsafe extern "system" fn GetClosestSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClosestSize(this, ::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight)).into())
        }
        unsafe extern "system" fn GetClosestPixelFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClosestPixelFormat(this, ::core::mem::transmute_copy(&pguiddstformat)).into())
        }
        unsafe extern "system" fn DoesSupportTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICBitmapSourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesSupportTransform(this, ::core::mem::transmute_copy(&dsttransform)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfissupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICBitmapSourceTransform_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
            GetClosestSize: GetClosestSize::<Identity, Impl, OFFSET>,
            GetClosestPixelFormat: GetClosestPixelFormat::<Identity, Impl, OFFSET>,
            DoesSupportTransform: DoesSupportTransform::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICColorContext_Impl: ::windows_core::BaseImpl {
    fn InitializeFromFilename(this: &Self::This, wzfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn InitializeFromMemory(this: &Self::This, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::Result<()>;
    fn InitializeFromExifColorSpace(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<WICColorContextType>;
    fn GetProfileBytes(this: &Self::This, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetExifColorSpace(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IWICColorContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICColorContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeFromFilename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromFilename(this, ::core::mem::transmute(&wzfilename)).into())
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromMemory(this, ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into())
        }
        unsafe extern "system" fn InitializeFromExifColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromExifColorSpace(this, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProfileBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProfileBytes(this, ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&pcbactual)).into())
        }
        unsafe extern "system" fn GetExifColorSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExifColorSpace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICColorContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeFromFilename: InitializeFromFilename::<Identity, Impl, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, Impl, OFFSET>,
            InitializeFromExifColorSpace: InitializeFromExifColorSpace::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetProfileBytes: GetProfileBytes::<Identity, Impl, OFFSET>,
            GetExifColorSpace: GetExifColorSpace::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICColorTransform_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn Initialize(this: &Self::This, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, picontextsource: ::core::option::Option<&IWICColorContext>, picontextdest: ::core::option::Option<&IWICColorContext>, pixelfmtdest: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICColorTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICColorTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICColorTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, picontextsource: *mut ::core::ffi::c_void, picontextdest: *mut ::core::ffi::c_void, pixelfmtdest: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pibitmapsource), ::windows_core::from_raw_borrowed(&picontextsource), ::windows_core::from_raw_borrowed(&picontextdest), ::core::mem::transmute_copy(&pixelfmtdest)).into())
        }
        IWICColorTransform_Vtbl { base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICComponentFactory_Impl: ::windows_core::BaseImpl + IWICImagingFactory_Impl {
    fn CreateMetadataReader(this: &Self::This, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataReaderFromContainer(this: &Self::This, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<IWICMetadataReader>;
    fn CreateMetadataWriter(this: &Self::This, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwmetadataoptions: u32) -> ::windows_core::Result<IWICMetadataWriter>;
    fn CreateMetadataWriterFromReader(this: &Self::This, pireader: ::core::option::Option<&IWICMetadataReader>, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICMetadataWriter>;
    fn CreateQueryReaderFromBlockReader(this: &Self::This, piblockreader: ::core::option::Option<&IWICMetadataBlockReader>) -> ::windows_core::Result<IWICMetadataQueryReader>;
    fn CreateQueryWriterFromBlockWriter(this: &Self::This, piblockwriter: ::core::option::Option<&IWICMetadataBlockWriter>) -> ::windows_core::Result<IWICMetadataQueryWriter>;
    fn CreateEncoderPropertyBag(this: &Self::This, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IWICComponentFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICImagingFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICComponentFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateMetadataReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMetadataReader(this, ::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppireader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMetadataReaderFromContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwoptions: u32, pistream: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMetadataReaderFromContainer(this, ::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwoptions), ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppireader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMetadataWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, dwmetadataoptions: u32, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMetadataWriter(this, ::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwmetadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMetadataWriterFromReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pireader: *mut ::core::ffi::c_void, pguidvendor: *const ::windows_core::GUID, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMetadataWriterFromReader(this, ::windows_core::from_raw_borrowed(&pireader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateQueryReaderFromBlockReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piblockreader: *mut ::core::ffi::c_void, ppiqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQueryReaderFromBlockReader(this, ::windows_core::from_raw_borrowed(&piblockreader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiqueryreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateQueryWriterFromBlockWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piblockwriter: *mut ::core::ffi::c_void, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQueryWriterFromBlockWriter(this, ::windows_core::from_raw_borrowed(&piblockwriter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEncoderPropertyBag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncoderPropertyBag(this, ::core::mem::transmute_copy(&ppropoptions), ::core::mem::transmute_copy(&ccount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipropertybag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICComponentFactory_Vtbl {
            base__: <IWICImagingFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateMetadataReader: CreateMetadataReader::<Identity, Impl, OFFSET>,
            CreateMetadataReaderFromContainer: CreateMetadataReaderFromContainer::<Identity, Impl, OFFSET>,
            CreateMetadataWriter: CreateMetadataWriter::<Identity, Impl, OFFSET>,
            CreateMetadataWriterFromReader: CreateMetadataWriterFromReader::<Identity, Impl, OFFSET>,
            CreateQueryReaderFromBlockReader: CreateQueryReaderFromBlockReader::<Identity, Impl, OFFSET>,
            CreateQueryWriterFromBlockWriter: CreateQueryWriterFromBlockWriter::<Identity, Impl, OFFSET>,
            CreateEncoderPropertyBag: CreateEncoderPropertyBag::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICComponentInfo_Impl: ::windows_core::BaseImpl {
    fn GetComponentType(this: &Self::This) -> ::windows_core::Result<WICComponentType>;
    fn GetCLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetSigningStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAuthor(this: &Self::This, cchauthor: u32, wzauthor: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetVendorGUID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetVersion(this: &Self::This, cchversion: u32, wzversion: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetSpecVersion(this: &Self::This, cchspecversion: u32, wzspecversion: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetFriendlyName(this: &Self::This, cchfriendlyname: u32, wzfriendlyname: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICComponentInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICComponentInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetComponentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetComponentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSigningStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSigningStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAuthor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAuthor(this, ::core::mem::transmute_copy(&cchauthor), ::core::mem::transmute(&wzauthor), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetVendorGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVendorGUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidvendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersion(this, ::core::mem::transmute_copy(&cchversion), ::core::mem::transmute(&wzversion), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetSpecVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpecVersion(this, ::core::mem::transmute_copy(&cchspecversion), ::core::mem::transmute(&wzspecversion), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICComponentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFriendlyName(this, ::core::mem::transmute_copy(&cchfriendlyname), ::core::mem::transmute(&wzfriendlyname), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        IWICComponentInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetComponentType: GetComponentType::<Identity, Impl, OFFSET>,
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            GetSigningStatus: GetSigningStatus::<Identity, Impl, OFFSET>,
            GetAuthor: GetAuthor::<Identity, Impl, OFFSET>,
            GetVendorGUID: GetVendorGUID::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetSpecVersion: GetSpecVersion::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsDecoder_Impl: ::windows_core::BaseImpl {
    fn GetParameters(this: &Self::This, pparameters: *mut WICDdsParameters) -> ::windows_core::Result<()>;
    fn GetFrame(this: &Self::This, arrayindex: u32, miplevel: u32, sliceindex: u32) -> ::windows_core::Result<IWICBitmapFrameDecode>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IWICDdsDecoder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsDecoder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICDdsDecoder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParameters(this, ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn GetFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsDecoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrame(this, ::core::mem::transmute_copy(&arrayindex), ::core::mem::transmute_copy(&miplevel), ::core::mem::transmute_copy(&sliceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICDdsDecoder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            GetFrame: GetFrame::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsEncoder_Impl: ::windows_core::BaseImpl {
    fn SetParameters(this: &Self::This, pparameters: *const WICDdsParameters) -> ::windows_core::Result<()>;
    fn GetParameters(this: &Self::This, pparameters: *mut WICDdsParameters) -> ::windows_core::Result<()>;
    fn CreateNewFrame(this: &Self::This, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IWICDdsEncoder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICDdsEncoder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameters(this, ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParameters(this, ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn CreateNewFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiframeencode: *mut *mut ::core::ffi::c_void, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNewFrame(this, ::core::mem::transmute_copy(&ppiframeencode), ::core::mem::transmute_copy(&parrayindex), ::core::mem::transmute_copy(&pmiplevel), ::core::mem::transmute_copy(&psliceindex)).into())
        }
        IWICDdsEncoder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            CreateNewFrame: CreateNewFrame::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICDdsFrameDecode_Impl: ::windows_core::BaseImpl {
    fn GetSizeInBlocks(this: &Self::This, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows_core::Result<()>;
    fn GetFormatInfo(this: &Self::This) -> ::windows_core::Result<WICDdsFormatInfo>;
    fn CopyBlocks(this: &Self::This, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IWICDdsFrameDecode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICDdsFrameDecode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSizeInBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSizeInBlocks(this, ::core::mem::transmute_copy(&pwidthinblocks), ::core::mem::transmute_copy(&pheightinblocks)).into())
        }
        unsafe extern "system" fn GetFormatInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDdsFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyBlocks(this, ::core::mem::transmute_copy(&prcboundsinblocks), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)).into())
        }
        IWICDdsFrameDecode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSizeInBlocks: GetSizeInBlocks::<Identity, Impl, OFFSET>,
            GetFormatInfo: GetFormatInfo::<Identity, Impl, OFFSET>,
            CopyBlocks: CopyBlocks::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWICDevelopRaw_Impl: ::windows_core::BaseImpl + IWICBitmapFrameDecode_Impl {
    fn QueryRawCapabilitiesInfo(this: &Self::This, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows_core::Result<()>;
    fn LoadParameterSet(this: &Self::This, parameterset: WICRawParameterSet) -> ::windows_core::Result<()>;
    fn GetCurrentParameterSet(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2>;
    fn SetExposureCompensation(this: &Self::This, ev: f64) -> ::windows_core::Result<()>;
    fn GetExposureCompensation(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetWhitePointRGB(this: &Self::This, red: u32, green: u32, blue: u32) -> ::windows_core::Result<()>;
    fn GetWhitePointRGB(this: &Self::This, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows_core::Result<()>;
    fn SetNamedWhitePoint(this: &Self::This, whitepoint: WICNamedWhitePoint) -> ::windows_core::Result<()>;
    fn GetNamedWhitePoint(this: &Self::This) -> ::windows_core::Result<WICNamedWhitePoint>;
    fn SetWhitePointKelvin(this: &Self::This, whitepointkelvin: u32) -> ::windows_core::Result<()>;
    fn GetWhitePointKelvin(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetKelvinRangeInfo(this: &Self::This, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows_core::Result<()>;
    fn SetContrast(this: &Self::This, contrast: f64) -> ::windows_core::Result<()>;
    fn GetContrast(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetGamma(this: &Self::This, gamma: f64) -> ::windows_core::Result<()>;
    fn GetGamma(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetSharpness(this: &Self::This, sharpness: f64) -> ::windows_core::Result<()>;
    fn GetSharpness(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetSaturation(this: &Self::This, saturation: f64) -> ::windows_core::Result<()>;
    fn GetSaturation(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetTint(this: &Self::This, tint: f64) -> ::windows_core::Result<()>;
    fn GetTint(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetNoiseReduction(this: &Self::This, noisereduction: f64) -> ::windows_core::Result<()>;
    fn GetNoiseReduction(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetDestinationColorContext(this: &Self::This, pcolorcontext: ::core::option::Option<&IWICColorContext>) -> ::windows_core::Result<()>;
    fn SetToneCurve(this: &Self::This, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows_core::Result<()>;
    fn GetToneCurve(this: &Self::This, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn SetRotation(this: &Self::This, rotation: f64) -> ::windows_core::Result<()>;
    fn GetRotation(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetRenderMode(this: &Self::This, rendermode: WICRawRenderMode) -> ::windows_core::Result<()>;
    fn GetRenderMode(this: &Self::This) -> ::windows_core::Result<WICRawRenderMode>;
    fn SetNotificationCallback(this: &Self::This, pcallback: ::core::option::Option<&IWICDevelopRawNotificationCallback>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::Iids for IWICDevelopRaw {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapFrameDecode);
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICDevelopRaw {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryRawCapabilitiesInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryRawCapabilitiesInfo(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn LoadParameterSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadParameterSet(this, ::core::mem::transmute_copy(&parameterset)).into())
        }
        unsafe extern "system" fn GetCurrentParameterSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentParameterSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcurrentparameterset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExposureCompensation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExposureCompensation(this, ::core::mem::transmute_copy(&ev)).into())
        }
        unsafe extern "system" fn GetExposureCompensation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExposureCompensation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pev, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWhitePointRGB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWhitePointRGB(this, ::core::mem::transmute_copy(&red), ::core::mem::transmute_copy(&green), ::core::mem::transmute_copy(&blue)).into())
        }
        unsafe extern "system" fn GetWhitePointRGB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWhitePointRGB(this, ::core::mem::transmute_copy(&pred), ::core::mem::transmute_copy(&pgreen), ::core::mem::transmute_copy(&pblue)).into())
        }
        unsafe extern "system" fn SetNamedWhitePoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamedWhitePoint(this, ::core::mem::transmute_copy(&whitepoint)).into())
        }
        unsafe extern "system" fn GetNamedWhitePoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamedWhitePoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwhitepoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWhitePointKelvin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWhitePointKelvin(this, ::core::mem::transmute_copy(&whitepointkelvin)).into())
        }
        unsafe extern "system" fn GetWhitePointKelvin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWhitePointKelvin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwhitepointkelvin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKelvinRangeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKelvinRangeInfo(this, ::core::mem::transmute_copy(&pminkelvintemp), ::core::mem::transmute_copy(&pmaxkelvintemp), ::core::mem::transmute_copy(&pkelvintempstepvalue)).into())
        }
        unsafe extern "system" fn SetContrast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContrast(this, ::core::mem::transmute_copy(&contrast)).into())
        }
        unsafe extern "system" fn GetContrast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContrast(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontrast, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGamma<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGamma(this, ::core::mem::transmute_copy(&gamma)).into())
        }
        unsafe extern "system" fn GetGamma<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGamma(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgamma, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSharpness<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSharpness(this, ::core::mem::transmute_copy(&sharpness)).into())
        }
        unsafe extern "system" fn GetSharpness<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSharpness(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psharpness, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSaturation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSaturation(this, ::core::mem::transmute_copy(&saturation)).into())
        }
        unsafe extern "system" fn GetSaturation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSaturation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psaturation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTint(this, ::core::mem::transmute_copy(&tint)).into())
        }
        unsafe extern "system" fn GetTint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNoiseReduction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNoiseReduction(this, ::core::mem::transmute_copy(&noisereduction)).into())
        }
        unsafe extern "system" fn GetNoiseReduction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNoiseReduction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnoisereduction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDestinationColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolorcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestinationColorContext(this, ::windows_core::from_raw_borrowed(&pcolorcontext)).into())
        }
        unsafe extern "system" fn SetToneCurve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetToneCurve(this, ::core::mem::transmute_copy(&cbtonecurvesize), ::core::mem::transmute_copy(&ptonecurve)).into())
        }
        unsafe extern "system" fn GetToneCurve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetToneCurve(this, ::core::mem::transmute_copy(&cbtonecurvebuffersize), ::core::mem::transmute_copy(&ptonecurve), ::core::mem::transmute_copy(&pcbactualtonecurvebuffersize)).into())
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRotation(this, ::core::mem::transmute_copy(&rotation)).into())
        }
        unsafe extern "system" fn GetRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRotation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(protation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRenderMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRenderMode(this, ::core::mem::transmute_copy(&rendermode)).into())
        }
        unsafe extern "system" fn GetRenderMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRenderMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prendermode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotificationCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRaw_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotificationCallback(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        IWICDevelopRaw_Vtbl {
            base__: <IWICBitmapFrameDecode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryRawCapabilitiesInfo: QueryRawCapabilitiesInfo::<Identity, Impl, OFFSET>,
            LoadParameterSet: LoadParameterSet::<Identity, Impl, OFFSET>,
            GetCurrentParameterSet: GetCurrentParameterSet::<Identity, Impl, OFFSET>,
            SetExposureCompensation: SetExposureCompensation::<Identity, Impl, OFFSET>,
            GetExposureCompensation: GetExposureCompensation::<Identity, Impl, OFFSET>,
            SetWhitePointRGB: SetWhitePointRGB::<Identity, Impl, OFFSET>,
            GetWhitePointRGB: GetWhitePointRGB::<Identity, Impl, OFFSET>,
            SetNamedWhitePoint: SetNamedWhitePoint::<Identity, Impl, OFFSET>,
            GetNamedWhitePoint: GetNamedWhitePoint::<Identity, Impl, OFFSET>,
            SetWhitePointKelvin: SetWhitePointKelvin::<Identity, Impl, OFFSET>,
            GetWhitePointKelvin: GetWhitePointKelvin::<Identity, Impl, OFFSET>,
            GetKelvinRangeInfo: GetKelvinRangeInfo::<Identity, Impl, OFFSET>,
            SetContrast: SetContrast::<Identity, Impl, OFFSET>,
            GetContrast: GetContrast::<Identity, Impl, OFFSET>,
            SetGamma: SetGamma::<Identity, Impl, OFFSET>,
            GetGamma: GetGamma::<Identity, Impl, OFFSET>,
            SetSharpness: SetSharpness::<Identity, Impl, OFFSET>,
            GetSharpness: GetSharpness::<Identity, Impl, OFFSET>,
            SetSaturation: SetSaturation::<Identity, Impl, OFFSET>,
            GetSaturation: GetSaturation::<Identity, Impl, OFFSET>,
            SetTint: SetTint::<Identity, Impl, OFFSET>,
            GetTint: GetTint::<Identity, Impl, OFFSET>,
            SetNoiseReduction: SetNoiseReduction::<Identity, Impl, OFFSET>,
            GetNoiseReduction: GetNoiseReduction::<Identity, Impl, OFFSET>,
            SetDestinationColorContext: SetDestinationColorContext::<Identity, Impl, OFFSET>,
            SetToneCurve: SetToneCurve::<Identity, Impl, OFFSET>,
            GetToneCurve: GetToneCurve::<Identity, Impl, OFFSET>,
            SetRotation: SetRotation::<Identity, Impl, OFFSET>,
            GetRotation: GetRotation::<Identity, Impl, OFFSET>,
            SetRenderMode: SetRenderMode::<Identity, Impl, OFFSET>,
            GetRenderMode: GetRenderMode::<Identity, Impl, OFFSET>,
            SetNotificationCallback: SetNotificationCallback::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICDevelopRawNotificationCallback_Impl: ::windows_core::BaseImpl {
    fn Notify(this: &Self::This, notificationmask: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICDevelopRawNotificationCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRawNotificationCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICDevelopRawNotificationCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICDevelopRawNotificationCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::core::mem::transmute_copy(&notificationmask)).into())
        }
        IWICDevelopRawNotificationCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Notify: Notify::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICEnumMetadataItem_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWICEnumMetadataItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICEnumMetadataItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgeltschema), ::core::mem::transmute_copy(&rgeltid), ::core::mem::transmute_copy(&rgeltvalue), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICEnumMetadataItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienummetadataitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICEnumMetadataItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICFastMetadataEncoder_Impl: ::windows_core::BaseImpl {
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMetadataQueryWriter(this: &Self::This) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
impl ::windows_core::Iids for IWICFastMetadataEncoder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICFastMetadataEncoder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        unsafe extern "system" fn GetMetadataQueryWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFastMetadataEncoder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataQueryWriter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadataquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICFastMetadataEncoder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Commit: Commit::<Identity, Impl, OFFSET>,
            GetMetadataQueryWriter: GetMetadataQueryWriter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICFormatConverter_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn Initialize(this: &Self::This, pisource: ::core::option::Option<&IWICBitmapSource>, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: ::core::option::Option<&IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::Result<()>;
    fn CanConvert(this: &Self::This, srcpixelformat: *const ::windows_core::GUID, dstpixelformat: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICFormatConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFormatConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICFormatConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: *mut ::core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pisource), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::windows_core::from_raw_borrowed(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into())
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, srcpixelformat: *const ::windows_core::GUID, dstpixelformat: *const ::windows_core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanConvert(this, ::core::mem::transmute_copy(&srcpixelformat), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanconvert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICFormatConverter_Vtbl {
            base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICFormatConverterInfo_Impl: ::windows_core::BaseImpl + IWICComponentInfo_Impl {
    fn GetPixelFormats(this: &Self::This, cformats: u32, ppixelformatguids: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::Result<()>;
    fn CreateInstance(this: &Self::This) -> ::windows_core::Result<IWICFormatConverter>;
}
impl ::windows_core::Iids for IWICFormatConverterInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICComponentInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFormatConverterInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICFormatConverterInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPixelFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFormatConverterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows_core::GUID, pcactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelFormats(this, ::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&ppixelformatguids), ::core::mem::transmute_copy(&pcactual)).into())
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICFormatConverterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiconverter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiconverter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICFormatConverterInfo_Vtbl {
            base__: <IWICComponentInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPixelFormats: GetPixelFormats::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWICImagingFactory_Impl: ::windows_core::BaseImpl {
    fn CreateDecoderFromFilename(this: &Self::This, wzfilename: &::windows_core::PCWSTR, pguidvendor: *const ::windows_core::GUID, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromStream(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateDecoderFromFileHandle(this: &Self::This, hfile: usize, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateComponentInfo(this: &Self::This, clsidcomponent: *const ::windows_core::GUID) -> ::windows_core::Result<IWICComponentInfo>;
    fn CreateDecoder(this: &Self::This, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICBitmapDecoder>;
    fn CreateEncoder(this: &Self::This, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICBitmapEncoder>;
    fn CreatePalette(this: &Self::This) -> ::windows_core::Result<IWICPalette>;
    fn CreateFormatConverter(this: &Self::This) -> ::windows_core::Result<IWICFormatConverter>;
    fn CreateBitmapScaler(this: &Self::This) -> ::windows_core::Result<IWICBitmapScaler>;
    fn CreateBitmapClipper(this: &Self::This) -> ::windows_core::Result<IWICBitmapClipper>;
    fn CreateBitmapFlipRotator(this: &Self::This) -> ::windows_core::Result<IWICBitmapFlipRotator>;
    fn CreateStream(this: &Self::This) -> ::windows_core::Result<IWICStream>;
    fn CreateColorContext(this: &Self::This) -> ::windows_core::Result<IWICColorContext>;
    fn CreateColorTransformer(this: &Self::This) -> ::windows_core::Result<IWICColorTransform>;
    fn CreateBitmap(this: &Self::This, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, option: WICBitmapCreateCacheOption) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSource(this: &Self::This, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, option: WICBitmapCreateCacheOption) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromSourceRect(this: &Self::This, pibitmapsource: ::core::option::Option<&IWICBitmapSource>, x: u32, y: u32, width: u32, height: u32) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromMemory(this: &Self::This, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHBITMAP(this: &Self::This, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption) -> ::windows_core::Result<IWICBitmap>;
    fn CreateBitmapFromHICON(this: &Self::This, hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::Result<IWICBitmap>;
    fn CreateComponentEnumerator(this: &Self::This, componenttypes: u32, options: u32) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn CreateFastMetadataEncoderFromDecoder(this: &Self::This, pidecoder: ::core::option::Option<&IWICBitmapDecoder>) -> ::windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateFastMetadataEncoderFromFrameDecode(this: &Self::This, piframedecoder: ::core::option::Option<&IWICBitmapFrameDecode>) -> ::windows_core::Result<IWICFastMetadataEncoder>;
    fn CreateQueryWriter(this: &Self::This, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICMetadataQueryWriter>;
    fn CreateQueryWriterFromReader(this: &Self::This, piqueryreader: ::core::option::Option<&IWICMetadataQueryReader>, pguidvendor: *const ::windows_core::GUID) -> ::windows_core::Result<IWICMetadataQueryWriter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IWICImagingFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICImagingFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDecoderFromFilename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows_core::PCWSTR, pguidvendor: *const ::windows_core::GUID, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDecoderFromFilename(this, ::core::mem::transmute(&wzfilename), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDecoderFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDecoderFromStream(this, ::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDecoderFromFileHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows_core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDecoderFromFileHandle(this, ::core::mem::transmute_copy(&hfile), ::core::mem::transmute_copy(&pguidvendor), ::core::mem::transmute_copy(&metadataoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateComponentInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows_core::GUID, ppiinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateComponentInfo(this, ::core::mem::transmute_copy(&clsidcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDecoder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDecoder(this, ::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidecoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEncoder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, ppiencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncoder(this, ::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppipalette: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePalette(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipalette, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFormatConverter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFormatConverter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiformatconverter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapScaler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapScaler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapscaler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapClipper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapClipper(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapclipper, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFlipRotator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFlipRotator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmapfliprotator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwicstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwicstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiccolorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorTransformer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorTransformer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwiccolortransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmap(this, ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromSource(this, ::windows_core::from_raw_borrowed(&pibitmapsource), ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromSourceRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromSourceRect(this, ::windows_core::from_raw_borrowed(&pibitmapsource), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows_core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromMemory(this, ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&pixelformat), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbbuffersize), ::core::mem::transmute_copy(&pbbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromHBITMAP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromHBITMAP(this, ::core::mem::transmute_copy(&hbitmap), ::core::mem::transmute_copy(&hpalette), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapFromHICON<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapFromHICON(this, ::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppibitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateComponentEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateComponentEnumerator(this, ::core::mem::transmute_copy(&componenttypes), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromDecoder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidecoder: *mut ::core::ffi::c_void, ppifastencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFastMetadataEncoderFromDecoder(this, ::windows_core::from_raw_borrowed(&pidecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifastencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFastMetadataEncoderFromFrameDecode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piframedecoder: *mut ::core::ffi::c_void, ppifastencoder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFastMetadataEncoderFromFrameDecode(this, ::windows_core::from_raw_borrowed(&piframedecoder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppifastencoder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateQueryWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows_core::GUID, pguidvendor: *const ::windows_core::GUID, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQueryWriter(this, ::core::mem::transmute_copy(&guidmetadataformat), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateQueryWriterFromReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICImagingFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piqueryreader: *mut ::core::ffi::c_void, pguidvendor: *const ::windows_core::GUID, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQueryWriterFromReader(this, ::windows_core::from_raw_borrowed(&piqueryreader), ::core::mem::transmute_copy(&pguidvendor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiquerywriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICImagingFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDecoderFromFilename: CreateDecoderFromFilename::<Identity, Impl, OFFSET>,
            CreateDecoderFromStream: CreateDecoderFromStream::<Identity, Impl, OFFSET>,
            CreateDecoderFromFileHandle: CreateDecoderFromFileHandle::<Identity, Impl, OFFSET>,
            CreateComponentInfo: CreateComponentInfo::<Identity, Impl, OFFSET>,
            CreateDecoder: CreateDecoder::<Identity, Impl, OFFSET>,
            CreateEncoder: CreateEncoder::<Identity, Impl, OFFSET>,
            CreatePalette: CreatePalette::<Identity, Impl, OFFSET>,
            CreateFormatConverter: CreateFormatConverter::<Identity, Impl, OFFSET>,
            CreateBitmapScaler: CreateBitmapScaler::<Identity, Impl, OFFSET>,
            CreateBitmapClipper: CreateBitmapClipper::<Identity, Impl, OFFSET>,
            CreateBitmapFlipRotator: CreateBitmapFlipRotator::<Identity, Impl, OFFSET>,
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            CreateColorContext: CreateColorContext::<Identity, Impl, OFFSET>,
            CreateColorTransformer: CreateColorTransformer::<Identity, Impl, OFFSET>,
            CreateBitmap: CreateBitmap::<Identity, Impl, OFFSET>,
            CreateBitmapFromSource: CreateBitmapFromSource::<Identity, Impl, OFFSET>,
            CreateBitmapFromSourceRect: CreateBitmapFromSourceRect::<Identity, Impl, OFFSET>,
            CreateBitmapFromMemory: CreateBitmapFromMemory::<Identity, Impl, OFFSET>,
            CreateBitmapFromHBITMAP: CreateBitmapFromHBITMAP::<Identity, Impl, OFFSET>,
            CreateBitmapFromHICON: CreateBitmapFromHICON::<Identity, Impl, OFFSET>,
            CreateComponentEnumerator: CreateComponentEnumerator::<Identity, Impl, OFFSET>,
            CreateFastMetadataEncoderFromDecoder: CreateFastMetadataEncoderFromDecoder::<Identity, Impl, OFFSET>,
            CreateFastMetadataEncoderFromFrameDecode: CreateFastMetadataEncoderFromFrameDecode::<Identity, Impl, OFFSET>,
            CreateQueryWriter: CreateQueryWriter::<Identity, Impl, OFFSET>,
            CreateQueryWriterFromReader: CreateQueryWriterFromReader::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IWICJpegFrameDecode_Impl: ::windows_core::BaseImpl {
    fn DoesSupportIndexing(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIndexing(this: &Self::This, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows_core::Result<()>;
    fn ClearIndexing(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAcHuffmanTable(this: &Self::This, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetDcHuffmanTable(this: &Self::This, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetQuantizationTable(this: &Self::This, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::Result<()>;
    fn GetFrameHeader(this: &Self::This, pframeheader: *mut WICJpegFrameHeader) -> ::windows_core::Result<()>;
    fn GetScanHeader(this: &Self::This, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows_core::Result<()>;
    fn CopyScan(this: &Self::This, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows_core::Result<()>;
    fn CopyMinimalStream(this: &Self::This, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows_core::Iids for IWICJpegFrameDecode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICJpegFrameDecode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoesSupportIndexing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesSupportIndexing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfindexingsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIndexing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIndexing(this, ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&horizontalintervalsize)).into())
        }
        unsafe extern "system" fn ClearIndexing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearIndexing(this).into())
        }
        unsafe extern "system" fn GetAcHuffmanTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAcHuffmanTable(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pachuffmantable)).into())
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDcHuffmanTable(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pdchuffmantable)).into())
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuantizationTable(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pquantizationtable)).into())
        }
        unsafe extern "system" fn GetFrameHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameHeader(this, ::core::mem::transmute_copy(&pframeheader)).into())
        }
        unsafe extern "system" fn GetScanHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScanHeader(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&pscanheader)).into())
        }
        unsafe extern "system" fn CopyScan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyScan(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&scanoffset), ::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata), ::core::mem::transmute_copy(&pcbscandataactual)).into())
        }
        unsafe extern "system" fn CopyMinimalStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameDecode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyMinimalStream(this, ::core::mem::transmute_copy(&streamoffset), ::core::mem::transmute_copy(&cbstreamdata), ::core::mem::transmute_copy(&pbstreamdata), ::core::mem::transmute_copy(&pcbstreamdataactual)).into())
        }
        IWICJpegFrameDecode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DoesSupportIndexing: DoesSupportIndexing::<Identity, Impl, OFFSET>,
            SetIndexing: SetIndexing::<Identity, Impl, OFFSET>,
            ClearIndexing: ClearIndexing::<Identity, Impl, OFFSET>,
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, Impl, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, Impl, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, Impl, OFFSET>,
            GetFrameHeader: GetFrameHeader::<Identity, Impl, OFFSET>,
            GetScanHeader: GetScanHeader::<Identity, Impl, OFFSET>,
            CopyScan: CopyScan::<Identity, Impl, OFFSET>,
            CopyMinimalStream: CopyMinimalStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IWICJpegFrameEncode_Impl: ::windows_core::BaseImpl {
    fn GetAcHuffmanTable(this: &Self::This, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetDcHuffmanTable(this: &Self::This, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::Result<()>;
    fn GetQuantizationTable(this: &Self::This, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::Result<()>;
    fn WriteScan(this: &Self::This, cbscandata: u32, pbscandata: *const u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IWICJpegFrameEncode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICJpegFrameEncode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAcHuffmanTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAcHuffmanTable(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pachuffmantable)).into())
        }
        unsafe extern "system" fn GetDcHuffmanTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDcHuffmanTable(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pdchuffmantable)).into())
        }
        unsafe extern "system" fn GetQuantizationTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuantizationTable(this, ::core::mem::transmute_copy(&scanindex), ::core::mem::transmute_copy(&tableindex), ::core::mem::transmute_copy(&pquantizationtable)).into())
        }
        unsafe extern "system" fn WriteScan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICJpegFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteScan(this, ::core::mem::transmute_copy(&cbscandata), ::core::mem::transmute_copy(&pbscandata)).into())
        }
        IWICJpegFrameEncode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAcHuffmanTable: GetAcHuffmanTable::<Identity, Impl, OFFSET>,
            GetDcHuffmanTable: GetDcHuffmanTable::<Identity, Impl, OFFSET>,
            GetQuantizationTable: GetQuantizationTable::<Identity, Impl, OFFSET>,
            WriteScan: WriteScan::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockReader_Impl: ::windows_core::BaseImpl {
    fn GetContainerFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetReaderByIndex(this: &Self::This, nindex: u32) -> ::windows_core::Result<IWICMetadataReader>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWICMetadataBlockReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataBlockReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainerFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReaderByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReaderByIndex(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadatareader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienummetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICMetadataBlockReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetReaderByIndex: GetReaderByIndex::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICMetadataBlockWriter_Impl: ::windows_core::BaseImpl + IWICMetadataBlockReader_Impl {
    fn InitializeFromBlockReader(this: &Self::This, pimdblockreader: ::core::option::Option<&IWICMetadataBlockReader>) -> ::windows_core::Result<()>;
    fn GetWriterByIndex(this: &Self::This, nindex: u32) -> ::windows_core::Result<IWICMetadataWriter>;
    fn AddWriter(this: &Self::This, pimetadatawriter: ::core::option::Option<&IWICMetadataWriter>) -> ::windows_core::Result<()>;
    fn SetWriterByIndex(this: &Self::This, nindex: u32, pimetadatawriter: ::core::option::Option<&IWICMetadataWriter>) -> ::windows_core::Result<()>;
    fn RemoveWriterByIndex(this: &Self::This, nindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWICMetadataBlockWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICMetadataBlockReader);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataBlockWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeFromBlockReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimdblockreader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromBlockReader(this, ::windows_core::from_raw_borrowed(&pimdblockreader)).into())
        }
        unsafe extern "system" fn GetWriterByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWriterByIndex(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimetadatawriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimetadatawriter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddWriter(this, ::windows_core::from_raw_borrowed(&pimetadatawriter)).into())
        }
        unsafe extern "system" fn SetWriterByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWriterByIndex(this, ::core::mem::transmute_copy(&nindex), ::windows_core::from_raw_borrowed(&pimetadatawriter)).into())
        }
        unsafe extern "system" fn RemoveWriterByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataBlockWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveWriterByIndex(this, ::core::mem::transmute_copy(&nindex)).into())
        }
        IWICMetadataBlockWriter_Vtbl {
            base__: <IWICMetadataBlockReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeFromBlockReader: InitializeFromBlockReader::<Identity, Impl, OFFSET>,
            GetWriterByIndex: GetWriterByIndex::<Identity, Impl, OFFSET>,
            AddWriter: AddWriter::<Identity, Impl, OFFSET>,
            SetWriterByIndex: SetWriterByIndex::<Identity, Impl, OFFSET>,
            RemoveWriterByIndex: RemoveWriterByIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataHandlerInfo_Impl: ::windows_core::BaseImpl + IWICComponentInfo_Impl {
    fn GetMetadataFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetContainerFormats(this: &Self::This, ccontainerformats: u32, pguidcontainerformats: *mut ::windows_core::GUID, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceManufacturer(this: &Self::This, cchdevicemanufacturer: u32, wzdevicemanufacturer: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceModels(this: &Self::This, cchdevicemodels: u32, wzdevicemodels: &::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::Result<()>;
    fn DoesRequireFullStream(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesSupportPadding(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DoesRequireFixedSize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICMetadataHandlerInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICComponentInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataHandlerInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetadataFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidmetadataformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContainerFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows_core::GUID, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContainerFormats(this, ::core::mem::transmute_copy(&ccontainerformats), ::core::mem::transmute_copy(&pguidcontainerformats), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceManufacturer(this, ::core::mem::transmute_copy(&cchdevicemanufacturer), ::core::mem::transmute(&wzdevicemanufacturer), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn GetDeviceModels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows_core::PWSTR, pcchactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceModels(this, ::core::mem::transmute_copy(&cchdevicemodels), ::core::mem::transmute(&wzdevicemodels), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn DoesRequireFullStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesRequireFullStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrequiresfullstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoesSupportPadding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesSupportPadding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportspadding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoesRequireFixedSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataHandlerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoesRequireFixedSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffixedsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICMetadataHandlerInfo_Vtbl {
            base__: <IWICComponentInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetadataFormat: GetMetadataFormat::<Identity, Impl, OFFSET>,
            GetContainerFormats: GetContainerFormats::<Identity, Impl, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, Impl, OFFSET>,
            GetDeviceModels: GetDeviceModels::<Identity, Impl, OFFSET>,
            DoesRequireFullStream: DoesRequireFullStream::<Identity, Impl, OFFSET>,
            DoesSupportPadding: DoesSupportPadding::<Identity, Impl, OFFSET>,
            DoesRequireFixedSize: DoesRequireFixedSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataQueryReader_Impl: ::windows_core::BaseImpl {
    fn GetContainerFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetLocation(this: &Self::This, cchmaxlength: u32, wznamespace: &::windows_core::PWSTR, pcchactuallength: *mut u32) -> ::windows_core::Result<()>;
    fn GetMetadataByName(this: &Self::This, wzname: &::windows_core::PCWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWICMetadataQueryReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataQueryReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContainerFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainerFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidcontainerformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: ::windows_core::PWSTR, pcchactuallength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocation(this, ::core::mem::transmute_copy(&cchmaxlength), ::core::mem::transmute(&wznamespace), ::core::mem::transmute_copy(&pcchactuallength)).into())
        }
        unsafe extern "system" fn GetMetadataByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzname: ::windows_core::PCWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetadataByName(this, ::core::mem::transmute(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienumstring: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienumstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICMetadataQueryReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContainerFormat: GetContainerFormat::<Identity, Impl, OFFSET>,
            GetLocation: GetLocation::<Identity, Impl, OFFSET>,
            GetMetadataByName: GetMetadataByName::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataQueryWriter_Impl: ::windows_core::BaseImpl + IWICMetadataQueryReader_Impl {
    fn SetMetadataByName(this: &Self::This, wzname: &::windows_core::PCWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn RemoveMetadataByName(this: &Self::This, wzname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWICMetadataQueryWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICMetadataQueryReader);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataQueryWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMetadataByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzname: ::windows_core::PCWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMetadataByName(this, ::core::mem::transmute(&wzname), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn RemoveMetadataByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataQueryWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveMetadataByName(this, ::core::mem::transmute(&wzname)).into())
        }
        IWICMetadataQueryWriter_Vtbl {
            base__: <IWICMetadataQueryReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMetadataByName: SetMetadataByName::<Identity, Impl, OFFSET>,
            RemoveMetadataByName: RemoveMetadataByName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataReader_Impl: ::windows_core::BaseImpl {
    fn GetMetadataFormat(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetMetadataHandlerInfo(this: &Self::This) -> ::windows_core::Result<IWICMetadataHandlerInfo>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetValueByIndex(this: &Self::This, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetEnumerator(this: &Self::This) -> ::windows_core::Result<IWICEnumMetadataItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWICMetadataReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetadataFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidmetadataformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMetadataHandlerInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppihandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataHandlerInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppihandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValueByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValueByIndex(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValue(this, ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn GetEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppienummetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienummetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICMetadataReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetadataFormat: GetMetadataFormat::<Identity, Impl, OFFSET>,
            GetMetadataHandlerInfo: GetMetadataHandlerInfo::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetValueByIndex: GetValueByIndex::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetEnumerator: GetEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICMetadataReaderInfo_Impl: ::windows_core::BaseImpl + IWICMetadataHandlerInfo_Impl {
    fn GetPatterns(this: &Self::This, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows_core::Result<()>;
    fn MatchesPattern(this: &Self::This, guidcontainerformat: *const ::windows_core::GUID, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateInstance(this: &Self::This) -> ::windows_core::Result<IWICMetadataReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWICMetadataReaderInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICMetadataHandlerInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataReaderInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPatterns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatterns(this, ::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&ppattern), ::core::mem::transmute_copy(&pccount), ::core::mem::transmute_copy(&pcbactual)).into())
        }
        unsafe extern "system" fn MatchesPattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, pistream: *mut ::core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MatchesPattern(this, ::core::mem::transmute_copy(&guidcontainerformat), ::windows_core::from_raw_borrowed(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataReaderInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppireader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICMetadataReaderInfo_Vtbl {
            base__: <IWICMetadataHandlerInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPatterns: GetPatterns::<Identity, Impl, OFFSET>,
            MatchesPattern: MatchesPattern::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWICMetadataWriter_Impl: ::windows_core::BaseImpl + IWICMetadataReader_Impl {
    fn SetValue(this: &Self::This, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn SetValueByIndex(this: &Self::This, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn RemoveValue(this: &Self::This, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn RemoveValueByIndex(this: &Self::This, nindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWICMetadataWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICMetadataReader);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn SetValueByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueByIndex(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn RemoveValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveValue(this, ::core::mem::transmute_copy(&pvarschema), ::core::mem::transmute_copy(&pvarid)).into())
        }
        unsafe extern "system" fn RemoveValueByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveValueByIndex(this, ::core::mem::transmute_copy(&nindex)).into())
        }
        IWICMetadataWriter_Vtbl {
            base__: <IWICMetadataReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            SetValueByIndex: SetValueByIndex::<Identity, Impl, OFFSET>,
            RemoveValue: RemoveValue::<Identity, Impl, OFFSET>,
            RemoveValueByIndex: RemoveValueByIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICMetadataWriterInfo_Impl: ::windows_core::BaseImpl + IWICMetadataHandlerInfo_Impl {
    fn GetHeader(this: &Self::This, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows_core::Result<()>;
    fn CreateInstance(this: &Self::This) -> ::windows_core::Result<IWICMetadataWriter>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICMetadataWriterInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICMetadataHandlerInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICMetadataWriterInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows_core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHeader(this, ::core::mem::transmute_copy(&guidcontainerformat), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pheader), ::core::mem::transmute_copy(&pcbactual)).into())
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICMetadataWriterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICMetadataWriterInfo_Vtbl {
            base__: <IWICMetadataHandlerInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHeader: GetHeader::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPalette_Impl: ::windows_core::BaseImpl {
    fn InitializePredefined(this: &Self::This, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn InitializeCustom(this: &Self::This, pcolors: *const u32, ccount: u32) -> ::windows_core::Result<()>;
    fn InitializeFromBitmap(this: &Self::This, pisurface: ::core::option::Option<&IWICBitmapSource>, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn InitializeFromPalette(this: &Self::This, pipalette: ::core::option::Option<&IWICPalette>) -> ::windows_core::Result<()>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<WICBitmapPaletteType>;
    fn GetColorCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetColors(this: &Self::This, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows_core::Result<()>;
    fn IsBlackWhite(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsGrayscale(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn HasAlpha(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICPalette {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICPalette {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializePredefined<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializePredefined(this, ::core::mem::transmute_copy(&epalettetype), ::core::mem::transmute_copy(&faddtransparentcolor)).into())
        }
        unsafe extern "system" fn InitializeCustom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeCustom(this, ::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&ccount)).into())
        }
        unsafe extern "system" fn InitializeFromBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisurface: *mut ::core::ffi::c_void, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromBitmap(this, ::windows_core::from_raw_borrowed(&pisurface), ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&faddtransparentcolor)).into())
        }
        unsafe extern "system" fn InitializeFromPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromPalette(this, ::windows_core::from_raw_borrowed(&pipalette)).into())
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pepalettetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColorCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColors(this, ::core::mem::transmute_copy(&ccount), ::core::mem::transmute_copy(&pcolors), ::core::mem::transmute_copy(&pcactualcolors)).into())
        }
        unsafe extern "system" fn IsBlackWhite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBlackWhite(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisblackwhite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsGrayscale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsGrayscale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisgrayscale, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasAlpha<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPalette_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasAlpha(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasalpha, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICPalette_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializePredefined: InitializePredefined::<Identity, Impl, OFFSET>,
            InitializeCustom: InitializeCustom::<Identity, Impl, OFFSET>,
            InitializeFromBitmap: InitializeFromBitmap::<Identity, Impl, OFFSET>,
            InitializeFromPalette: InitializeFromPalette::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetColorCount: GetColorCount::<Identity, Impl, OFFSET>,
            GetColors: GetColors::<Identity, Impl, OFFSET>,
            IsBlackWhite: IsBlackWhite::<Identity, Impl, OFFSET>,
            IsGrayscale: IsGrayscale::<Identity, Impl, OFFSET>,
            HasAlpha: HasAlpha::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICPersistStream_Impl: ::windows_core::BaseImpl + super::super::System::Com::IPersistStream_Impl {
    fn LoadEx(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>, pguidpreferredvendor: *const ::windows_core::GUID, dwpersistoptions: u32) -> ::windows_core::Result<()>;
    fn SaveEx(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWICPersistStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IPersistStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPersistStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICPersistStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPersistStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pguidpreferredvendor: *const ::windows_core::GUID, dwpersistoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadEx(this, ::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&pguidpreferredvendor), ::core::mem::transmute_copy(&dwpersistoptions)).into())
        }
        unsafe extern "system" fn SaveEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPersistStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveEx(this, ::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&dwpersistoptions), ::core::mem::transmute_copy(&fcleardirty)).into())
        }
        IWICPersistStream_Vtbl {
            base__: <super::super::System::Com::IPersistStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadEx: LoadEx::<Identity, Impl, OFFSET>,
            SaveEx: SaveEx::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICPixelFormatInfo_Impl: ::windows_core::BaseImpl + IWICComponentInfo_Impl {
    fn GetFormatGUID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetColorContext(this: &Self::This) -> ::windows_core::Result<IWICColorContext>;
    fn GetBitsPerPixel(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetChannelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetChannelMask(this: &Self::This, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICPixelFormatInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICComponentInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICPixelFormatInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFormatGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatGUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColorContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppicolorcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBitsPerPixel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBitsPerPixel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puibitsperpixel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChannelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChannelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puichannelcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChannelMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChannelMask(this, ::core::mem::transmute_copy(&uichannelindex), ::core::mem::transmute_copy(&cbmaskbuffer), ::core::mem::transmute_copy(&pbmaskbuffer), ::core::mem::transmute_copy(&pcbactual)).into())
        }
        IWICPixelFormatInfo_Vtbl {
            base__: <IWICComponentInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFormatGUID: GetFormatGUID::<Identity, Impl, OFFSET>,
            GetColorContext: GetColorContext::<Identity, Impl, OFFSET>,
            GetBitsPerPixel: GetBitsPerPixel::<Identity, Impl, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            GetChannelMask: GetChannelMask::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPixelFormatInfo2_Impl: ::windows_core::BaseImpl + IWICPixelFormatInfo_Impl {
    fn SupportsTransparency(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetNumericRepresentation(this: &Self::This) -> ::windows_core::Result<WICPixelFormatNumericRepresentation>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICPixelFormatInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICPixelFormatInfo);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICPixelFormatInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SupportsTransparency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportsTransparency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportstransparency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumericRepresentation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPixelFormatInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumericRepresentation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumericrepresentation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICPixelFormatInfo2_Vtbl {
            base__: <IWICPixelFormatInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SupportsTransparency: SupportsTransparency::<Identity, Impl, OFFSET>,
            GetNumericRepresentation: GetNumericRepresentation::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICPlanarBitmapFrameEncode_Impl: ::windows_core::BaseImpl {
    fn WritePixels(this: &Self::This, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::Result<()>;
    fn WriteSource(this: &Self::This, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, prcsource: *const WICRect) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICPlanarBitmapFrameEncode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICPlanarBitmapFrameEncode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WritePixels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WritePixels(this, ::core::mem::transmute_copy(&linecount), ::core::mem::transmute_copy(&pplanes), ::core::mem::transmute_copy(&cplanes)).into())
        }
        unsafe extern "system" fn WriteSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarBitmapFrameEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppplanes: *const *mut ::core::ffi::c_void, cplanes: u32, prcsource: *const WICRect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteSource(this, ::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&prcsource)).into())
        }
        IWICPlanarBitmapFrameEncode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WritePixels: WritePixels::<Identity, Impl, OFFSET>,
            WriteSource: WriteSource::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarBitmapSourceTransform_Impl: ::windows_core::BaseImpl {
    fn DoesSupportTransform(this: &Self::This, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows_core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CopyPixels(this: &Self::This, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICPlanarBitmapSourceTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICPlanarBitmapSourceTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoesSupportTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows_core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoesSupportTransform(this, ::core::mem::transmute_copy(&puiwidth), ::core::mem::transmute_copy(&puiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pguiddstformats), ::core::mem::transmute_copy(&pplanedescriptions), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&pfissupported)).into())
        }
        unsafe extern "system" fn CopyPixels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarBitmapSourceTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyPixels(this, ::core::mem::transmute_copy(&prcsource), ::core::mem::transmute_copy(&uiwidth), ::core::mem::transmute_copy(&uiheight), ::core::mem::transmute_copy(&dsttransform), ::core::mem::transmute_copy(&dstplanaroptions), ::core::mem::transmute_copy(&pdstplanes), ::core::mem::transmute_copy(&cplanes)).into())
        }
        IWICPlanarBitmapSourceTransform_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DoesSupportTransform: DoesSupportTransform::<Identity, Impl, OFFSET>,
            CopyPixels: CopyPixels::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWICPlanarFormatConverter_Impl: ::windows_core::BaseImpl + IWICBitmapSource_Impl {
    fn Initialize(this: &Self::This, ppplanes: *const ::core::option::Option<IWICBitmapSource>, cplanes: u32, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: ::core::option::Option<&IWICPalette>, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::Result<()>;
    fn CanConvert(this: &Self::This, psrcpixelformats: *const ::windows_core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWICPlanarFormatConverter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWICBitmapSource);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICPlanarFormatConverter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppplanes: *const *mut ::core::ffi::c_void, cplanes: u32, dstformat: *const ::windows_core::GUID, dither: WICBitmapDitherType, pipalette: *mut ::core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&ppplanes), ::core::mem::transmute_copy(&cplanes), ::core::mem::transmute_copy(&dstformat), ::core::mem::transmute_copy(&dither), ::windows_core::from_raw_borrowed(&pipalette), ::core::mem::transmute_copy(&alphathresholdpercent), ::core::mem::transmute_copy(&palettetranslate)).into())
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICPlanarFormatConverter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrcpixelformats: *const ::windows_core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows_core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanConvert(this, ::core::mem::transmute_copy(&psrcpixelformats), ::core::mem::transmute_copy(&csrcplanes), ::core::mem::transmute_copy(&dstpixelformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanconvert, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWICPlanarFormatConverter_Vtbl {
            base__: <IWICBitmapSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWICProgressCallback_Impl: ::windows_core::BaseImpl {
    fn Notify(this: &Self::This, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICProgressCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICProgressCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICProgressCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICProgressCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::core::mem::transmute_copy(&uframenum), ::core::mem::transmute_copy(&operation), ::core::mem::transmute_copy(&dblprogress)).into())
        }
        IWICProgressCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Notify: Notify::<Identity, Impl, OFFSET> }
    };
}
pub trait IWICProgressiveLevelControl_Impl: ::windows_core::BaseImpl {
    fn GetLevelCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCurrentLevel(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetCurrentLevel(this: &Self::This, nlevel: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWICProgressiveLevelControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICProgressiveLevelControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLevelCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLevelCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclevels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCurrentLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICProgressiveLevelControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentLevel(this, ::core::mem::transmute_copy(&nlevel)).into())
        }
        IWICProgressiveLevelControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLevelCount: GetLevelCount::<Identity, Impl, OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Identity, Impl, OFFSET>,
            SetCurrentLevel: SetCurrentLevel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWICStream_Impl: ::windows_core::BaseImpl + super::super::System::Com::IStream_Impl {
    fn InitializeFromIStream(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn InitializeFromFilename(this: &Self::This, wzfilename: &::windows_core::PCWSTR, dwdesiredaccess: u32) -> ::windows_core::Result<()>;
    fn InitializeFromMemory(this: &Self::This, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::Result<()>;
    fn InitializeFromIStreamRegion(this: &Self::This, pistream: ::core::option::Option<&super::super::System::Com::IStream>, uloffset: u64, ulmaxsize: u64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWICStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeFromIStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromIStream(this, ::windows_core::from_raw_borrowed(&pistream)).into())
        }
        unsafe extern "system" fn InitializeFromFilename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzfilename: ::windows_core::PCWSTR, dwdesiredaccess: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromFilename(this, ::core::mem::transmute(&wzfilename), ::core::mem::transmute_copy(&dwdesiredaccess)).into())
        }
        unsafe extern "system" fn InitializeFromMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromMemory(this, ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffersize)).into())
        }
        unsafe extern "system" fn InitializeFromIStreamRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, uloffset: u64, ulmaxsize: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeFromIStreamRegion(this, ::windows_core::from_raw_borrowed(&pistream), ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&ulmaxsize)).into())
        }
        IWICStream_Vtbl {
            base__: <super::super::System::Com::IStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeFromIStream: InitializeFromIStream::<Identity, Impl, OFFSET>,
            InitializeFromFilename: InitializeFromFilename::<Identity, Impl, OFFSET>,
            InitializeFromMemory: InitializeFromMemory::<Identity, Impl, OFFSET>,
            InitializeFromIStreamRegion: InitializeFromIStreamRegion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWICStreamProvider_Impl: ::windows_core::BaseImpl {
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn GetPersistOptions(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPreferredVendorGUID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn RefreshStream(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWICStreamProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWICStreamProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppistream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppistream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPersistOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPersistOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpersistoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreferredVendorGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreferredVendorGUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidpreferredvendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RefreshStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWICStreamProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshStream(this).into())
        }
        IWICStreamProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            GetPersistOptions: GetPersistOptions::<Identity, Impl, OFFSET>,
            GetPreferredVendorGUID: GetPreferredVendorGUID::<Identity, Impl, OFFSET>,
            RefreshStream: RefreshStream::<Identity, Impl, OFFSET>,
        }
    };
}
